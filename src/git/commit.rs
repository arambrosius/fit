use std::process::Command;

use super::runner::run_git_command;

pub(crate) fn commit(message: &str, no_verify: bool) {
    let mut args = ["commit", "-m", message].to_vec();

    if no_verify {
        args.push("--no-verify");
    }

    let output = Command::new("git")
        .args(args)
        .output()
        .expect("failed to execute commit");

    let result = run_git_command(output);
    println!("{}", result)
}
