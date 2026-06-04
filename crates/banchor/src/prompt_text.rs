use std::fmt::Write;

pub fn lossless_field_value(value: &str) -> String {
    let mut rendered = String::new();

    for character in value.chars() {
        push_lossless_character(&mut rendered, character);
    }

    rendered
}

fn push_lossless_character(output: &mut String, character: char) {
    match character {
        '\n' => output.push_str(r"\n"),
        '\r' => output.push_str(r"\r"),
        '\t' => output.push_str(r"\t"),
        '\\' => output.push_str(r"\\"),
        character if character.is_control() => {
            write!(output, r"\u{{{:x}}}", character as u32).unwrap();
        }
        character => output.push(character),
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::lossless_field_value;

    #[test]
    fn plain_text_stays_readable() {
        assert_eq!(
            lossless_field_value("rename HostContext::L2a to HostContext::CliL2a"),
            "rename HostContext::L2a to HostContext::CliL2a"
        );
    }

    #[test]
    fn control_characters_become_single_line_escape_sequences() {
        assert_eq!(
            lossless_field_value("line1\nline2\ttail\r\u{7}"),
            r"line1\nline2\ttail\r\u{7}"
        );
    }

    #[test]
    fn unicode_text_stays_readable() {
        assert_eq!(lossless_field_value("mission α é 🙂"), "mission α é 🙂");
    }

    #[test]
    fn backslashes_are_escaped_without_changing_adjacent_text() {
        assert_eq!(
            lossless_field_value(r"path\to\mission"),
            r"path\\to\\mission"
        );
    }

    proptest! {
        #[test]
        fn rendered_field_values_are_single_line_and_lossless(value in ".*") {
            let rendered = lossless_field_value(&value);

            prop_assert!(!rendered.chars().any(char::is_control));
            prop_assert_eq!(decode_lossless_field_value(&rendered), value);
        }
    }

    fn decode_lossless_field_value(value: &str) -> String {
        let mut decoded = String::new();
        let mut characters = value.chars().peekable();

        while let Some(character) = characters.next() {
            if character != '\\' {
                decoded.push(character);
                continue;
            }

            match characters.next() {
                Some('n') => decoded.push('\n'),
                Some('r') => decoded.push('\r'),
                Some('t') => decoded.push('\t'),
                Some('\\') => decoded.push('\\'),
                Some('u') => decoded.push(decode_unicode_escape(&mut characters)),
                Some(other) => {
                    decoded.push('\\');
                    decoded.push(other);
                }
                None => decoded.push('\\'),
            }
        }

        decoded
    }

    fn decode_unicode_escape(
        characters: &mut std::iter::Peekable<impl Iterator<Item = char>>,
    ) -> char {
        assert_eq!(characters.next(), Some('{'));

        let mut hex = String::new();
        for character in characters.by_ref() {
            if character == '}' {
                break;
            }

            hex.push(character);
        }

        let codepoint = u32::from_str_radix(&hex, 16).unwrap();
        char::from_u32(codepoint).unwrap()
    }
}
