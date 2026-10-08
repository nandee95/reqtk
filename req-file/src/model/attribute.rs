use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub type Attributes = Vec<Attribute>;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Attribute {
    pub key: Spanned<String>,
    pub value: Spanned<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

impl Attribute {
    pub const FILE_ID_PREFIX: &str = "id-prefix";
    pub const FILE_ID_TYPE: &str = "id-type";
    pub const FILE_ID_COUNT: &str = "id-count";

    pub const TYPE_ICON: &str = "icon";
    pub const TYPE_COLOR: &str = "color";

    pub const REQ_TYPE: &str = "type";
    pub const REQ_DESCRIPTION: &str = "description";

    pub const ID_TYPE_INCREMENTAL_DEFAULT: u8 = 0;

    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: Spanned::new(key.to_string(), None),
            value: Spanned::new(value.to_string(), None),
            span: None,
        }
    }
}

pub trait AttributesExt {
    fn get_mut(&mut self, key: &str) -> Option<&mut String>;
    fn get(&self, key: &str) -> Option<&String>;
    fn set(&mut self, key: &str, value: &str);
    fn remove_attr(&mut self, key: &str);
}

impl AttributesExt for Attributes {
    fn get_mut(&mut self, key: &str) -> Option<&mut String> {
        self.iter_mut()
            .find(|v| v.key.value == key)
            .map(|v| &mut v.value.value)
    }
    fn get(&self, key: &str) -> Option<&String> {
        self.iter()
            .find(|v| v.key.value == key)
            .map(|v| &v.value.value)
    }
    fn set(&mut self, key: &str, value: &str) {
        if let Some(attr) = self.get_mut(key) {
            *attr = value.to_string();
        } else {
            self.push(Attribute {
                key: Spanned::new(key.to_string(), None),
                value: Spanned::new(value.to_string(), None),
                span: None,
            });
        }
    }
    fn remove_attr(&mut self, key: &str) {
        if let Some(pos) = self.iter().position(|v| v.key.value == key) {
            self.remove(pos);
        }
    }
}
