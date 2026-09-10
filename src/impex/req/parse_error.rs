use crate::*;

#[derive(Debug)]
pub enum ParseError {
    InvalidIdentifier {
        kind: IdentifierKind,
        id: String,
        regex: String,
    },
    UnexpectedSequence {
        expected: Vec<TokenValue>,
        found: String,
    },
    UnexpectedEof {
        expected: Vec<TokenValue>,
    },
    UnclosedRequirementBody,
    UnclosedMultiLineAttribute {
        id: String,
    },
    MismatchedAttributeKey {
        expected: String,
        found: String,
    },
    Custom(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const ONE_OF: fn(&Vec<TokenValue>) -> &'static str = |expected: &Vec<TokenValue>| {
            if expected.len() > 1 { " one of" } else { "" }
        };
        const FMT_EXPECTED: fn(&Vec<TokenValue>) -> String = |expected: &Vec<TokenValue>| {
            expected
                .iter()
                .map(|v| v.display_issue())
                .collect::<Vec<_>>()
                .join(", ")
        };
        match self {
            ParseError::InvalidIdentifier { kind, id, regex } => {
                write!(
                    f,
                    "invalid {:?} identifier: {:?} (regex: {})",
                    kind,
                    Self::shorten(id),
                    regex
                )
            }
            ParseError::UnexpectedSequence { expected, found } => {
                write!(
                    f,
                    "unexpected sequence {:?}, expected{} {}",
                    Self::shorten(found),
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
            ParseError::Custom(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl ParseError {
    fn shorten(input: &str) -> String {
        const MAX_STRING_DISPLAY: usize = 32;
        if input.len() > MAX_STRING_DISPLAY {
            format!("{}…", &input[..MAX_STRING_DISPLAY - 1])
        } else {
            input.to_string()
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

impl From<Localized<Spanned<ParseError>>> for Issue {
    fn from(val: Localized<Spanned<ParseError>>) -> Self {
        Issue {
            severity: IssueSeverity::Error,
            location: Some(IssueLocation {
                location: val.location.clone(),
                span: Some(val.span),
            }),
            message: val.to_string(),
        }
    }
}
