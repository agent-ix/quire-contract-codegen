//! The concrete-playback block Kani prints on the console.
//!
//! Kani's report carries no concrete playback. The falsifying playback is a Rust unit test Kani
//! prints on stdout, and [`select_counterexample`] extracts that one fenced block from the
//! console text. It reads a payload to hand on verbatim, never a verdict: whether a run
//! falsified is decided by the report alone, and the block is looked up only after the report
//! names a failed property check.
//!
//! This is the sole reader of Kani's printed playback wording. It provides typed entries and a
//! typed refusal to witness decoding; the report remains the sole verdict source (AD-004 L-6).

const PLAYBACK_HEADER: &str = "Concrete playback unit test";
const PLAYBACK_FENCE: &str = "```";
const PLAYBACK_ENTRY_POINT: &str = "kani::concrete_playback_run";
const PLAYBACK_COVER_MARKER: &str = "/// Check for `cover`";

/// One fenced concrete-playback block Kani printed, with the `module::harness` path it is headed
/// for ("Concrete playback unit test for `<harness>`:").
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PlaybackBlock<'a> {
    /// The path in the block's heading; empty when the heading names none.
    pub(crate) harness: &'a str,
    /// The unit test between the fences, trimmed, verbatim.
    pub(crate) test: &'a str,
    /// Whether the block witnesses a failed property: it runs the playback entry point and is not
    /// the playback of a satisfied cover, which witnesses reachability.
    pub(crate) counterexample: bool,
}

/// Every playback block in `text`, in print order. A fence with no closing fence ends the scan.
pub(crate) fn playback_blocks(text: &str) -> Vec<PlaybackBlock<'_>> {
    let mut blocks = Vec::new();
    let mut rest = text;
    while let Some((_, after_header)) = rest.split_once(PLAYBACK_HEADER) {
        let Some((heading, after_open)) = after_header.split_once(PLAYBACK_FENCE) else {
            break;
        };
        let Some((body, after_close)) = after_open.split_once(PLAYBACK_FENCE) else {
            break;
        };
        let test = body.trim();
        blocks.push(PlaybackBlock {
            harness: heading.split('`').nth(1).unwrap_or_default(),
            test,
            counterexample: test.contains(PLAYBACK_ENTRY_POINT)
                && !test
                    .lines()
                    .any(|line| line.starts_with(PLAYBACK_COVER_MARKER)),
        });
        rest = after_close;
    }
    blocks
}

/// A concrete property playback selected by the Kani output reader.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CounterexamplePlayback {
    test: String,
}

impl CounterexamplePlayback {
    /// The unit test printed by Kani for the falsifying check.
    pub fn test(&self) -> &str {
        &self.test
    }
}

/// Selects a falsifying playback from console output for one harness.
pub fn select_counterexample(text: &str, harness: Option<&str>) -> Option<CounterexamplePlayback> {
    let blocks = playback_blocks(text);
    counterexample_from_blocks(&blocks, harness)
}

/// Selects a member's playback from blocks already scanned for the batch.
pub(crate) fn counterexample_from_blocks(
    blocks: &[PlaybackBlock<'_>],
    harness: Option<&str>,
) -> Option<CounterexamplePlayback> {
    blocks
        .iter()
        .find(|block| block.counterexample && harness.is_none_or(|path| block.harness == path))
        .map(|block| CounterexamplePlayback {
            test: block.test.to_owned(),
        })
}

const HARNESS_MARKER: &str = "/// Test generated for harness `";
const CHECK_MARKER: &str = "/// Check for `";

/// Stable reasons the Kani playback reader rejects a transcript.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlaybackRefusalCode {
    /// The caller's byte ceiling was exceeded.
    BytesLimit,
    /// The caller's value ceiling was exceeded.
    ValuesLimit,
    /// A concrete byte was not an unsigned byte.
    ByteInvalid,
    /// The check kind was not an assertion or cover.
    CheckKind,
    /// A check heading was absent.
    CheckMissing,
    /// The check text was absent or malformed.
    CheckTextMissing,
    /// A rendered-value comment was absent.
    CommentMissing,
    /// The concrete-values vector was malformed.
    ConcreteValuesMalformed,
    /// The concrete-values vector was absent.
    ConcreteValuesMissing,
    /// Only a cover was present.
    Cover,
    /// The harness heading was absent.
    HarnessMissing,
    /// More than one assertion block was present.
    MultipleAssertions,
}

