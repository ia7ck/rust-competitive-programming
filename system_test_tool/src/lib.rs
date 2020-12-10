use md5::{Digest, Md5};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;
use std::{env, thread};

pub fn system_test<F>(solve: F, problem_url: &str)
where
    F: 'static + Fn(&str, &mut String) + Send + Clone,
{
    let td = TestcaseDir::new(problem_url);
    td.download_testcase(problem_url);

    let mut inputs = td.testcase("in");
    let mut outputs = td.testcase("out");
    inputs.sort();
    outputs.sort();

    let mut handles = vec![];
    for (input, output) in inputs.into_iter().zip(outputs.into_iter()) {
        assert_eq!(input.file_stem(), output.file_stem());
        let input_string = fs::read_to_string(&input).unwrap();
        let output_string = fs::read_to_string(&output).unwrap();
        let solve = solve.clone();
        let h = thread::spawn(move || {
            let mut result = String::new();
            let now = Instant::now();
            solve(&input_string, &mut result);
            let duration = now.elapsed();

            if result.trim() != output_string.trim() {
                assert!(
                    false,
                    "Wrong Answer: input={}, output={}",
                    input.display(),
                    output.display()
                );
            }

            println!(
                "testcase {} takes {} ms",
                input.display(),
                duration.as_millis()
            );
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }
}

struct TestcaseDir {
    dir: PathBuf,
}

impl TestcaseDir {
    pub fn new(seed: &str) -> Self {
        let mut dir = env::temp_dir();
        dir.push(format!("{:x}", Md5::digest(seed.as_bytes())));
        Self { dir }
    }
    pub fn dir(&self) -> &Path {
        self.dir.as_path()
    }
    pub fn download_testcase(&self, problem_url: &str) {
        self.clear();
        let status = Command::new("oj")
            .arg("download")
            .arg(problem_url)
            .arg("--directory")
            .arg(self.dir().as_os_str())
            .arg("--system")
            .arg("--silent")
            .status()
            .expect("failed to start oj");
        assert!(
            status.success(),
            "`oj dl {} -d {}` failed",
            problem_url,
            self.dir().display()
        );
    }
    fn clear(&self) {
        if self.dir().exists() {
            fs::remove_dir_all(self.dir()).unwrap_or_else(|e| {
                panic!("dir={}, {}", self.dir().display(), e);
            });
        }
    }
    pub fn testcase(&self, ext: &str) -> Vec<PathBuf> {
        let ends_with = |p: &PathBuf, ext: &str| p.extension().eq(&Some(OsStr::new(ext)));
        fs::read_dir(self.dir())
            .unwrap_or_else(|e| {
                panic!("dir={}, {}", self.dir.display(), e);
            })
            .map(|dir_entry| dir_entry.unwrap().path())
            .filter(|p| ends_with(p, ext))
            .collect()
    }
}
