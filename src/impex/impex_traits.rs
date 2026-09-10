use crate::*;

pub trait RequirementTreeImport {
    fn extension(&self) -> &'static str;
    fn import(&self, location: &Location) -> Result<RequirementTree, CommandError>;
}

pub trait RequirementTreeExport {
    fn extension(&self) -> &'static str;
    fn export(&self, location: &Location, req_tree: RequirementTree) -> Result<(), CommandError>;
}
