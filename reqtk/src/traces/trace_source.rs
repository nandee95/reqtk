use crate::*;
use req_file::prelude::*;
use serde::Serialize;
use std::io::BufReader;

#[derive(Debug, Eq, PartialEq, Serialize, Default)]
pub struct TraceSource {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub defines: Vec<Localized<Spanned<String>>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub consumes: Vec<Localized<Spanned<String>>>,
}

impl TraceSource {
    pub fn find_defines_in_tokens(
        &mut self,
        location: &Location,
        tokens: &Vec<Token>,
        ids: &mut Vec<String>,
    ) -> Result<(), CommandError> {
        let mut prev = None;
        for token in tokens {
            if prev == Some(&TokenKind::RequirementStart)
                && let TokenKind::Identifier(id) = &token.token
            {
                if ids.contains(id) {
                    return Err(CommandError::other(format!(
                        "Duplicate identifier found: {}",
                        id
                    )));
                } else {
                    ids.push(id.clone());
                    self.defines.push(Localized::new(
                        Spanned::new(id.clone(), token.source.span),
                        Some(location.clone()),
                    ));
                }
            }
            prev = Some(&token.token);
        }
        Ok(())
    }

    pub fn find_consumes_in_tokens(
        &mut self,
        location: &Location,
        tokens: &Vec<Token>,
        ids: &mut [String],
    ) -> Result<(), CommandError> {
        for token in tokens {
            if let TokenKind::AttributeValue(_) = &token.token {
                self.find_consumes_in_block(location, ids, &token.source)?;
            }
        }
        Ok(())
    }

    pub fn find_consumes_in_source(
        &mut self,
        location: &Location,
        ids: &[String],
    ) -> Result<(), CommandError> {
        let mut bufreader = BufReader::new(location.reader()?);
        let iter = match location
            .extension()
            .and_then(|ext| CommentIterator::from_extension(&mut bufreader, &ext))
        {
            Some(iter) => iter,
            None => {
                return Err(CommandError::other(format!(
                    "Source is not supported {:?}",
                    location
                )));
            }
        };
        for comment in iter {
            let comment = comment.err_localized(location)?;
            self.find_consumes_in_block(location, ids, &comment)?;
        }
        Ok(())
    }

    pub fn find_consumes_in_block(
        &mut self,
        location: &Location,
        ids: &[String],
        block: &Spanned<String>,
    ) -> Result<(), CommandError> {
        for reference in ReferenceIterator::new(&block.value, block.span) {
            if ids.iter().any(|v| v.as_str() == reference.value) {
                self.consumes.push(Localized::new(
                    Spanned::new(reference.value, reference.span),
                    Some(location.clone()),
                ));
            } else {
                return Err(CommandError::other(format!(
                    "Reference to undefined identifier found: {}",
                    reference.value
                )));
            }
        }
        Ok(())
    }
}
