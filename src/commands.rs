use std::env::{set_current_dir, current_dir};
use std::fmt::format;
use std::io::{Read, Stdin, Write};
use std::process::{Stdio, Command};

fn list_commands(text_path: &str) -> Vec<String> {
    return Vec::new()
}

fn call(path: &str, input: &str, args: Option<String>, working_directory: &str) -> String {
    let dir = current_dir().expect("could not get current working directory");
    set_current_dir(working_directory).expect("TODO: panic message");

    let mut cmd = Command::new(path);
    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped());
    if let Some(args) = args {
        cmd.arg(args);
    }
    let process = cmd.spawn().expect(&format!("could not start process: {}", path));
    process.stdin.expect(&format!("could not access the stdin pipe of subprocess: {}", path))
        .write(input.as_ref()).expect("could not write to stdin of subprocess");

    let mut result = String::new();
    process.stdout.unwrap().read_to_string(&mut result).unwrap();

    set_current_dir(dir).expect("TODO: panic message");
    return result
}