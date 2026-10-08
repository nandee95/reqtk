use crate::prelude::*;
use regex::Regex;
use std::{str::FromStr, sync::OnceLock};

#[derive(Debug)]
pub enum VerifyError {
    InvalidIdentifier {
        id: String,
        kind: IdentifierKind,
        message: String,
    },
    DuplicateIndentifier {
        id: String,
        kind: IdentifierKind,
    },
    RequirementIdMissingPrefix {
        id: String,
        prefix: String,
    },
    InvalidRequirementType {
        id: String,
        possible_types: Vec<String>,
    },
    InvalidAttributeValue {
        key: String,
        value: String,
        error: String,
    },
}

impl std::fmt::Display for VerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyError::InvalidIdentifier {
                id,
                kind,
                message: regex,
            } => {
                write!(
                    f,
                    "{:?} identifier of {:?} does not match regex {:?}",
                    id, kind, regex
                )
            }
            VerifyError::DuplicateIndentifier { id, kind } => {
                write!(f, "Duplicate {:?} identifier: {:?}", kind, id)
            }
            VerifyError::RequirementIdMissingPrefix { id, prefix } => {
                write!(
                    f,
                    "Requirement ID {:?} does not start with prefix {:?}",
                    id, prefix
                )
            }
            VerifyError::InvalidRequirementType { id, possible_types } => {
                write!(
                    f,
                    "Invalid requirement type: {:?}. Possible types: \"{}\"",
                    id,
                    possible_types.join("\", \"")
                )
            }
            VerifyError::InvalidAttributeValue {
                key,
                value,
                error: expected,
            } => {
                write!(
                    f,
                    "Invalid value for attribute {:?}={:?}. Error: {}",
                    key, value, expected
                )
            }
        }
    }
}

pub struct Verifier {
    errors: Vec<Spanned<VerifyError>>,
    id_prefix: Option<String>,
    requirement_ids: Vec<String>,
    requirement_types: Vec<String>,
}

impl Verifier {
    pub fn verify(tree: &RequirementTree) -> Result<(), Vec<Spanned<VerifyError>>> {
        let mut verifier = Self::new(tree);
        verifier.verify_requirement_tree(tree);

        verifier.finish()
    }

