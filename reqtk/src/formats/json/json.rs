use crate::*;
use req_file::prelude::*;

pub struct Json {
    pub spans: bool,
}

impl FormatImport for Json {
    fn extension(&self) -> &'static str {
        "json"
    }
    fn import(&self, location: &Location) -> Result<RequirementTree, CommandError> {
        Ok(serde_json::from_reader(location.reader()?).err_localized(location)?)
    }
}

impl FormatExport for Json {
    fn extension(&self) -> &'static str {
        "json"
    }

    fn export(
        &self,
        location: &Location,
        mut req_tree: RequirementTree,
    ) -> Result<(), CommandError> {
        if !self.spans {
            req_tree.clear_spans();
        };

        Ok(
            JsonFormatter::serialize_into(location.writer(true)?, &req_tree)
                .err_localized(location)?,
        )
    }
}
