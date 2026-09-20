//! Integrated bounded-Kani corpus generation over Contract IR's validated finite ABI.
//!
//! The Contract IR lowerers remain the semantic authority.  This module owns the codegen-side
//! vertical slice: once one lowering is admitted, it renders the four corpus roles from the same
//! profile selection and finite input.  A non-success outcome returns before any role is emitted.

use std::{collections::BTreeMap, fmt::Write as _};

use quire_contract_ir::kani::{
    CheckedArithmeticRequest, CollectionQuery, CounterexamplePacket, DispatchIndex, GraphRequest,
    KaniOutcome, KaniOutcomeKind, KaniProfile, ReplaySource, ValidatedFiniteInput, WitnessValue,
};
use sha2::{Digest as _, Sha256};

use crate::{
    prepare_bounded_collection_query, prepare_checked_arithmetic, prepare_finite_graph_reaches,
    Artifact,
};

/// One semantic family represented in the bounded corpus.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundedCorpusFamily {
    /// Checked arithmetic and its definedness boundary.
    DefinednessArithmetic,
    /// Identity-preserving positive-length reachability.
    FiniteReferenceGraph,
    /// Ordered, duplicate-preserving finite collection queries.
    BoundedCollection,
}

impl BoundedCorpusFamily {
    const fn construct(self) -> &'static str {
        match self {
            Self::DefinednessArithmetic => "checked-arithmetic",
            Self::FiniteReferenceGraph => "finite-reference-graph",
            Self::BoundedCollection => "bounded-collection-query",
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::DefinednessArithmetic => "arithmetic",
            Self::FiniteReferenceGraph => "graph",
            Self::BoundedCollection => "collection",
        }
    }
}

/// One exact semantic request selected for corpus generation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BoundedCorpusRequest {
    /// A checked arithmetic operation.
    Arithmetic(CheckedArithmeticRequest),
    /// A finite positive-length reachability query.
    Graph(GraphRequest),
    /// An ordered finite collection query.
    Collection(CollectionQuery),
}

impl BoundedCorpusRequest {
    const fn family(&self) -> BoundedCorpusFamily {
        match self {
            Self::Arithmetic(_) => BoundedCorpusFamily::DefinednessArithmetic,
            Self::Graph(_) => BoundedCorpusFamily::FiniteReferenceGraph,
            Self::Collection(_) => BoundedCorpusFamily::BoundedCollection,
        }
    }

    fn source_id(&self) -> &str {
        match self {
            Self::Arithmetic(request) => request.source_id,
            Self::Graph(request) => &request.source_id,
            Self::Collection(request) => &request.source_id,
        }
    }
}

/// Deterministic generated artifacts for one admitted finite corpus case.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedCorpusArtifacts {
    /// Boolean executable oracle artifact.
    pub oracle: Artifact,
    /// Finite, directly shaped strategy artifact.
    pub strategy: Artifact,
    /// Kani harness artifact with no input-erasing assumptions.
    pub kani_harness: Artifact,
    /// Provenance and proof-dependency record for this exact selection.
    pub provenance: Artifact,
}

/// Complete codegen result for one supported corpus case.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedCorpusCase {
    /// Semantic family selected before lowering.
    pub family: BoundedCorpusFamily,
    /// Exact Contract IR outcome shared by native/oracle/strategy/Kani case execution.
    pub outcome: KaniOutcome,
    /// Generated roles derived from the same profile and finite input selection.
    pub artifacts: BoundedCorpusArtifacts,
    /// Retained false counterexample, when the admitted case is a counterexample. This corpus
    /// never runs Kani, so it never holds a backend transcript: its packet's `source` is always
    /// `ReplaySource::Input`, carrying the canonical assignments that produced the false result,
    /// never a fabricated `ReplaySource::Witness`.
    pub counterexample: Option<CounterexamplePacket>,
}

