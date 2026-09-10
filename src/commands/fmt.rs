use crate::*;
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct FmtCommand {
    /// Input locations. When missing the inputs are taken from nearest 'reqtk.json'.
    /// Use a single '-' for stdin.
    inputs: Option<Vec<PathBuf>>,
    /// Formatting style to apply.
    #[arg(short, long, default_value_t = Formatting::Reformat)]
    format: Formatting,
}

impl Command for FmtCommand {
    fn execute(&self, mut context: CommandContext) {
        let targets = context.target(
            self.inputs.as_ref().unwrap_or(&Vec::new()),
            &None,
            None,
            ReqTkTargets::Requirements.into(),
        );
        if let Some(targets) = context.consume_err(targets) {
            for target in targets {
                context.consume_err(self.run_fmt(&target));
            }
        }
    }
}
impl FmtCommand {
    fn run_fmt(&self, target: &Target) -> Result<(), CommandError> {
        let tokens = TextTokenizer::tokenize_location(&target.input)?;
        TokenWriter::write_location(&target.output, tokens, self.format)?;
        Ok(())
    }
}