impl PlaybackRefusalCode {
    /// Every stable playback-reader refusal code.
    pub const fn all() -> &'static [Self] {
        &[
            Self::BytesLimit,
            Self::ValuesLimit,
            Self::ByteInvalid,
            Self::CheckKind,
            Self::CheckMissing,
            Self::CheckTextMissing,
            Self::CommentMissing,
            Self::ConcreteValuesMalformed,
            Self::ConcreteValuesMissing,
            Self::Cover,
            Self::HarnessMissing,
            Self::MultipleAssertions,
        ]
    }

    /// Finds a reader refusal code by its stable spelling.
    pub fn from_code(code: &str) -> Option<Self> {
        Self::all()
            .iter()
            .copied()
            .find(|candidate| candidate.as_str() == code)
    }

    /// The stable code retained in the witness decoder's public failure envelope.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BytesLimit => "cg_witness_decode_bytes_limit",
            Self::ValuesLimit => "cg_witness_decode_values_limit",
            Self::ByteInvalid => "kani_witness_byte_invalid",
            Self::CheckKind => "kani_witness_check_kind_refused",
            Self::CheckMissing => "kani_witness_check_missing",
            Self::CheckTextMissing => "kani_witness_check_text_missing",
            Self::CommentMissing => "kani_witness_comment_missing",
            Self::ConcreteValuesMalformed => "kani_witness_concrete_vals_malformed",
            Self::ConcreteValuesMissing => "kani_witness_concrete_vals_missing",
            Self::Cover => "kani_witness_cover_refused",
            Self::HarnessMissing => "kani_witness_harness_missing",
            Self::MultipleAssertions => "kani_witness_multiple_assertions_refused",
        }
    }
}

/// Why a transcript did not decode: the typed cause and the identities the reader named.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlaybackRefusal {
    /// Stable machine-readable cause code.
    pub code: PlaybackRefusalCode,
    /// The source or input identity that first caused the refusal.
    pub source_id: String,
    /// Refusal context. Decode-limit refusals use the module symbol with `;bytes=N` or
    /// `;values=N` appended.
    pub context: String,
}

impl PlaybackRefusal {
    pub(crate) fn new(code: PlaybackRefusalCode, source_id: &str, context: &str) -> Self {
        Self {
            code,
            source_id: source_id.to_owned(),
            context: context.to_owned(),
        }
    }
}

/// One selected assertion playback block, read.
#[derive(Debug)]
pub struct Playback<'a> {
    pub(crate) source_len: usize,
    pub(crate) harness: &'a str,
    pub(crate) check_text: &'a str,
    /// The selected check clause after its kind, including Kani's outer quotes.
    pub(crate) check_clause: &'a str,
    /// Rendered values and their untyped bytes, one per `kani::any()` call.
    pub(crate) entries: Vec<(RenderedValue, Vec<u8>)>,
}

/// A value rendered in Kani's decoded-value comment. The numeric spellings `0` and `1`
/// can denote either a Boolean or an integer; both interpretations are retained until the
/// persisted binding supplies the primitive type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RenderedValue {
    pub(crate) boolean: Option<bool>,
    pub(crate) integer: Option<i64>,
}

fn rendered_value(comment: &str) -> RenderedValue {
    let boolean = match comment.to_ascii_lowercase().as_str() {
        "true" | "1" => Some(true),
        "false" | "0" => Some(false),
        _ => None,
    };
    RenderedValue {
        boolean,
        integer: comment.parse().ok(),
    }
}

/// The check kind Kani names in a block's `Check for` clause.
enum CheckKind {
    /// The only kind that witnesses falsity.
    Assertion,
    /// A reached cover statement.
    Cover,
    /// Any other kind Kani reports.
    Other,
}

