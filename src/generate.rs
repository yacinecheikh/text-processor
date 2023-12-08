use std::fmt::format;
use std::fs;
use std::io::Error;
use crate::args::Arguments;

use crate::parse;
use crate::commands;
use crate::parse::Section;


pub fn prepare_filesystem(args: &Arguments) -> Result<(), String> {
    // ignore error
    let filename = args.file.as_str();
    let _ = fs::create_dir(format!(".{}.generation", filename));
    let _ = fs::create_dir(format!(".{}.generation/data", filename));
    let _ = fs::create_dir(format!(".{}.generation/targets", filename));
    let _ = fs::create_dir(format!(".{}.generation/libs", filename));

    prepare_libs(args)?;
    Ok(())
}

pub fn prepare_libs(args: &Arguments) -> Result<(), String> {
    for lib in args.libs.iter() {
        // each lib is an absolute path to a directory of binaries
        let metadata = fs::metadata(lib.as_str());
        match metadata {
            Ok(meta) => {
                todo!()
            }
            Err(err) => {
                return Err(format!("could not find library \"{}\"", lib))
            }
        }

    }
    Ok(())
}

pub fn generate_target(file: &str, target: &str) -> Result<(), String> {
    // TODO

    let text = "".to_string();

    match process_file(text) {
        Ok(text) => {
            // TODO: write to a file
        }
        Err(msg) => {
            return Err(msg)
        }
    }

    Err("TODO".to_string())

    // TODO
}

fn process_file(mut text: String) -> Result<String, String> {
    let mut left_text = text.as_str();
    let mut processed = String::new();

    while left_text.len() > 0 {
        let (empty_lines, left) = parse::strip_empty_lines(left_text);
        processed.push_str(empty_lines);
        left_text = left;

        if let Some(result) = parse::parse_section(left_text) {
            match result {
                Ok((section, text)) => {
                    left_text = text
                    // TODO: use commands.rs to call the binary
                }
                Err(msg) => {
                    // parsing (syntax) error
                    return Err(msg)
                }
            }
        } else {
            match parse::strip_line(left_text) {
                None => {
                    // end of text, should not happen
                    panic!("got no text while calling strip_line() when left_text.len() > 0")
                }
                Some((line, text)) => {
                    left_text = text;
                    processed.push_str(line)
                }
            }
        }
    }
    // TODO: write the output to a file with the same name, a different extension, and in the same folder
    Ok(processed)
}