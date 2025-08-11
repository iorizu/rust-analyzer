//! Utility to strip leading indentation from strings.

// Inspired by this comment: https://news.ycombinator.com/item?id=44857130

use std::iter;

pub fn strip_indent(input: &str) -> String {
    let last_line_start = 1 + input
        .char_indices()
        .rev()
        .filter(|&(_, ch)| ch == '\n')
        .next()
        .unwrap()
        .0;
    let last_line_indent_end = last_line_start + input[last_line_start..]
        .char_indices()
        .filter(|&(_, ch)| !matches!(ch, '\t' | ' '))
        .next()
        .unwrap_or((input[last_line_start..].len(), 'a'))
        .0;
    let last_line_indent = &input[last_line_start..last_line_indent_end];
    let indent_length = last_line_indent_end - last_line_start;

    let mut buffer = String::new();
    
    for line in lines_with_newline(input) {
        if line.starts_with(last_line_indent) {
            let line_stripped = line.split_at(indent_length).1;
            buffer.push_str(line_stripped);
        }
        else if line.find("\n").unwrap() == 0 {
            buffer.push_str(line);
        }
        else {
            let last_line = &input[last_line_start..];
            panic!("Line '{line}' was not prefixed by same indentation as the last line '{last_line}'");
        }
    }
    buffer
}

fn lines_with_newline(input: &str) -> impl Iterator<Item = &str> {
    let mut lines = input;
    iter::from_fn(move || {
        match lines.find('\n') {
            Some(idx) => {
                match lines.split_at_checked(idx + 1) {
                    Some((line, rest)) => {
                        lines = rest;
                        Some(line)
                    },
                    None => None,
                }
            },
            None => None,
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let input = r#"
        mod foo;

        struct Bar {
            bar: bool
        }

        fn baz() -> i32 {
            let x = 42;
            x + 21
        }
        "#;
        let input_no_indent = r#"
mod foo;

struct Bar {
    bar: bool
}

fn baz() -> i32 {
    let x = 42;
    x + 21
}
"#;
        let output = strip_indent(input);
        assert_eq!(&output[..], input_no_indent);
    }

    #[test]
    fn idempodent() {
        let input = r#"
        mod foo;

        struct Bar {
            bar: bool
        }

        fn baz() -> i32 {
            let x = 42;
            x + 21
        }
        "#;
        let output = strip_indent(input);
        let output_output = strip_indent(&output);
        assert_eq!(&output_output[..], &output[..]);
    }
}