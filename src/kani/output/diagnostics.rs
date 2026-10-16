//! The bounded diagnostic record of a run Kani gave no verdict for (FR-017).
//!
//! The record is read from the launcher's two streams alone. It names the last stage the output
//! shows a member started and carries a redacted tail of each stream. It never feeds
//! classification: nothing in this module is consulted until the outcome is already decided.
//!
//! The record is serialize-only. The Kani output this module reads is untrusted text, so no
//! decoder for the record exists.

use std::path::Path;

use serde::Serialize;

/// The most bytes one stream's excerpt keeps.
const EXCERPT_LIMIT: usize = 8192;

/// The last stage of a run that Kani's output shows a member started, or the stage that failed
/// for `compile` and `launcher`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniFailurePhase {
    /// The launcher ended before Kani printed its banner, as on an argument error.
    Launcher,
    /// The crate did not build.
    Compile,
    /// Kani printed the member's `Checking harness` line and reached its per-harness pipeline.
    Backend,
    /// Kani handed the member's problem to its SAT solver.
    Solver,
    /// The output names no stage, or only a stage of another member.
    Unknown,
}

/// The tail of one launcher stream after redaction.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KaniStreamExcerpt {
    /// The longest redacted tail within 8192 bytes, kept on a line start unless the stream's
    /// final line alone is longer.
    pub text: String,
    /// The bytes of redacted text before `text`; zero when `text` is the whole redacted stream.
    pub omitted_bytes: u64,
}

/// Why a `NoVerdict` run has no verdict: its failure phase and the tails of its two streams.
///
/// The record serializes as `{ phase, stdout { text, omittedBytes }, stderr { text,
/// omittedBytes } }` and is never read back, so it implements no deserialization:
///
/// ```compile_fail
/// let _: quire_contract_codegen::KaniNoVerdictDiagnostics = serde_json::from_str("{}").unwrap();
/// ```
///
/// Serializing the same value compiles:
///
/// ```
/// fn serializes(diagnostics: &quire_contract_codegen::KaniNoVerdictDiagnostics) -> String {
///     serde_json::to_string(diagnostics).unwrap()
/// }
/// # let _ = serializes;
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KaniNoVerdictDiagnostics {
    /// The member's failure phase.
    pub phase: KaniFailurePhase,
    /// The redacted tail of the launcher's stdout.
    pub stdout: KaniStreamExcerpt,
    /// The redacted tail of the launcher's stderr.
    pub stderr: KaniStreamExcerpt,
}

/// The directories a run's output is redacted against. The process environment is never read.
#[derive(Clone, Copy, Debug)]
pub(crate) struct KnownDirectories<'a> {
    /// The crate directory, redacted as `<CRATE_DIR>`.
    pub(crate) crate_directory: &'a Path,
    /// The Cargo target directory, redacted as `<TARGET_DIR>`.
    pub(crate) target_directory: &'a Path,
    /// The launcher's home directory, redacted as `<HOME>` when given.
    pub(crate) home_directory: Option<&'a Path>,
}

/// What one launcher process printed and was asked, from which each member's record is taken.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ProcessOutput<'a> {
    /// The captured stdout, whole.
    pub(crate) stdout: &'a [u8],
    /// The captured stderr, whole.
    pub(crate) stderr: &'a [u8],
    /// The `module::harness` path of every member the launch passed to `--harness`.
    pub(crate) selections: &'a [String],
    /// The launch's complete argument vector.
    pub(crate) arguments: &'a [String],
    /// Whether the run exported a report.
    pub(crate) report_exported: bool,
    /// The directories to redact.
    pub(crate) directories: KnownDirectories<'a>,
}

impl ProcessOutput<'_> {
    /// The record of the member whose path is `member`.
    pub(crate) fn diagnostics_of(&self, member: &str) -> KaniNoVerdictDiagnostics {
        KaniNoVerdictDiagnostics {
            phase: self.phase_of(member),
            stdout: excerpt(self.stdout, &self.directories),
            stderr: excerpt(self.stderr, &self.directories),
        }
    }

    /// The first of the phase rules that applies to `member`, read from the whole streams.
    fn phase_of(&self, member: &str) -> KaniFailurePhase {
        if self.report_exported {
            return KaniFailurePhase::Unknown;
        }
        let stdout = String::from_utf8_lossy(self.stdout);
        let stderr = String::from_utf8_lossy(self.stderr);
        let segments = Segments::of(&stdout, self.selections);
        let by_segment = if segments.of_member(member).is_some_and(|own| own.solving) {
            Some(KaniFailurePhase::Solver)
        } else if segments.of_member(member).is_some() {
            Some(KaniFailurePhase::Backend)
        } else if segments.any() {
            Some(KaniFailurePhase::Unknown)
        } else {
            None
        };
        if let Some(phase) = by_segment {
            // Harnesses checked in parallel interleave their lines, so no segment is one
            // member's output.
            return if runs_in_parallel(self.arguments) {
                KaniFailurePhase::Unknown
            } else {
                phase
            };
        }
        if has_line_starting(&stdout, "error: Failed to execute cargo (exit status: ")
            || has_line_starting(&stderr, "error: could not compile ")
        {
            KaniFailurePhase::Compile
        } else if !has_line_starting(&stdout, "Kani Rust Verifier ") {
            KaniFailurePhase::Launcher
        } else {
            KaniFailurePhase::Unknown
        }
    }
}

