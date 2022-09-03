use std::thread;
use std::thread::JoinHandle;

use anyhow::Result;
use glob::glob;
use log::info;

use oj_test::{download_online_judge_testcase, ProblemSolver};

fn main() -> Result<()> {
    env_logger::init();

    let mut solvers = Vec::new();
    for entry in glob("**/examples/*.rs")? {
        let path = entry?;
        solvers.push(ProblemSolver::new(path.as_path()));
    }
    solvers.sort_by(|s1, s2| s1.solver_path().cmp(s2.solver_path()));

    let mut handles: Vec<JoinHandle<Result<()>>> = Vec::new();
    for solvers_chunk in solvers.chunks(5) {
        let solvers_chunk = solvers_chunk.to_vec();
        let h = thread::spawn(move || {
            for s in solvers_chunk {
                if let Some(problem_url) = s.problem_url() {
                    let dir_suffix = s.solver_path().with_extension("");
                    let testcase_dir =
                        download_online_judge_testcase(problem_url, dir_suffix.as_path())?;
                    s.run(testcase_dir.as_path())?;
                } else {
                    info!("skip {}", s);
                }
            }
            Ok(())
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap()?;
    }

    Ok(())
}
