use anyhow::Result;
use glob::glob;

use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

fn main() -> Result<()> {
    let mut tests = Vec::new();
    for entry in glob("**/examples/*.rs")? {
        let path = entry?;
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let mut buf = String::new();
        reader.read_line(&mut buf)?;
        if let Some(url) = parse_line_comment(&buf) {
            tests.push((path, url));
        }
    }
    tests.sort();
    for (path, url) in tests {
        let download_dir = env::temp_dir().join(path.with_extension(""));
        if download_dir.exists() {
            fs::remove_dir_all(&download_dir)?;
        }
        println!(
            "oj download {} --directory {} --system --silent",
            url,
            download_dir.display()
        );
        let status = Command::new("oj")
            .arg("download")
            .arg(url)
            .arg("--directory")
            .arg(download_dir.as_os_str())
            .arg("--system")
            .arg("--silent")
            .status()?;
        assert!(status.success(), "failed: oj download");

        let example_name = path.file_stem().unwrap();
        let cargo_command = format!(
            "cargo run --quiet --release --example {}",
            example_name.to_string_lossy()
        );
        println!(
            "oj test --directory {} --command \"{}\" --jobs 2",
            download_dir.display(),
            cargo_command
        );
        let status = Command::new("oj")
            .arg("test")
            .arg("--directory")
            .arg(download_dir.as_os_str())
            .arg("--command")
            .arg(cargo_command)
            .arg("--jobs")
            .arg("2")
            .status()?;
        assert!(status.success(), "failed: oj test");
    }

    Ok(())
}

fn parse_line_comment(s: &str) -> Option<String> {
    if s.trim_start().starts_with("//") {
        let t = s.replacen("//", "", 1);
        if t.trim_start().starts_with("oj:") {
            return Some(t.replacen("oj:", "", 1).trim().to_string());
        }
    }
    None
}