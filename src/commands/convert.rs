use crate::*;
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct ConvertCommand {
    /// Input locations. When missing the inputs are taken from nearest 'reqtk.json'.
    /// Use a single '-' for stdin.
    inputs: Option<Vec<PathBuf>>,
    /// Output file for single input.
    /// Use a '-' for stdout.
    #[arg(short, long)]
    output: Option<Output>,
    /// Input format  (defaults to extension of <INPUT> if auto)
    #[arg(short, long, default_value="auto", value_parser=validate_from)]
    pub from: String,
    /// Output format (defaults to extension of --output if auto)
    #[arg(short, long, default_value="auto", value_parser=validate_to)]
    pub to: String,
    /// Include spans (for json format only)
    #[arg(long)]
    pub span: bool,
}

impl Command for ConvertCommand {
    fn execute(&self, mut context: CommandContext) {
        let Some(to) = context.consume_err(self.get_output_extension()) else {
            return;
        };

        let targets = context.target(
            self.inputs.as_ref().unwrap_or(&Vec::new()),
            &self.output,
            Some(&to),
            ReqTkTargets::Requirements.into(),
        );
        let Some(targets) = context.consume_err(targets) else {
            return;
        };

        for target in targets {
            let Some(from) = context.consume_err(self.get_input_extension(&target)) else {
                return;
            };
            context.consume_err(self.run_convert(&to, &from, &target));
        }
    }
}

impl ConvertCommand {
    fn get_output_extension(&self) -> Result<String, CommandError> {
        let extension = if self.to == "auto" {
            if let Some(Output::Path(path)) = &self.output {
                if let Some(extension) = path.extension().and_then(|v| v.to_str()) {
                    extension.to_string()
                } else {
                    return Err(CommandError::other(
                        IssueSeverity::Error,
                        "output doesn't have an extension".into(),
                    ));
                }
            } else {
                return Err(CommandError::other(
                    IssueSeverity::Error,
                    "failed to detect output format, provide with --to".into(),
                ));
            }
        } else {
            self.to.clone()
        };

        match validate_to(&extension) {
            Ok(value) => Ok(value),
            Err(error) => Err(CommandError::other(IssueSeverity::Error, error)),
        }
    }

    fn get_input_extension(&self, target: &Target) -> Result<String, CommandError> {
        let extension = if self.from == "auto" {
            if let Location::File(path) = &target.input {
                if let Some(extension) = path.extension().and_then(|v| v.to_str()) {
                    extension.to_string()
                } else {
                    return Err(CommandError::other(
                        IssueSeverity::Error,
                        "input doesn't have an extension".into(),
                    ));
                }
            } else {
                return Err(CommandError::other(
                    IssueSeverity::Error,
                    "failed to detect input format, provide with --from".into(),
                ));
            }
        } else {
            self.from.clone()
        };

        match validate_from(&extension) {
            Ok(value) => Ok(value),
            Err(error) => Err(CommandError::other(IssueSeverity::Error, error)),
        }
    }

    fn run_convert(&self, to: &str, from: &str, target: &Target) -> Result<(), CommandError> {
        let importer = self.get_importer(from)?;
        let exporter = self.get_exporter(to)?;

        let tree = importer.import(&target.input)?;
        exporter.export(&target.output, tree)?;

        Ok(())
    }

    fn get_importer(&self, format: &str) -> Result<Box<dyn RequirementTreeImport>, CommandError> {
        let importers: [Box<dyn RequirementTreeImport>; 2] =
            [Box::new(ReqImpex), Box::new(JsonImpex { spans: self.span })];

        let extensions = importers
            .iter()
            .map(|importer| importer.extension())
            .collect::<Vec<_>>();
        importers
            .into_iter()
            .find(|v| v.extension() == format)
            .ok_or_else(|| CommandError::Other {
                severity: IssueSeverity::Error,
                message: format!(
                    "unrecognized input extension {}, expected one of: {}",
                    format,
                    extensions.join(", ")
                ),
                location: None,
                span: None,
            })
    }

    fn get_exporter(&self, format: &str) -> Result<Box<dyn RequirementTreeExport>, CommandError> {
        let exporters: [Box<dyn RequirementTreeExport>; 3] = [
            Box::new(ReqImpex),
            Box::new(JsonImpex { spans: self.span }),
            Box::new(MdEx),
        ];
        let extensions = exporters
            .iter()
            .map(|exporter| exporter.extension())
            .collect::<Vec<_>>();
        exporters
            .into_iter()
            .find(|v| v.extension() == format)
            .ok_or_else(|| CommandError::Other {
                severity: IssueSeverity::Error,
                message: format!(
                    "unrecognized output extension, expected one of: {}",
                    extensions.join(", ")
                ),
                location: None,
                span: None,
            })
    }
}

pub fn validate_to(value: &str) -> Result<String, String> {
    if value == "auto" {
        return Ok(value.to_string());
    }
    let exporters: [Box<dyn RequirementTreeExport>; 3] = [
        Box::new(ReqImpex),
        Box::new(JsonImpex { spans: false }),
        Box::new(MdEx),
    ];
    if exporters.iter().any(|e| e.extension() == value) {
        Ok(value.to_string())
    } else {
        Err(format!(
            "invalid format '{}', expected one of: {}",
            value,
            exporters
                .iter()
                .map(|e| e.extension())
                .collect::<Vec<_>>()
                .join(", ")
        ))
    }
}

pub fn validate_from(value: &str) -> Result<String, String> {
    if value == "auto" {
        return Ok(value.to_string());
    }
    let exporters: [Box<dyn RequirementTreeImport>; 2] =
        [Box::new(ReqImpex), Box::new(JsonImpex { spans: false })];
    if exporters.iter().any(|e| e.extension() == value) {
        Ok(value.to_string())
    } else {
        Err(format!(
            "invalid format '{}', expected one of: {}",
            value,
            exporters
                .iter()
                .map(|e| e.extension())
                .collect::<Vec<_>>()
                .join(", ")
        ))
    }
}
