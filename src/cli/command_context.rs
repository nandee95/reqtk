use crate::*;
use adar::prelude::*;
use glob::glob;
use std::path::PathBuf;

#[FlagEnum]
pub enum ReqTkTargets {
    Requirements,
    Sources,
}

pub struct Target {
    pub input: Location,
    pub output: Location,
}

impl Target {
    pub fn file(path: PathBuf) -> Self {
        let location = Location::File(path);
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

pub struct CommandContext<'a> {
    issues: &'a mut IssueTracker,
}

impl<'a> CommandContext<'a> {
    const INPUT_STDIN: &'static str = "-";
    pub fn new(issues: &'a mut IssueTracker) -> CommandContext<'a> {
        Self { issues }
    }

    pub fn push_issue(&mut self, issue: Issue) {
        self.issues.push(issue);
    }

    pub fn extend_issues(&mut self, issues: Issues) {
        self.issues.extend(issues);
    }

    pub fn has_error(&self) -> bool {
        self.issues
            .iter()
            .any(|v| v.severity == IssueSeverity::Error)
    }

    pub fn consume_err<J, E>(&mut self, result: Result<J, E>) -> Option<J>
    where
        E: Into<CommandError>,
    {
        self.issues.consume_err(result.map_err(|e| e.into()))
    }
    pub fn target(
        &mut self,
        inputs: &Vec<PathBuf>,
        output: &Option<Output>,
        output_extension: Option<&str>,
        targets: Flags<ReqTkTargets>,
    ) -> Result<Vec<Target>, CommandError> {
        let mut result = Vec::new();
        if inputs.is_empty() {
            let (reqtk_json_path, reqtk_json) = ReqTkJson::get()?;

            if targets.any(ReqTkTargets::Requirements) {
                for pattern in &reqtk_json.requirements {
                    for path in glob(pattern)
                        .err_localized(&Location::File(reqtk_json_path.clone()))?
                        .flatten()
                    {
                        if path.is_file() {
                            result.push(Target::file(path));
                        }
                    }
                }
            }

            if targets.any(ReqTkTargets::Sources) {
                for pattern in &reqtk_json.sources {
                    for path in glob(pattern)
                        .err_localized(&Location::File(reqtk_json_path.clone()))?
                        .flatten()
                    {
                        if path.is_file() {
                            result.push(Target::file(path));
                        }
                    }
                }
            }
        } else if inputs.as_slice() == [PathBuf::from(Self::INPUT_STDIN)] {
            result.push(Target::stdio());
        } else {
            result.extend(inputs.iter().map(|v| Target::file(PathBuf::from(v))));
        }

        if let Some(output) = output {
            if result.len() == 1 {
                result[0].output = match output {
                    Output::Stdout => Location::StdIo,
                    Output::Path(path) => Location::File(path.clone()),
                };
            } else {
                return Err(CommandError::Other {
                    severity: IssueSeverity::Error,
                    message: "Output can only be used with exactly one input".into(),
                    location: None,
                    span: None,
                });
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
                .push(Issue::new(IssueSeverity::Warning, "No inputs found".into()));
        }

        Ok(result)
    }
}