/// Whether `arguments` ask Kani to check harnesses in parallel.
fn runs_in_parallel(arguments: &[String]) -> bool {
    arguments
        .iter()
        .any(|argument| argument == "-j" || argument == "--jobs" || argument.starts_with("--jobs="))
}

fn has_line_starting(text: &str, prefix: &str) -> bool {
    text.lines().any(|line| line.starts_with(prefix))
}

/// What stdout shows of one member's `Checking harness <path>...` segment.
struct Segment<'a> {
    member: &'a str,
    /// Whether the segment holds a line beginning `Solving with `.
    solving: bool,
}

/// The harness segments of a stdout: each starts at a line that is exactly
/// `Checking harness <path>...` for a path the launch passed, and runs to the next such line.
struct Segments<'a>(Vec<Segment<'a>>);

impl<'a> Segments<'a> {
    fn of(stdout: &str, selections: &'a [String]) -> Self {
        let mut segments: Vec<Segment<'a>> = Vec::new();
        for line in stdout.lines() {
            let heading = line
                .strip_prefix("Checking harness ")
                .and_then(|rest| rest.strip_suffix("..."))
                .and_then(|path| selections.iter().find(|selection| *selection == path));
            if let Some(member) = heading {
                segments.push(Segment {
                    member,
                    solving: false,
                });
            } else if line.starts_with("Solving with ") {
                if let Some(open) = segments.last_mut() {
                    open.solving = true;
                }
            }
        }
        Self(segments)
    }

    fn any(&self) -> bool {
        !self.0.is_empty()
    }

    /// The segment of `member`, its `solving` true when any segment headed by it holds a solver
    /// line.
    fn of_member(&self, member: &str) -> Option<Segment<'a>> {
        let own = || self.0.iter().filter(|segment| segment.member == member);
        own().next().map(|first| Segment {
            member: first.member,
            solving: own().any(|segment| segment.solving),
        })
    }
}

/// The redacted tail of `bytes`.
fn excerpt(bytes: &[u8], directories: &KnownDirectories<'_>) -> KaniStreamExcerpt {
    let text = String::from_utf8_lossy(bytes);
    let text = strip_control(&text);
    let text = redact_paths(&text, directories);
    let text = redact_timestamps(&text);
    let text = redact_durations(&text);
    let text = redact_process_ids(&text);
    let start = tail_start(&text);
    KaniStreamExcerpt {
        omitted_bytes: u64::try_from(start).unwrap_or(u64::MAX),
        text: text[start..].to_owned(),
    }
}

/// The byte offset where the kept tail of `text` starts.
fn tail_start(text: &str) -> usize {
    let bytes = text.as_bytes();
    let Some(least) = text.len().checked_sub(EXCERPT_LIMIT).filter(|least| *least > 0) else {
        return 0;
    };
    let content_end = text.trim_end_matches('\n').len();
    let final_line_start = text[..content_end].rfind('\n').map_or(0, |index| index + 1);
    let final_line_fits = content_end > 0 && text.len() - final_line_start <= EXCERPT_LIMIT;
    if content_end > 0 && !final_line_fits {
        return (least..=text.len())
            .find(|offset| text.is_char_boundary(*offset))
            .unwrap_or(text.len());
    }
    if bytes[least - 1] == b'\n' {
        return least;
    }
    bytes[least..]
        .iter()
        .position(|byte| *byte == b'\n')
        .map_or(text.len(), |index| least + index + 1)
}

/// Removes ANSI escape sequences and replaces every other control character except line feed
/// and tab with U+FFFD.
fn strip_control(text: &str) -> String {
    let mut kept = String::with_capacity(text.len());
    let mut characters = text.chars().peekable();
    while let Some(character) = characters.next() {
        match character {
            '\u{1b}' => skip_escape(&mut characters),
            '\n' | '\t' => kept.push(character),
            other if other.is_control() => kept.push('\u{fffd}'),
            other => kept.push(other),
        }
    }
    kept
}

