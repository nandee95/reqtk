use crate::*;
use clap::{
    Args,
    builder::styling::{AnsiColor, Reset},
};
use std::{
    io::{BufReader, BufWriter, Read},
    path::PathBuf,
};

#[derive(Args, Debug)]
pub struct FormatCommand {
    /// Input locations. When missing the inputs are taken from nearest 'reqtk.json'.
    /// Use a single '-' for stdin.
    // {@REQTK-24}
    inputs: Option<Vec<PathBuf>>,

    /// Output location for single input.
    /// Use a '-' for stdout.
    // {@REQTK-25}
    #[arg(short, long)]
    output: Option<Output>,

    /// Formatting style to apply.
    // {@REQTK-9}
    #[arg(short, long, default_value_t = Formatting::Reformat)]
    format: Formatting,

    /// Show differences instead of writing to the output(s).
    #[arg(long)]
    diff: bool,
}

impl Command for FormatCommand {
    fn execute(&self, mut context: CommandContext) {
        let targets = context.target(
            self.inputs.as_ref().unwrap_or(&Vec::new()),
            &self.output,
            None,
            ReqTkTargets::Requirements.into(),
        );

        if self.diff && self.output.is_some() {
            context.push_issue(Issue {
                severity: IssueSeverity::Error,
                location: None,
                message: "Diff mode cannot be used with an output specified.".into(),
            });
            return;
        }
        if let Some(targets) = context.consume_err(targets) {
            for target in targets {
                context.consume_err(self.run_fmt(&target));
            }
        }
    }
}
impl FormatCommand {
    fn run_fmt(&self, target: &Target) -> Result<(), CommandError> {
        if self.diff {
            let mut input = String::new();
            target
                .input
                .reader()?
                .read_to_string(&mut input)
                .err_localized(&target.input)?;
            let mut input_reader = BufReader::new(input.as_bytes());
            let tokens = TextTokenizer::tokenize(&mut input_reader)?;
            let mut output = BufWriter::new(Vec::new());
            TokenWriter::write(&mut output, tokens, self.format)?;
            let output = output.into_inner().expect("Failed to flush buffer");
            let output = String::from_utf8_lossy(&output);

            let mut changed = false;
            for diff in DiffIterator::new(&input, &output) {
                changed = true;
                let span = if diff.from.is_empty() {
                    diff.to.span
                } else {
                    diff.from.span
                };
                println!(
                    "{}@ {}:{} => {}:{}{}",
                    AnsiColor::Cyan.render_fg(),
                    target.input,
                    span.start,
                    target.input,
                    span.end,
                    Reset
                );
                if !diff.from.is_empty() {
                    for line in diff.from.lines() {
                        println!("{}- {}{}", AnsiColor::Red.render_fg(), line, Reset);
                    }
                }
                if !diff.to.is_empty() {
                    for line in diff.to.lines() {
                        println!("{}+ {}{}", AnsiColor::Green.render_fg(), line, Reset);
                    }
                }
            }
            if changed {
                return Err(CommandError::Other {
                    severity: IssueSeverity::Error,
                    message: "Changes detected.".into(),
                    location: None,
                    span: None,
                });
            }
        } else {
            let tokens = TextTokenizer::tokenize_location(&target.input)?;
            TokenWriter::write_location(&target.output, tokens, self.format)?;
        }
        Ok(())
    }
}
