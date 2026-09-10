use crate::*;
use clap::Args;
use std::{
    collections::HashMap,
    io::{BufReader, Read, Seek, SeekFrom, Write},
    path::PathBuf,
};
#[derive(Args, Debug)]
pub struct TransformCommand {
    /// Input locations. When missing the inputs are taken from nearest 'reqtk.json'.
    /// Use a single '-' for stdin.
    // {@REQTK-21}
    inputs: Option<Vec<PathBuf>>,

    /// Output location for single input.
    /// Use a '-' for stdout.
    // {@REQTK-22}
    #[arg(short, long)]
    output: Option<Output>,

    /// Re-apply ids for the file. [possible values: incremental|incremental-N] (N=length)
    // {@REQTK-12}
    #[arg(long)]
    id: Option<IdType>,

    /// Re-apply a prefix to the file.
    // {@REQTK-13}
    #[arg(long)]
    prefix: Option<String>,
}

impl Command for TransformCommand {
    fn execute(&self, mut context: CommandContext) {
        let targets = context.target(
            self.inputs.as_ref().unwrap_or(&Vec::new()),
            &self.output,
            None,
            ReqTkTargets::Requirements | ReqTkTargets::Sources,
        );

        if !self.id.is_some() && !self.prefix.is_some() {
            context.push_issue(Issue::new(
                IssueSeverity::Warning,
                "No transformation specified".into(),
            ));
            return;
        }

        let Some(targets) = context.consume_err(targets) else {
            return;
        };

        let (reqs, sources): (Vec<_>, Vec<_>) = targets.iter().partition(|a| match &a.input {
            Location::StdIo => true,
            Location::Path(path) => {
                path.extension().and_then(|ext| ext.to_str()).unwrap_or("") == "req"
            }
        });

        if reqs.len() != 1 && self.prefix.is_some() {
            context.push_issue(Issue::new(
                IssueSeverity::Error,
                "Prefix can only be used with a single requirement input".into(),
            ));
            return;
        }

        let mut trees = reqs
            .into_iter()
            .filter_map(|target| {
                context
                    .consume_err(ReqImpex.import(&target.input))
                    .map(|v| (target, v))
            })
            .collect::<Vec<_>>();

        if context.has_error() {
            return;
        }

        let mut map = HashMap::new();
        for (_, tree) in &mut trees {
            map.extend(tree.re_id(self.id.clone(), self.prefix.clone()));
        }
        for (_, tree) in &mut trees {
            tree.update_refs(&map);
        }

        for (target, tree) in &trees {
            let Some(LocationIo::File(mut writer)) = context.consume_err(target.output.writer())
            else {
                continue;
            };
            context.consume_err(TokenWriter::write_reformat(
                &mut writer,
                &TreeTokenizer::tokenize(tree),
            ));
            let pos = writer.stream_position().unwrap();
            writer.set_len(pos).unwrap();
        }

        for source in sources {
            let Some(mut writer) = context.consume_err(source.input.writer()) else {
                continue;
            };

            let Some(extension) = source.input.extension() else {
                continue;
            };

            let mut bufread = BufReader::new(&mut writer);
            let Some(comments) = CommentIterator::from_extension(&mut bufread, &extension) else {
                context.push_issue(Issue::new(
                    IssueSeverity::Warning,
                    format!("Source extension is not supported: {}", extension),
                ));
                continue;
            };

            let mut replacements = Vec::new();
            for comment in comments {
                let Some(comment) = context.consume_err(comment) else {
                    continue;
                };

                for from in ReferenceIterator::new(&comment, comment.span) {
                    let Some((_, to)) = map.iter().find(|(k, _)| k == &*from) else {
                        continue;
                    };

                    replacements.push((from.span.to_range(), to));
                }
            }
            if !replacements.is_empty() {
                replace_ranges_in_writer(&mut writer, &replacements).unwrap();
            }
        }
    }
}

