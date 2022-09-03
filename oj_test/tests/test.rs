use std::path::Path;

use oj_test::{check_oj_version, exists_artifacts, ProblemSolver};

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
    let testcase_dir = Path::new(std::env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("hand-made");

    solver.run(testcase_dir.as_path()).unwrap();
}
