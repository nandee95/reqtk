use crate::*;
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct TokenizeCommand {
    /// Input locations. When missing the inputs are taken from nearest 'reqtk.json'.
    /// Use a single '-' for stdin.
    // {@REQTK-24}
    inputs: Option<Vec<PathBuf>>,

    /// Output location for single input.
    /// Use a '-' for stdout.
    // {@REQTK-25}
    #[arg(short, long)]
    output: Option<Output>,

    /// Output format.
    // {@REQTK-15}
    #[arg(short, long, default_value_t = OutputFormat::Human)]
    format: OutputFormat,
}

impl Command for TokenizeCommand {
    fn execute(&self, mut context: CommandContext) {
        let targets = context.target(
            self.inputs.as_ref().unwrap_or(&Vec::new()),
            &None,
            Some("tokens.json"),
            ReqTkTargets::Requirements.into(),
        );
        let Some(targets) = context.consume_err(targets) else {
            return;
        };

        for target in &targets {
            context.consume_err(self.run_tokenize(target));
        }
    }
}

impl TokenizeCommand {
    fn run_tokenize(&self, target: &Target) -> Result<(), CommandError> {
        let tokens = TextTokenizer::tokenize_location(&target.input)?;

        match self.format {
            OutputFormat::Human => {
                for token in &tokens {
                    println!(
                        "{}:{} => {:?}",
                        target.input, token.source.span.start, token.token
                    );
                }
            }
            OutputFormat::Json => JsonFormatter::serialize_into(
                target.output.with_extension("json").writer()?,
                &tokens,
            )
            .err_localized(&target.input)?,
        }

        Ok(())
    }
}
