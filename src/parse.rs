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

pub fn parse_header(line: &str) -> Option<CallHeader> {
    let (indent, line) = split_indent(line);
    if !line.starts_with(DELIMITER) {
        return None
    }
    let (_, line) = split(line, DELIMITER.as_bytes().len());

    if !line.contains(":") {
        return Some(CallHeader::Oneliner {
            // TODO: strip right ? (or at least remove the \n if still here)
            name: line.to_string(),
        })
    }
    let (start, end) = line.split_once(":").unwrap();
    // TODO: remove the trailing \n in split_line
    if end.len() == 0 {
        return Some(CallHeader::Block {
            name: start.to_string(),
            base_indent: indent.to_string(),
        })
    }
    // support both ".title: Title" and ".title:Title"
    let end = end.strip_prefix(" ").unwrap_or(end);
    return Some(CallHeader::Mixed {
        name: start.to_string(),
        base_indent: indent.to_string(),
        argument: end.to_string(),
    })
}



pub fn parse_command(header: CallHeader, text: &str) -> Result<(Call, &str), String> {
    match header {
        CallHeader::Oneliner {
            name
        } => {
            // no block to parse
            return Ok((Call {
                name,
                body: None,
                argument: None,
            }, text))
        }
        CallHeader::Block {
            name,
            base_indent
        } => {
        }
        CallHeader::Mixed {
            name, base_indent, argument
        } => {//
        }
    }
    Err("test".to_string())
}


// TODO: this code is too complex because of me trying to guess the indent level while parsing
//  (instead of parsing two times to separate tasks)
fn parse_block2<'a>(text: &'a str, base_indent: Option<&str>) -> Option<(String, &'a str)>{
    // find first indented line to get the indent level
    let (empty, left) = split_empty_lines(text);
    let result = split_line(left);
    if result.is_none() {
        // nothing to parse -> no block to return
        // (for Block: make an empty block and print a warning)
        // (TODO, not in this function)
        return None
    }
    let (first_line, left) = result.unwrap();
    let (indent, line) = split_indent(first_line);
    // nothing that can be parsed -> no block to return
    // same as above
    if let Some(base_indent) = base_indent {
        if indent.starts_with(base_indent) {
            // the first line does not belong to the block
            // -> warning + empty block if needed (Block), None if not (Mixed)
            return None
        }
    }

    // from this point, the first line either follows the previous indentation level,
    // or there was no indentation before
    let mut block: String = String::new();
    block.push_str(empty);
    block.push_str(first_line);
    let mut text = left;
    while true { // sorry
        let (empty, left) = split_empty_lines(text);
        match split_line(left) {
            None => {
                // end of block
                return Some((block, text))
            }
            Some((full_line, left)) => {
                let (indent, line) = split_indent(full_line);
                if !indent.starts_with(base_indent) {
                    // end of block
                    return Some((block, text))
                }
                block.push_str(empty);
                block.push_str(full_line);
                text = left;
            }
        }
    }
    None // never used
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

