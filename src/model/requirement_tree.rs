use crate::*;
use serde::{Deserialize, Serialize, ser::SerializeStruct};
use std::{collections::HashMap, fmt::Display, str::FromStr};

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

#[derive(Debug, Clone)]
pub enum IdType {
    Incremental(u8),
}

impl FromStr for IdType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value == "incremental" {
            return Ok(Self::Incremental(ID_TYPE_INCREMENTAL_DEFAULT));
        } else if let Some(n) = value.strip_prefix("incremental-") {
            return Ok(Self::Incremental(
                n.parse::<u8>()
                    .map_err(|_| "invalid incremental length".to_string())?,
            ));
        }

        Err(format!("unknown id-type: {value:?}"))
    }
}

impl std::fmt::Display for IdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Incremental(n) => {
                if *n == ID_TYPE_INCREMENTAL_DEFAULT {
                    f.write_str("incremental")
                } else {
                    f.write_fmt(format_args!("incremental-{n}"))
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Change<T: Copy> {
    from: T,
    to: T,
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

    pub fn re_id(
        &mut self,
        id_type: Option<IdType>,
        prefix: Option<String>,
    ) -> HashMap<String, String> {
        let mut map = HashMap::new();
        let mut count = 0;

        let prefix_from = self.attributes.get(ATTR_ID_PREFIX).unwrap_or("");

        let prefix = Change {
            from: prefix_from,
            to: prefix.as_deref().unwrap_or(prefix_from),
        };

        for requirement in &mut self.requirements {
            requirement.re_id(id_type.clone(), prefix, &mut count, &mut map);
        }

        let new_prefix = prefix.to.to_string();

        match &id_type {
            Some(IdType::Incremental(_)) => {
                self.attributes.set(ATTR_ID_COUNT, &count.to_string());
            }
            None => {}
        }

        if let Some(id_type) = &id_type {
            self.attributes.set(ATTR_ID_TYPE, &id_type.to_string());
        }

        self.attributes.set(ATTR_ID_PREFIX, &new_prefix);

        map
    }

    pub fn update_refs(&mut self, map: &HashMap<String, String>) {
        for req in &mut self.requirements {
            req.update_refs(map);
        }
    }

    pub fn find(&self, id: &str) -> Option<&Requirement> {
        for req in &self.requirements {
            if let Some(found) = req.find(id) {
                return Some(found);
            }
        }
        None
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

    pub fn re_id(
        &mut self,
        id_type: Option<IdType>,
        prefix: Change<&str>,
        count: &mut usize,
        map: &mut HashMap<String, String>,
    ) {
        let new_id = format!(
            "{}{}",
            prefix.to,
            match &id_type {
                Some(IdType::Incremental(n)) => {
                    *count += 1;

                    format!("{:0width$}", count, width = *n as usize)
                }
                None => {
                    self.id
                        .value
                        .strip_prefix(prefix.from)
                        .unwrap_or(&self.id.value)
                        .to_string()
                }
            }
        );
        map.insert(self.id.value.clone(), new_id.clone());
        self.id.value = new_id;

        for child in &mut self.children {
            child.re_id(id_type.clone(), prefix, count, map);
        }
    }

    fn update_refs(&mut self, map: &HashMap<String, String>) {
        if let Some(description) = self.attributes.get_mut("description") {
            let mut replacements = Vec::new();
            for reference in ReferenceIterator::new(
                description,
                Span::new(Cursor::start(), Cursor::end(description)),
            ) {
                let Some(to) = map.get(*reference) else {
                    continue;
                };

                replacements.push((reference.span.to_range(), to.to_string()));
            }
            for (range, replacement) in replacements.iter().rev() {
                description.replace_range(range.clone(), replacement);
            }
        }

        for child in &mut self.children {
            child.update_refs(map);
        }
    }

    pub fn find(&self, id: &str) -> Option<&Requirement> {
        if self.id.value == id {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find(id) {
                return Some(found);
            }
        }
        None
    }
}
