//! The panic scan of FR-014-AC-39, FR-018-AC-17 and FR-018-AC-19, shared by the generator-source
//! scans and the emitted-source scans so that one list defines what "panic-free" means.
//!
//! The scan is lexical, over identifiers, not over spellings: `.unwrap(`, `.unwrap ()`,
//! `Option::unwrap`, `use Option::unwrap` and a method split across lines all name the identifier
//! `unwrap`. `non_test_code` removes comments and `#[cfg(test)]` items by structure (the
//! attribute at the start of a line, then the item it governs, found by matching braces outside
//! string and character literals), wherever they sit in the file.

/// The identifiers the scan bans wherever they appear: the panicking `Option` and `Result`
/// methods, called, named on a path or imported. `unwrap_or`, `unwrap_or_else` and `expect_err`
/// are different identifiers and are not banned.
const PANIC_METHODS: [&str; 3] = ["unwrap", "expect", "unwrap_unchecked"];

/// The macros the scan bans, in any delimiter form and with any path prefix: the identifier,
/// optional whitespace, then `!` (not `!=`).
const PANIC_MACROS: [&str; 10] = [
    "panic",
    "unreachable",
    "todo",
    "unimplemented",
    "assert",
    "assert_eq",
    "assert_ne",
    "debug_assert",
    "debug_assert_eq",
    "debug_assert_ne",
];

/// The identifier `abort` (`process::abort`, or a bare `abort` after an import) is banned unless
/// it is a method (`.abort`).
const ABORT: &str = "abort";

fn is_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// The identifiers of `text` with their byte ranges. Text inside comments and string literals is
/// scanned too: a word a generator emits counts.
fn identifiers(text: &str) -> Vec<(usize, usize)> {
    let bytes = text.as_bytes();
    let mut found = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        if bytes[at].is_ascii_alphabetic() || bytes[at] == b'_' {
            let start = at;
            while at < bytes.len() && is_identifier_byte(bytes[at]) {
                at += 1;
            }
            found.push((start, at));
        } else {
            at += 1;
        }
    }
    found
}

/// The first byte at or after `from` that is not ASCII whitespace.
fn next_non_whitespace(bytes: &[u8], from: usize) -> Option<(usize, u8)> {
    bytes
        .iter()
        .enumerate()
        .skip(from)
        .find(|(_, byte)| !byte.is_ascii_whitespace())
        .map(|(at, byte)| (at, *byte))
}

/// The last byte before `before` that is not ASCII whitespace.
fn previous_non_whitespace(bytes: &[u8], before: usize) -> Option<u8> {
    bytes
        .iter()
        .take(before)
        .rev()
        .find(|byte| !byte.is_ascii_whitespace())
        .copied()
}

/// Every banned token `text` contains, each named once, sorted: a banned identifier by its
/// name, a macro as `name!`.
pub fn panic_tokens_in(text: &str) -> Vec<String> {
    let bytes = text.as_bytes();
    let mut found = std::collections::BTreeSet::new();
    for (start, end) in identifiers(text) {
        let Some(name) = text.get(start..end) else {
            continue;
        };
        if PANIC_METHODS.contains(&name) {
            found.insert(name.to_owned());
        }
        if PANIC_MACROS.contains(&name) {
            let bang = next_non_whitespace(bytes, end)
                .is_some_and(|(at, byte)| byte == b'!' && bytes.get(at + 1) != Some(&b'='));
            if bang {
                found.insert(format!("{name}!"));
            }
        }
        if name == ABORT && previous_non_whitespace(bytes, start) != Some(b'.') {
            found.insert(ABORT.to_owned());
        }
    }
    found.into_iter().collect()
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Code,
    Literal,
    Comment,
}

/// The end of a (non-raw) string literal whose body starts at `from`.
fn string_end(bytes: &[u8], from: usize) -> usize {
    let mut at = from;
    while at < bytes.len() {
        match bytes[at] {
            b'\\' => at += 2,
            b'"' => return at + 1,
            _ => at += 1,
        }
    }
    bytes.len()
}

