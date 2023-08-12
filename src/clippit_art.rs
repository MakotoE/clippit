use std::mem::take;
use textwrap::wrap;

const CLIPPY_ART: &str = r#"/‾‾\
|  |
@  @
|| |/
|| ||
|\_/|
\___/
  /\
"#;

/// Inputs a string and outputs ascii art of Clippy saying the text.
///
/// Call `add_str()` to input strings and call `finish()` at the end after all text as been added.
/// `ClippyArt` implements `Iterator` to return the output string.
#[derive(Default, Clone, PartialOrd, PartialEq)]
pub struct ClippyArt {
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

impl ClippyArt {
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

    /// Adds text to be processed.
    pub fn add_str(&mut self, s: &str) {
        let lines = wrap(s, usize::min((self.output_width - 4) as usize, 2000));

        for (i, line) in lines.iter().enumerate() {
            for char in line.chars() {
                if char == '\n' {
                    ClippyArt::add_string_to_buffer(
                        &mut self.buf,
                        &self.line,
                        self.output_width - 4 - self.line_char_length,
                    );
                    self.line.clear();
                    self.line_char_length = 0;
                } else {
                    self.line.push(char);
                    self.line_char_length += 1;
                }

                if self.line_char_length == self.output_width - 4 {
                    ClippyArt::add_string_to_buffer(&mut self.buf, &self.line, 0);
                    self.line.clear();
                    self.line_char_length = 0;
                }
            }

            if i < lines.len() - 1 {
                ClippyArt::add_string_to_buffer(
                    &mut self.buf,
                    &self.line,
                    self.output_width - 4 - self.line_char_length,
                );
                self.line.clear();
                self.line_char_length = 0;
            }
        }
    }

    fn add_string_to_buffer(buf: &mut String, line: &str, space_count: u16) {
        if !line.is_empty() {
            buf.push_str("| ");
            buf.push_str(line);
            for _ in 0..space_count {
                buf.push(' ');
            }
            buf.push_str(" |\n");
        }
    }

    /// Appends the last line of the speech bubble.
    ///
    /// `add_str()` or `finish()` should not be called after `finish()` was called.
    pub fn finish(&mut self) {
        if !self.line.is_empty() {
            ClippyArt::add_string_to_buffer(
                &mut self.buf,
                &self.line,
                self.output_width - 4 - self.line.chars().count() as u16,
            );
            self.line.clear();
            self.line_char_length = 0;
        }

        self.buf.push('\\');
        for _ in 0..self.output_width - 2 {
            self.buf.push('_');
        }
        self.buf.push_str("/\n");
    }
}

impl Iterator for ClippyArt {
    type Item = String;

    /// Returns `Some` if there is a string remaining in the buffer. Returns `None` if the buffer is
    /// clear.
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
            let mut clippy = ClippyArt::new(0);
            let result: String = clippy.by_ref().collect();
            assert_eq!(result, CLIPPY_ART.to_string() + "/‾  \\\n");

            clippy.finish();
            let result: String = clippy.by_ref().collect();
            assert_eq!(result, "\\___/\n");
        }
        {
            // Add empty string
            let mut clippy = ClippyArt::new(0);
            clippy.add_str("");
            clippy.finish();
            let result: String = clippy.collect();
            assert_eq!(result, CLIPPY_ART.to_string() + "/‾  \\\n\\___/\n");
        }
        {
            let mut clippy = ClippyArt::new(0);
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
            let mut clippy = ClippyArt::new(6);
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
            let mut clippy = ClippyArt::new(6);
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
            let mut clippy = ClippyArt::new(6);
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
            let mut clippy = ClippyArt::new(6);
            clippy.add_str("a\n");
            let result: String = clippy.by_ref().collect();
            assert_eq!(result, CLIPPY_ART.to_string() + "/‾  ‾\\\n| a  |\n");

            clippy.add_str("b");
            clippy.finish();
            let result: String = clippy.collect();
            assert_eq!(result, "| b  |\n\\____/\n");
        }
        {
            // Word wrapping
            let mut clippy = ClippyArt::new(7);
            clippy.add_str("a");
            clippy.add_str("\n");
            clippy.add_str("aaaa\n");
            clippy.add_str("b");
            clippy.finish();
            let result: String = clippy.collect();
            assert_eq!(
                result,
                CLIPPY_ART.to_string() + "/‾  ‾‾\\\n| a   |\n| aaa |\n| a   |\n| b   |\n\\_____/\n"
            );
        }
    }
}
