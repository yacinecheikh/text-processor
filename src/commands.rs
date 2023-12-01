use std::env::{set_current_dir, current_dir};
use std::fmt::format;
use std::io::{Read, Stdin, Write};
use std::os::unix::raw::mode_t;
use std::process::{Stdio, Command};

// TODO: remove set_current_dir() (not needed for commands, only once for the source file)


fn init_commands(text_path: &str, libs: Vec<String>, targets: Vec<String>) {
    let path = std::path::Path::new(text_path);
    let parent = path.parent();
    println!("{:?}", parent);
}


fn list_commands(text_path: &str) -> Vec<String> {
    return Vec::new()
}

// find path
fn find(command: &str) -> String {
    String::new()
}

fn call(path: &str, input: &str, args: Option<String>, working_directory: &str) -> Option<String> {
    let dir = current_dir().ok()?;
    set_current_dir(working_directory).ok()?;

    let mut cmd = Command::new(path);
    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped());
    if let Some(args) = args {
        cmd.arg(args);
    }
    let process = cmd.spawn().ok()?;
    process.stdin?.write(input.as_ref()).ok()?;

    let mut result = String::new();
    process.stdout?.read_to_string(&mut result).ok()?;

    set_current_dir(dir).ok()?;
    return Some(result)
}

fn command_exists(path: &str) -> bool {
    false
}
