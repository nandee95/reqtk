use crate::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Option<Span>,
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for Spanned<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        if let Some(span) = self.span {
            let mut state = serializer.serialize_struct("Spanned", 2)?;
            state.serialize_field("value", &self.value)?;
            state.serialize_field("span", &span)?;
            state.end()
        } else {
            self.value.serialize(serializer)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Spanned<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct SpannedHelper<T> {
            value: T,
            span: Option<Span>,
        }

        let helper = SpannedHelper::deserialize(deserializer)?;
        Ok(Spanned {
            value: helper.value,
            span: helper.span,
        })
    }
}

impl<T> Spanned<T> {
    pub fn new<J>(value: J, span: Option<Span>) -> Self
    where
        J: Into<T>,
    {
        Self {
            value: value.into(),
            span,
        }
    }

    pub fn clear_span(&mut self) {
        self.span = None;
    }
}

impl Spanned<String> {
    pub fn trim(&self) -> Spanned<String> {
        let Some(span) = self.span else {
            return Spanned::new(self.value.trim().to_string(), None);
        };
        let len = self.value.len();
        let leading_len = len - self.value.trim_start().len();
        let trailing_len = len - self.value.trim_end().len();

        let trimmed = &self.value[leading_len..len - trailing_len];

        let start_cursor = span.start.advance(&self.value[..leading_len]);
        let end_cursor = start_cursor.advance(trimmed);

        Self {
            value: trimmed.to_string(),
            span: Some(Span::new(start_cursor, end_cursor)),
        }
    }

    pub fn unindent(&self) -> Spanned<String> {
        let indent = Self::leading_indentation(&self.value);
        let trimmed = self.trim();

        let unindented = trimmed
            .value
            .replace(&format!("\n{}", indent), "\n")
            .to_string();
        Self {
            value: unindented,
            span: trimmed.span,
        }
    }

    fn leading_indentation(s: &str) -> &str {
        let Some(pos) = s.find(|c: char| !c.is_whitespace()) else {
            return "";
        };

        let start = s[..pos].rfind('\n').map(|i| i + 1).unwrap_or(0);

        &s[start..pos]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spanned_string_no_leading_trailing_whitespaces() {
        let text = "hello";
        let start = Cursor::new(11, 5, 550);
        let input = Spanned::new(
            text.to_string(),
            Some(Span::new(start, start.advance(text))),
        );

        let trimmed = input.trim();
        assert_eq!(trimmed, input);

        let unindented = input.unindent();
        assert_eq!(unindented, input);
    }

    #[test]
    fn test_spanned_string_leading_trailing_non_newline_whitespaces() {
        let input_str = "  hello  ";
        let input = Spanned::new(
            input_str.to_string(),
            Some(Span::new(
                Cursor::new(11, 5, 500),
                Cursor::new(11, 5 + input_str.len(), 500 + input_str.len()),
            )),
        );

        let trimmed = input.trim();
        assert_eq!(trimmed.value, "hello");
        assert_eq!(
            trimmed.span.unwrap(),
            Span::new(Cursor::new(11, 7, 502), Cursor::new(11, 12, 507))
        );

        let unindented = input.unindent();
        assert_eq!(unindented.value, "hello");
        assert_eq!(
            unindented.span.unwrap(),
            Span::new(Cursor::new(11, 7, 502), Cursor::new(11, 12, 507))
        );
    }

    #[test]
    fn test_spanned_string_leading_trailing_newline_whitespaces() {
        let input_str = "\n\n  hello\t\t\n\n";
        let input = Spanned::new(
            input_str.to_string(),
            Some(Span::new(
                Cursor::new(11, 12, 500),
                Cursor::new(15, 1, 500 + input_str.len()),
            )),
        );

        let trimmed = input.trim();
        assert_eq!(trimmed.value, "hello");
        assert_eq!(
            trimmed.span.unwrap(),
            Span::new(
                Cursor::new(13, 3, 504),
                Cursor::new(13, 8, input.span.unwrap().end.offset - 4)
            )
        );

        let unindented = input.unindent();
        assert_eq!(unindented.value, "hello");
        assert_eq!(
            unindented.span.unwrap(),
            Span::new(
                Cursor::new(13, 3, 504),
                Cursor::new(13, 8, input.span.unwrap().end.offset - 4)
            )
        );
    }

    #[test]
    fn test_spanned_string_with_indentation() {
        let input_str = "\n\t\t\n\t\t\thello\n\t\t\tworld\n\t\t\tthere\n\t\t\t\n";
        let input = Spanned::new(
            input_str.to_string(),
            Some(Span::new(
                Cursor::new(11, 1, 500),
                Cursor::new(17, 1, 500 + input_str.len()),
            )),
        );

        let trimmed = input.trim();
        assert_eq!(trimmed.value, "hello\n\t\t\tworld\n\t\t\tthere");
        assert_eq!(
            trimmed.span.unwrap(),
            Span::new(
                Cursor::new(13, 4, 507),
                Cursor::new(15, 9, input.span.unwrap().end.offset - 5)
            )
        );

        let unindented = input.unindent();
        assert_eq!(unindented.value, "hello\nworld\nthere");
        assert_eq!(
            unindented.span.unwrap(),
            Span::new(
                Cursor::new(13, 4, 507),
                Cursor::new(15, 9, input.span.unwrap().end.offset - 5)
            )
        );
    }
}