/// Generates the complete codegen corpus case from one already validated Contract IR input.
///
/// Contract IR performs profile/dispatch/finite-population validation and semantic lowering first.
/// Consequently a refused, invalid, incomplete, or exhausted case returns its original typed
/// outcome and this function emits no artifact.  Generated Kani source uses a concrete case and
/// intentionally contains no `kani::assume` call.
pub fn generate_bounded_kani_corpus_case(
    profile: &KaniProfile,
    dispatch: &DispatchIndex,
    input: &ValidatedFiniteInput,
    request: BoundedCorpusRequest,
) -> Result<BoundedCorpusCase, KaniOutcome> {
    if profile.selection != input.input().profile {
        return Err(KaniOutcome::non_success(
            KaniOutcomeKind::InvalidInput,
            "kani_profile_input_mismatch",
            request.source_id(),
            profile.selection.revision.clone(),
        ));
    }
    let family = request.family();
    let request_source_id = request.source_id().to_owned();
    let revision = profile.selection.revision.clone();
    // Each arm collects only the *raw* (fallible-conversion-free) per-case assignment data here.
    // Converting a raw `i128` into the closed `WitnessValue` wire representation is fallible
    // (`integer_assignment`, below), and a provable case must never be refused over an assignment
    // that no packet will ever carry: conversion happens later, only inside the `!value` branch
    // that actually builds a retained counterexample packet.
    let (value, detail, oracle_body, raw_assignments) = match request {
        BoundedCorpusRequest::Arithmetic(request) => {
            let lowered = prepare_checked_arithmetic(profile, dispatch, input, request)?;
            let raw_assignments = arithmetic_assignments(&lowered.request);
            // Admission establishes the checked arithmetic/definedness property; the numeric
            // result itself is not a Boolean verdict (zero is as valid as any other in-range
            // result).
            (
                true,
                format!("value={}", lowered.value),
                render_arithmetic_oracle(&lowered),
                raw_assignments,
            )
        }
        BoundedCorpusRequest::Graph(request) => {
            let lowered = prepare_finite_graph_reaches(profile, dispatch, input, request)?;
            let raw_assignments = graph_assignments(&lowered.request);
            (
                lowered.reachable,
                format!("expanded={}", lowered.expanded.join(",")),
                render_graph_oracle(&lowered, input),
                raw_assignments,
            )
        }
        BoundedCorpusRequest::Collection(request) => {
            let lowered = prepare_bounded_collection_query(profile, dispatch, input, request)?;
            let values = lowered
                .query
                .values
                .iter()
                .map(|value| format!("{value}i128"))
                .collect::<Vec<_>>()
                .join(", ");
            let predicate = match lowered.query.kind {
                quire_contract_ir::kani::QueryKind::ForAllNonNegative => {
                    "values.iter().all(|value| *value >= 0i128)".to_owned()
                }
                quire_contract_ir::kani::QueryKind::ExistsEqual(expected) => {
                    format!("values.iter().any(|value| *value == {expected}i128)")
                }
            };
            let raw_assignments = collection_assignments(&lowered.query);
            (
                lowered.value,
                format!("examined={}", lowered.examined),
                format!("{{ let values = [{values}]; {predicate} }}"),
                raw_assignments,
            )
        }
    };
    let outcome = if value {
        KaniOutcome::proved(
            request_source_id.clone(),
            profile.selection.revision.clone(),
        )
    } else {
        KaniOutcome::counterexample(
            request_source_id.clone(),
            profile.selection.revision.clone(),
        )
    };
    let identity = digest(&format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        family.label(),
        family.construct(),
        profile.selection.revision,
        profile.selection.executable_digest,
        profile.selection.options_digest,
        detail,
    ));
    let artifacts = render_artifacts(
        family,
        &identity,
        value,
        &detail,
        &oracle_body,
        profile,
        input,
    );
    // Fallible conversion to `WitnessValue` runs only here, for a case that actually retains a
    // packet: a provable case's `raw_assignments` are dropped unconverted, so an admitted value
    // outside `i64`'s range can never refuse generation of a case no packet will carry.
    let counterexample = (!value)
        .then(|| build_assignments(&request_source_id, &revision, raw_assignments))
        .transpose()?
        .map(|assignments| CounterexamplePacket {
            profile_revision: profile.selection.revision.clone(),
            input: input.input().clone(),
            // This corpus is generated from a concrete, already-admitted finite selection and
            // never runs Kani, so it never has a backend transcript to build a `Witness` from.
            // `identity` (the artifact digest, computed above and unrelated to what produced this
            // counterexample) and a string literal were both refused as fabrications by ir#156;
            // `assignments` is the real per-family concrete input this case was generated from
            // (see the match above).
            source: ReplaySource::Input(assignments),
        });
    Ok(BoundedCorpusCase {
        family,
        outcome,
        artifacts,
        counterexample,
    })
}

