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
/// Trace: FR-017-AC-35, TC-027
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
    /// Reads the streams once: the two excerpts and the stage each member reached. The
    /// result serves every member of the group, so the work does not grow with the member count.
    pub(crate) fn analyse(&self) -> ProcessDiagnostics {
        let stdout = String::from_utf8_lossy(self.stdout);
        let stderr = String::from_utf8_lossy(self.stderr);
        ProcessDiagnostics {
            stdout: excerpt(self.stdout, &self.directories),
            stderr: excerpt(self.stderr, &self.directories),
            stages: Stages::read(&stdout, self.selections),
            report_exported: self.report_exported,
            parallel: runs_in_parallel(self.arguments),
            cargo_failed: has_line_starting(
                &stdout,
                "error: Failed to execute cargo (exit status: ",
            ) || has_line_starting(&stderr, "error: could not compile "),
            banner: has_line_starting(&stdout, "Kani Rust Verifier "),
        }
    }
}

/// What a launcher process printed, read once for all of its members.
#[derive(Clone, Debug)]
pub(crate) struct ProcessDiagnostics {
    stdout: KaniStreamExcerpt,
    stderr: KaniStreamExcerpt,
    stages: Stages,
    report_exported: bool,
    parallel: bool,
    /// Whether the output reports a crate that did not build.
    cargo_failed: bool,
    /// Whether stdout holds Kani's banner line.
    banner: bool,
}

impl ProcessDiagnostics {
    /// The record of the member whose path is `member`.
    pub(crate) fn diagnostics_of(&self, member: &str) -> KaniNoVerdictDiagnostics {
        KaniNoVerdictDiagnostics {
            phase: self.phase_of(member),
            stdout: self.stdout.clone(),
            stderr: self.stderr.clone(),
        }
    }

    /// The first of the phase rules that applies to `member`.
    fn phase_of(&self, member: &str) -> KaniFailurePhase {
        if self.report_exported {
            return KaniFailurePhase::Unknown;
        }
        let by_segment = match self.stages.stage_of(member) {
            Some(Stage { solving: true }) => Some(KaniFailurePhase::Solver),
            Some(Stage { solving: false }) => Some(KaniFailurePhase::Backend),
            None if self.stages.any() => Some(KaniFailurePhase::Unknown),
            None => None,
        };
        if let Some(phase) = by_segment {
            // Harnesses checked in parallel interleave their lines, so no segment is one
            // member's output.
            return if self.parallel {
                KaniFailurePhase::Unknown
            } else {
                phase
            };
        }
        if self.cargo_failed {
            KaniFailurePhase::Compile
        } else if !self.banner {
            KaniFailurePhase::Launcher
        } else {
            KaniFailurePhase::Unknown
        }
    }
}

/// Whether `arguments` ask Kani to check harnesses in parallel: `-j`, `--jobs`, `--jobs=<n>`,
/// `-j<digits>` or `-j=<digits>`.
fn runs_in_parallel(arguments: &[String]) -> bool {
    arguments.iter().any(|argument| {
        let attached = argument.strip_prefix("-j").is_some_and(|rest| {
            let count = rest.strip_prefix('=').unwrap_or(rest);
            !count.is_empty() && count.bytes().all(|byte| byte.is_ascii_digit())
        });
        argument == "-j" || argument == "--jobs" || argument.starts_with("--jobs=") || attached
    })
}

fn has_line_starting(text: &str, prefix: &str) -> bool {
    text.lines().any(|line| line.starts_with(prefix))
}

/// What stdout shows of one member's `Checking harness <path>...` segment.
#[derive(Clone, Copy, Debug)]
struct Stage {
    /// Whether a segment headed by the member holds a line beginning `Solving with `.
    solving: bool,
}

/// The members that have a segment in a stdout: a segment starts at a line that is exactly
/// `Checking harness <path>...` for a path the launch passed, and runs to the next such line.
#[derive(Clone, Debug)]
struct Stages(std::collections::BTreeMap<String, Stage>);

