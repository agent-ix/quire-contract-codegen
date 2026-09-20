//! Joins a real Kani concrete-playback transcript to the generator's own persisted obligation
//! schema (Linear IR-211).
//!
//! [`quire_contract_ir::kani::Witness::decode`] needs a `&[WitnessBinding]` schema to type the
//! untyped bytes a `cargo kani --concrete-playback print` transcript retains. The generator's own
//! schema for one harness is [`crate::kani_obligations::KaniObligationIdentity::arguments`] — the
//! bindings `src/kani_obligations.rs`'s `abi()` partitions into `role: KaniBindingRole::Argument`,
//! in the exact order `symbolic_arguments` (same module) walks to emit one `kani::any()` call per
//! binding. `render()` builds `identity.arguments` from that same `abi.arguments` slice and
//! `symbolic_arguments` is called with that identical slice, so position *i* of
//! `identity.arguments` is position *i* of the emitted `kani::any()` calls, which is position *i*
//! of the concrete bytes Kani's playback records — not an assumption, but what those two call
//! sites in `src/kani_obligations.rs` are read to say.
//!
//! Before this module, nothing in this crate ever built a [`WitnessBinding`] or called
//! `Witness::decode`; every existing caller (`quire-contract-ir`'s own tests) used a hand-built
//! schema. This module is the missing join, and nothing else: it does not call an executor, and
//! it does not validate a decoded value against its IR domain or replay it natively (that is
//! `cg#50`'s remaining, `QSL#243`-blocked leg). It ends with typed values in hand.
//!
//! [`decode_falsification`] also checks the transcript's own claimed identity: [`Witness::parse`]
//! extracts a harness symbol from the transcript but never compares it to anything (`source_id` is
//! only ever used to label its own error outcomes — see `quire-contract-ir`'s `witness.rs`), so a
//! transcript from a sibling obligation over the same operation (same union ABI, hence the same
//! arity and byte widths) would otherwise decode cleanly under the wrong identity.
//! `decode_falsification` re-derives [`Witness::harness_symbol`] and refuses
//! (`cg_witness_harness_identity_mismatch`) when it disagrees with the caller's declared
//! `{module_symbol}::{harness_symbol}` — the fully-qualified Rust path, matched against exactly
//! that combined form because every harness this crate renders lives in
//! `mod {module_symbol} { fn {harness_symbol}() }` and Kani's `Test generated for harness` line
//! names a harness by its qualified path, never the module alone. There is no separate check for
//! `module_symbol` in isolation because the transcript never encodes one in isolation either; it
//! is still validated, as the qualified path's first component.

use quire_contract_ir::kani::{
    KaniOutcome, KaniOutcomeKind, Witness, WitnessBinding, WitnessValue,
};

use crate::{
    kani::{KaniBindingRole, KaniPrimitiveType},
    kani_obligations::ObligationBinding,
};

/// Why the generator's persisted argument schema could not be translated into a witness schema.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WitnessSchemaError {
    /// A binding in [`crate::kani_obligations::KaniObligationIdentity::arguments`] is not
    /// [`KaniBindingRole::Argument`].
    ///
    /// `identity.arguments` is built by `src/kani_obligations.rs`'s `abi()` partitioning on
    /// exactly this role, so every generator-emitted identity satisfies this already; this is a
    /// named refusal rather than an unwrap, panic, or silent skip, because a witness schema built
    /// from a non-argument binding would silently mistype a `kani::any()` position instead of
    /// refusing to build one at all.
    NonArgumentBinding {
        /// The offending binding's generated identifier.
        identifier: String,
    },
}

/// Maps the generator's own persisted argument schema
/// ([`crate::kani_obligations::KaniObligationIdentity::arguments`]) to the schema
/// `Witness::decode` needs, preserving
/// order exactly: position *i* of `arguments` becomes position *i* of the returned schema, which
/// is position *i* of the `kani::any()` calls `symbolic_arguments` emitted for this harness (see
/// the module documentation for why that ordering holds).
///
/// Refuses with [`WitnessSchemaError::NonArgumentBinding`] rather than including a binding this
/// module cannot confirm corresponds to a `kani::any()` call.
pub fn witness_schema(
    arguments: &[ObligationBinding],
) -> Result<Vec<WitnessBinding>, WitnessSchemaError> {
    arguments
        .iter()
        .map(|binding| {
            if binding.role != KaniBindingRole::Argument {
                return Err(WitnessSchemaError::NonArgumentBinding {
                    identifier: binding.identifier.clone(),
                });
            }
            Ok(WitnessBinding {
                identifier: binding.identifier.clone(),
                value_type: witness_value_type(binding.primitive_type),
            })
        })
        .collect()
}

/// Total, compiler-enforced mapping from the generator's Rust primitive vocabulary to the IR's
/// witness value-type vocabulary.
///
/// This match has no wildcard arm: adding a third [`KaniPrimitiveType`] variant without adding
/// the matching [`quire_contract_ir::kani::WitnessValueType`] arm fails this function to compile,
/// rather than falling through to a default or a guessed mapping. Today the two vocabularies are
/// exactly parallel (`Boolean`/`Boolean`, `I64`/`I64`); there is nothing else to map.
const fn witness_value_type(
    primitive: KaniPrimitiveType,
) -> quire_contract_ir::kani::WitnessValueType {
    use quire_contract_ir::kani::WitnessValueType;
    match primitive {
        KaniPrimitiveType::Boolean => WitnessValueType::Boolean,
        KaniPrimitiveType::I64 => WitnessValueType::I64,
    }
}

