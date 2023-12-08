use crate::parse::Command::{Block, Oneliner};

enum CallHeader {
    // syntaxes:
    // .command:\n (waiting for a block, indented or not)
    // .command: arg\n (may have a block, or not)
    // .command\n (not waiting for a block)
    Block {
        name: String,
        base_indent: String,
    },
    Oneliner {
        name: String,
    },
    Mixed {
        // may include a block, but it has to be indented for clarity
        name: String,
        base_indent: String,
        argument: String,
    }
}

// TODO: remove
struct CommandLine {
    // first line of what can be an indented block
    base_indent: String,
    name: String,
    argument: Option<String>,
}

struct Call {
    name: String,
    body: Option<String>,
    argument: Option<String>,
}

// TODO: remove
enum Command {
    // completely parsed call
    Oneliner {
        name: String,
        arg: Option<String>,
    },
    Block {
        name: String,
        arg: Option<String>,
        content: String,
        // indent ?
    },
}


/* generic string parsing utilities*/


fn split(text: &str, offset: usize) -> (&str, &str) {
    (&text[..offset], &text[offset..])
}


pub fn split_indent(text: &str) -> (&str, &str) {
    let mut offset = 0;
    for ch in text.chars() {
        match ch {
            ' ' | '\t' => {
                offset += ch.len_utf8();
            }
            _ => {
                break
            }
        }
    }
    split(text, offset)
}


pub fn split_line(text: &str) -> Option<(&str, &str)> {
    let mut offset = 0;
    for ch in text.chars() {
        match ch {
            '\n' => {
                // the \n should be removed, but this requires testing
                offset += ch.len_utf8();
                break
            }

            _ => {
                offset += ch.len_utf8()
            }
        }
    }
    if offset == 0 {
        // no char left in text
        None
    } else {
        Some(split(text, offset))
    }
}

fn is_empty(line: &str) -> bool {
    for ch in line.chars() {
        match ch {
            ' ' | '\t' => {},
            '\n' => return true,
            _ => return false,
        }
    }
    return true
}

pub fn split_empty_lines(mut text: &str) -> (&str, &str) {
    let mut offset = 0;
    // only change after a complete line is parsed
    let mut committed_offset = 0;
    for ch in text.chars() {
        match ch {
            '\t' | ' ' => {
                offset += ch.len_utf8()
            }
            '\n' => {
                offset += ch.len_utf8();
                committed_offset = offset
            }
            _ => {
                break
            }
        }
    }
    split(text, committed_offset)
}

static DELIMITER: &str = ".";

pub fn parse_command_line(line: &str) -> Option<CommandLine> {
    let (indent, line) = split_indent(line);
    if !line.starts_with(DELIMITER) {
        return None
    }
    // strip the prefix
    let line = &line[DELIMITER.as_bytes().len()..];
    let mut call = CommandLine {
        base_indent: indent,
        name: "".to_string(),
        argument: None,
        block_syntax: false,
    };
    if !line.contains(':') {
        // not expecting a block nor a parameter
    }
    match line.split_once(": ") {
        None => {
            // no parameter, no block
            call.name = line.to_string();
        }
        Some((name, arg)) => {
            call.name = name.to_string();
            if !is_empty(arg) {
                call.argument = Some(arg.to_string());
            }
        }
    }
    return Some(call)
}

pub fn parse_command_block(command_header: CommandLine, text: &str) -> Result<(Command, &str), String> {
    let (empty, left_text) = split_empty_lines(text);
    match split_line(left_text) {
        None => {
            if let Some(_) = command_header.argument {
                // no need for a block
                return Ok((Oneliner {
                    name: command_header.name,
                    arg: command_header.argument,
                }, text))
            } else {
                // should expect a block, but no block was found
                // -> send empty lines (empty block),
                // and send a warning
                println!("Warning: only empty lines found in block under command {}", &command_header.name);
                return Ok((Block {
                    name: command_header.name,
                    arg: command_header.argument,
                    content: empty.to_string(),
                }, left_text))
            }
        }
        Some((line, left_text)) => {
            /*
            let block = Block {
                name: command_header.name,
                arg: command_header.parameter,
                content: "".to_string(),
            };

             */
            let (indent, line) = split_indent(line.as_str());
            if !indent.starts_with(command_header.base_indent) {
                match command_header.argument {
                    Some(_) => {
                        // actually not a block, but a oneliner
                        return Ok((Oneliner {
                            name: command_header.name,
                            arg: command_header.argument,
                        }, text))
                    }

                    None => {
                        //
                    }
                }
            }

            //block.content
            return Ok((block, left_text))
        }
    }
    None
}

pub fn parse_block(section: SectionHeader, text: &str) -> Option<(Section, &str)> {
    match split_line(text) {
        // no text left -> inline
        None => {
            None
        }
        Some((line, mut text)) => {
            let (indent, rest) = split_indent(&line);
            let mut section = Section {
                header: section,
                indent,
                content: rest.to_string(),
            };
            while let Some((line, next)) = split_line(text) {
                let (indent, rest) = split_indent(&line);
                if indent.starts_with(&section.indent) {
                    // the line is in the block
                    section.content.push_str(rest);
                    text = next;
                } else {
                    // end of block (before the text ends)
                    return Some((section, text))
                }
            }
            // no line left -> end of block
            Some((section, text))
        }
    }
}

