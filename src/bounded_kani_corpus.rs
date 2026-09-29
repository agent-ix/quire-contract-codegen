//! Integrated bounded-Kani corpus generation over Contract IR's validated finite ABI.
//!
//! The Contract IR lowerers remain the semantic authority.  This module owns the codegen-side
//! vertical slice: once one lowering is admitted, it renders the four corpus roles (oracle,
//! strategy, Kani harness, and proof-dependency graph) from the same profile selection, finite
//! input, and declared proof-dependency census.  A non-success outcome returns before any role is
//! emitted.

use std::{collections::BTreeMap, fmt::Write as _};

use quire_contract_ir::kani::{
    CheckedArithmeticRequest, CollectionQuery, CounterexamplePacket, DispatchIndex, GraphRequest,
    KaniOutcome, KaniOutcomeKind, KaniProfile, ReplaySource, ValidatedFiniteInput, WitnessValue,
};
use serde::{Deserialize, Serialize};

use crate::{
    kani::{dependency_readiness, deterministic_json, normalize_dependencies},
    prepare_bounded_collection_query, prepare_checked_arithmetic, prepare_finite_graph_reaches,
    Artifact, ProofDependencyEdge, ProofDependencyKind, ProofDependencyRequest, ProofReadiness,
};

/// Stable schema identity for [`CorpusProofDependencyGraph`].
///
/// Deliberately distinct from `src/kani.rs`'s `quire.kani-proof-graph/v2`
/// ([`crate::kani::ProofDependencyGraph`], validated against
/// `schemas/kani-proof-graph-v2.schema.json`): that schema requires a Contract-IR `requirementId`/
/// `requirementRevision` this corpus's finite-ABI input has no analogue for, and requires a
/// cargo-kani CLI `options` array of at least fifteen entries that this generator never builds
/// (`corpus`'s harnesses are plain `#[kani::proof]`, not `#[kani::proof_for_contract]`, and no
/// unwind/solver/harness-filter option vector is ever assembled for them). Reusing that exact
/// envelope here would mean fabricating those fields; this schema instead carries only what
/// [`generate_bounded_kani_corpus_case`] actually derives (ir#80).
pub const CORPUS_PROOF_GRAPH_SCHEMA: &str = "quire.kani-corpus-proof-graph/v1";

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
    /// Deterministic corpus-scoped proof-dependency graph for this case
    /// ([`CORPUS_PROOF_GRAPH_SCHEMA`]).
    pub proof_graph: Artifact,
}

/// Deterministic, corpus-scoped proof-dependency graph for one bounded-Kani corpus case.
///
/// Carries only what [`generate_bounded_kani_corpus_case`] actually derives: the case's own
/// `#[kani::proof]` symbol, its semantic family/construct, its case name (the same name its
/// sibling artifact paths carry), the derived readiness, and the sorted declared
/// dependency census. See [`CORPUS_PROOF_GRAPH_SCHEMA`] for why this is a distinct envelope from
/// `src/kani.rs`'s `quire.kani-proof-graph/v2`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CorpusProofDependencyGraph {
    /// Stable graph schema identity ([`CORPUS_PROOF_GRAPH_SCHEMA`]).
    pub schema_version: String,
    /// The corpus case's own `#[kani::proof]` symbol; identical to the suffix-bearing function
    /// name in [`BoundedCorpusArtifacts::kani_harness`].
    pub proof_id: String,
    /// Semantic family label (e.g. `arithmetic`).
    pub family: String,
    /// Contract IR construct name this case lowers (e.g. `checked-arithmetic`).
    pub construct: String,
    /// The corpus case's name, `{family}_{number}`; the same name its artifact paths carry.
    pub identity: String,
    /// Derived dependency readiness; this generator never executes or classifies a proof.
    pub readiness: ProofReadiness,
    /// Sorted complete declared dependency census.
    pub dependencies: Vec<ProofDependencyEdge>,
}

/// Numbers the corpus cases of one generation run.
///
/// Every emitted artifact path is `corpus/{family}_{number}.*`, so each case emitted through one
/// counter gets its own files. The number is taken only once every fallible step of
/// [`generate_bounded_kani_corpus_case`] has succeeded, so a refused case never consumes one.
/// Callers generating more than one case into the same output tree share one counter; a fresh
/// counter per call makes an identical request yield identical output.
#[derive(Debug, Default)]
pub struct EmittedCorpusIdentities(usize);