/// The checked-arithmetic family's raw per-case input assignments: exactly its two operands,
/// keyed by declared parameter identifier (AD-016 "Replay source"). `minimum` and `maximum` are
/// the checked property's own domain, fixed by the request definition rather than a concrete
/// value drawn for this instance, so neither belongs here.
fn arithmetic_assignments(request: &CheckedArithmeticRequest) -> Vec<(String, i128)> {
    vec![
        ("left".to_owned(), request.left),
        ("right".to_owned(), request.right),
    ]
}

/// The finite-reference-graph family's raw per-case input assignments.
fn graph_assignments(request: &GraphRequest) -> Vec<(String, i128)> {
    // Widening, not narrowing: `usize` never exceeds `i128` on any supported target.
    vec![("max_expansions".to_owned(), request.max_expansions as i128)]
}

/// The bounded-collection-query family's raw per-case input assignments: exactly the ordered
/// population's own concrete values, keyed by declared parameter identifier (AD-016 "Replay
/// source"). `max_items` bounds the property's domain and `expected` (for `ExistsEqual`) is the
/// query's own oracle target; both are fixed by the request definition, not a concrete value
/// drawn for this instance, so neither belongs here.
fn collection_assignments(query: &CollectionQuery) -> Vec<(String, i128)> {
    query
        .values
        .iter()
        .enumerate()
        .map(|(index, item)| (format!("value_{index}"), *item))
        .collect()
}

fn render_arithmetic_oracle(lowered: &quire_contract_ir::kani::ArithmeticLowering) -> String {
    let operator = match lowered.request.operator {
        quire_contract_ir::NumericOperator::Add => "checked_add",
        quire_contract_ir::NumericOperator::Subtract => "checked_sub",
        quire_contract_ir::NumericOperator::Multiply => "checked_mul",
        quire_contract_ir::NumericOperator::Divide => "checked_div",
        quire_contract_ir::NumericOperator::Remainder => "checked_rem",
    };
    format!(
        "{}i128.{operator}({}i128).is_some_and(|value| value >= {}i128 && value <= {}i128)",
        lowered.request.left,
        lowered.request.right,
        lowered.request.minimum,
        lowered.request.maximum,
    )
}

fn render_graph_oracle(
    lowered: &quire_contract_ir::kani::GraphLowering,
    input: &ValidatedFiniteInput,
) -> String {
    let mut edges = input
        .input()
        .references
        .iter()
        .filter(|edge| edge.field_id == lowered.request.field_id)
        .map(|edge| format!("({:?}, {:?})", edge.source_id, edge.target_id))
        .collect::<Vec<_>>();
    edges.sort();
    format!(
        "{{ let edges: &[(&str, &str)] = &[{}]; let mut visited: Vec<&str> = Vec::new(); \
         let mut frontier = vec![{:?}]; while let Some(current) = frontier.pop() {{ \
         if visited.iter().any(|known| *known == current) {{ continue; }} \
         if visited.len() == {} {{ return false; }} visited.push(current); \
         for (source, target) in edges.iter().rev() {{ if *source == current {{ \
         if *target == {:?} {{ return true; }} frontier.push(*target); }} }} }} false }}",
        edges.join(", "),
        lowered.request.start_id,
        lowered.request.max_expansions,
        lowered.request.target_id,
    )
}

