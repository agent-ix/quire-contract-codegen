//! Integrated bounded-Kani corpus generation over Contract IR's validated finite ABI.
//!
//! The Contract IR lowerers remain the semantic authority.  This module owns the codegen-side
//! vertical slice: once one lowering is admitted, it renders the four corpus roles from the same
//! profile selection and finite input.  A non-success outcome returns before any role is emitted.

use std::fmt::Write as _;

use quire_contract_ir::kani::{
    CheckedArithmeticRequest, CollectionQuery, CounterexamplePacket, DispatchIndex, GraphRequest,
    KaniOutcome, KaniOutcomeKind, KaniProfile, ValidatedFiniteInput,
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
    /// Retained false witness, when the admitted case is a counterexample.
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
    let (value, detail, oracle_body) = match request {
        BoundedCorpusRequest::Arithmetic(request) => {
            let lowered = prepare_checked_arithmetic(profile, dispatch, input, request)?;
            // Admission establishes the checked arithmetic/definedness property; the numeric
            // result itself is not a Boolean verdict (zero is as valid as any other in-range
            // result).
            (
                true,
                format!("value={}", lowered.value),
                render_arithmetic_oracle(&lowered),
            )
        }
        BoundedCorpusRequest::Graph(request) => {
            let lowered = prepare_finite_graph_reaches(profile, dispatch, input, request)?;
            (
                lowered.reachable,
                format!("expanded={}", lowered.expanded.join(",")),
                render_graph_oracle(&lowered, input),
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
            (
                lowered.value,
                format!("examined={}", lowered.examined),
                format!("{{ let values = [{values}]; {predicate} }}"),
            )
        }
    };
    let outcome = if value {
        KaniOutcome::proved(request_source_id, profile.selection.revision.clone())
    } else {
        KaniOutcome::counterexample(request_source_id, profile.selection.revision.clone())
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
    let counterexample = (!value).then(|| CounterexamplePacket {
        profile_revision: profile.selection.revision.clone(),
        input: input.input().clone(),
        witness: identity.clone(),
    });
    Ok(BoundedCorpusCase {
        family,
        outcome,
        artifacts,
        counterexample,
    })
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

#[cfg(test)]
mod tests {
    use quire_contract_ir::kani::{
        CapabilityDisposition, CapabilityEntry, DispatchIndex, FiniteInput, FiniteObject,
        FiniteReference, GraphRequest, KaniOutcomeKind, KaniProfile, ModuleDescriptor,
        PopulationCompleteness, ProfileSelection, QueryKind, ResourceBounds, SemanticFamily,
    };
    use quire_contract_ir::NumericOperator;

    use super::{generate_bounded_kani_corpus_case, BoundedCorpusRequest};

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
        assert_eq!(
            generated.counterexample.unwrap().input,
            input.input().clone()
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
}
