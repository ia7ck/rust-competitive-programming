use anyhow::Result;
use glob::glob;

use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;

enum JudgeType {
    Normal, // Compare actual output with expected output normally.
    SpecialJudge { judge_program_path: PathBuf }, // For problem which has one or more answers.
}

struct Test {
    judge_type: JudgeType,
    solver_path: PathBuf,
    url: String,
}

impl Test {
    pub fn run(&self, testcase_dir: &Path) -> Result<()> {
        let solver_name = self.solver_path.file_stem().unwrap().to_string_lossy();
        let solve_command = format!("cargo run --quiet --release --example {}", solver_name);
        match &self.judge_type {
            JudgeType::Normal => {
                println!(
                    "oj test --directory {} --command \"{}\" --jobs 2",
                    testcase_dir.display(),
                    solve_command
                );
                let status = Command::new("oj")
                    .arg("test")
                    .arg("--directory")
                    .arg(testcase_dir.as_os_str())
                    .arg("--command")
                    .arg(solve_command)
                    .arg("--jobs")
                    .arg("2")
                    .status()?;
                assert!(status.success(), "failed: oj test");
            }
            JudgeType::SpecialJudge { judge_program_path } => {
                let judge_name = judge_program_path.file_stem().unwrap().to_string_lossy();
                let judge_command = format!("cargo run --quiet --release --example {}", judge_name);
                println!(
                    "oj test --directory {} --command \"{}\" --judge-command \"{}\" --jobs 2",
                    testcase_dir.display(),
                    solve_command,
                    judge_command,
                );
                let status = Command::new("oj")
                    .arg("test")
                    .arg("--directory")
                    .arg(testcase_dir.as_os_str())
                    .arg("--command")
                    .arg(solve_command)
                    .arg("--judge-command")
                    .arg(judge_command)
                    .arg("--jobs")
                    .arg("2")
                    .status()?;
                assert!(status.success(), "failed: oj test");
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut tests = Vec::new();
    for entry in glob("**/examples/*.rs")? {
        let path = entry?;
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let mut first_line = String::new();
        reader.read_line(&mut first_line)?;
        if let Some(url) = parse_problem_url(&first_line) {
            let mut second_line = String::new();
            reader.read_line(&mut second_line)?;
            let t = if let Some(judge_program) = parse_judge_rs_program(&second_line) {
                let judge_program_path = path.parent().unwrap().join(&judge_program);
                Test {
                    judge_type: JudgeType::SpecialJudge { judge_program_path },
                    solver_path: path,
                    url,
                }
            } else {
                Test {
                    judge_type: JudgeType::Normal,
                    solver_path: path,
                    url,
                }
            };
            tests.push(t);
        }
    }
    tests.sort_by(|t1, t2| t1.solver_path.cmp(&t2.solver_path));
    for t in tests {
        let download_dir = env::temp_dir().join(t.solver_path.with_extension(""));
        if download_dir.exists() {
            fs::remove_dir_all(&download_dir)?;
        }
        println!(
            "oj download {} --directory {} --system --silent",
            t.url,
            download_dir.display()
        );
        let status = Command::new("oj")
            .arg("download")
            .arg(&t.url)
            .arg("--directory")
            .arg(download_dir.as_os_str())
            .arg("--system")
            .arg("--silent")
            .status()?;
        assert!(status.success(), "failed: oj download");

        t.run(&download_dir)?;
    }

    Ok(())
}

fn parse_problem_url(s: &str) -> Option<String> {
    if s.trim_start().starts_with("//") {
        let t = s.replacen("//", "", 1);
        if t.trim_start().starts_with("oj") {
            let u = t.replacen("oj", "", 1);
            if u.trim_start().starts_with(':') {
                return Some(u.replacen(':', "", 1).trim().to_string());
            }
        }
    }
    None
}

fn parse_judge_rs_program(s: &str) -> Option<String> {
    if s.trim_start().starts_with("//") {
        let t = s.replacen("//", "", 1);
        if t.trim_start().starts_with("oj_judge_rs_program") {
            let u = t.replacen("oj_judge_rs_program", "", 1);
            if u.trim_start().starts_with(':') {
                return Some(u.replacen(':', "", 1).trim().to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::{parse_judge_rs_program, parse_problem_url};

    #[test]
    fn parse_meta_data_test() {
        assert_eq!(
            parse_problem_url("// oj: http://example.com"),
            Some("http://example.com".to_string())
        );
        assert_eq!(
            parse_problem_url("//oj: http://example.com"),
            Some("http://example.com".to_string())
        );
        assert_eq!(
            parse_problem_url("// oj:http://example.com"),
            Some("http://example.com".to_string())
        );
        assert_eq!(
            parse_problem_url("// oj : http://example.com"),
            Some("http://example.com".to_string())
        );

        assert_eq!(
            parse_judge_rs_program("// oj_judge_rs_program: ./my_judge.rs"),
            Some("./my_judge.rs".to_string())
        );
        assert_eq!(parse_judge_rs_program("fn main() {"), None);
    }
}