fn render_artifacts(
    family: BoundedCorpusFamily,
    identity: &str,
    value: bool,
    detail: &str,
    oracle_body: &str,
    profile: &KaniProfile,
    input: &ValidatedFiniteInput,
) -> BoundedCorpusArtifacts {
    let label = family.label();
    let value_literal = if value { "true" } else { "false" };
    let mut oracle = String::new();
    let _ = writeln!(
        oracle,
        "// Generated bounded-Kani {label} oracle: {identity}"
    );
    let _ = writeln!(oracle, "pub fn corpus_oracle() -> bool {{ {oracle_body} }}");
    let mut strategy = String::new();
    let _ = writeln!(strategy, "// Generated finite strategy: {identity}");
    let _ = writeln!(strategy, "pub const CORPUS_CASE: bool = {value_literal};");
    let mut harness = String::new();
    let _ = writeln!(harness, "// Generated Kani harness: {identity}");
    let _ = writeln!(harness, "#[kani::proof]");
    // The proof symbol is derived from the case's own identity digest and carries the
    // same identity the artifact path (`corpus/{label}-{identity}.kani.rs`) carries,
    // rather than only the family label: two files sharing a `#[kani::proof]` symbol
    // would be indistinguishable proofs in Kani's own output. This binds the symbol to
    // the case's identity; it does not by itself make that identity unique across every
    // distinct case (tracked separately as #73).
    let _ = writeln!(harness, "fn corpus_case_{label}_{identity}() {{");
    let _ = writeln!(harness, "    assert!(corpus_oracle());");
    let _ = writeln!(harness, "}}");
    let provenance = format!(
        "family={label}\nconstruct={}\nidentity={identity}\nprofile={}\nrevision={}\nexecutable={}\noptions={}\nabi={}\nmodel={}\nsource={}\nbounds={:?}\ndetail={detail}\nproof_dependencies=none\n",
        family.construct(),
        profile.selection.profile,
        profile.selection.revision,
        profile.selection.executable_digest,
        profile.selection.options_digest,
        profile.selection.abi_revision,
        input.input().model_id,
        input.input().source_id,
        input.input().bounds,
    );
    BoundedCorpusArtifacts {
        oracle: artifact(format!("corpus/{label}-{identity}.oracle.rs"), oracle),
        strategy: artifact(format!("corpus/{label}-{identity}.strategy.rs"), strategy),
        kani_harness: artifact(format!("corpus/{label}-{identity}.kani.rs"), harness),
        provenance: artifact(format!("corpus/{label}-{identity}.provenance"), provenance),
    }
}

fn artifact(path: String, contents: String) -> Artifact {
    Artifact {
        path,
        sha256: digest(&contents),
        contents,
    }
}

fn digest(value: &str) -> String {
    format!("{:x}", Sha256::digest(value.as_bytes()))
}

/// Converts one family's raw per-case assignment data into the closed `WitnessValue` map, for a
/// case that actually retains a counterexample packet. Called only from inside the `!value`
/// branch in [`generate_bounded_kani_corpus_case`], so a provable case's own raw values -- which
/// may fall outside `i64`'s range even though Contract IR's `i128` fields admit them -- are never
/// even attempted and can never refuse generation of a case no packet will carry.
fn build_assignments(
    source_id: &str,
    revision: &str,
    raw_assignments: Vec<(String, i128)>,
) -> Result<BTreeMap<String, WitnessValue>, KaniOutcome> {
    raw_assignments
        .into_iter()
        .map(|(key, value)| {
            integer_assignment(source_id, revision, value).map(|witness| (key, witness))
        })
        .collect()
}

/// Converts one bounded-Kani finite-domain integer into the closed `WitnessValue` wire
/// representation, refusing rather than silently truncating a value this bounded domain does
/// not actually produce.
fn integer_assignment(
    source_id: &str,
    revision: &str,
    value: i128,
) -> Result<WitnessValue, KaniOutcome> {
    i64::try_from(value)
        .map(WitnessValue::Integer)
        .map_err(|_| {
            KaniOutcome::non_success(
                KaniOutcomeKind::InvalidInput,
                "kani_corpus_assignment_out_of_range",
                source_id.to_owned(),
                revision.to_owned(),
            )
        })
}

