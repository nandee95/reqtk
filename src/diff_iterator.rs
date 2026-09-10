use crate::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Diff {
    pub from: Spanned<String>,
    pub to: Spanned<String>,
}

impl Diff {
    pub fn new(from: Spanned<String>, to: Spanned<String>) -> Self {
        Self { from, to }
    }
}

/// Iterates through two text files line by line, yielding `Diff` objects for changed line sequences.
///
/// The algorithm works as follows:
/// 1. Skip all matching lines at the current positions
/// 2. When a mismatch is found, advance through `from` one line at a time
/// 3. For each candidate line from `from`, scan all remaining lines in `to`
/// 4. When the first matching line pair is found (anchor), stop
/// 5. Collect all lines before the anchor as the diff section
/// 6. Yield a `Diff` with the collected sequences and advance past the anchor
pub struct DiffIterator {
    from_lines: Vec<Spanned<String>>,
    to_lines: Vec<Spanned<String>>,
    from_idx: usize,
    to_idx: usize,
}

impl DiffIterator {
    /// Creates a new `DiffIterator` from two text strings.
    pub fn new(from: &str, to: &str) -> Self {
        Self {
            from_lines: Self::split_lines(from),
            to_lines: Self::split_lines(to),
            from_idx: 0,
            to_idx: 0,
        }
    }

    /// Splits a string into lines with proper span tracking.
    fn split_lines(text: &str) -> Vec<Spanned<String>> {
        let mut lines = Vec::new();
        let mut cursor = Cursor::start();

        for line in text.lines() {
            let line_start = cursor;
            let line_with_newline = format!("{}\n", line);
            let line_end = cursor.advance(&line_with_newline);

            lines.push(
                line.to_string()
                    .into_spanned(Span::new(line_start, cursor.advance(line))),
            );

            cursor = line_end;
        }

        // Handle case where file doesn't end with newline
        if !text.is_empty() && !text.ends_with('\n') && !lines.is_empty() {
            let last_idx = lines.len() - 1;
            let line_start = lines[last_idx].span.start;
            lines[last_idx].span = Span::new(line_start, cursor);
        }

        lines
    }

    /// Merges multiple spanned strings into a single spanned string.
    fn merge_lines(lines: &[Spanned<String>]) -> Option<Spanned<String>> {
        if lines.is_empty() {
            return None;
        }

        let merged = lines
            .iter()
            .map(|line| line.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        let span = Span::new(lines[0].span.start, lines[lines.len() - 1].span.end);

        Some(merged.into_spanned(span))
    }
}

impl Iterator for DiffIterator {
    type Item = Diff;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip matching lines
        while self.from_idx < self.from_lines.len()
            && self.to_idx < self.to_lines.len()
            && self.from_lines[self.from_idx].as_str() == self.to_lines[self.to_idx].as_str()
        {
            self.from_idx += 1;
            self.to_idx += 1;
        }

        // Check if we've reached the end of both files
        if self.from_idx >= self.from_lines.len() && self.to_idx >= self.to_lines.len() {
            return None;
        }

        // If only from is left, it was all removed
        if self.from_idx >= self.from_lines.len() {
            let to_diff = &self.to_lines[self.to_idx..];
            self.to_idx = self.to_lines.len();
            let to_spanned = Self::merge_lines(to_diff).unwrap();
            let from_spanned =
                String::new().into_spanned(Span::new(Cursor::start(), Cursor::start()));
            return Some(Diff::new(from_spanned, to_spanned));
        }

        // If only to is left, it was all added
        if self.to_idx >= self.to_lines.len() {
            let from_diff = &self.from_lines[self.from_idx..];
            self.from_idx = self.from_lines.len();
            let from_spanned = Self::merge_lines(from_diff).unwrap();
            let to_spanned =
                String::new().into_spanned(Span::new(Cursor::start(), Cursor::start()));
            return Some(Diff::new(from_spanned, to_spanned));
        }

        // Find the closest matching line pair (anchor) that requires the fewest total iterations.
        // This minimizes the diff chunk size by preferring nearby matches.
        let mut best_anchor = None;
        let mut best_cost = usize::MAX;

        for from_end in self.from_idx..self.from_lines.len() {
            for to_end in self.to_idx..self.to_lines.len() {
                if self.from_lines[from_end].as_str() == self.to_lines[to_end].as_str() {
                    let cost = (from_end - self.from_idx) + (to_end - self.to_idx);
                    if cost < best_cost {
                        best_cost = cost;
                        best_anchor = Some((from_end, to_end));
                    }
                }
            }
        }

        let (from_end, to_end) =
            best_anchor.unwrap_or((self.from_lines.len(), self.to_lines.len()));
        let from_diff = &self.from_lines[self.from_idx..from_end];
        let to_diff = &self.to_lines[self.to_idx..to_end];

