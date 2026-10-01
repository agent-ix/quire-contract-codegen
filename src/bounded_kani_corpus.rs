//! Integrated bounded-Kani corpus generation over Contract IR's validated finite ABI.
//!
//! The Contract IR lowerers remain the semantic authority.  This module owns the codegen-side
//! vertical slice: once one lowering is admitted, it renders the four corpus roles (oracle,
//! strategy, Kani harness, and proof-dependency graph) from the same profile selection, finite
//! input, and declared proof-dependency census.  A non-success outcome returns before any role is
//! emitted.

use std::{collections::BTreeSet, fmt::Write as _};

use quire_contract_ir::kani::{
    CheckedArithmeticRequest, CollectionQuery, DispatchIndex, FiniteInput, FiniteObject,
    FiniteReference, GraphRequest, KaniOutcome, KaniOutcomeKind, KaniProfile,
    PopulationCompleteness, ProfileSelection, QueryKind, ResourceBounds, ValidatedFiniteInput,
};
use serde::{Deserialize, Serialize};

use crate::{
    artifact::Artifact,
    canonical,
    kani_census::{
        dependency_readiness, normalize_dependencies, ProofDependencyEdge, ProofDependencyKind,
        ProofDependencyRequest, ProofReadiness,
    },
    prepare_bounded_collection_query, prepare_checked_arithmetic, prepare_finite_graph_reaches,
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
    /// The corpus case's name, `{family}_{digest}`; the same name its artifact paths carry.
    pub identity: String,
    /// Derived dependency readiness; this generator never executes or classifies a proof.
    pub readiness: ProofReadiness,
    /// Sorted complete declared dependency census.
    pub dependencies: Vec<ProofDependencyEdge>,
}

/// Registry of the corpus case identities emitted into one output tree.
///
/// A case's identity is the SHA-256 of its canonical request content: the
/// request, the finite input it is evaluated over, the profile selection, and the normalized
/// proof-dependency census. Every emitted artifact path is `corpus/{family}_{identity}.*`, so the
/// same content always lands on the same files and two distinct cases never share one, whatever
/// the emission order. Callers generating more than one case into the same output tree share one
/// registry; a case whose identity it already holds is refused as
/// `kani_corpus_identity_collision` rather than emitted again. The identity is recorded only once
/// every fallible step of [`generate_bounded_kani_corpus_case`] has succeeded, so a refused case
/// never claims one.
#[derive(Debug, Default)]
pub struct EmittedCorpusIdentities(BTreeSet<String>);

impl EmittedCorpusIdentities {
    /// Starts an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Records `identity`; `false` when it was already held.
    fn claim(&mut self, identity: &str) -> bool {
        self.0.insert(identity.to_owned())
    }
}

/// The canonical content one corpus case is identified by.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CaseIdentity<'a> {
    construct: &'static str,
    profile: &'a ProfileSelection,
    input: InputIdentity<'a>,
    request: RequestIdentity<'a>,
    dependencies: &'a [ProofDependencyEdge],
}

/// A request integer in the identity, written as its decimal text.
///
/// The canonical encoder carries a number as an IEEE 754 double and refuses an integer above
/// 2^53, but a corpus request is `i128`: its own range is the case's domain. The encoder's
/// documented route for an exact wide integer is a decimal string, so every request integer is
/// one, and the identity of a large operand is exact rather than refused or rounded.
#[derive(Clone, Copy)]
struct Wide(i128);

impl Serialize for Wide {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&self.0)
    }
}

