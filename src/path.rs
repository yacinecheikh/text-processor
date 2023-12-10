use std::env;
use std::path::{Path, PathBuf};

/* 100% noob ugly code, please don't hit me */

pub fn absolute(path: &Path) -> PathBuf {
    env::current_dir().unwrap().join(path)
}

pub fn filename(path: &Path) -> PathBuf {
    PathBuf::from(path.file_name().unwrap())
}

pub fn folder(path: &Path) -> PathBuf {
    let abs = absolute(path);
    abs.parent().unwrap().to_path_buf()
    //return abs.parent().unwrap()//.to_str().unwrap().to_string()
}

pub fn set_extension(path: &Path, extension: &str) -> PathBuf {
    return path.with_extension(extension)
}

pub struct Cd {
    previous_directory: PathBuf,
}

impl Drop for Cd {
    fn drop(&mut self) {
        env::set_current_dir(&self.previous_directory).unwrap()
    }
}

pub fn cd(directory: &Path) -> Cd {
    let result =  Cd {
        previous_directory: env::current_dir().unwrap(),
    };
    env::set_current_dir(directory).unwrap();
    result
}
