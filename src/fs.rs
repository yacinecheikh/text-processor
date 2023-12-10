use std::{env, fs, os};
use std::fmt::format;
use std::io::Error;
use std::path::Path;
use crate::args::Arguments;
use crate::path::{absolute, cd, filename, folder};

/* manage cache directories to resolve library and target binaries */

pub fn setup(args: &mut Arguments) -> Result<(), String>{
    if let Err(msg) = prepare_folders(args.file.as_path()) {
        return Err(format!("error while creating cache directories: {msg}"))
    }
    for lib in args.libs.iter() {
        add_lib(lib, args.file.as_path())?
    }
    for target in args.targets.iter() {
        //add_target(target)?;
    }
    // TODO: replace relative Paths in Arguments by local (symlink) paths
    // TODO(better): don't symlink every lib, use them directly instead (and use Makefiles)
    // TODO: keep the data store (.{file}.tmp)
    Ok(())
}

// this is almost useless
pub fn cleanup(filepath: &Path) -> Result<(), Error>{
    let folder = folder(filepath);
    let _cd = cd(folder.as_path());
    let filename = filename(filepath);
    fs::remove_dir_all(format!(".{}.generation", filename.display()))?;
    Ok(())
}

pub fn resolve(command: &str) -> String {
    todo!()
}


//TODO: all of this is useless
fn prepare_folders(file: &Path) -> Result<(), Error>{
    let filename = filename(file);
    let filename = filename.to_str().unwrap();
    let folder = folder(file);
    let _cd = cd(folder.as_path());
    fs::create_dir(format!(".{}.generation", filename))?;
    fs::create_dir(format!(".{}.generation/data", filename))?;
    fs::create_dir(format!(".{}.generation/target", filename))?;
    fs::create_dir(format!(".{}.generation/lib", filename))?;
    Ok(())
}


// this too
fn add_lib(lib: &Path, file: &Path) -> Result<(), String> {
    // each lib is an absolute path to a directory of binaries
    let lib_path = absolute(lib);
    //let lib_path = lib_path.to_str().unwrap();
    let metadata = fs::metadata(lib_path.as_path());
    match metadata {
        Ok(meta) => {
            if !meta.is_dir() {
                return Err(format!("library \"{}\" is not a directory", lib_path.display()))
            }
            let file_name = filename(file);
            let file_folder = folder(file);
            let name = filename(lib_path.as_path());
            let _cd = cd(file_folder.join(format!(".{}.generation/lib", file_name.display())).as_path());
            match os::unix::fs::symlink(lib_path.as_path(), name.clone()) {
                Ok(_) => {}
                Err(err) => {
                    println!("libpath: {}", lib_path.as_path().display());
                    println!("link: {}", name.as_path().display());
                    println!("cwd: {}", env::current_dir().unwrap().display());
                    return Err(format!("could not link to library \"{err}\""))
                }
            }
        }
        Err(err) => {
            return Err(format!("could not access library \"{}\"", lib_path.display()))
        }
    }
    Ok(())
}

// this too
fn add_target(target: &Path) {
    todo!()
}