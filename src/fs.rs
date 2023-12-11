use std::{env, fs, os};
use std::fmt::format;
use std::io::Error;
use std::path::{Path, PathBuf};
use crate::args::Arguments;
use crate::path::{absolute, cd, filename, folder};

/* manage cache directories to resolve library and target binaries */

pub fn setup(args: &mut Arguments) {
    let directory = folder(args.file.as_path());
    let name = filename(args.file.as_path());
    let _cd = cd(directory.as_path());
    fs::create_dir(format!(".{}.tmp", name.display())).expect("could not create cache directory");
}

pub fn cleanup(filepath: &Path) -> Result<(), Error>{
    let folder = folder(filepath);
    let _cd = cd(folder.as_path());
    let filename = filename(filepath);
    fs::remove_dir_all(format!(".{}.tmp", filename.display()))?;
    Ok(())
}

pub fn resolve(command: &str, args: &Arguments) -> Option<PathBuf> {
    for lib in args.libs.iter() {
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
    for target in args.targets.iter() {
        let path = target.join(command);
        match fs::metadata(path.as_path()) {
            Ok(data) => {
                if data.is_file() {
                    return Some(absolute(path.as_path()));
                }
            }
            Err(_) => {}
        }
    }
    None
}


