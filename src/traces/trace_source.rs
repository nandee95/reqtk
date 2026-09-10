use crate::*;
use serde::Serialize;
use std::io::BufReader;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Trace {
    pub id: String,
    pub span: Span,
    pub file: String,
}

#[derive(Debug, Eq, PartialEq, Serialize, Default)]
pub struct TraceSource {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub defines: Vec<Trace>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub consumes: Vec<Trace>,
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
            if prev == Some(&TokenValue::RequirementStart)
                && let TokenValue::Identifier(id) = &token.value
            {
                if ids.contains(id) {
                    return Err(CommandError::other(
                        IssueSeverity::Error,
                        format!("Duplicate identifier found: {}", id),
                    ));
                } else {
                    ids.push(id.clone());
                    self.defines.push(Trace {
                        id: id.clone(),
                        span: token.source.span.clone(),
                        file: location.to_string(),
                    });
                }
            }
            prev = Some(&token.value);
        }
        Ok(())
    }

    pub fn find_consumes_in_tokens(
        &mut self,
        location: &Location,
        tokens: &Vec<Token>,
        ids: &mut Vec<String>,
    ) -> Result<(), CommandError> {
        for token in tokens {
            if let TokenValue::AttributeValue(_) = &token.value {
                self.find_consumes_in_block(location, ids, &*token.source, token.source.span);
            }
        }
        Ok(())
    }

    pub fn find_consumes_in_source(
        &mut self,
        location: &Location,
        ids: &Vec<String>,
    ) -> Result<(), CommandError> {
        let mut bufreader = BufReader::new(location.reader()?);
        let iter = match location
            .extension()
            .and_then(|ext| CommentIterator::from_extension(&mut bufreader, &ext))
        {
            Some(iter) => iter,
            None => {
                return Err(CommandError::Other {
                    severity: IssueSeverity::Warning,
                    message: format!("Source is not supported {:?}", location),
                    location: Some(location.clone()),
                    span: None,
                });
            }
        };
        for comment in iter {
            let comment = comment.err_localized(location)?;
            self.find_consumes_in_block(location, ids, &comment, comment.span);
        }
        Ok(())
    }

    pub fn find_consumes_in_block(
        &mut self,
        location: &Location,
        ids: &Vec<String>,
        block: &str,
        span: Span,
    ) {
        let mut cursor = span.start;
        for (index, substring) in block.split('@').enumerate() {
            if index != 0
                && let Some(key) = ids
                    .iter()
                    .filter(|a| substring.starts_with(*a))
                    .max_by_key(|v| v.len())
            {
                self.consumes.push(Trace {
                    id: key.clone(),
                    span: Span::new(cursor, cursor.advance(key)),
                    file: location.to_string(),
                });
            }

            cursor = cursor.advance(substring);
            cursor = cursor.advance_ch('@');
        }
    }
}
