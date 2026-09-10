use glob::PatternError;
use std::io::Error as IoError;

use crate::*;

#[derive(Debug)]
pub enum CommandError {
    Io(Localized<std::io::Error>),
    Parse(Vec<Localized<Spanned<ParseError>>>),
    Verify(Vec<Localized<Spanned<VerifyError>>>),
    Other {
        severity: IssueSeverity,
        message: String,
        location: Option<Location>,
        span: Option<Span>,
    },
}

impl CommandError {
    pub fn other(severity: IssueSeverity, message: String) -> Self {
        Self::Other {
            severity,
            message,
            location: None,
            span: None,
        }
    }
}

impl From<Localized<std::io::Error>> for CommandError {
    fn from(err: Localized<std::io::Error>) -> Self {
        CommandError::Io(err)
    }
}

impl From<Vec<Localized<Spanned<ParseError>>>> for CommandError {
    fn from(err: Vec<Localized<Spanned<ParseError>>>) -> Self {
        CommandError::Parse(err)
    }
}

impl From<Vec<Localized<Spanned<VerifyError>>>> for CommandError {
    fn from(err: Vec<Localized<Spanned<VerifyError>>>) -> Self {
        CommandError::Verify(err)
    }
}

impl From<Localized<PatternError>> for CommandError {
    fn from(value: Localized<PatternError>) -> Self {
        CommandError::Other {
            severity: IssueSeverity::Error,
            message: value.msg.to_string(),
            location: Some(value.location.clone()),
            span: None,
        }
    }
}

impl From<IoError> for CommandError {
    fn from(value: IoError) -> Self {
        CommandError::Other {
            severity: IssueSeverity::Error,
            message: value.to_string(),
            location: None,
            span: None,
        }
    }
}

impl From<Localized<serde_json::Error>> for CommandError {
    fn from(e: Localized<serde_json::Error>) -> Self {
        let cursor = Cursor::new(e.line(), e.column());
        CommandError::Other {
            severity: IssueSeverity::Error,
            message: e.to_string(),
            location: Some(e.location.clone()),
            span: Some(Span::new(cursor, cursor)),
        }
    }
}

impl From<serde_json::Error> for CommandError {
    fn from(e: serde_json::Error) -> Self {
        CommandError::Other {
            severity: IssueSeverity::Error,
            message: e.to_string(),
            location: None,
            span: None,
        }
    }
}

impl From<CommandError> for Issues {
    fn from(val: CommandError) -> Self {
        match val {
            CommandError::Parse(err) => err
                .into_iter()
                .map(|e| Issue {
                    severity: IssueSeverity::Error,
                    location: Some(IssueLocation {
                        location: e.location.clone(),
                        span: Some(e.span),
                    }),
                    message: e.to_string(),
                })
                .collect::<Vec<_>>(),
            CommandError::Io(err) => vec![Issue {
                severity: IssueSeverity::Error,
                location: Some(IssueLocation {
                    location: err.location.clone(),
                    span: None,
                }),
                message: err.to_string(),
            }],
            CommandError::Verify(err) => err
                .into_iter()
                .map(|e| Issue {
                    severity: IssueSeverity::Error,
                    location: Some(IssueLocation {
                        location: e.location.clone(),
                        span: Some(e.span),
                    }),
                    message: e.to_string(),
                })
                .collect::<Vec<_>>(),
            CommandError::Other {
                severity,
                message,
                location,
                span,
            } => {
                vec![Issue {
                    severity,
                    location: location.map(|l| IssueLocation { location: l, span }),
                    message,
                }]
            }
        }
    }
}
