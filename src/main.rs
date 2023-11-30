use std::arch::x86_64::__cpuid;
use std::fmt::format;
use std::fs;
use std::process::{Command, Stdio};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::ops::Deref;
use std::thread::yield_now;

//static delimiter: &str = ".";

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

fn parse_command(mut line: &str) -> Option<CommandCall> {
    if line.len() == 0 || !line.starts_with(".") {
        return None
    }
    // trying to get utf8 best practices right, even if '.' is always 1 byte
    line = &line['.'.len_utf8()..];

    let mut name = String::new();
    let mut block = false; // ".start" suffix after ".<command>"

    // this code needs to keep a &str slice and not just an iterator
    // because of forward peeking
    while let Some(ch) = line.chars().next() {
        line = &line[ch.len_utf8()..];
        match ch {
            '.' => {
                // ".start" suffix ?
                if line.starts_with("start") {
                    // optimistic parsing, may revert to block = false
                    // (example: .<command>.start-logging.sh)
                    block = true;
                    line = &line["start".as_bytes().len()..];
                }
                if line.len() == 0 {
                    return Some(CommandCall {
                        name,
                        block,
                        parameter: None,
                    })
                } else if line.starts_with(" ") {
                    line = &line[' '.len_utf8()..];
                    return Some(CommandCall {
                        name,
                        block,
                        parameter: Some(line.to_string()),
                    })
                } else {
                    // parsing .start as a suffix was a mistake; keep parsing
                    block = false;
                    name.push_str(".start")
                }
            }
            ' ' => {
                // command parameter
                return Some(CommandCall {
                    name,
                    block,
                    parameter: Some(line.to_string()),
                })
            }
            _ => {
                name.push(ch)
            }
        }
    }

    return Some(CommandCall {
        name,
        block,
        parameter: None,
    })
}

fn split_line(mut text: &str) -> Option<(String, &str)> {
    if text.len() == 0 {
        return None
    }
    let mut line = String::new();
    while let Some(ch) = text.chars().next() {
        text = &text[ch.len_utf8()..];
        match ch {
            '\n' => {
                println!("line length: {}", line.len());
                return Some((line, text))
            }
            _ => {
                line.push(ch)
            }
        }
    }
    println!("line length: {}", line.len());
    Some((line, text))
}


fn command_exists(path: &str) -> bool {
    match fs::metadata(path) {
        Ok(_) => true,
        Err(_) => false
    }
}

fn read_block<'a>(mut text: &'a str, cmd: &str) -> (String, &'a str) {
    let mut extracted = String::new();

    let prefix = format!(".{}.end\n", cmd);
    let prefix = prefix.as_str();
    while let Some(ch) = text.chars().next() {
        text = &text[ch.len_utf8()..];
        match ch {
            '\n' if text.starts_with(prefix) => {
                // end of block
                text = &text[prefix.as_bytes().len()..];
                return (extracted, text)
            }
            ch => {
                extracted.push(ch);
            }
        }
    }

    // if the block never ends, everything is considered part of the block
    return (extracted, text)
}

fn read_paragraph(mut text: &str) -> (String, &str) {
    let mut extracted = String::new();
    while let Some(ch) = text.chars().next() {
        text = &text[ch.len_utf8()..];
        match ch {
            '\n' if text.starts_with("\n") => {
                // end of paragraph
                return (extracted, &text[ch.len_utf8()..])
            }
            ch => {
                extracted.push(ch)
            }
        }
    }
    (extracted, text)
}


fn combine_texts(added: &str, text: &str) -> String {
    let mut result = String::new();
    result.push_str(added);
    result.push_str(text);
    result
}




fn main() {
    let input = fs::read("/dev/stdin")
        .unwrap();
    // use utf8 strings
    let mut input = String::from_utf8(input).unwrap();
    input.push('\n'); // needed when parsing the end of a block (".<command>.end\n")

    let mut processed = Vec::new();

    while let Some((line, left_text)) = split_line(&input) {
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
