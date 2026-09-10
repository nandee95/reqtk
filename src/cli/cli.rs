use crate::*;
use adar::macros::EnumTraitDeref;
use clap::{Parser, Subcommand, ValueEnum};
use std::{path::PathBuf, str::FromStr};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Sub command to run
    #[command(subcommand)]
    pub sub_command: SubCommands,
    /// Issue reporting format
    #[arg(short, long, default_value_t = OutputFormat::Human)]
    pub report: OutputFormat,
}

#[derive(Subcommand, Debug)]
#[EnumTraitDeref(Command)]
pub enum SubCommands {
    /// Format req files
    Fmt(FmtCommand),
    /// Analyze req files for issues
    Check(CheckCommand),
    /// Convert between known requirement formats (req, json)
    Convert(ConvertCommand),
    /// Transform a req file (e.g. reflow ids, minify)
    Transform(TransformCommand),
    /// Tokenize a req file (output: JSON)
    Tokenize(TokenizeCommand),
    /// Find traces for requirements (output: JSON)
    Trace(TraceCommand),
    /// Finds a single requirement (output: JSON)
    Find(FindCommand),
    /// Initializes a reqtk.json workspace file
    Init(InitCommand),
}

#[derive(ValueEnum, Debug, Copy, Clone)]
pub enum OutputFormat {
    Human,
    Json,
}

#[derive(Clone, Debug)]
pub enum Output {
    Stdout,
    Path(PathBuf),
}

impl FromStr for Output {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Output::Stdout),
            path => Ok(Output::Path(PathBuf::from(path))),
        }
    }
}

impl std::fmt::Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Output::Stdout => write!(f, "-"),
            Output::Path(path) => write!(f, "{}", path.to_string_lossy()),
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_possible_value().unwrap().get_name())
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum FindOutputFormat {
    Req,
    Json,
}

impl ToString for FindOutputFormat {
    fn to_string(&self) -> String {
        match self {
            FindOutputFormat::Req => "req".into(),
            FindOutputFormat::Json => "json".into(),
        }
    }
}

impl FromStr for FindOutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "req" => Ok(FindOutputFormat::Req),
            "json" => Ok(FindOutputFormat::Json),
            _ => Err(format!("Invalid output format: {}", s)),
        }
    }
}
