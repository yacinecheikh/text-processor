use std::env;
use std::fmt::Error;
use std::string::ToString;

pub struct Arguments {
    pub libs: Vec<String>,
    pub targets: Vec<String>,
    pub file: String,
}

static DEFAULT_FILE: &str = "/dev/stdin";

pub fn parse_args() -> Result<Arguments, String> {
    let mut arguments = Arguments {
        libs: vec![],
        targets: vec![],
        file: DEFAULT_FILE.to_string(),
    };
    let mut args = env::args();
    args.next(); // skip args[0]
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--lib" => {
                match args.next() {
                    None => {
                        return Err("expected argument after --lib".to_string())
                    }
                    Some(arg) => {
                        for lib in arg.split(",") {
                            arguments.libs.push(lib.to_string());
                        }
                    }
                }
            }
            "--target" => {
                match args.next() {
                    None => {
                        return Err("expected argument after --target".to_string())
                    }
                    Some(arg) => {
                        for target in arg.split(",") {
                            arguments.targets.push(target.to_string());
                        }
                    }
                }
            }
            _ => {
                if arguments.file == DEFAULT_FILE {
                    arguments.file = arg;
                } else {
                    return Err(format!("Already processing file {}", arguments.file));
                }
            }
        }
    }
    Ok(arguments)
}