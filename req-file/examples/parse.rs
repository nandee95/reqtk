use req_file::{text_tokenizer::TextTokenizer, token_parser::TokenParser};
use std::{error::Error, io::BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let req = "@REQ-1(Hello){@REQ-2(World)\t\t\t\t{[type=functional][description\t\t\t\t]Hello world![/description]}";

    let mut reader = BufReader::new(req.as_bytes());
    let tokens = TextTokenizer::tokenize(&mut reader)?;

    let tree = TokenParser::parse(tokens);

    println!("{:?}", tree);

    Ok(())
}
