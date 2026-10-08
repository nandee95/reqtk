use crate::prelude::*;
pub struct ReferenceIterator<'a> {
    string: &'a str,
    span: Option<Span>,
}

impl<'a> ReferenceIterator<'a> {
    pub fn new(string: &'a str, span: Option<Span>) -> Self {
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
                            let reference_span = if let Some(span) = self.span {
                                let start = span.start.advance(&self.string[..id_start]);
                                let end = start.advance(key);
                                self.span = Some(Span::new(end.advance("}"), span.end));
                                Some(Span::new(start, end))
                            } else {
                                None
                            };

                            self.string = &self.string[i + 1..];
                            return Some(Spanned::new(key, reference_span));
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
        let traces: Vec<_> = ReferenceIterator::new(block, Some(span)).collect();

        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].value, "REQ-001");
        assert_eq!(traces[0].span.unwrap().start.line, 1);
        assert_eq!(traces[0].span.unwrap().start.column, 24); // Position of 'R' in REQ-001
    }

    #[test]
    fn test_find_multiple_references() {
        let block = "First: {@REQ-001}, Second: {@REQ-002}";
        let span = Span::new(Cursor::start(), Cursor::end(block));
        let traces: Vec<_> = ReferenceIterator::new(block, Some(span)).collect();

        assert_eq!(traces.len(), 2);
        assert_eq!(traces[0].value, "REQ-001");
        assert_eq!(traces[1].value, "REQ-002");
        assert_eq!(
            traces[1].span.unwrap().start,
            Cursor::start().advance(&block[..block.find("REQ-002").unwrap()])
        );
    }

    #[test]
    fn test_find_references_without_span() {
        let block = "First: {@REQ-001}, Second: {@REQ-002}";
        let traces: Vec<_> = ReferenceIterator::new(block, None).collect();

        assert_eq!(traces.len(), 2);
        assert_eq!(traces[0].value, "REQ-001");
        assert_eq!(traces[1].value, "REQ-002");
        assert!(traces.iter().all(|trace| trace.span.is_none()));
    }

    #[test]
    fn test_no_references() {
        let block = "No references here";
        let span = Span::new(Cursor::start(), Cursor::end(block));
        let traces: Vec<_> = ReferenceIterator::new(block, Some(span)).collect();

        assert!(traces.is_empty());
    }

    #[test]
    fn test_utf8() {
        let block = "Reference: {@RÉQ-001}";
        let span = Span::new(Cursor::start(), Cursor::end(block));

        let traces: Vec<_> = ReferenceIterator::new(block, Some(span)).collect();

        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].value, "RÉQ-001"); // Longer match wins
    }

    #[test]
    fn test_multiline_reference() {
        let block = "Line one\nReferences {@REQ-001}\nLine three";
        let span = Span::new(Cursor::start(), Cursor::end(block));

        let traces: Vec<_> = ReferenceIterator::new(block, Some(span)).collect();

        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].span.unwrap().start.line, 2);
        assert_eq!(traces[0].span.unwrap().start.column, 14); // "References {@" = 13 chars, so column 14
    }
}
