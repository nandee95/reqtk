use std::fmt::Display;

use crate::{IntoSpanned, Span, Spanned};
use serde::{Deserialize, Serialize, ser::SerializeStruct};

pub type Attributes = Vec<Attribute>;
pub type Requirements = Vec<Requirement>;
pub type RequirementTypes = Vec<RequirementType>;

#[derive(Deserialize, Debug, Clone)]
pub struct MaybeSpanned<const I: u32> {
    pub value: String,
    pub span: Option<Span>,
}

impl<const I: u32> Display for MaybeSpanned<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub const MS_ID: u32 = 0;
pub const MS_KEY: u32 = 1;
pub const MS_VALUE: u32 = 2;
pub const MS_TITLE: u32 = 3;

impl<const I: u32> Default for MaybeSpanned<I> {
    fn default() -> Self {
        MaybeSpanned {
            value: String::new(),
            span: None,
        }
    }
}

impl<const I: u32> MaybeSpanned<I> {
    pub fn unspanned(value: &str) -> Self {
        Self {
            value: value.to_string(),
            span: None,
        }
    }
    pub fn spanned(value: &str, span: Span) -> Self {
        Self {
            value: value.to_string(),
            span: Some(span),
        }
    }

    pub fn unspan(&mut self) {
        self.span = None;
    }

    pub fn key() -> &'static str {
        match I {
            MS_ID => "id",
            MS_KEY => "key",
            MS_VALUE => "value",
            MS_TITLE => "title",
            _ => panic!("Invalid MaybeSpanned index"),
        }
    }

    pub fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    pub fn trim(&self) -> Self {
        if let Some(span) = &self.span {
            self.value.clone().into_spanned(*span).trim().into()
        } else {
            self.clone()
        }
    }
}

impl<const I: u32> From<Spanned<String>> for MaybeSpanned<I> {
    fn from(spanned: Spanned<String>) -> Self {
        MaybeSpanned {
            value: spanned.to_string(),
            span: Some(spanned.span),
        }
    }
}

impl<const I: u32> Serialize for MaybeSpanned<I> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(span) = &self.span {
            let mut s = serializer.serialize_struct(Self::key(), 2)?;
            s.serialize_field("value", &self.value)?;
            s.serialize_field("span", span)?;
            s.end()
        } else {
            serializer.serialize_str(&self.value)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Attribute {
    pub key: MaybeSpanned<MS_KEY>,
    pub value: MaybeSpanned<MS_VALUE>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RequirementType {
    pub id: MaybeSpanned<MS_ID>,
    pub title: MaybeSpanned<MS_TITLE>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attributes: Attributes,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Requirement {
    pub id: MaybeSpanned<MS_ID>,
    pub title: MaybeSpanned<MS_TITLE>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attributes: Attributes,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Requirements,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RequirementTree {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attributes: Attributes,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub types: RequirementTypes,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub requirements: Requirements,
}

impl RequirementTree {
    pub fn clear_spans(&mut self) {
        for attr in &mut self.attributes {
            attr.key.unspan();
            attr.value.unspan();
            attr.span = None;
        }
        for req_type in &mut self.types {
            req_type.id.unspan();
            req_type.title.unspan();
            for attr in &mut req_type.attributes {
                attr.key.unspan();
                attr.value.unspan();
                attr.span = None;
            }
            req_type.span = None;
        }
        for req in &mut self.requirements {
            req.clear_spans();
        }
    }
}

impl Requirement {
    pub fn clear_spans(&mut self) {
        self.id.unspan();
        self.title.unspan();
        self.span = None;
        for attr in &mut self.attributes {
            attr.key.unspan();
            attr.value.unspan();
            attr.span = None;
        }
        for child in &mut self.children {
            child.clear_spans();
        }
    }
}
