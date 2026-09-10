use crate::*;

pub struct ReqImpex;

impl RequirementTreeImport for ReqImpex {
    fn extension(&self) -> &'static str {
        "req"
    }
    fn import(&self, location: &Location) -> Result<RequirementTree, CommandError> {
        let tokens = TextTokenizer::tokenize_location(location)?;
        Parser::parse_location(location, tokens)
    }
}

impl RequirementTreeExport for ReqImpex {
    fn extension(&self) -> &'static str {
        "req"
    }
    fn export(&self, location: &Location, req_tree: RequirementTree) -> Result<(), CommandError> {
        let tokens = TreeTokenizer::tokenize(&req_tree);
        TokenWriter::write_reformat(&mut location.writer()?, &tokens).err_localized(location)?;
        Ok(())
    }
}
