use std::fs;
use std::io::{Error, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Stdio, Command};
use crate::args::Arguments;
use crate::generate::Context;
use crate::parse::Section;
use crate::path::{absolute, cd, filename, parent_folder};


pub fn init(args: &mut Arguments) {
    let directory = parent_folder(args.file.as_path());
    let name = filename(args.file.as_path());
    let _cd = cd(directory.as_path());
    fs::create_dir(format!(".{}.tmp", name.display())).expect("could not create cache directory");
}

pub fn cleanup(filepath: &Path) -> Result<(), Error> {
    let folder = parent_folder(filepath);
    let _cd = cd(folder.as_path());
    let filename = filename(filepath);
    fs::remove_dir_all(format!(".{}.tmp", filename.display()))?;
    Ok(())
}



pub fn resolve(command: &str, context: &Context) -> Option<PathBuf> {
    // is the command in a library ?
    for lib in context.libs.iter() {
        let path = lib.join(command);
        match fs::metadata(path.as_path()) {
            Ok(data) => {
                if data.is_file() {
                    return Some(absolute(path.as_path()));
                }
            }
            Err(_) => {}
        }
    }
    // is the command in the target binaries ?
    let path = context.target.join(command);
    match fs::metadata(path.as_path()) {
        Ok(data) => {
            if data.is_file() {
                return Some(absolute(path.as_path()));
            }
        }
        Err(_) => {}
    }
    None
}

// TODO: remove set_current_dir() (not needed for commands, only once for the source file)

pub fn eval(call: Section, context: &Context) -> Result<String, String> {
    let Section { name, argument, body } = call;
    run(name.as_str(), context, body, argument)
}

fn run(command: &str, context: &Context, input: Option<String>, parameter: Option<String>) -> Result<String, String> {
    let command_path = resolve(command, context)
        .expect(&format!("could not find binary {}", command));
    let _cd = cd(parent_folder(context.outfile).as_path());

    let mut cmd = Command::new(command_path);
    if input.is_some() {
        cmd.stdin(Stdio::piped());
    }
    cmd.stdout(Stdio::piped());
    if let Some(parameter) = parameter {
        cmd.arg(parameter);
    }

    let process = cmd.spawn()
        .expect("could not spawn subprocess, hint: check permissions");

    if input.is_some() {
        process.stdin
            .expect(&format!("could not get the stdin of subprocess {command}"))
            .write(input.unwrap().as_ref())
            .expect(&format!("could not write to the stdin of subprocess {command}"));
    }

    let mut result = String::new();
    process.stdout.unwrap().read_to_string(&mut result).unwrap();

    return Ok(result)
}
