use std::str::FromStr;

use regex::Regex;

use crate::*;

pub const PANIC_MISSING_SPAN: &str = "Missing span";
pub const PANIC_INVALID_REGEX: &str = "Invalid regex";
pub const PANIC_INVALID_GLOB: &str = "Invalid glob";

#[derive(Debug)]
pub enum VerifyError {
    InvalidIdentifier {
        id: String,
        kind: IdentifierKind,
        regex: String,
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
            VerifyError::InvalidIdentifier { id, kind, regex } => {
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
    pub fn verify_location(
        location: &Location,
        tree: &RequirementTree,
    ) -> Result<(), CommandError> {
        Ok(Self::verify(tree).map_err(|e| {
            e.into_iter()
                .map(|err| err.into_localized(location))
                .collect::<Vec<_>>()
        })?)
    }

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
            if attribute.key.as_str() == "id-prefix" {
                id_prefix = Some(attribute.value.as_str().to_string());
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

        const BUILIN_REQUIREMENT_TYPES: [&str; 5] = [
            "folder",
            "functional",
            "informative",
            "parameter",
            "limitation",
        ];
        if let Some(requirement_type) = requirement
            .attributes
            .iter()
            .find(|a| a.key.as_str() == "type")
            && !self
                .requirement_types
                .iter()
                .any(|v| v == requirement_type.value.as_str())
            && !BUILIN_REQUIREMENT_TYPES.contains(&requirement_type.value.as_str())
        {
            self.errors.push(
                VerifyError::InvalidRequirementType {
                    id: requirement_type.value.to_string(),
                    possible_types: BUILIN_REQUIREMENT_TYPES
                        .iter()
                        .map(ToString::to_string)
                        .chain(self.requirement_types.clone())
                        .collect(),
                }
                .into_spanned(*requirement_type.value.span().expect(PANIC_MISSING_SPAN)),
            );
        }

        self.verify_duplicate_identifier(&requirement.id, IdentifierKind::Requirement);

        if let Some(prefix) = &self.id_prefix
            && !requirement.id.as_str().starts_with(prefix)
        {
            self.errors.push(
                VerifyError::RequirementIdMissingPrefix {
                    id: requirement.id.to_string(),
                    prefix: prefix.clone(),
                }
                .into_spanned(*requirement.id.span().expect(PANIC_MISSING_SPAN)),
            );
        }

        for child in &requirement.children {
            self.verify_requirement(child);
        }
    }

    fn verify_attribute(&mut self, attribute: &Attribute, attributes: &mut Vec<String>) {
        self.verify_identifier(&attribute.key, IdentifierKind::Attribute);
        if attributes.contains(&attribute.key.to_string()) {
            self.errors.push(
                VerifyError::DuplicateIndentifier {
                    id: attribute.key.to_string(),
                    kind: IdentifierKind::Attribute,
                }
                .into_spanned(*attribute.key.span().expect(PANIC_MISSING_SPAN)),
            );
        } else {
            attributes.push(attribute.key.to_string());
        }
    }

    fn verify_identifier<const I: u32>(&mut self, id: &MaybeSpanned<I>, kind: IdentifierKind) {
        const MATCH_REGEX: [&str; 3] = [
            "",
            "^[a-z0-9-]+$", // RequirementType
            "^[a-z0-9-]+$", // Attribute
        ];

        let match_regex = MATCH_REGEX[kind as usize];
        let re = Regex::new(match_regex).expect(PANIC_INVALID_REGEX);

        if !re.is_match(id.as_str()) {
            self.errors.push(
                VerifyError::InvalidIdentifier {
                    kind,
                    id: id.to_string(),
                    regex: match_regex.to_string(),
                }
                .into_spanned(
                    *id.span()
                        .unwrap_or_else(|| panic!("{}({:?})", PANIC_MISSING_SPAN, kind)),
                ),
            );
        }
    }

    fn verify_duplicate_identifier<const I: u32>(
        &mut self,
        id: &MaybeSpanned<I>,
        kind: IdentifierKind,
    ) {
        let ids = match kind {
            IdentifierKind::Requirement => &mut self.requirement_ids,
            IdentifierKind::RequirementType => &mut self.requirement_types,
            _ => return,
        };

        if ids.iter().any(|v| v == id.as_str()) {
            self.errors.push(
                VerifyError::DuplicateIndentifier {
                    id: id.to_string(),
                    kind,
                }
                .into_spanned(*id.span().expect(PANIC_MISSING_SPAN)),
            );
        } else {
            ids.push(id.to_string());
        }
    }

    fn verify_file_level_attribute_value<const I: u32, const J: u32>(
        &mut self,
        key: &MaybeSpanned<I>,
        value: &MaybeSpanned<J>,
    ) {
        match key.as_str() {
            "id-type" => {
                if let Err(err) = IdType::from_str(value.as_str()) {
                    self.errors.push(
                        VerifyError::InvalidAttributeValue {
                            key: key.to_string(),
                            value: value.to_string(),
                            error: err,
                        }
                        .into_spanned(*value.span().expect(PANIC_MISSING_SPAN)),
                    );
                }
            }
            "id-count" => {
                if let Err(err) = u32::from_str(value.as_str()) {
                    self.errors.push(
                        VerifyError::InvalidAttributeValue {
                            key: key.to_string(),
                            value: value.to_string(),
                            error: err.to_string(),
                        }
                        .into_spanned(*value.span().expect(PANIC_MISSING_SPAN)),
                    );
                }
            }
            _ => {}
        }
    }
}
