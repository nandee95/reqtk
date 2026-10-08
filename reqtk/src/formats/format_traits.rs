use crate::*;
use req_file::prelude::*;

pub trait FormatImport {
    fn extension(&self) -> &'static str;
    fn import(&self, location: &Location) -> Result<RequirementTree, CommandError>;
}

pub trait FormatExport {
    fn extension(&self) -> &'static str;
    fn export(&self, location: &Location, req_tree: RequirementTree) -> Result<(), CommandError>;
}
