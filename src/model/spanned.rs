use crate::*;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct Spanned<T> {
    value: T,
    pub span: Span,
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

pub trait IntoSpanned<T> {
    fn into_spanned(self, span: Span) -> Spanned<T>;
}

impl<T> IntoSpanned<T> for T {
    fn into_spanned(self, span: Span) -> Spanned<T> {
        Spanned { value: self, span }
    }
}

impl Spanned<String> {
    pub fn trim(&self) -> Spanned<String> {
        let len = self.value.len();
        let leading_len = len - self.value.trim_start().len();
        let trailing_len = len - self.value.trim_end().len();

        let trimmed = &self.value[leading_len..len - trailing_len];

        let start_cursor = self.span.start.advance(&self.value[..leading_len]);
        let end_cursor = start_cursor.advance(trimmed);

        Spanned {
            value: trimmed.to_string(),
            span: Span::new(start_cursor, end_cursor),
        }
    }

    pub fn unindent(&self) -> Spanned<String> {
        let indent = Self::leading_indentation(&self.value);
        let trimmed = self.trim();

        let unindented = trimmed
            .value
            .replace(&format!("\n{}", indent), "\n")
            .to_string();
        unindented.clone().into_spanned(trimmed.span)
    }

    fn leading_indentation(s: &str) -> &str {
        let Some(pos) = s.find(|c: char| !c.is_whitespace()) else {
            return "";
        };

        let start = s[..pos].rfind('\n').map(|i| i + 1).unwrap_or(0);

        &s[start..pos]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct Localized<T> {
    #[serde(flatten)]
    value: T,
    pub location: Location,
}

impl<T> Deref for Localized<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Localized<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

pub trait IntoLocalized<T> {
    fn into_localized(self, location: &Location) -> Localized<T>;
}

impl<T> IntoLocalized<T> for T {
    fn into_localized(self, location: &Location) -> Localized<T> {
        Localized {
            value: self,
            location: location.clone(),
        }
    }
}

pub trait ErrorLocalizedSpanned<T, E>: Sized {
    fn err_localized(self, location: &Location) -> Result<T, Localized<E>>;
    fn err_spanned(self, span: &Span) -> Result<T, Spanned<E>>;
}

impl<T, E> ErrorLocalizedSpanned<T, E> for Result<T, E> {
    fn err_localized(self, location: &Location) -> Result<T, Localized<E>> {
        self.map_err(|e| e.into_localized(location))
    }

    fn err_spanned(self, span: &Span) -> Result<T, Spanned<E>> {
        self.map_err(|e| Spanned {
            value: e,
            span: *span,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spanned_string_no_leading_trailing_whitespaces() {
        let text = "hello";
        let start = Cursor::new(11, 5, 550);
        let input = text
            .to_string()
            .into_spanned(Span::new(start, start.advance(text)));

        let trimmed = input.trim();
        assert_eq!(trimmed, input);

        let unindented = input.unindent();
        assert_eq!(unindented, input);
    }

    #[test]
    fn test_spanned_string_leading_trailing_non_newline_whitespaces() {
        let input_str = "  hello  ";
        let input = input_str.to_string().into_spanned(Span::new(
            Cursor::new(11, 5, 500),
            Cursor::new(11, 5 + input_str.len(), 500 + input_str.len()),
        ));

        let trimmed = input.trim();
        assert_eq!(*trimmed, "hello");
        assert_eq!(
            trimmed.span,
            Span::new(Cursor::new(11, 7, 502), Cursor::new(11, 12, 507))
        );

        let unindented = input.unindent();
        assert_eq!(unindented.value, "hello");
        assert_eq!(
            unindented.span,
            Span::new(Cursor::new(11, 7, 502), Cursor::new(11, 12, 507))
        );
    }

    #[test]
    fn test_spanned_string_leading_trailing_newline_whitespaces() {
        let input_str = "\n\n  hello\t\t\n\n";
        let input = input_str.to_string().into_spanned(Span::new(
            Cursor::new(11, 12, 500),
            Cursor::new(15, 1, 500 + input_str.len()),
        ));

        let trimmed = input.trim();
        assert_eq!(trimmed.value, "hello");
        assert_eq!(
            trimmed.span,
            Span::new(
                Cursor::new(13, 3, 504),
                Cursor::new(13, 8, input.span.end.offset - 4)
            )
        );

        let unindented = input.unindent();
        assert_eq!(unindented.value, "hello");
        assert_eq!(
            unindented.span,
            Span::new(
                Cursor::new(13, 3, 504),
                Cursor::new(13, 8, input.span.end.offset - 4)
            )
        );
    }

    #[test]
    fn test_spanned_string_with_indentation() {
        let input_str = "\n\t\t\n\t\t\thello\n\t\t\tworld\n\t\t\tthere\n\t\t\t\n";
        let input = input_str.to_string().into_spanned(Span::new(
            Cursor::new(11, 1, 500),
            Cursor::new(17, 1, 500 + input_str.len()),
        ));

        let trimmed = input.trim();
        assert_eq!(trimmed.value, "hello\n\t\t\tworld\n\t\t\tthere");
        assert_eq!(
            trimmed.span,
            Span::new(
                Cursor::new(13, 4, 507),
                Cursor::new(15, 9, input.span.end.offset - 5)
            )
        );

        let unindented = input.unindent();
        assert_eq!(unindented.value, "hello\nworld\nthere");
        assert_eq!(
            unindented.span,
            Span::new(
                Cursor::new(13, 4, 507),
                Cursor::new(15, 9, input.span.end.offset - 5)
            )
        );
    }
}
