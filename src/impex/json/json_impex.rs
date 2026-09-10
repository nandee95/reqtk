use crate::*;

pub struct JsonImpex {
    pub spans: bool,
}

impl RequirementTreeImport for JsonImpex {
    fn extension(&self) -> &'static str {
        "json"
    }
    fn import(&self, location: &Location) -> Result<RequirementTree, CommandError> {
        Ok(serde_json::from_reader(location.reader()?).err_localized(location)?)
    }
}

impl RequirementTreeExport for JsonImpex {
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

        Ok(JsonFormatter::serialize_into(location.writer()?, &req_tree).err_localized(location)?)
    }
}
