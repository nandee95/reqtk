use crate::*;
use clap::Args;
use req_file::prelude::*;
use std::{io::stdout, path::PathBuf};

#[derive(Args, Debug)]
// {@REQTK-17}
pub struct FindCommand {
    /// ID of the requirement to find.
    // {@REQTK-18}
    id: String,

    /// Input locations. When missing the inputs are taken from nearest 'reqtk.json'.
    /// Use a single '-' for stdin.
    // {@REQTK-27}
    inputs: Option<Vec<PathBuf>>,

    /// Output format.
    // {@REQTK-19}
    #[clap(short, long, default_value_t = FindOutputFormat::Req)]
    format: FindOutputFormat,
}

impl Command for FindCommand {
    fn execute(&self, context: &mut CommandContext) {
        let targets = context.target(
            self.inputs.as_ref().unwrap_or(&Vec::new()),
            &None,
            Some("find.json"),
            WorkspaceTarget::Requirements.into(),
        );
        let Some(targets) = context.consume_err(targets) else {
            return;
        };

        context.consume_err(self.run_find(targets));
    }
}

impl FindCommand {
    fn run_find(&self, targets: Vec<Target>) -> Result<(), CommandError> {
        let mut result = Vec::new();
        for target in targets {
            let tokens = TextTokenizer::tokenize(&mut target.input.reader()?)
                .err_localized(&target.input)?;

            let mut tree = TokenParser::parse(tokens).err_localized(&target.input)?;
            tree.clear_spans();

            if let Some(requirement) = tree.find(&self.id) {
                result.push((*requirement).clone());
            }
        }

        if !result.is_empty() {
            match self.format {
                FindOutputFormat::Req => {
                    let tokens = TreeTokenizer::tokenize(&RequirementTree {
                        requirements: result,
                        ..Default::default()
                    });
                    TokenWriter::write_reformat(&mut stdout(), &tokens)?;
                    Ok(())
                }
                FindOutputFormat::Json => Ok(JsonFormatter::serialize_into(stdout(), &result)?),
            }
        } else {
            Err(CommandError::other(format!(
                "Requirement with ID '{}' not found.",
                self.id
            )))
        }
    }
}
