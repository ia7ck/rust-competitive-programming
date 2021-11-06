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
    problem_url: String,
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
        if let Some(url) = parse_property(&first_line, "problem") {
            let mut second_line = String::new();
            reader.read_line(&mut second_line)?;
            let t = if let Some(judge_program) = parse_property(&second_line, "judge_program_rs") {
                let judge_program_path = path.parent().unwrap().join(&judge_program);
                Test {
                    judge_type: JudgeType::SpecialJudge { judge_program_path },
                    solver_path: path,
                    problem_url: url,
                }
            } else {
                Test {
                    judge_type: JudgeType::Normal,
                    solver_path: path,
                    problem_url: url,
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
            t.problem_url,
            download_dir.display()
        );
        let status = Command::new("oj")
            .arg("download")
            .arg(&t.problem_url)
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

fn parse_property(s: &str, key: &str) -> Option<String> {
    let v: Vec<&str> = s.splitn(2, ':').map(|t| t.trim()).collect();
    if v.len() != 2 {
        return None;
    }
    if !v[0].starts_with("//") {
        return None;
    }
    if !v[0].ends_with(key) {
        return None;
    }
    Some(v[1].to_string())
}

#[cfg(test)]
mod tests {
    use crate::parse_property;

    #[test]
    fn parse_property_test() {
        assert_eq!(
            parse_property("// problem: http://example.com", "problem"),
            Some("http://example.com".to_string())
        );
        assert_eq!(
            parse_property("//problem: http://example.com", "problem"),
            Some("http://example.com".to_string())
        );
        assert_eq!(
            parse_property("// problem:http://example.com", "problem"),
            Some("http://example.com".to_string())
        );
        assert_eq!(
            parse_property("// problem : http://example.com", "problem"),
            Some("http://example.com".to_string())
        );

        assert_eq!(
            parse_property("// judge_program_rs: ./my_judge.rs", "judge_program_rs"),
            Some("./my_judge.rs".to_string())
        );
        assert_eq!(parse_property("fn main() {", "judge_program_rs"), None);
    }
}
