use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Requirements = Vec<Requirement>;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Requirement {
    pub id: Spanned<String>,
    pub title: Spanned<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attributes: Attributes,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Requirements,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

impl Requirement {
    pub fn clear_spans(&mut self) {
        self.id.clear_span();
        self.title.clear_span();
        self.span = None;
        for attr in &mut self.attributes {
            attr.key.clear_span();
            attr.value.clear_span();
            attr.span = None;
        }
        for child in &mut self.children {
            child.clear_spans();
        }
    }

    pub(crate) fn re_id(
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

    pub(crate) fn update_refs(&mut self, map: &HashMap<String, String>) {
        if let Some(description) = self.attributes.get_mut("description") {
            let mut replacements = Vec::new();

            for reference in ReferenceIterator::new(description, Some(Span::of(description))) {
                let Some(to) = map.get(reference.value) else {
                    continue;
                };

                if let Some(span) = reference.span {
                    replacements.push((span.to_range(), to.to_string()));
                }
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
