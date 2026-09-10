use std::cell::LazyCell;

use crate::*;
use adar::prelude::*;
use regex::Regex;
use serde::Serialize;

pub struct TokenCh {
    pub ch: char,
    pub str: &'static str,
}

macro_rules! token_ch {
    ($ch:literal, $str:literal) => {
        TokenCh { ch: $ch, str: $str }
    };
}

pub const TOKEN_REQUIREMENT_START: TokenCh = token_ch!('@', "@");
pub const TOKEN_REQUIREMENT_TYPE_START: TokenCh = token_ch!('$', "$");
pub const TOKEN_REQUIREMENT_TITLE_START: TokenCh = token_ch!('(', "(");
pub const TOKEN_REQUIREMENT_TITLE_END: TokenCh = token_ch!(')', ")");
pub const TOKEN_REQUIREMENT_BODY_START: TokenCh = token_ch!('{', "{");
pub const TOKEN_REQUIREMENT_BODY_END: TokenCh = token_ch!('}', "}");
pub const TOKEN_ATTRIBUTE_START: TokenCh = token_ch!('[', "[");
pub const TOKEN_ATTRIBUTE_SEPARATOR: TokenCh = token_ch!('=', "=");
pub const TOKEN_ATTRIBUTE_END: TokenCh = token_ch!(']', "]");
pub const TOKEN_ATTRIBUTE_CLOSE: TokenCh = token_ch!('/', "/");
pub const ESCAPE_CHARACTER: TokenCh = token_ch!('\\', "\\");

#[FlagEnum]
pub enum TokenFormatFlags {
    NewLineAfter,
    SpaceAfter,
    StartInline,
    EndInline,
    IncrementIdentation,
    DecrementIdentation,
    IdentBefore,
}

#[derive(Debug)]
pub struct TokenFollower {
    pub kind: TokenValue,
    pub terminator: Vec<TokenValue>,
}

impl Default for TokenFollower {
    fn default() -> Self {
        Self {
            kind: TokenValue::RawString(String::new()),
            terminator: Vec::new(),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
#[ReflectEnum]
pub enum TokenValue {
    RequirementStart,
    RequirementTypeStart,
    TitleStart,
    TitleEnd,
    BodyStart,
    BodyEnd,
    AttributeStart,
    AttributeClose,
    AttributeSeparator,
    AttributeEnd,
    RawString(String),
    Identifier(String),
    Title(String),
    AttributeKey(String),
    AttributeValue(String),
}

impl Serialize for TokenValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.name())
    }
}

