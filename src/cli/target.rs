use crate::*;
use serde::{Deserialize, Serialize};
use std::io::{Error as IoError, stdin, stdout};
use std::{fs::File, path::PathBuf};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Location {
    File(PathBuf),
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

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::File(file_input) => {
                write!(f, "{}", file_input.display())
            }
            Location::StdIo => write!(f, "stdin"),
        }
    }
}

impl Location {
    pub fn with_extension(&self, extension: &str) -> Self {
        match self {
            Location::File(path) => {
                let mut new_path = path.clone();
                new_path.set_extension(extension);
                Location::File(new_path)
            }
            Location::StdIo => Location::StdIo,
        }
    }

    pub fn extension(&self) -> Option<String> {
        match self {
            Location::File(path) => path
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|s| s.to_string()),
            Location::StdIo => None,
        }
    }

    pub fn reader(&self) -> Result<LocationIo, Localized<IoError>> {
        Ok(match &self {
            Location::File(path) => LocationIo::File(File::open(path).err_localized(self)?),
            Location::StdIo => LocationIo::StdInOut,
        })
    }

    pub fn writer(&self) -> Result<LocationIo, Localized<IoError>> {
        Ok(match &self {
            Location::File(path) => LocationIo::File(File::create(path).err_localized(self)?),
            Location::StdIo => LocationIo::StdInOut,
        })
    }
}

pub enum LocationIo {
    File(File),
    StdInOut,
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
