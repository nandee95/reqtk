use crate::*;
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct CheckCommand {
    /// Input locations. When missing the inputs are taken from nearest 'reqtk.json'.
    /// Use a single '-' for stdin.
    // {@REQTK-24}
    inputs: Option<Vec<PathBuf>>,
}

impl Command for CheckCommand {
    fn execute(&self, mut context: CommandContext) {
        let targets = context.target(
            self.inputs.as_ref().unwrap_or(&Vec::new()),
            &None,
            None,
            ReqTkTargets::Requirements.into(),
        );
        if let Some(targets) = context.consume_err(targets) {
            for target in targets {
                context.consume_err(Self::run_check(&target));
            }
        }
    }
}

impl CheckCommand {
    fn run_check(target: &Target) -> Result<(), CommandError> {
        let tokens = TextTokenizer::tokenize_location(&target.input)?;
        let tree = Parser::parse_location(&target.input, tokens)?;
        Verifier::verify_location(&target.input, &tree)?;
        Ok(())
    }
}
