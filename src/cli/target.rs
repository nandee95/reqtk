use crate::*;
use serde::{Deserialize, Serialize};
use std::io::{Error as IoError, ErrorKind, Seek, SeekFrom, stdin, stdout};
use std::{fs::File, path::PathBuf};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Location {
    Path(PathBuf),
    StdIo,
}

impl Serialize for Location {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Location {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == "stdio" {
            Ok(Location::StdIo)
        } else {
            Ok(Location::Path(PathBuf::from(s)))
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::Path(file_input) => {
                write!(f, "{}", file_input.display())
            }
            Location::StdIo => write!(f, "stdio"),
        }
    }
}

impl Location {
    pub fn with_extension(&self, extension: &str) -> Self {
        match self {
            Location::Path(path) => {
                let mut new_path = path.clone();
                new_path.set_extension(extension);
                Location::Path(new_path)
            }
            Location::StdIo => Location::StdIo,
        }
    }

    pub fn extension(&self) -> Option<String> {
        match self {
            Location::Path(path) => path
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|s| s.to_string()),
            Location::StdIo => None,
        }
    }

    pub fn reader(&self) -> Result<LocationIo, Localized<IoError>> {
        Ok(match &self {
            Location::Path(path) => {
                LocationIo::File(File::options().read(true).open(path).err_localized(self)?)
            }
            Location::StdIo => LocationIo::StdInOut,
        })
    }

    pub fn writer(&self) -> Result<LocationIo, Localized<IoError>> {
        Ok(match &self {
            Location::Path(path) => LocationIo::File(
                File::options()
                    .read(true)
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(path)
                    .err_localized(self)?,
            ),
            Location::StdIo => LocationIo::StdInOut,
        })
    }
}

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
