use crate::*;
use clap::{
    Args, ValueEnum,
    builder::styling::{AnsiColor, Reset},
};
use req_file::prelude::*;
use std::{
    io::{BufReader, BufWriter, Read},
    path::PathBuf,
};

#[derive(ValueEnum, Debug, Clone, Copy)]
enum ClapFormatting {
    Preserve,
    Reformat,
    Minify,
}

impl std::fmt::Display for ClapFormatting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_possible_value().unwrap().get_name())
    }
}

impl From<ClapFormatting> for Formatting {
    fn from(val: ClapFormatting) -> Self {
        match val {
            ClapFormatting::Preserve => Formatting::Preserve,
            ClapFormatting::Reformat => Formatting::Reformat,
            ClapFormatting::Minify => Formatting::Minify,
        }
    }
}

#[derive(Args, Debug)]
// {@REQTK-9}
pub struct FormatCommand {
    /// Input locations. When missing the inputs are taken from nearest 'reqtk.json'.
    /// Use a single '-' for stdin.
    // {@REQTK-27}
    inputs: Option<Vec<PathBuf>>,

    /// Output location for single input.
    /// Use a '-' for stdout.
    // {@REQTK-28}
    #[arg(short, long)]
    output: Option<Output>,

    /// Formatting style to apply.
    // {@REQTK-10}
    #[arg(short, long, default_value_t = ClapFormatting::Reformat)]
    format: ClapFormatting,

    /// Show differences instead of writing to the output(s).
    // {@REQTK-11}
    #[arg(long)]
    diff: bool,
}

impl Command for FormatCommand {
    fn execute(&self, context: &mut CommandContext) {
        let targets = context.target(
            self.inputs.as_ref().unwrap_or(&Vec::new()),
            &self.output,
            None,
            WorkspaceTarget::Requirements.into(),
        );

        if self.diff && self.output.is_some() {
            context.issues.push_issue(Issue::new(
                IssueSeverity::Error,
                "Diff mode cannot be used with an output specified.",
            ));
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
            TokenWriter::write(&mut output, tokens, self.format.into())?;
            let output = output.into_inner().expect("Failed to flush buffer");
            let output = String::from_utf8_lossy(&output);

            let mut changed = false;
            for diff in DiffIterator::new(&input, &output) {
                changed = true;
                let span = if diff.from.value.is_empty() {
                    diff.to.span
                } else {
                    diff.from.span
                };
                if let Some(span) = span {
                    println!(
                        "{}@ {}:{} => {}:{}{}",
                        AnsiColor::Cyan.render_fg(),
                        target.input,
                        span.start,
                        target.input,
                        span.end,
                        Reset
                    );
                } else {
                    println!("{}@ {}{}", AnsiColor::Cyan.render_fg(), target.input, Reset);
                }
                if !diff.from.value.is_empty() {
                    for line in diff.from.value.lines() {
                        println!("{}- {}{}", AnsiColor::Red.render_fg(), line, Reset);
                    }
                }
                if !diff.to.value.is_empty() {
                    for line in diff.to.value.lines() {
                        println!("{}+ {}{}", AnsiColor::Green.render_fg(), line, Reset);
                    }
                }
            }
            if changed {
                return Err(CommandError::other("Changes detected."));
            }
        } else {
            let tokens = TextTokenizer::tokenize(&mut target.input.reader()?)
                .err_localized(&target.input)?;
            TokenWriter::write(&mut target.output.writer(true)?, tokens, self.format.into())
                .err_localized(&target.output)?;
        }
        Ok(())
    }
}
