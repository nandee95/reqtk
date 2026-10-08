use crate::prelude::*;
use std::io::{Error as IoError, Write};

#[derive(Debug, Copy, Clone)]
pub enum Formatting {
    Preserve,
    Reformat,
    Minify,
}

#[derive(Debug)]
pub struct TokenWriter;

impl TokenWriter {
    pub fn write(
        writer: &mut dyn Write,
        tokens: Vec<Token>,
        formatting: Formatting,
    ) -> Result<(), IoError> {
        match formatting {
            Formatting::Preserve => Self::write_unchanged(writer, tokens),
            Formatting::Reformat => Self::write_reformat(writer, &tokens),
            Formatting::Minify => Self::write_minify(writer, &tokens),
        }
    }

    pub fn write_unchanged(writer: &mut dyn Write, tokens: Vec<Token>) -> Result<(), IoError> {
        for token in tokens {
            writer.write_all(token.source.value.as_bytes())?;
        }
        Ok(())
    }

    pub fn write_reformat<'a, T>(writer: &mut dyn Write, tokens: &'a [T]) -> Result<(), IoError>
    where
        &'a T: Into<&'a TokenKind>,
    {
        let mut indent = 0;
        let mut iter = tokens.iter().enumerate().peekable();
        let mut prev = None;
        let mut keep_inline = false;
        while let Some((index, token)) = iter.next() {
            let token: &TokenKind = token.into();
            let flags = token.flags();

            if flags.any(TokenFormatFlags::DecrementIdentation) && indent > 0 {
                indent -= 1;
            }

            if flags.any(TokenFormatFlags::StartInline) {
                keep_inline = true;
            } else if flags.any(TokenFormatFlags::EndInline) {
                keep_inline = false;
            }

            if flags.any(TokenFormatFlags::IdentBefore)
                || (prev == Some(TokenKind::AttributeEnd) && token.has_contents())
            {
                writer.write_all(&vec![b'\t'; indent])?;
            }

            if token.has_contents() {
                let text = token.value_as_source(&"\t".repeat(indent), &tokens[..index]);
                writer.write_all(text.as_bytes())?;
            } else {
                writer.write_all(token.as_str().as_bytes())?;
            }

            if flags.any(TokenFormatFlags::IncrementIdentation) {
                indent += 1;
            }

            if iter.peek().is_some() {
                if flags.any(TokenFormatFlags::NewLineAfter)
                    || (token.has_contents() && !keep_inline)
                {
                    writer.write_all(b"\n")?;
                }

                if flags.any(TokenFormatFlags::SpaceAfter) {
                    writer.write_all(b" ")?;
                }
            }

            prev = Some(token.clone());
        }
        Ok(())
    }

    pub fn write_minify<'a, T>(writer: &mut dyn Write, tokens: &'a [T]) -> Result<(), IoError>
    where
        &'a T: Into<&'a TokenKind>,
    {
        for token in tokens {
            let token: &TokenKind = token.into();

            writer.write_all(token.value_as_source("", tokens).as_bytes())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_string(tokens: &[TokenKind]) -> String {
        let mut output = Vec::new();
        TokenWriter::write_reformat(&mut output, tokens).unwrap();
        String::from_utf8(output).unwrap()
    }

    #[test]
    fn test_requirement_writer() {
        let tokens = |id: &str, title: &str| {
            vec![
                TokenKind::RequirementStart,
                TokenKind::Identifier(id.into()),
                TokenKind::TitleStart,
                TokenKind::Title(title.into()),
                TokenKind::TitleEnd,
            ]
        };

        assert_eq!(test_string(&tokens("", "")), r#"@()"#);
        assert_eq!(test_string(&tokens("id", "")), r#"@id()"#);
        assert_eq!(test_string(&tokens("", "title")), r#"@(title)"#);

        assert_eq!(test_string(&tokens("🔥", "title")), r#"@🔥(title)"#);
        assert_eq!(test_string(&tokens("i", "title")), r#"@i(title)"#);
        assert_eq!(test_string(&tokens("í", "title")), r#"@í(title)"#);
        assert_eq!(test_string(&tokens("ídé", "title")), r#"@ídé(title)"#);
        assert_eq!(test_string(&tokens("id", "title")), r#"@id(title)"#);
        assert_eq!(test_string(&tokens("i(d", "title")), r#"@i\(d(title)"#);
        assert_eq!(test_string(&tokens("i@d", "title")), r#"@i@d(title)"#);
        assert_eq!(test_string(&tokens("i[d", "title")), r#"@i[d(title)"#);
        assert_eq!(test_string(&tokens("i]d", "title")), r#"@i]d(title)"#);
        assert_eq!(test_string(&tokens("i\\(d", "title")), r#"@i\\\(d(title)"#);
        assert_eq!(test_string(&tokens("(", "title")), r#"@\((title)"#);
        assert_eq!(test_string(&tokens("\\", "title")), r#"@\\(title)"#);
        assert_eq!(test_string(&tokens("\\(", "title")), r#"@\\\((title)"#);

        assert_eq!(test_string(&tokens("id", "🔥")), r#"@id(🔥)"#);
        assert_eq!(test_string(&tokens("id", "e")), r#"@id(e)"#);
        assert_eq!(test_string(&tokens("id", "é")), r#"@id(é)"#);
        assert_eq!(test_string(&tokens("id", "títlé")), r#"@id(títlé)"#);
        assert_eq!(test_string(&tokens("id", "title")), r#"@id(title)"#);
        assert_eq!(test_string(&tokens("id", "tit)le")), r#"@id(tit\)le)"#);
        assert_eq!(test_string(&tokens("id", "tit@le")), r#"@id(tit@le)"#);
        assert_eq!(test_string(&tokens("id", "tit[le")), r#"@id(tit[le)"#);
        assert_eq!(test_string(&tokens("id", "tit]le")), r#"@id(tit]le)"#);
        assert_eq!(test_string(&tokens("id", "tit\\)le")), r#"@id(tit\\\)le)"#);
        assert_eq!(test_string(&tokens("id", ")")), r#"@id(\))"#);
        assert_eq!(test_string(&tokens("id", "\\")), r#"@id(\\)"#);
        assert_eq!(test_string(&tokens("id", "\\)")), r#"@id(\\\))"#);
    }

    #[test]
    fn test_single_line_attribute_writer() {
        let t = |key: &str, value: &str| {
            vec![
                TokenKind::AttributeStart,
                TokenKind::AttributeKey(key.into()),
                TokenKind::AttributeSeparator,
                TokenKind::AttributeValue(value.into()),
                TokenKind::AttributeEnd,
            ]
        };

        assert_eq!(test_string(&t("", "")), r#"[=]"#);
        assert_eq!(test_string(&t("key", "")), r#"[key=]"#);
        assert_eq!(test_string(&t("", "value")), r#"[=value]"#);

        assert_eq!(test_string(&t("🔥", "value")), r#"[🔥=value]"#);
        assert_eq!(test_string(&t("e", "value")), r#"[e=value]"#);
        assert_eq!(test_string(&t("é", "value")), r#"[é=value]"#);
        assert_eq!(test_string(&t("kéy", "value")), r#"[kéy=value]"#);
        assert_eq!(test_string(&t("key", "value")), r#"[key=value]"#);
        assert_eq!(test_string(&t("k]ey", "value")), r#"[k\]ey=value]"#);
        assert_eq!(test_string(&t("k@ey", "value")), r#"[k@ey=value]"#);
        assert_eq!(test_string(&t("k$ey", "value")), r#"[k$ey=value]"#);
        assert_eq!(test_string(&t("k=ey", "value")), r#"[k\=ey=value]"#);
        assert_eq!(test_string(&t("k\\]ey", "value")), r#"[k\\\]ey=value]"#);
        assert_eq!(test_string(&t("=", "value")), r#"[\==value]"#);
        assert_eq!(test_string(&t("\\", "value")), r#"[\\=value]"#);
        assert_eq!(test_string(&t("\\=", "value")), r#"[\\\==value]"#);

        assert_eq!(test_string(&t("key", "🔥")), r#"[key=🔥]"#);
        assert_eq!(test_string(&t("key", "a")), r#"[key=a]"#);
        assert_eq!(test_string(&t("key", "á")), r#"[key=á]"#);
        assert_eq!(test_string(&t("key", "válué")), r#"[key=válué]"#);
        assert_eq!(test_string(&t("key", "value")), r#"[key=value]"#);
        assert_eq!(test_string(&t("key", "va]lue")), r#"[key=va\]lue]"#);
        assert_eq!(test_string(&t("key", "val@ue")), r#"[key=val@ue]"#);
        assert_eq!(test_string(&t("key", "val]ue")), r#"[key=val\]ue]"#);
        assert_eq!(test_string(&t("key", "va=lue")), r#"[key=va=lue]"#);
        assert_eq!(test_string(&t("key", "va\\]lue")), r#"[key=va\\\]lue]"#);
        assert_eq!(test_string(&t("key", "]")), r#"[key=\]]"#);
        assert_eq!(test_string(&t("key", "\\")), r#"[key=\\]"#);
        assert_eq!(test_string(&t("key", "\\]")), r#"[key=\\\]]"#);
        assert_eq!(test_string(&t("key", "=")), r#"[key==]"#);
    }
}
