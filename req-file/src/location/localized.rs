use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct Localized<T> {
    #[serde(flatten)]
    pub value: T,
    pub location: Option<Location>,
}

impl<T> Localized<T> {
    pub fn new(value: T, location: Option<Location>) -> Self {
        Self { value, location }
    }
}

impl<T> From<T> for Localized<Spanned<T>> {
    fn from(value: T) -> Self {
        Self {
            value: Spanned::new(value, None),
            location: None,
        }
    }
}

impl<T> From<Spanned<T>> for Localized<Spanned<T>> {
    fn from(value: Spanned<T>) -> Self {
        Self {
            value,
            location: None,
        }
    }
}
