use std::io::{BufReader, BufWriter};

use glob::glob;
use reqtk::{TextTokenizer, TokenWriter};

#[test]
fn test_format_is_lossless() {
    for result in glob("tests/samples/*.req").unwrap() {
        let file = result.unwrap();

        let input = std::fs::read_to_string(&file).unwrap();

        for i in 0..input.chars().count() {
            let substr: String = input.chars().take(i).collect();

            let compressed_slice = substr.replace(char::is_whitespace, "").replace("\\", "");

            let tokens = TextTokenizer::tokenize(&mut BufReader::new(substr.as_bytes())).unwrap();
            let mut output = BufWriter::new(Vec::new());
            TokenWriter::write_reformat(&mut output, &tokens).unwrap();

            let stdout = String::from_utf8(output.into_inner().unwrap()).unwrap();
            let compressed_stdout = stdout.replace(char::is_whitespace, "").replace("\\", "");
            assert_eq!(compressed_slice, compressed_stdout);

            let substr2: String = input.chars().skip(i).collect();

            let compressed_slice2 = substr2.replace(char::is_whitespace, "").replace("\\", "");

            let tokens = TextTokenizer::tokenize(&mut BufReader::new(substr2.as_bytes())).unwrap();
            let mut output = BufWriter::new(Vec::new());
            TokenWriter::write_reformat(&mut output, &tokens).unwrap();

            let stdout = String::from_utf8(output.into_inner().unwrap()).unwrap();
            let compressed_stdout = stdout.replace(char::is_whitespace, "").replace("\\", "");
            assert_eq!(compressed_slice2, compressed_stdout);
        }
    }
}
