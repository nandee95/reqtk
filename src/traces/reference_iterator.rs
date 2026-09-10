use crate::*;
pub struct ReferenceIterator<'a> {
    string: &'a str,
    span: Span,
}

impl<'a> ReferenceIterator<'a> {
    pub fn new(string: &'a str, span: Span) -> Self {
        Self { string, span }
    }
}

impl<'a> Iterator for ReferenceIterator<'a> {
    type Item = Spanned<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut offset = 0;

        while let Some(open) = self.string[offset..].find("{@") {
            let open = offset + open;
            let id_start = open + 2;

            let bytes = self.string.as_bytes();
            let mut i = id_start;

            while i < bytes.len() {
                match bytes[i] {
                    b'}' => {
                        let mut backslashes = 0;
                        let mut j = i;

                        while j > id_start && bytes[j - 1] == b'\\' {
                            backslashes += 1;
                            j -= 1;
                        }

                        // `}` is escaped iff preceded by an odd number of `\`.
                        if backslashes % 2 == 1 {
                            i += 1;
                            continue;
                        }

                        let key = &self.string[id_start..i];

                        // Empty IDs and IDs containing whitespace are invalid.
                        if !key.is_empty() && !key.chars().any(char::is_whitespace) {
                            let start = self.span.start.advance(&self.string[..id_start]);
                            let end = start.advance(key);

                            self.string = &self.string[i + 1..];
                            self.span = Span::new(end.advance("}"), self.span.end);

                            return Some(key.into_spanned(Span::new(start, end)));
                        }

                        break;
                    }

                    c if c.is_ascii_whitespace() => break,

                    _ => i += 1,
                }
            }

            // No valid reference here; continue searching after `{@`.
            offset = id_start;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_single_reference() {
        let block = "This text references {@REQ-001}.";
        let span = Span::new(Cursor::start(), Cursor::end(block));

        let traces: Vec<_> = ReferenceIterator::new(block, span).collect();

        assert_eq!(traces.len(), 1);
        assert_eq!(*traces[0], "REQ-001");
        assert_eq!(traces[0].span.start.line, 1);
        assert_eq!(traces[0].span.start.column, 24); // Position of 'R' in REQ-001
    }

    #[test]
    fn test_find_multiple_references() {
        let block = "First: {@REQ-001}, Second: {@REQ-002}";
        let span = Span::new(Cursor::start(), Cursor::end(block));

        let traces: Vec<_> = ReferenceIterator::new(block, span).collect();

        assert_eq!(traces.len(), 2);
        assert_eq!(*traces[0], "REQ-001");
        assert_eq!(*traces[1], "REQ-002");
    }

    #[test]
    fn test_no_references() {
        let block = "No references here";
        let span = Span::new(Cursor::start(), Cursor::end(block));

        let traces: Vec<_> = ReferenceIterator::new(block, span).collect();

        assert!(traces.is_empty());
    }

    #[test]
    fn test_utf8() {
        let block = "Reference: {@RÉQ-001}";
        let span = Span::new(Cursor::start(), Cursor::end(block));

        let traces: Vec<_> = ReferenceIterator::new(block, span).collect();

        assert_eq!(traces.len(), 1);
        assert_eq!(*traces[0], "RÉQ-001"); // Longer match wins
    }

    #[test]
    fn test_multiline_reference() {
        let block = "Line one\nReferences {@REQ-001}\nLine three";
        let span = Span::new(Cursor::start(), Cursor::end(block));

        let traces: Vec<_> = ReferenceIterator::new(block, span).collect();

        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].span.start.line, 2);
        assert_eq!(traces[0].span.start.column, 14); // "References {@" = 13 chars, so column 14
    }
}