#[cfg(test)]
mod tests {
    use quire_contract_ir::kani::{
        CapabilityDisposition, CapabilityEntry, DispatchIndex, FiniteInput, FiniteObject,
        FiniteReference, GraphRequest, KaniOutcomeKind, KaniProfile, ModuleDescriptor,
        PopulationCompleteness, ProfileSelection, QueryKind, ResourceBounds, SemanticFamily,
    };
    use quire_contract_ir::NumericOperator;

    use super::{
        arithmetic_assignments, collection_assignments, generate_bounded_kani_corpus_case,
        BoundedCorpusRequest,
    };

    fn fixture() -> (
        KaniProfile,
        DispatchIndex,
        quire_contract_ir::kani::ValidatedFiniteInput,
    ) {
        let selection = ProfileSelection {
            profile: "kani-bounded/1".to_owned(),
            revision: "r1".to_owned(),
            executable_digest: "exe".to_owned(),
            options_digest: "opts".to_owned(),
            abi_revision: "abi".to_owned(),
        };
        let profile = KaniProfile::new(
            selection.clone(),
            vec![
                CapabilityEntry {
                    construct: "checked-arithmetic".to_owned(),
                    disposition: CapabilityDisposition::Supported {
                        module: "arithmetic".to_owned(),
                    },
                },
                CapabilityEntry {
                    construct: "finite-reference-graph".to_owned(),
                    disposition: CapabilityDisposition::Supported {
                        module: "graphs".to_owned(),
                    },
                },
                CapabilityEntry {
                    construct: "bounded-collection-query".to_owned(),
                    disposition: CapabilityDisposition::Supported {
                        module: "collections".to_owned(),
                    },
                },
            ],
        )
        .unwrap();
        let dispatch = DispatchIndex::new(vec![
            ModuleDescriptor {
                module_id: "arithmetic".to_owned(),
                family: SemanticFamily::DefinednessArithmetic,
                abi_revision: "abi".to_owned(),
                constructs: vec!["checked-arithmetic".to_owned()],
            },
            ModuleDescriptor {
                module_id: "graphs".to_owned(),
                family: SemanticFamily::ObjectsReferencesGraphs,
                abi_revision: "abi".to_owned(),
                constructs: vec!["finite-reference-graph".to_owned()],
            },
            ModuleDescriptor {
                module_id: "collections".to_owned(),
                family: SemanticFamily::CollectionsQueries,
                abi_revision: "abi".to_owned(),
                constructs: vec!["bounded-collection-query".to_owned()],
            },
        ])
        .unwrap();
        let input = FiniteInput {
            model_id: "model".to_owned(),
            source_id: "source".to_owned(),
            profile: selection,
            completeness: PopulationCompleteness::Complete,
            bounds: ResourceBounds {
                max_objects: 2,
                max_references: 1,
                max_input_bytes: 2,
            },
            input_bytes: 1,
            objects: vec![
                FiniteObject {
                    identity: "a".to_owned(),
                    type_id: "node".to_owned(),
                    snapshot_id: "s".to_owned(),
                },
                FiniteObject {
                    identity: "b".to_owned(),
                    type_id: "node".to_owned(),
                    snapshot_id: "s".to_owned(),
                },
            ],
            references: vec![FiniteReference {
                source_id: "a".to_owned(),
                field_id: "next".to_owned(),
                target_id: "b".to_owned(),
            }],
        }
        .validate()
        .unwrap();
        (profile, dispatch, input)
    }

