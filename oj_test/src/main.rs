use anyhow::{ensure, Result};
use glob::glob;
use log::info;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

enum JudgeType {
    Normal, // Compare actual output with expected output normally.
    SpecialJudge { judge_program_path: PathBuf }, // For problem which has one or more answers.
}

struct ProblemSolver {
    path: PathBuf,
    test_property: TestProperty,
}

impl ProblemSolver {
    fn new(path: &Path) -> Self {
        let source_code = fs::read_to_string(path).unwrap_or_else(|err| panic!(err));
        Self {
            path: path.to_path_buf(),
            test_property: TestProperty::new(&source_code),
        }
    }

    fn run_test(&self, testcase_dir: &Path) -> Result<()> {
        let solver_name = self.path.file_stem().unwrap().to_string_lossy();
        let solve_command = format!("cargo run --quiet --release --example {}", solver_name);
        match self.judge_type().unwrap() {
            JudgeType::Normal => {
                let mut oj_command = Command::new("oj");
                oj_command
                    .arg("test")
                    .arg("--directory")
                    .arg(testcase_dir.as_os_str())
                    .arg("--command")
                    .arg(solve_command)
                    .arg("--jobs")
                    .arg("2");
                info!("execute {:?}", oj_command);
                let status = oj_command.status()?;
                ensure!(status.success(), "failed: oj test");
            }
            JudgeType::SpecialJudge { judge_program_path } => {
                let judge_name = judge_program_path.file_stem().unwrap().to_string_lossy();
                let judge_command = format!("cargo run --quiet --release --example {}", judge_name);
                let mut oj_command = Command::new("oj");
                oj_command
                    .arg("test")
                    .arg("--directory")
                    .arg(testcase_dir.as_os_str())
                    .arg("--command")
                    .arg(solve_command)
                    .arg("--judge-command")
                    .arg(judge_command)
                    .arg("--jobs")
                    .arg("2");
                info!("execute {:?}", oj_command);
                let status = oj_command.status()?;
                ensure!(status.success(), "failed: oj test");
            }
        }
        Ok(())
    }

    fn url(&self) -> String {
        self.test_property.get("problem").unwrap().clone()
    }

    fn judge_type(&self) -> Option<JudgeType> {
        let _problem_url = self.test_property.get("problem")?;
        match self.test_property.get("judge_program_rs") {
            Some(judge_program_rs) => {
                let judge_program_path = self.path.parent().unwrap().join(&judge_program_rs);
                Some(JudgeType::SpecialJudge { judge_program_path })
            }
            None => Some(JudgeType::Normal),
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let mut solvers = Vec::new();
    for entry in glob("**/examples/*.rs")? {
        let path = entry?;
        solvers.push(ProblemSolver::new(&path));
    }
    solvers.sort_by(|s1, s2| s1.path.cmp(&s2.path));
    for s in solvers {
        let download_dir = env::temp_dir().join(s.path.with_extension(""));
        if download_dir.exists() {
            fs::remove_dir_all(&download_dir)?;
        }
        let mut oj_command = Command::new("oj");
        oj_command
            .arg("download")
            .arg(&s.url())
            .arg("--directory")
            .arg(download_dir.as_os_str())
            .arg("--system")
            .arg("--silent");
        info!("execute {:?}", oj_command);
        let status = oj_command.status()?;
        ensure!(status.success(), "failed: oj download");

        s.run_test(&download_dir)?;
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
