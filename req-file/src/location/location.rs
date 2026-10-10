use std::path::PathBuf;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Location {
    Path(PathBuf),
    StdIo,
}

#[cfg(feature = "serde")]
impl serde::Serialize for Location {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Location {
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
}
