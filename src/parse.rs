struct Call {
    name: String,
    parameter: String,
    content: String,
}

struct SectionHeader {
    name: String,
    parameter: String,
    previous_indent: String,
}

struct Section {
    header: SectionHeader,
    indent: String,
    content: String,
}


fn split_indent(mut text: &str) -> (String, &str) {
    let mut indent = String::new();
    while let Some(ch) = text.chars().next() {
        match ch {
            ' ' | '\t' => {
                text = &text[1..];
                indent.push(ch)
            }
            _ => {
                return (indent, text)
            }
        }
    }
    return (indent, text)
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
                //println!("line length: {}", line.len());
                line.push(ch);
                return Some((line, text))
            }
            _ => {
                line.push(ch)
            }
        }
    }
    //println!("line length: {}", line.len());
    Some((line, text))
}

static DELIMITER: &str = ".";

fn parse_section(line: &str) -> Option<SectionHeader> {
    let (indent, line) = split_indent(line);
    if !line.starts_with(DELIMITER) {
        return None
    }
    let line = &line[DELIMITER.as_bytes().len()..];
    let mut call = SectionHeader {
        name: "".to_string(),
        parameter: "".to_string(),
        previous_indent: indent,
    };
    let (start, end) = line.split_once(":").unwrap();
    call.name = start.to_string();
    call.parameter = end.to_string();
    Some(call)
}

fn parse_block(section: SectionHeader, text: &str) -> Option<(Section, &str)> {
    match split_line(text) {
        // no text left
        // TODO: might be a bug depending on what behaviour we want
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

