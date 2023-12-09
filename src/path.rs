use std::env;
use std::path::{Path, PathBuf};

/* 100% noob ugly code, please don't hit me */

pub fn absolute(path: &str) -> PathBuf {
    env::current_dir().unwrap().join(path)
}

pub fn filename(path: &str) -> &str {
    Path::new(path).file_name().unwrap().to_str().unwrap()
}

pub fn folder(path: &str) -> String {
    let abs = absolute(path);
    return abs.parent().unwrap().to_str().unwrap().to_string()
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
        previous_directory: env::current_dir().unwrap(),
    };
    env::set_current_dir(directory).unwrap();
    result
}
