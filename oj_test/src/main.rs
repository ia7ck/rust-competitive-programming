use std::env;

use anyhow::Result;
use glob::glob;
use log::info;

use oj_test::{OnlineJudgeTestcase, ProblemSolver};

fn main() -> Result<()> {
    env_logger::init();

    let mut solvers = Vec::new();
    for entry in glob("**/examples/*.rs")? {
        let path = entry?;
        solvers.push(ProblemSolver::new(path.as_path()));
    }
    solvers.sort_by(|s1, s2| s1.solver_path().cmp(&s2.solver_path()));
    for s in solvers {
        if let Some(problem_url) = s.problem_url() {
            let testcase_dir = env::temp_dir().join(s.solver_path().with_extension(""));
            let testcase = OnlineJudgeTestcase::new(testcase_dir.as_path(), problem_url);
            s.run(testcase)?;
        } else {
            info!("skip {:?}", s);
        }
    }

    Ok(())
}
