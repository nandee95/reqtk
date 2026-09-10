use crate::*;
pub struct IconProvider;

impl IconProvider {
    pub fn get_material_icon(name: &str, color: &str) -> Option<String> {
        MATERIAL_ICONS.get(name).map(|path| format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path d="{}" fill="{}"/></svg>"#,
            path,
            color.replace("\"", "\\\"")
        ))
    }
}