/// Consumes the rest of the escape sequence whose introducer was just read: a CSI sequence to
/// its final byte, an OSC or string sequence to its terminator, or the one character of a short
/// escape.
fn skip_escape(characters: &mut std::iter::Peekable<std::str::Chars<'_>>) {
    match characters.peek().copied() {
        Some('[') => {
            characters.next();
            for character in characters.by_ref() {
                if ('\u{40}'..='\u{7e}').contains(&character) {
                    break;
                }
            }
        }
        Some(']' | 'P' | 'X' | '^' | '_') => {
            characters.next();
            while let Some(character) = characters.next() {
                if character == '\u{7}' {
                    break;
                }
                if character == '\u{1b}' {
                    characters.next_if_eq(&'\\');
                    break;
                }
            }
        }
        Some(character) if ('\u{20}'..='\u{2f}').contains(&character) => {
            while characters.next_if(|next| ('\u{20}'..='\u{2f}').contains(next)).is_some() {}
            characters.next_if(|next| ('\u{30}'..='\u{7e}').contains(next));
        }
        Some(character) if ('\u{30}'..='\u{7e}').contains(&character) => {
            characters.next();
        }
        _ => {}
    }
}

/// The known directories in match order: longest first, then crate, target, home; each without
/// a trailing `/`, and none that is empty, `/` or not absolute.
fn ordered_directories(directories: &KnownDirectories<'_>) -> Vec<(String, &'static str)> {
    let mut known: Vec<(String, &'static str)> = [
        (Some(directories.crate_directory), "<CRATE_DIR>"),
        (Some(directories.target_directory), "<TARGET_DIR>"),
        (directories.home_directory, "<HOME>"),
    ]
    .into_iter()
    .filter_map(|(directory, placeholder)| {
        let directory = directory?.to_string_lossy();
        let trimmed = directory.trim_end_matches('/');
        (directory.starts_with('/') && !trimmed.is_empty())
            .then(|| (trimmed.to_owned(), placeholder))
    })
    .collect();
    known.sort_by_key(|(directory, _)| std::cmp::Reverse(directory.len()));
    known
}

fn opens_path(previous: Option<char>) -> bool {
    previous.is_none_or(|previous| {
        matches!(previous, '\n' | ' ' | '\t' | '(' | '[' | '{' | '\'' | '"' | '`' | '=')
    })
}

fn closes_path(character: char) -> bool {
    matches!(
        character,
        ' ' | '\t' | '\n' | ')' | ']' | '}' | '\'' | '"' | '`' | ',' | ';' | ':'
    )
}