        self.from_idx = from_end;
        self.to_idx = to_end;

        // Merge the diff sections
        let from_spanned = if from_diff.is_empty() {
            String::new().into_spanned(Span::new(Cursor::start(), Cursor::start()))
        } else {
            Self::merge_lines(from_diff).unwrap()
        };

        let to_spanned = if to_diff.is_empty() {
            String::new().into_spanned(Span::new(Cursor::start(), Cursor::start()))
        } else {
            Self::merge_lines(to_diff).unwrap()
        };

        Some(Diff::new(from_spanned, to_spanned))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_diff() {
        let from = "line1\nline2\nline3";
        let to = "line1\nchanged\nline3";

        let diffs: Vec<_> = DiffIterator::new(from, to).collect();

        assert_eq!(diffs.len(), 1);
        assert_eq!(*diffs[0].from, "line2");
        assert_eq!(*diffs[0].to, "changed");
    }

    #[test]
    fn test_multiple_line_diff() {
        let from = "a\nb\nc\nd";
        let to = "a\nx\ny\nz\nd";

        let diffs: Vec<_> = DiffIterator::new(from, to).collect();

        assert_eq!(diffs.len(), 1);
        assert_eq!(*diffs[0].from, "b\nc");
        assert_eq!(*diffs[0].to, "x\ny\nz");
    }

    #[test]
    fn test_no_diff() {
        let from = "line1\nline2\nline3";
        let to = "line1\nline2\nline3";

        let diffs: Vec<_> = DiffIterator::new(from, to).collect();

        assert!(diffs.is_empty());
    }

    #[test]
    fn test_added_lines() {
        let from = "a\nd";
        let to = "a\nb\nc\nd";

        let diffs: Vec<_> = DiffIterator::new(from, to).collect();

        assert_eq!(diffs.len(), 1);
        assert_eq!(*diffs[0].from, "");
        assert_eq!(*diffs[0].to, "b\nc");
    }

    #[test]
    fn test_removed_lines() {
        let from = "a\nb\nc\nd";
        let to = "a\nd";

        let diffs: Vec<_> = DiffIterator::new(from, to).collect();

        assert_eq!(diffs.len(), 1);
        assert_eq!(*diffs[0].from, "b\nc");
        assert_eq!(*diffs[0].to, "");
    }

    #[test]
    fn test_multiple_diffs() {
        let from = "a\nb\nd\ne";
        let to = "a\nx\nd\ny";

        let diffs: Vec<_> = DiffIterator::new(from, to).collect();

        assert_eq!(diffs.len(), 2);
        assert_eq!(*diffs[0].from, "b");
        assert_eq!(*diffs[0].to, "x");
        assert_eq!(*diffs[1].from, "e");
        assert_eq!(*diffs[1].to, "y");
    }

    #[test]
    fn test_prefers_closest_anchor() {
        // This test checks that the algorithm finds the closest matching anchor,
        // not just any matching anchor far away.
        // from: "header\nchanged_from\nfooter"
        // to:   "header\nchanged_to\nfooter"
        // Should produce 1 diff: from="changed_from" to="changed_to"
        // NOT a large diff containing everything after the mismatch.
        let from = "header\nchanged_from\nfooter";
        let to = "header\nchanged_to\nfooter";

        let diffs: Vec<_> = DiffIterator::new(from, to).collect();

        assert_eq!(diffs.len(), 1);
        assert_eq!(*diffs[0].from, "changed_from");
        assert_eq!(*diffs[0].to, "changed_to");
    }

    #[test]
    fn test_prefers_closest_when_duplicates_exist() {
        // When there are duplicate lines later in the file, the algorithm
        // should still prefer the closest matching anchor.
        // from: "a\nb\nc\nd\nd\nd"
        // to:   "a\nx\nc\nd\nd\nd"
        // Should produce 1 diff: from="b" to="x"
        // NOT a diff that includes everything up to a far duplicate "d"
        let from = "a\nb\nc\nd\nd\nd";
        let to = "a\nx\nc\nd\nd\nd";

        let diffs: Vec<_> = DiffIterator::new(from, to).collect();

        assert_eq!(diffs.len(), 1);
        assert_eq!(*diffs[0].from, "b");
        assert_eq!(*diffs[0].to, "x");
    }

    #[test]
    fn test_indentation_change_with_nearby_anchor() {
        // Simulates the requirements.req case where indentation changes
        // but there's a matching line nearby that should be the anchor.
        let from = "	line1\n	line2\n	anchor";
        let to = "		line1\n		line2\n	anchor";

        let diffs: Vec<_> = DiffIterator::new(from, to).collect();

        // The diff should be small, containing only the changed indentation
        assert_eq!(diffs.len(), 1);
        assert_eq!(*diffs[0].from, "	line1\n	line2");
        assert_eq!(*diffs[0].to, "		line1\n		line2");
    }
}
