use std::path::Path;

use oj_test::ProblemSolver;

#[test]
#[ignore]
fn local_testcase() {
    env_logger::init();

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
