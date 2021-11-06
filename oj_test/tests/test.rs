use std::path::{Path, PathBuf};

use anyhow::Result;

use oj_test::{ProblemSolver, Testcase};

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
fn local_testcase() {
    let solver = ProblemSolver::new(
        Path::new(std::env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("double.rs")
            .as_path(),
    );
    let testcase = LocalTestcase {};
    let result = solver.run(testcase);
    assert!(result.is_ok());
}
