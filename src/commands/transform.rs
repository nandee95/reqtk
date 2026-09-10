use crate::*;
use clap::Args;
use std::path::PathBuf;
use std::str::FromStr;

const ATTR_ID_PREFIX: &str = "id-prefix";
const ATTR_ID_TYPE: &str = "id-type";
const ATTR_ID_COUNT: &str = "id-count";

const ID_TYPE_INCREMENTAL_DEFAULT: u8 = 0;

#[derive(Debug, Clone)]
pub enum IdType {
    Incremental(u8),
}

impl FromStr for IdType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value == "incremental" {
            return Ok(Self::Incremental(ID_TYPE_INCREMENTAL_DEFAULT));
        } else if let Some(n) = value.strip_prefix("incremental-") {
            return Ok(Self::Incremental(
                n.parse::<u8>()
                    .map_err(|_| "invalid incremental length".to_string())?,
            ));
        }

        Err(format!("unknown id-type: {value:?}"))
    }
}

impl std::fmt::Display for IdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Incremental(n) => {
                if *n == ID_TYPE_INCREMENTAL_DEFAULT {
                    f.write_str("incremental")
                } else {
                    f.write_fmt(format_args!("incremental-{n}"))
                }
            }
        }
    }
}

#[derive(Args, Debug)]
pub struct TransformCommand {
    /// Input locations. When missing the inputs are taken from nearest 'reqtk.json'.
    /// Use a single '-' for stdin.
    inputs: Option<Vec<PathBuf>>,
    /// Output file for single input.
    /// Use a '-' for stdout.
    #[arg(short, long)]
    output: Option<Output>,
    /// Formatting of the output.
    #[arg(short, long, default_value_t = Formatting::Reformat)]
    pub format: Formatting,
    /// Re-apply ids for the file. [possible values: incremental|incremental(n)|random|random(n)] (n=length)
    #[arg(long)]
    pub re_id: Option<IdType>,
    /// Re-apply a prefix to the file.
    #[arg(long)]
    pub re_prefix: Option<String>,
}

impl Command for TransformCommand {
    fn execute(&self, mut context: CommandContext) {
        let targets = context.target(
            self.inputs.as_ref().unwrap_or(&Vec::new()),
            &self.output,
            None,
            ReqTkTargets::Requirements.into(),
        );
        let Some(targets) = context.consume_err(targets) else {
            return;
        };

        if targets.len() != 1 && self.re_prefix.is_some() {
            context.push_issue(Issue::new(
                IssueSeverity::Error,
                "Prefix can only be used with a single input".into(),
            ));
            return;
        }

        for target in &targets {
            context.consume_err(self.run_transform(target));
        }
    }
}

impl TransformCommand {
    fn run_transform(&self, target: &Target) -> Result<(), CommandError> {
        let mut tree = ReqImpex.import(&target.input)?;

        let prefix = self
            .re_prefix
            .as_deref()
            .or_else(|| {
                tree.attributes
                    .iter()
                    .find(|v| v.key.as_str() == ATTR_ID_PREFIX)
                    .map(|v| v.value.as_str())
            })
            .unwrap_or_default()
            .to_string();

        let id_type = self.re_id.clone().unwrap_or_else(|| {
            tree.attributes
                .iter()
                .find(|v| v.key.as_str() == ATTR_ID_TYPE)
                .and_then(|v| IdType::from_str(v.value.as_str()).ok())
                .unwrap_or(IdType::Incremental(ID_TYPE_INCREMENTAL_DEFAULT))
        });

        self.walk_tree(&mut tree, &id_type, &prefix)?;

        if !prefix.is_empty() {
            tree.attributes.set("id-prefix", &prefix);
        }

        ReqImpex.export(&target.output, tree)?;
        Ok(())
    }

    fn walk_tree(
        &self,
        tree: &mut RequirementTree,
        id_type: &IdType,
        prefix: &str,
    ) -> Result<(), CommandError> {
        let mut count = 0;
        let mut existing = Vec::new();
        for requirement in &mut tree.requirements {
            self.walk_requirement(requirement, id_type, prefix, &mut count, &mut existing)?;
        }

        if self.re_id.is_some() {
            if let Some(IdType::Incremental(_)) = &self.re_id {
                tree.attributes.set("id-count", &count.to_string());
            } else {
                tree.attributes.remove_attr(ATTR_ID_COUNT);
            }
            tree.attributes.set("id-type", &id_type.to_string());
        }

        Ok(())
    }

    fn walk_requirement(
        &self,
        requirement: &mut Requirement,
        id_type: &IdType,
        prefix: &str,
        count: &mut usize,
        existing: &mut Vec<String>,
    ) -> Result<(), CommandError> {
        requirement.id.value = format!(
            "{}{}",
            prefix,
            match id_type {
                IdType::Incremental(n) => {
                    *count += 1;

                    format!("{:0width$}", count, width = *n as usize)
                }
            }
        );

        for child in &mut requirement.children {
            self.walk_requirement(child, id_type, prefix, count, existing)?;
        }

        Ok(())
    }
}

trait AttributesExt {
    fn get_mut(&mut self, key: &str) -> Option<&mut String>;
    fn set(&mut self, key: &str, value: &str);
    fn remove_attr(&mut self, key: &str);
}

impl AttributesExt for Attributes {
    fn get_mut(&mut self, key: &str) -> Option<&mut String> {
        self.iter_mut()
            .find(|v| v.key.as_str() == key)
            .map(|v| &mut v.value.value)
    }
    fn set(&mut self, key: &str, value: &str) {
        if let Some(attr) = self.get_mut(key) {
            *attr = value.to_string();
        } else {
            self.push(Attribute {
                key: MaybeSpanned::unspanned(key),
                value: MaybeSpanned::unspanned(value),
                span: None,
            });
        }
    }
    fn remove_attr(&mut self, key: &str) {
        if let Some(pos) = self.iter().position(|v| v.key.as_str() == key) {
            self.remove(pos);
        }
    }
}
