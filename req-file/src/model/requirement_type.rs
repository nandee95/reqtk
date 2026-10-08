use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub const TYPE_FOLDER: &str = "folder";
pub const TYPE_FUNCTIONAL: &str = "functional";
pub const TYPE_LIMITATION: &str = "limitation";
pub const TYPE_PARAMETER: &str = "parameter";
pub const TYPE_INFORMATIVE: &str = "informative";

pub type RequirementTypes = Vec<RequirementType>;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RequirementType {
    pub id: Spanned<String>,
    pub title: Spanned<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attributes: Attributes,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

impl RequirementType {
    pub fn builtin_types() -> &'static [RequirementType; 5] {
        static REQUIREMENT_TYPES: OnceLock<[RequirementType; 5]> = OnceLock::new();
        REQUIREMENT_TYPES.get_or_init(|| {
            [
                RequirementType {
                    id: Spanned::new(TYPE_FOLDER, None),
                    title: Spanned::new("Folder", None),
                    attributes: vec![
                        Attribute::new(Attribute::TYPE_ICON, "folder"),
                        Attribute::new(Attribute::TYPE_COLOR, "#facc15"),
                    ],
                    span: None,
                },
                RequirementType {
                    id: Spanned::new(TYPE_FUNCTIONAL, None),
                    title: Spanned::new("Functional", None),
                    attributes: vec![
                        Attribute::new(Attribute::TYPE_ICON, "bolt"),
                        Attribute::new(Attribute::TYPE_COLOR, "#3b82f6"),
                    ],
                    span: None,
                },
                RequirementType {
                    id: Spanned::new(TYPE_LIMITATION, None),
                    title: Spanned::new("Limitation", None),
                    attributes: vec![
                        Attribute::new(Attribute::TYPE_ICON, "warning"),
                        Attribute::new(Attribute::TYPE_COLOR, "#ef4444"),
                    ],
                    span: None,
                },
                RequirementType {
                    id: Spanned::new(TYPE_PARAMETER, None),
                    title: Spanned::new("Parameter", None),
                    attributes: vec![
                        Attribute::new(Attribute::TYPE_ICON, "tag"),
                        Attribute::new(Attribute::TYPE_COLOR, "#22c55e"),
                    ],
                    span: None,
                },
                RequirementType {
                    id: Spanned::new(TYPE_INFORMATIVE, None),
                    title: Spanned::new("Informative", None),
                    attributes: vec![
                        Attribute::new(Attribute::TYPE_ICON, "article"),
                        Attribute::new(Attribute::TYPE_COLOR, "#a855f7"),
                    ],
                    span: None,
                },
            ]
        })
    }
}
