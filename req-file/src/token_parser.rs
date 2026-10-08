use crate::prelude::*;
use peekmore::{PeekMore, PeekMoreIterator};
use std::vec::IntoIter;

pub struct TokenParser {
    iter: PeekMoreIterator<IntoIter<Token>>,
    eof: Span,
}

impl TokenParser {
    pub fn parse(tokens: Vec<Token>) -> Result<RequirementTree, Vec<Spanned<ParseError>>> {
        let mut result = RequirementTree::default();
        let mut errors = Vec::new();

        let eof = tokens
            .last()
            .and_then(|v| v.source.span)
            .map(|v| v.end)
            .unwrap_or_else(Cursor::start);
        let eof = Span::new(eof, eof);
        let iter = tokens.into_iter().peekmore();

        let mut parser = Self { iter, eof };

        let mut error_seq: Option<Spanned<String>> = None;
        let expected = vec![
            TokenKind::RequirementStart,
            TokenKind::RequirementTypeStart,
            TokenKind::AttributeStart,
        ];
        while let Some(token) = parser.iter.peek().cloned() {
            match token.token {
                TokenKind::RequirementStart
                | TokenKind::RequirementTypeStart
                | TokenKind::AttributeStart => {
                    if let Some(error_seq_val) = error_seq {
                        errors.push(Spanned::new(
                            ParseError::UnexpectedSequence {
                                expected: expected.clone(),
                                found: error_seq_val.value.clone(),
                            },
                            error_seq_val.span,
                        ));
                        error_seq = None;
                    }
                    match token.token {
                        TokenKind::RequirementStart => {
                            if parser
                                .parse_requirement(&mut result.requirements, &mut errors)
                                .is_err()
                            {
                                break;
                            }
                        }
                        TokenKind::RequirementTypeStart => {
                            if parser
                                .parse_requirement_type(&mut result.types, &mut errors)
                                .is_err()
                            {
                                break;
                            }
                        }
                        TokenKind::AttributeStart => {
                            if parser
                                .parse_attribute(&mut result.attributes, &mut errors)
                                .is_err()
                            {
                                break;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                _ => {
                    if let Some(error_seq) = &mut error_seq {
                        error_seq.value.push_str(token.token.as_str());
                        error_seq.span = error_seq
                            .span
                            .zip(token.source.span)
                            .map(|(a, b)| a.union(b));
                    } else {
                        error_seq = Some(Spanned::new(token.token.as_str(), token.source.span));
                    }
                    parser.iter.next();
                }
            }
        }
        if let Some(error_seq) = error_seq {
            errors.push(Spanned::new(
                ParseError::UnexpectedSequence {
                    expected,
                    found: error_seq.value.clone(),
                },
                error_seq.span,
            ));
        }

        if errors.is_empty() {
            Ok(result)
        } else {
            Err(errors)
        }
    }

    fn parse_attribute(
        &mut self,
        attributes: &mut Attributes,
        errors: &mut Vec<Spanned<ParseError>>,
    ) -> Result<(), ()> {
        let mut attribute = Attribute::default();

        let attribute_start = self
            .expect(&[TokenKind::AttributeStart], errors)?
            .source
            .span;

        if matches!(self.peek_next_n(), Some([TokenKind::AttributeKey(_)])) {
            attribute.key = self.expect_contents(TokenKind::AttributeKey(String::new()), errors)?;
        } else {
            attribute.key.span = attribute_start.map(|v| v.end.zero_span());
        }

        let attribute_end;
        (attribute.value, attribute_end) = if self
            .iter
            .peek()
            .map(|t| t.token == TokenKind::AttributeSeparator)
            .unwrap_or(false)
        {
            let separator = self
                .expect(&[TokenKind::AttributeSeparator], errors)?
                .source
                .span;

            let value = if matches!(self.peek_next_n(), Some([TokenKind::AttributeValue(_)])) {
                self.expect_contents(TokenKind::AttributeValue(String::new()), errors)?
            } else {
                Spanned::new("", separator.map(|v| v.start.zero_span()))
            };
            let attribute_end = self.expect(&[TokenKind::AttributeEnd], errors)?.source.span;
            (value.trim(), attribute_end)
        } else {
            let start_token = self.expect(&[TokenKind::AttributeEnd], errors)?;

            let value = if matches!(self.peek_next_n(), Some([TokenKind::AttributeValue(_)])) {
                self.expect_contents(TokenKind::AttributeValue(String::new()), errors)?
            } else {
                Spanned::new("", start_token.source.span.map(|v| v.start.zero_span()))
            };

            self.expect(&[TokenKind::AttributeStart], errors)?;
            self.expect(&[TokenKind::AttributeClose], errors)?;
            let close_key = self.expect_contents(TokenKind::AttributeKey(String::new()), errors)?;

            if close_key.value.as_str() != attribute.key.value.as_str() {
                errors.push(Spanned::new(
                    ParseError::MismatchedAttributeKey {
                        expected: attribute.key.value.clone(),
                        found: close_key.value.clone(),
                    },
                    close_key.span,
                ));
            }
            let attribute_end = self.expect(&[TokenKind::AttributeEnd], errors)?.source.span;

            (value, attribute_end)
        };

        attribute.span = attribute_start
            .zip(attribute_end)
            .map(|(a, b)| Span::new(a.start, b.end));
        attributes.push(attribute);

        Ok(())
    }

    fn parse_requirement(
        &mut self,
        requirements: &mut Requirements,
        errors: &mut Vec<Spanned<ParseError>>,
    ) -> Result<(), ()> {
        let mut requirement = Requirement::default();

        let requirement_start = self
            .expect(&[TokenKind::RequirementStart], errors)?
            .source
            .span;

        if matches!(self.peek_next_n(), Some([TokenKind::Identifier(_)])) {
            requirement.id = self.expect_contents(TokenKind::Identifier(String::new()), errors)?;
        } else {
            requirement.id.span = requirement_start.map(|v| v.end.zero_span());
        }

        let title_start = self.expect(&[TokenKind::TitleStart], errors)?.source.span;
        if matches!(self.peek_next_n(), Some([TokenKind::Title(_)])) {
            requirement.title = self.expect_contents(TokenKind::Title(String::new()), errors)?;
        } else {
            requirement.title.span = title_start.map(|v| v.end.zero_span());
        }

        self.expect(&[TokenKind::TitleEnd], errors)?;
        let body_start = self.expect(&[TokenKind::BodyStart], errors)?;

        let mut error_seq = String::new();
        let expected = vec![
            TokenKind::AttributeStart,
            TokenKind::RequirementStart,
            TokenKind::BodyEnd,
        ];
        let requirement_end;
        loop {
            if let Some(token) = self.iter.peek().cloned() {
                match &token.token {
                    TokenKind::AttributeStart | TokenKind::RequirementStart => {
                        if !error_seq.is_empty() {
                            errors.push(Spanned::new(
                                ParseError::UnexpectedSequence {
                                    expected: expected.clone(),
                                    found: error_seq,
                                },
                                token.source.span,
                            ));
                            error_seq = String::new();
                        }
                        match &token.token {
                            TokenKind::AttributeStart => {
                                self.parse_attribute(&mut requirement.attributes, errors)?;
                            }
                            TokenKind::RequirementStart => {
                                self.parse_requirement(&mut requirement.children, errors)?;
                            }
                            _ => unreachable!(),
                        }
                    }
                    TokenKind::BodyEnd => {
                        requirement_end = token.source.span;
                        break;
                    }
                    _ => {
                        error_seq.push_str(token.token.as_str());
                        self.iter.next();
                    }
                }
            } else {
                errors.push(Spanned::new(
                    ParseError::UnclosedRequirementBody,
                    body_start.source.span,
                ));
                return Err(());
            }
        }

        self.iter.next();
        requirement.span = requirement_start
            .zip(requirement_end)
            .map(|(a, b)| Span::new(a.start, b.end));
        requirements.push(requirement);

        Ok(())
    }

    fn parse_requirement_type(
        &mut self,
        requirement_types: &mut RequirementTypes,
        errors: &mut Vec<Spanned<ParseError>>,
    ) -> Result<(), ()> {
        let mut requirement_type = RequirementType::default();

        let requirement_type_start = self
            .expect(&[TokenKind::RequirementTypeStart], errors)?
            .source
            .span;
        if matches!(self.peek_next_n(), Some([TokenKind::Identifier(_)])) {
            requirement_type.id =
                self.expect_contents(TokenKind::Identifier(String::new()), errors)?;
        } else {
            requirement_type.id.span = requirement_type_start.map(|v| v.end.zero_span());
        }
        let title_start = self.expect(&[TokenKind::TitleStart], errors)?.source.span;
        if matches!(self.peek_next_n(), Some([TokenKind::Title(_)])) {
            requirement_type.title =
                self.expect_contents(TokenKind::Title(String::new()), errors)?;
        } else {
            requirement_type.title.span = title_start.map(|v| v.end.zero_span());
        }
        self.expect(&[TokenKind::TitleEnd], errors)?;
        let body_start = self.expect(&[TokenKind::BodyStart], errors)?;

        let mut error_seq = String::new();
        let expected = vec![TokenKind::AttributeStart, TokenKind::BodyEnd];
        let requirement_type_end;
        loop {
            if let Some(token) = self.iter.peek().cloned() {
                match &token.token {
                    TokenKind::AttributeStart => {
                        if !error_seq.is_empty() {
                            errors.push(Spanned::new(
                                ParseError::UnexpectedSequence {
                                    expected: expected.clone(),
                                    found: error_seq,
                                },
                                token.source.span,
                            ));
                            error_seq = String::new();
                        }
                        self.parse_attribute(&mut requirement_type.attributes, errors)?;
                    }
                    TokenKind::BodyEnd => {
                        requirement_type_end = token.source.span;
                        break;
                    }
                    _ => {
                        error_seq.push_str(token.token.as_str());
                        self.iter.next();
                    }
                }
            } else {
                errors.push(Spanned::new(
                    ParseError::UnclosedRequirementBody,
                    body_start.source.span,
                ));
                return Err(());
            }
        }
        self.iter.next();
        requirement_type.span = requirement_type_start
            .zip(requirement_type_end)
            .map(|(a, b)| Span::new(a.start, b.end));
        requirement_types.push(requirement_type);

        Ok(())
    }

    fn expect(
        &mut self,
        expected: &[TokenKind],
        errors: &mut Vec<Spanned<ParseError>>,
    ) -> Result<Token, ()> {
        let mut error_seq: Option<Spanned<String>> = None;
        for token in self.iter.by_ref() {
            if expected.iter().any(|e| e.is_same_kind(&token.token)) {
                if let Some(error_seq) = error_seq {
                    errors.push(Spanned::new(
                        ParseError::UnexpectedSequence {
                            expected: expected.to_vec(),
                            found: error_seq.value.clone(),
                        },
                        error_seq.span,
                    ));
                }
                return Ok(token);
            } else {
                if let Some(error_seq) = &mut error_seq {
                    error_seq.value.push_str(token.token.as_str());
                    error_seq.span = error_seq
                        .span
                        .zip(token.source.span)
                        .map(|(a, b)| a.union(b));
                } else {
                    error_seq = Some(Spanned::new(token.token.as_str(), token.source.span));
                }
            }
        }

        if let Some(error_seq) = error_seq {
            errors.push(Spanned::new(
                ParseError::UnexpectedSequence {
                    expected: expected.to_vec(),
                    found: error_seq.value.clone(),
                },
                error_seq.span,
            ));
        }

        errors.push(Spanned::new(
            ParseError::UnexpectedEof {
                expected: expected.to_vec(),
            },
            Some(self.eof),
        ));
        Err(())
    }

    fn expect_contents(
        &mut self,
        expected: TokenKind,
        errors: &mut Vec<Spanned<ParseError>>,
    ) -> Result<Spanned<String>, ()> {
        let mut error_seq: Option<Spanned<String>> = None;
        for token in self.iter.by_ref() {
            if !expected.is_same_kind(&token.token) {
                if let Some(error_seq) = &mut error_seq {
                    error_seq.value.push_str(token.token.as_str());
                    error_seq.span = error_seq
                        .span
                        .zip(token.source.span)
                        .map(|(a, b)| a.union(b));
                } else {
                    error_seq = Some(Spanned::new(token.token.as_str(), token.source.span));
                }
            } else {
                match &token.token {
                    TokenKind::RawString(s)
                    | TokenKind::Identifier(s)
                    | TokenKind::Title(s)
                    | TokenKind::AttributeKey(s)
                    | TokenKind::AttributeValue(s) => {
                        if let Some(error_seq) = error_seq {
                            errors.push(Spanned::new(
                                ParseError::UnexpectedSequence {
                                    expected: vec![expected],
                                    found: error_seq.value.clone(),
                                },
                                error_seq.span,
                            ));
                        }

                        return Ok(Spanned::new(s.clone(), token.source.span));
                    }
                    _ => {}
                }
            }
        }

        if let Some(error_seq) = error_seq {
            errors.push(Spanned::new(
                ParseError::UnexpectedSequence {
                    expected: vec![expected.clone()],
                    found: error_seq.value.clone(),
                },
                error_seq.span,
            ));
        }

        errors.push(Spanned::new(
            ParseError::UnexpectedEof {
                expected: vec![expected],
            },
            Some(self.eof),
        ));
        Err(())
    }

    fn peek_next_n<const N: usize>(&mut self) -> Option<[TokenKind; N]> {
        self.iter
            .peek_amount(N)
            .iter()
            .map(|token| token.as_ref().map(|t| t.token.clone()))
            .collect::<Option<Vec<_>>>()?
            .try_into()
            .ok()
    }
}
