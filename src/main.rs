use reqtk::main_args;
use std::process::ExitCode;

fn main() -> ExitCode {
    ExitCode::from(main_args(std::env::args().collect()))
}
