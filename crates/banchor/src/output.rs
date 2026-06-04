use std::io::{self, Write};

use crate::BanchorError;

pub fn write_stdout_text(text: impl std::fmt::Display) -> Result<(), BanchorError> {
    let stdout = io::stdout();
    write_text(stdout.lock(), text)
}

pub fn write_stdout_lines(
    lines: impl IntoIterator<Item = impl std::fmt::Display>,
) -> Result<(), BanchorError> {
    let stdout = io::stdout();
    write_lines(stdout.lock(), lines)
}

fn write_text(mut writer: impl Write, text: impl std::fmt::Display) -> Result<(), BanchorError> {
    writeln!(writer, "{text}").map_err(BanchorError::from_stdout_error)
}

fn write_lines(
    mut writer: impl Write,
    lines: impl IntoIterator<Item = impl std::fmt::Display>,
) -> Result<(), BanchorError> {
    for line in lines {
        writeln!(writer, "{line}").map_err(BanchorError::from_stdout_error)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::{self, Write};

    use crate::BanchorError;

    use super::{write_lines, write_text};

    #[test]
    fn text_writer_preserves_payload_and_adds_one_line_break() {
        for (payload, expected) in [("directive", b"directive\n".as_slice()), ("", b"\n")] {
            let mut output = Vec::new();

            write_text(&mut output, payload).unwrap();

            assert_eq!(output, expected);
        }
    }

    #[test]
    fn line_writer_preserves_order_and_line_boundaries() {
        let cases = [
            (Vec::<&str>::new(), b"".as_slice()),
            (vec!["one"], b"one\n".as_slice()),
            (vec!["one", "two"], b"one\ntwo\n".as_slice()),
        ];

        for (lines, expected) in cases {
            let mut output = Vec::new();

            write_lines(&mut output, lines).unwrap();

            assert_eq!(output, expected);
        }
    }

    #[test]
    fn write_errors_preserve_broken_pipe_as_silent_exit_signal() {
        let result = write_text(BrokenPipeWriter, "directive");

        assert!(matches!(result, Err(BanchorError::BrokenStdoutPipe)));
    }

    struct BrokenPipeWriter;

    impl Write for BrokenPipeWriter {
        fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
            Err(io::Error::from(io::ErrorKind::BrokenPipe))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
}
