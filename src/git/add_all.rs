use std::process::Command;

pub(crate) fn add_all() {
    let output = Command::new("git")
        .arg("add")
        .arg("--all")
        .output()
        .expect("failed to execute addAll");

    println!("{:?}", output.stdout)
}
