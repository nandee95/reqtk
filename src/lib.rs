#[allow(clippy::only_used_in_recursion)]
pub mod cli;
pub mod commands;
pub mod impex;
pub mod issue_tracker;
pub mod model;
pub mod reqtk_json;
pub mod traces;

pub use cli::*;
pub use commands::*;
pub use impex::*;
pub use issue_tracker::*;
pub use model::*;
pub use reqtk_json::*;
pub use traces::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn main_args(args: Vec<String>) -> u8 {
    use clap::Parser;
    let cli = Cli::parse_from(args);

    let mut issues = IssueTracker::new(cli.report);

    if issues.is_empty() {
        let context = CommandContext::new(&mut issues);
        cli.sub_command.execute(context);
    }

    issues.report()
}
