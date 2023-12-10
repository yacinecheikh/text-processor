use std::env;
use std::path::PathBuf;
use std::string::ToString;

pub struct Arguments {
    pub libs: Vec<PathBuf>,
    pub targets: Vec<PathBuf>,
    pub file: PathBuf,
}

static DEFAULT_FILE: &str = "/dev/stdin";

pub fn parse_args() -> Result<Arguments, String> {
    let mut arguments = Arguments {
        libs: vec![],
        targets: vec![],
        file: PathBuf::from(DEFAULT_FILE),
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
                            arguments.libs.push(PathBuf::from(lib));
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
                            arguments.targets.push(PathBuf::from(target));
                        }
                    }
                }
            }
            _ => {
                if arguments.file == PathBuf::from(DEFAULT_FILE) {
                    arguments.file = PathBuf::from(arg);
                } else {
                    return Err(format!("Already processing file {}", arguments.file.to_str().unwrap()));
                }
            }
        }
    }
    Ok(arguments)
}