/// Classify every byte of `text` as code, string or character literal, or comment.
fn classify(text: &str) -> Vec<Kind> {
    let bytes = text.as_bytes();
    let mut kinds = vec![Kind::Code; bytes.len()];
    let mut at = 0;
    while at < bytes.len() {
        let end = match bytes[at] {
            b'/' if bytes.get(at + 1) == Some(&b'/') => {
                let end = bytes[at..]
                    .iter()
                    .position(|byte| *byte == b'\n')
                    .map_or(bytes.len(), |line| at + line);
                Some((end, Kind::Comment))
            }
            b'/' if bytes.get(at + 1) == Some(&b'*') => {
                let mut depth = 0usize;
                let mut end = bytes.len();
                let mut cursor = at;
                while cursor < bytes.len() {
                    if bytes[cursor..].starts_with(b"/*") {
                        depth += 1;
                        cursor += 2;
                    } else if bytes[cursor..].starts_with(b"*/") {
                        depth -= 1;
                        cursor += 2;
                        if depth == 0 {
                            end = cursor;
                            break;
                        }
                    } else {
                        cursor += 1;
                    }
                }
                Some((end, Kind::Comment))
            }
            b'"' => Some((string_end(bytes, at + 1), Kind::Literal)),
            b'r' if at == 0 || !is_identifier_byte(bytes[at - 1]) || bytes[at - 1] == b'b' => {
                let hashes = bytes[at + 1..]
                    .iter()
                    .take_while(|byte| **byte == b'#')
                    .count();
                if bytes.get(at + 1 + hashes) == Some(&b'"') {
                    let closing: Vec<u8> = std::iter::once(b'"')
                        .chain(std::iter::repeat_n(b'#', hashes))
                        .collect();
                    let body = at + 2 + hashes;
                    let end = bytes[body..]
                        .windows(closing.len())
                        .position(|window| window == closing.as_slice())
                        .map_or(bytes.len(), |close| body + close + closing.len());
                    Some((end, Kind::Literal))
                } else {
                    None
                }
            }
            b'\'' => {
                if bytes.get(at + 1) == Some(&b'\\') {
                    let end = bytes[at + 2..]
                        .iter()
                        .position(|byte| *byte == b'\'')
                        .map_or(bytes.len(), |close| at + 2 + close + 1);
                    Some((end, Kind::Literal))
                } else {
                    // A character literal is one character then `'`; anything else is a lifetime.
                    text.get(at + 1..)
                        .and_then(|rest| rest.chars().next())
                        .filter(|character| {
                            bytes.get(at + 1 + character.len_utf8()) == Some(&b'\'')
                        })
                        .map(|character| (at + 2 + character.len_utf8(), Kind::Literal))
                }
            }
            _ => None,
        };
        match end {
            Some((end, kind)) => {
                for slot in kinds.iter_mut().take(end).skip(at) {
                    *slot = kind;
                }
                at = end.max(at + 1);
            }
            None => at += 1,
        }
    }
    kinds
}

/// The end of the item that starts at `from`: after its matching `}`, or after the `;` that ends
/// it first. Braces and semicolons in literals and comments do not count.
fn item_end(bytes: &[u8], kinds: &[Kind], from: usize) -> usize {
    let mut depth = 0usize;
    for at in from..bytes.len() {
        if kinds[at] != Kind::Code {
            continue;
        }
        match bytes[at] {
            b'{' => depth += 1,
            b'}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return at + 1;
                }
            }
            b';' if depth == 0 => return at + 1,
            _ => {}
        }
    }
    bytes.len()
}

/// The non-test code of a Rust source file: comments removed, and every item an attribute
/// `#[cfg(test)]` governs removed, wherever it sits. The attribute is recognised at the start of
/// a line, in code; a mention of it in a comment or a literal removes nothing. String literals
/// are kept: what a generator emits counts.
pub fn non_test_code(source: &str) -> String {
    let kinds = classify(source);
    let bytes = source.as_bytes();
    let mut removed = vec![false; bytes.len()];
    let mut line_start = 0;
    for line in source.split_inclusive('\n') {
        let indent = line.len() - line.trim_start().len();
        let attribute = line_start + indent;
        if line.trim_start().starts_with("#[cfg(test)]")
            && kinds.get(attribute) == Some(&Kind::Code)
        {
            let end = item_end(bytes, &kinds, attribute + "#[cfg(test)]".len());
            for slot in removed.iter_mut().take(end).skip(attribute) {
                *slot = true;
            }
        }
        line_start += line.len();
    }
    let kept: Vec<u8> = bytes
        .iter()
        .zip(kinds.iter().zip(&removed))
        .filter(|(_, (kind, removed))| **kind != Kind::Comment && !**removed)
        .map(|(byte, _)| *byte)
        .collect();
    String::from_utf8_lossy(&kept).into_owned()
}