impl TokenValue {
    pub fn is_same_kind(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::RawString(_), Self::RawString(_)) => true,
            (Self::Identifier(_), Self::Identifier(_)) => true,
            (Self::Title(_), Self::Title(_)) => true,
            (Self::AttributeKey(_), Self::AttributeKey(_)) => true,
            (Self::AttributeValue(_), Self::AttributeValue(_)) => true,
            (a, b) => a == b,
        }
    }

    pub fn flags(&self) -> Flags<TokenFormatFlags> {
        use TokenFormatFlags::*;
        match self {
            TokenValue::RequirementStart => StartInline | IdentBefore,
            TokenValue::RequirementTypeStart => StartInline | IdentBefore,
            TokenValue::TitleStart => Flags::empty(),
            TokenValue::TitleEnd => EndInline | SpaceAfter,
            TokenValue::BodyStart => IncrementIdentation | NewLineAfter,
            TokenValue::BodyEnd => DecrementIdentation | IdentBefore | NewLineAfter,
            TokenValue::AttributeStart => StartInline | IdentBefore,
            TokenValue::AttributeClose => Flags::empty(),
            TokenValue::AttributeSeparator => Flags::empty(),
            TokenValue::AttributeEnd => EndInline | NewLineAfter,
            _ => Flags::empty(),
        }
    }

    pub fn follower<'a, T>(&self, tokens: &'a [T]) -> Option<TokenFollower>
    where
        &'a T: Into<&'a TokenValue>,
    {
        match self {
            Self::RequirementStart => Some(TokenFollower {
                kind: Self::Identifier(String::new()),
                terminator: vec![TokenValue::TitleStart],
            }),
            Self::TitleStart => Some(TokenFollower {
                kind: Self::Title(String::new()),
                terminator: vec![Self::TitleEnd],
            }),
            Self::RequirementTypeStart => Some(TokenFollower {
                kind: Self::Identifier(String::new()),
                terminator: vec![Self::TitleStart],
            }),
            Self::AttributeStart => Some(TokenFollower {
                kind: Self::AttributeKey(String::new()),
                terminator: vec![
                    Self::AttributeSeparator,
                    Self::AttributeEnd,
                    Self::AttributeClose,
                ],
            }),
            Self::AttributeClose => Some(TokenFollower {
                kind: Self::AttributeKey(String::new()),
                terminator: vec![Self::AttributeEnd],
            }),
            Self::AttributeSeparator => Some(TokenFollower {
                kind: Self::AttributeValue(String::new()),
                terminator: vec![Self::AttributeEnd],
            }),
            Self::AttributeEnd => match tokens {
                [.., a, b, _]
                    if a.into() == &Self::AttributeStart
                        && matches!(b.into(), Self::AttributeKey(_)) =>
                {
                    Some(TokenFollower {
                        kind: Self::AttributeValue(String::new()),
                        terminator: vec![Self::AttributeStart],
                    })
                }
                _ => None,
            },
            _ => None,
        }
    }

    pub fn set_contents(&mut self, contents: String) {
        match self {
            Self::RawString(content) => *content = contents,
            Self::Identifier(content) => *content = contents,
            Self::Title(content) => *content = contents,
            Self::AttributeKey(content) => *content = contents,
            Self::AttributeValue(content) => *content = contents,
            _ => {}
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::RequirementStart => TOKEN_REQUIREMENT_START.str,
            Self::RequirementTypeStart => TOKEN_REQUIREMENT_TYPE_START.str,
            Self::TitleStart => TOKEN_REQUIREMENT_TITLE_START.str,
            Self::TitleEnd => TOKEN_REQUIREMENT_TITLE_END.str,
            Self::BodyStart => TOKEN_REQUIREMENT_BODY_START.str,
            Self::BodyEnd => TOKEN_REQUIREMENT_BODY_END.str,
            Self::AttributeStart => TOKEN_ATTRIBUTE_START.str,
            Self::AttributeClose => TOKEN_ATTRIBUTE_CLOSE.str,
            Self::AttributeSeparator => TOKEN_ATTRIBUTE_SEPARATOR.str,
            Self::AttributeEnd => TOKEN_ATTRIBUTE_END.str,
            Self::RawString(str) => str,
            Self::Identifier(str) => str,
            Self::Title(str) => str,
            Self::AttributeKey(str) => str,
            Self::AttributeValue(str) => str,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::RequirementStart => TOKEN_REQUIREMENT_START.ch,
            Self::RequirementTypeStart => TOKEN_REQUIREMENT_TYPE_START.ch,
            Self::TitleStart => TOKEN_REQUIREMENT_TITLE_START.ch,
            Self::TitleEnd => TOKEN_REQUIREMENT_TITLE_END.ch,
            Self::BodyStart => TOKEN_REQUIREMENT_BODY_START.ch,
            Self::BodyEnd => TOKEN_REQUIREMENT_BODY_END.ch,
            Self::AttributeStart => TOKEN_ATTRIBUTE_START.ch,
            Self::AttributeClose => TOKEN_ATTRIBUTE_CLOSE.ch,
            Self::AttributeSeparator => TOKEN_ATTRIBUTE_SEPARATOR.ch,
            Self::AttributeEnd => TOKEN_ATTRIBUTE_END.ch,
            _ => panic!("TokenKind {:?} does not have a corresponding char", self),
        }
    }

    pub fn display_issue(&self) -> String {
        match self {
            Self::RequirementStart => TOKEN_REQUIREMENT_START.str,
            Self::RequirementTypeStart => TOKEN_REQUIREMENT_TYPE_START.str,
            Self::TitleStart => TOKEN_REQUIREMENT_TITLE_START.str,
            Self::TitleEnd => TOKEN_REQUIREMENT_TITLE_END.str,
            Self::BodyStart => TOKEN_REQUIREMENT_BODY_START.str,
            Self::BodyEnd => TOKEN_REQUIREMENT_BODY_END.str,
            Self::AttributeStart => TOKEN_ATTRIBUTE_START.str,
            Self::AttributeClose => TOKEN_ATTRIBUTE_CLOSE.str,
            Self::AttributeSeparator => TOKEN_ATTRIBUTE_SEPARATOR.str,
            Self::AttributeEnd => TOKEN_ATTRIBUTE_END.str,
            Self::RawString(_) => "string",
            Self::Identifier(_) => "identifier",
            Self::Title(_) => "title",
            Self::AttributeKey(_) => "key",
            Self::AttributeValue(_) => "value",
        }
        .to_string()
    }

    pub fn has_contents(&self) -> bool {
        matches!(
            self,
            Self::RawString(_)
                | Self::Identifier(_)
                | Self::Title(_)
                | Self::AttributeKey(_)
                | Self::AttributeValue(_)
        )
    }

    pub fn value_as_source<'a, T>(&self, indent: &str, tokens: &'a [T]) -> String
    where
        &'a T: Into<&'a TokenValue>,
    {
        let mut result = self.as_str().to_string();

        if self.has_contents() {
            result = result.replace(
                ESCAPE_CHARACTER.ch,
                &format!("{}{}", ESCAPE_CHARACTER.ch, ESCAPE_CHARACTER.ch),
            );

            if let Some(follower) = tokens
                .get(tokens.len().saturating_sub(1))
                .and_then(|v| v.into().follower(tokens))
            {
                if self.is_same_kind(&follower.kind) {
                    for terminator in &follower.terminator {
                        let ch = terminator.to_char();
                        result = result.replace(ch, &format!("{}{}", ESCAPE_CHARACTER.ch, ch));
                    }
                }
            }
            if !indent.is_empty() {
                result = result.replace("\n", &format!("\n{}", indent));
            }
        }

        result
    }
}

