use std::{fs::File, path::PathBuf};

use crate::*;
use clap::Args;

#[derive(Args, Debug)]
pub struct InitCommand {}

impl Command for InitCommand {
    fn execute(&self, mut context: CommandContext) {
        context.consume_err(Self::run_init());
    }
}

impl InitCommand {
    fn run_init() -> Result<(), CommandError> {
        let reqtk_json_path = PathBuf::from("reqtk.json");
        let requirements_req_path = PathBuf::from("requirements.req");

        for path in [&reqtk_json_path, &requirements_req_path] {
            if path.exists() {
                return Err(CommandError::Other {
                    severity: IssueSeverity::Error,
                    message: "File already exists".into(),
                    location: Some(Location::Path(path.clone())),
                    span: None,
                });
            }
        }

        let reqtk_json = File::create(&reqtk_json_path)?;
        serde_json::to_writer_pretty(
            reqtk_json,
            &ReqTkJson {
                schema: Some("https://raw.githubusercontent.com/nandee95/reqtk/refs/heads/main/reqtk.schema.json".into()),
                sources: vec!["src/**/*".into()],
                requirements: vec!["requirements.req".into()],
            },
        )?;

        let mut requirements_req = File::create(&requirements_req_path)?;

        TokenWriter::write_reformat(
            &mut requirements_req,
            &[
                //[id-type=incremental]
                TokenValue::AttributeStart,
                TokenValue::AttributeKey("id-type".into()),
                TokenValue::AttributeSeparator,
                TokenValue::AttributeValue("incremental".into()),
                TokenValue::AttributeEnd,
                //[id-prefix=REQ-]
                TokenValue::AttributeStart,
                TokenValue::AttributeKey("id-prefix".into()),
                TokenValue::AttributeSeparator,
                TokenValue::AttributeValue("REQ-".into()),
                TokenValue::AttributeEnd,
                //[id-count=1]
                TokenValue::AttributeStart,
                TokenValue::AttributeKey("id-count".into()),
                TokenValue::AttributeSeparator,
                TokenValue::AttributeValue("1".into()),
                TokenValue::AttributeEnd,
                // @REQ-1(My requirement) {
                TokenValue::RequirementStart,
                TokenValue::Identifier("REQ-1".into()),
                TokenValue::TitleStart,
                TokenValue::Title("My requirement".into()),
                TokenValue::TitleEnd,
                TokenValue::BodyStart,
                // [type=functional]
                TokenValue::AttributeStart,
                TokenValue::AttributeKey("type".into()),
                TokenValue::AttributeSeparator,
                TokenValue::AttributeValue("functional".into()),
                TokenValue::AttributeEnd,
                // [description]Hello world![/description]
                TokenValue::AttributeStart,
                TokenValue::AttributeKey("description".into()),
                TokenValue::AttributeEnd,
                TokenValue::AttributeValue("Hello world!".into()),
                TokenValue::AttributeStart,
                TokenValue::AttributeClose,
                TokenValue::AttributeKey("description".into()),
                TokenValue::AttributeEnd,
                // }
                TokenValue::BodyEnd,
            ],
        )?;
        Ok(())
    }
}