/// Every field of a [`BoundedCorpusRequest`], including those the rendered oracle does not read.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
enum RequestIdentity<'a> {
    Arithmetic {
        source_id: &'a str,
        operator: &'static str,
        left: Wide,
        right: Wide,
        minimum: Wide,
        maximum: Wide,
    },
    Graph {
        source_id: &'a str,
        start_id: &'a str,
        target_id: &'a str,
        field_id: &'a str,
        max_expansions: usize,
    },
    Collection {
        source_id: &'a str,
        values: Vec<Wide>,
        max_items: usize,
        kind: QueryKindIdentity,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
enum QueryKindIdentity {
    ForAllNonNegative,
    ExistsEqual(Wide),
}

impl<'a> From<&'a BoundedCorpusRequest> for RequestIdentity<'a> {
    // Each request struct is destructured without `..`, so a field added to one is a compile
    // error here rather than a field silently left out of the identity.
    fn from(request: &'a BoundedCorpusRequest) -> Self {
        match request {
            BoundedCorpusRequest::Arithmetic(request) => {
                let CheckedArithmeticRequest {
                    source_id,
                    operator,
                    left,
                    right,
                    minimum,
                    maximum,
                } = request;
                Self::Arithmetic {
                    source_id,
                    operator: checked_method(*operator),
                    left: Wide(*left),
                    right: Wide(*right),
                    minimum: Wide(*minimum),
                    maximum: Wide(*maximum),
                }
            }
            BoundedCorpusRequest::Graph(request) => {
                let GraphRequest {
                    source_id,
                    start_id,
                    target_id,
                    field_id,
                    max_expansions,
                } = request;
                Self::Graph {
                    source_id,
                    start_id,
                    target_id,
                    field_id,
                    max_expansions: *max_expansions,
                }
            }
            BoundedCorpusRequest::Collection(request) => {
                let CollectionQuery {
                    source_id,
                    values,
                    max_items,
                    kind,
                } = request;
                Self::Collection {
                    source_id,
                    values: values.iter().copied().map(Wide).collect(),
                    max_items: *max_items,
                    kind: match kind {
                        QueryKind::ForAllNonNegative => QueryKindIdentity::ForAllNonNegative,
                        QueryKind::ExistsEqual(expected) => {
                            QueryKindIdentity::ExistsEqual(Wide(*expected))
                        }
                    },
                }
            }
        }
    }
}

/// A [`FiniteInput`] with its population order removed: the rendered oracle sorts the reference
/// edges it reads, so the same graph offered in another order is the same case.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct InputIdentity<'a> {
    model_id: &'a str,
    source_id: &'a str,
    profile: &'a ProfileSelection,
    completeness: &'a PopulationCompleteness,
    bounds: &'a ResourceBounds,
    input_bytes: usize,
    /// `(identity, type_id, snapshot_id)`, sorted.
    objects: Vec<(&'a str, &'a str, &'a str)>,
    /// `(source_id, field_id, target_id)`, sorted.
    references: Vec<(&'a str, &'a str, &'a str)>,
}

impl<'a> From<&'a FiniteInput> for InputIdentity<'a> {
    fn from(input: &'a FiniteInput) -> Self {
        let FiniteInput {
            model_id,
            source_id,
            profile,
            completeness,
            bounds,
            input_bytes,
            objects,
            references,
        } = input;
        let mut objects: Vec<_> = objects
            .iter()
            .map(|object| {
                let FiniteObject {
                    identity,
                    type_id,
                    snapshot_id,
                } = object;
                (identity.as_str(), type_id.as_str(), snapshot_id.as_str())
            })
            .collect();
        objects.sort_unstable();
        let mut references: Vec<_> = references
            .iter()
            .map(|reference| {
                let FiniteReference {
                    source_id,
                    field_id,
                    target_id,
                } = reference;
                (source_id.as_str(), field_id.as_str(), target_id.as_str())
            })
            .collect();
        references.sort_unstable();
        Self {
            model_id,
            source_id,
            profile,
            completeness,
            bounds,
            input_bytes: *input_bytes,
            objects,
            references,
        }
    }
}

