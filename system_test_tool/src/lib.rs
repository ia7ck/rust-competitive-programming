use md5::{Digest, Md5};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub trait Solution {
    fn solve(&self, input: &str) -> String;
    fn problem_url(&self) -> &'static str;
}

pub fn system_test<S: Solution>(solution: &S) {
    let url = solution.problem_url();
    let td = TestcaseDir::new(url);
    td.download_testcase(url);

    let mut inputs = td.testcase("in");
    let mut outputs = td.testcase("out");
    inputs.sort();
    outputs.sort();

    let read_to_string = |path: &Path| {
        fs::read_to_string(path).expect(&format!("failed to read {}", path.display()))
    };

    for (input, output) in inputs.iter().zip(outputs.iter()) {
        let input_string = read_to_string(input);
        let output_string = read_to_string(output);
        let actual = solution.solve(&input_string);
        if actual.trim() != output_string.trim() {
            assert!(
                false,
                "Wrong Answer: input={}, output={}",
                input.display(),
                output.display()
            );
        }
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
            fs::remove_dir_all(self.dir())
                .expect(&format!("failed to remove {}", self.dir().display()));
        }
    }
    pub fn testcase(&self, ext: &str) -> Vec<PathBuf> {
        let ends_with = |p: &PathBuf, ext: &str| p.extension().eq(&Some(OsStr::new(ext)));
        fs::read_dir(self.dir())
            .expect(&format!(
                "failed to read directory `{}`",
                self.dir().display()
            ))
            .map(|dir_entry| dir_entry.unwrap().path())
            .filter(|p| ends_with(p, ext))
            .collect()
    }
}
