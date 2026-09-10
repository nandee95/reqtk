use crate::*;

#[derive(Debug)]
pub struct TreeTokenizer;

impl TreeTokenizer {
    pub fn tokenize(tree: &RequirementTree) -> Vec<TokenValue> {
        let mut tokens = vec![];

        Self::tokenize_attributes(&mut tokens, &tree.attributes);
        Self::tokenize_requirement_types(&mut tokens, &tree.types);
        Self::tokenize_requirements(&mut tokens, &tree.requirements);

        tokens
    }

    fn tokenize_requirements(tokens: &mut Vec<TokenValue>, requirements: &Requirements) {
        for requirement in requirements {
            tokens.extend_from_slice(&[
                TokenValue::RequirementStart,
                TokenValue::Identifier(requirement.id.to_string()),
                TokenValue::TitleStart,
                TokenValue::Title(requirement.title.to_string()),
                TokenValue::TitleEnd,
                TokenValue::BodyStart,
            ]);

            Self::tokenize_attributes(tokens, &requirement.attributes);
            Self::tokenize_requirements(tokens, &requirement.children);
            tokens.push(TokenValue::BodyEnd);
        }
    }
    fn tokenize_attributes(tokens: &mut Vec<TokenValue>, attributes: &Attributes) {
        const EXTEND_MULTILINE: fn(&mut Vec<TokenValue>, &str, &str) =
            |tokens: &mut Vec<TokenValue>, key: &str, value: &str| {
                tokens.extend_from_slice(&[
                    TokenValue::AttributeStart,
                    TokenValue::AttributeKey(key.into()),
                    TokenValue::AttributeEnd,
                    TokenValue::AttributeValue(value.into()),
                    TokenValue::AttributeStart,
                    TokenValue::AttributeClose,
                    TokenValue::AttributeKey(key.into()),
                    TokenValue::AttributeEnd,
                ]);
            };

        for attribute in attributes {
            if attribute.key.as_str() == "description" {
                continue;
            }
            if attribute.value.as_str().contains("\n") {
                EXTEND_MULTILINE(tokens, attribute.key.as_str(), attribute.value.as_str());
            } else {
                tokens.extend_from_slice(&[
                    TokenValue::AttributeStart,
                    TokenValue::AttributeKey(attribute.key.to_string()),
                    TokenValue::AttributeSeparator,
                    TokenValue::AttributeValue(attribute.value.to_string()),
                    TokenValue::AttributeEnd,
                ]);
            }
        }

        if let Some(attribute) = attributes.iter().find(|v| v.key.as_str() == "description") {
            EXTEND_MULTILINE(tokens, "description", attribute.value.as_str());
        }
    }

    fn tokenize_requirement_types(
        tokens: &mut Vec<TokenValue>,
        requirement_types: &RequirementTypes,
    ) {
        for value in requirement_types {
            tokens.extend_from_slice(&[
                TokenValue::RequirementTypeStart,
                TokenValue::Identifier(value.id.to_string()),
                TokenValue::TitleStart,
                TokenValue::Title(value.title.to_string()),
                TokenValue::TitleEnd,
                TokenValue::BodyStart,
            ]);
            Self::tokenize_attributes(tokens, &value.attributes);
            tokens.push(TokenValue::BodyEnd);
        }
    }
}
