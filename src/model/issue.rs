use crate::*;
use clap::builder::styling::{AnsiColor, Reset};
use serde::{Deserialize, Serialize};

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
pub struct IssueLocation {
    pub location: Location,
    pub span: Option<Span>,
}

impl std::fmt::Display for IssueLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.span {
            Some(span) => write!(
                f,
                "{}:{}:{}",
                self.location, span.start.line, span.start.column
            ),
            None => write!(f, "{}", self.location),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Issue {
    pub severity: IssueSeverity,
    pub location: Option<IssueLocation>,
    pub message: String,
}

impl Issue {
    pub fn new(severity: IssueSeverity, message: String) -> Self {
        Self {
            severity,
            location: None,
            message,
        }
    }

    pub fn new_location(severity: IssueSeverity, location: Location, message: String) -> Self {
        Self {
            severity,
            location: Some(IssueLocation {
                location,
                span: None,
            }),
            message,
        }
    }

    pub fn new_location_span(
        severity: IssueSeverity,
        location: Location,
        span: Span,
        message: String,
    ) -> Self {
        Self {
            severity,
            location: Some(IssueLocation {
                location,
                span: Some(span),
            }),
            message,
        }
    }

    pub fn report(&self) -> String {
        format!(
            "{}{color}{}{reset}: {}",
            self.location
                .as_ref()
                .map(|l| format!("{} ", l))
                .unwrap_or_default(),
            self.severity.as_str(),
            self.message,
            color = match self.severity {
                IssueSeverity::Error => AnsiColor::Red,
                IssueSeverity::Warning => AnsiColor::Yellow,
            }
            .render_fg(),
            reset = Reset,
        )
    }
}

pub type Issues = Vec<Issue>;
