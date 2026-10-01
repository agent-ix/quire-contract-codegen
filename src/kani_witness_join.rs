//! Reads a real Kani concrete-playback transcript against the generator's own persisted
//! obligation schema (Linear IR-211, IR-92).
//!
//! A `cargo kani --concrete-playback print` block retains only the untyped bytes Kani handed to
//! each `kani::any()` call, in call order. This module selects the one assertion playback block
//! of a run, reads those bytes, and types them with
//! [`crate::kani_obligations::KaniObligationIdentity::arguments`]: the bindings
//! `src/kani_obligations.rs`'s `abi()` partitions into `role: KaniBindingRole::Argument`, in the
//! exact order `symbolic_arguments` walks to emit one `kani::any()` call per binding. `render()`
//! builds `identity.arguments` from that same `abi.arguments` slice and `symbolic_arguments` is
//! called with that identical slice, so position *i* of `identity.arguments` is position *i* of
//! the emitted `kani::any()` calls, which is position *i* of the concrete bytes Kani's playback
//! records.
//!
//! The block selection, the refusal codes and the comment cross-check are a behavioural port of
//! `quire_contract_ir::kani::Witness`, which IR deletes; the code is rewritten, the behaviour is
//! the same.
//!
//! The values come out as [`qsl_replay::WitnessValue`], the type QSL's replay envelope carries,
//! so [`crate::spine_replay`] hands them to QSL without a conversion. The transcript grammar
//! read here is Kani's; QSL's `Witness::decode` reads QSL's own transcript grammar and cannot
//! read this one, so the playback bytes are decoded here.
//!
//! [`decode_falsification`] also checks the transcript's own claimed identity: a transcript from
//! a sibling obligation over the same operation (same union ABI, hence the same arity and byte
//! widths) would otherwise decode cleanly under the wrong identity. It refuses
//! (`cg_witness_harness_identity_mismatch`) when the harness the transcript names is not the
//! caller's declared `{module_symbol}::{harness_symbol}`: every harness this crate renders lives
//! in `mod {module_symbol} { fn {harness_symbol}() }` and Kani's `Test generated for harness`
//! line names a harness by its qualified path.
//!
//! It does not validate a decoded value against its IR domain ([`first_out_of_domain`] does) or
//! replay it ([`crate::spine_replay`] does).

use qsl_replay::WitnessValue;

use crate::{
    kani::{KaniBindingRole, KaniPrimitiveType},
    kani_obligations::ObligationBinding,
};

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
    fn new(code: &str, source_id: &str, context: &str) -> Self {
        Self {
            code: code.to_owned(),
            source_id: source_id.to_owned(),
            context: context.to_owned(),
        }
    }
}

/// Why the generator's persisted argument schema could not be used to type a transcript.
#[derive(Clone, Debug, Eq, PartialEq)]
enum WitnessSchemaError {
    /// A binding in [`crate::kani_obligations::KaniObligationIdentity::arguments`] is not
    /// [`KaniBindingRole::Argument`].
    ///
    /// `identity.arguments` is built by `src/kani_obligations.rs`'s `abi()` partitioning on
    /// exactly this role, so every generator-emitted identity satisfies this already; this is a
    /// named refusal rather than an unwrap, panic, or silent skip, because a schema built from a
    /// non-argument binding would silently mistype a `kani::any()` position instead of refusing
    /// to build one at all.
    NonArgumentBinding {
        /// The offending binding's generated identifier.
        identifier: String,
    },
}

/// The argument identifiers and primitive types of `arguments`, in order, refusing with
/// [`WitnessSchemaError::NonArgumentBinding`] rather than including a binding that does not
/// correspond to a `kani::any()` call.
fn argument_types(
    arguments: &[ObligationBinding],
) -> Result<Vec<(&str, KaniPrimitiveType)>, WitnessSchemaError> {
    arguments
        .iter()
        .map(|binding| {
            if binding.role == KaniBindingRole::Argument {
                Ok((binding.identifier.as_str(), binding.primitive_type))
            } else {
                Err(WitnessSchemaError::NonArgumentBinding {
                    identifier: binding.identifier.clone(),
                })
            }
        })
        .collect()
}

/// The exact byte width Kani's concrete playback encodes for `primitive`.
///
/// The match has no wildcard arm: a third [`KaniPrimitiveType`] variant fails this function to
/// compile instead of guessing a width.
const fn byte_width(primitive: KaniPrimitiveType) -> usize {
    match primitive {
        KaniPrimitiveType::Boolean => 1,
        KaniPrimitiveType::I64 => 8,
    }
}

