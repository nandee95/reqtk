use crate::*;
use clap::Args;
use req_file::prelude::*;
use std::{fs::File, path::PathBuf};

#[derive(Args, Debug)]
// {@REQTK-24}
pub struct InitCommand {}

impl Command for InitCommand {
    fn execute(&self, context: &mut CommandContext) {
        context.consume_err(Self::run_init());
    }
}

impl InitCommand {
    fn run_init() -> Result<(), CommandError> {
        let reqtk_json_path = PathBuf::from("reqtk.json");
        let requirements_req_path = PathBuf::from("requirements.req");

        for path in [&reqtk_json_path, &requirements_req_path] {
            if path.exists() {
                return Err(CommandError::other("File already exists"));
            }
        }

        let reqtk_json = File::create(&reqtk_json_path)?;
        serde_json::to_writer_pretty(
            reqtk_json,
            &WorkspaceConfig {
                schema: Some("https://raw.githubusercontent.com/nandee95/reqtk/refs/heads/main/reqtk/reqtk.schema.json".into()),
                sources: vec!["src/**/*".into()],
                requirements: vec!["requirements.req".into()],
            },
        )?;

        let mut requirements_req = File::create(&requirements_req_path)?;

        TokenWriter::write_reformat(
            &mut requirements_req,
            &[
                //[id-type=incremental]
                TokenKind::AttributeStart,
                TokenKind::AttributeKey(Attribute::FILE_ID_TYPE.into()),
                TokenKind::AttributeSeparator,
                TokenKind::AttributeValue("incremental".into()),
                TokenKind::AttributeEnd,
                //[id-prefix=REQ-]
                TokenKind::AttributeStart,
                TokenKind::AttributeKey(Attribute::FILE_ID_PREFIX.into()),
                TokenKind::AttributeSeparator,
                TokenKind::AttributeValue("REQ-".into()),
                TokenKind::AttributeEnd,
                //[id-count=1]
                TokenKind::AttributeStart,
                TokenKind::AttributeKey(Attribute::FILE_ID_COUNT.into()),
                TokenKind::AttributeSeparator,
                TokenKind::AttributeValue("1".into()),
                TokenKind::AttributeEnd,
                // @REQ-1(My requirement) {
                TokenKind::RequirementStart,
                TokenKind::Identifier("REQ-1".into()),
                TokenKind::TitleStart,
                TokenKind::Title("My requirement".into()),
                TokenKind::TitleEnd,
                TokenKind::BodyStart,
                // [type=functional]
                TokenKind::AttributeStart,
                TokenKind::AttributeKey(Attribute::REQ_TYPE.into()),
                TokenKind::AttributeSeparator,
                TokenKind::AttributeValue("functional".into()),
                TokenKind::AttributeEnd,
                // [description]Hello world![/description]
                TokenKind::AttributeStart,
                TokenKind::AttributeKey(Attribute::REQ_DESCRIPTION.into()),
                TokenKind::AttributeEnd,
                TokenKind::AttributeValue("Hello world!".into()),
                TokenKind::AttributeStart,
                TokenKind::AttributeClose,
                TokenKind::AttributeKey(Attribute::REQ_DESCRIPTION.into()),
                TokenKind::AttributeEnd,
                // }
                TokenKind::BodyEnd,
            ],
        )?;
        Ok(())
    }
}
