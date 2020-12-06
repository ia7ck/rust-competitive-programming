use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub trait Solution {
    fn solve(&self, input: &str) -> String;
    fn problem_url(&self) -> &'static str;
    fn testcase_dir(&self) -> &'static str;
}

pub fn system_test<S: Solution>(solution: &S) {
    let url = solution.problem_url();
    let dir = solution.testcase_dir();
    // .in, .out ファイルを入れるディレクトリを作成
    fs::create_dir(dir).expect(&format!("failed to create directory `{}`", dir));
    let status = Command::new("oj")
        .args(&["download", url])
        .args(&["--directory", dir])
        .arg("--system")
        .arg("--silent")
        .status()
        .expect("failed to run command");
    assert!(status.success(), "oj dl failed");
    let mut all: Vec<PathBuf> = fs::read_dir(dir)
        .expect(&format!("failed to read directory `{}`", dir))
        .map(|dir_entry| dir_entry.unwrap().path())
        .collect();
    all.sort();
    let ends_with = |p: &PathBuf, ext: &str| p.extension().unwrap().eq(ext);
    let collect_by_extension = |ext: &str| {
        all.iter()
            .filter(|&p| ends_with(p, ext))
            .collect::<Vec<_>>()
    };
    let inputs = collect_by_extension("in");
    let outputs = collect_by_extension("out");
    for (&input, &output) in inputs.iter().zip(outputs.iter()) {
        let input_string = fs::read_to_string(input)
            .expect(&format!("failed to read input file `{}`", input.display()));
        let output_string = fs::read_to_string(output).expect(&format!(
            "failed to read output file `{}`",
            output.display()
        ));
        assert_eq!(
            solution.solve(&input_string).trim(),
            output_string.trim(),
            "input file = {}\nBefore run test again, remove the testcase directory.",
            input.display()
        );
    }
    // 全てのケースに通ったら .in, .out が入っているディレクトリを消す
    fs::remove_dir_all(dir).expect(&format!("failed to remove directory `{}`", dir));
}
