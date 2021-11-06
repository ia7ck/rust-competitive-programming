use anyhow::Result;
use glob::glob;

use std::collections::HashMap;
use std::env;
use std::fs;
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
    fn run(&self, testcase_dir: &Path) -> Result<()> {
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
        let source_code = fs::read_to_string(&path)?;
        let property = TestProperty::new(&source_code);
        if let Some(url) = property.get("problem") {
            let t = if let Some(judge_program) = property.get("judge_program_rs") {
                let judge_program_path = path.parent().unwrap().join(&judge_program);
                Test {
                    judge_type: JudgeType::SpecialJudge { judge_program_path },
                    solver_path: path,
                    problem_url: url.to_string(),
                }
            } else {
                Test {
                    judge_type: JudgeType::Normal,
                    solver_path: path,
                    problem_url: url.to_string(),
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

struct TestProperty {
    properties: HashMap<String, String>,
}

impl TestProperty {
    fn new(source_code: &str) -> Self {
        let mut properties = HashMap::new();
        for l in source_code.lines() {
            let v: Vec<&str> = l.splitn(2, ':').map(|t| t.trim()).collect();
            if v.len() != 2 {
                continue;
            }
            if !v[0].starts_with("//") {
                continue;
            }
            let key = v[0].trim_start_matches('/').trim();
            properties.insert(key.to_string(), v[1].to_string());
        }
        Self { properties }
    }
    fn get(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }
}

#[cfg(test)]
mod tests {
    use crate::TestProperty;

    #[test]
    fn parse_property_test() {
        let source_code = r#"// problem1: https://example1.com
//problem2: https://example2.com
 // problem3:https://example3.com
// problem4 : https://example4.com

// judge_program_rs: ./my_judge.rs
fn main() {
// return;
}"#;
        let property = TestProperty::new(source_code);
        assert_eq!(
            property.get("problem1").cloned(),
            Some("https://example1.com".to_string())
        );
        assert_eq!(
            property.get("problem2").cloned(),
            Some("https://example2.com".to_string())
        );
        assert_eq!(
            property.get("problem3").cloned(),
            Some("https://example3.com".to_string())
        );
        assert_eq!(
            property.get("problem4").cloned(),
            Some("https://example4.com".to_string())
        );
        assert_eq!(
            property.get("judge_program_rs").cloned(),
            Some("./my_judge.rs".to_string())
        );
        assert_eq!(property.get("return"), None);
    }
}
