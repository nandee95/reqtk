use crate::*;
use peekmore::{PeekMore, PeekMoreIterator};
use std::vec::IntoIter;

pub struct Parser {
    iter: PeekMoreIterator<IntoIter<Token>>,
    eof: Span,
}

impl Parser {
    pub fn parse_location(
        location: &Location,
        tokens: Vec<Token>,
    ) -> Result<RequirementTree, CommandError> {
        Ok(Self::parse(tokens).map_err(|e| {
            e.into_iter()
                .map(|err| err.into_localized(location))
                .collect::<Vec<_>>()
        })?)
    }

    pub fn parse(tokens: Vec<Token>) -> Result<RequirementTree, Vec<Spanned<ParseError>>> {
        let mut result = RequirementTree::default();
        let mut errors = Vec::new();

        let eof = tokens
            .last()
            .map(|v| v.source.span.end)
            .unwrap_or_else(Cursor::start);
        let eof = Span::new(eof, eof);
        let iter = tokens.into_iter().peekmore();

        let mut parser = Self { iter, eof };

        let mut error_seq: Option<Spanned<String>> = None;
        let expected = vec![
            TokenValue::RequirementStart,
            TokenValue::RequirementTypeStart,
            TokenValue::AttributeStart,
        ];
        while let Some(token) = parser.iter.peek().cloned() {
            match token.token {
                TokenValue::RequirementStart
                | TokenValue::RequirementTypeStart
                | TokenValue::AttributeStart => {
                    if let Some(error_seq_val) = error_seq {
                        errors.push(
                            ParseError::UnexpectedSequence {
                                expected: expected.clone(),
                                found: (*error_seq_val).clone(),
                            }
                            .into_spanned(error_seq_val.span),
                        );
                        error_seq = None;
                    }
                    match token.token {
                        TokenValue::RequirementStart => {
                            if parser
                                .parse_requirement(&mut result.requirements, &mut errors)
                                .is_err()
                            {
                                break;
                            }
                        }
                        TokenValue::RequirementTypeStart => {
                            if parser
                                .parse_requirement_type(&mut result.types, &mut errors)
                                .is_err()
                            {
                                break;
                            }
                        }
                        TokenValue::AttributeStart => {
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
                        error_seq.push_str(token.token.as_str());
                        error_seq.span = error_seq.span.union(token.source.span);
                    } else {
                        error_seq = Some(
                            token
                                .token
                                .as_str()
                                .to_string()
                                .into_spanned(token.source.span),
                        );
                    }
                    parser.iter.next();
                }
            }
        }
        if let Some(error_seq) = error_seq {
            errors.push(
                ParseError::UnexpectedSequence {
                    expected,
                    found: (*error_seq).clone(),
                }
                .into_spanned(error_seq.span),
            );
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
            .expect(&[TokenValue::AttributeStart], errors)?
            .source
            .span;

        if matches!(self.peek_next_n(), Some([TokenValue::AttributeKey(_)])) {
            attribute.key =
                self.expect_contents(TokenValue::AttributeKey(String::new()), errors)?;
        } else {
            attribute.key.span = Some(attribute_start.end.zero_span());
        }

        let attribute_end;
        (attribute.value, attribute_end) = if self
            .iter
            .peek()
            .map(|t| t.token == TokenValue::AttributeSeparator)
            .unwrap_or(false)
        {
            let separator = self
                .expect(&[TokenValue::AttributeSeparator], errors)?
                .source
                .span;

            let value = if matches!(self.peek_next_n(), Some([TokenValue::AttributeValue(_)])) {
                self.expect_contents(TokenValue::AttributeValue(String::new()), errors)?
            } else {
                MaybeSpanned::spanned("", separator.start.zero_span())
            };
            let attribute_end = self
                .expect(&[TokenValue::AttributeEnd], errors)?
                .source
                .span;
            (value.trim(), attribute_end)
        } else {
            let start_token = self.expect(&[TokenValue::AttributeEnd], errors)?;

            let value = if matches!(self.peek_next_n(), Some([TokenValue::AttributeValue(_)])) {
                self.expect_contents(TokenValue::AttributeValue(String::new()), errors)?
            } else {
                MaybeSpanned::spanned("", start_token.source.span.start.zero_span())
            };

            self.expect(&[TokenValue::AttributeStart], errors)?;
            self.expect(&[TokenValue::AttributeClose], errors)?;
            let close_key =
                self.expect_contents::<MS_KEY>(TokenValue::AttributeKey(String::new()), errors)?;

            if close_key.as_str() != attribute.key.as_str() {
                errors.push(
                    ParseError::MismatchedAttributeKey {
                        expected: attribute.key.as_str().to_string(),
                        found: close_key.to_string(),
                    }
                    .into_spanned(close_key.span.expect(PANIC_MISSING_SPAN)),
                );
            }
            let attribute_end = self
                .expect(&[TokenValue::AttributeEnd], errors)?
                .source
                .span;

            (value, attribute_end)
        };

        attribute.span = Some(Span::new(attribute_start.start, attribute_end.end));
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
            .expect(&[TokenValue::RequirementStart], errors)?
            .source
            .span;

        if matches!(self.peek_next_n(), Some([TokenValue::Identifier(_)])) {
            requirement.id = self.expect_contents(TokenValue::Identifier(String::new()), errors)?;
        } else {
            requirement.id.span = Some(requirement_start.end.zero_span());
        }

        let title_start = self.expect(&[TokenValue::TitleStart], errors)?.source.span;
        if matches!(self.peek_next_n(), Some([TokenValue::Title(_)])) {
            requirement.title = self.expect_contents(TokenValue::Title(String::new()), errors)?;
        } else {
            requirement.title.span = Some(title_start.end.zero_span());
        }

        self.expect(&[TokenValue::TitleEnd], errors)?;
        let body_start = self.expect(&[TokenValue::BodyStart], errors)?;

        let mut error_seq = String::new();
        let expected = vec![
            TokenValue::AttributeStart,
            TokenValue::RequirementStart,
            TokenValue::BodyEnd,
        ];
        let requirement_end;
        loop {
            if let Some(token) = self.iter.peek().cloned() {
                match &token.token {
                    TokenValue::AttributeStart | TokenValue::RequirementStart => {
                        if !error_seq.is_empty() {
                            errors.push(
                                ParseError::UnexpectedSequence {
                                    expected: expected.clone(),
                                    found: error_seq,
                                }
                                .into_spanned(token.source.span),
                            );
                            error_seq = String::new();
                        }
                        match &token.token {
                            TokenValue::AttributeStart => {
                                self.parse_attribute(&mut requirement.attributes, errors)?;
                            }
                            TokenValue::RequirementStart => {
                                self.parse_requirement(&mut requirement.children, errors)?;
                            }
                            _ => unreachable!(),
                        }
                    }
                    TokenValue::BodyEnd => {
                        requirement_end = token.source.span;
                        break;
                    }
                    _ => {
                        error_seq.push_str(token.token.as_str());
                        self.iter.next();
                    }
                }
            } else {
                errors
                    .push(ParseError::UnclosedRequirementBody.into_spanned(body_start.source.span));
                return Err(());
            }
        }

        self.iter.next();
        requirement.span = Some(Span::new(requirement_start.start, requirement_end.end));
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
            .expect(&[TokenValue::RequirementTypeStart], errors)?
            .source
            .span;
        if matches!(self.peek_next_n(), Some([TokenValue::Identifier(_)])) {
            requirement_type.id =
                self.expect_contents(TokenValue::Identifier(String::new()), errors)?;
        } else {
            requirement_type.id.span = Some(requirement_type_start.end.zero_span());
        }
        let title_start = self.expect(&[TokenValue::TitleStart], errors)?.source.span;
        if matches!(self.peek_next_n(), Some([TokenValue::Title(_)])) {
            requirement_type.title =
                self.expect_contents(TokenValue::Title(String::new()), errors)?;
        } else {
            requirement_type.title.span = Some(title_start.end.zero_span());
        }
        self.expect(&[TokenValue::TitleEnd], errors)?;
        let body_start = self.expect(&[TokenValue::BodyStart], errors)?;

        let mut error_seq = String::new();
        let expected = vec![TokenValue::AttributeStart, TokenValue::BodyEnd];
        let requirement_type_end;
        loop {
            if let Some(token) = self.iter.peek().cloned() {
                match &token.token {
                    TokenValue::AttributeStart => {
                        if !error_seq.is_empty() {
                            errors.push(
                                ParseError::UnexpectedSequence {
                                    expected: expected.clone(),
                                    found: error_seq,
                                }
                                .into_spanned(token.source.span),
                            );
                            error_seq = String::new();
                        }
                        self.parse_attribute(&mut requirement_type.attributes, errors)?;
                    }
                    TokenValue::BodyEnd => {
                        requirement_type_end = token.source.span;
                        break;
                    }
                    _ => {
                        error_seq.push_str(token.token.as_str());
                        self.iter.next();
                    }
                }
            } else {
                errors
                    .push(ParseError::UnclosedRequirementBody.into_spanned(body_start.source.span));
                return Err(());
            }
        }
        self.iter.next();
        requirement_type.span = Some(Span::new(
            requirement_type_start.start,
            requirement_type_end.end,
        ));
        requirement_types.push(requirement_type);

        Ok(())
    }

    fn expect(
        &mut self,
        expected: &[TokenValue],
        errors: &mut Vec<Spanned<ParseError>>,
    ) -> Result<Token, ()> {
        let mut error_seq: Option<Spanned<String>> = None;
        for token in self.iter.by_ref() {
            if expected.iter().any(|e| e.is_same_kind(&token.token)) {
                if let Some(error_seq) = error_seq {
                    errors.push(
                        ParseError::UnexpectedSequence {
                            expected: expected.to_vec(),
                            found: (*error_seq).clone(),
                        }
                        .into_spanned(error_seq.span),
                    );
                }
                return Ok(token);
            } else {
                if let Some(error_seq) = &mut error_seq {
                    error_seq.push_str(token.token.as_str());
                    error_seq.span = error_seq.span.union(token.source.span);
                } else {
                    error_seq = Some(
                        token
                            .token
                            .as_str()
                            .to_string()
                            .into_spanned(token.source.span),
                    );
                }
            }
        }

        if let Some(error_seq) = error_seq {
            errors.push(
                ParseError::UnexpectedSequence {
                    expected: expected.to_vec(),
                    found: (*error_seq).clone(),
                }
                .into_spanned(error_seq.span),
            );
        }

        errors.push(
            ParseError::UnexpectedEof {
                expected: expected.to_vec(),
            }
            .into_spanned(self.eof),
        );
        Err(())
    }

    fn expect_contents<const I: u32>(
        &mut self,
        expected: TokenValue,
        errors: &mut Vec<Spanned<ParseError>>,
    ) -> Result<MaybeSpanned<I>, ()> {
        let mut error_seq: Option<Spanned<String>> = None;
        for token in self.iter.by_ref() {
            if !expected.is_same_kind(&token.token) {
                if let Some(error_seq) = &mut error_seq {
                    error_seq.push_str(token.token.as_str());
                    error_seq.span = error_seq.span.union(token.source.span);
                } else {
                    error_seq = Some(
                        token
                            .token
                            .as_str()
                            .to_string()
                            .into_spanned(token.source.span),
                    );
                }
            } else {
                match &token.token {
                    TokenValue::RawString(s)
                    | TokenValue::Identifier(s)
                    | TokenValue::Title(s)
                    | TokenValue::AttributeKey(s)
                    | TokenValue::AttributeValue(s) => {
                        if let Some(error_seq) = error_seq {
                            errors.push(
                                ParseError::UnexpectedSequence {
                                    expected: vec![expected],
                                    found: (*error_seq).clone(),
                                }
                                .into_spanned(error_seq.span),
                            );
                        }

                        return Ok(MaybeSpanned {
                            value: s.clone(),
                            span: Some(token.source.span),
                        });
                    }
                    _ => {}
                }
            }
        }

        if let Some(error_seq) = error_seq {
            errors.push(
                ParseError::UnexpectedSequence {
                    expected: vec![expected.clone()],
                    found: (*error_seq).clone(),
                }
                .into_spanned(error_seq.span),
            );
        }

        errors.push(
            ParseError::UnexpectedEof {
                expected: vec![expected],
            }
            .into_spanned(self.eof),
        );
        Err(())
    }

    fn peek_next_n<const N: usize>(&mut self) -> Option<[TokenValue; N]> {
        self.iter
            .peek_amount(N)
            .iter()
            .map(|token| token.as_ref().map(|t| t.token.clone()))
            .collect::<Option<Vec<_>>>()?
            .try_into()
            .ok()
    }
}
