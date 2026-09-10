use crate::*;
use fontdue::Font;
use html_escape::encode_safe;
use std::{fs::create_dir_all, io::Write};

pub struct MarkdownExporter<'a> {
    location: Location,
    writer: LocationIo,
    folder: String,
    tree: &'a RequirementTree,
}

impl<'a> MarkdownExporter<'a> {
    pub fn export(location: Location, req_tree: &'a RequirementTree) -> Result<(), CommandError> {
        let resources = match &location {
            Location::Path(path_buf) => path_buf.with_extension(""),
            Location::StdIo => {
                return Err(CommandError::Other {
                    severity: IssueSeverity::Error,
                    message: "Cannot determine resources folder for StdIo".into(),
                    location: None,
                    span: None,
                });
            }
        };
        let mut exporter = Self {
            writer: location.writer()?,
            location: location.clone(),
            folder: resources
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            tree: req_tree,
        };

        create_dir_all(&resources).err_localized(&Location::Path(resources.clone()))?;

        for req_type in [req_tree.types.clone(), Self::builtin_types().to_vec()]
            .iter()
            .flatten()
        {
            std::fs::write(
                resources.join(format!("type_{}.svg", req_type.id)),
                IconProvider::get_material_icon(
                    req_type
                        .attributes
                        .iter()
                        .find(|a| a.key.as_str() == "icon")
                        .map(|a| a.value.as_str())
                        .unwrap_or("error"),
                    req_type
                        .attributes
                        .iter()
                        .find(|a| a.key.as_str() == "color")
                        .map(|a| a.value.as_str())
                        .unwrap_or("red"),
                )
                .ok_or_else(|| CommandError::Other {
                    severity: IssueSeverity::Error,
                    message: "Failed to generate icon".into(),
                    location: Some(location.clone()),
                    span: None,
                })?
                .as_bytes(),
            )
            .err_localized(&location)?;
        }

        // Build markdown content section by section
        let mut markdown_content = String::new();

        for req in &exporter.tree.requirements {
            exporter.build_requirement_markdown(&mut markdown_content, req)?;
        }

        // Write the markdown output
        exporter
            .writer
            .write_all(markdown_content.as_bytes())
            .err_localized(&exporter.location)?;

        Ok(())
    }

    fn build_requirement_markdown(
        &self,
        output: &mut String,
        requirement: &Requirement,
    ) -> Result<(), CommandError> {
        let type_key = requirement
            .attributes
            .iter()
            .find(|v| v.key.as_str() == "type")
            .map(|v| v.value.as_str())
            .unwrap_or("folder");

        let requirement_type = self.get_requirement_type(type_key).map(|v| {
            format!(
                "<img src=\"{}/type_{}.svg\" /> ",
                self.folder,
                v.id.as_str()
            )
        });

        // Write opening tags as raw HTML
        output.push_str("<details open>\n");

        // Note: <b> is used here instead of <hX> because it allows a denser view in most renderers
        output.push_str(&format!(
            "<summary><a id=\"{}\">{} <code>{}</code> <b>{}</b></a></summary>\n",
            requirement.id.as_str().replace("\"", "\\\""),
            requirement_type.unwrap_or_default(),
            encode_safe(requirement.id.as_str()),
            encode_safe(requirement.title.as_str()),
        ));

        output.push_str("<ul>\n");

        // Add description if present - this can be markdown content
        if let Some(description) = requirement
            .attributes
            .iter()
            .find(|a| a.key.as_str() == "description")
            && !description.value.as_str().trim().is_empty()
        {
            let mut resolved = description.value.value.clone();
            let references = ReferenceIterator::new(
                &description.value.value,
                Span::of(&description.value.value),
            )
            .collect::<Vec<_>>();
            for reference in references.into_iter().rev() {
                let Some(ref_requirement) = self.tree.find(&reference) else {
                    continue;
                };
                let Some(ref_type) = ref_requirement
                    .attributes
                    .get("type")
                    .and_then(|v| self.get_requirement_type(v))
                else {
                    continue;
                };

                std::fs::write(
                    format!("{}/badge_{}.svg", self.folder, *reference),
                    Self::badge(
                        ref_type.attributes.get("icon").unwrap_or("folder"),
                        ref_type.attributes.get("color").unwrap_or("red"),
                        ref_requirement.id.as_str(),
                        ref_requirement.title.as_str(),
                    ),
                )?;

                let range = reference.span.to_range();
                resolved.replace_range(
                    ((range.start as isize - 2) as usize)..(range.end + 1),
                    &format!(
                        "[![{}]({}/badge_{}.svg)](#{})",
                        *reference,
                        self.folder,
                        *reference,
                        url_escape::encode_fragment(*reference),
                    ),
                );
            }

            output.push_str(&format!("<blockquote>\n\n{}\n\n</blockquote>\n", resolved));
        }

        // Recursively process children
        for child in &requirement.children {
            self.build_requirement_markdown(output, child)?;
        }

        output.push_str("</ul>\n</details>\n");

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

        Self::builtin_types()
            .iter()
            .find(|rt| rt.id.as_str() == requirement_type)
            .cloned()
    }
    pub fn text_width(font: &Font, text: &str, font_size: f32) -> f32 {
        text.chars()
            .map(|c| font.metrics(c, font_size).advance_width)
            .sum()
    }
    pub fn badge(icon: &str, icon_color: &str, id: &str, title: &str) -> String {
        let path = MATERIAL_ICONS.get(icon).copied().unwrap_or("");

        let escaped_id = html_escape::encode_text(id);
        let escaped_title = html_escape::encode_text(title);

        const ARIAL_BYTES: &[u8] = include_bytes!("../../../resources/arial.ttf");
        let font = Font::from_bytes(ARIAL_BYTES, fontdue::FontSettings::default()).unwrap();

        const FONT_SIZE: f32 = 12.0;
        const ICON_SIZE: f32 = 17.0;
        const PADDING: f32 = 3.0;
        const GAP: f32 = 3.0;
        const ID_FILL: &str = "#AAA";
        const BACKGROUND_FILL: &str = "#222";

        let id_width = Self::text_width(&font, id, FONT_SIZE);
        let title_width = Self::text_width(&font, title, FONT_SIZE);

        let icon_x = PADDING;
        let id_x = icon_x + ICON_SIZE;
        let title_x = id_x + id_width + GAP;

        let width = title_x + title_width + PADDING * 2.0;

        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg"
            width="{width}"
            height="{ICON_SIZE}"
            viewBox="0 0 {width} {ICON_SIZE}"
            role="img"
            aria-label="{escaped_id} {escaped_title}">

            <rect
                x="0"
                y="0"
                width="{width}"
                height="{ICON_SIZE}"
                rx="3"
                fill="{BACKGROUND_FILL}"
            />

            <path
                d="{path}"
                fill="{icon_color}"
                transform="translate({icon_x}, 0),scale(0.7)"
            />

            <text
                x="{id_x}"
                y="12"
                font-family="Arial"
                font-size="{FONT_SIZE}"
                fill="{ID_FILL}"
            >{escaped_id}</text>

            <text
                x="{title_x}"
                y="12"
                font-family="Arial"
                font-size="{FONT_SIZE}"
                fill="white"
            >{escaped_title}</text>
        </svg>"#
        )
    }

    fn builtin_types() -> [RequirementType; 5] {
        [
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
        ]
    }
}