/// The `Check for` line of `block`, found by its anchor rather than by searching the whole block
/// (Kani appends caller-controlled contract text to the harness doc line, which may contain the
/// same words): the check kind and the rest of the BLOCK after the kind's closing backtick.
/// Kani prints a contract harness's check text across several lines, so the rest of the block,
/// not of the line, is where the check text's closing quote is looked for.
fn check_clause<'a>(
    block: &'a str,
    source_id: &str,
    context: &str,
) -> Result<(CheckKind, &'a str), PlaybackRefusal> {
    let missing = || PlaybackRefusal::new(PlaybackRefusalCode::CheckMissing, source_id, context);
    let mut offset = 0;
    let line_start = block
        .split_inclusive('\n')
        .find_map(|line| {
            let start = offset + (line.len() - line.trim_start().len());
            offset += line.len();
            line.trim_start().starts_with(CHECK_MARKER).then_some(start)
        })
        .ok_or_else(missing)?;
    let (kind, rest) = block[line_start + CHECK_MARKER.len()..]
        .split_once('`')
        .ok_or_else(missing)?;
    let kind = match kind {
        "assertion" => CheckKind::Assertion,
        "cover" => CheckKind::Cover,
        _ => CheckKind::Other,
    };
    Ok((kind, rest))
}

/// The single assertion playback block of a run. A run may retain several blocks (a reached
/// cover alongside a falsified contract is ordinary); none or more than one assertion block
/// refuses rather than guessing which falsification is meant.
pub(crate) fn select_assertion_block<'a>(
    transcript: &'a str,
    source_id: &str,
    context: &str,
) -> Result<&'a str, PlaybackRefusal> {
    let starts: Vec<usize> = transcript
        .match_indices(HARNESS_MARKER)
        .map(|(start, _)| start)
        .collect();
    if starts.is_empty() {
        return Err(PlaybackRefusal::new(
            PlaybackRefusalCode::HarnessMissing,
            source_id,
            context,
        ));
    }
    let ends = starts
        .iter()
        .skip(1)
        .copied()
        .chain(std::iter::once(transcript.len()));
    let mut assertion = None;
    let (mut saw_cover, mut saw_other) = (false, false);
    for (start, end) in starts.iter().copied().zip(ends) {
        let block = &transcript[start..end];
        match check_clause(block, source_id, context)?.0 {
            CheckKind::Assertion if assertion.is_some() => {
                return Err(PlaybackRefusal::new(
                    PlaybackRefusalCode::MultipleAssertions,
                    source_id,
                    context,
                ));
            }
            CheckKind::Assertion => assertion = Some(block),
            CheckKind::Cover => saw_cover = true,
            CheckKind::Other => saw_other = true,
        }
    }
    assertion.ok_or_else(|| {
        let code = if saw_cover {
            PlaybackRefusalCode::Cover
        } else if saw_other {
            PlaybackRefusalCode::CheckKind
        } else {
            PlaybackRefusalCode::CheckMissing
        };
        PlaybackRefusal::new(code, source_id, context)
    })
}

/// Reads the harness symbol, check text and concrete entries of one assertion block.
pub(crate) fn read_block<'a>(
    block: &'a str,
    source_id: &str,
    context: &str,
    max_values: usize,
) -> Result<Playback<'a>, PlaybackRefusal> {
    let harness = block
        .strip_prefix(HARNESS_MARKER)
        .ok_or_else(|| {
            PlaybackRefusal::new(PlaybackRefusalCode::HarnessMissing, source_id, context)
        })?
        .split_once('`')
        .map(|(harness, _)| harness)
        .ok_or_else(|| {
            PlaybackRefusal::new(PlaybackRefusalCode::HarnessMissing, source_id, context)
        })?;
    let (_, after_kind) = check_clause(block, source_id, context)?;
    let no_text =
        || PlaybackRefusal::new(PlaybackRefusalCode::CheckTextMissing, source_id, context);
    // Kani always prints `: "` directly after the check kind; anything else is not a check text.
    let after_quote = after_kind.strip_prefix(": \"").ok_or_else(no_text)?;
    let check_text = &after_quote[..unescaped_quote(after_quote).ok_or_else(no_text)?];
    let entries = concrete_entries(block, source_id, context, max_values)?;
    Ok(Playback {
        source_len: block.len(),
        harness,
        check_text,
        check_clause: after_kind,
        entries,
    })
}

