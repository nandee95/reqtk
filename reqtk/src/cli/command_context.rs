use crate::*;
use adar::prelude::*;
use clap::builder::styling::{AnsiColor, Reset};
use req_file::prelude::*;
use std::{ffi::OsStr, path::PathBuf};

macro_rules! print_verbose {
    ($($arg:tt)*) => {
        println!("{}verbose{}: {}", AnsiColor::Blue.render_fg(), Reset, format_args!($($arg)*))
    };
}

pub struct Target {
    pub input: Location,
    pub output: Location,
}

impl Target {
    pub fn file(path: PathBuf) -> Self {
        let location = Location::Path(path);
        Self {
            input: location.clone(),
            output: location,
        }
    }

    pub fn stdio() -> Self {
        Self {
            input: Location::StdIo,
            output: Location::StdIo,
        }
    }

    pub fn with_output_extension(self, extension: &str) -> Self {
        Self {
            input: self.input,
            output: self.output.with_extension(extension),
        }
    }
}

pub struct CommandContext {
    pub issues: IssueTracker,
    pub verbose: bool,
}

impl CommandContext {
    const INPUT_STDIN: &'static str = "-";
    pub fn new(cli: &Cli) -> CommandContext {
        Self {
            issues: IssueTracker::new(cli.report),
            verbose: cli.verbose,
        }
    }

    pub fn consume_err<J, E>(&mut self, result: Result<J, E>) -> Option<J>
    where
        E: Into<CommandError>,
    {
        self.issues.consume_err(result.map_err(|e| e.into()))
    }
    pub fn target(
        &mut self,
        inputs: &[PathBuf],
        output: &Option<Output>,
        output_extension: Option<&str>,
        targets: Flags<WorkspaceTarget>,
    ) -> Result<Vec<Target>, CommandError> {
        let mut result = Vec::new();
        if inputs.is_empty() {
            let workspace = Workspace::detect()?;
            if self.verbose {
                print_verbose!("Detected workspace: {:?}", workspace.location);
            }
            result.extend(workspace.targets(targets)?);
        } else {
            if self.verbose {
                print_verbose!("Workspace is ignored! (files specified by args)");
            }
            for v in inputs.iter() {
                if v == &PathBuf::from(Self::INPUT_STDIN) {
                    result.push(Target::stdio());
                    continue;
                } else if v.file_name() == Some(OsStr::new("reqtk.json")) {
                    result.extend(Workspace::new(Location::Path(v.clone()))?.targets(targets)?);
                } else {
                    result.push(Target::file(PathBuf::from(v)));
                }
            }
        }

        if let Some(output) = output {
            if result.len() == 1 {
                result[0].output = match output {
                    Output::Stdout => Location::StdIo,
                    Output::Path(path) => Location::Path(path.clone()),
                };
            } else {
                return Err(CommandError::other(
                    "Output can only be used with exactly one input",
                ));
            }
        }

        if let Some(extension) = output_extension {
            result = result
                .into_iter()
                .map(|v| v.with_output_extension(extension))
                .collect();
        }

        if result.is_empty() {
            self.issues
                .push_issue(Issue::new(IssueSeverity::Warning, "No inputs found"));
        }

        if self.verbose {
            for target in &result {
                print_verbose!("Resolved target: {} => {}", target.input, target.output);
            }
        }

        Ok(result)
    }
}