impl TryFrom<char> for TokenValue {
    type Error = ();

    fn try_from(value: char) -> Result<Self, ()> {
        if value == TOKEN_REQUIREMENT_START.ch {
            Ok(Self::RequirementStart)
        } else if value == TOKEN_REQUIREMENT_TYPE_START.ch {
            Ok(Self::RequirementTypeStart)
        } else if value == TOKEN_REQUIREMENT_TITLE_START.ch {
            Ok(Self::TitleStart)
        } else if value == TOKEN_REQUIREMENT_TITLE_END.ch {
            Ok(Self::TitleEnd)
        } else if value == TOKEN_REQUIREMENT_BODY_START.ch {
            Ok(Self::BodyStart)
        } else if value == TOKEN_REQUIREMENT_BODY_END.ch {
            Ok(Self::BodyEnd)
        } else if value == TOKEN_ATTRIBUTE_START.ch {
            Ok(Self::AttributeStart)
        } else if value == TOKEN_ATTRIBUTE_CLOSE.ch {
            Ok(Self::AttributeClose)
        } else if value == TOKEN_ATTRIBUTE_SEPARATOR.ch {
            Ok(Self::AttributeSeparator)
        } else if value == TOKEN_ATTRIBUTE_END.ch {
            Ok(Self::AttributeEnd)
        } else {
            Err(())
        }
    }
}

impl std::fmt::Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.has_contents() {
            write!(f, "{}", self.as_str())
        } else {
            write!(f, "{}", self.name())
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize)]
pub struct Token {
    pub value: TokenValue,
    #[serde(flatten)]
    pub source: Spanned<String>,
}

impl Token {
    pub fn from_source(mut kind: TokenValue, source: Spanned<String>) -> Self {
        kind.set_contents(Self::source_to_value(source.as_str()));
        Token {
            value: kind,
            source,
        }
    }

    pub fn from_value(value: TokenValue, span: Span) -> Self {
        let source = value.as_str().to_string();
        Token {
            value,
            source: source.into_spanned(span),
        }
    }

    pub fn source_to_value(input: &str) -> String {
        Self::trim_whitespaces(Self::unescape(input))
    }

    fn unescape(input: &str) -> String {
        thread_local! {
            static UNESCAPE_REGEX: LazyCell<Regex> =
                LazyCell::<Regex>::new(|| Regex::new(r"\\(.)").expect(PANIC_INVALID_REGEX));
        }
        UNESCAPE_REGEX.with(|regex| regex.replace_all(input, r"$1").to_string())
    }

    fn trim_whitespaces(input: String) -> String {
        let trim_start = input.trim_start();
        let leading_ws = &input[..input.len() - trim_start.len()];
        let trim = input.trim();
        if let Some((_, indent)) = leading_ws.rsplit_once('\n') {
            trim.replace(&format!("\n{}", indent), "\n")
        } else {
            trim.to_string()
        }
        .replace("\r\n", "\n")
    }
}

impl<'a> From<&'a Token> for &'a TokenValue {
    fn from(val: &'a Token) -> Self {
        &val.value
    }
}
