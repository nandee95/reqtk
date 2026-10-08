use serde::Serialize;
use serde_json::ser::{Formatter, PrettyFormatter};
use std::io::{self, Write};

pub struct JsonFormatter<'a> {
    inner: PrettyFormatter<'a>,
    array_depth: usize,
}

impl<'a> Default for JsonFormatter<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> JsonFormatter<'a> {
    pub fn new() -> Self {
        Self {
            inner: PrettyFormatter::with_indent(b"  "),
            array_depth: 0,
        }
    }

    pub fn serialize_into<T, W>(writer: W, value: &T) -> Result<(), serde_json::Error>
    where
        T: Serialize,
        W: Write,
    {
        let mut ser = serde_json::Serializer::with_formatter(writer, JsonFormatter::new());
        value.serialize(&mut ser)
    }
}

impl<'a> Formatter for JsonFormatter<'a> {
    fn begin_array<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.array_depth += 1;
        writer.write_all(b"[")
    }

    fn end_array<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.array_depth -= 1;
        writer.write_all(b"]")
    }

    fn begin_array_value<W: ?Sized + io::Write>(
        &mut self,
        writer: &mut W,
        first: bool,
    ) -> io::Result<()> {
        if !first {
            writer.write_all(b",")?;
        }
        Ok(())
    }

    fn end_array_value<W: ?Sized + io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
        Ok(())
    }

    // Delegate object formatting to PrettyFormatter
    fn begin_object<W: ?Sized + io::Write>(&mut self, w: &mut W) -> io::Result<()> {
        self.inner.begin_object(w)
    }
    fn end_object<W: ?Sized + io::Write>(&mut self, w: &mut W) -> io::Result<()> {
        self.inner.end_object(w)
    }
    fn begin_object_key<W: ?Sized + io::Write>(
        &mut self,
        w: &mut W,
        first: bool,
    ) -> io::Result<()> {
        self.inner.begin_object_key(w, first)
    }
    fn end_object_key<W: ?Sized + io::Write>(&mut self, w: &mut W) -> io::Result<()> {
        self.inner.end_object_key(w)
    }
    fn begin_object_value<W: ?Sized + io::Write>(&mut self, w: &mut W) -> io::Result<()> {
        self.inner.begin_object_value(w)
    }
    fn end_object_value<W: ?Sized + io::Write>(&mut self, w: &mut W) -> io::Result<()> {
        self.inner.end_object_value(w)
    }
}
