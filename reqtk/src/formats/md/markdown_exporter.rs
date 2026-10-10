use crate::*;
use fontdue::Font;
use html_escape::encode_safe;
use req_file::prelude::*;
use std::{fs::create_dir_all, io::Write, path::Path};

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
                return Err(CommandError::other(
                    "Cannot determine resources folder for StdIo",
                ));
            }
        };
        let mut exporter = Self {
            writer: location.writer(true)?,
            location: location.clone(),
            folder: resources
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            tree: req_tree,
        };

        create_dir_all(&resources).err_localized(&Location::Path(resources.clone()))?;

        for req_type in [
            req_tree.types.clone(),
            RequirementType::builtin_types().to_vec(),
        ]
        .iter()
        .flatten()
        {
            std::fs::write(
                resources.join(format!("type_{}.svg", req_type.id.value)),
                IconProvider::get_material_icon(
                    req_type
                        .attributes
                        .iter()
                        .find(|a| a.key.value == Attribute::TYPE_ICON)
                        .map(|a| a.value.value.as_str())
                        .unwrap_or("error"),
                    req_type
                        .attributes
                        .iter()
                        .find(|a| a.key.value == Attribute::TYPE_COLOR)
                        .map(|a| a.value.value.as_str())
                        .unwrap_or("red"),
                )
                .ok_or_else(|| CommandError::other("Invalid material icon"))?
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
            .find(|v| v.key.value == "type")
            .map(|v| v.value.value.as_str());

        let requirement_type = format!(
            "<img src=\"{}/type_{}.svg\" /> ",
            self.folder,
            self.tree.resolve_requirement_type(type_key).id.value
        );

        // Write opening tags as raw HTML
        output.push_str("<details open>\n");

        // Note: <b> is used here instead of <hX> because it allows a denser view in most renderers
        output.push_str(&format!(
            "<summary><a id=\"{}\">{} <code>{}</code> <b>{}</b></a></summary>\n",
            requirement.id.value.replace("\"", "\\\""),
            requirement_type,
            encode_safe(requirement.id.value.as_str()),
            encode_safe(requirement.title.value.as_str()),
        ));

        output.push_str("<ul>\n");

        // Add description if present - this can be markdown content
        if let Some(description) = requirement
            .attributes
            .iter()
            .find(|a| a.key.value == "description")
            && !description.value.value.as_str().trim().is_empty()
        {
            let mut resolved = description.value.value.clone();
            let references = ReferenceIterator::new(
                &description.value.value,
                Some(Span::of(&description.value.value)),
            )
            .collect::<Vec<_>>();
            for reference in references.into_iter().rev() {
                let Some(ref_requirement) = self.tree.find(reference.value) else {
                    continue;
                };
                let ref_type = self.tree.resolve_requirement_type(
                    ref_requirement.attributes.get("type").map(String::as_str),
                );
                let filename = format!("{}/badge_{}.svg", self.folder, reference.value);

                if !Path::new(&filename).exists() {
                    std::fs::write(
                        filename,
                        Self::generate_badge_svg(
                            ref_type
                                .attributes
                                .get("icon")
                                .map(String::as_str)
                                .unwrap_or_else(|| "folder"),
                            ref_type
                                .attributes
                                .get("color")
                                .map(String::as_str)
                                .unwrap_or_else(|| "red"),
                            &ref_requirement.id.value,
                            &ref_requirement.title.value,
                        ),
                    )?;
                }

                if let Some(span) = reference.span {
                    let range = span.to_range();
                    resolved.replace_range(
                        ((range.start as isize - 2) as usize)..(range.end + 1),
                        &format!(
                            "[![{}]({}/badge_{}.svg)](#{})",
                            reference.value,
                            self.folder,
                            reference.value,
                            url_escape::encode_fragment(reference.value),
                        ),
                    );
                }
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

    pub fn text_width(font: &Font, text: &str, font_size: f32) -> f32 {
        text.chars()
            .map(|c| font.metrics(c, font_size).advance_width)
            .sum()
    }
    pub fn generate_badge_svg(icon: &str, icon_color: &str, id: &str, title: &str) -> String {
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
}
