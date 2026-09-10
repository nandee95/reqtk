use std::io::stderr;

use crate::*;

pub struct IssueTracker {
    issues: Vec<Issue>,
    report: OutputFormat,
}

impl std::ops::Deref for IssueTracker {
    type Target = Vec<Issue>;

    fn deref(&self) -> &Self::Target {
        &self.issues
    }
}

impl std::ops::DerefMut for IssueTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.issues
    }
}

impl IssueTracker {
    pub fn new(report: OutputFormat) -> Self {
        IssueTracker {
            issues: Vec::new(),
            report,
        }
    }

    pub fn consume_issues<T>(&mut self, issues: Result<T, Vec<Issue>>) -> Option<T> {
        match issues {
            Ok(value) => Some(value),
            Err(errs) => {
                self.issues.extend(errs);
                None
            }
        }
    }

    pub fn consume_err<T, E>(&mut self, result: Result<T, E>) -> Option<T>
    where
        E: Into<Issues>,
    {
        match result {
            Ok(value) => Some(value),
            Err(err) => {
                self.issues.extend(err.into());
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
                        eprintln!("{}", issue.report());
                    }
                }
                OutputFormat::Json => {
                    JsonFormatter::serialize_into(stderr(), &self.issues).unwrap();
                }
            }
            self.issues
                .iter()
                .any(|i| matches!(i.severity, IssueSeverity::Error)) as u8
        }
    }
}
