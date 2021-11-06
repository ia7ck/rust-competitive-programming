use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{ensure, Result};
use log::info;

pub struct ProblemSolver {
    solver_path: PathBuf,
    test_property: TestProperty,
}

impl Debug for ProblemSolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.solver_path)
    }
}

impl ProblemSolver {
    pub fn new(path: &Path) -> Self {
        Self {
            solver_path: path.to_path_buf(),
            test_property: TestProperty::from(path),
        }
    }

    pub fn solver_path(&self) -> &Path {
        self.solver_path.as_path()
    }

    pub fn run<T>(&self, testcase: T) -> Result<()>
    where
        T: Testcase,
    {
        testcase.setup()?;

        let solver_name = self.solver_path.file_stem().unwrap().to_string_lossy();
        let solve_command = format!("cargo run --quiet --release --example {}", solver_name);

        let mut oj_command = Command::new("oj");
        oj_command
            .arg("test")
            .arg("--directory")
            .arg(testcase.testcase_dir().as_os_str())
            .arg("--command")
            .arg(solve_command);

        if std::env::consts::OS != "windows" {
            oj_command.arg("--jobs").arg("2");
        }

        // special judge
        // for problem which has multiple answers
        if let Some(judge_program_path) = self.judge_program_path() {
            let judge_name = judge_program_path.file_stem().unwrap().to_string_lossy();
            let judge_command = format!("cargo run --quiet --release --example {}", judge_name);
            oj_command.arg("--judge-command").arg(judge_command);
        }

        info!("execute {:?}", oj_command);
        let status = oj_command.status()?;
        ensure!(status.success(), "failed: oj test");

        Ok(())
    }

    pub fn problem_url(&self) -> Option<&str> {
        self.test_property.get("problem")
    }

    fn judge_program_path(&self) -> Option<PathBuf> {
        self.test_property
            .get("judge_program_rs")
            .map(|judge_program_rs| self.solver_path.parent().unwrap().join(judge_program_rs))
    }
}

pub trait Testcase {
    fn setup(&self) -> Result<()>;
    fn testcase_dir(&self) -> PathBuf;
}

pub struct OnlineJudgeTestcase {
    dir: PathBuf,
    problem_url: String,
}

impl OnlineJudgeTestcase {
    pub fn new(dir: &Path, problem_url: &str) -> Self {
        Self {
            dir: dir.to_path_buf(),
            problem_url: problem_url.to_string(),
        }
    }
}

impl Testcase for OnlineJudgeTestcase {
    // download testcase
    fn setup(&self) -> Result<()> {
        let testcase_dir = self.testcase_dir();
        if testcase_dir.exists() {
            // clear temporary directory
            fs::remove_dir_all(testcase_dir.as_path()).unwrap_or_else(|err| panic!("{}", err));
        }
        let mut oj_command = Command::new("oj");
        oj_command
            .arg("download")
            .arg(self.problem_url.as_str())
            .arg("--directory")
            .arg(testcase_dir.as_os_str())
            .arg("--system")
            .arg("--silent");
        info!("execute {:?}", oj_command);
        let status = oj_command.status()?;
        ensure!(status.success(), "failed: oj download");
        Ok(())
    }

    fn testcase_dir(&self) -> PathBuf {
        self.dir.clone()
    }
}

struct TestProperty {
    properties: HashMap<String, String>,
}

impl TestProperty {
    fn new(solver_source_code: &str) -> Self {
        let mut properties = HashMap::new();
        for l in solver_source_code.lines() {
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

    fn from(solver_path: &Path) -> Self {
        let source_code = fs::read_to_string(solver_path).unwrap_or_else(|err| panic!("{}", err));
        Self::new(&source_code)
    }

    fn get(&self, key: &str) -> Option<&str> {
        self.properties.get(key).map(|s| s.as_str())
    }
}

pub fn check_oj_version() -> Result<()> {
    let mut cmd = Command::new("oj");
    cmd.arg("--version");
    info!("{:?}", cmd);
    ensure!(cmd.status().is_ok(), "oj is not installed");
    Ok(())
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
        assert_eq!(property.get("problem1"), Some("https://example1.com"));
        assert_eq!(property.get("problem2"), Some("https://example2.com"));
        assert_eq!(property.get("problem3"), Some("https://example3.com"));
        assert_eq!(property.get("problem4"), Some("https://example4.com"));
        assert_eq!(property.get("judge_program_rs"), Some("./my_judge.rs"));
        assert_eq!(property.get("return"), None);
    }
}
