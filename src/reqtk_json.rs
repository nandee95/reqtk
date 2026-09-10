use crate::*;
use serde::{Deserialize, Serialize};
use std::{env::current_dir, path::PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct ReqTkJson {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    #[serde(default)]
    pub sources: Vec<String>,
    #[serde(default)]
    pub requirements: Vec<String>,
}

impl ReqTkJson {
    pub fn get() -> Result<(PathBuf, ReqTkJson), CommandError> {
        let mut reqtk_folder = Some(current_dir()?);

        while let Some(folder) = reqtk_folder {
            let path = folder.join("reqtk.json");
            let location = Location::File(path.clone());
            if let Ok(mut reader) = location.reader() {
                let reqtkjson = serde_json::from_reader(&mut reader).err_localized(&location)?;
                return Ok((path, reqtkjson));
            }

            reqtk_folder = folder.parent().map(|p| p.to_path_buf());
        }

        Err(CommandError::Other {
            severity: IssueSeverity::Error,
            message: "reqtk.json not found!".into(),
            location: None,
            span: None,
        })
    }
}
