use std::fmt::format;
use std::fs;
use std::io::Error;
use std::path::Path;
use crate::args::Arguments;

use crate::{parse, path};
use crate::external;
use crate::parse::Section;
use crate::path::{cd, filename};


pub fn generate_target(file: &Path, target: &Path, text: String) -> Result<(), String> {
    // TODO

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

fn combine_texts(added: &str, text: &str) -> String {
    let mut result = String::new();
    result.push_str(added);
    result.push_str(text);
    result
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