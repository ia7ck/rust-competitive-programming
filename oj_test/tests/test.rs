use std::path::{Path, PathBuf};

use anyhow::Result;

use oj_test::{check_oj_version, exists_artifacts, ProblemSolver, Testcase};

struct LocalTestcase {}

impl Testcase for LocalTestcase {
    fn setup(&self) -> Result<()> {
        Ok(())
    }

    fn testcase_dir(&self) -> PathBuf {
        Path::new(std::env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("hand-made")
    }
}

#[test]
#[ignore]
fn local_testcase() {
    env_logger::init();

    check_oj_version().unwrap();
    exists_artifacts().unwrap();

    let solver = ProblemSolver::new(
        Path::new(std::env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("double.rs")
            .as_path(),
    );
    let testcase = LocalTestcase {};
    solver.run(testcase).unwrap();
}
