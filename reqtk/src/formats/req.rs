use crate::*;
use req_file::prelude::*;

pub struct Req;

impl FormatImport for Req {
    fn extension(&self) -> &'static str {
        "req"
    }
    fn import(&self, location: &Location) -> Result<RequirementTree, CommandError> {
        let tokens = TextTokenizer::tokenize(&mut location.reader()?).err_localized(location)?;
        Ok(TokenParser::parse(tokens).err_localized(location)?)
    }
}

impl FormatExport for Req {
    fn extension(&self) -> &'static str {
        "req"
    }
    fn export(&self, location: &Location, req_tree: RequirementTree) -> Result<(), CommandError> {
        let tokens = TreeTokenizer::tokenize(&req_tree);
        TokenWriter::write_reformat(&mut location.writer(true)?, &tokens)
            .err_localized(location)?;
        Ok(())
    }
}
