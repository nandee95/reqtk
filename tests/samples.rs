use std::{env, io::Write, process::Stdio};

use glob::glob;

#[test]
fn test_fmt_is_lossless() {
    let exe = env::var("CARGO_BIN_EXE_reqtk").unwrap();
    for result in glob("tests/samples/*.req").unwrap() {
        let file = result.unwrap();

        let commands = vec![vec!["fmt".to_string(), "-".into()]];

        let input = std::fs::read_to_string(&file).unwrap();

        for i in 0..input.len() {
            let slice = &input[..i];

            let compressed_slice = slice.replace(char::is_whitespace, "").replace("\\", "");
            for args in &commands {
                let process = std::process::Command::new(&exe)
                    .args(args)
                    .stdin(std::process::Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap();

                process
                    .stdin
                    .as_ref()
                    .unwrap()
                    .write_all(slice.as_bytes())
                    .unwrap();

                let output = process.wait_with_output().unwrap();

                let stdout = String::from_utf8(output.stdout).unwrap();
                let compressed_stdout = stdout.replace(char::is_whitespace, "").replace("\\", "");
                assert_eq!(compressed_slice, compressed_stdout);
            }

            let slice2 = &input[i..];

            let compressed_slice2 = slice2.replace(char::is_whitespace, "").replace("\\", "");
            for args in &commands {
                let process = std::process::Command::new(&exe)
                    .args(args)
                    .stdin(std::process::Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap();

                process
                    .stdin
                    .as_ref()
                    .unwrap()
                    .write_all(slice2.as_bytes())
                    .unwrap();

                let output = process.wait_with_output().unwrap();

                let stdout = String::from_utf8(output.stdout).unwrap();
                let compressed_stdout = stdout.replace(char::is_whitespace, "").replace("\\", "");
                assert_eq!(compressed_slice2, compressed_stdout);
            }
        }
    }
}
