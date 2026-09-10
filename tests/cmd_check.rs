use glob::glob;
use reqtk::*;
use std::{
    env,
    io::Write,
    process::{ExitStatus, Stdio},
};

#[test]
fn test_check_samples() {
    let exe = env::var("CARGO_BIN_EXE_reqtk").unwrap();
    for sample in glob("tests/samples/*.req").unwrap() {
        let sample = sample.unwrap();
        println!("sample: {:?}", sample);
        let process = std::process::Command::new(&exe)
            .args([
                "-r".to_string(),
                "json".into(),
                "check".into(),
                sample.to_str().unwrap().into(),
            ])
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let output = process.wait_with_output().unwrap();

        assert!(output.stdout.is_empty());
        println!("stderr: {:?}", String::from_utf8_lossy(&output.stderr));
        if ["tests/samples/no_text.req"].contains(&sample.to_str().unwrap()) {
            assert!(!output.stderr.is_empty());
            assert!(!output.status.success());
        } else {
            assert!(output.stderr.is_empty());
            assert!(output.status.success());
        }
    }
}

#[test]
fn test_check_file_level_attribute_key() {
    assert_eq!(
        run_check_fail("[=]"),
        vec![Issue {
            severity: IssueSeverity::Error,
            location: Some(IssueLocation {
                location: Location::StdIo,
                span: Some(Span::new(Cursor::end("["), Cursor::end("["))),
            }),
            message: "\"\" identifier of attribute does not match regex \"^[a-z0-9-]+$\""
                .to_string(),
        }]
    );
    assert_eq!(
        run_check_fail("[_=]"),
        vec![Issue {
            severity: IssueSeverity::Error,
            location: Some(IssueLocation {
                location: Location::StdIo,
                span: Some(Span::new(Cursor::end("["), Cursor::end("[_"))),
            }),
            message: "\"_\" identifier of attribute does not match regex \"^[a-z0-9-]+$\""
                .to_string(),
        }]
    );
    assert_eq!(run_check_success("[a=]"), vec![]);
}

#[test]
fn test_check_requirement_attribute_key() {
    assert_eq!(
        run_check_fail("@id(Title){[=]}"),
        vec![Issue {
            severity: IssueSeverity::Error,
            location: Some(IssueLocation {
                location: Location::StdIo,
                span: Some(Span::new(
                    Cursor::end("@id(Title){["),
                    Cursor::end("@id(Title){[")
                )),
            }),
            message: "\"\" identifier of attribute does not match regex \"^[a-z0-9-]+$\""
                .to_string(),
        }]
    );
    assert_eq!(
        run_check_fail("@id(Title){[_=]}"),
        vec![Issue {
            severity: IssueSeverity::Error,
            location: Some(IssueLocation {
                location: Location::StdIo,
                span: Some(Span::new(
                    Cursor::end("@id(Title){["),
                    Cursor::end("@id(Title){[_")
                )),
            }),
            message: "\"_\" identifier of attribute does not match regex \"^[a-z0-9-]+$\""
                .to_string(),
        }]
    );
    assert_eq!(run_check_success("@id(Title){[a=]}"), vec![]);
}

fn run_check_fail(source: &str) -> Issues {
    let result = run_check(source);
    assert!(!result.0.success());
    result.1
}

fn run_check_success(source: &str) -> Issues {
    let result = run_check(source);
    assert!(result.0.success());
    result.1
}

fn run_check(source: &str) -> (ExitStatus, Issues) {
    let exe = env::var("CARGO_BIN_EXE_reqtk").unwrap();
    let process = std::process::Command::new(&exe)
        .args(["-r", "json", "check", "-"])
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    process
        .stdin
        .as_ref()
        .unwrap()
        .write_all(source.as_bytes())
        .unwrap();

    let output = process.wait_with_output().unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.is_empty());
    (
        output.status,
        if output.stderr.is_empty() {
            vec![]
        } else {
            serde_json::from_slice(&output.stderr).unwrap()
        },
    )
}
