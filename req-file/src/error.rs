use crate::prelude::*;

pub(crate) const PANIC_INVALID_REGEX: &str = "Invalid regex";

#[derive(Debug)]
pub enum ParseError {
    UnexpectedSequence {
        expected: Vec<TokenKind>,
        found: String,
    },
    UnexpectedEof {
        expected: Vec<TokenKind>,
    },
    UnclosedRequirementBody,
    UnclosedMultiLineAttribute {
        id: String,
    },
    MismatchedAttributeKey {
        expected: String,
        found: String,
    },
}

#[derive(Debug)]
pub enum VerifyError {
    InvalidIdentifier {
        id: String,
        kind: IdentifierKind,
        message: String,
    },
    DuplicateIndentifier {
        id: String,
        kind: IdentifierKind,
    },
    RequirementIdMissingPrefix {
        id: String,
        prefix: String,
    },
    InvalidRequirementType {
        id: String,
        possible_types: Vec<String>,
    },
    InvalidAttributeValue {
        key: String,
        value: String,
        message: String,
    },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const ONE_OF: fn(&Vec<TokenKind>) -> &'static str = |expected: &Vec<TokenKind>| {
            if expected.len() > 1 { " one of" } else { "" }
        };
        const FMT_EXPECTED: fn(&Vec<TokenKind>) -> String = |expected: &Vec<TokenKind>| {
            expected
                .iter()
                .map(|v| v.display_issue())
                .collect::<Vec<_>>()
                .join(", ")
        };
        match self {
            ParseError::UnexpectedSequence { expected, found } => {
                write!(
                    f,
                    "unexpected sequence {:?}, expected{} {}",
                    shorten(found),
                    ONE_OF(expected),
                    FMT_EXPECTED(expected),
                )
            }
            ParseError::UnexpectedEof { expected } => {
                write!(
                    f,
                    "unexpected EOF, expected{} {}",
                    ONE_OF(expected),
                    FMT_EXPECTED(expected)
                )
            }
            ParseError::UnclosedRequirementBody => {
                write!(f, "unclosed requirement body")
            }
            ParseError::UnclosedMultiLineAttribute { id } => {
                write!(f, "unclosed multi-line attribute {:?}", id)
            }
            ParseError::MismatchedAttributeKey { expected, found } => {
                write!(
                    f,
                    "mismatched attribute key, expected: {:?}, found {:?}",
                    expected, found
                )
            }
        }
    }
}

impl std::fmt::Display for VerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyError::InvalidIdentifier { id, kind, message } => {
                write!(f, "invalid {:?} identifier of {:?}: {}", id, kind, message)
            }
            VerifyError::DuplicateIndentifier { id, kind } => {
                write!(f, "duplicate {:?} identifier: {:?}", kind, id)
            }
            VerifyError::RequirementIdMissingPrefix { id, prefix } => {
                write!(
                    f,
                    "requirement identifier {:?} does not start with prefix {:?}",
                    id, prefix
                )
            }
            VerifyError::InvalidRequirementType { id, possible_types } => {
                write!(
                    f,
                    "invalid requirement type: {:?}, possible: \"{}\"",
                    id,
                    possible_types.join("\", \"")
                )
            }
            VerifyError::InvalidAttributeValue {
                key,
                value,
                message,
            } => {
                write!(
                    f,
                    "invalid value for attribute {:?}={:?}: {}",
                    key, value, message
                )
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum IdentifierKind {
    Requirement,
    RequirementType,
    Attribute,
}

impl std::fmt::Debug for IdentifierKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentifierKind::Requirement => write!(f, "requirement"),
            IdentifierKind::RequirementType => write!(f, "requirement type"),
            IdentifierKind::Attribute => write!(f, "attribute"),
        }
    }
}

fn shorten(input: &str) -> String {
    const MAX_STRING_DISPLAY: usize = 32;
    if input.len() > MAX_STRING_DISPLAY {
        format!("{}…", &input[..MAX_STRING_DISPLAY - 1])
    } else {
        input.to_string()
    }
}