    /// Trace: FR-007-AC-1, FR-007-AC-2, FR-007-AC-4, FR-007-AC-5, TC-023.
    #[test]
    fn tc_023_generates_deterministic_complete_artifacts_for_every_supported_family() {
        let (profile, dispatch, input) = fixture();
        let cases = vec![
            BoundedCorpusRequest::Arithmetic(quire_contract_ir::kani::CheckedArithmeticRequest {
                source_id: "source",
                operator: NumericOperator::Add,
                left: 1,
                right: 1,
                minimum: 0,
                maximum: 2,
            }),
            BoundedCorpusRequest::Graph(GraphRequest {
                source_id: "source".to_owned(),
                start_id: "a".to_owned(),
                target_id: "b".to_owned(),
                field_id: "next".to_owned(),
                max_expansions: 2,
            }),
            BoundedCorpusRequest::Collection(quire_contract_ir::kani::CollectionQuery {
                source_id: "source".to_owned(),
                values: vec![2, 2, 7],
                max_items: 3,
                kind: QueryKind::ExistsEqual(7),
            }),
        ];
        for request in cases {
            let first =
                generate_bounded_kani_corpus_case(&profile, &dispatch, &input, request.clone())
                    .unwrap();
            let second =
                generate_bounded_kani_corpus_case(&profile, &dispatch, &input, request).unwrap();
            assert_eq!(first, second);
            assert_eq!(first.outcome.boolean_claim(), Some(true));
            assert!(!first
                .artifacts
                .kani_harness
                .contents
                .contains("kani::assume"));
            assert!(first
                .artifacts
                .provenance
                .contents
                .contains("proof_dependencies=none"));
            syn::parse_file(&format!(
                "{}\n{}",
                first.artifacts.oracle.contents, first.artifacts.kani_harness.contents
            ))
            .expect("the generated oracle and Kani harness must be valid Rust syntax");
        }
    }

    /// Trace: FR-007-AC-3, TC-023.
    #[test]
    fn tc_023_non_success_emits_no_partial_artifacts_or_boolean_claim() {
        let (profile, dispatch, input) = fixture();
        let error = generate_bounded_kani_corpus_case(
            &profile,
            &dispatch,
            &input,
            BoundedCorpusRequest::Collection(quire_contract_ir::kani::CollectionQuery {
                source_id: "source".to_owned(),
                values: vec![1, 2],
                max_items: 1,
                kind: QueryKind::ForAllNonNegative,
            }),
        )
        .unwrap_err();
        assert_eq!(error.kind, KaniOutcomeKind::ResourceExhausted);
        assert_eq!(error.boolean_claim(), None);
    }

    /// Trace: FR-007-AC-3, FR-007-AC-4, TC-023.
    #[test]
    fn tc_023_false_case_retains_a_replayable_counterexample_packet() {
        let (profile, dispatch, input) = fixture();
        let generated = generate_bounded_kani_corpus_case(
            &profile,
            &dispatch,
            &input,
            BoundedCorpusRequest::Graph(GraphRequest {
                source_id: "source".to_owned(),
                start_id: "b".to_owned(),
                target_id: "a".to_owned(),
                field_id: "next".to_owned(),
                max_expansions: 2,
            }),
        )
        .unwrap();
        assert_eq!(generated.outcome.boolean_claim(), Some(false));
        let packet = generated.counterexample.unwrap();
        assert_eq!(packet.input, input.input().clone());
        // This corpus never runs Kani, so its packet carries the real canonical input
        // assignments as an `Input` arm, never a fabricated `Witness` (ir#156).
        assert_eq!(
            packet.source,
            quire_contract_ir::kani::ReplaySource::Input(std::collections::BTreeMap::from([(
                "max_expansions".to_owned(),
                quire_contract_ir::kani::WitnessValue::Integer(2),
            )]))
        );
    }

    /// Trace: FR-007-AC-2, TC-023.
    #[test]
    fn tc_023_admitted_zero_arithmetic_is_a_proof_not_a_false_verdict() {
        let (profile, dispatch, input) = fixture();
        let generated = generate_bounded_kani_corpus_case(
            &profile,
            &dispatch,
            &input,
            BoundedCorpusRequest::Arithmetic(quire_contract_ir::kani::CheckedArithmeticRequest {
                source_id: "checked-zero",
                operator: NumericOperator::Subtract,
                left: 1,
                right: 1,
                minimum: 0,
                maximum: 1,
            }),
        )
        .unwrap();
        assert_eq!(generated.outcome.kind, KaniOutcomeKind::Proved);
        assert_eq!(generated.outcome.source_id, "checked-zero");
    }