/// Decodes one real `cargo kani --concrete-playback print` transcript — an obligation harness's
/// [`crate::KaniRunOutcome::Falsified`] `counterexample` text — into typed values, using this
/// obligation's own persisted argument schema.
///
/// `harness_symbol`, `module_symbol` and `arguments` are the three fields of one
/// [`crate::kani_obligations::KaniObligationIdentity`] this join actually needs; the function
/// takes them directly rather than the whole identity so a caller holding only the generator's
/// persisted `identity.{harnessSymbol,moduleSymbol,arguments}` JSON fields (there is no
/// `Deserialize` path back to a typed `KaniObligationIdentity` — see `kani_obligations.rs`) can
/// call it without hand-building an identity it cannot fully reconstruct.
///
/// `transcript` is expected verbatim from the backend: this function neither trims caller
/// assumptions into it nor repairs a disagreement. Every failure mode is a named
/// [`KaniOutcome`]: a schema translation refusal from [`witness_schema`], a transcript that does
/// not parse as a single selected assertion playback block ([`Witness::parse`]), a harness
/// identity that does not match the caller's declared `module_symbol::harness_symbol`
/// (`cg_witness_harness_identity_mismatch`), or a decode refusal (`kani_witness_arity_mismatch`,
/// `kani_witness_width_mismatch`, `kani_witness_boolean_byte_invalid`,
/// `kani_witness_comment_mismatch`) from [`Witness::decode`] itself.
pub fn decode_falsification(
    harness_symbol: &str,
    module_symbol: &str,
    arguments: &[ObligationBinding],
    transcript: &str,
) -> Result<Vec<(String, WitnessValue)>, KaniOutcome> {
    let schema = witness_schema(arguments).map_err(|error| match error {
        WitnessSchemaError::NonArgumentBinding { identifier } => KaniOutcome::non_success(
            KaniOutcomeKind::InvalidInput,
            "cg_witness_schema_non_argument_binding",
            harness_symbol,
            identifier.as_str(),
        ),
    })?;
    let witness = Witness::parse(harness_symbol, module_symbol, transcript)?;
    // Kani names a harness by its fully-qualified Rust path in the `Test generated for harness`
    // line `Witness::harness_symbol` re-derives from — every harness this crate renders lives in
    // `mod {module_symbol} { fn {harness_symbol}() }` (`src/kani_obligations.rs`'s `render`), so
    // the qualified form is `{module_symbol}::{harness_symbol}`, not `harness_symbol` alone.
    let declared_harness_symbol = format!("{module_symbol}::{harness_symbol}");
    let observed_harness_symbol = witness.harness_symbol()?;
    if observed_harness_symbol != declared_harness_symbol {
        return Err(KaniOutcome::non_success(
            KaniOutcomeKind::Refused,
            "cg_witness_harness_identity_mismatch",
            declared_harness_symbol.as_str(),
            observed_harness_symbol.as_str(),
        ));
    }
    witness.decode(&schema)
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

    /// The mapping is total and order-preserving: every `KaniPrimitiveType` this crate emits maps
    /// to exactly one `WitnessValueType`, and the schema comes back in the same order as the
    /// generator's own `arguments`.
    #[test]
    fn witness_schema_preserves_order_and_maps_every_primitive() {
        let arguments = vec![
            argument("amount_current", KaniPrimitiveType::I64),
            argument("flag_current", KaniPrimitiveType::Boolean),
            argument("balance_pre", KaniPrimitiveType::I64),
        ];
        let schema = witness_schema(&arguments).expect("every argument binding maps");
        assert_eq!(
            schema,
            vec![
                WitnessBinding {
                    identifier: "amount_current".to_owned(),
                    value_type: quire_contract_ir::kani::WitnessValueType::I64,
                },
                WitnessBinding {
                    identifier: "flag_current".to_owned(),
                    value_type: quire_contract_ir::kani::WitnessValueType::Boolean,
                },
                WitnessBinding {
                    identifier: "balance_pre".to_owned(),
                    value_type: quire_contract_ir::kani::WitnessValueType::I64,
                },
            ]
        );
    }

    /// A `Result`-role binding refuses rather than being silently included: it does not
    /// correspond to any `kani::any()` call, so typing it would mistype a position that does not
    /// exist in the concrete bytes.
    #[test]
    fn witness_schema_refuses_a_non_argument_binding() {
        let mut result_binding = argument("post_state", KaniPrimitiveType::I64);
        result_binding.role = KaniBindingRole::Result;
        let error = witness_schema(&[result_binding]).unwrap_err();
        assert_eq!(
            error,
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

    /// `decode_falsification` is a real join, not a signature that merely compiles: given a
    /// transcript whose qualified harness symbol matches the caller's declared identity, it
    /// decodes the same value `witness_schema` + `Witness::parse` + `Witness::decode` would,
    /// called separately.
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
    /// under a different harness's identity. `Witness::parse`/`Witness::decode` are purely
    /// positional and never compare the harness symbol a caller declares against the one the
    /// transcript actually names (see the module doc) — `decode_falsification` is the join that
    /// adds that comparison, so this is the only place in the crate that can refuse it.
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
}
