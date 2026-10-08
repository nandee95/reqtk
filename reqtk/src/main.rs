#![doc = include_str!("../README.md")]
#![allow(clippy::module_inception)]
use clap::Parser;
use reqtk::*;
use std::process::ExitCode;
fn main() -> ExitCode {
    let cli = Cli::parse();

    let mut context = CommandContext::new(&cli);
    cli.sub_command.execute(&mut context);

    ExitCode::from(context.issues.report())
}