/// Decodes one real `cargo kani --concrete-playback print` transcript -- an obligation harness's
/// [`crate::KaniRunOutcome::Falsified`] `counterexample` text -- into typed values, using this
/// obligation's own persisted argument schema.
///
/// `harness_symbol`, `module_symbol` and `arguments` are the three fields of one
/// [`crate::kani_obligations::KaniObligationIdentity`] this join needs; the function takes them
/// directly so a caller holding only the persisted `identity.{harnessSymbol,moduleSymbol,
/// arguments}` JSON fields can call it.
///
/// `transcript` is expected verbatim from the backend. Every failure is a named
/// [`DecodeFailure`]: `cg_witness_schema_non_argument_binding`; a transcript that is not a
/// single selected assertion playback block (`kani_witness_harness_missing`,
/// `kani_witness_check_missing`, `kani_witness_check_text_missing`,
/// `kani_witness_cover_refused`, `kani_witness_check_kind_refused`,
/// `kani_witness_multiple_assertions_refused`, `kani_witness_concrete_vals_missing`,
/// `kani_witness_concrete_vals_malformed`, `kani_witness_comment_missing`,
/// `kani_witness_byte_invalid`); a harness that is not the declared one
/// (`cg_witness_harness_identity_mismatch`); or bytes that do not fit the schema
/// (`kani_witness_arity_mismatch`, `kani_witness_width_mismatch`,
/// `kani_witness_boolean_byte_invalid`, `kani_witness_comment_mismatch`).
///
/// # Errors
///
/// The [`DecodeFailure`] named above.
pub fn decode_falsification(
    harness_symbol: &str,
    module_symbol: &str,
    arguments: &[ObligationBinding],
    transcript: &str,
) -> Result<Vec<(String, WitnessValue)>, DecodeFailure> {
    let schema = argument_types(arguments).map_err(|error| match error {
        WitnessSchemaError::NonArgumentBinding { identifier } => DecodeFailure::new(
            "cg_witness_schema_non_argument_binding",
            harness_symbol,
            &identifier,
        ),
    })?;
    let block = select_assertion_block(transcript, harness_symbol, module_symbol)?;
    let playback = read_block(block, harness_symbol, module_symbol)?;
    let declared = format!("{module_symbol}::{harness_symbol}");
    if playback.harness != declared {
        return Err(DecodeFailure::new(
            "cg_witness_harness_identity_mismatch",
            &declared,
            playback.harness,
        ));
    }
    decode_values(&playback, &schema)
}

/// One selected assertion playback block, read.
struct Playback<'a> {
    harness: &'a str,
    check_text: &'a str,
    /// Kani's decoded-value comment and the untyped bytes, one per `kani::any()` call.
    entries: Vec<(&'a str, Vec<u8>)>,
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
fn select_assertion_block<'a>(
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
fn read_block<'a>(
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

/// Joins the untyped entries with the schema, cross-checking every value against Kani's own
/// decoded-value comment. A Boolean is never inferred from "any nonzero byte": Kani encodes
/// `bool` as exactly `0` or `1`.
fn decode_values(
    playback: &Playback<'_>,
    schema: &[(&str, KaniPrimitiveType)],
) -> Result<Vec<(String, WitnessValue)>, DecodeFailure> {
    let source = playback.harness;
    if playback.entries.len() != schema.len() {
        return Err(DecodeFailure::new(
            "kani_witness_arity_mismatch",
            source,
            playback.check_text,
        ));
    }
    schema
        .iter()
        .zip(&playback.entries)
        .map(|(&(identifier, primitive), (comment, bytes))| {
            let failure = |code| DecodeFailure::new(code, source, identifier);
            if bytes.len() != byte_width(primitive) {
                return Err(failure("kani_witness_width_mismatch"));
            }
            let (value, comment_agrees) = match (primitive, bytes.as_slice()) {
                (KaniPrimitiveType::Boolean, [byte @ (0 | 1)]) => {
                    let value = *byte == 1;
                    (
                        WitnessValue::Boolean(value),
                        boolean_comment(comment) == Some(value),
                    )
                }
                (KaniPrimitiveType::Boolean, _) => {
                    return Err(failure("kani_witness_boolean_byte_invalid"))
                }
                (KaniPrimitiveType::I64, _) => {
                    let mut buffer = [0u8; 8];
                    buffer.copy_from_slice(bytes);
                    let value = i64::from_le_bytes(buffer);
                    (
                        WitnessValue::Integer(value),
                        comment.parse::<i64>().is_ok_and(|parsed| parsed == value),
                    )
                }
            };
            if !comment_agrees {
                return Err(failure("kani_witness_comment_mismatch"));
            }
            Ok((identifier.to_owned(), value))
        })
        .collect()
}

/// Kani's decoded-value comment for a Boolean: `true`/`1` and `false`/`0` in any case.
fn boolean_comment(comment: &str) -> Option<bool> {
    match comment.to_ascii_lowercase().as_str() {
        "true" | "1" => Some(true),
        "false" | "0" => Some(false),
        _ => None,
    }
}

/// The first decoded value outside its argument's declared integer domain, as the argument's
/// name. A Boolean has no domain narrower than its type, and a value no binding names is left to
/// the replay adapter to refuse.
pub(crate) fn first_out_of_domain<'a>(
    arguments: &[ObligationBinding],
    values: &'a [(String, WitnessValue)],
) -> Option<&'a str> {
    values.iter().find_map(|(name, value)| {
        let bounds = arguments
            .iter()
            .find(|binding| binding.identifier == *name)?
            .integer_bounds
            .as_ref()?;
        match value {
            WitnessValue::Integer(integer)
                if !(bounds.minimum..=bounds.maximum).contains(integer) =>
            {
                Some(name.as_str())
            }
            WitnessValue::Integer(_) | WitnessValue::Boolean(_) => None,
        }
    })
}

