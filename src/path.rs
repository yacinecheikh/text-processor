use std::fmt::format;
use std::path::{Path, PathBuf};

pub fn folder(path: &str) -> &Path {
    Path::new(path).parent().unwrap()
}

pub fn set_extension(path: &str, extension: &str) -> String {
    let name = std::path::Path::new(path).file_stem().unwrap();
    let folder = folder(path);
    let result = format!("{}/{}.{}", folder.to_str().unwrap(), name.to_str().unwrap(), extension);
    return result
}

struct Cd {
    previous_directory: PathBuf,
}

impl Drop for Cd {
    fn drop(&mut self) {
        std::env::set_current_dir(&self.previous_directory).unwrap()
    }
}

fn cd(directory: &str) -> Cd {
    return Cd {
        previous_directory: std::env::current_dir().unwrap(),
    }
}