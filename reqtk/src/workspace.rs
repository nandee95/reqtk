use crate::*;
use adar::prelude::*;
use glob::glob;
use req_file::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

pub struct Workspace {
    pub location: Location,
    pub config: WorkspaceConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkspaceConfig {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    #[serde(default)]
    pub sources: Vec<PathBuf>,
    #[serde(default)]
    pub requirements: Vec<PathBuf>,
}

#[FlagEnum]
pub enum WorkspaceTarget {
    Requirements,
    Sources,
}

impl Workspace {
    pub fn new(location: Location) -> Result<Self, CommandError> {
        let Location::Path(path) = &location else {
            return Err(CommandError::other(
                "Workspace location must be a file path!",
            ));
        };

        let mut reader = location.reader()?;
        let mut config: WorkspaceConfig =
            serde_json::from_reader(&mut reader).err_localized(&location)?;

        config.sources = config
            .sources
            .into_iter()
            .map(|p| path.parent().unwrap_or_else(|| Path::new(".")).join(p))
            .collect();

        config.requirements = config
            .requirements
            .into_iter()
            .map(|p| path.parent().unwrap_or_else(|| Path::new(".")).join(p))
            .collect();

        Ok(Workspace { location, config })
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
        Err(CommandError::other("reqtk.json not found!"))
    }

    pub fn targets(&self, targets: Flags<WorkspaceTarget>) -> Result<Vec<Target>, CommandError> {
        let mut result = Vec::new();
        if targets.any(WorkspaceTarget::Requirements) {
            for pattern in &self.config.requirements {
                for path in glob(pattern.as_os_str().to_str().unwrap())
                    .err_localized(&self.location)?
                    .flatten()
                {
                    if path.is_file() {
                        result.push(Target::file(path));
                    }
                }
            }
        }

        if targets.any(WorkspaceTarget::Sources) {
            for pattern in &self.config.sources {
                for path in glob(pattern.as_os_str().to_str().unwrap())
                    .err_localized(&self.location)?
                    .flatten()
                {
                    if path.is_file() {
                        result.push(Target::file(path));
                    }
                }
            }
        }
        Ok(result)
    }
}
