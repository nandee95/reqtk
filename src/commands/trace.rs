use crate::*;
use clap::Args;
use std::collections::HashMap;
use std::io::stdout;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct TraceCommand {
    /// Input locations. When missing the inputs are taken from nearest 'reqtk.json'.
    // {@REQTK-24}
    inputs: Option<Vec<PathBuf>>,
}

impl Command for TraceCommand {
    fn execute(&self, mut context: CommandContext) {
        let result = self.execute_trace(&mut context);
        context.consume_err(result);
    }
}
impl TraceCommand {
    fn execute_trace(&self, context: &mut CommandContext) -> Result<(), CommandError> {
        let targets = context.target(
            self.inputs.as_ref().unwrap_or(&Vec::new()),
            &None,
            Some("trace.json"),
            ReqTkTargets::Requirements | ReqTkTargets::Sources,
        )?;

        let (reqs, sources): (Vec<_>, Vec<_>) = targets.iter().partition(|a| match &a.input {
            Location::StdIo => true,
            Location::Path(path) => {
                path.extension().and_then(|ext| ext.to_str()).unwrap_or("") == "req"
            }
        });

        if context.has_error() {
            return Ok(());
        }

        let Some(reqs) = reqs
            .into_iter()
            .map(|v| {
                context.consume_err(|| -> Result<_, CommandError> {
                    Ok((
                        v,
                        TextTokenizer::tokenize(&mut v.input.reader()?).err_localized(&v.input)?,
                    ))
                }())
            })
            .collect::<Option<Vec<_>>>()
        else {
            return Ok(());
        };

        if context.has_error() {
            return Ok(());
        }

        let mut ids = Vec::new();
        let mut result = HashMap::new();
        for (req, tokens) in &reqs {
            let mut traces = TraceSource::default();
            traces.find_defines_in_tokens(&req.input, tokens, &mut ids)?;
            result.insert(req.input.clone(), traces);
        }

        for (req, tokens) in &reqs {
            let Some(traces) = result.get_mut(&req.input) else {
                continue;
            };

            context.consume_err(traces.find_consumes_in_tokens(&req.input, tokens, &mut ids));
        }

        drop(reqs);

        for source in sources {
            let mut traces = TraceSource::default();
            context.consume_err(traces.find_consumes_in_source(&source.input, &ids));

            result.insert(source.input.clone(), traces);
        }

        if context.has_error() {
            return Ok(());
        }
        context.consume_err(
            JsonFormatter::serialize_into(
                &mut stdout(),
                &result
                    .iter()
                    .filter(|(_, v)| !v.consumes.is_empty() || !v.defines.is_empty())
                    .collect::<HashMap<_, _>>(),
            )
            .err_localized(&Location::StdIo),
        );
        Ok(())
    }
}
