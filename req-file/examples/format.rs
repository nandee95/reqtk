use req_file::{
    text_tokenizer::TextTokenizer,
    token_writer::{Formatting, TokenWriter},
};
use std::{
    error::Error,
    io::{BufReader, BufWriter},
};

fn main() -> Result<(), Box<dyn Error>> {
    let req = "@REQ-1(Hello){@REQ-2(World)\t\t\t\t{[type=functional][description\t\t\t\t]Hello world![/description]}}";

    let mut reader = BufReader::new(req.as_bytes());
    let tokens = TextTokenizer::tokenize(&mut reader)?;
    for format in [
        Formatting::Preserve,
        Formatting::Reformat,
        Formatting::Minify,
    ] {
        let mut writer = BufWriter::new(Vec::new());
        TokenWriter::write(&mut writer, tokens.clone(), format)?;
        let result = String::from_utf8(writer.into_inner()?)?;
        println!("{:?}:\n{}", format, result);
    }

    Ok(())
}
