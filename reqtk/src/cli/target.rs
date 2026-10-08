use req_file::prelude::*;
use std::{
    fs::File,
    io::{Error as IoError, ErrorKind, Seek, SeekFrom, stdin, stdout},
};

pub enum LocationIo {
    File(File),
    StdInOut,
}

impl LocationIo {
    pub fn set_len(&mut self, size: u64) -> std::io::Result<()> {
        match self {
            LocationIo::File(f) => f.set_len(size),
            LocationIo::StdInOut => Err(IoError::new(
                ErrorKind::Unsupported,
                "set_len not supported on stdin/stdout",
            )),
        }
    }
}

impl Seek for LocationIo {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match self {
            LocationIo::File(f) => f.seek(pos),
            LocationIo::StdInOut => Err(IoError::new(
                ErrorKind::Unsupported,
                "seek not supported on stdin/stdout",
            )),
        }
    }
}

impl std::io::Read for LocationIo {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            LocationIo::File(f) => f.read(buf),
            LocationIo::StdInOut => stdin().read(buf),
        }
    }
}
impl std::io::Write for LocationIo {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            LocationIo::File(f) => f.write(buf),
            LocationIo::StdInOut => stdout().write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            LocationIo::File(f) => f.flush(),
            LocationIo::StdInOut => stdout().flush(),
        }
    }
}

pub trait ErrorLocalizedSpanned<T, E>: Sized {
    fn err_localized(self, location: &Location) -> Result<T, Localized<E>>;
    fn err_spanned(self, span: Span) -> Result<T, Spanned<E>>;
}

impl<T, E> ErrorLocalizedSpanned<T, E> for Result<T, E> {
    fn err_localized(self, location: &Location) -> Result<T, Localized<E>> {
        self.map_err(|e| Localized::new(e, Some(location.clone())))
    }

    fn err_spanned(self, span: Span) -> Result<T, Spanned<E>> {
        self.map_err(|e| Spanned::new(e, Some(span)))
    }
}

pub trait LocationExt {
    fn reader(&self) -> Result<LocationIo, Localized<IoError>>;
    fn writer(&self, create_trunc: bool) -> Result<LocationIo, Localized<IoError>>;
}

impl LocationExt for Location {
    fn reader(&self) -> Result<LocationIo, Localized<IoError>> {
        Ok(match &self {
            Location::Path(path) => {
                LocationIo::File(File::options().read(true).open(path).err_localized(self)?)
            }
            Location::StdIo => LocationIo::StdInOut,
        })
    }

    fn writer(&self, create_trunc: bool) -> Result<LocationIo, Localized<IoError>> {
        Ok(match &self {
            Location::Path(path) => LocationIo::File(
                File::options()
                    .read(true)
                    .create(create_trunc)
                    .truncate(create_trunc)
                    .write(true)
                    .open(path)
                    .err_localized(self)?,
            ),
            Location::StdIo => LocationIo::StdInOut,
        })
    }
}
