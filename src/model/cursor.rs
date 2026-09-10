use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeTuple};

use crate::Span;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Cursor {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Cursor {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self {
            line,
            column,
            offset,
        }
    }

    pub fn start() -> Self {
        Self {
            line: 1,
            column: 1,
            offset: 0,
        }
    }
    pub fn end(string: &str) -> Self {
        Self::start().advance(string)
    }

    pub fn zero_span(&self) -> Span {
        Span::new(*self, *self)
    }

    pub fn advance(&self, string: &str) -> Self {
        let mut result = *self;
        for ch in string.chars() {
            result.advance_ch(ch);
        }
        result
    }

    pub fn advance_with(&mut self, string: &str) -> Span {
        let before = *self;
        for ch in string.chars() {
            self.advance_ch(ch);
        }
        Span::new(before, *self)
    }

    pub fn advance_ch(&mut self, ch: char) {
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += ch.len_utf16();
        }

        self.offset += ch.len_utf8();
    }

    pub fn min(self, other: Cursor) -> Cursor {
        if (self.line, self.column) < (other.line, other.column) {
            self
        } else {
            other
        }
    }
    pub fn max(self, other: Cursor) -> Cursor {
        if (self.line, self.column) > (other.line, other.column) {
            self
        } else {
            other
        }
    }
}

impl Serialize for Cursor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_tuple(2)?;
        state.serialize_element(&self.line)?;
        state.serialize_element(&self.column)?;
        state.serialize_element(&self.offset)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Cursor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (line, column, offset): (usize, usize, usize) = Deserialize::deserialize(deserializer)?;
        Ok(Cursor {
            line,
            column,
            offset,
        })
    }
}

impl std::fmt::Display for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_cursor_advance() {
        assert_eq!(Cursor::end("abc"), Cursor::new(1, 4, 3));
        assert_eq!(Cursor::end("a🔥c"), Cursor::new(1, 5, 6));
        assert_eq!(Cursor::end("abc\nabc"), Cursor::new(2, 4, 7));
        assert_eq!(Cursor::end("a🔥c\na🔥c"), Cursor::new(2, 5, 13));
    }
}