fn replace_ranges_in_writer(
    io: &mut LocationIo,
    replacements: &[(std::ops::Range<usize>, &String)],
) -> std::io::Result<()> {
    assert!(replacements.iter().all(|v| v.0.start <= v.0.end));
    assert!(replacements.windows(2).all(|w| w[0].0.end <= w[1].0.start));

    let mut diff: isize = 0;
    let adjusted = replacements
        .iter()
        .map(|(range, to)| {
            let adjusted_range =
                ((range.start as isize + diff) as usize)..((range.end as isize + diff) as usize);
            diff += to.len() as isize - range.len() as isize;
            (range, adjusted_range, *to)
        })
        .collect::<Vec<_>>();

    let old_len = io.seek(SeekFrom::End(0))? as isize;
    let new_len = old_len + diff;

    if new_len < 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Replacement resulted in negative file length",
        ));
    }

    // Read every gap (untouched region following each range) into memory
    // BEFORE writing anything. A write for one replacement can land on an
    // offset that overlaps the *old* location of a gap another replacement
    // still needs to read; reading everything up front makes the order of
    // writes irrelevant.
    let mut gaps = Vec::with_capacity(adjusted.len());
    let mut old_last = old_len as usize;
    for (range, _, _) in adjusted.iter().rev() {
        io.seek(SeekFrom::Start(range.end as u64))?;
        let mut buffer = vec![0u8; old_last - range.end];
        io.read_exact(&mut buffer)?;
        gaps.push(buffer);
        old_last = range.start;
    }

    // Grow before writing (so writes past the old EOF have somewhere to land).
    if new_len > old_len {
        io.set_len(new_len as u64)?;
    }

    for ((_, adjusted_range, to), buffer) in adjusted.iter().rev().zip(gaps.iter()) {
        io.seek(SeekFrom::Start(adjusted_range.start as u64))?;
        io.write_all(to.as_bytes())?;
        io.write_all(buffer)?;
    }

    // Shrink only after every surviving byte has been relocated.
    if new_len < old_len {
        io.set_len(new_len as u64)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Seek, SeekFrom, Write};

    // Adjust this if `LocationIo` isn't `std::fs::File` itself but a thin
    // wrapper around it (e.g. change to `LocationIo::new(file)`).
    fn make_io(content: &[u8]) -> LocationIo {
        let mut file = tempfile::tempfile().expect("failed to create temp file");
        file.write_all(content).expect("failed to write content");
        file.rewind().expect("failed to rewind");
        LocationIo::File(file)
    }

    fn read_all(io: &mut LocationIo) -> String {
        io.seek(SeekFrom::Start(0)).unwrap();
        let mut buf = String::new();
        io.read_to_string(&mut buf).unwrap();
        buf
    }

    #[test]
    fn no_replacements_is_noop() {
        let mut io = make_io(b"Hello, world!");
        replace_ranges_in_writer(&mut io, &[]).unwrap();
        assert_eq!(read_all(&mut io), "Hello, world!");
    }

    #[test]
    fn same_length_replacement() {
        let mut io = make_io(b"Hello, world!");
        let to = "Earth".to_string();
        replace_ranges_in_writer(&mut io, &[(7..12, &to)]).unwrap();
        assert_eq!(read_all(&mut io), "Hello, Earth!");
    }

    #[test]
    fn longer_replacement_grows_file() {
        let mut io = make_io(b"Hello, world!");
        let to = "Universe".to_string();
        replace_ranges_in_writer(&mut io, &[(7..12, &to)]).unwrap();
        assert_eq!(read_all(&mut io), "Hello, Universe!");
    }

    #[test]
    fn shorter_replacement_shrinks_file() {
        let mut io = make_io(b"Hello, world!");
        let to = "Rs".to_string();
        replace_ranges_in_writer(&mut io, &[(7..12, &to)]).unwrap();
        assert_eq!(read_all(&mut io), "Hello, Rs!");
    }

    #[test]
    fn replacement_at_start() {
        let mut io = make_io(b"Hello, world!");
        let to = "Goodbye".to_string();
        replace_ranges_in_writer(&mut io, &[(0..5, &to)]).unwrap();
        assert_eq!(read_all(&mut io), "Goodbye, world!");
    }

    #[test]
    fn replacement_at_end() {
        let mut io = make_io(b"Hello, world!");
        let to = "Rust?".to_string();
        let len = "Hello, world!".len();
        replace_ranges_in_writer(&mut io, &[((len - 6)..len, &to)]).unwrap();
        assert_eq!(read_all(&mut io), "Hello, Rust?");
    }

    #[test]
    fn deletion_with_empty_replacement() {
        let mut io = make_io(b"Hello, world!");
        let to = String::new();
        // Removes ", world" (indices 5..12), leaving "Hello!"
        replace_ranges_in_writer(&mut io, &[(5..12, &to)]).unwrap();
        assert_eq!(read_all(&mut io), "Hello!");
    }

    #[test]
    fn insertion_at_empty_range() {
        let mut io = make_io(b"Hello world!");
        let to = ",".to_string();
        // Empty range: pure insertion, no deletion
        replace_ranges_in_writer(&mut io, &[(5..5, &to)]).unwrap();
        assert_eq!(read_all(&mut io), "Hello, world!");
    }

    #[test]
    fn multiple_non_overlapping_replacements_mixed_lengths() {
        let mut io = make_io(b"aaa BBB ccc DDD eee");
        let shrink = "X".to_string();
        let grow = "YYYYYY".to_string();
        // "BBB" is at 4..7, "DDD" is at 12..15
        replace_ranges_in_writer(&mut io, &[(4..7, &shrink), (12..15, &grow)]).unwrap();
        assert_eq!(read_all(&mut io), "aaa X ccc YYYYYY eee");
    }

    #[test]
    fn multiple_replacements_all_shrinking() {
        let mut io = make_io(b"one two three four five");
        let a = "2".to_string();
        let b = "4".to_string();
        // "two" is at 4..7, "four" is at 14..18
        replace_ranges_in_writer(&mut io, &[(4..7, &a), (14..18, &b)]).unwrap();
        assert_eq!(read_all(&mut io), "one 2 three 4 five");
    }

    #[test]
    #[should_panic]
    fn panics_on_inverted_range() {
        let mut io = make_io(b"Hello, world!");
        let to = "x".to_string();
        #[allow(clippy::reversed_empty_ranges)]
        let bad_range = 5..3;
        replace_ranges_in_writer(&mut io, &[(bad_range, &to)]).unwrap();
    }

    #[test]
    #[should_panic]
    fn panics_on_overlapping_ranges() {
        let mut io = make_io(b"Hello, world!");
        let a = "x".to_string();
        let b = "y".to_string();
        replace_ranges_in_writer(&mut io, &[(0..5, &a), (3..8, &b)]).unwrap();
    }

    #[test]
    #[should_panic]
    fn panics_on_unsorted_ranges() {
        let mut io = make_io(b"Hello, world!");
        let a = "x".to_string();
        let b = "y".to_string();
        replace_ranges_in_writer(&mut io, &[(7..12, &a), (0..5, &b)]).unwrap();
    }
}
