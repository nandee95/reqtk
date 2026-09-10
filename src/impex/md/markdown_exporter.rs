use std::io::Write;

use crate::*;

pub struct MarkdownExporter<'a> {
    location: Location,
    writer: LocationIo,
    tree: &'a RequirementTree,
}

impl<'a> MarkdownExporter<'a> {
    pub fn export(location: Location, req_tree: &'a RequirementTree) -> Result<(), CommandError> {
        let mut exporter = Self {
            writer: location.writer()?,
            location,
            tree: req_tree,
        };

        for req in &exporter.tree.requirements {
            exporter.export_requirement(0, req)?;
        }
        Ok(())
    }

    fn export_requirement(
        &mut self,
        depth: u32,
        requirement: &Requirement,
    ) -> Result<(), CommandError> {
        let type_key = requirement
            .attributes
            .iter()
            .find(|v| v.key.as_str() == "type")
            .map(|v| v.value.to_string())
            .unwrap_or("folder".to_string());

        let requirement_type = self.get_requirement_type(&type_key).and_then(|v| {
            let icon = v.attributes.iter().find(|a| a.key.as_str() == "icon")?;
            let color = v.attributes.iter().find(|a| a.key.as_str() == "color")?;
            Some(format!(
                "<img src=\"https://api.iconify.design/ic/baseline-{}.svg?color=%23{}\" /> ",
                icon.value.as_str(),
                color.value.as_str().trim_start_matches('#')
            ))
        });

        write!(self.writer, "<details open>\n").err_localized(&self.location)?;

        // Note: <b> is used here instead of <hX> because it allows a denser view in most renderers
        writeln!(
            self.writer,
            "<summary><a id=\"{}\">{} {} <b>{}</b></a></summary>\n",
            requirement.id.as_str().replace("\"", "\\\""),
            requirement_type.unwrap_or_default(),
            requirement.id,
            requirement.title,
        )
        .err_localized(&self.location)?;

        write!(self.writer, "<blockquote>\n").err_localized(&self.location)?;

        if let Some(description) = requirement
            .attributes
            .iter()
            .find(|a| a.key.as_str() == "description")
        {
            writeln!(self.writer, "{}", description.value).err_localized(&self.location)?;
        }

        for child in &requirement.children {
            self.export_requirement(depth + 1, child)?;
        }

        write!(self.writer, "</blockquote>\n</details>\n").err_localized(&self.location)?;
        Ok(())
    }

    fn get_requirement_type(&self, requirement_type: &str) -> Option<RequirementType> {
        if let Some(rt) = self
            .tree
            .types
            .iter()
            .find(|rt| rt.id.as_str() == requirement_type)
        {
            return Some(rt.clone());
        }

        let builtins = &[
            RequirementType {
                id: MaybeSpanned::unspanned("folder"),
                title: MaybeSpanned::unspanned("Folder"),
                attributes: vec![
                    Attribute {
                        key: MaybeSpanned::unspanned("icon"),
                        value: MaybeSpanned::unspanned("folder"),
                        span: None,
                    },
                    Attribute {
                        key: MaybeSpanned::unspanned("color"),
                        value: MaybeSpanned::unspanned("#facc15"),
                        span: None,
                    },
                ],
                span: None,
            },
            RequirementType {
                id: MaybeSpanned::unspanned("functional"),
                title: MaybeSpanned::unspanned("Functional"),
                attributes: vec![
                    Attribute {
                        key: MaybeSpanned::unspanned("icon"),
                        value: MaybeSpanned::unspanned("bolt"),
                        span: None,
                    },
                    Attribute {
                        key: MaybeSpanned::unspanned("color"),
                        value: MaybeSpanned::unspanned("#3b82f6"),
                        span: None,
                    },
                ],
                span: None,
            },
            RequirementType {
                id: MaybeSpanned::unspanned("limitation"),
                title: MaybeSpanned::unspanned("Limitation"),
                attributes: vec![
                    Attribute {
                        key: MaybeSpanned::unspanned("icon"),
                        value: MaybeSpanned::unspanned("warning"),
                        span: None,
                    },
                    Attribute {
                        key: MaybeSpanned::unspanned("color"),
                        value: MaybeSpanned::unspanned("#ef4444"),
                        span: None,
                    },
                ],
                span: None,
            },
            RequirementType {
                id: MaybeSpanned::unspanned("parameter"),
                title: MaybeSpanned::unspanned("Parameter"),
                attributes: vec![
                    Attribute {
                        key: MaybeSpanned::unspanned("icon"),
                        value: MaybeSpanned::unspanned("tag"),
                        span: None,
                    },
                    Attribute {
                        key: MaybeSpanned::unspanned("color"),
                        value: MaybeSpanned::unspanned("#22c55e"),
                        span: None,
                    },
                ],
                span: None,
            },
            RequirementType {
                id: MaybeSpanned::unspanned("informative"),
                title: MaybeSpanned::unspanned("Informative"),
                attributes: vec![
                    Attribute {
                        key: MaybeSpanned::unspanned("icon"),
                        value: MaybeSpanned::unspanned("article"),
                        span: None,
                    },
                    Attribute {
                        key: MaybeSpanned::unspanned("color"),
                        value: MaybeSpanned::unspanned("#a855f7"),
                        span: None,
                    },
                ],
                span: None,
            },
        ];
        builtins
            .iter()
            .find(|rt| rt.id.as_str() == requirement_type)
            .cloned()
    }
}
