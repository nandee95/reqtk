use crate::*;
use peekmore::PeekMore;
use std::io::{BufReader, Error as IoError, Read};
use utf8_chars::BufReadCharsExt;

pub struct TextTokenizer {
    cursor: Cursor,
    result: Vec<Token>,
    follower: TokenFollower,
}

impl TextTokenizer {
    pub fn tokenize_location(location: &Location) -> Result<Vec<Token>, Localized<IoError>> {
        Self::tokenize(&mut location.reader()?).err_localized(location)
    }

    pub fn tokenize<R: Read>(reader: &mut R) -> Result<Vec<Token>, IoError> {
        let mut bufreader = BufReader::new(reader);
        let mut iter = bufreader.chars().peekmore();

        let mut tokenizer = Self {
            cursor: Cursor::start(),
            result: vec![],
            follower: TokenFollower::default(),
        };

        loop {
            if !tokenizer.read_token(&mut iter)? {
                break;
            }

            let token = tokenizer.result.last().unwrap();

            tokenizer.follower = token.token.follower(&tokenizer.result).unwrap_or_default();
        }

        Ok(tokenizer.result)
    }

    fn read_token<I: Iterator<Item = Result<char, IoError>>>(
        &mut self,
        iter: &mut I,
    ) -> Result<bool, IoError> {
        let mut text = String::new();
        let mut escaped = false;
        loop {
            if let Some(ch) = iter.next().transpose()? {
                if !escaped
                    && let Ok(token) = TokenValue::try_from(ch)
                    && (self.follower.terminator.is_empty()
                        || self.follower.terminator.contains(&token))
                {
                    let text_span = self.cursor.advance_with(&text);
                    if !text.trim().is_empty() {
                        self.result.push(Token::from_source(
                            self.follower.kind.clone(),
                            text.into_spanned(text_span),
                        ));
                    }

                    let start = self.cursor;
                    self.cursor.advance_ch(ch);
                    self.result.push(Token::from_source(
                        token,
                        ch.to_string().into_spanned(Span::new(start, self.cursor)),
                    ));
                    break;
                }
                text.push(ch);
                escaped = !escaped && ch == ESCAPE_CHARACTER.ch;
            } else {
                let text_span = self.cursor.advance_with(&text);
                if !text.trim().is_empty() {
                    self.result.push(Token::from_source(
                        self.follower.kind.clone(),
                        text.into_spanned(text_span),
                    ));
                }
                return Ok(false);
            }
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_vec_eq {
        ($actual:expr, $expected:expr) => {{
            let actual = &$actual;
            let expected = &$expected;

            for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
                assert_eq!(actual, expected, "vector mismatch at index {index}");
            }
        }};
    }

    #[test]
    fn test_tokenize_empty() {
        let input = "";
        let tokens = TextTokenizer::tokenize(&mut input.as_bytes()).unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_tokenize_non_token_string() {
        let input = "asd";
        let tokens = TextTokenizer::tokenize(&mut input.as_bytes()).unwrap();
        assert_eq!(
            tokens,
            vec![Token::from_value(
                TokenValue::RawString(input.into()),
                Span::new(Cursor::start(), Cursor::new(1, 4, 3))
            )]
        );
    }
    #[test]
    fn test_tokenize_start_with_non_token_characters() {
        let input = "asd@";
        let tokens = TextTokenizer::tokenize(&mut input.as_bytes()).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::from_value(
                    TokenValue::RawString("asd".into()),
                    Span::new(Cursor::start(), Cursor::new(1, 4, 3))
                ),
                Token::from_value(
                    TokenValue::RequirementStart,
                    Span::new(Cursor::new(1, 4, 3), Cursor::new(1, 5, 4))
                )
            ]
        );
    }