/// Parses the sole assertion playback in a Kani transcript into typed entries.
///
/// # Errors
///
/// Returns a named playback refusal when the transcript has no unique assertion, malformed
/// concrete values, a transcript larger than `max_bytes`, or more values than `max_values`.
pub fn parse_assertion_playback<'a>(
    transcript: &'a str,
    source_id: &str,
    context: &str,
    max_bytes: usize,
    max_values: usize,
) -> Result<Playback<'a>, PlaybackRefusal> {
    if transcript.len() > max_bytes {
        return Err(PlaybackRefusal::new(
            PlaybackRefusalCode::BytesLimit,
            source_id,
            &format!("{context};bytes={max_bytes}"),
        ));
    }
    let block = select_assertion_block(transcript, source_id, context)?;
    let mut parsed = read_block(block, source_id, context, max_values)?;
    parsed.source_len = transcript.len();
    Ok(parsed)
}

/// The index of the first `"` in `text` not preceded by an odd number of backslashes.
fn unescaped_quote(text: &str) -> Option<usize> {
    let mut escaped = false;
    text.char_indices().find_map(|(index, ch)| {
        let quote = ch == '"' && !escaped;
        escaped = ch == '\\' && !escaped;
        quote.then_some(index)
    })
}

/// The body of the `[` at the start of `text` and what follows its matching `]`.
fn bracketed(text: &str) -> Option<(&str, &str)> {
    let mut depth = 0usize;
    for (index, ch) in text.char_indices() {
        match ch {
            '[' => depth += 1,
            ']' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some((text.get(1..index)?, &text[index + 1..]));
                }
            }
            _ if index == 0 => return None,
            _ => {}
        }
    }
    None
}

