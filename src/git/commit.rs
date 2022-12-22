use std::process::Command;

pub(crate) fn commit(message: &str, no_verify: bool) {
    let mut args = ["commit", "-m", message].to_vec();

    if no_verify {
        args.push("-n");
    }

    let output = Command::new("git")
        .args(args)
        .spawn()
        .expect("failed to execute commit");

    println!("{:?}", output.stdout)
}
