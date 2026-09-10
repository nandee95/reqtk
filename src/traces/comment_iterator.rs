use crate::*;
use peekmore::{PeekMore, PeekMoreIterator};
use std::io::{self, BufRead};
use utf8_chars::{BufReadCharsExt, Chars};

pub struct CommentIterator<'r> {
    iter: PeekMoreIterator<Chars<'r, dyn BufRead + 'r>>,
    cursor: Cursor,
    trackers: &'static [[&'static str; 2]],
    excluded: &'static [[&'static str; 2]],
    escape: Option<char>,
    active: Option<usize>,
    excluded_active: Option<usize>,
    escaped: bool,
    text: String,
    start: Cursor,
}

impl<'r> CommentIterator<'r> {
    pub fn from_extension(reader: &'r mut dyn BufRead, extension: &str) -> Option<Self> {
        match extension {
            // C-style: // line comments, /* */ block comments, "..." strings
            "rs" | "js" | "ts" | "jsx" | "tsx" | "c" | "h" | "cpp" | "cc" | "cxx" | "hpp"
            | "java" | "go" | "swift" | "kt" | "kts" | "cs" | "scala" | "dart" | "php" | "css"
            | "scss" | "less" | "proto" | "glsl" | "hlsl" => Some(Self::new(
                reader,
                &[["//", "\n"], ["/*", "*/"]],
                &[["\"", "\""]],
                Some('\\'),
            )),

            // Python: # line comments, triple/single/double-quoted strings
            "py" | "pyi" => Some(Self::new(
                reader,
                &[["#", "\n"]],
                &[
                    ["\"\"\"", "\"\"\""],
                    ["'''", "'''"],
                    ["\"", "\""],
                    ["'", "'"],
                ],
                Some('\\'),
            )),

            // Shell-family: # line comments, single- and double-quoted strings.
            // (Single-quoted strings in sh/bash don't actually support escaping,
            // but sharing the escape char here is a harmless simplification.)
            "sh" | "bash" | "zsh" | "fish" | "rb" | "pl" | "pm" | "yaml" | "yml" | "toml"
            | "ini" | "cfg" | "r" | "R" | "make" | "mk" | "dockerfile" | "nim" | "elixir"
            | "ex" | "exs" => Some(Self::new(
                reader,
                &[["#", "\n"]],
                &[["\"", "\""], ["'", "'"]],
                Some('\\'),
            )),

            // HTML/XML: only block comments, no string-exclusion needed for
            // comment detection since <!-- --> can't legally nest inside tags
            // in a way that matters here.
            "html" | "htm" | "xml" | "svg" | "xhtml" | "vue" => {
                Some(Self::new(reader, &[["<!--", "-->"]], &[], None))
            }

            // SQL: -- line comments, /* */ block comments, '...' strings
            // (SQL strings use '' to escape a literal quote, not backslash,
            // so no escape char is passed here — see note below).
            "sql" => Some(Self::new(
                reader,
                &[["--", "\n"], ["/*", "*/"]],
                &[["'", "'"]],
                None,
            )),

            // Lua: -- line comments, --[[ ]] block comments, "..."/'...' strings
            "lua" => Some(Self::new(
                reader,
                &[["--[[", "]]"], ["--", "\n"]],
                &[["\"", "\""], ["'", "'"]],
                Some('\\'),
            )),

            // Haskell: -- line comments, {- -} block comments, "..." strings
            "hs" | "lhs" => Some(Self::new(
                reader,
                &[["--", "\n"], ["{-", "-}"]],
                &[["\"", "\""]],
                Some('\\'),
            )),

            _ => None,
        }
    }

    pub fn new(
        reader: &'r mut dyn BufRead,
        trackers: &'static [[&'static str; 2]],
        excluded: &'static [[&'static str; 2]],
        escape: Option<char>,
    ) -> Self {
        Self {
            iter: reader.chars().peekmore(),
            cursor: Cursor::start(),
            trackers,
            excluded,
            escape,
            active: None,
            excluded_active: None,
            escaped: false,
            text: String::new(),
            start: Cursor::start(),
        }
    }

    fn try_match(&mut self, first: char, pattern: &str) -> bool {
        let mut chars = pattern.chars();
        match chars.next() {
            Some(c) if c == first => {}
            _ => return false,
        }
        for (i, c) in chars.clone().enumerate() {
            if !matches!(self.iter.peek_nth(i), Some(&Ok::<char, io::Error>(c2)) if c2 == c) {
                return false;
            }
        }
        for _ in chars {
            if let Some(Ok(c)) = self.iter.next() {
                self.cursor = self.cursor.advance_ch(c);
            }
        }
        true
    }
}

impl<'r> Iterator for CommentIterator<'r> {
    type Item = Result<Spanned<String>, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ch) = self.iter.next() {
            let ch = match ch {
                Ok(c) => c,
                Err(e) => return Some(Err(e)),
            };
            if self.escaped {
                self.escaped = false;
                if self.active.is_some() {
                    self.text.push(ch);
                }
                self.cursor = self.cursor.advance_ch(ch);
                continue;
            }

            if let Some(esc) = self.escape
                && ch == esc
            {
                self.escaped = true;
                if self.active.is_some() {
                    self.text.push(ch);
                }
                self.cursor = self.cursor.advance_ch(ch);
                continue;
            }

            match self.active {
                None => {
                    if let Some(i) = self.excluded_active {
                        if self.try_match(ch, self.excluded[i][1]) {
                            self.excluded_active = None;
                        }
                        self.cursor = self.cursor.advance_ch(ch);
                        continue;
                    }

                    let mut matched_excluded = None;
                    for (i, pair) in self.excluded.iter().enumerate() {
                        if self.try_match(ch, pair[0]) {
                            matched_excluded = Some(i);
                            break;
                        }
                    }
                    if let Some(i) = matched_excluded {
                        self.excluded_active = Some(i);
                        self.cursor = self.cursor.advance_ch(ch);
                        continue;
                    }

                    let mut matched = None;
                    for (i, tracker) in self.trackers.iter().enumerate() {
                        if self.try_match(ch, tracker[0]) {
                            matched = Some(i);
                            break;
                        }
                    }
                    self.cursor = self.cursor.advance_ch(ch);
                    if let Some(i) = matched {
                        self.active = Some(i);
                        // start *after* the opening marker has been fully consumed
                        self.start = self.cursor;
                    }
                }
                Some(i) => {
                    // cursor position *before* attempting to match the closing marker
                    let before_closing = self.cursor;
                    if self.try_match(ch, self.trackers[i][1]) {
                        self.cursor = self.cursor.advance_ch(ch);
                        self.active = None;
                        let text = std::mem::take(&mut self.text);
                        let span = Span::new(self.start, before_closing);
                        return Some(Ok(text.into_spanned(span)));
                    } else {
                        self.text.push(ch);
                        self.cursor = self.cursor.advance_ch(ch);
                    }
                }
            }
        }
        None
    }
}
