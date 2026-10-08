use crate::*;
use clap::Args;
use req_file::prelude::*;
use std::path::PathBuf;

#[derive(Args, Debug)]
// {@REQTK-5}
pub struct ConvertCommand {
    /// Input locations. When missing the inputs are taken from nearest 'reqtk.json'.
    /// Use a single '-' for stdin.
    // {@REQTK-27}
    inputs: Option<Vec<PathBuf>>,

    /// Output location for single input.
    /// Use a '-' for stdout.
    // {@REQTK-28}
    #[arg(short, long)]
    output: Option<Output>,

    /// Input format  (defaults to extension of <INPUT> if auto)
    // {@REQTK-6}
    #[arg(short, long, default_value="auto", value_parser=validate_from)]
    from: String,

    /// Output format (defaults to extension of --output if auto)
    // {@REQTK-7}
    #[arg(short, long, default_value="auto", value_parser=validate_to)]
    to: String,

    /// Include spans (for json format only)
    // {@REQTK-8}
    #[arg(long)]
    span: bool,
}

impl Command for ConvertCommand {
    fn execute(&self, context: &mut CommandContext) {
        let Some(to) = context.consume_err(self.get_output_extension()) else {
            return;
        };

        let targets = context.target(
            self.inputs.as_ref().unwrap_or(&Vec::new()),
            &self.output,
            Some(&to),
            WorkspaceTarget::Requirements.into(),
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
                    return Err(CommandError::other("output doesn't have an extension"));
                }
            } else {
                return Err(CommandError::other(
                    "failed to detect output format, provide with --to",
                ));
            }
        } else {
            self.to.clone()
        };

        match validate_to(&extension) {
            Ok(value) => Ok(value),
            Err(error) => Err(CommandError::other(error)),
        }
    }

    fn get_input_extension(&self, target: &Target) -> Result<String, CommandError> {
        let extension = if self.from == "auto" {
            if let Location::Path(path) = &target.input {
                if let Some(extension) = path.extension().and_then(|v| v.to_str()) {
                    extension.to_string()
                } else {
                    return Err(CommandError::other("input doesn't have an extension"));
                }
            } else {
                return Err(CommandError::other(
                    "failed to detect input format, provide with --from",
                ));
            }
        } else {
            self.from.clone()
        };

        match validate_from(&extension) {
            Ok(value) => Ok(value),
            Err(error) => Err(CommandError::other(error)),
        }
    }

    fn run_convert(&self, to: &str, from: &str, target: &Target) -> Result<(), CommandError> {
        let importer = self.get_importer(from)?;
        let exporter = self.get_exporter(to)?;

        let tree = importer.import(&target.input)?;
        exporter.export(&target.output, tree)?;

        Ok(())
    }

    fn get_importer(&self, format: &str) -> Result<Box<dyn FormatImport>, CommandError> {
        let importers: [Box<dyn FormatImport>; 2] =
            [Box::new(Req), Box::new(Json { spans: self.span })];

        let extensions = importers
            .iter()
            .map(|importer| importer.extension())
            .collect::<Vec<_>>();
        importers
            .into_iter()
            .find(|v| v.extension() == format)
            .ok_or_else(|| {
                CommandError::other(format!(
                    "unrecognized input extension {}, expected one of: {}",
                    format,
                    extensions.join(", ")
                ))
            })
    }

    fn get_exporter(&self, format: &str) -> Result<Box<dyn FormatExport>, CommandError> {
        let exporters: [Box<dyn FormatExport>; 3] = [
            Box::new(Req),
            Box::new(Json { spans: self.span }),
            Box::new(Markdown),
        ];
        let extensions = exporters
            .iter()
            .map(|exporter| exporter.extension())
            .collect::<Vec<_>>();
        exporters
            .into_iter()
            .find(|v| v.extension() == format)
            .ok_or_else(|| {
                CommandError::other(format!(
                    "unrecognized output extension, expected one of: {}",
                    extensions.join(", ")
                ))
            })
    }
}

pub fn validate_to(value: &str) -> Result<String, String> {
    if value == "auto" {
        return Ok(value.to_string());
    }
    let exporters: [Box<dyn FormatExport>; 3] = [
        Box::new(Req),
        Box::new(Json { spans: false }),
        Box::new(Markdown),
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
    let exporters: [Box<dyn FormatImport>; 2] = [Box::new(Req), Box::new(Json { spans: false })];
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
