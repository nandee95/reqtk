use crate::*;
use clap::Args;
use req_file::prelude::*;
use std::path::PathBuf;

#[derive(Args, Debug)]
// {@REQTK-22}
pub struct CheckCommand {
    /// Input locations. When missing the inputs are taken from nearest 'reqtk.json'.
    /// Use a single '-' for stdin.
    // {@REQTK-27}
    inputs: Option<Vec<PathBuf>>,
}

impl Command for CheckCommand {
    fn execute(&self, context: &mut CommandContext) {
        let targets = context.target(
            self.inputs.as_ref().unwrap_or(&Vec::new()),
            &None,
            None,
            WorkspaceTarget::Requirements.into(),
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
        let mut reader = target.input.reader()?;
        let tokens = TextTokenizer::tokenize(&mut reader)?;
        let tree = TokenParser::parse(tokens).err_localized(&target.input)?;
        Verifier::verify(&tree).err_localized(&target.input)?;
        Ok(())
    }
}
