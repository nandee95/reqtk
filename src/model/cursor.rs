use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeTuple};

use crate::Span;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Cursor {
    pub line: usize,
    pub column: usize,
}

impl Cursor {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn start() -> Self {
        Self { line: 1, column: 1 }
    }
    pub fn zero_span(&self) -> Span {
        Span::new(*self, *self)
    }

    pub fn advance(&self, string: &str) -> Self {
        let mut result = *self;
        for ch in string.chars() {
            result = result.advance_ch(ch);
        }
        result
    }

    pub fn advance_with(&mut self, string: &str) -> Span {
        let before = *self;
        for ch in string.chars() {
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        Span::new(before, *self)
    }

    pub fn advance_ch(&self, ch: char) -> Self {
        let mut result = *self;

        if ch == '\n' {
            result.line += 1;
            result.column = 1;
        } else {
            result.column += 1;
        }

        result
    }

    pub fn advance_by(&self, lines: usize, columns: usize) -> Self {
        Self {
            line: self.line + lines,
            column: self.column + columns,
        }
    }

    pub fn min(self, other: Cursor) -> Cursor {
        Cursor {
            line: self.line.min(other.line),
            column: self.column.min(other.column),
        }
    }
    pub fn max(self, other: Cursor) -> Cursor {
        Cursor {
            line: self.line.max(other.line),
            column: self.column.max(other.column),
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
        state.end()
    }
}

impl<'de> Deserialize<'de> for Cursor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (line, column): (usize, usize) = Deserialize::deserialize(deserializer)?;
        Ok(Cursor { line, column })
    }
}

impl std::fmt::Display for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
