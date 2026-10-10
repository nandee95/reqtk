use crate::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Span {
    pub start: Cursor,
    pub end: Cursor,
}

impl Span {
    pub fn of(text: &str) -> Self {
        Self::new(Cursor::start(), Cursor::end(text))
    }
    pub fn begin() -> Self {
        Self::new(Cursor::start(), Cursor::start())
    }

    pub fn new(start: Cursor, end: Cursor) -> Self {
        Self { start, end }
    }

    pub fn union(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    pub fn to_range(&self) -> std::ops::Range<usize> {
        self.start.offset..self.end.offset
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Span {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeTuple;

        let mut state = serializer.serialize_tuple(2)?;
        state.serialize_element(&self.start)?;
        state.serialize_element(&self.end)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Span {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (start, end): (Cursor, Cursor) = serde::Deserialize::deserialize(deserializer)?;
        Ok(Span { start, end })
    }
}
