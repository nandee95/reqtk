use crate::*;
use clap::Args;
use std::{io::stdout, path::PathBuf};

#[derive(Args, Debug)]
pub struct FindCommand {
    /// ID of the requirement to find.
    id: String,
    /// Input locations. When missing the inputs are taken from nearest 'reqtk.json'.
    /// Use a single '-' for stdin.
    inputs: Option<Vec<PathBuf>>,
    /// Output format.
    #[clap(short, long, default_value_t = FindOutputFormat::Req)]
    format: FindOutputFormat,
}

impl Command for FindCommand {
    fn execute(&self, mut context: CommandContext) {
        let targets = context.target(
            self.inputs.as_ref().unwrap_or(&Vec::new()),
            &None,
            Some("find.json"),
            ReqTkTargets::Requirements.into(),
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

            let mut tree = Parser::parse_location(&target.input, tokens)?;
            tree.clear_spans();

            if let Some(requirement) = Self::find_requirement_by_id(&self.id, &tree.requirements) {
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
                    TokenWriter::write_reformat(stdout(), &tokens)?;
                    Ok(())
                }
                FindOutputFormat::Json => Ok(JsonFormatter::serialize_into(stdout(), &result)?),
            }
        } else {
            Err(CommandError::Other {
                severity: IssueSeverity::Error,
                message: format!("Requirement with ID '{}' not found.", self.id),
                location: None,
                span: None,
            })
        }
    }

    fn find_requirement_by_id<'a>(
        id: &str,
        requirements: &'a Vec<Requirement>,
    ) -> Option<&'a Requirement> {
        for requirement in requirements {
            if requirement.id.as_str() == id {
                return Some(requirement);
            }
            if let Some(found) = Self::find_requirement_by_id(id, &requirement.children) {
                return Some(found);
            }
        }
        None
    }
}
