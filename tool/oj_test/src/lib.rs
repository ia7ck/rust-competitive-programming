use std::collections::HashMap;
use std::env;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{ensure, Result};
use glob::glob;
use log::info;

pub struct OjTestArgs {
    pub pattern: String,
    pub dry_run: bool,
}

pub struct OjTestRunner {
    testcase_dir: PathBuf,
}

impl OjTestRunner {
    pub fn new() -> Result<Self> {
        let testcase_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("testcases");

        if !testcase_dir.exists() {
            fs::create_dir_all(&testcase_dir)?;
        }

        Ok(Self { testcase_dir })
    }

    pub fn run(&self, args: OjTestArgs) -> Result<()> {
        let mut solvers = Vec::new();
        for entry in glob(&args.pattern)? {
            let path = entry?;
            solvers.push(ProblemSolver::new(path.as_path()));
        }
        solvers.sort_by(|s1, s2| s1.solver_path().cmp(s2.solver_path()));

        info!("Found {} solvers", solvers.len());

        for solver in solvers {
            if let Some(problem_url) = solver.problem_url() {
                if args.dry_run {
                    println!(
                        "Would test: {} -> {}",
                        solver.solver_path().display(),
                        problem_url
                    );
                    continue;
                }

                let testcase_dir = self.get_or_download_testcase(problem_url)?;
                solver.run(testcase_dir.as_path())?;
            } else {
                info!("skip {} (no problem URL)", solver);
            }
        }

        Ok(())
    }

    fn get_or_download_testcase(&self, problem_url: &str) -> Result<PathBuf> {
        let dir_name = problem_url.replace(['/', ':', '.', '?', '&'], "_");
        let testcase_dir = self.testcase_dir.join(dir_name);

        if testcase_dir.exists() && testcase_dir.read_dir()?.count() > 0 {
            info!("Using existing testcase for {}", problem_url);
            return Ok(testcase_dir);
        }

        info!("Downloading testcase for {}", problem_url);
        download_testcase(problem_url, &testcase_dir)?;

        Ok(testcase_dir)
    }
}

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

    pub fn problem_url(&self) -> Option<&str> {
        self.test_property.get("problem")
    }

    pub fn run(&self, testcase_dir: &Path) -> Result<()> {
        let solver = example_binary_path(self.solver_path.as_path());

        if !solver.exists() {
            build_example(self.solver_path.as_path())?;
        }

        let mut oj_command = Command::new("oj");
        oj_command
            .arg("test")
            .arg("--directory")
            .arg(testcase_dir.as_os_str())
            .arg("--command")
            .arg(solver);

        if env::consts::OS != "windows" {
            oj_command.arg("--jobs").arg("2");
        }

        // special judge
        if let Some(judge_program_path) = self.judge_program_path() {
            let judge = example_binary_path(judge_program_path.as_path());

            if !judge.exists() {
                build_example(judge_program_path.as_path())?;
            }

            oj_command.arg("--judge-command").arg(judge);
        }

        info!("execute {:?}", oj_command);
        let status = oj_command.status()?;
        ensure!(
            status.success(),
            "failed: oj test for {}",
            self.solver_path.display()
        );

        Ok(())
    }

    fn judge_program_path(&self) -> Option<PathBuf> {
        self.test_property
            .get("judge_program_rs")
            .map(|judge_program_rs| self.solver_path.parent().unwrap().join(judge_program_rs))
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

fn download_testcase(problem_url: &str, testcase_dir: &Path) -> Result<()> {
    if testcase_dir.exists() {
        fs::remove_dir_all(testcase_dir)?;
    }

    let mut oj_command = Command::new("oj");
    oj_command
        .arg("download")
        .arg(problem_url)
        .arg("--directory")
        .arg(testcase_dir.as_os_str())
        .arg("--system")
        .arg("--silent");

    info!("execute {:?}", oj_command);
    let status = oj_command.status()?;
    ensure!(status.success(), "failed: oj download for {}", problem_url);

    Ok(())
}

fn build_example(example_path: &Path) -> Result<()> {
    let example_name = example_path
        .file_stem()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            anyhow::anyhow!("Invalid example file name: {}", example_path.display())
        })?;

    info!("Building example: {}", example_name);

    let mut cargo_command = Command::new("cargo");
    cargo_command
        .arg("build")
        .arg("--release")
        .arg("--example")
        .arg(example_name);

    info!("execute {:?}", cargo_command);
    let status = cargo_command.status()?;
    ensure!(
        status.success(),
        "failed: cargo build --release --example {}",
        example_name
    );

    Ok(())
}

fn cargo_target_examples_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
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
