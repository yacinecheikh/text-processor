
// syntaxes:
// .command:\n (waiting for a block, indented or not)
// .command: arg\n (may have a block, or not)
// .command\n (not waiting for a block)
struct Tag {
    name: String,
    accept_block: bool, // true if ":" was found before the end of line
    base_indent: String, // indentation at the head line
    argument: Option<String>, // if an argument is found, having a block after : is optional and requires further indentation
}
pub struct Section {
    name: String,
    argument: Option<String>,
    body: Option<String>,
}



/* generic string parsing utilities*/


fn split(text: &str, offset: usize) -> (&str, &str) {
    (&text[..offset], &text[offset..])
}


fn strip_indent(line: &str) -> (&str, &str) {
    let mut offset = 0;
    for ch in line.chars() {
        match ch {
            ' ' | '\t' => {
                offset += ch.len_utf8();
            }
            _ => {
                break
            }
        }
    }
    split(line, offset)
}


pub fn strip_line(text: &str) -> Option<(&str, &str)> {
    let mut offset = 0;
    for ch in text.chars() {
        match ch {
            '\n' => {
                // the \n should be removed, but this requires testing
                //offset += ch.len_utf8();
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
            // TODO: remove \n at end of line
            '\n' => return true, // special case because i may have left a \n at the end of each line
            _ => return false,
        }
    }
    return true
}

pub fn strip_empty_lines(mut text: &str) -> (&str, &str) {
    let mut offset = 0;
    // only commit changes after a complete line is parsed
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
    if text.as_bytes().len() == offset {
        // all the text is blank lines
        split(text, offset)
    } else {
        split(text, committed_offset)
    }
}

/* syntax-specific parsing */

fn get_next_indent(text: &str) -> Option<String> {
    let (_, rest) = strip_empty_lines(text);
    match strip_line(rest) {
        None => {
            // no line to add in the block
            None
        }
        Some((line, _)) => {
            let (indent, _) = strip_indent(line);
            return Some(indent.to_string())
        }
    }
}

static DELIMITER: &str = ".";

fn parse_argument(s: &str) -> Option<String> {
    if s.len() == 0 {
        None
    } else {
        // optional space after ":"
        let s = s.strip_prefix(" ").unwrap_or(s);
        Some(s.to_string())
    }
}

fn parse_tag(line: &str) -> Option<Tag> {
    // parse lines that look like this: .{command}[:[ [{argument}]]]
    let (indent, line) = strip_indent(line);
    if !line.starts_with(DELIMITER) {
        return None
    }
    let indent = indent.to_string();
    let (_, line) = split(line, DELIMITER.as_bytes().len());

    if !line.contains(":") { // simplest case
        Some(Tag {
            name: line.to_string(),
            accept_block: false,
            base_indent: indent,
            argument: None,
        })
    } else {
        let (begin, end) = line.split_once(":").unwrap();
        Some(Tag {
            name: begin.to_string(),
            accept_block: true,
            base_indent: indent,
            argument: parse_argument(end),
        })
    }
}



fn parse_body<'a>(text: &'a str, tag: &Tag) -> Option<(String, &'a str)> {
    let result = get_next_indent(text);
    // all of this is just safety checks
    if result.is_none() {
        // no body content was found
        return None
    }
    let indent = result.unwrap();
    // check that the indentation level is correct
    if !indent.starts_with(tag.base_indent.as_str()) {
        return None
    }
    if tag.argument.is_some() && indent.len() == tag.base_indent.len() {
        // using .tag:\n is the only case where using an empty indentation is accepted
        // (in order to allow commands to process all the text, and one-line commands with a parameter)
        return None
    }

    // at this point, there is at least one line that is correctly indented in the body
    // so this should work
    let mut body = String::new();
    let mut text = text;
    while text.len() > 0 {
        let (empty, left) = strip_empty_lines(text);
        let result = strip_line(left);
        if result.is_none() {
            // end of text
            break
        }
        let (line, left) = result.unwrap();
        let (line_indent, _) = strip_indent(line);
        if line_indent.starts_with(indent.as_str()) {
            // the line is still in the body
            body.push_str(empty);
            body.push_str(line);
            body.push('\n');
            text = left;
        } else {
            // not in the block anymore
            break
        }
    }
    return Some((body, text));
}

pub fn parse_section(text: &str) -> Option<Result<(Section, &str), String>> {
    let (line, text) = strip_line(text)?;
    let tag = parse_tag(line)?;

    let mut result = Section {
        name: tag.name,
        argument: tag.argument,
        body: None,
    };
    let mut left_text = text;
    match parse_body(left_text, &tag) {
        None if tag.argument.is_none() => {
            // Syntax error: expecting a body
            return Some(Err(format!("Syntax error: expecting a body after \".{}:\"", tag.name)));
        }
        Some((body, text)) => {
            left_text = text;
            result.body = Some(body)
        }
        _ => {}
    }

    Some(Ok((result, left_text)))
}