    /// Regression for the PR #101 review finding: assignment maps were built (and their
    /// fallible `i64::try_from` conversion `?`-propagated) before `value` was known, for every
    /// family, even though the Arithmetic arm hardcodes `value = true` so its map is discarded
    /// 100% of the time. Contract IR's `CheckedArithmeticRequest` fields are `i128`, admitting
    /// the full range, so a request with an operand outside `i64`'s range that is otherwise
    /// perfectly provable was refused solely because of an assignment no packet would ever
    /// carry. Fixed by converting assignments only inside the `!value` branch that actually
    /// builds a retained counterexample packet.
    ///
    /// Trace: FR-007-AC-1, FR-007-AC-2, TC-023.
    #[test]
    fn tc_023_provable_arithmetic_with_an_out_of_i64_range_operand_still_generates() {
        let (profile, dispatch, input) = fixture();
        let left = i64::MAX as i128 + 1;
        let generated = generate_bounded_kani_corpus_case(
            &profile,
            &dispatch,
            &input,
            BoundedCorpusRequest::Arithmetic(quire_contract_ir::kani::CheckedArithmeticRequest {
                source_id: "out-of-i64-range",
                operator: NumericOperator::Add,
                left,
                right: 0,
                minimum: left,
                maximum: left,
            }),
        )
        .expect(
            "a provable case must never be refused over an assignment map no packet will carry",
        );
        assert_eq!(generated.outcome.kind, KaniOutcomeKind::Proved);
        assert_eq!(generated.outcome.source_id, "out-of-i64-range");
        assert!(generated.counterexample.is_none());
    }

    /// Trace: FR-007-AC-2, TC-023.
    #[test]
    fn tc_023_collection_oracle_evaluates_the_selected_ordered_population() {
        let (profile, dispatch, input) = fixture();
        let generated = generate_bounded_kani_corpus_case(
            &profile,
            &dispatch,
            &input,
            BoundedCorpusRequest::Collection(quire_contract_ir::kani::CollectionQuery {
                source_id: "source".to_owned(),
                values: vec![2, 2, 7],
                max_items: 3,
                kind: QueryKind::ExistsEqual(7),
            }),
        )
        .unwrap();
        assert!(generated
            .artifacts
            .oracle
            .contents
            .contains("let values = [2i128, 2i128, 7i128]"));
        assert!(generated
            .artifacts
            .oracle
            .contents
            .contains("values.iter().any(|value| *value == 7i128)"));
    }

