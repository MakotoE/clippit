use std::mem::take;

/*
/‾‾\
|  |
@  @
|| |/
|| ||
|\_/|
\___/
  /\
/‾  ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾\
| asdf                 |
| asdf                 |
\______________________/
 */

// https://github.com/gbigwood/Clippo

const CLIPPY_ART: &str = r#"/‾‾\
|  |
@  @
|| |/
|| ||
|\_/|
\___/
  /\
"#;

#[derive(Default, Clone, PartialOrd, PartialEq)]
pub struct ClippyOutput {
    buf: String,
    terminal_width: u16,

    // Incomplete last line without vertical bars
    line: String,

    // Length of line in chars. Since this is the number of characters, the displayed width may be
    // different. For example, combining characters will cause lines to appear shorter.
    line_char_length: u16,
}

impl ClippyOutput {
    pub fn new(mut terminal_width: u16) -> Self {
        if terminal_width < 5 {
            terminal_width = 5;
        }

        let mut s = CLIPPY_ART.to_string() + "/‾  ";
        for _ in 0..terminal_width - 5 {
            s.push('_');
        }
        s.push_str("\\\n");
        Self {
            buf: s,
            terminal_width,
            line: String::new(),
            line_char_length: 0,
        }
    }

    pub fn add_str(&mut self, s: &str) {
        for char in s.chars() {
            self.line.push(char);
            self.line_char_length += 1;

            if self.line_char_length == self.terminal_width - 4 {
                self.buf.push_str("| ");
                self.buf.push_str(&take(&mut self.line));
                self.line_char_length = 0;
                self.buf.push_str(" |\n");
            }
        }
    }

    pub fn finish(&mut self) {
        if !self.line.is_empty() {
            let line_length = self.line.chars().count() as u16;
            self.buf.push_str("| ");
            self.buf.push_str(&take(&mut self.line));
            self.line_char_length = 0;

            for _ in 0..self.terminal_width - 4 - line_length {
                self.buf.push(' ');
            }

            self.buf.push_str(" |\n");
        }

        self.buf.push('\\');
        for _ in 0..self.terminal_width - 2 {
            self.buf.push('_');
        }
        self.buf.push('/');
    }
}

impl Iterator for ClippyOutput {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let result = take(&mut self.buf);
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clippy_output() {
        {
            let mut clippy = ClippyOutput::new(0);
            let result: String = clippy.by_ref().collect();
            assert_eq!(result, CLIPPY_ART.to_string() + "/‾  \\\n");

            clippy.finish();
            let result: String = clippy.collect();
            assert_eq!(result, "\\___/");
        }
        {
            let mut clippy = ClippyOutput::new(0);
            clippy.add_str("a");
            let result: String = clippy.by_ref().collect();
            assert_eq!(result, CLIPPY_ART.to_string() + "/‾  \\\n| a |\n");

            clippy.finish();
            let result: String = clippy.collect();
            assert_eq!(result, "\\___/");
        }
        {
            let mut clippy = ClippyOutput::new(0);
            clippy.add_str("aa");
            clippy.finish();

            let result: String = clippy.collect();
            assert_eq!(
                result,
                CLIPPY_ART.to_string() + "/‾  \\\n| a |\n| a |\n\\___/"
            );
        }
    }
}