impl EmittedCorpusIdentities {
    /// Starts a counter at case number zero.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn next(&mut self) -> usize {
        let number = self.0;
        self.0 = self.0.saturating_add(1);
        number
    }
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
///
/// `dependencies` is the caller-declared proof-dependency census for this exact case. It is
/// validated with the same rules `generate_kani_bundle` applies to its own census
/// (`crate::kani::validate_dependencies`),
/// plus a Required-only rule this corpus adds on top: the corpus's generated harnesses are
/// self-contained by construction (literal operands/edges baked in at generation time, no external
/// call, no `// proof-dependency-site:` marker, no `kani::assume`, no `#[kani::stub]`), so any
/// declared `Assumed` or `Stubbed` edge is refused rather than accepted and fabricated a
/// `sourceSite` pointing at a marker/assume/stub this generator never renders (ir#80 review finding
/// F1). A valid census is retained in full as the case's own
/// [`BoundedCorpusArtifacts::proof_graph`] artifact. An empty census is the ordinary case and
/// yields a `Ready` [`ProofReadiness`]; an invalid census -- an empty or duplicate declared
/// identity, an inconsistent kind/state/path combination, or any non-`Required` kind -- returns a
/// typed `InvalidInput` `kani_corpus_dependency_invalid` result before any artifact is emitted.
///
/// `emitted` hands out this case's number only after every other fallible step in this function
/// has already succeeded, so cases emitted through one counter never share artifact paths.
pub fn generate_bounded_kani_corpus_case(
    profile: &KaniProfile,
    dispatch: &DispatchIndex,
    input: &ValidatedFiniteInput,
    request: BoundedCorpusRequest,
    dependencies: &[ProofDependencyRequest<'_>],
    emitted: &mut EmittedCorpusIdentities,
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
    // The declared census is validated, and its normalized/sorted form is fixed, before any
    // lowering begins: an invalid census is refused before the generator does any semantic work,
    // and the single normalized form computed here is the one the emitted proof-dependency-graph
    // artifact (`render_artifacts`) carries.
    //
    // Validation is two layers. First, the shared rules (`crate::kani::validate_dependencies`)
    // -- non-empty and unique declared identities, and a closed kind/state/path shape per entry.
    // Second, this corpus's own Required-only rule (ir#80 review finding F1): the corpus's
    // generated harnesses are self-contained by construction -- rendering no
    // `// proof-dependency-site:` marker, no `kani::assume`, and no `#[kani::stub]` -- so an
    // `Assumed` or `Stubbed` declared edge would fabricate an `assumption:<id>`/`stub:<id>`
    // `sourceSite` pointing at a marker this generator never emits. Both layers report the
    // identical typed refusal below, since a caller cannot act differently on either failure.
    //
    // `validate_dependencies`'s second argument is the "root proof id" a dependency must not name
    // itself; passing `request_source_id` here would spuriously refuse a legitimate dependency
    // whose declared proof id happens to equal the request's own source id, even though a real
    // self-dependency cannot occur -- this corpus case's own harness symbol is named from its case
    // number, which is taken only after this census is accepted. The empty string is passed
    // instead: `validate_dependencies` requires every declared proof id to be non-empty, so `""`
    // can never equal a legitimate one and the self-dependency check can never spuriously fire
    // (ir#80 review finding F5).
    let declared_dependencies_invalid = dependencies
        .iter()
        .any(|dependency| dependency.kind != ProofDependencyKind::Required)
        || crate::kani::validate_dependencies(dependencies, "").is_err();
    if declared_dependencies_invalid {
        // The failing diagnostic's own `path` (which census index) and `message` (which rule) are
        // not carried into this refusal: `KaniOutcome::non_success`'s `source_id`/`context` fields
        // already carry `request_source_id`/`revision` -- the request's own identity, not the
        // census's -- and folding diagnostic detail into either would conflate two different
        // things this outcome identifies. A caller that needs the failing census index re-runs
        // `crate::kani::validate_dependencies` directly over the same census for the full
        // diagnostic list (ir#80 review finding F7).
        return Err(KaniOutcome::non_success(
            KaniOutcomeKind::InvalidInput,
            "kani_corpus_dependency_invalid",
            request_source_id,
            revision,
        ));
    }
    let normalized_dependencies = normalize_dependencies(dependencies);
    // Each arm collects only the *raw* (fallible-conversion-free) per-case assignment data here.
    // Converting a raw `i128` into the closed `WitnessValue` wire representation is fallible
    // (`integer_assignment`, below), and a provable case must never be refused over an assignment
    // that no packet will ever carry: conversion happens later, only inside the `!value` branch
    // that actually builds a retained counterexample packet.
    let (value, oracle_body, raw_assignments) = match request {
        BoundedCorpusRequest::Arithmetic(request) => {
            let lowered = prepare_checked_arithmetic(profile, dispatch, input, request)?;
            let raw_assignments = arithmetic_assignments(&lowered.request);
            // Admission establishes the checked arithmetic/definedness property; the numeric
            // result itself is not a Boolean verdict (zero is as valid as any other in-range
            // result).
            (true, render_arithmetic_oracle(&lowered), raw_assignments)
        }
        BoundedCorpusRequest::Graph(request) => {
            let lowered = prepare_finite_graph_reaches(profile, dispatch, input, request)?;
            let raw_assignments = graph_assignments(&lowered.request);
            (
                lowered.reachable,
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
    // Fallible conversion to `WitnessValue` runs only here, for a case that actually retains a
    // packet: a provable case's `raw_assignments` are dropped unconverted, so an admitted value
    // outside `i64`'s range can never refuse generation of a case no packet will carry. This is
    // also the last fallible step before the case is emitted, so a call that fails here never
    // consumes a case number.
    let counterexample_assignments = (!value)
        .then(|| build_assignments(&request_source_id, &revision, raw_assignments))
        .transpose()?;
    let name = format!("{}_{}", family.label(), emitted.next());
    let artifacts = render_artifacts(family, &name, value, &oracle_body, &normalized_dependencies);
    let counterexample = counterexample_assignments.map(|assignments| CounterexamplePacket {
        profile_revision: profile.selection.revision.clone(),
        input: input.input().clone(),
        // This corpus is generated from a concrete, already-admitted finite selection and
        // never runs Kani, so it never has a backend transcript to build a `Witness` from.
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

/// Renders one corpus case's artifacts, all named from the case's `name`. Infallible: the caller
/// ([`generate_bounded_kani_corpus_case`]) has already run every fallible step.
///
/// `dependencies` is the already-validated, already-normalized declared census (see
/// [`normalize_dependencies`]).
fn render_artifacts(
    family: BoundedCorpusFamily,
    name: &str,
    value: bool,
    oracle_body: &str,
    dependencies: &[ProofDependencyEdge],
) -> BoundedCorpusArtifacts {
    let label = family.label();
    let value_literal = if value { "true" } else { "false" };
    let mut oracle = String::new();
    let _ = writeln!(oracle, "// Generated bounded-Kani {label} oracle: {name}");
    let _ = writeln!(oracle, "pub fn corpus_oracle() -> bool {{ {oracle_body} }}");
    let mut strategy = String::new();
    let _ = writeln!(strategy, "// Generated finite strategy: {name}");
    let _ = writeln!(strategy, "pub const CORPUS_CASE: bool = {value_literal};");
    // The proof symbol carries the same case name the artifact paths carry, so two cases never
    // declare one `#[kani::proof]` symbol. The identical string is `proof_graph.proof_id` below,
    // so the graph names the exact symbol it describes.
    let proof_id = format!("corpus_case_{name}");
    let mut harness = String::new();
    let _ = writeln!(harness, "// Generated Kani harness: {name}");
    let _ = writeln!(harness, "#[kani::proof]");
    let _ = writeln!(harness, "fn {proof_id}() {{");
    let _ = writeln!(harness, "    assert!(corpus_oracle());");
    let _ = writeln!(harness, "}}");
    let proof_graph_value = CorpusProofDependencyGraph {
        schema_version: CORPUS_PROOF_GRAPH_SCHEMA.to_owned(),
        proof_id,
        family: label.to_owned(),
        construct: family.construct().to_owned(),
        identity: name.to_owned(),
        readiness: dependency_readiness(dependencies),
        dependencies: dependencies.to_vec(),
    };
    let proof_graph_contents = deterministic_json(&proof_graph_value)
        .expect("a corpus proof-dependency graph is plain finite data with no fallible conversion");
    BoundedCorpusArtifacts {
        oracle: Artifact::new(format!("corpus/{name}.oracle.rs"), oracle),
        strategy: Artifact::new(format!("corpus/{name}.strategy.rs"), strategy),
        kani_harness: Artifact::new(format!("corpus/{name}.kani.rs"), harness),
        proof_graph: Artifact::new(
            format!("corpus/{name}.proof-graph.json"),
            proof_graph_contents,
        ),
    }
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
        BoundedCorpusRequest, CorpusProofDependencyGraph, EmittedCorpusIdentities, ProofReadiness,
        CORPUS_PROOF_GRAPH_SCHEMA,
    };
    use crate::{ProofDependencyKind, ProofDependencyRequest, ProofDependencyState};

    fn fixture() -> (
        KaniProfile,
        DispatchIndex,
        quire_contract_ir::kani::ValidatedFiniteInput,
    ) {
        let selection = ProfileSelection {
            profile: "kani-bounded/1".to_owned(),
            revision: "r1".to_owned(),
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

    fn arithmetic(source_id: &'static str, left: i128, right: i128) -> BoundedCorpusRequest {
        BoundedCorpusRequest::Arithmetic(quire_contract_ir::kani::CheckedArithmeticRequest {
            source_id,
            operator: NumericOperator::Add,
            left,
            right,
            minimum: 0,
            maximum: 2,
        })
    }

    /// Trace: FR-015-AC-23, FR-015-AC-10, TC-023.
    #[test]
    fn tc_023_generates_deterministic_complete_artifacts_for_every_supported_family() {
        let (profile, dispatch, input) = fixture();
        let cases = vec![
            arithmetic("source", 1, 1),
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
            // A fresh counter per call: an identical request yields identical output.
            let first = generate_bounded_kani_corpus_case(
                &profile,
                &dispatch,
                &input,
                request.clone(),
                &[],
                &mut EmittedCorpusIdentities::new(),
            )
            .unwrap();
            let second = generate_bounded_kani_corpus_case(
                &profile,
                &dispatch,
                &input,
                request,
                &[],
                &mut EmittedCorpusIdentities::new(),
            )
            .unwrap();
            assert_eq!(first, second);
            assert_eq!(first.outcome.boolean_claim(), Some(true));
            assert!(!first
                .artifacts
                .kani_harness
                .contents
                .contains("kani::assume"));
            let graph: CorpusProofDependencyGraph =
                serde_json::from_str(&first.artifacts.proof_graph.contents)
                    .expect("the corpus proof-dependency graph must deserialize");
            assert_eq!(graph.schema_version, CORPUS_PROOF_GRAPH_SCHEMA);
            assert!(graph.dependencies.is_empty());
            assert_eq!(graph.readiness, ProofReadiness::Ready);
            assert_eq!(
                graph.proof_id,
                proof_symbol(&first.artifacts.kani_harness.contents),
                "the graph's proof id must equal the harness's own #[kani::proof] symbol"
            );
            syn::parse_file(&format!(
                "{}\n{}",
                first.artifacts.oracle.contents, first.artifacts.kani_harness.contents
            ))
            .expect("the generated oracle and Kani harness must be valid Rust syntax");
        }
    }

    /// A declared `Required` dependency must actually reach the emitted graph, not be silently
    /// dropped (ir#80).
    ///
    /// Trace: FR-015-AC-10, FR-015-AC-22, TC-023.
    #[test]
    fn tc_023_declared_required_dependency_appears_in_the_graph() {
        let (profile, dispatch, input) = fixture();
        let dependency = ProofDependencyRequest {
            proof_id: "upstream-lemma",
            kind: ProofDependencyKind::Required,
            state: ProofDependencyState::Passed,
            original_path: None,
            replacement_path: None,
        };
        let with_dependency = generate_bounded_kani_corpus_case(
            &profile,
            &dispatch,
            &input,
            arithmetic("with-dependency", 1, 1),
            &[dependency],
            &mut EmittedCorpusIdentities::new(),
        )
        .unwrap();
        let graph: CorpusProofDependencyGraph =
            serde_json::from_str(&with_dependency.artifacts.proof_graph.contents)
                .expect("the corpus proof-dependency graph must deserialize");
        assert_eq!(graph.dependencies.len(), 1);
        assert_eq!(graph.dependencies[0].proof_id, "upstream-lemma");
        assert_eq!(graph.dependencies[0].kind, ProofDependencyKind::Required);
        assert_eq!(graph.dependencies[0].state, ProofDependencyState::Passed);
        assert_eq!(graph.dependencies[0].source_site, None);
        assert_eq!(graph.readiness, ProofReadiness::Ready);
    }

    /// A declared `Required` dependency whose state is not `Passed` makes the case `Incomplete`.
    ///
    /// Trace: FR-015-AC-10, FR-015-AC-22, TC-023.
    #[test]
    fn tc_023_missing_required_dependency_yields_incomplete_readiness() {
        let (profile, dispatch, input) = fixture();
        let dependency = ProofDependencyRequest {
            proof_id: "missing-lemma",
            kind: ProofDependencyKind::Required,
            state: ProofDependencyState::Missing,
            original_path: None,
            replacement_path: None,
        };
        let generated = generate_bounded_kani_corpus_case(
            &profile,
            &dispatch,
            &input,
            arithmetic("missing-dependency", 1, 1),
            &[dependency],
            &mut EmittedCorpusIdentities::new(),
        )
        .unwrap();
        let graph: CorpusProofDependencyGraph =
            serde_json::from_str(&generated.artifacts.proof_graph.contents)
                .expect("the corpus proof-dependency graph must deserialize");
        assert_eq!(graph.readiness, ProofReadiness::Incomplete);
    }

    /// A declared census with a duplicate proof identity is refused by the shared dependency
    /// rules, and the refusal consumes no case number: the next accepted case is still case 0.
    ///
    /// Trace: FR-015-AC-22, TC-023.
    #[test]
    fn tc_023_duplicate_dependency_identity_is_refused_and_consumes_no_case_number() {
        let (profile, dispatch, input) = fixture();
        let mut emitted = EmittedCorpusIdentities::new();
        let duplicate = ProofDependencyRequest {
            proof_id: "upstream-lemma",
            kind: ProofDependencyKind::Required,
            state: ProofDependencyState::Passed,
            original_path: None,
            replacement_path: None,
        };
        let error = generate_bounded_kani_corpus_case(
            &profile,
            &dispatch,
            &input,
            arithmetic("duplicate-census", 1, 1),
            &[duplicate, duplicate],
            &mut emitted,
        )
        .unwrap_err();
        assert_eq!(error.kind, KaniOutcomeKind::InvalidInput);
        assert_eq!(error.code, "kani_corpus_dependency_invalid");
        let retry = generate_bounded_kani_corpus_case(
            &profile,
            &dispatch,
            &input,
            arithmetic("duplicate-census", 1, 1),
            &[],
            &mut emitted,
        )
        .unwrap();
        assert_eq!(retry.artifacts.oracle.path, "corpus/arithmetic_0.oracle.rs");
    }

    /// This corpus's generated harnesses render no `// proof-dependency-site:` marker, no
    /// `kani::assume`, and no `#[kani::stub]`, so a declared `Assumed` dependency must be refused
    /// (ir#80 review finding F1), consuming no case number.
    ///
    /// Trace: FR-015-AC-22, TC-023.
    #[test]
    fn tc_023_assumed_dependency_kind_is_refused_and_consumes_no_case_number() {
        let (profile, dispatch, input) = fixture();
        let mut emitted = EmittedCorpusIdentities::new();
        let assumed = ProofDependencyRequest {
            proof_id: "assumed-lemma",
            kind: ProofDependencyKind::Assumed,
            state: ProofDependencyState::Assumed,
            original_path: Some("crate::assumed_predicate"),
            replacement_path: None,
        };
        let error = generate_bounded_kani_corpus_case(
            &profile,
            &dispatch,
            &input,
            arithmetic("assumed-census", 1, 1),
            &[assumed],
            &mut emitted,
        )
        .unwrap_err();
        assert_eq!(error.kind, KaniOutcomeKind::InvalidInput);
        assert_eq!(error.code, "kani_corpus_dependency_invalid");
        let retry = generate_bounded_kani_corpus_case(
            &profile,
            &dispatch,
            &input,
            arithmetic("assumed-census", 1, 1),
            &[],
            &mut emitted,
        )
        .unwrap();
        assert_eq!(retry.artifacts.oracle.path, "corpus/arithmetic_0.oracle.rs");
    }

    /// Trace: FR-015-AC-23, TC-023.
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
            &[],
            &mut EmittedCorpusIdentities::new(),
        )
        .unwrap_err();
        assert_eq!(error.kind, KaniOutcomeKind::ResourceExhausted);
        assert_eq!(error.boolean_claim(), None);
    }

    /// Trace: FR-015-AC-23, TC-023.
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
            &[],
            &mut EmittedCorpusIdentities::new(),
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

    /// Trace: FR-015-AC-10, TC-023.
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
            &[],
            &mut EmittedCorpusIdentities::new(),
        )
        .unwrap();
        assert_eq!(generated.outcome.kind, KaniOutcomeKind::Proved);
        assert_eq!(generated.outcome.source_id, "checked-zero");
    }

    /// Regression for the PR #101 review finding: a provable request with an operand outside
    /// `i64`'s range must not be refused over an assignment no packet will ever carry.
    ///
    /// Trace: FR-015-AC-23, FR-015-AC-10, TC-023.
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
            &[],
            &mut EmittedCorpusIdentities::new(),
        )
        .expect(
            "a provable case must never be refused over an assignment map no packet will carry",
        );
        assert_eq!(generated.outcome.kind, KaniOutcomeKind::Proved);
        assert_eq!(generated.outcome.source_id, "out-of-i64-range");
        assert!(generated.counterexample.is_none());
    }

    /// Trace: FR-015-AC-10, TC-023.
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
            &[],
            &mut EmittedCorpusIdentities::new(),
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

    /// Cases emitted through one counter -- including the identical request twice -- get distinct
    /// case names, artifact paths and `#[kani::proof]` symbols, and each symbol carries the case
    /// name its own artifact path carries (#61, ir#73).
    ///
    /// Trace: TC-023.
    #[test]
    fn tc_023_cases_in_one_run_get_distinct_names_paths_and_proof_symbols() {
        let (profile, dispatch, input) = fixture();
        let mut emitted = EmittedCorpusIdentities::new();
        let cases = [
            arithmetic("first", 1, 1),
            arithmetic("first", 1, 1),
            arithmetic("second", 1, 0),
        ]
        .map(|request| {
            generate_bounded_kani_corpus_case(
                &profile,
                &dispatch,
                &input,
                request,
                &[],
                &mut emitted,
            )
            .unwrap()
        });
        for (number, case) in cases.iter().enumerate() {
            assert_eq!(
                case.artifacts.kani_harness.path,
                format!("corpus/arithmetic_{number}.kani.rs")
            );
            assert_eq!(
                proof_symbol(&case.artifacts.kani_harness.contents),
                format!("corpus_case_arithmetic_{number}")
            );
        }
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

    /// Pins the exact content the Arithmetic arm feeds into `assignments`.
    ///
    /// Trace: TC-023.
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

    /// Pins the Collection family's content: exactly the ordered population's own values, never
    /// `max_items` or the query's `expected` oracle target.
    ///
    /// Trace: TC-023.
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

    /// A case refused at the last fallible step (assignment conversion) consumes no case number,
    /// and a retry reports the same error (ir#57 review finding F1).
    ///
    /// Trace: TC-023.
    #[test]
    fn tc_023_retry_after_assignment_out_of_range_reports_the_real_error() {
        let (profile, dispatch, input) = fixture();
        let mut emitted = EmittedCorpusIdentities::new();
        let request = || {
            BoundedCorpusRequest::Collection(quire_contract_ir::kani::CollectionQuery {
                source_id: "out-of-range".to_owned(),
                values: vec![i64::MAX as i128 + 1],
                max_items: 1,
                kind: QueryKind::ExistsEqual(999),
            })
        };
        for _ in 0..2 {
            let error = generate_bounded_kani_corpus_case(
                &profile,
                &dispatch,
                &input,
                request(),
                &[],
                &mut emitted,
            )
            .unwrap_err();
            assert_eq!(error.kind, KaniOutcomeKind::InvalidInput);
            assert_eq!(error.code, "kani_corpus_assignment_out_of_range");
        }
        let next = generate_bounded_kani_corpus_case(
            &profile,
            &dispatch,
            &input,
            arithmetic("after", 1, 1),
            &[],
            &mut emitted,
        )
        .unwrap();
        assert_eq!(next.artifacts.oracle.path, "corpus/arithmetic_0.oracle.rs");
    }
}