    /// Reproduces #61 directly, and pins the fix to derivation rather than to a uniqueness
    /// guarantee the code does not hold (identity/symbol uniqueness across distinct cases with the
    /// same result value is tracked separately as #73, and is not claimed by FR-007-AC-6). Before
    /// the fix, every corpus case of a family declared the identical
    /// `#[kani::proof] fn corpus_case_{label}()` symbol regardless of case identity: within one
    /// crate that fails to compile, but the corpus's actual shape is one file per case, and across
    /// separate crates the same symbol name is indistinguishable in Kani's own output. The fix
    /// makes the symbol carry the case's own identity digest -- the same identity its artifact path
    /// already carries -- so this asserts the symbol's suffix literally equals the identity embedded
    /// in the case's own `kani_harness.path`, not merely that two arbitrarily chosen cases differ.
    ///
    /// Trace: FR-007-AC-6, TC-023.
    #[test]
    fn tc_023_kani_proof_symbol_is_derived_from_the_case_identity_in_its_artifact_path() {
        let (profile, dispatch, input) = fixture();
        let first = generate_bounded_kani_corpus_case(
            &profile,
            &dispatch,
            &input,
            BoundedCorpusRequest::Arithmetic(quire_contract_ir::kani::CheckedArithmeticRequest {
                source_id: "first",
                operator: NumericOperator::Add,
                left: 1,
                right: 1,
                minimum: 0,
                maximum: 2,
            }),
        )
        .unwrap();
        let second = generate_bounded_kani_corpus_case(
            &profile,
            &dispatch,
            &input,
            BoundedCorpusRequest::Arithmetic(quire_contract_ir::kani::CheckedArithmeticRequest {
                source_id: "second",
                operator: NumericOperator::Add,
                left: 1,
                right: 0,
                minimum: 0,
                maximum: 2,
            }),
        )
        .unwrap();
        for case in [&first, &second] {
            let symbol = proof_symbol(&case.artifacts.kani_harness.contents);
            let identity = symbol
                .strip_prefix("corpus_case_arithmetic_")
                .expect("arithmetic proof symbol must be prefixed corpus_case_arithmetic_");
            // Pins derivation, not just a difference: a symbol built from any digest other than
            // the exact identity embedded in the artifact path (for example `digest(detail)`
            // instead of `identity`) would still produce two differing, correctly prefixed
            // symbols, but would fail this equality.
            assert_eq!(
                case.artifacts.kani_harness.path,
                format!("corpus/arithmetic-{identity}.kani.rs"),
                "the proof symbol's identity suffix must equal the identity in its own artifact \
                 path, not merely differ from another case's"
            );
        }
        // These two cases were chosen with different result values (1+1 vs 1+0), so their
        // identities -- and therefore their symbols -- do differ here. That is a property of this
        // pair's inputs, not a universal the generator enforces: see #73.
        assert_ne!(
            first.artifacts.kani_harness.path, second.artifacts.kani_harness.path,
            "these two cases have distinct result values and must emit distinct files"
        );
        assert_ne!(
            proof_symbol(&first.artifacts.kani_harness.contents),
            proof_symbol(&second.artifacts.kani_harness.contents),
            "these two cases have distinct identities and must emit distinct proof symbols"
        );
    }

    /// Extracts the `#[kani::proof]` function's name from generated harness source, verbatim.
    fn proof_symbol(contents: &str) -> &str {
        let after_fn = contents
            .split("\nfn ")
            .nth(1)
            .expect("generated harness must declare a proof function");
        after_fn
            .split('(')
            .next()
            .expect("proof function name must be followed by its parameter list")
    }

    /// Regression for the PR #101 review finding that the Collection and Arithmetic families'
    /// `assignments` maps had no assertion on their content at all: stripping every
    /// `assignments.insert` from the Collection arm left all 10 tests (this file's and
    /// `tests/bounded_kani_corpus.rs`'s) green. This pins the exact content
    /// `generate_bounded_kani_corpus_case`'s Arithmetic arm feeds into `assignments` before the
    /// checked-arithmetic Kani harness compiles.
    ///
    /// Trace: FR-007-AC-4, TC-023.
    #[test]
    fn tc_023_arithmetic_assignments_are_exactly_the_two_operands() {
        let request = quire_contract_ir::kani::CheckedArithmeticRequest {
            source_id: "source",
            operator: NumericOperator::Add,
            left: 3,
            right: -4,
            minimum: -100,
            maximum: 100,
        };
        assert_eq!(
            arithmetic_assignments(&request),
            vec![("left".to_owned(), 3), ("right".to_owned(), -4)],
            "assignments must carry exactly the two operands, not the checked range"
        );
    }

    /// See [`tc_023_arithmetic_assignments_are_exactly_the_two_operands`]. Pins the Collection
    /// family's content: exactly the ordered population's own values, never `max_items` or the
    /// query's `expected` oracle target.
    ///
    /// Trace: FR-007-AC-4, TC-023.
    #[test]
    fn tc_023_collection_assignments_are_exactly_the_ordered_population() {
        let query = quire_contract_ir::kani::CollectionQuery {
            source_id: "source".to_owned(),
            values: vec![2, 2, 7],
            max_items: 3,
            kind: QueryKind::ExistsEqual(7),
        };
        assert_eq!(
            collection_assignments(&query),
            vec![
                ("value_0".to_owned(), 2),
                ("value_1".to_owned(), 2),
                ("value_2".to_owned(), 7),
            ],
            "assignments must carry exactly the ordered population, not max_items or expected"
        );
    }
}
