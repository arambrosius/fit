use std::{process::Output, str};

pub(crate) fn run_git_command(output: Output) -> String {
    let err_string = match str::from_utf8(&output.stderr) {
        Ok(v) => v.trim(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let out_string = match str::from_utf8(&output.stdout) {
        Ok(v) => v.trim(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    if !err_string.is_empty() {
        eprintln!("{}", err_string);
    }

    return out_string.to_string();
}
