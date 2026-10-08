use std::io::stderr;

use crate::*;
use clap::builder::styling::{AnsiColor, Reset};
use req_file::prelude::*;
use serde::{Deserialize, Serialize};

pub type Issues = Vec<Localized<Spanned<Issue>>>;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum IssueSeverity {
    Error,
    Warning,
}

impl IssueSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            IssueSeverity::Error => "error",
            IssueSeverity::Warning => "warning",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Issue {
    pub severity: IssueSeverity,
    pub message: String,
}

impl Issue {
    pub fn new(severity: IssueSeverity, message: impl Into<String>) -> Self {
        Self {
            severity,
            message: message.into(),
        }
    }
}

impl From<CommandError> for Issues {
    fn from(error: CommandError) -> Self {
        error
            .errors
            .into_iter()
            .map(|v| {
                Localized::new(
                    Spanned::new(
                        match v.value.value {
                            CommandErrorKind::Io(error) => {
                                Issue::new(IssueSeverity::Error, error.to_string())
                            }
                            CommandErrorKind::Parse(error) => {
                                Issue::new(IssueSeverity::Error, error.to_string())
                            }
                            CommandErrorKind::Verify(error) => {
                                Issue::new(IssueSeverity::Error, error.to_string())
                            }
                            CommandErrorKind::Glob(error) => {
                                Issue::new(IssueSeverity::Error, error.to_string())
                            }
                            CommandErrorKind::Json(error) => {
                                Issue::new(IssueSeverity::Error, error.to_string())
                            }
                            CommandErrorKind::Other(message) => {
                                Issue::new(IssueSeverity::Error, message)
                            }
                        },
                        v.value.span,
                    ),
                    v.location.clone(),
                )
            })
            .collect()
    }
}

pub struct IssueTracker {
    issues: Issues,
    report: OutputFormat,
}

impl IssueTracker {
    pub fn new(report: OutputFormat) -> Self {
        IssueTracker {
            issues: Vec::new(),
            report,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.issues.is_empty()
    }

    pub fn has_error(&self) -> bool {
        self.issues
            .iter()
            .any(|v| v.value.value.severity == IssueSeverity::Error)
    }

    pub fn push_issue(&mut self, issue: impl Into<Localized<Spanned<Issue>>>) {
        self.issues.push(issue.into());
    }

    pub fn consume_err<T, E>(&mut self, result: Result<T, E>) -> Option<T>
    where
        E: Into<CommandError>,
    {
        match result {
            Ok(value) => Some(value),
            Err(err) => {
                let error: CommandError = err.into();
                let issues: Issues = error.into();
                self.issues.extend(issues);
                None
            }
        }
    }

    pub fn report(&self) -> u8 {
        if self.issues.is_empty() {
            0
        } else {
            match self.report {
                OutputFormat::Human => {
                    for issue in &self.issues {
                        eprintln!(
                            "{}{color}{}{reset}: {}",
                            issue
                                .location
                                .as_ref()
                                .map(|l| format!("{} ", l))
                                .unwrap_or_default(),
                            issue.value.value.severity.as_str(),
                            issue.value.value.message,
                            color = match issue.value.value.severity {
                                IssueSeverity::Error => AnsiColor::Red,
                                IssueSeverity::Warning => AnsiColor::Yellow,
                            }
                            .render_fg(),
                            reset = Reset,
                        )
                    }
                }
                OutputFormat::Json => {
                    JsonFormatter::serialize_into(stderr(), &self.issues).unwrap();
                }
            }
            self.issues
                .iter()
                .any(|i| matches!(i.value.value.severity, IssueSeverity::Error)) as u8
        }
    }
}
