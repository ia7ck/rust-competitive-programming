use std::collections::HashMap;
use std::env;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{ensure, Result};
use log::info;

pub struct ProblemSolver {
    solver_path: PathBuf,
    test_property: TestProperty,
}

impl Display for ProblemSolver {
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

    pub fn run(&self, testcase_dir: &Path) -> Result<()> {
        let mut oj_command = Command::new("oj");
        oj_command
            .arg("test")
            .arg("--directory")
            .arg(testcase_dir.as_os_str())
            .arg("--command")
            .arg(example_binary_path(self.solver_path.as_path()));

        if env::consts::OS != "windows" {
            oj_command.arg("--jobs").arg("2");
        }

        // special judge
        // for problem which has multiple answers
        if let Some(judge_program_path) = self.judge_program_path() {
            oj_command
                .arg("--judge-command")
                .arg(example_binary_path(judge_program_path.as_path()));
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

pub fn download_online_judge_testcase(problem_url: &str, dir_suffix: &Path) -> Result<PathBuf> {
    let dir = env::temp_dir().join(dir_suffix);
    if dir.exists() {
        fs::remove_dir_all(&dir).unwrap_or_else(|err| panic!("{}", err));
    }
    let mut oj_command = Command::new("oj");
    oj_command
        .arg("download")
        .arg(problem_url)
        .arg("--directory")
        .arg(dir.as_os_str())
        .arg("--system")
        .arg("--silent");
    info!("execute {:?}", oj_command);
    let status = oj_command.status()?;
    ensure!(status.success(), "failed: oj download");
    Ok(dir)
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

fn cargo_target_examples_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("target")
        .join("release")
        .join("examples")
}

fn example_binary_path(source_path: &Path) -> PathBuf {
    let file_name = source_path.file_name().unwrap();
    let path = cargo_target_examples_dir().join(file_name);
    if env::consts::OS == "windows" {
        path.with_extension("exe")
    } else {
        path.with_extension("")
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
        assert_eq!(property.get("problem1"), Some("https://example1.com"));
        assert_eq!(property.get("problem2"), Some("https://example2.com"));
        assert_eq!(property.get("problem3"), Some("https://example3.com"));
        assert_eq!(property.get("problem4"), Some("https://example4.com"));
        assert_eq!(property.get("judge_program_rs"), Some("./my_judge.rs"));
        assert_eq!(property.get("return"), None);
    }
}