fn redact_path_token(token: &str, known: &[(String, &'static str)]) -> String {
    for (directory, placeholder) in known {
        match token.strip_prefix(directory.as_str()) {
            Some("") => return (*placeholder).to_owned(),
            Some(rest) if rest.starts_with('/') => return format!("{placeholder}{rest}"),
            _ => {}
        }
    }
    if token.matches('/').count() >= 2 {
        "<PATH>".to_owned()
    } else {
        token.to_owned()
    }
}

fn redact_paths(text: &str, directories: &KnownDirectories<'_>) -> String {
    let known = ordered_directories(directories);
    let mut redacted = String::with_capacity(text.len());
    let mut previous: Option<char> = None;
    let mut rest = text;
    while let Some(character) = rest.chars().next() {
        if character == '/' && opens_path(previous) {
            let end = rest.find(closes_path).unwrap_or(rest.len());
            redacted.push_str(&redact_path_token(&rest[..end], &known));
            previous = rest[..end].chars().next_back();
            rest = &rest[end..];
        } else {
            redacted.push(character);
            previous = Some(character);
            rest = &rest[character.len_utf8()..];
        }
    }
    redacted
}

/// The length of the digits at the start of `bytes`.
fn digits(bytes: &[u8]) -> usize {
    bytes.iter().take_while(|byte| byte.is_ascii_digit()).count()
}

/// The length of the RFC 3339 date-time at the start of `bytes`, or `None`.
fn timestamp_length(bytes: &[u8]) -> Option<usize> {
    let number = |at: usize, count: usize, least: u32, most: u32| -> Option<u32> {
        let field = bytes.get(at..at + count)?;
        if digits(field) != count {
            return None;
        }
        let value = field
            .iter()
            .fold(0_u32, |value, digit| value * 10 + u32::from(digit - b'0'));
        (least..=most).contains(&value).then_some(value)
    };
    let at = |index: usize, expected: &[u8]| bytes.get(index).filter(|b| expected.contains(b));
    number(0, 4, 0, 9999)?;
    at(4, b"-")?;
    number(5, 2, 1, 12)?;
    at(7, b"-")?;
    number(8, 2, 1, 31)?;
    at(10, b"Tt")?;
    number(11, 2, 0, 23)?;
    at(13, b":")?;
    number(14, 2, 0, 59)?;
    at(16, b":")?;
    number(17, 2, 0, 60)?;
    let mut end = 19;
    if bytes.get(end) == Some(&b'.') {
        let fraction = digits(&bytes[end + 1..]);
        if fraction == 0 {
            return None;
        }
        end += 1 + fraction;
    }
    match bytes.get(end)? {
        b'Z' | b'z' => Some(end + 1),
        b'+' | b'-' => {
            let offset = &bytes[end + 1..];
            let well_formed = digits(offset) >= 2
                && offset.get(2) == Some(&b':')
                && offset.get(3..).is_some_and(|minutes| digits(minutes) >= 2);
            well_formed.then_some(end + 6)
        }
        _ => None,
    }
}

/// Replaces every match of `length_at`, tried at each byte offset whose previous character
/// satisfies `may_start`, with `placeholder`; the text outside the matches is kept as written.
fn replace_matches(
    text: &str,
    placeholder: &str,
    may_start: impl Fn(Option<char>) -> bool,
    length_at: impl Fn(&str) -> Option<usize>,
) -> String {
    let mut replaced = String::with_capacity(text.len());
    let mut previous: Option<char> = None;
    let mut rest = text;
    while let Some(character) = rest.chars().next() {
        if let Some(length) = may_start(previous)
            .then(|| length_at(rest))
            .flatten()
            .filter(|length| *length > 0)
        {
            replaced.push_str(placeholder);
            previous = rest[..length].chars().next_back();
            rest = &rest[length..];
        } else {
            replaced.push(character);
            previous = Some(character);
            rest = &rest[character.len_utf8()..];
        }
    }
    replaced
}

fn redact_timestamps(text: &str) -> String {
    replace_matches(
        text,
        "<TIMESTAMP>",
        |_| true,
        |rest| timestamp_length(rest.as_bytes()),
    )
}

/// The length of the duration at the start of `rest`, or `None`.
fn duration_length(rest: &str) -> Option<usize> {
    let bytes = rest.as_bytes();
    let mut end = digits(bytes);
    if end == 0 {
        return None;
    }
    if bytes.get(end) == Some(&b'.') {
        let fraction = digits(&bytes[end + 1..]);
        if fraction > 0 {
            end += 1 + fraction;
        }
    }
    if matches!(bytes.get(end), Some(b'e' | b'E')) {
        let sign = usize::from(matches!(bytes.get(end + 1), Some(b'+' | b'-')));
        let exponent = digits(bytes.get(end + 1 + sign..)?);
        if exponent > 0 {
            end += 1 + sign + exponent;
        }
    }
    let unit = if bytes[end..].starts_with(b"ms") {
        2
    } else if bytes[end..].starts_with(b"s") {
        1
    } else {
        return None;
    };
    let end = end + unit;
    let continues = rest[end..]
        .chars()
        .next()
        .is_some_and(|next| next.is_alphanumeric() || next == '_');
    (!continues).then_some(end)
}

fn redact_durations(text: &str) -> String {
    replace_matches(
        text,
        "<DURATION>",
        |previous| previous.is_none_or(|p| !(p.is_alphanumeric() || p == '_' || p == '.')),
        duration_length,
    )
}

/// Replaces the number after `pid`, `PID` or `process` and the id of a panicking thread.
fn redact_process_ids(text: &str) -> String {
    let mut redacted = String::with_capacity(text.len());
    let mut previous: Option<char> = None;
    let mut rest = text;
    while let Some(character) = rest.chars().next() {
        let word_start = previous.is_none_or(|p| !(p.is_alphanumeric() || p == '_'));
        let consumed = word_start.then(|| process_id_span(rest)).flatten();
        if let Some((keep, number)) = consumed {
            redacted.push_str(&rest[..keep]);
            redacted.push_str("<PID>");
            previous = Some('>');
            rest = &rest[keep + number..];
        } else {
            redacted.push(character);
            previous = Some(character);
            rest = &rest[character.len_utf8()..];
        }
    }
    redacted
}

/// For `rest` starting at a word, the length of the text kept before an id and the length of the
/// id.
fn process_id_span(rest: &str) -> Option<(usize, usize)> {
    for word in ["pid ", "PID ", "process "] {
        if let Some(after) = rest.strip_prefix(word) {
            let number = digits(after.as_bytes());
            return (number > 0).then_some((word.len(), number));
        }
    }
    let after = rest.strip_prefix("thread '")?;
    let name = after.find('\'')?;
    let tail = after[name + 1..].strip_prefix(" (")?;
    let number = digits(tail.as_bytes());
    tail[number..]
        .strip_prefix(") panicked")
        .filter(|_| number > 0)
        .map(|_| (rest.len() - tail.len(), number))
}
