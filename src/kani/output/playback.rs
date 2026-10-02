//! The concrete-playback block Kani prints on the console.
//!
//! Kani's report carries no concrete playback. The falsifying playback is a Rust unit test Kani
//! prints on stdout, and [`counterexample_playback`] extracts that one fenced block from the
//! console text. It reads a payload to hand on verbatim, never a verdict: whether a run
//! falsified is decided by the report alone, and the block is looked up only after the report
//! names a failed property check.
//!
//! INTERIM: this file also holds the block scan the witness decode reads, and `DecodeFailure`,
//! the error the scan raises. Step 5 gives the scan typed entries and its own refusal and moves
//! `DecodeFailure` to `replay/witness.rs` (AD-004).

const PLAYBACK_HEADER: &str = "Concrete playback unit test";
const PLAYBACK_FENCE: &str = "```";
const PLAYBACK_ENTRY_POINT: &str = "kani::concrete_playback_run";
const PLAYBACK_COVER_MARKER: &str = "/// Check for `cover`";

/// The concrete-playback unit test Kani printed for a failed property check, verbatim, or `None`
/// when it printed none. A playback printed for a satisfied cover witnesses reachability and is
/// never returned. A fence with no closing fence ends the scan.
pub(crate) fn counterexample_playback(text: &str) -> Option<String> {
    let mut rest = text;
    while let Some(start) = rest.find(PLAYBACK_HEADER) {
        let tail = &rest[start..];
        let fence = tail.find(PLAYBACK_FENCE)?;
        let body = &tail[fence + PLAYBACK_FENCE.len()..];
        let end = body.find(PLAYBACK_FENCE)?;
        let test = body[..end].trim();
        if test.contains(PLAYBACK_ENTRY_POINT)
            && !test
                .lines()
                .any(|line| line.starts_with(PLAYBACK_COVER_MARKER))
        {
            return Some(test.to_owned());
        }
        rest = &body[end + PLAYBACK_FENCE.len()..];
    }
    None
}

const HARNESS_MARKER: &str = "/// Test generated for harness `";
const CHECK_MARKER: &str = "/// Check for `";

/// Why a transcript did not decode: the stable cause code and the identities the decoder named.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecodeFailure {
    /// Stable machine-readable cause code.
    pub code: String,
    /// The source or input identity that first caused the refusal.
    pub source_id: String,
    /// The profile and bound context of the refusal.
    pub context: String,
}

impl DecodeFailure {
    pub(crate) fn new(code: &str, source_id: &str, context: &str) -> Self {
        Self {
            code: code.to_owned(),
            source_id: source_id.to_owned(),
            context: context.to_owned(),
        }
    }
}

/// One selected assertion playback block, read.
pub(crate) struct Playback<'a> {
    pub(crate) harness: &'a str,
    pub(crate) check_text: &'a str,
    /// Kani's decoded-value comment and the untyped bytes, one per `kani::any()` call.
    pub(crate) entries: Vec<(&'a str, Vec<u8>)>,
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
) -> Result<(CheckKind, &'a str), DecodeFailure> {
    let missing = || DecodeFailure::new("kani_witness_check_missing", source_id, context);
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
) -> Result<&'a str, DecodeFailure> {
    let starts: Vec<usize> = transcript
        .match_indices(HARNESS_MARKER)
        .map(|(start, _)| start)
        .collect();
    if starts.is_empty() {
        return Err(DecodeFailure::new(
            "kani_witness_harness_missing",
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
                return Err(DecodeFailure::new(
                    "kani_witness_multiple_assertions_refused",
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
            "kani_witness_cover_refused"
        } else if saw_other {
            "kani_witness_check_kind_refused"
        } else {
            "kani_witness_check_missing"
        };
        DecodeFailure::new(code, source_id, context)
    })
}

/// Reads the harness symbol, check text and concrete entries of one assertion block.
pub(crate) fn read_block<'a>(
    block: &'a str,
    source_id: &str,
    context: &str,
) -> Result<Playback<'a>, DecodeFailure> {
    let harness = block[HARNESS_MARKER.len()..]
        .split_once('`')
        .map(|(harness, _)| harness)
        .ok_or_else(|| DecodeFailure::new("kani_witness_harness_missing", source_id, context))?;
    let (_, after_kind) = check_clause(block, source_id, context)?;
    let no_text = || DecodeFailure::new("kani_witness_check_text_missing", source_id, context);
    // Kani always prints `: "` directly after the check kind; anything else is not a check text.
    let after_quote = after_kind.strip_prefix(": \"").ok_or_else(no_text)?;
    let check_text = &after_quote[..unescaped_quote(after_quote).ok_or_else(no_text)?];
    let entries = concrete_entries(block, source_id, context)?;
    Ok(Playback {
        harness,
        check_text,
        entries,
    })
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
fn concrete_entries<'a>(
    block: &'a str,
    source_id: &str,
    context: &str,
) -> Result<Vec<(&'a str, Vec<u8>)>, DecodeFailure> {
    const VEC: &str = "vec![";
    let failure = |code| DecodeFailure::new(code, source_id, context);
    let after_marker = block
        .split_once("let concrete_vals")
        .ok_or_else(|| failure("kani_witness_concrete_vals_missing"))?
        .1;
    let outer = after_marker
        .find(VEC)
        .ok_or_else(|| failure("kani_witness_concrete_vals_missing"))?;
    let (mut body, _) = bracketed(&after_marker[outer + VEC.len() - 1..])
        .ok_or_else(|| failure("kani_witness_concrete_vals_malformed"))?;
    let mut entries = Vec::new();
    while let Some(entry) = body.find(VEC) {
        let comment = body[..entry]
            .rfind("//")
            .map(|at| {
                let scope = &body[at + 2..entry];
                scope[..scope.find('\n').unwrap_or(scope.len())].trim()
            })
            .ok_or_else(|| failure("kani_witness_comment_missing"))?;
        let (inner, rest) = bracketed(&body[entry + VEC.len() - 1..])
            .ok_or_else(|| failure("kani_witness_concrete_vals_malformed"))?;
        let bytes = inner
            .split(',')
            .map(str::trim)
            .filter(|token| !token.is_empty())
            .map(|token| {
                token
                    .parse::<u8>()
                    .map_err(|_| failure("kani_witness_byte_invalid"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        entries.push((comment, bytes));
        body = rest;
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Trace: FR-017-AC-5, FR-017-AC-12, TC-027
    #[test]
    fn tc_027_playback_scanning_returns_the_property_block_and_stops_at_an_unterminated_fence() {
        let cover = "Concrete playback unit test for `h`:\n```\n/// Check for `cover`: \"c\"\nfn t() { kani::concrete_playback_run(v, h); }\n```\n";
        let property = "Concrete playback unit test for `h`:\n```\n/// Check for `assertion`: \"a\"\nfn u() { kani::concrete_playback_run(v, h); }\n```\n";
        let no_entry = "Concrete playback unit test for `h`:\n```\nfn b() {}\n```\n";
        let unterminated =
            "Concrete playback unit test for `h`:\n```\nfn c() { kani::concrete_playback_run(v, h); }";
        assert_eq!(
            counterexample_playback(&format!("{cover}{no_entry}{property}")).as_deref(),
            Some("/// Check for `assertion`: \"a\"\nfn u() { kani::concrete_playback_run(v, h); }")
        );
        assert_eq!(counterexample_playback(&format!("{cover}{no_entry}")), None);
        assert_eq!(
            counterexample_playback(&format!("{cover}{unterminated}")),
            None
        );
        assert_eq!(counterexample_playback(""), None);
    }
}
