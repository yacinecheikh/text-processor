use std::fs;
use std::path::{Path, PathBuf};
use crate::args::Arguments;

use crate::{parse};
use crate::commands;
use crate::parse::Section;


pub struct Context<'a> {
    pub outfile: &'a Path,
    pub target: &'a Path, // relative paths are ok
    pub libs: &'a [PathBuf]
}

// TODO: "globalize" current target
// (to avoid giving it as a parameter to every function)
// (alternative: use a symlink in the file system)

pub fn generate_all(args: &Arguments) {
    // read text once
    let input = fs::read(&args.file)
        .expect("could not read file");
    let input = String::from_utf8(input).expect("not valid utf-8");
    let input = input.as_str();

    for target in args.targets.iter() {
        let target_name = target.file_name()
            .expect(format!("not a valid target: {}", target.display()).as_str());
        let output_file = args.file.with_extension(target_name);
        let compilation_context = Context {
            outfile: output_file.as_path(),
            target: target.as_path(),
            libs: args.libs.as_slice(),
        };
        generate_target(compilation_context, input);
    }
}

pub fn generate_target(context: Context, source: &str) {
    //let output_file = config.outfile.with_extension(config.target.file_name());
    match process_file(source, &context) {
        Ok(result) => {
            // last step: compile the result
            // TODO: define as a constant the name of the compiling binary
            match commands::eval(Section{
                name: "generate".to_string(),
                argument: None,
                body: Some(result),
            }, &context) {
                Ok(result) => {
                    fs::write(context.outfile, result)
                        .expect("could not write to output file");
                }
                Err(msg) => {
                    println!("failed to run the target compiler (generate for current target), error: {msg}");
                }
            }
        }
        Err(msg) => {
            // compilation error
            println!("error while compiling: {}", msg);
        }
    }
}

pub fn combine_texts(added: &str, text: &str) -> String {
    let mut result = String::new();
    result.push_str(added);
    result.push_str(text);
    result
}

fn process_file(source: &str, context: &Context) -> Result<String, String> {
    // same value (just using a slice to move forward without cloning the string)
    let mut text = source.to_string();
    let mut left_text = text.as_str();
    // output
    let mut processed = String::new();

    while left_text.len() > 0 {
        let (empty_lines, left) = parse::strip_empty_lines(left_text);
        processed.push_str(empty_lines);
        left_text = left;

        if empty_lines.len() > 0 {
            continue
        }

        if let Some(result) = parse::parse_section(left_text) {
            match result {
                Ok((section, left)) => {
                    left_text = left;
                    // needed for error message
                    let command_name = section.name.clone();
                    match commands::eval(section, context) {
                        Err(e) => {
                            return Err(format!("error when running command {}: {}", command_name, e))
                        }
                        Ok(result) => {
                            text = combine_texts(result.as_str(), left_text);
                            left_text = text.as_str();
                        }
                    }
                    // TODO: use commands.rs to call the binary
                }
                Err(msg) => {
                    // parsing (syntax) error
                    return Err(msg)
                }
            }

            continue
        }
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
    // TODO: write the output to a file with the same name, a different extension, and in the same folder
    Ok(processed)
}