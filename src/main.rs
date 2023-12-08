mod commands;
mod parse;
mod args;

mod generate;

mod path;

use std::fs;
use std::process::{Command, exit, Stdio};
use std::io::{Read, Write};
use std::ops::Deref;
use crate::args::Arguments;
use crate::generate::generate_target;
//static delimiter: &str = ".";



#[cfg(test)]
mod tests;

struct Text(Vec<String>);

impl Text {
    fn read(n: usize) {
    }
}

struct CommandCall {
    name: String,
    block: bool,
    parameter: Option<String>,
}

fn combine_texts(added: &str, text: &str) -> String {
    let mut result = String::new();
    result.push_str(added);
    result.push_str(text);
    result
}



fn main() {
    let args = args::parse_args();
    let Ok(args) = args else {
        let err = args.err().unwrap();
        println!("error while parsing arguments: {}", err);
        exit(0);
    };
    println!("file: {}", args.file);
    println!("libs: {:?}", args.libs);
    println!("targets: {:?}", args.targets);

    if let Err(msg) = generate::prepare_filesystem(&args) {
        println!("{}", msg);
        exit(0);
    }

    let input = fs::read(&args.file)
        .unwrap();
    // use utf8 strings
    let source = String::from_utf8(input.clone()).expect("not utf-8");
    let mut input = String::from_utf8(input).unwrap();


    for target in args.targets {
        println!("generating {}", &target);
        let result = generate::generate_target(&args.file, &target, source.clone());
        match result {
            Ok(()) => {
                // TODO: target compiles itself as a file, a Vec<u8> or a String ?
                // generated output is left to the target compiler ?
            }
            Err(err) => {}
        }
    }

    // TODO: check if still needed with new syntax
    input.push('\n'); // needed when parsing the end of a block (".<command>.end\n")

    let mut processed: Vec<String> = Vec::new();


    /*
    while let Some((line, left_text)) = parse::split_line(&input) {
        match parse::parse_section_header(&line) {
            None => {
                processed.push(line);
                // TODO: use a slice instead of a String input (to save some O(n) copying of an entire book)
                input = left_text.to_string();
            }
            /*
            Some(section_header) => {
                match parse_block(section_header, &input) {
                    None => {
                        //
                    }
                    Some(_) => {}
                }
            }
             */
        }

     */
    /*
        match parse_command(&line) {
            None => {
                processed.push(line);
                // O(n), but i have to keep the String in order to add new text to it
                input = left_text.to_string();
            }
            Some(call) => {
                let CommandCall {
                    name, block, parameter
                } = call;

                let (cmd_input, left_text) = match block {
                    true => read_block(left_text, &name),
                    false => read_paragraph(left_text)
                };

                let path = format!("commands/{}", name);
                /*
                if !command_exists(&path) {
                    panic!("undefined command: {}", &path);
                }
                 */

                let mut cmd = Command::new(&path);
                cmd.stdin(Stdio::piped())
                    .stdout(Stdio::piped());
                if let Some(param) = parameter {
                    cmd.arg(param);
                }

                let child_process = cmd.spawn().expect(&format!("Could not call command: {}", &path));
                child_process.stdin.unwrap().write(cmd_input.as_bytes()).unwrap();
                let mut output = String::new();
                child_process.stdout.unwrap().read_to_string(&mut output).unwrap();

                // remove the processed part and replace it with the result of the subprocess
                input = combine_texts(&output, left_text);
            }
        }
    }

     */
    let result = processed.join("\n");
    println!("{}", result);



/*
    let lines: Vec<&str> = stdin.split("\n").collect();

    for (i, line) in lines.into_iter().enumerate() {
        if line.starts_with(".") {
            let mut command = &line[1..];
            let mut input = Vec::new();
            if command.contains(" ") {
                let parts: Vec<&str> = command.splitn(1, " ").collect();
                command = parts[0];
                input.push(parts[1]);
                while lines[]
            }
            let path = format!("commands/{}", command);
            match fs::metadata(path.clone()) {
                Ok(_) => {
                    let output = Command::new(path)
                        //.stdin("test")
                        .output()
                        .unwrap()
                        .stdout;
                    let output = String::from_utf8(output).unwrap();

                }
                Err(_) => {
                    println!("WARNING undefined command at line {}: {}", i, command)
                }
            }
            println!("macro: {}", &line[1..]);
        }
    }
    println!("{}", stdin);
    println!("Hello, world!");

 */
}
