use glob::PatternError;
use req_file::prelude::*;
use std::io::Error as IoError;

pub struct CommandError {
    pub errors: Vec<Localized<Spanned<CommandErrorKind>>>,
}

#[derive(Debug)]
pub enum CommandErrorKind {
    Io(IoError),
    Parse(ParseError),
    Verify(VerifyError),
    Glob(PatternError),
    Json(serde_json::Error),
    Other(String),
}

impl CommandError {
    pub fn other<T: Into<String>>(message: T) -> Self {
        Self {
            errors: vec![Localized {
                location: None,
                value: Spanned::new(CommandErrorKind::Other(message.into()), None),
            }],
        }
    }
}

impl From<Localized<std::io::Error>> for CommandError {
    fn from(err: Localized<std::io::Error>) -> Self {
        Self {
            errors: vec![Localized::new(
                Spanned::new(CommandErrorKind::Io(err.value), None),
                err.location.clone(),
            )],
        }
    }
}

impl From<Localized<Vec<Spanned<ParseError>>>> for CommandError {
    fn from(err: Localized<Vec<Spanned<ParseError>>>) -> Self {
        Self {
            errors: err
                .value
                .into_iter()
                .map(|e| {
                    Localized::new(
                        Spanned::new(CommandErrorKind::Parse(e.value), e.span),
                        err.location.clone(),
                    )
                })
                .collect(),
        }
    }
}

impl From<Localized<Vec<Spanned<VerifyError>>>> for CommandError {
    fn from(err: Localized<Vec<Spanned<VerifyError>>>) -> Self {
        Self {
            errors: err
                .value
                .into_iter()
                .map(|e| {
                    Localized::new(
                        Spanned::new(CommandErrorKind::Verify(e.value), e.span),
                        err.location.clone(),
                    )
                })
                .collect(),
        }
    }
}

impl From<Localized<PatternError>> for CommandError {
    fn from(value: Localized<PatternError>) -> Self {
        Self {
            errors: vec![Localized::new(
                Spanned::new(CommandErrorKind::Other(value.value.to_string()), None),
                value.location.clone(),
            )],
        }
    }
}

impl From<IoError> for CommandError {
    fn from(value: IoError) -> Self {
        Self {
            errors: vec![Localized::new(
                Spanned::new(CommandErrorKind::Io(value), None),
                None,
            )],
        }
    }
}

impl From<Localized<serde_json::Error>> for CommandError {
    fn from(e: Localized<serde_json::Error>) -> Self {
        let cursor = Cursor::new(e.value.line(), e.value.column(), 0); // Note: error printer is not using offset
        Self {
            errors: vec![Localized::new(
                Spanned::new(CommandErrorKind::Json(e.value), Some(cursor.zero_span())),
                e.location.clone(),
            )],
        }
    }
}

impl From<serde_json::Error> for CommandError {
    fn from(e: serde_json::Error) -> Self {
        Self {
            errors: vec![Localized::new(
                Spanned::new(CommandErrorKind::Json(e), None),
                None,
            )],
        }
    }
}