/// The `let concrete_vals: Vec<Vec<u8>> = vec![ ... ];` section of `block` as `(decoded-value
/// comment, untyped bytes)` pairs in declaration order. Each entry is found by its `vec![`, not
/// by its comment, so a value with no comment refuses instead of being skipped. A harness with
/// no `kani::any()` legitimately has no entries.
fn concrete_entries(
    block: &str,
    source_id: &str,
    context: &str,
    max_values: usize,
) -> Result<Vec<(RenderedValue, Vec<u8>)>, PlaybackRefusal> {
    const VEC: &str = "vec![";
    let failure = |code| PlaybackRefusal::new(code, source_id, context);
    let after_marker = block
        .split_once("let concrete_vals")
        .ok_or_else(|| failure(PlaybackRefusalCode::ConcreteValuesMissing))?
        .1;
    let outer = after_marker
        .find(VEC)
        .ok_or_else(|| failure(PlaybackRefusalCode::ConcreteValuesMissing))?;
    let (mut body, _) = bracketed(&after_marker[outer + VEC.len() - 1..])
        .ok_or_else(|| failure(PlaybackRefusalCode::ConcreteValuesMalformed))?;
    let mut entries = Vec::new();
    while let Some(entry) = body.find(VEC) {
        if entries.len() >= max_values {
            return Err(PlaybackRefusal::new(
                PlaybackRefusalCode::ValuesLimit,
                source_id,
                &format!("{context};values={max_values}"),
            ));
        }
        let comment = body[..entry]
            .rfind("//")
            .map(|at| {
                let scope = &body[at + 2..entry];
                scope[..scope.find('\n').unwrap_or(scope.len())].trim()
            })
            .ok_or_else(|| failure(PlaybackRefusalCode::CommentMissing))?;
        let (inner, rest) = bracketed(&body[entry + VEC.len() - 1..])
            .ok_or_else(|| failure(PlaybackRefusalCode::ConcreteValuesMalformed))?;
        let bytes = inner
            .split(',')
            .map(str::trim)
            .filter(|token| !token.is_empty())
            .map(|token| {
                token
                    .parse::<u8>()
                    .map_err(|_| failure(PlaybackRefusalCode::ByteInvalid))
            })
            .collect::<Result<Vec<_>, _>>()?;
        entries.push((rendered_value(comment), bytes));
        body = rest;
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;

    #[trace("FR-016-AC-1", "TC-026")]
    #[test]
    fn assertion_reader_types_rendered_values_and_refusals() {
        let transcript = "/// Test generated for harness `m::h`\n/// Check for `assertion`: \"x\"\nlet concrete_vals: Vec<Vec<u8>> = vec![\n// true\nvec![1],\n// -1\nvec![255, 255, 255, 255, 255, 255, 255, 255],\n];";
        let playback = parse_assertion_playback(transcript, "h", "m", 4096, 2)
            .expect("one assertion with two values parses");
        assert_eq!(playback.harness, "m::h");
        assert_eq!(playback.entries[0].0.boolean, Some(true));
        assert_eq!(playback.entries[0].0.integer, None);
        assert_eq!(playback.entries[1].0.integer, Some(-1));
        assert_eq!(playback.entries[1].0.boolean, None);

        let malformed = transcript.replace(": \"x\"", ": x");
        assert_eq!(
            parse_assertion_playback(&malformed, "h", "m", 4096, 2)
                .expect_err("missing check quotes refuse")
                .code,
            PlaybackRefusalCode::CheckTextMissing
        );
    }

    #[trace("FR-017-AC-5", "FR-017-AC-12", "TC-027")]
    #[test]
    fn tc_027_playback_scanning_returns_the_property_block_and_stops_at_an_unterminated_fence() {
        let cover = "Concrete playback unit test for `h`:\n```\n/// Check for `cover`: \"c\"\nfn t() { kani::concrete_playback_run(v, h); }\n```\n";
        let property = "Concrete playback unit test for `h`:\n```\n/// Check for `assertion`: \"a\"\nfn u() { kani::concrete_playback_run(v, h); }\n```\n";
        let no_entry = "Concrete playback unit test for `h`:\n```\nfn b() {}\n```\n";
        let unterminated =
            "Concrete playback unit test for `h`:\n```\nfn c() { kani::concrete_playback_run(v, h); }";
        assert_eq!(
            select_counterexample(&format!("{cover}{no_entry}{property}"), None)
                .as_ref()
                .map(CounterexamplePlayback::test),
            Some("/// Check for `assertion`: \"a\"\nfn u() { kani::concrete_playback_run(v, h); }")
        );
        assert_eq!(
            select_counterexample(&format!("{cover}{no_entry}"), None),
            None
        );
        assert_eq!(
            select_counterexample(&format!("{cover}{unterminated}"), None),
            None
        );
        assert_eq!(select_counterexample("", None), None);
    }

    /// A block is taken by the path in its own heading: another harness's earlier failing block is
    /// never the answer, a path with no failing block gets none, and every block is listed under
    /// the path it is headed for.
    #[trace("FR-017-AC-23", "TC-043")]
    #[test]
    fn tc_043_a_playback_is_taken_by_the_path_it_is_headed_for() {
        let block = |path: &str, kind: &str, body: &str| {
            format!(
                "Concrete playback unit test for `{path}`:\n```\n/// Check for `{kind}`: \"c\"\nfn {body}() {{ kani::concrete_playback_run(v, h); }}\n```\n"
            )
        };
        let text = format!(
            "{}{}{}{}",
            block("a::check", "assertion", "first"),
            block("b::check", "cover", "cover_b"),
            block("b::check", "assertion", "second"),
            block("c::check", "cover", "cover_c"),
        );
        let taken = |path| select_counterexample(&text, Some(path));
        assert!(taken("a::check").is_some_and(|test| test.test().contains("fn first")));
        assert!(taken("b::check").is_some_and(|test| test.test().contains("fn second")));
        assert_eq!(
            taken("c::check"),
            None,
            "a cover block is not a counterexample"
        );
        assert_eq!(taken("d::check"), None);
        // The unfiltered scan still answers with the first failing block of the text.
        assert!(
            select_counterexample(&text, None).is_some_and(|test| test.test().contains("fn first"))
        );
        let blocks = playback_blocks(&text);
        assert_eq!(
            blocks
                .iter()
                .map(|block| (block.harness, block.counterexample))
                .collect::<Vec<_>>(),
            [
                ("a::check", true),
                ("b::check", false),
                ("b::check", true),
                ("c::check", false)
            ]
        );
    }
}
