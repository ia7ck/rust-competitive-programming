use anyhow::Result;
use glob::glob;
use log::info;

use oj_test::{download_online_judge_testcase, ProblemSolver};

fn main() -> Result<()> {
    env_logger::init();

    let mut solvers = Vec::new();
    for entry in glob("**/examples/*.rs")? {
        let path = entry?;
        // oj download に失敗するのでスキップ
        if path.ends_with("cycle_detection.rs") {
            continue;
        }
        solvers.push(ProblemSolver::new(path.as_path()));
    }
    solvers.sort_by(|s1, s2| s1.solver_path().cmp(s2.solver_path()));

    for s in solvers {
        if let Some(problem_url) = s.problem_url() {
            let dir_suffix = s.solver_path().with_extension("");
            let testcase_dir = download_online_judge_testcase(problem_url, dir_suffix.as_path())?;
            s.run(testcase_dir.as_path())?;
        } else {
            info!("skip {}", s);
        }
    }

    Ok(())
}
