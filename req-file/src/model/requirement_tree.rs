use crate::prelude::*;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RequirementTree {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub attributes: Attributes,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub types: RequirementTypes,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub requirements: Requirements,
}

#[derive(Copy, Clone)]
pub(crate) struct Change<T: Copy> {
    pub(crate) from: T,
    pub(crate) to: T,
}

#[derive(Debug, Clone)]
pub enum IdType {
    Incremental(u8),
}

impl FromStr for IdType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value == "incremental" {
            return Ok(Self::Incremental(Attribute::ID_TYPE_INCREMENTAL_DEFAULT));
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
                if *n == Attribute::ID_TYPE_INCREMENTAL_DEFAULT {
                    f.write_str("incremental")
                } else {
                    f.write_fmt(format_args!("incremental-{n}"))
                }
            }
        }
    }
}

impl RequirementTree {
    pub fn resolve_requirement_type(&self, id: Option<&str>) -> &RequirementType {
        let id = id.unwrap_or(TYPE_FOLDER);
        if let Some(builtin) = RequirementType::builtin_types()
            .iter()
            .find(|v| v.id.value == id)
        {
            builtin
        } else if let Some(custom) = self.types.iter().find(|t| t.id.value == id) {
            custom
        } else if let Some(folder) = RequirementType::builtin_types()
            .iter()
            .find(|v| v.id.value == TYPE_LIMITATION)
        {
            folder
        } else {
            panic!("Requirement type not found!")
        }
    }

    pub fn clear_spans(&mut self) {
        for attr in &mut self.attributes {
            attr.key.clear_span();
            attr.value.clear_span();
            attr.span = None;
        }
        for req_type in &mut self.types {
            req_type.id.clear_span();
            req_type.title.clear_span();
            for attr in &mut req_type.attributes {
                attr.key.clear_span();
                attr.value.clear_span();
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

        let prefix_from = self
            .attributes
            .get(Attribute::FILE_ID_PREFIX)
            .map(|v| v.as_str())
            .unwrap_or("");

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
                self.attributes
                    .set(Attribute::FILE_ID_COUNT, &count.to_string());
            }
            None => {}
        }

        if let Some(id_type) = &id_type {
            self.attributes
                .set(Attribute::FILE_ID_TYPE, &id_type.to_string());
        }

        self.attributes.set(Attribute::FILE_ID_PREFIX, &new_prefix);

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
