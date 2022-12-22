use super::runner::run_git_command;
use std::process::Command;

pub(crate) fn pull(branch_name: String) {
    let output = Command::new("git")
        .arg("pull")
        .arg("--ff-only")
        .arg("origin")
        .arg(branch_name)
        .output()
        .expect("failed to execute pull");

    let result = run_git_command(output);

    println!("{}", result);
}
