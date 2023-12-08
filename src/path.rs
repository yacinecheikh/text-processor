use std::env;
use std::fmt::format;
use std::path::{Path, PathBuf};

pub fn absolutize(path: &str) -> PathBuf {
    Path::new(path).join(env::current_dir().unwrap())
}

pub fn folder(path: &str) -> &Path {
    Path::new(path).parent().unwrap()
}

pub fn set_extension(path: &str, extension: &str) -> PathBuf {
    let path = Path::new(path);
    return path.with_extension(extension)
}

pub struct Cd {
    previous_directory: PathBuf,
}

impl Drop for Cd {
    fn drop(&mut self) {
        std::env::set_current_dir(&self.previous_directory).unwrap()
    }
}

pub fn cd(directory: &str) -> Cd {
    let result =  Cd {
        previous_directory: std::env::current_dir().unwrap(),
    };
    env::set_current_dir(directory).unwrap();
    result
}