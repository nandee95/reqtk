use crate::prelude::*;

#[derive(Debug)]
pub struct TreeTokenizer;

impl TreeTokenizer {
    pub fn tokenize(tree: &RequirementTree) -> Vec<TokenKind> {
        let mut tokens = vec![];

        Self::tokenize_attributes(&mut tokens, &tree.attributes);
        Self::tokenize_requirement_types(&mut tokens, &tree.types);
        Self::tokenize_requirements(&mut tokens, &tree.requirements);

        tokens
    }

    fn tokenize_requirements(tokens: &mut Vec<TokenKind>, requirements: &Requirements) {
        for requirement in requirements {
            tokens.extend_from_slice(&[
                TokenKind::RequirementStart,
                TokenKind::Identifier(requirement.id.value.to_string()),
                TokenKind::TitleStart,
                TokenKind::Title(requirement.title.value.to_string()),
                TokenKind::TitleEnd,
                TokenKind::BodyStart,
            ]);

            Self::tokenize_attributes(tokens, &requirement.attributes);
            Self::tokenize_requirements(tokens, &requirement.children);
            tokens.push(TokenKind::BodyEnd);
        }
    }
    fn tokenize_attributes(tokens: &mut Vec<TokenKind>, attributes: &Attributes) {
        const EXPAND_MULTILINE: fn(&mut Vec<TokenKind>, &str, &str) =
            |tokens: &mut Vec<TokenKind>, key: &str, value: &str| {
                tokens.extend_from_slice(&[
                    TokenKind::AttributeStart,
                    TokenKind::AttributeKey(key.into()),
                    TokenKind::AttributeEnd,
                    TokenKind::AttributeValue(value.into()),
                    TokenKind::AttributeStart,
                    TokenKind::AttributeClose,
                    TokenKind::AttributeKey(key.into()),
                    TokenKind::AttributeEnd,
                ]);
            };

        for attribute in attributes {
            if attribute.key.value.as_str() == "description" {
                continue;
            }
            if attribute.value.value.as_str().contains("\n") {
                EXPAND_MULTILINE(
                    tokens,
                    attribute.key.value.as_str(),
                    attribute.value.value.as_str(),
                );
            } else {
                tokens.extend_from_slice(&[
                    TokenKind::AttributeStart,
                    TokenKind::AttributeKey(attribute.key.value.to_string()),
                    TokenKind::AttributeSeparator,
                    TokenKind::AttributeValue(attribute.value.value.to_string()),
                    TokenKind::AttributeEnd,
                ]);
            }
        }

        if let Some(attribute) = attributes
            .iter()
            .find(|v| v.key.value.as_str() == "description")
        {
            EXPAND_MULTILINE(tokens, "description", attribute.value.value.as_str());
        }
    }

    fn tokenize_requirement_types(
        tokens: &mut Vec<TokenKind>,
        requirement_types: &RequirementTypes,
    ) {
        for value in requirement_types {
            tokens.extend_from_slice(&[
                TokenKind::RequirementTypeStart,
                TokenKind::Identifier(value.id.value.to_string()),
                TokenKind::TitleStart,
                TokenKind::Title(value.title.value.to_string()),
                TokenKind::TitleEnd,
                TokenKind::BodyStart,
            ]);
            Self::tokenize_attributes(tokens, &value.attributes);
            tokens.push(TokenKind::BodyEnd);
        }
    }
}
