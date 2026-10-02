//! Reads a real Kani concrete-playback transcript against the generator's own persisted
//! obligation schema (Linear IR-211, IR-92).
//!
//! A `cargo kani --concrete-playback print` block retains only the untyped bytes Kani handed to
//! each `kani::any()` call, in call order. This module selects the one assertion playback block
//! of a run, reads those bytes, and types them with
//! [`crate::kani::identity::KaniObligationIdentity::arguments`]: the bindings
//! `src/kani/generate/clause.rs`'s `abi()` partitions into `role: KaniBindingRole::Argument`, in the
//! exact order `symbolic_arguments` walks to emit one `kani::any()` call per binding. `render()`
//! builds `identity.arguments` from that same `abi.arguments` slice and `symbolic_arguments` is
//! called with that identical slice, so position *i* of `identity.arguments` is position *i* of
//! the emitted `kani::any()` calls, which is position *i* of the concrete bytes Kani's playback
//! records.
//!
//! The block selection, the refusal codes and the comment cross-check reproduce the behaviour of
//! the witness reader Contract IR no longer carries; the code is rewritten, the behaviour is the
//! same.
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

use crate::kani::{
    abi::{KaniBindingRole, KaniPrimitiveType},
    identity::ObligationBinding,
    output::playback::{read_block, select_assertion_block, DecodeFailure, Playback},
};

/// Why the generator's persisted argument schema could not be used to type a transcript.
#[derive(Clone, Debug, Eq, PartialEq)]
enum WitnessSchemaError {
    /// A binding in [`crate::kani::identity::KaniObligationIdentity::arguments`] is not
    /// [`KaniBindingRole::Argument`].
    ///
    /// `identity.arguments` is built by `src/kani/generate/clause.rs`'s `abi()` partitioning on
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
/// [`crate::kani::identity::KaniObligationIdentity`] this join needs; the function takes them
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
    use quire_contract_model::{IntegerDomain, OverflowPolicy};

    use super::*;
    use crate::kani::abi::KaniIntegerBounds;

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