impl CaseIdentity<'_> {
    /// Lowercase hex SHA-256 over this content's RFC 8785 canonical JSON, or `None` when the
    /// encoder refuses the content: a non-request integer above 2^53 in magnitude (request
    /// integers are [`Wide`] decimal text) or an encoding over the artifact byte ceiling.
    fn digest(&self) -> Option<String> {
        canonical::content_digest(self)
            .ok()
            .map(|digest| digest.to_string())
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
/// The case is named from its canonical content digest ([`EmittedCorpusIdentities`]), so identical
/// requests name identical artifacts and distinct requests never share a path. `emitted` records
/// that identity only after every other fallible step has succeeded; a case whose identity it
/// already holds returns `InvalidInput` `kani_corpus_identity_collision` and emits nothing.
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
    // self-dependency cannot occur -- this corpus case's own harness symbol is named from its
    // content digest, which is claimed only after this census is accepted. The empty string is passed
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
    // Computed before lowering consumes the request; recorded only after lowering succeeds.
    let Some(identity) = (CaseIdentity {
        construct: family.construct(),
        profile: &profile.selection,
        input: InputIdentity::from(input.input()),
        request: RequestIdentity::from(&request),
        dependencies: &normalized_dependencies,
    })
    .digest() else {
        return Err(KaniOutcome::non_success(
            KaniOutcomeKind::InvalidInput,
            "kani_corpus_identity_unencodable",
            request_source_id,
            revision,
        ));
    };
    let (value, oracle_body) = match request {
        BoundedCorpusRequest::Arithmetic(request) => {
            let lowered = prepare_checked_arithmetic(profile, dispatch, input, request)?;
            // Admission establishes the checked arithmetic/definedness property; the numeric
            // result itself is not a Boolean verdict (zero is as valid as any other in-range
            // result).
            (true, render_arithmetic_oracle(&lowered))
        }
        BoundedCorpusRequest::Graph(request) => {
            let lowered = prepare_finite_graph_reaches(profile, dispatch, input, request)?;
            (lowered.reachable, render_graph_oracle(&lowered, input))
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
                format!("{{ let values = [{values}]; {predicate} }}"),
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
    if !emitted.claim(&identity) {
        return Err(KaniOutcome::non_success(
            KaniOutcomeKind::InvalidInput,
            "kani_corpus_identity_collision",
            request_source_id,
            revision,
        ));
    }
    let name = format!("{}_{identity}", family.label());
    let artifacts = render_artifacts(family, &name, value, &oracle_body, &normalized_dependencies);
    Ok(BoundedCorpusCase {
        family,
        outcome,
        artifacts,
    })
}

const fn checked_method(operator: quire_contract_model::NumericOperator) -> &'static str {
    match operator {
        quire_contract_model::NumericOperator::Add => "checked_add",
        quire_contract_model::NumericOperator::Subtract => "checked_sub",
        quire_contract_model::NumericOperator::Multiply => "checked_mul",
        quire_contract_model::NumericOperator::Divide => "checked_div",
        quire_contract_model::NumericOperator::Remainder => "checked_rem",
    }
}

fn render_arithmetic_oracle(lowered: &quire_contract_ir::kani::ArithmeticLowering) -> String {
    let operator = checked_method(lowered.request.operator);
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
    let proof_graph_contents = canonical::json_file(&proof_graph_value)
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

#[cfg(test)]
mod tests {
    use quire_contract_ir::kani::{
        CapabilityDisposition, CapabilityEntry, DispatchIndex, FiniteInput, FiniteObject,
        FiniteReference, GraphRequest, KaniOutcomeKind, KaniProfile, ModuleDescriptor,
        PopulationCompleteness, ProfileSelection, QueryKind, ResourceBounds, SemanticFamily,
    };
    use quire_contract_model::NumericOperator;

    use super::{
        generate_bounded_kani_corpus_case, BoundedCorpusRequest, CorpusProofDependencyGraph,
        EmittedCorpusIdentities, ProofReadiness, CORPUS_PROOF_GRAPH_SCHEMA,
    };
    use crate::{ProofDependencyKind, ProofDependencyRequest, ProofDependencyState};

    type InputEdit = Box<dyn Fn(&mut FiniteInput)>;

    type Fixture = (
        KaniProfile,
        DispatchIndex,
        quire_contract_ir::kani::ValidatedFiniteInput,
    );

    fn fixture() -> Fixture {
        fixture_with(|_| {}, |_| {})
    }

    /// The standard fixture with its profile selection and finite input edited before validation.
    fn fixture_with(
        edit_selection: impl FnOnce(&mut ProfileSelection),
        edit_input: impl FnOnce(&mut FiniteInput),
    ) -> Fixture {
        let mut selection = ProfileSelection {
            profile: "kani-bounded/1".to_owned(),
            revision: "r1".to_owned(),
            abi_revision: "abi".to_owned(),
        };
        edit_selection(&mut selection);
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
        let mut input = FiniteInput {
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
        };
        edit_input(&mut input);
        let input = input.validate().unwrap();
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

    /// Trace: TC-023.
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
    /// Trace: TC-023.
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
    /// Trace: TC-023.
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
    /// rules, and the refusal claims no identity: the same request with a valid census is accepted.
    ///
    /// Trace: TC-023.
    #[test]
    fn tc_023_duplicate_dependency_identity_is_refused_and_claims_no_identity() {
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
        assert_eq!(
            retry.artifacts.oracle.path,
            format!("corpus/{}.oracle.rs", case_name(&retry))
        );
    }

    /// This corpus's generated harnesses render no `// proof-dependency-site:` marker, no
    /// `kani::assume`, and no `#[kani::stub]`, so a declared `Assumed` dependency must be refused
    /// (ir#80 review finding F1), claiming no identity.
    ///
    /// Trace: TC-023.
    #[test]
    fn tc_023_assumed_dependency_kind_is_refused_and_claims_no_identity() {
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
        assert_eq!(
            retry.artifacts.oracle.path,
            format!("corpus/{}.oracle.rs", case_name(&retry))
        );
    }

    /// Trace: TC-023.
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

    /// Trace: TC-023.
    #[test]
    fn tc_023_unreachable_graph_request_classifies_as_false() {
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
        assert_eq!(generated.outcome.kind, KaniOutcomeKind::Counterexample);
        assert_eq!(generated.outcome.boolean_claim(), Some(false));
    }

    /// Trace: TC-023.
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

    /// A provable request with an operand outside `i64`'s range still generates.
    ///
    /// Trace: TC-023.
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
        .expect("a provable case with an operand outside i64's range generates");
        assert_eq!(generated.outcome.kind, KaniOutcomeKind::Proved);
        assert_eq!(generated.outcome.source_id, "out-of-i64-range");
    }

    /// Operands above 2^53 that a double would conflate name different cases: each `i128` request
    /// value is identity text, not a number the encoder rounds or refuses.
    ///
    /// Trace: TC-023.
    #[test]
    fn tc_023_operands_a_double_cannot_tell_apart_name_different_cases() {
        let fixture = fixture();
        let arithmetic = |operand: i128| {
            BoundedCorpusRequest::Arithmetic(quire_contract_ir::kani::CheckedArithmeticRequest {
                source_id: "wide",
                operator: NumericOperator::Add,
                left: operand,
                right: 0,
                minimum: operand,
                maximum: operand,
            })
        };
        let below = 1_i128 << 60;
        assert_ne!(
            identity_of(&fixture, arithmetic(below)),
            identity_of(&fixture, arithmetic(below + 1))
        );
    }

    /// A bound above 2^53 is a number the encoder refuses, so the case is refused with a typed
    /// outcome and emits nothing, and claims no identity.
    ///
    /// Trace: TC-023.
    #[test]
    fn tc_023_content_the_encoder_refuses_is_a_typed_refusal_that_claims_nothing() {
        let fixture = fixture();
        let mut emitted = EmittedCorpusIdentities::new();
        let refusal = emit(
            &fixture,
            collection(1 << 60, "huge-bound"),
            &[],
            &mut emitted,
        )
        .expect_err("a bound above 2^53 has no canonical encoding");
        assert_eq!(refusal.kind, KaniOutcomeKind::InvalidInput);
        assert_eq!(refusal.code, "kani_corpus_identity_unencodable");
        assert_eq!(refusal.source_id, "huge-bound");
        assert!(emit(&fixture, collection(3, "huge-bound"), &[], &mut emitted).is_ok());
    }

    /// Trace: TC-023.
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

    /// The case name carried by a case's own harness path.
    fn case_name(case: &super::BoundedCorpusCase) -> String {
        case.artifacts
            .kani_harness
            .path
            .strip_prefix("corpus/")
            .and_then(|path| path.strip_suffix(".kani.rs"))
            .expect("harness path is corpus/<name>.kani.rs")
            .to_owned()
    }

    fn emit(
        fixture: &Fixture,
        request: BoundedCorpusRequest,
        dependencies: &[ProofDependencyRequest<'_>],
        emitted: &mut EmittedCorpusIdentities,
    ) -> Result<super::BoundedCorpusCase, quire_contract_ir::kani::KaniOutcome> {
        let (profile, dispatch, input) = fixture;
        generate_bounded_kani_corpus_case(profile, dispatch, input, request, dependencies, emitted)
    }

    fn collection(max_items: usize, source_id: &str) -> BoundedCorpusRequest {
        BoundedCorpusRequest::Collection(quire_contract_ir::kani::CollectionQuery {
            source_id: source_id.to_owned(),
            values: vec![2, 2, 7],
            max_items,
            kind: QueryKind::ExistsEqual(7),
        })
    }

    /// A case's name, paths and `#[kani::proof]` symbol come from its request content, so the
    /// same request yields the same ones in any emission order and in any run, and the symbol
    /// carries the name its artifact paths carry.
    ///
    /// Trace: TC-023.
    #[test]
    fn tc_023_case_identity_is_independent_of_emission_order_and_run() {
        let fixture = fixture();
        let requests = || {
            [
                arithmetic("first", 1, 1),
                collection(3, "second"),
                arithmetic("third", 1, 0),
            ]
        };
        let mut forward = EmittedCorpusIdentities::new();
        let in_order: Vec<_> = requests()
            .into_iter()
            .map(|request| emit(&fixture, request, &[], &mut forward).unwrap())
            .collect();
        let mut backward = EmittedCorpusIdentities::new();
        let mut reversed: Vec<_> = requests()
            .into_iter()
            .rev()
            .map(|request| emit(&fixture, request, &[], &mut backward).unwrap())
            .collect();
        reversed.reverse();
        assert_eq!(in_order, reversed);
        for case in &in_order {
            assert_eq!(
                proof_symbol(&case.artifacts.kani_harness.contents),
                format!("corpus_case_{}", case_name(case)),
            );
        }
    }

    /// Requests that differ in any one field -- including a field the rendered oracle never
    /// reads (`max_items`), the source id, or the declared census -- get distinct names, paths
    /// and proof symbols, so no case overwrites another.
    ///
    /// Trace: TC-023.
    #[test]
    fn tc_023_distinct_requests_get_distinct_names_paths_and_proof_symbols() {
        let fixture = fixture();
        let census = [ProofDependencyRequest {
            proof_id: "upstream-lemma",
            kind: ProofDependencyKind::Required,
            state: ProofDependencyState::Passed,
            original_path: None,
            replacement_path: None,
        }];
        let mut emitted = EmittedCorpusIdentities::new();
        let cases = [
            emit(&fixture, arithmetic("a", 1, 1), &[], &mut emitted),
            emit(&fixture, arithmetic("b", 1, 1), &[], &mut emitted),
            emit(&fixture, arithmetic("a", 1, 0), &[], &mut emitted),
            emit(&fixture, arithmetic("a", 1, 1), &census, &mut emitted),
            emit(&fixture, collection(3, "a"), &[], &mut emitted),
            emit(&fixture, collection(4, "a"), &[], &mut emitted),
        ]
        .map(|case| case.expect("each distinct request is accepted"));
        let mut names: Vec<_> = cases.iter().map(case_name).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), cases.len());
        let mut symbols: Vec<_> = cases
            .iter()
            .map(|case| proof_symbol(&case.artifacts.kani_harness.contents).to_owned())
            .collect();
        symbols.sort();
        symbols.dedup();
        assert_eq!(symbols.len(), cases.len());
    }

    /// Emitting one request twice through one registry refuses the second deterministically and
    /// leaves the first untouched; a fresh registry accepts it again with the same artifacts.
    ///
    /// Trace: TC-023.
    #[test]
    fn tc_023_a_request_emitted_twice_is_refused_as_an_identity_collision() {
        let fixture = fixture();
        let mut emitted = EmittedCorpusIdentities::new();
        let first = emit(&fixture, arithmetic("same", 1, 1), &[], &mut emitted).unwrap();
        for _ in 0..2 {
            let refusal = emit(&fixture, arithmetic("same", 1, 1), &[], &mut emitted).unwrap_err();
            assert_eq!(refusal.kind, KaniOutcomeKind::InvalidInput);
            assert_eq!(refusal.code, "kani_corpus_identity_collision");
        }
        let again = emit(
            &fixture,
            arithmetic("same", 1, 1),
            &[],
            &mut EmittedCorpusIdentities::new(),
        )
        .unwrap();
        assert_eq!(first, again);
    }

    fn identity_of(fixture: &Fixture, request: BoundedCorpusRequest) -> String {
        case_name(&emit(fixture, request, &[], &mut EmittedCorpusIdentities::new()).unwrap())
    }

    /// Asserts that the base case and every variant each have a different identity.
    fn assert_each_variation_changes_identity(
        fixture: &Fixture,
        base: BoundedCorpusRequest,
        variants: Vec<(&str, BoundedCorpusRequest)>,
    ) {
        let base_identity = identity_of(fixture, base);
        let mut seen = vec![base_identity];
        for (field, request) in variants {
            let identity = identity_of(fixture, request);
            assert!(
                !seen.contains(&identity),
                "varying `{field}` alone must change the case identity"
            );
            seen.push(identity);
        }
    }

    fn checked(
        source_id: &'static str,
        operator: NumericOperator,
        [left, right, minimum, maximum]: [i128; 4],
    ) -> BoundedCorpusRequest {
        BoundedCorpusRequest::Arithmetic(quire_contract_ir::kani::CheckedArithmeticRequest {
            source_id,
            operator,
            left,
            right,
            minimum,
            maximum,
        })
    }

    fn reach(
        source_id: &str,
        start_id: &str,
        target_id: &str,
        field_id: &str,
        max_expansions: usize,
    ) -> BoundedCorpusRequest {
        BoundedCorpusRequest::Graph(GraphRequest {
            source_id: source_id.to_owned(),
            start_id: start_id.to_owned(),
            target_id: target_id.to_owned(),
            field_id: field_id.to_owned(),
            max_expansions,
        })
    }

    /// Trace: TC-023.
    #[test]
    fn tc_023_every_arithmetic_request_field_changes_the_identity() {
        use NumericOperator::{Add, Subtract};
        assert_each_variation_changes_identity(
            &fixture(),
            checked("s", Add, [1, 1, 0, 2]),
            vec![
                ("source_id", checked("t", Add, [1, 1, 0, 2])),
                ("operator", checked("s", Subtract, [1, 1, 0, 2])),
                ("left", checked("s", Add, [0, 1, 0, 2])),
                ("right", checked("s", Add, [1, 0, 0, 2])),
                ("minimum", checked("s", Add, [1, 1, -1, 2])),
                ("maximum", checked("s", Add, [1, 1, 0, 3])),
            ],
        );
    }

    /// Trace: TC-023.
    #[test]
    fn tc_023_every_graph_request_field_changes_the_identity() {
        assert_each_variation_changes_identity(
            &fixture(),
            reach("s", "a", "b", "next", 2),
            vec![
                ("source_id", reach("t", "a", "b", "next", 2)),
                ("start_id", reach("s", "b", "b", "next", 2)),
                ("target_id", reach("s", "a", "a", "next", 2)),
                ("field_id", reach("s", "a", "b", "other", 2)),
                ("max_expansions", reach("s", "a", "b", "next", 3)),
            ],
        );
    }

    /// Trace: TC-023.
    #[test]
    fn tc_023_every_collection_request_field_changes_the_identity() {
        let query = |source_id: &str, values: Vec<i128>, max_items, kind| {
            BoundedCorpusRequest::Collection(quire_contract_ir::kani::CollectionQuery {
                source_id: source_id.to_owned(),
                values,
                max_items,
                kind,
            })
        };
        assert_each_variation_changes_identity(
            &fixture(),
            query("s", vec![2, 2, 7], 3, QueryKind::ExistsEqual(7)),
            vec![
                (
                    "source_id",
                    query("t", vec![2, 2, 7], 3, QueryKind::ExistsEqual(7)),
                ),
                (
                    "values",
                    query("s", vec![2, 7, 7], 3, QueryKind::ExistsEqual(7)),
                ),
                (
                    "max_items",
                    query("s", vec![2, 2, 7], 4, QueryKind::ExistsEqual(7)),
                ),
                (
                    "kind",
                    query("s", vec![2, 2, 7], 3, QueryKind::ForAllNonNegative),
                ),
                (
                    "expected value",
                    query("s", vec![2, 2, 7], 3, QueryKind::ExistsEqual(2)),
                ),
            ],
        );
    }

    /// Every part of the finite input and the profile selection is part of the identity: the
    /// graph oracle is built from the input's references, and the input also fixes the bounds
    /// the case was admitted under.
    ///
    /// Trace: TC-023.
    #[test]
    fn tc_023_every_input_and_profile_field_changes_the_identity() {
        let request = || checked("s", NumericOperator::Add, [1, 1, 0, 2]);
        let base = identity_of(&fixture(), request());
        let object = |identity: &str| FiniteObject {
            identity: identity.to_owned(),
            type_id: "node".to_owned(),
            snapshot_id: "s".to_owned(),
        };
        let edits: Vec<(&str, InputEdit)> = vec![
            ("model_id", Box::new(|i| i.model_id = "other".to_owned())),
            ("source_id", Box::new(|i| i.source_id = "other".to_owned())),
            ("max_objects", Box::new(|i| i.bounds.max_objects = 3)),
            ("max_references", Box::new(|i| i.bounds.max_references = 2)),
            (
                "max_input_bytes",
                Box::new(|i| i.bounds.max_input_bytes = 3),
            ),
            ("input_bytes", Box::new(|i| i.input_bytes = 2)),
            (
                "object identity",
                Box::new(move |i| {
                    i.objects[1] = object("c");
                    i.references[0].target_id = "c".to_owned();
                }),
            ),
            (
                "object type_id",
                Box::new(|i| i.objects[1].type_id = "other".to_owned()),
            ),
            (
                "object snapshot_id",
                Box::new(|i| i.objects[1].snapshot_id = "t".to_owned()),
            ),
            (
                "reference source_id",
                Box::new(|i| i.references[0].source_id = "b".to_owned()),
            ),
            (
                "reference field_id",
                Box::new(|i| i.references[0].field_id = "other".to_owned()),
            ),
            (
                "reference target_id",
                Box::new(|i| i.references[0].target_id = "a".to_owned()),
            ),
            (
                "extra reference",
                Box::new(|i| {
                    i.bounds.max_references = 2;
                    i.references.push(FiniteReference {
                        source_id: "b".to_owned(),
                        field_id: "next".to_owned(),
                        target_id: "a".to_owned(),
                    });
                }),
            ),
        ];
        let mut seen = vec![base];
        for (field, edit) in edits {
            let identity = identity_of(&fixture_with(|_| {}, edit), request());
            assert!(
                !seen.contains(&identity),
                "varying input `{field}` alone must change the case identity"
            );
            seen.push(identity);
        }
        let revised = identity_of(
            &fixture_with(|selection| selection.revision = "r2".to_owned(), |_| {}),
            request(),
        );
        assert!(
            !seen.contains(&revised),
            "the profile selection is part of the identity"
        );
    }

    /// The same graph offered with its objects and references in another order is the same case
    /// (so a second emission is refused), and a genuinely different graph is a different one.
    ///
    /// Trace: TC-023.
    #[test]
    fn tc_023_identity_is_canonical_over_the_input_population_order() {
        let node = |identity: &str| FiniteObject {
            identity: identity.to_owned(),
            type_id: "node".to_owned(),
            snapshot_id: "s".to_owned(),
        };
        let edge = |source: &str, target: &str| FiniteReference {
            source_id: source.to_owned(),
            field_id: "next".to_owned(),
            target_id: target.to_owned(),
        };
        let population = |objects: Vec<FiniteObject>, references: Vec<FiniteReference>| {
            fixture_with(
                |_| {},
                move |input| {
                    input.bounds.max_objects = 3;
                    input.bounds.max_references = 2;
                    input.objects = objects;
                    input.references = references;
                },
            )
        };
        let request = || reach("s", "a", "b", "next", 2);
        let forward = population(
            vec![node("a"), node("b"), node("c")],
            vec![edge("a", "b"), edge("b", "c")],
        );
        let permuted = population(
            vec![node("c"), node("a"), node("b")],
            vec![edge("b", "c"), edge("a", "b")],
        );
        let different = population(
            vec![node("a"), node("b"), node("c")],
            vec![edge("a", "b"), edge("c", "b")],
        );
        assert_eq!(
            identity_of(&forward, request()),
            identity_of(&permuted, request())
        );
        assert_ne!(
            identity_of(&forward, request()),
            identity_of(&different, request())
        );
        let mut emitted = EmittedCorpusIdentities::new();
        emit(&forward, request(), &[], &mut emitted).unwrap();
        let refusal = emit(&permuted, request(), &[], &mut emitted).unwrap_err();
        assert_eq!(refusal.code, "kani_corpus_identity_collision");
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