    #[test]
    fn test_tokenize_end_with_non_token_characters() {
        let input = ")asd";
        let tokens = TextTokenizer::tokenize(&mut input.as_bytes()).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::from_value(
                    TokenValue::TitleEnd,
                    Span::new(Cursor::new(1, 1, 0), Cursor::new(1, 2, 1))
                ),
                Token::from_value(
                    TokenValue::RawString("asd".into()),
                    Span::new(Cursor::new(1, 2, 1), Cursor::new(1, 5, 4))
                ),
            ]
        );
    }

    #[test]
    fn test_tokenize_utf8_characters() {
        let input = "válüé";
        let tokens = TextTokenizer::tokenize(&mut input.as_bytes()).unwrap();
        assert_eq!(
            tokens,
            vec![Token::from_value(
                TokenValue::RawString("válüé".into()),
                Span::new(Cursor::new(1, 1, 0), Cursor::new(1, 6, 8))
            ),]
        );
    }

    #[test]
    fn test_tokenize_single_attribute() {
        let input = "[attribute=value]";
        let tokens = TextTokenizer::tokenize(&mut input.as_bytes()).unwrap();
        assert_vec_eq!(
            tokens,
            vec![
                Token::from_value(
                    TokenValue::AttributeStart,
                    Span::new(Cursor::new(1, 1, 0), Cursor::new(1, 2, 1))
                ),
                Token::from_value(
                    TokenValue::AttributeKey("attribute".into()),
                    Span::new(Cursor::new(1, 2, 1), Cursor::new(1, 11, 10))
                ),
                Token::from_value(
                    TokenValue::AttributeSeparator,
                    Span::new(Cursor::new(1, 11, 10), Cursor::new(1, 12, 11))
                ),
                Token::from_value(
                    TokenValue::AttributeValue("value".into()),
                    Span::new(Cursor::new(1, 12, 11), Cursor::new(1, 17, 16))
                ),
                Token::from_value(
                    TokenValue::AttributeEnd,
                    Span::new(Cursor::new(1, 17, 16), Cursor::new(1, 18, 17))
                ),
            ]
        );
    }

    #[test]
    fn test_tokenize_simple_requirement() {
        let input = "@id(Title){[description=Hello world]}";
        let tokens = TextTokenizer::tokenize(&mut input.as_bytes()).unwrap();
        assert_vec_eq!(
            tokens,
            vec![
                Token::from_value(
                    TokenValue::RequirementStart,
                    Span::new(Cursor::new(1, 1, 0), Cursor::new(1, 2, 1))
                ),
                Token::from_value(
                    TokenValue::Identifier("id".into()),
                    Span::new(Cursor::new(1, 2, 1), Cursor::new(1, 4, 3))
                ),
                Token::from_value(
                    TokenValue::TitleStart,
                    Span::new(Cursor::new(1, 4, 3), Cursor::new(1, 5, 4))
                ),
                Token::from_value(
                    TokenValue::Title("Title".into()),
                    Span::new(Cursor::new(1, 5, 4), Cursor::new(1, 10, 9))
                ),
                Token::from_value(
                    TokenValue::TitleEnd,
                    Span::new(Cursor::new(1, 10, 9), Cursor::new(1, 11, 10))
                ),
                Token::from_value(
                    TokenValue::BodyStart,
                    Span::new(Cursor::new(1, 11, 10), Cursor::new(1, 12, 11))
                ),
                Token::from_value(
                    TokenValue::AttributeStart,
                    Span::new(Cursor::new(1, 12, 11), Cursor::new(1, 13, 12))
                ),
                Token::from_value(
                    TokenValue::AttributeKey("description".into()),
                    Span::new(Cursor::new(1, 13, 12), Cursor::new(1, 24, 23))
                ),
                Token::from_value(
                    TokenValue::AttributeSeparator,
                    Span::new(Cursor::new(1, 24, 23), Cursor::new(1, 25, 24))
                ),
                Token::from_value(
                    TokenValue::AttributeValue("Hello world".into()),
                    Span::new(Cursor::new(1, 25, 24), Cursor::new(1, 36, 35))
                ),
                Token::from_value(
                    TokenValue::AttributeEnd,
                    Span::new(Cursor::new(1, 36, 35), Cursor::new(1, 37, 36))
                ),
                Token::from_value(
                    TokenValue::BodyEnd,
                    Span::new(Cursor::new(1, 37, 36), Cursor::new(1, 38, 37))
                ),
            ]
        );
    }

    #[test]
    fn test_tokenize_requirement_empty_title() {
        let input = "@id(){[description=Hello world]}";
        let tokens = TextTokenizer::tokenize(&mut input.as_bytes()).unwrap();
        assert_vec_eq!(
            tokens,
            vec![
                Token::from_value(
                    TokenValue::RequirementStart,
                    Span::new(Cursor::new(1, 1, 0), Cursor::new(1, 2, 1))
                ),
                Token::from_value(
                    TokenValue::Identifier("id".into()),
                    Span::new(Cursor::new(1, 2, 1), Cursor::new(1, 4, 3))
                ),
                Token::from_value(
                    TokenValue::TitleStart,
                    Span::new(Cursor::new(1, 4, 3), Cursor::new(1, 5, 4))
                ),
                Token::from_value(
                    TokenValue::TitleEnd,
                    Span::new(Cursor::new(1, 5, 4), Cursor::new(1, 6, 5))
                ),
                Token::from_value(
                    TokenValue::BodyStart,
                    Span::new(Cursor::new(1, 6, 5), Cursor::new(1, 7, 6))
                ),
                Token::from_value(
                    TokenValue::AttributeStart,
                    Span::new(Cursor::new(1, 7, 6), Cursor::new(1, 8, 7))
                ),
                Token::from_value(
                    TokenValue::AttributeKey("description".into()),
                    Span::new(Cursor::new(1, 8, 7), Cursor::new(1, 19, 18))
                ),
                Token::from_value(
                    TokenValue::AttributeSeparator,
                    Span::new(Cursor::new(1, 19, 18), Cursor::new(1, 20, 19))
                ),
                Token::from_value(
                    TokenValue::AttributeValue("Hello world".into()),
                    Span::new(Cursor::new(1, 20, 19), Cursor::new(1, 31, 30))
                ),
                Token::from_value(
                    TokenValue::AttributeEnd,
                    Span::new(Cursor::new(1, 31, 30), Cursor::new(1, 32, 31))
                ),
                Token::from_value(
                    TokenValue::BodyEnd,
                    Span::new(Cursor::new(1, 32, 31), Cursor::new(1, 33, 32))
                ),
            ]
        );
    }

    #[test]
    fn test_tokenize_simple_requirement_type() {
        let input = "$custom(Custom){[icon=build]}";
        let tokens = TextTokenizer::tokenize(&mut input.as_bytes()).unwrap();
        assert_vec_eq!(
            tokens,
            vec![
                Token::from_value(
                    TokenValue::RequirementTypeStart,
                    Span::new(Cursor::new(1, 1, 0), Cursor::new(1, 2, 1))
                ),
                Token::from_value(
                    TokenValue::Identifier("custom".into()),
                    Span::new(Cursor::new(1, 2, 1), Cursor::new(1, 8, 7))
                ),
                Token::from_value(
                    TokenValue::TitleStart,
                    Span::new(Cursor::new(1, 8, 7), Cursor::new(1, 9, 8))
                ),
                Token::from_value(
                    TokenValue::Title("Custom".into()),
                    Span::new(Cursor::new(1, 9, 8), Cursor::new(1, 15, 14))
                ),
                Token::from_value(
                    TokenValue::TitleEnd,
                    Span::new(Cursor::new(1, 15, 14), Cursor::new(1, 16, 15))
                ),
                Token::from_value(
                    TokenValue::BodyStart,
                    Span::new(Cursor::new(1, 16, 15), Cursor::new(1, 17, 16))
                ),
                Token::from_value(
                    TokenValue::AttributeStart,
                    Span::new(Cursor::new(1, 17, 16), Cursor::new(1, 18, 17))
                ),
                Token::from_value(
                    TokenValue::AttributeKey("icon".into()),
                    Span::new(Cursor::new(1, 18, 17), Cursor::new(1, 22, 21))
                ),
                Token::from_value(
                    TokenValue::AttributeSeparator,
                    Span::new(Cursor::new(1, 22, 21), Cursor::new(1, 23, 22))
                ),
                Token::from_value(
                    TokenValue::AttributeValue("build".into()),
                    Span::new(Cursor::new(1, 23, 22), Cursor::new(1, 28, 27))
                ),
                Token::from_value(
                    TokenValue::AttributeEnd,
                    Span::new(Cursor::new(1, 28, 27), Cursor::new(1, 29, 28))
                ),
                Token::from_value(
                    TokenValue::BodyEnd,
                    Span::new(Cursor::new(1, 29, 28), Cursor::new(1, 30, 29))
                ),
            ]
        );
    }
}