#[cfg(test)]
mod tests {
    use quire_contract_ir::{IntegerDomain, OverflowPolicy};

    use super::*;
    use crate::kani::KaniIntegerBounds;

    fn argument(identifier: &str, primitive_type: KaniPrimitiveType) -> ObligationBinding {
        ObligationBinding {
            identifier: identifier.to_owned(),
            role: KaniBindingRole::Argument,
            primitive_type,
            integer_bounds: match primitive_type {
                KaniPrimitiveType::Boolean => None,
                KaniPrimitiveType::I64 => Some(KaniIntegerBounds {
                    domain: IntegerDomain::Signed,
                    minimum: 0,
                    maximum: 1000,
                    overflow: OverflowPolicy::Reject,
                }),
            },
            dependencies: Vec::new(),
        }
    }

    /// The schema is order-preserving: every argument keeps its identifier and primitive type, in
    /// the generator's own `arguments` order.
    ///
    /// Trace: FR-016-AC-8, TC-026
    #[test]
    fn argument_types_preserve_order_and_type() {
        let arguments = vec![
            argument("amount_current", KaniPrimitiveType::I64),
            argument("flag_current", KaniPrimitiveType::Boolean),
            argument("balance_pre", KaniPrimitiveType::I64),
        ];
        assert_eq!(
            argument_types(&arguments).expect("every argument binding maps"),
            vec![
                ("amount_current", KaniPrimitiveType::I64),
                ("flag_current", KaniPrimitiveType::Boolean),
                ("balance_pre", KaniPrimitiveType::I64),
            ]
        );
    }

    /// A `Result`-role binding refuses rather than being silently included: it does not
    /// correspond to any `kani::any()` call, so typing it would mistype a position that does not
    /// exist in the concrete bytes.
    ///
    /// Trace: FR-016-AC-8, TC-026
    #[test]
    fn argument_types_refuse_a_non_argument_binding() {
        let mut result_binding = argument("post_state", KaniPrimitiveType::I64);
        result_binding.role = KaniBindingRole::Result;
        assert_eq!(
            argument_types(&[result_binding]).unwrap_err(),
            WitnessSchemaError::NonArgumentBinding {
                identifier: "post_state".to_owned(),
            }
        );
    }

