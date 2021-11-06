use std::{env, ops};

use anyhow::{anyhow, ensure, Result};
use glob::glob;
use log::info;

use oj_test::{check_oj_version, OnlineJudgeTestcase, ProblemSolver};

const CHUNK_LENGTH: usize = 10;

fn target_chunk_range(args: &[String]) -> Result<ops::Range<usize>> {
    ensure!(args.len() >= 2);
    let arg: Vec<&str> = args[1].split('-').collect();
    ensure!(arg.len() == 2);
    let start = arg[0].parse::<usize>()?;
    let end = arg[1].parse::<usize>()?;
    ensure!(start <= CHUNK_LENGTH);
    ensure!(end <= CHUNK_LENGTH);
    Ok(start..end)
}

fn main() -> Result<()> {
    env_logger::init();

    check_oj_version()?;

    let args: Vec<String> = env::args().collect();
    let target_range = target_chunk_range(&args).map_err(|_| {
        anyhow!(concat!(
            "Usage examples:\n",
            "$ cargo run --bin oj_test -- 0-10 # test all\n",
            "$ cargo run --bin oj_test -- 0-5  # test only first half\n",
            "$ cargo run --bin oj_test -- 8-10 # test only last one fifth"
        ))
    })?;

    let mut solvers = Vec::new();
    for entry in glob("**/examples/*.rs")? {
        let path = entry?;
        solvers.push(ProblemSolver::new(path.as_path()));
    }
    solvers.sort_by(|s1, s2| s1.solver_path().cmp(s2.solver_path()));

    let chunk_size = solvers.len() / CHUNK_LENGTH + 1;
    info!("chunk size = {}", chunk_size);
    for (i, solvers) in solvers.chunks(chunk_size).enumerate() {
        info!("chunk #{} {:?}", i, solvers);
        if target_range.contains(&i) {
            for s in solvers {
                if let Some(problem_url) = s.problem_url() {
                    let testcase_dir = env::temp_dir().join(s.solver_path().with_extension(""));
                    let testcase = OnlineJudgeTestcase::new(testcase_dir.as_path(), problem_url);
                    s.run(testcase)?;
                } else {
                    info!("skip {:?}", s);
                }
            }
        }
    }

    Ok(())
}