    fn finish(self) -> Result<(), Vec<Spanned<VerifyError>>> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors)
        }
    }

    fn new(tree: &RequirementTree) -> Self {
        let mut id_prefix = None;
        for attribute in &tree.attributes {
            if attribute.key.value == Attribute::FILE_ID_PREFIX {
                id_prefix = Some(attribute.value.value.to_string());
                break;
            }
        }

        Self {
            errors: Vec::new(),
            id_prefix,
            requirement_ids: Vec::new(),
            requirement_types: Vec::new(),
        }
    }

    fn verify_requirement_tree(&mut self, tree: &RequirementTree) {
        {
            let mut attribute_ids = Vec::<String>::new();
            for attribute in &tree.attributes {
                self.verify_attribute(attribute, &mut attribute_ids);
                self.verify_file_level_attribute_value(&attribute.key, &attribute.value);
            }
        }
        for requirement_type in &tree.types {
            self.verify_requirement_type(requirement_type);
        }
        for requirement in &tree.requirements {
            self.verify_requirement(requirement);
        }
    }

    fn verify_requirement_type(&mut self, req_type: &RequirementType) {
        self.verify_identifier(&req_type.id, IdentifierKind::RequirementType);
        self.verify_duplicate_identifier(&req_type.id, IdentifierKind::RequirementType);

        let mut attribute_ids = Vec::<String>::new();
        for attribute in &req_type.attributes {
            self.verify_attribute(attribute, &mut attribute_ids);
        }
    }

    fn verify_requirement(&mut self, requirement: &Requirement) {
        {
            let mut attribute_ids = Vec::<String>::new();
            for attribute in &requirement.attributes {
                self.verify_attribute(attribute, &mut attribute_ids);
            }
        }
        if let Some(requirement_type) = requirement
            .attributes
            .iter()
            .find(|a| a.key.value == "type")
            && !self
                .requirement_types
                .iter()
                .any(|v| v == &requirement_type.value.value)
            && !RequirementType::builtin_types()
                .iter()
                .any(|v| v.id.value == requirement_type.value.value)
        {
            self.errors.push(Spanned::new(
                VerifyError::InvalidRequirementType {
                    id: requirement_type.value.value.clone(),
                    possible_types: RequirementType::builtin_types()
                        .iter()
                        .map(|v| v.id.value.clone())
                        .chain(self.requirement_types.clone())
                        .collect(),
                },
                requirement_type.value.span,
            ));
        }

        self.verify_duplicate_identifier(&requirement.id, IdentifierKind::Requirement);

        if let Some(prefix) = &self.id_prefix
            && !requirement.id.value.starts_with(prefix)
        {
            self.errors.push(Spanned::new(
                VerifyError::RequirementIdMissingPrefix {
                    id: requirement.id.value.clone(),
                    prefix: prefix.clone(),
                },
                requirement.id.span,
            ));
        }

        for child in &requirement.children {
            self.verify_requirement(child);
        }
    }

    fn verify_attribute(&mut self, attribute: &Attribute, attributes: &mut Vec<String>) {
        self.verify_identifier(&attribute.key, IdentifierKind::Attribute);
        if attributes.contains(&attribute.key.value) {
            self.errors.push(Spanned::new(
                VerifyError::DuplicateIndentifier {
                    id: attribute.key.value.clone(),
                    kind: IdentifierKind::Attribute,
                },
                attribute.key.span,
            ));
        } else {
            attributes.push(attribute.key.value.clone());
        }
    }

    fn verify_identifier(&mut self, id: &Spanned<String>, kind: IdentifierKind) {
        static REGEX_LOCK: OnceLock<Regex> = OnceLock::new();
        let regex =
            REGEX_LOCK.get_or_init(|| Regex::new("^[a-z0-9-]+$").expect(PANIC_INVALID_REGEX));

        if !regex.is_match(&id.value) {
            self.errors.push(Spanned::new(
                VerifyError::InvalidIdentifier {
                    kind,
                    id: id.value.clone(),
                    message:
                        "Identifier must consist of lowercase letters, numbers, and hyphens only"
                            .to_string(),
                },
                id.span,
            ));
        }
    }

    fn verify_duplicate_identifier(&mut self, id: &Spanned<String>, kind: IdentifierKind) {
        let ids = match kind {
            IdentifierKind::Requirement => &mut self.requirement_ids,
            IdentifierKind::RequirementType => &mut self.requirement_types,
            _ => return,
        };

        if ids.iter().any(|v| v == &id.value) {
            self.errors.push(Spanned::new(
                VerifyError::DuplicateIndentifier {
                    id: id.value.clone(),
                    kind,
                },
                id.span,
            ));
        } else {
            ids.push(id.value.clone());
        }
    }

    fn verify_file_level_attribute_value(
        &mut self,
        key: &Spanned<String>,
        value: &Spanned<String>,
    ) {
        match key.value.as_str() {
            Attribute::FILE_ID_TYPE => {
                if let Err(err) = IdType::from_str(&value.value) {
                    self.errors.push(Spanned::new(
                        VerifyError::InvalidAttributeValue {
                            key: key.value.clone(),
                            value: value.value.clone(),
                            error: err,
                        },
                        value.span,
                    ));
                }
            }
            Attribute::FILE_ID_COUNT => {
                if let Err(err) = u32::from_str(&value.value) {
                    self.errors.push(Spanned::new(
                        VerifyError::InvalidAttributeValue {
                            key: key.value.clone(),
                            value: value.value.clone(),
                            error: err.to_string(),
                        },
                        value.span,
                    ));
                }
            }
            _ => {}
        }
    }
}
