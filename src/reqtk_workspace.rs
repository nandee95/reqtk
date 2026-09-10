use crate::*;
use adar::prelude::*;
use glob::glob;
use serde::{Deserialize, Serialize};
use std::env::current_dir;

pub struct ReqTkWorkspace {
    pub json: ReqTkJson,
    pub location: Location,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReqTkJson {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    #[serde(default)]
    pub sources: Vec<String>,
    #[serde(default)]
    pub requirements: Vec<String>,
}

#[FlagEnum]
pub enum ReqTkTargets {
    Requirements,
    Sources,
}

impl ReqTkWorkspace {
    pub fn new(location: Location) -> Result<Self, CommandError> {
        let mut reader = location.reader()?;
        let json = serde_json::from_reader(&mut reader).err_localized(&location)?;
        Ok(Self { json, location })
    }

    pub fn detect() -> Result<Self, CommandError> {
        let mut reqtk_folder = Some(current_dir()?);

        while let Some(folder) = reqtk_folder {
            let path = folder.join("reqtk.json");
            if path.is_file() {
                let location = Location::Path(path.clone());
                return Self::new(location);
            }

            reqtk_folder = folder.parent().map(|p| p.to_path_buf());
        }
        Err(CommandError::Other {
            severity: IssueSeverity::Error,
            message: "reqtk.json not found!".into(),
            location: Some(Location::Path("reqtk.json".into())),
            span: None,
        })
    }

    pub fn targets(&self, targets: Flags<ReqTkTargets>) -> Result<Vec<Target>, CommandError> {
        let mut result = Vec::new();
        if targets.any(ReqTkTargets::Requirements) {
            for pattern in &self.json.requirements {
                for path in glob(pattern).err_localized(&self.location)?.flatten() {
                    if path.is_file() {
                        result.push(Target::file(path));
                    }
                }
            }
        }

        if targets.any(ReqTkTargets::Sources) {
            for pattern in &self.json.sources {
                for path in glob(pattern).err_localized(&self.location)?.flatten() {
                    if path.is_file() {
                        result.push(Target::file(path));
                    }
                }
            }
        }
        Ok(result)
    }
}
