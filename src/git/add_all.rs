use std::process::Command;

use super::runner::run_git_command;

pub(crate) fn add_all() {
    let output = Command::new("git")
        .arg("add")
        .arg("--all")
        .output()
        .expect("failed to execute addAll");

    let result = run_git_command(output);
    println!("{}", result);
}