    /// A minimal, hand-built `cargo kani --concrete-playback print` transcript naming
    /// `qualified_harness_symbol` (the `{module}::{harness}` path Kani actually records — see the
    /// module doc) and encoding one `i64` concrete value, for exercising `decode_falsification`
    /// without a real Kani run.
    fn synthetic_transcript(qualified_harness_symbol: &str, value: i64) -> String {
        let bytes = value
            .to_le_bytes()
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "/// Test generated for harness `{qualified_harness_symbol}`\n\
/// Check for `assertion`: \"synthetic assertion\"\n\
#[test]\n\
fn kani_concrete_playback_synthetic() {{\n\
    let concrete_vals: Vec<Vec<u8>> = vec![\n\
        // {value}\n\
        vec![{bytes}],\n\
    ];\n\
    kani::concrete_playback_run(concrete_vals, synthetic);\n\
}}\n"
        )
    }

    /// Like [`synthetic_transcript`], with a second concrete value: an `i64` followed by a
    /// `Boolean`, each preceded by Kani's own decoded-value comment.
    fn two_value_transcript(qualified_harness_symbol: &str, first: i64, second: bool) -> String {
        let bytes = first
            .to_le_bytes()
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "/// Test generated for harness `{qualified_harness_symbol}`\n\
/// Check for `assertion`: \"synthetic assertion\"\n\
#[test]\n\
fn kani_concrete_playback_synthetic() {{\n\
    let concrete_vals: Vec<Vec<u8>> = vec![\n\
        // {first}\n\
        vec![{bytes}],\n\
        // {second}\n\
        vec![{flag}],\n\
    ];\n\
    kani::concrete_playback_run(concrete_vals, synthetic);\n\
}}\n",
            flag = u8::from(second),
        )
    }

    /// A transcript whose value count or byte width disagrees with the
    /// persisted schema refuses by name rather than decoding: one `i64`
    /// value against a two-argument schema is an arity mismatch, and against
    /// a one-`Boolean` schema is a width mismatch.
    ///
    /// Trace: TC-026
    #[test]
    fn decode_falsification_refuses_arity_and_width_mismatches() {
        let transcript = synthetic_transcript("mod::harness_ok", 42);
        let arity = decode_falsification(
            "harness_ok",
            "mod",
            &[
                argument("first", KaniPrimitiveType::I64),
                argument("second", KaniPrimitiveType::I64),
            ],
            &transcript,
        )
        .expect_err("one value cannot decode against two arguments");
        assert_eq!(arity.code, "kani_witness_arity_mismatch", "{arity:?}");
        let width = decode_falsification(
            "harness_ok",
            "mod",
            &[argument("flag", KaniPrimitiveType::Boolean)],
            &transcript,
        )
        .expect_err("eight bytes cannot decode as a Boolean");
        assert_eq!(width.code, "kani_witness_width_mismatch", "{width:?}");
    }

    /// `decode_falsification` is a real join, not a signature that merely compiles: given a
    /// transcript whose qualified harness symbol matches the caller's declared identity, it
    /// decodes the value the transcript's bytes encode.
    ///
    /// Trace: TC-026
    #[test]
    fn decode_falsification_decodes_a_matching_transcript() {
        let arguments = vec![argument("value", KaniPrimitiveType::I64)];
        let transcript = synthetic_transcript("mod::harness_ok", 42);
        let decoded = decode_falsification("harness_ok", "mod", &arguments, &transcript)
            .expect("a transcript whose qualified harness symbol matches the identity decodes");
        assert_eq!(
            decoded,
            vec![("value".to_owned(), WitnessValue::Integer(42))]
        );
    }

    /// MUTATE (both directions): a transcript recorded for one harness must not decode silently
    /// under a different harness's identity. The block reader and the byte decoder are
    /// purely positional and never compare the harness symbol a caller declares against the one
    /// the transcript actually names — `decode_falsification` adds that comparison, so this is
    /// the only place in the crate that can refuse it.
    ///
    /// Trace: TC-026
    #[test]
    fn decode_falsification_refuses_a_transcript_whose_harness_symbol_disagrees() {
        let arguments = vec![argument("value", KaniPrimitiveType::I64)];
        let transcript = synthetic_transcript("mod::harness_ok", 42);

        // direction 1: the declared identity names the transcript's real harness -> decodes.
        decode_falsification("harness_ok", "mod", &arguments, &transcript)
            .expect("the matching direction must still decode");

        // direction 2: the declared identity names a sibling harness in the same module, over
        // the same union ABI (same arity and widths) -> must refuse by name, not decode under
        // the wrong label.
        let error = decode_falsification("sibling_harness", "mod", &arguments, &transcript)
            .expect_err("a wrong-harness transcript must refuse, not decode silently");
        assert_eq!(
            error.code, "cg_witness_harness_identity_mismatch",
            "{error:?}"
        );
        assert_eq!(error.source_id, "mod::sibling_harness");
        assert_eq!(error.context, "mod::harness_ok");
    }

    /// Naming is by position with two or more values: the first decoded value takes the first
    /// binding's identifier and the second takes the second's, so a swapped or shifted pairing
    /// changes the result. A `Boolean` is placed after an `i64` so the two positions also differ
    /// in width and cannot be confused by value alone.
    ///
    /// Trace: FR-016-AC-8, TC-026
    #[test]
    fn decode_falsification_names_each_value_by_its_binding_position() {
        let arguments = vec![
            argument("amount", KaniPrimitiveType::I64),
            argument("flag", KaniPrimitiveType::Boolean),
        ];
        let transcript = two_value_transcript("mod::harness_ok", 42, true);
        let decoded = decode_falsification("harness_ok", "mod", &arguments, &transcript)
            .expect("a two-value transcript decodes against a two-binding schema");
        assert_eq!(
            decoded,
            vec![
                ("amount".to_owned(), WitnessValue::Integer(42)),
                ("flag".to_owned(), WitnessValue::Boolean(true)),
            ]
        );
    }

    /// The integer domain is inclusive at both ends: the minimum and maximum are in domain, and
    /// the values one past either end and the extremes of `i64` are not. A value with no
    /// bounded binding, and a Boolean, are never out of domain.
    ///
    /// Trace: FR-016-AC-2, TC-026
    #[test]
    fn first_out_of_domain_is_inclusive_at_both_bounds() {
        let arguments = vec![argument("amount", KaniPrimitiveType::I64)];
        let check = |value: i64| {
            first_out_of_domain(
                &arguments,
                &[("amount".to_owned(), WitnessValue::Integer(value))],
            )
            .map(str::to_owned)
        };
        for inside in [0, 1, 999, 1000] {
            assert_eq!(check(inside), None, "{inside}");
        }
        for outside in [-1, 1001, i64::MIN, i64::MAX] {
            assert_eq!(check(outside).as_deref(), Some("amount"), "{outside}");
        }
        assert_eq!(
            first_out_of_domain(
                &arguments,
                &[
                    ("unbound".to_owned(), WitnessValue::Integer(i64::MAX)),
                    ("amount".to_owned(), WitnessValue::Integer(5)),
                ]
            ),
            None
        );
        let flag = vec![argument("flag", KaniPrimitiveType::Boolean)];
        assert_eq!(
            first_out_of_domain(&flag, &[("flag".to_owned(), WitnessValue::Boolean(true))]),
            None
        );
    }

    /// Kani's own `//` comment is a cross-check on the bytes, and a Boolean is exactly `0` or `1`:
    /// a value whose comment disagrees, and a Boolean byte of `2`, each refuse by name.
    ///
    /// Trace: TC-026
    #[test]
    fn decode_falsification_refuses_a_disagreeing_comment_and_an_invalid_boolean_byte() {
        let block = |comment: &str, byte: u8| {
            format!(
                "/// Test generated for harness `mod::h`\n\
/// Check for `assertion`: \"a\"\n\
#[test]\nfn t() {{\n    let concrete_vals: Vec<Vec<u8>> = vec![\n        // {comment}\n        vec![{byte}],\n    ];\n}}\n"
            )
        };
        let flag = [argument("flag", KaniPrimitiveType::Boolean)];
        let decode = |comment: &str, byte: u8| {
            decode_falsification("h", "mod", &flag, &block(comment, byte))
        };
        assert_eq!(
            decode("true", 1).expect("agreeing comment"),
            vec![("flag".to_owned(), WitnessValue::Boolean(true))]
        );
        assert_eq!(
            decode("false", 1).unwrap_err().code,
            "kani_witness_comment_mismatch"
        );
        assert_eq!(
            decode("true", 2).unwrap_err().code,
            "kani_witness_boolean_byte_invalid"
        );
    }

    /// Only a single assertion block is a witness: a run holding only a cover block, or two
    /// assertion blocks, refuses rather than picking one; a cover beside one assertion selects
    /// the assertion.
    ///
    /// Trace: TC-026
    #[test]
    fn decode_falsification_selects_exactly_one_assertion_block() {
        let block = |kind: &str, value: i64| {
            synthetic_transcript("mod::h", value).replace("`assertion`", &format!("`{kind}`"))
        };
        let argument = [argument("value", KaniPrimitiveType::I64)];
        let decode = |transcript: String| decode_falsification("h", "mod", &argument, &transcript);
        assert_eq!(
            decode(block("cover", 1)).unwrap_err().code,
            "kani_witness_cover_refused"
        );
        assert_eq!(
            decode(block("assertion", 1) + &block("assertion", 2))
                .unwrap_err()
                .code,
            "kani_witness_multiple_assertions_refused"
        );
        assert_eq!(
            decode(block("cover", 1) + &block("assertion", 7)).expect("one assertion block"),
            vec![("value".to_owned(), WitnessValue::Integer(7))]
        );
    }

    /// A contract harness's check text spans several lines; the closing quote is not on the
    /// `Check for` line. It decodes, and its check text is the whole multi-line text.
    ///
    /// Trace: FR-016-AC-1, TC-026
    #[test]
    fn decode_falsification_reads_a_multi_line_check_text() {
        let transcript = synthetic_transcript("mod::h", 42).replace(
            "\"synthetic assertion\"",
            "\"first line\n///   second line\n///   third line\"",
        );
        let block = read_block(&transcript, "h", "mod").expect("a multi-line check text reads");
        assert_eq!(
            block.check_text,
            "first line\n///   second line\n///   third line"
        );
        assert_eq!(
            decode_falsification(
                "h",
                "mod",
                &[argument("v", KaniPrimitiveType::I64)],
                &transcript
            )
            .expect("decodes"),
            vec![("v".to_owned(), WitnessValue::Integer(42))]
        );
    }

    /// Negative and extreme integers decode from their little-endian bytes.
    ///
    /// Trace: TC-026
    #[test]
    fn decode_falsification_decodes_negative_and_extreme_integers() {
        for value in [-1, i64::MIN, i64::MAX] {
            let transcript = synthetic_transcript("mod::h", value);
            assert_eq!(
                decode_falsification(
                    "h",
                    "mod",
                    &[argument("v", KaniPrimitiveType::I64)],
                    &transcript
                )
                .expect("decodes"),
                vec![("v".to_owned(), WitnessValue::Integer(value))]
            );
        }
    }

    /// Every remaining refusal names its own code: each malformed transcript below breaks
    /// exactly one rule, so a check that is relaxed, swallowed or re-coded turns one case red.
    ///
    /// Trace: TC-026
    #[test]
    fn decode_falsification_refuses_each_malformed_transcript_by_code() {
        let good = synthetic_transcript("mod::h", 42);
        let one = [argument("v", KaniPrimitiveType::I64)];
        let code = |transcript: &str, arguments: &[ObligationBinding]| {
            decode_falsification("h", "mod", arguments, transcript)
                .expect_err(&format!("must refuse: {transcript}"))
                .code
        };
        let cases: Vec<(String, &str)> = vec![
            ("no playback".to_owned(), "kani_witness_harness_missing"),
            (
                good.replace("/// Check for `assertion`", "/// no clause"),
                "kani_witness_check_missing",
            ),
            (
                good.replace("`assertion`", "`other`"),
                "kani_witness_check_kind_refused",
            ),
            (
                good.replace("\"synthetic assertion\"", "no quoted text"),
                "kani_witness_check_text_missing",
            ),
            (
                good.replace("\"synthetic assertion\"", "\"unterminated"),
                "kani_witness_check_text_missing",
            ),
            (
                good.replace("`assertion`: \"synthetic", "`assertion` \"synthetic")
                    .replace(
                        "concrete_vals, synthetic)",
                        "concrete_vals, \"later quote\")",
                    ),
                "kani_witness_check_text_missing",
            ),
            (
                good.replace("let concrete_vals", "let other_vals"),
                "kani_witness_concrete_vals_missing",
            ),
            (
                good.replace("vec![\n// 42", "vec![\n// 42\nvec![1, 2"),
                "kani_witness_concrete_vals_malformed",
            ),
            (good.replace("// 42\n", ""), "kani_witness_comment_missing"),
            (
                good.replace(", 0, 0, 0]", ", 999, 0]"),
                "kani_witness_byte_invalid",
            ),
            (
                good.replace("0, 0, 0, 0, 0, 0, 0]", "0, 0, 0, 0, 0, 0]"),
                "kani_witness_width_mismatch",
            ),
        ];
        for (transcript, expected) in &cases {
            assert_eq!(code(transcript, &one), *expected, "{transcript}");
        }
        let mut result_binding = argument("post_state", KaniPrimitiveType::I64);
        result_binding.role = KaniBindingRole::Result;
        assert_eq!(
            code(&good, &[result_binding]),
            "cg_witness_schema_non_argument_binding"
        );
    }
}
