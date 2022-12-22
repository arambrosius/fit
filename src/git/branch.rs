use super::runner::run_git_command;
use std::process::Command;

pub(crate) fn get_current_branch_name() -> String {
    let output = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()
        .expect("failed to execute show current branch name");

    return run_git_command(output);
}

// pub(crate) fn get_all_local_branches() -> Vec<String> {
//     let output = Command::new("git")
//         .arg("branch")
//         .output()
//         .expect("failed to execute show all local branches");

//     let all_branches = run_git_command(output);

//     return all_branches
//         .split('\n')
//         .map(|s| s.trim().to_string())
//         .collect();
// }

// pub(crate) fn move_branch(new_name: String) {
//     let output = Command::new("git")
//         .arg("branch")
//         .arg("-m")
//         .arg(new_name)
//         .output()
//         .expect("failed to execute move branch");

//     run_git_command(output);
// }

// pub(crate) fn delete_branch(branch_name: String) {
//     let output = Command::new("git")
//         .arg("branch")
//         .arg("-D")
//         .arg(branch_name)
//         .output()
//         .expect("failed to execute delete branch");

//     run_git_command(output);
// }

pub(crate) fn switch_branch(
    branch_name: &str,
    is_new: bool,
    should_force: bool,
    should_detach: bool,
) {
    let mut args = ["switch"].to_vec();

    if is_new {
        args.push("-c");
    }
    if should_force {
        args.push("-f");
    }
    if should_detach {
        args.push("-d");
    }

    args.push(branch_name);

    let output = Command::new("git")
        .args(args)
        .output()
        .expect("failed to execute force create branch");

    run_git_command(output);
}

// pub(crate) fn force_checkout_branch(branch_name: String, sha: String) {
//     let output = Command::new("git")
//         .arg("switch")
//         .arg("-C")
//         .arg(branch_name)
//         .arg(sha)
//         .output()
//         .expect("failed to execute force checkout branch");

//     run_git_command(output);
// }

// pub(crate) fn force_create_branch(branch_name: String, sha: String) {
//     let output = Command::new("git")
//         .arg("branch")
//         .arg("-f")
//         .arg(branch_name)
//         .arg(sha)
//         .output()
//         .expect("failed to execute force create branch");

//     run_git_command(output);
// }