impl Stages {
    fn read(stdout: &str, selections: &[String]) -> Self {
        let mut stages = std::collections::BTreeMap::new();
        let mut open: Option<&str> = None;
        for line in stdout.lines() {
            let heading = line
                .strip_prefix("Checking harness ")
                .and_then(|rest| rest.strip_suffix("..."))
                .and_then(|path| selections.iter().find(|selection| *selection == path));
            if let Some(member) = heading {
                stages
                    .entry(member.clone())
                    .or_insert(Stage { solving: false });
                open = Some(member);
            } else if line.starts_with("Solving with ") {
                if let Some(stage) = open.and_then(|member| stages.get_mut(member)) {
                    stage.solving = true;
                }
            }
        }
        Self(stages)
    }

    fn any(&self) -> bool {
        !self.0.is_empty()
    }

    fn stage_of(&self, member: &str) -> Option<Stage> {
        self.0.get(member).copied()
    }
}

/// The redacted tail of `bytes`.
fn excerpt(bytes: &[u8], directories: &KnownDirectories<'_>) -> KaniStreamExcerpt {
    let text = redact_text(&String::from_utf8_lossy(bytes), directories);
    let start = tail_start(&text);
    KaniStreamExcerpt {
        omitted_bytes: u64::try_from(start).unwrap_or(u64::MAX),
        text: text[start..].to_owned(),
    }
}

/// `text` with control characters and escapes removed, then paths, date-times, durations and
/// process identities redacted.
pub(crate) fn redact_text(text: &str, directories: &KnownDirectories<'_>) -> String {
    let text = strip_control(text);
    let text = redact_paths(&text, directories);
    let text = redact_timestamps(&text);
    let text = redact_durations(&text);
    redact_process_ids(&text)
}

