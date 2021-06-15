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
| Text goes            |
| here                 |
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

    // output_width >= 5
    output_width: u16,

    // Incomplete last line without vertical bars
    line: String,

    // Length of line in chars. Since this is the number of characters, the displayed width may be
    // different. For example, combining characters will cause lines to appear shorter.
    // line_char_length <= self.output_width - 4
    line_char_length: u16,
}

impl ClippyOutput {
    pub fn new(mut output_width: u16) -> Self {
        if output_width < 5 {
            output_width = 5;
        }

        let mut s = CLIPPY_ART.to_string() + "/‾  ";
        for _ in 0..output_width - 5 {
            s.push('‾');
        }
        s.push_str("\\\n");
        Self {
            buf: s,
            output_width,
            line: String::new(),
            line_char_length: 0,
        }
    }

    pub fn add_str(&mut self, s: &str) {
        for char in s.chars() {
            if char == '\n' {
                self.buf.push_str("| ");
                self.buf.push_str(&take(&mut self.line));
                for _ in 0..self.output_width - 4 - self.line_char_length {
                    self.buf.push(' ');
                }
                self.line_char_length = 0;
                self.buf.push_str(" |\n");
            } else {
                self.line.push(char);
                self.line_char_length += 1;
            }

            if self.line_char_length == self.output_width - 4 {
                self.buf.push_str("| ");
                self.buf.push_str(&take(&mut self.line));
                self.line_char_length = 0;
                self.buf.push_str(" |\n");
            }
        }
    }

    /// `add_str()` or `finish()` should not be called after `finish()`.
    pub fn finish(&mut self) {
        if !self.line.is_empty() {
            let line_length = self.line.chars().count() as u16;
            self.buf.push_str("| ");
            self.buf.push_str(&take(&mut self.line));
            self.line_char_length = 0;

            for _ in 0..self.output_width - 4 - line_length {
                self.buf.push(' ');
            }

            self.buf.push_str(" |\n");
        }

        self.buf.push('\\');
        for _ in 0..self.output_width - 2 {
            self.buf.push('_');
        }
        self.buf.push_str("/\n");
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
            // Minimum output width
            let mut clippy = ClippyOutput::new(0);
            let result: String = clippy.by_ref().collect();
            assert_eq!(result, CLIPPY_ART.to_string() + "/‾  \\\n");

            clippy.finish();
            let result: String = clippy.by_ref().collect();
            assert_eq!(result, "\\___/\n");
        }
        {
            // Add empty string
            let mut clippy = ClippyOutput::new(0);
            clippy.add_str("");
            clippy.finish();
            let result: String = clippy.collect();
            assert_eq!(result, CLIPPY_ART.to_string() + "/‾  \\\n\\___/\n");
        }
        {
            let mut clippy = ClippyOutput::new(0);
            clippy.add_str("a");
            let result: String = clippy.by_ref().collect();
            assert_eq!(result, CLIPPY_ART.to_string() + "/‾  \\\n| a |\n");

            clippy.finish();
            let result: String = clippy.by_ref().collect();
            assert_eq!(result, "\\___/\n");

            // Add string after finish()
            clippy.add_str("a");
            clippy.finish();
            let result: String = clippy.collect();
            assert_eq!(result, "| a |\n\\___/\n");
        }
        {
            // Output width = 6
            let mut clippy = ClippyOutput::new(6);
            clippy.add_str("aa");
            clippy.finish();

            let result: String = clippy.collect();
            assert_eq!(
                result,
                CLIPPY_ART.to_string() + "/‾  ‾\\\n| aa |\n\\____/\n"
            );
        }
        {
            // Wrap long line
            let mut clippy = ClippyOutput::new(6);
            clippy.add_str("aaa");
            clippy.finish();

            let result: String = clippy.collect();
            assert_eq!(
                result,
                CLIPPY_ART.to_string() + "/‾  ‾\\\n| aa |\n| a  |\n\\____/\n"
            );
        }
        {
            // Append string
            let mut clippy = ClippyOutput::new(6);
            clippy.add_str("a");
            let result: String = clippy.by_ref().collect();
            assert_eq!(result, CLIPPY_ART.to_string() + "/‾  ‾\\\n");

            clippy.add_str("a");
            clippy.finish();
            let result: String = clippy.collect();
            assert_eq!(result, "| aa |\n\\____/\n");
        }
        {
            // Newline in string
            let mut clippy = ClippyOutput::new(6);
            clippy.add_str("a\n");
            let result: String = clippy.by_ref().collect();
            assert_eq!(result, CLIPPY_ART.to_string() + "/‾  ‾\\\n| a  |\n");

            clippy.add_str("b");
            clippy.finish();
            let result: String = clippy.collect();
            assert_eq!(result, "| b  |\n\\____/\n");
        }
    }
}
