#[allow(clippy::module_inception)]
pub mod cli;
pub mod commands;
pub mod diff_iterator;
pub mod generated;
pub mod icon_provider;
pub mod impex;
pub mod issue_tracker;
pub mod model;
pub mod reqtk_workspace;
pub mod traces;

pub use cli::*;
pub use commands::*;
pub use diff_iterator::*;
pub use generated::*;
pub use icon_provider::*;
pub use impex::*;
pub use issue_tracker::*;
pub use model::*;
pub use reqtk_workspace::*;
pub use traces::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn main_args(args: Vec<String>) -> u8 {
    use clap::Parser;
    let cli = Cli::parse_from(args);

    let mut issues = IssueTracker::new(cli.report);

    if issues.is_empty() {
        let context = CommandContext::new(&mut issues, cli.verbose);
        cli.sub_command.execute(context);
    }

    issues.report()
}