/// The byte offset where the kept tail of `text` starts.
fn tail_start(text: &str) -> usize {
    let bytes = text.as_bytes();
    let Some(least) = text
        .len()
        .checked_sub(EXCERPT_LIMIT)
        .filter(|least| *least > 0)
    else {
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
            while characters
                .next_if(|next| ('\u{20}'..='\u{2f}').contains(next))
                .is_some()
            {}
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
        matches!(
            previous,
            '\n' | ' ' | '\t' | '(' | '[' | '{' | '\'' | '"' | '`' | '='
        )
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
    bytes
        .iter()
        .take_while(|byte| byte.is_ascii_digit())
        .count()
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
    at(10, b"Tt ")?;
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
            number(end + 1, 2, 0, 23)?;
            at(end + 3, b":")?;
            number(end + 4, 2, 0, 59)?;
            Some(end + 6)
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
        |previous| previous.is_none_or(|p| !(p.is_alphanumeric() || p == '_' || p == '.')),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kani::test_support::implements_deserialize;

    const HOME: &str = "/h/u";

    fn directories(home: Option<&'static str>) -> KnownDirectories<'static> {
        KnownDirectories {
            crate_directory: Path::new("/h/u/ws/crate"),
            target_directory: Path::new("/h/u/ws/crate/target"),
            home_directory: home.map(Path::new),
        }
    }

    fn redacted(text: &str, home: Option<&'static str>) -> String {
        excerpt(text.as_bytes(), &directories(home)).text
    }

    /// An excerpt of `text` with nothing in it to redact.
    fn kept(text: &str) -> KaniStreamExcerpt {
        excerpt(text.as_bytes(), &directories(None))
    }

    /// Every input FR-017-AC-35 names yields its stated output, one per line, in order.
    ///
    /// Trace: FR-017-AC-35, TC-027
    #[test]
    fn tc_027_every_redaction_input_yields_its_stated_output() {
        let pairs = [
            ("(/h/u/ws/crate)", "(<CRATE_DIR>)"),
            (
                "/h/u/ws/crate/src/lib.rs:6:21",
                "<CRATE_DIR>/src/lib.rs:6:21",
            ),
            (
                "/h/u/ws/crate/target/debug/x.out",
                "<TARGET_DIR>/debug/x.out",
            ),
            ("/h/u/.kani/bin", "<HOME>/.kani/bin"),
            ("/h/u/ws/crate2/src/lib.rs", "<HOME>/ws/crate2/src/lib.rs"),
            ("/opt/tool/bin", "<PATH>"),
            ("https://host/x/y", "https://host/x/y"),
            ("and/or", "and/or"),
            ("/", "/"),
            ("2026-10-16T22:27:04.275975036Z", "<TIMESTAMP>"),
            ("Runtime Symex: 0.00331318s", "Runtime Symex: <DURATION>"),
            ("Runtime Solver: 1.385e-05s", "Runtime Solver: <DURATION>"),
            ("in 0.02s", "in <DURATION>"),
            ("check_ttl_30s", "check_ttl_30s"),
            ("u32s", "u32s"),
            ("pid 4242", "pid <PID>"),
            (
                "thread 'main' (4242) panicked",
                "thread 'main' (<PID>) panicked",
            ),
        ];
        let input: Vec<_> = pairs.iter().map(|(input, _)| *input).collect();
        let expected: Vec<_> = pairs.iter().map(|(_, output)| *output).collect();
        assert_eq!(
            redacted(&input.join("\n"), Some(HOME))
                .lines()
                .collect::<Vec<_>>(),
            expected
        );
    }

    /// A path is matched at a component boundary only, a home directory of `/` redacts nothing
    /// as `<HOME>`, and an absent one leaves a path under it as `<PATH>`.
    ///
    /// Trace: FR-017-AC-35, TC-027
    #[test]
    fn tc_027_known_directories_match_whole_components_and_unusable_homes_redact_nothing() {
        let sibling = KnownDirectories {
            crate_directory: Path::new("/x/crate"),
            target_directory: Path::new("/x/crate/target/"),
            home_directory: Some(Path::new("/x")),
        };
        let text = "/x/crate2/lib.rs /x/crate/lib.rs /x/crate/target /x/other/y /x";
        assert_eq!(
            excerpt(text.as_bytes(), &sibling).text,
            "<HOME>/crate2/lib.rs <CRATE_DIR>/lib.rs <TARGET_DIR> <HOME>/other/y <HOME>"
        );
        let stream = "/h/u/.kani/bin /h/u/ws/crate/src/lib.rs";
        let absent = redacted(stream, None);
        assert_eq!(absent, "<PATH> <CRATE_DIR>/src/lib.rs");
        assert_eq!(redacted(stream, Some("/")), absent);
        assert_eq!(redacted(stream, Some("relative/home")), absent);
        assert_eq!(redacted(stream, Some("")), absent);
    }

    /// A directory is tried longest first and the crate wins an equal-length tie with the home.
    ///
    /// Trace: FR-017-AC-35, TC-027
    #[test]
    fn tc_027_the_longest_known_directory_wins_and_a_tie_goes_crate_target_home() {
        let same = KnownDirectories {
            crate_directory: Path::new("/a/b"),
            target_directory: Path::new("/a/b"),
            home_directory: Some(Path::new("/a/b")),
        };
        assert_eq!(excerpt(b"/a/b/c", &same).text, "<CRATE_DIR>/c");
        let nested = KnownDirectories {
            crate_directory: Path::new("/a/b"),
            target_directory: Path::new("/a"),
            home_directory: Some(Path::new("/a/b/c")),
        };
        assert_eq!(
            excerpt(b"/a/b/c/d /a/z", &nested).text,
            "<HOME>/d <TARGET_DIR>/z"
        );
    }

    /// Every character the spec lists as opening a path token opens one, and any other
    /// preceding character does not.
    ///
    /// Trace: FR-017-AC-35, TC-027
    #[test]
    fn tc_027_every_listed_path_opener_starts_a_token_and_no_other_character_does() {
        for opener in ['\n', ' ', '\t', '(', '[', '{', '\'', '"', '`', '='] {
            let text = format!("x{opener}/h/u/ws/crate/src/lib.rs");
            assert_eq!(
                redacted(&text, Some(HOME)),
                format!("x{opener}<CRATE_DIR>/src/lib.rs"),
                "{opener:?}"
            );
        }
        for other in [
            'a', '0', ',', ';', ':', ')', ']', '}', '<', '>', '-', '.', '!',
        ] {
            let text = format!("x{other}/h/u/ws/crate/src/lib.rs");
            assert_eq!(redacted(&text, Some(HOME)), text, "{other:?}");
        }
        assert_eq!(redacted("/h/u/.kani/bin", Some(HOME)), "<HOME>/.kani/bin");
    }

    /// Every character the spec lists as ending a path token ends it, and any other character
    /// stays inside it.
    ///
    /// Trace: FR-017-AC-35, TC-027
    #[test]
    fn tc_027_every_listed_path_closer_ends_a_token_and_no_other_character_does() {
        for closer in [
            ' ', '\t', '\n', ')', ']', '}', '\'', '"', '`', ',', ';', ':',
        ] {
            let text = format!("/opt/a/b{closer}rest");
            assert_eq!(
                redacted(&text, None),
                format!("<PATH>{closer}rest"),
                "{closer:?}"
            );
        }
        for inside in ['.', '-', '_', '<', '>', '!', '0', '|', '\\', '#'] {
            let text = format!("/opt/a/b{inside}rest");
            assert_eq!(redacted(&text, None), "<PATH>", "{inside:?}");
        }
    }

    /// Redaction rules that are about boundaries: durations, process identities and timestamps.
    ///
    /// Trace: FR-017-AC-35, TC-027
    #[test]
    fn tc_027_duration_pid_and_timestamp_rules_respect_their_boundaries() {
        for (input, output) in [
            ("took 5ms.", "took <DURATION>."),
            ("1.5e+3s done", "<DURATION> done"),
            ("0.5s", "<DURATION>"),
            ("x1.5s", "x1.5s"),
            ("1.2.3s", "1.2.3s"),
            ("5sec", "5sec"),
            ("5s_x", "5s_x"),
            ("30m", "30m"),
            ("subprocess 12", "subprocess 12"),
            ("PID 7 and process 8", "PID <PID> and process <PID>"),
            ("pid  9", "pid  9"),
            ("pidx 9", "pidx 9"),
            (
                "thread 'a b' (12) panicked at",
                "thread 'a b' (<PID>) panicked at",
            ),
            ("thread 'main' (x) panicked", "thread 'main' (x) panicked"),
            ("2026-10-16T22:27:04+02:00 up", "<TIMESTAMP> up"),
            ("2026-10-16T22:27:04 no zone", "2026-10-16T22:27:04 no zone"),
            ("2026-13-16T22:27:04Z", "2026-13-16T22:27:04Z"),
            ("x2026-10-16T22:27:04Z", "x2026-10-16T22:27:04Z"),
            ("12026-10-16T22:27:04Z", "12026-10-16T22:27:04Z"),
            ("_2026-10-16T22:27:04Z", "_2026-10-16T22:27:04Z"),
            (".2026-10-16T22:27:04Z", ".2026-10-16T22:27:04Z"),
            ("at 2026-10-16 22:27:04Z up", "at <TIMESTAMP> up"),
            ("2026-10-16t22:27:04z", "<TIMESTAMP>"),
            ("2026-10-16T22:27:04+24:00", "2026-10-16T22:27:04+24:00"),
            ("2026-10-16T22:27:04+02:60", "2026-10-16T22:27:04+02:60"),
            ("2026-10-16T22:27:04+0200", "2026-10-16T22:27:04+0200"),
            ("2026-10-16T22:27:04-02:00:00", "<TIMESTAMP>:00"),
            ("2026-10-16T22:27:04.Z", "2026-10-16T22:27:04.Z"),
        ] {
            assert_eq!(redacted(input, None), output, "{input}");
        }
    }

    /// Escapes are removed and other control characters are replaced; invalid UTF-8 is decoded
    /// with U+FFFD.
    ///
    /// Trace: FR-017-AC-34, TC-027
    #[test]
    fn tc_027_ansi_escapes_go_and_controls_and_invalid_bytes_become_replacement_characters() {
        let colored = "\u{1b}[1;31merror\u{1b}[0m: \u{1b}]0;title\u{7}bad\u{1b}(Bx\tz\ry\u{7}";
        assert_eq!(kept(colored).text, "error: badx\tz\u{fffd}y\u{fffd}");
        assert_eq!(
            excerpt(b"a\xffb\xc3", &directories(None)).text,
            "a\u{fffd}b\u{fffd}"
        );
    }

    /// A stream of exactly the limit is whole; one byte more keeps the longest line-aligned
    /// suffix and counts what it dropped.
    ///
    /// Trace: FR-017-AC-34, TC-027
    #[test]
    fn tc_027_a_stream_at_the_limit_is_whole_and_one_over_keeps_a_line_aligned_suffix() {
        let whole = format!("{}\n", "a".repeat(8191));
        let at_limit = kept(&whole);
        assert_eq!((at_limit.text.len(), at_limit.omitted_bytes), (8192, 0));

        let over = format!("first\n{}\n", "b".repeat(8190));
        assert_eq!(over.len(), 8197);
        let tail = kept(&over);
        assert_eq!(tail.text, format!("{}\n", "b".repeat(8190)));
        assert_eq!(tail.omitted_bytes, 6);

        let at_start = format!("{}\n{}", "c".repeat(3), "d".repeat(8192 - 1));
        let tail = kept(&at_start);
        assert_eq!(tail.omitted_bytes, 4);
        assert_eq!(tail.text, "d".repeat(8191));
    }

    /// A final line over the limit is cut at a character boundary and ends at the last byte; a
    /// 4-byte character is never split.
    ///
    /// Trace: FR-017-AC-34, TC-027
    #[test]
    fn tc_027_a_final_line_over_the_limit_is_cut_on_a_character_boundary() {
        let long = format!("x\n{}", "y".repeat(10_000));
        let tail = kept(&long);
        assert_eq!(tail.text, "y".repeat(8192));
        assert_eq!(tail.omitted_bytes, 1810);

        // The cut falls inside the 4-byte character at offset 3809, so the tail starts at the
        // next character, 3813.
        let emoji = format!("a{}b", "\u{1f980}".repeat(3000));
        let tail = kept(&emoji);
        assert_eq!(tail.text, format!("{}b", "\u{1f980}".repeat(2047)));
        assert_eq!(tail.omitted_bytes, 3813);
    }

    /// One 10000-byte line and its line feed keeps the last 8192 bytes ending in the line feed,
    /// never an empty excerpt; a long line followed by a short last line keeps only the last.
    ///
    /// Trace: FR-017-AC-34, TC-027
    #[test]
    fn tc_027_a_long_line_with_its_line_feed_is_never_an_empty_excerpt() {
        let one = format!("{}\n", "z".repeat(10_000));
        let tail = kept(&one);
        assert_eq!(tail.text.len(), 8192);
        assert!(tail.text.ends_with('\n'));
        assert_eq!(tail.omitted_bytes, 1809);

        let last = format!("{}\nerror: last\n", "z".repeat(10_000));
        let tail = kept(&last);
        assert_eq!(tail.text, "error: last\n");
        assert_eq!(tail.omitted_bytes, 10_001);
    }

    /// A stream of line feeds only is kept up to the limit; an empty stream is an empty excerpt.
    ///
    /// Trace: FR-017-AC-34, TC-027
    #[test]
    fn tc_027_a_line_feed_only_stream_is_kept_and_an_empty_one_is_empty() {
        let tail = kept(&"\n".repeat(9000));
        assert_eq!((tail.text.len(), tail.omitted_bytes), (8192, 808));
        let short = kept("\n\n\n");
        assert_eq!((short.text.as_str(), short.omitted_bytes), ("\n\n\n", 0));
        let empty = kept("");
        assert_eq!((empty.text.as_str(), empty.omitted_bytes), ("", 0));
    }

    /// A stream of the 8 MiB limit still yields an excerpt of at most 8192 bytes, counting the
    /// rest as omitted.
    ///
    /// Trace: FR-017-AC-34, TC-027
    #[test]
    fn tc_027_a_stream_at_the_eight_mebibyte_limit_still_yields_a_bounded_excerpt() {
        let line = "stream line of text that Kani printed\n";
        let stream = line.repeat(8 * 1024 * 1024 / line.len() + 1);
        let tail = excerpt(&stream.as_bytes()[..8 * 1024 * 1024], &directories(None));
        assert!(tail.text.len() <= 8192);
        assert!(tail.text.starts_with("stream line"));
        assert_eq!(
            usize::try_from(tail.omitted_bytes).unwrap() + tail.text.len(),
            8 * 1024 * 1024
        );
    }

    fn output<'a>(
        stdout: &'a str,
        stderr: &'a str,
        selections: &'a [String],
        arguments: &'a [String],
        report_exported: bool,
    ) -> ProcessDiagnostics {
        ProcessOutput {
            stdout: stdout.as_bytes(),
            stderr: stderr.as_bytes(),
            selections,
            arguments,
            report_exported,
            directories: directories(None),
        }
        .analyse()
    }

    const BANNER: &str = "Kani Rust Verifier 0.68.0 (cargo plugin)\n";

    fn names(paths: &[&str]) -> Vec<String> {
        paths.iter().map(|path| (*path).to_owned()).collect()
    }

    /// Each member takes the stage of its own segment; a member that never started is `unknown`
    /// even beside a neighbour that reached the solver.
    ///
    /// Trace: FR-017-AC-36, TC-043
    #[test]
    fn tc_043_each_member_takes_the_phase_of_its_own_segment() {
        let members = names(&["a::check", "b::check", "c::check"]);
        let stdout = format!(
            "{BANNER}Checking harness a::check...\nSolving with CaDiCaL 3.0.0\nChecking harness b::check...\n"
        );
        let process = output(&stdout, "", &members, &[], false);
        let phases: Vec<_> = members.iter().map(|m| process.phase_of(m)).collect();
        assert_eq!(
            phases,
            [
                KaniFailurePhase::Solver,
                KaniFailurePhase::Backend,
                KaniFailurePhase::Unknown
            ]
        );
        let compile = format!("{BANNER}error: Failed to execute cargo (exit status: 101).\n");
        let process = output(&compile, "", &members, &[], false);
        assert!(members
            .iter()
            .all(|m| process.phase_of(m) == KaniFailurePhase::Compile));
    }

    /// A solver line before any segment, or in another member's segment, is not the member's.
    ///
    /// Trace: FR-017-AC-33, TC-027
    #[test]
    fn tc_027_a_solver_line_outside_the_members_segment_does_not_make_it_solver() {
        let members = names(&["a::check", "b::check"]);
        let stdout = format!(
            "{BANNER}Solving with CaDiCaL 3.0.0\nChecking harness a::check...\nChecking harness b::check...\nSolving with CaDiCaL 3.0.0\n"
        );
        let process = output(&stdout, "", &members, &[], false);
        assert_eq!(process.phase_of("a::check"), KaniFailurePhase::Backend);
        assert_eq!(process.phase_of("b::check"), KaniFailurePhase::Solver);
    }

    /// `-j`, `--jobs` and `--jobs=N` make every member a segment would decide `unknown`, and
    /// leave a member decided by another rule where it was.
    ///
    /// Trace: FR-017-AC-33, TC-027
    #[test]
    fn tc_027_parallel_jobs_make_segment_phases_unknown_but_not_compile_or_launcher() {
        let members = names(&["a::check"]);
        let stdout = format!("{BANNER}Checking harness a::check...\nSolving with CaDiCaL 3.0.0\n");
        for arguments in [
            names(&["-j", "2"]),
            names(&["--jobs", "2"]),
            names(&["--jobs=2"]),
            names(&["-j2"]),
            names(&["-j=2"]),
            names(&["-j16"]),
        ] {
            let process = output(&stdout, "", &members, &arguments, false);
            assert_eq!(
                process.phase_of("a::check"),
                KaniFailurePhase::Unknown,
                "{arguments:?}"
            );
        }
        let lookalikes = names(&["--jobsx=2", "-jx", "-j=", "-j=x", "-j2x", "-junk"]);
        let process = output(&stdout, "", &members, &lookalikes, false);
        assert_eq!(process.phase_of("a::check"), KaniFailurePhase::Solver);
        let compile = format!("{BANNER}error: Failed to execute cargo (exit status: 1)\n");
        let jobs = names(&["-j", "2"]);
        let process = output(&compile, "", &members, &jobs, false);
        assert_eq!(process.phase_of("a::check"), KaniFailurePhase::Compile);
        let process = output("", "usage\n", &members, &jobs, false);
        assert_eq!(process.phase_of("a::check"), KaniFailurePhase::Launcher);
    }

    /// The phase comes from the whole stream, not the excerpt: a marker 20000 bytes before the
    /// end still decides it.
    ///
    /// Trace: FR-017-AC-33, TC-027
    #[test]
    fn tc_027_a_marker_before_the_excerpt_window_still_decides_the_phase() {
        let members = names(&["a::check"]);
        let stdout = format!(
            "{BANNER}Checking harness a::check...\nSolving with CaDiCaL 3.0.0\n{}",
            "filler line\n".repeat(2000)
        );
        let process = output(&stdout, "", &members, &[], false);
        let record = process.diagnostics_of("a::check");
        assert_eq!(record.phase, KaniFailurePhase::Solver);
        assert!(record.stdout.omitted_bytes > 0);
        assert!(!record.stdout.text.contains("Solving with"));
    }

    /// The record and its parts implement no deserialization, so Kani output can never be
    /// decoded into them. The probe is sound: a deserializable control type reports `true`.
    ///
    /// Trace: FR-017-AC-35, TC-027
    #[test]
    fn tc_027_the_record_and_its_parts_implement_no_deserialize() {
        assert!(
            implements_deserialize!(String),
            "the probe sees a Deserialize impl"
        );
        assert!(!implements_deserialize!(KaniNoVerdictDiagnostics));
        assert!(!implements_deserialize!(KaniStreamExcerpt));
        assert!(!implements_deserialize!(KaniFailurePhase));
    }

    /// The record serializes as `diagnostics { phase, stdout { text, omittedBytes }, stderr {...} }`.
    ///
    /// Trace: FR-017-AC-35, TC-027
    #[test]
    fn tc_027_the_record_serializes_with_the_stated_member_names() {
        let members = names(&["a::check"]);
        let record = output("out\n", "", &members, &[], false).diagnostics_of("a::check");
        assert_eq!(
            serde_json::to_value(&record).unwrap(),
            serde_json::json!({
                "phase": "launcher",
                "stdout": { "text": "out\n", "omittedBytes": 0 },
                "stderr": { "text": "", "omittedBytes": 0 },
            })
        );
    }
}
