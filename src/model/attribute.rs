use crate::*;

pub type Attributes = Vec<Attribute>;
pub type Requirements = Vec<Requirement>;

pub const ATTR_ID_PREFIX: &str = "id-prefix";
pub const ATTR_ID_TYPE: &str = "id-type";
pub const ATTR_ID_COUNT: &str = "id-count";

pub const ID_TYPE_INCREMENTAL_DEFAULT: u8 = 0;

pub trait AttributesExt {
    fn get_mut(&mut self, key: &str) -> Option<&mut String>;
    fn get(&self, key: &str) -> Option<&str>;
    fn set(&mut self, key: &str, value: &str);
    fn remove_attr(&mut self, key: &str);
}

impl AttributesExt for Attributes {
    fn get_mut(&mut self, key: &str) -> Option<&mut String> {
        self.iter_mut()
            .find(|v| v.key.as_str() == key)
            .map(|v| &mut v.value.value)
    }
    fn get(&self, key: &str) -> Option<&str> {
        self.iter()
            .find(|v| v.key.as_str() == key)
            .map(|v| v.value.value.as_str())
    }
    fn set(&mut self, key: &str, value: &str) {
        if let Some(attr) = self.get_mut(key) {
            *attr = value.to_string();
        } else {
            self.push(Attribute {
                key: MaybeSpanned::unspanned(key),
                value: MaybeSpanned::unspanned(value),
                span: None,
            });
        }
    }
    fn remove_attr(&mut self, key: &str) {
        if let Some(pos) = self.iter().position(|v| v.key.as_str() == key) {
            self.remove(pos);
        }
    }
}
