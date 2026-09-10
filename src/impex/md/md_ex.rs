use crate::*;

pub struct MdEx;

impl RequirementTreeExport for MdEx {
    fn extension(&self) -> &'static str {
        "md"
    }

    fn export(
        &self,
        location: &Location,
        mut req_tree: RequirementTree,
    ) -> Result<(), CommandError> {
        req_tree.clear_spans();
        MarkdownExporter::export(location.clone(), &req_tree)
    }
}
