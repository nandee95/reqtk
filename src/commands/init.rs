use std::fs::File;

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
        let location = Location::File("reqtk.json".into());

        if let Location::File(ref path) = location
            && path.exists()
        {
            return Err(CommandError::Other {
                severity: IssueSeverity::Error,
                message: "File already exists".into(),
                location: Some(location),
                span: None,
            });
        }

        let file = File::create("reqtk.json")?;
        serde_json::to_writer_pretty(
            file,
            &ReqTkJson {
                schema: Some("https://raw.githubusercontent.com/nandee95/reqtk/refs/heads/main/reqtk.schema.json".into()),
                sources: vec!["src/**/*".into()],
                requirements: vec!["requirements.req".into()],
            },
        )?;
        Ok(())
    }
}
