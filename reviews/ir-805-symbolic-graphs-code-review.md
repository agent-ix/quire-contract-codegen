---
id: SR-14100
title: IR805 symbolic graph code and Rust review
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen; PR474 at 4495d32538ba2b9c0915d915bc0f7c8231ccdc60;
  12 changed Rust modules and graph requirements/owner projections
review_set: subset
---

## Summary

Independent code review with Rust lane of draft PR474 at frozen4495 against81d7f52. The relationship graph integration has no confirmed runtime correctness defect in the inspected supported shape, but its current AC106 authored-bound qualification remains open and one AC108 tag credits a producer-only refusal. Review is read-only; no generation-time proof verdict or tag count is credited as semantic completion.

## Verdict

**CONDITIONAL — not currently mergeable with FND-001/FND-002 unresolved.** The available relationship-only integration is a coherent partial slice, but full AC98/105/106 and IR805 are not complete. A scoped merge needs an explicit tracked disposition of the current bound-control gap and retention of field owner dependencies; no such deferral is assumed. No unqualified merge approval is given.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The relationship-side implementation has no qualified installed-backend controls for authored population extent +1/-1 or edge upper +1, which AC106 explicitly requires. The three new wrappers exercise authored negation, zero-length/self-loop evaluator faults and wrong-role dispatch only. The pure-bound oracle question on IR805 comment10882823 remains unanswered; 40/40 PASS therefore does not discharge this current owned qualification gap. Define the intended mutation/original-domain observation with the requirement owner and add the corresponding failing controls, or obtain an explicit tracked deferral before merging the partial slice. Do not substitute traversal faults, unrelated assertions, safety failures or out-of-domain playbacks. | FR-015-AC-106; tests/it/kani_obligations_symbolic_graphs.rs:579-667,946-1118 |
| FND-002 | medium | The AC108 trace on the_locked_producer_refuses_an_unbounded_relationship_before_ir_admission names a generator-disposition criterion, but this test only asserts QSL CallSiteRefusal::Compile/UnsupportedConstruct before IR admission and never calls graph negotiation. It supplies upstream capability context and verifies no AC108 generator disposition. Remove that AC tag while retaining the producer observation and its TC025 context; do not fabricate a package or RequiresBound result. | FR-015-AC-108; tests/it/kani_obligations_symbolic_graphs.rs:923-942 |

## Coverage

Read every changed source/test module, actual graph lowerer and generated helper, negotiation accounting, identity/seal, runner adapter, public exports and layout scanner, with surrounding shared renderer/spec/ABI and typed owner projections. Clause id plus argument path distinguishes repeated paths and clauses; location refusal precedes owner lookups. Typed population/end supplies inclusive count0..N and degree lower..upper. Active guards exclude dormant edges from paths while dormant rows remain false; Floyd closure computes positive-length reachability without seeding identity. Zero population bypasses clause assertion without indexing start/target. Integer conversions, checked storage/source limits, unwind and typed refusals preserve original domains. Identity includes source package, actual clause form/path, selected end, population, domain, subject, ceilings, solver/options/unwind; the private seal protects mutated public artifacts. Supported sibling bytes survive per-item refusals. No new vendoring, wire/schema copy, dependency pin change, compatibility generator, gate weakening or oracle/publication-to-kani backedge was found.

The native controls use the same retained authored QSL source/site and strict emitted/admitted packages, decoded concrete degree/target vectors and qsl_replay::replay_state_clause. Healthy/negated unique kids and nonunique litter, zero population, actual zero-length and self-loop evaluator changes, and selected-occurrence dispatch to a separately admitted litter carrier with both role extents in native snapshots are credible partial controls. Wrong-role mutation changes a real carrier traversal and selected dispatch; no constant Boolean replacement is credited. The three wrappers are not the complete mutant family: population+/-1 and edge-upper+1 remain FND001. Field Reference/Option/Sequence are genuinely emitted as reaches_field without a population link by locked QSL5b; reaches_population's locked9ff typed API excludes reaches_field. Exact owner accessor/lookup were independently compared by root to current IR6ca8809; current QSL main emission was not executed here and is not claimed. Existing IR8528dd7a448/QSL820c1c4d710/IR8052cbb66f4 retain those dependencies. Field-edge success/installed-backend controls get no credit, and no guessed same-type population is used. The actual producer rejects unbounded relationship reach before admission, so no fake real-package requires_bound evidence is credited. Legacy graph corpus remains until separate AC111 qualification; neither IR598 nor IR348 closes here.

AD004 L2 review includes all Rust roots,54 source files containing cfg(test), the three private library-included integration modules, core/test_support's shared std-only allocator and exact scanner diff. graph is a real new generate family beside scalar/collection; negotiate's explicit graph edge and rank registry reflect the module graph, with no scanner narrowing or hidden dependency exception. Canonical trace attributes occur on actual test/ignored-test wrappers in the delta. Full optional whole-repository semantic review was not requested; assigned graph semantic scrutiny ran. Each correct binding below records a real test supporting part of that AC, never full discharge of a compound criterion. The upstream-only AC108 binding is wrong and has matching FND002; correct_ac is explicitly none because no local generator criterion is asserted.

No applicable AssuranceProfile was found under spec/. AD004 is architecture context, not an AssuranceProfile. No own spec/plan edit exists, so spec-review/subanalyses did not run. No source edit, public GitHub finding, aggregate CI or reviewer compiler/Kani run occurred. Author evidence was inspected at its actual clean head4495: full make kani-gate terminal exit0,40/40passed,769s,Kani0.68.0, plus confined graph/layout23PASS/3ignored, library72PASS and locked all-target lintPASS. Raw full log and graph Rust/report/console/authored/native artifacts were read; root-final-author-verification.json agrees. Historical13b evidence was not relabeled as4495. Both target directories are retained. Root owns final integrated aggregate/full Kani after latest custody.

No heavy reservation acquired or heavy command executed. The final exact process guard is recorded in resource-release.json. Native Titan16GiB/MemorySwapMax0 author scope evidence is retained; no40GiB exception or target change was used. Actual model was read from this reviewer's native rollout turn_context: gpt-6.1-sol.

## Examined Scope and Bindings

Excerpts are meaningful nonempty verbatim recorded source substrings of at most600 characters. Examined criteria in the gap method receive structural inspection; context units remain context_only. Bindings establish partial support, not whole criterion completion.

```yaml
repo: agent-ix/quire-contract-codegen
ticket: IR-805
pr: quire-contract-codegen#474
checked_head: 4495d32538ba2b9c0915d915bc0f7c8231ccdc60
assigned_base: 81d7f52e6708d5da041184c6714499fe1db7855c
model: gpt-6.1-sol
run: 231a982b-f9b9-436b-b6fa-bbd9315d14e5
artifact_id: SR-14100
method: code-review
clean: false
scope:
- id: FR-015-AC-7
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Every generated harness contains exactly one non-vacuity cover; a precondition harness covers that the
    precondition holds within the IR bounds, and a contract harness''s cover stands after the contract call, so
    a run that satisfies every check without satisfying the cover is not a proof. The cover is the last statement
    of the harness, after every assertion the harness carries: the state-clause and frame-effect harnesses and the
    scalar harness place it after their assertions, and a contract harness''s checks are inside the contract call
    its cover follows. A cover that preceded an assertion'
- id: FR-015-AC-98
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A graph claim (one reachability application of either form above, at its `state_clause` node id and argument
    path) generates a harness when the admitted package gives its covering object type `E` a finite population `[0,
    N]` through `select_population` and the selected edge has finite multiplicity (for `quire.op.model.reaches_field`
    the field's `Reference`, `Option` or bounded `Collection` member; for `quire.op.model.reaches` the destination
    end's finite multiplicity). Every symbolic object and edge is bounded inclusively by those declarations, including
    zero and maximum where admitted.
- id: FR-015-AC-100
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: For the collection family, changing the selected field's authored cardinality or element bound, admitted
    body claim, customer subject or unwind changes the harness identity; equal admitted inputs regenerate byte-identically.
    The identity records every symbolic collection domain and the solver and complete ordered option vector, and
    FR-017 run evidence records the actual Kani, Rust and solver versions, unwind and every option used by the installed
    backend. For the graph family (AC-98), changing the claim's clause id or argument path, the reachability form,
    relationship or field, its role
- id: FR-015-AC-101
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A request whose symbolic collection domain exceeds the generator's representational or resource ceiling
    settles `unsupported` with a typed reason naming the failing domain or ceiling and no harness; it never tightens
    the model domain to make a proof tractable. A missing or malformed selected declaration likewise refuses with
    its typed reason and no harness. For a graph claim, a population or edge domain whose symbolic object or edge
    count exceeds the ceiling likewise settles `unsupported` naming the domain, with no harness and no reduced population
    or multiplicity. PARTIAL (IR-268, CG
- id: FR-015-AC-102
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'An admitted collection body claim is settled through the FR-015 V2 obligation path, one disposition
    per requested item; supported items alone emit harnesses. The finite-input lowerer and hand-built corpus of
    AC-86 to AC-95 neither supply the symbolic bounds nor classify the proof verdict. Graph items take the same
    V2 path (IR-805), one disposition per requested item, and are not implementation or proof evidence for IR-268.
    IMPLEMENTED for the collection family (IR-268, CG #396 fe20ecc): `tests/it/kani_obligations_symbolic_domains.rs`'
- id: FR-015-AC-105
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'For a graph claim the harness draws an object count in `[0, N]` of the one covering population and symbolic
    objects within it; for every object, the selected edge as its shape states: a `Reference` field has exactly
    one target, an `Option` field an absent or one target, a `Collection` field or finite multi-valued relationship
    end a count in `[lower, upper]` (a unique end admits no repeated target, a non-unique end repeats, and `lower
    > 0`, such as `{2,3}`, forces at least `lower` targets) and every target is an object of the population. Start
    and target are drawn from that population and'
- id: FR-015-AC-106
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'The independent native oracle replays the same authored clause over each concrete graph through QSL''s
    replay (`qsl_replay`), and does not substitute a separately authored reachability. With the installed backend,
    a QSL-emitted, IR-admitted healthy case yields `Verified` with its cover satisfied for one field-edge claim
    and one relationship-end claim over a `{0,3}` unique and a `{0,3}` non-unique end; each seeded mutant yields
    `Falsified` with an in-domain playback: reachability that allows a zero-length path (so an isolated `start =
    target` reads reachable), one that rejects a self-loop,'
- id: FR-015-AC-107
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Graph claims are located by `state_clause` node id and argument path: one reachability written twice
    in one condition yields two items at positions 0 and 1; one node under two clauses yields two items with different
    identities; a field-edge and a relationship-end reachability yield separate items; a collection claim, a clause
    with no reachability and a path that does not name a reachability application each settle `unsupported` naming
    the missing claim with no harness. A mixed request settles one disposition per item and a supported item''s
    harness is unchanged by a refused sibling.'
- id: FR-015-AC-108
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A graph claim settles by the disposition sequence in the Description (the paragraph "Disposition sequence
    for a graph claim"), with no harness, domain reduction or caller `max_expansions` substitute. While the population
    selection for the claim is `Unavailable` (no available population selection for this admitted claim), the claim
    settles `unsupported` naming it. Given a selection, `Unbounded` or `UnknownClosureOpenExtent`, that is a claim
    whose population has no finite authored bound (once a selection is available), settles `requires_bound` naming
    the population; an unbounded
- id: FR-017-AC-2
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: An absent launcher is refused with a typed reason naming its path before anything runs.
- id: FR-017-AC-4
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A run is verified only when the process exited successfully, the report states success, and every cover
    property is reported satisfied; a successful run with an unsatisfied cover is cover-unsatisfied with its satisfied
    and total counts; a report of success from an unsuccessfully exited process, a report with no cover property
    and a report with no successful check are each inconclusive with their own reason; console banners never decide
    the verdict.
- id: FR-017-AC-5
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A failed non-unwinding check with a concrete playback is falsified carrying that playback verbatim and
    never the playback of a satisfied cover; a failure with no playback is inconclusive for that reason; a failed
    unwinding assertion is inconclusive as an exhausted bound rather than falsified, and a succeeded unwinding check
    in a results listing is not a failure.
- id: FR-017-AC-6
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The execution evidence of a single run carries the obligation kind, the harness path, the launcher path,
    the complete argument vector, the unwind bound, the solver, the exit code and the outcome.
- id: FR-015
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'When a caller selects Kani generation for complete-V1 contracts whose scalar

    oracles FR-014 generated, the code generator shall emit one bounded Kani

    obligation per precondition, postcondition, invariant and frame condition,

    each bounded by its model-domain bounds. It is the Kani

    backend''s one generator

    ([ADR-001](../../decisions/ADR-001-overlapping-generators-and-input-models.md)),

    and every item it lowers is a claim over an admitted `quire.checked-package/v2`

    package.'
- id: FR-017
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  excerpt: 'When a caller runs an FR-015, FR-022/FR-014 or FR-035 harness, the code generator shall invoke the

    installed Kani backend and retain the backend''s own reported outcome as typed

    execution evidence. A caller may hand the generator several harnesses to run together

    (FR-017-AC-21 to FR-017-AC-25); the generator then forms groups, runs each

    group in one launcher process, and retains one evidence record per harness. Unless a

    statement says "batch" or "group", it describes a run of one harness.'
- id: TC-025
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: context_only
  excerpt: 'Verify that bounded contracts produce one harness per obligation kind and

    refuse non-finite obligations. IR-268 covers the collection half of planned V2

    symbolic obligations: the claim is an admitted QSL clause/function body over a

    selected model collection field. Graph reachability is required by

    FR-015-AC-98 and FR-015-AC-105 to FR-015-AC-108 (IR-805); it is not IR-268

    evidence. The finite-input collection and graph corpus is not proof evidence

    for either symbolic family.'
- id: AD-004
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: context_only
  excerpt: 'AD-001 states what the generator is and which seams it has. This AD states how `src/` is cut so

    that the code matches the subsystems in the Subsystem Registry of [spec.md](../spec.md): the

    directory tree, which module goes where, which way imports may point, and what is deleted. It is

    the design that IR-348 implements. It describes this repository only. Seams to IR, RT and QSL are

    AD-001''s. The replay seam (AD-002) and the evidence chain (AD-003) are the seam descriptions of

    IR-324; both are merged, and this AD references them.'
- id: StR-001
  path: spec/core/stakeholder/StR-001-traceable-generation.md
  role: context_only
  excerpt: 'Assurance engineers require that the generator shall derive executable, property-test, proof, and

    coverage artifacts reproducibly from one validated contract without hiding unsupported semantics,

    vacuity, rejection, or assumptions.'
- id: src/kani/generate/graph.rs
  path: src/kani/generate/graph.rs
  role: examined
  excerpt: "//! Source-bound symbolic reference graphs at state-clause argument paths (FR-015).\n//!\n//! Population\
    \ extent and edge multiplicity come only from admitted owner projections.\nuse quire_contract_model::selected_population::{\n\
    \    select_population, CheckedSelectedPopulation, CheckedSelectedPopulationOutcome as Population,\n    CheckedSelectedPopulationSource,\n\
    };\nuse quire_contract_model::{\n    CheckedCollectionKind, CheckedMemberType, CheckedNodeId, CheckedPackageV2,\n\
    \    CheckedReachesPopulationError, CheckedRelationshipEndError, CheckedSemanticNodeV2,\n};\nuse serde_json::Value;\n\
    \nuse"
- id: src/kani/generate/mod.rs
  path: src/kani/generate/mod.rs
  role: examined
  excerpt: '//! The one Kani generator: the request and result vocabulary, the negotiation entry and the

    //! family lowerers.


    // Census validation; interim, see the file header.

    pub(crate) mod census_validation;

    // The V1 clause lowering the precondition and contract families share.

    pub(crate) mod clause;

    // Bounded collection claims over admitted V2 model fields.

    pub(crate) mod collection;

    // Symbolic graph claims at exact state-clause argument paths.

    pub(crate) mod graph;

    // The contract family.

    pub(crate) mod contract;

    // The bounded-Kani corpus family.

    pub(crate) mod corpus;

    // The state-frame'
- id: src/kani/generate/negotiate.rs
  path: src/kani/generate/negotiate.rs
  role: examined
  excerpt: '//! Separate bounded Kani obligations with per-item backend negotiation (FR-015).

    //!

    //! Every requested item is accounted for before any harness bytes are exposed: each receives

    //! exactly one [`ObligationRecord`] whose disposition is `supported`, `requires_bound`,

    //! `unsupported` or `invalid_request` (Contract IR FR-036). One `invalid_request` rejects the

    //! whole request and no harness is returned. Otherwise one harness is returned per supported

    //! item and never one for any other disposition.

    //!

    //! Each supported item is one IR clause lowered to one obligation — a'
- id: src/kani/generate/outcome.rs
  path: src/kani/generate/outcome.rs
  role: examined
  excerpt: "//! The request and result vocabulary of Kani obligation negotiation (FR-015): the items a\n//! request\
    \ names, the records it returns and the reasons an item has no harness. A leaf inside\n//! `generate/`: it imports\
    \ no family.\n\nuse quire_contract_model::{\n    BoundPackage, CheckedModelFieldsError, CheckedNodeId, CheckedNodeTag,\
    \ CheckedPackageIncomplete,\n    CheckedPackageLimit, CheckedPackageRefusal, CheckedPackageV2, CheckedSourceMapEntry,\n\
    \    ClauseKind, ClauseRef, ContractPackageV1, DependencyIdentity, SourceSpan,\n};\nuse serde::{Serialize, Serializer};\n\
    \nuse crate::{\n   "
- id: src/kani/identity.rs
  path: src/kani/identity.rs
  role: examined
  excerpt: "//! The harness and identity record types of a generated Kani obligation.\n//!\n//! The generators build\
    \ these records and the runner, the witness join and the replay read\n//! them, so they live below both: a harness\
    \ is run from its identity and its source text, and\n//! no reader imports a generator.\n\nuse std::{num::NonZeroU64,\
    \ time::Duration};\n\nuse quire_canonical::FixedShape;\nuse quire_contract_model::{CheckedNodeId, ClauseRef,\
    \ DependencyIdentity, SourceSpan};\nuse serde::{Deserialize, Serialize};\n\nuse crate::{\n    core::artifact::Artifact,\n\
    \    core::identity::{HarnessPath, HarnessSymbol,"
- id: src/kani/run/harness.rs
  path: src/kani/run/harness.rs
  role: examined
  excerpt: "//! The generated harness of any kind this crate can run, and exactly what execution reads from\n//!\
    \ it (FR-017).\n\nuse crate::{\n    core::artifact::Artifact,\n    kani::abi::KaniSolver,\n    kani::identity::{\n\
    \        CollectionElementIdentity, KaniCollectionObligationHarness, KaniGraphObligationHarness,\n        KaniObligationHarness,\
    \ KaniScalarObligationHarness, ObligationKind, StateFrameHarness,\n        StateFrameProperty, SymbolicArgumentBounds,\
    \ SymbolicBounds,\n    },\n};\n\n/// A generated harness of either kind this module can run.\n///\n/// The two\
    \ kinds carry their own identity types but"
- id: src/lib.rs
  path: src/lib.rs
  role: examined
  excerpt: '//! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.


    #![forbid(unsafe_code)]

    #![deny(missing_docs)]


    // Integration fixture modules name this crate as an external dependency.

    #[cfg(test)]

    extern crate self as quire_contract_codegen;


    // Private integration control needs internal generation and execution in the library test binary.

    #[cfg(test)]

    #[path = "../tests/it/kani_render_private.rs"]

    mod kani_render_private;


    #[cfg(test)]

    #[path = "../tests/it/kani_scalar_native_private.rs"]

    mod kani_scalar_native_private;


    #[cfg(all(test, target_os ='
- id: tests/it/kani_obligations.rs
  path: tests/it/kani_obligations.rs
  role: examined
  excerpt: '//! FR-015 separate bounded Kani obligations, FR-017 Kani execution, and Contract IR FR-036

    //! backend negotiation.

    //!

    //! The default lane checks negotiation, refusal and harness shape without running Kani. The

    //! `kani` lane (`make kani`, `#[ignore]` here) measures the installed backend, runs real

    //! verified and seeded-failing harnesses one at a time, and writes execution evidence under

    //! `CARGO_TARGET_TMPDIR/kani-obligation-evidence`.


    // package.rs holds a process-global `application_registry()` static keyed by small integer

    // fixture codes that this file and'
- id: tests/it/kani_obligations_symbolic_domains.rs
  path: tests/it/kani_obligations_symbolic_domains.rs
  role: examined
  excerpt: "//! Source-backed symbolic collection and graph obligation fixtures.\n//!\n//! QSL emits every admitted\
    \ package through its public `call_site` facade from\n//! native source and a test-authored domain model document.\
    \ No Contract IR\n//! package wire is constructed in this module.\n\nuse crate::scratch_crate::{runtime_dependency,\
    \ write_manifest, TemporaryDirectory};\nuse ix_trace_rs::trace;\nuse qsl_replay::{\n    call_site, object_reference_witness,\
    \ replay, write_object_environment, ByteDigest, Category,\n    DeclarationKey, DependencyInput, DigestDomain,\
    \ DigestRecord, DomainPackageRef,\n   "
- id: tests/it/kani_obligations_symbolic_graphs.rs
  path: tests/it/kani_obligations_symbolic_graphs.rs
  role: examined
  excerpt: "//! Authored QSL symbolic graph controls (FR-015).\nuse ix_trace_rs::trace;\nuse qsl_replay::{call_site,\
    \ DependencyInput, Identifier, QualifiedName, SourceIdentity};\nuse quire_contract_model::{\n    CheckedPackageEvidence,\
    \ CheckedPackageReadLimits, CheckedPackageV2, CheckedPackageV2ReadResult,\n};\nuse serde_json::{json, Value};\n\
    \nfn fixture(edge: &str) -> CheckedPackageV2 {\n    fixture_condition(edge, \"not reaches(self, self, EDGE)\"\
    , 3, true)\n}\n\nfn fixture_condition(edge: &str, condition: &str, maximum: u64, bounded: bool) -> CheckedPackageV2\
    \ {\n    fixture_data(edge, condition, maximum,"
- id: tests/it/layout.rs
  path: tests/it/layout.rs
  role: examined
  excerpt: '//! AD-004 layout decisions L-1 and L-2 and FR-024-AC-10, checked over `src/`.

    //!

    //! L-1: `src/` holds the eight subsystem directories and `lib.rs`, and every module the AD''s

    //! module-to-subsystem map names sits in the directory the map names.

    //!

    //! L-2: no file imports an item through the crate root, an import names a directory the

    //! importing directory may reach, the module graph has no cycle (`#[cfg(test)]` modules

    //! included, because the scan reads whole files), and the order inside `kani/` holds.

    //!

    //! The scan reads `crate::` paths in code. Comments and string and'
- id: tests/it/main.rs
  path: tests/it/main.rs
  role: examined
  excerpt: '//! IR-237: the integration test crate. Cargo builds a `tests/<name>/main.rs` directory as one

    //! test target automatically (no `[[test]]` entry needed), so every former top-level `tests/*.rs`

    //! file below is a module of the single `it` binary instead of its own executable -- each still

    //! links the crate and its dependencies once per `cargo test` invocation rather than once per

    //! former file.

    //!

    //! `common`, `composite_equality_support`, `exact_scalar_support`, `exact_function_support` and

    //! `fixtures` stay at `tests/`, one level up from here; the modules that use them reach'
findings:
- fnd: FND-001
  severity: medium
  confidence: high
  method: code-review
  check: test-intent
  artifact_id: FR-015-AC-106
  related_ids:
  - FR-015-AC-98
  - FR-015-AC-105
  - TC-025
  path: tests/it/kani_obligations_symbolic_graphs.rs
  lines: 579-667,946-1118
  excerpt: 'each seeded mutant yields `Falsified` with an in-domain playback: reachability that allows a zero-length
    path (so an isolated `start = target` reads reachable), one that rejects a self-loop, one following another
    field or role, a population extent raised or lowered by one, an edge upper bound raised by one, and a negated
    claim.'
  finding: The relationship-side implementation has no qualified installed-backend controls for authored population
    extent +1/-1 or edge upper +1, which AC106 explicitly requires. The three new wrappers exercise authored negation,
    zero-length/self-loop evaluator faults and wrong-role dispatch only. The pure-bound oracle question on IR805
    comment10882823 remains unanswered; 40/40 PASS therefore does not discharge this current owned qualification
    gap. Define the intended mutation/original-domain observation with the requirement owner and add the corresponding
    failing controls, or obtain an explicit tracked deferral before merging the partial slice. Do not substitute
    traversal faults, unrelated assertions, safety failures or out-of-domain playbacks.
- fnd: FND-002
  severity: medium
  confidence: high
  method: code-review
  check: trace
  artifact_id: FR-015-AC-108
  related_ids:
  - TC-025
  path: tests/it/kani_obligations_symbolic_graphs.rs
  lines: 923-942
  excerpt: A graph claim settles by the disposition sequence in the Description (the paragraph "Disposition sequence
    for a graph claim"), with no harness, domain reduction or caller `max_expansions` substitute.
  finding: The AC108 trace on the_locked_producer_refuses_an_unbounded_relationship_before_ir_admission names a
    generator-disposition criterion, but this test only asserts QSL CallSiteRefusal::Compile/UnsupportedConstruct
    before IR admission and never calls graph negotiation. It supplies upstream capability context and verifies
    no AC108 generator disposition. Remove that AC tag while retaining the producer observation and its TC025 context;
    do not fabricate a package or RequiresBound result.
  test_id: tests/it/kani_obligations_symbolic_graphs.rs::the_locked_producer_refuses_an_unbounded_relationship_before_ir_admission
  claimed_ac: FR-015-AC-108
  correct_ac: none (upstream producer capability context only)
bindings:
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::authored_relationship_domains_use_population_and_typed_destination_multiplicity
  ac_id: FR-015-AC-98
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::same_authored_clause_native_replay_distinguishes_isolation_self_loop_and_selected_role
  ac_id: FR-015-AC-98
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::authored_pre_post_and_invariant_forms_keep_the_clause_kind
  ac_id: FR-015-AC-98
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::actual_second_role_carrier_and_native_snapshot_preserve_both_relationships
  ac_id: FR-015-AC-98
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::a_subtype_operand_uses_the_near_ends_covering_population_without_a_subtype_cap
  ac_id: FR-015-AC-98
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::distinct_operation_operands_are_drawn_independently_from_the_same_population
  ac_id: FR-015-AC-98
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::repeated_occurrences_keep_their_paths_and_equal_inputs_regenerate_identically
  ac_id: FR-015-AC-100
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::graph_identity_binds_subject_unwind_solver_and_complete_ordered_options
  ac_id: FR-015-AC-100
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::zero_population_is_retained_and_inadequate_resources_refuse_without_domain_reduction
  ac_id: FR-015-AC-101
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::refused_graph_sibling_leaves_supported_artifact_byte_identical
  ac_id: FR-015-AC-102
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::authored_relationship_domains_use_population_and_typed_destination_multiplicity
  ac_id: FR-015-AC-105
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::zero_population_is_retained_and_inadequate_resources_refuse_without_domain_reduction
  ac_id: FR-015-AC-105
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::a_subtype_operand_uses_the_near_ends_covering_population_without_a_subtype_cap
  ac_id: FR-015-AC-105
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::distinct_operation_operands_are_drawn_independently_from_the_same_population
  ac_id: FR-015-AC-105
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::same_authored_clause_native_replay_distinguishes_isolation_self_loop_and_selected_role
  ac_id: FR-015-AC-106
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::tc_025_real_kani_graph_relationship_claim_verifies_and_negated_claim_falsifies
  ac_id: FR-015-AC-106
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::tc_025_real_kani_positive_length_evaluator_rejects_zero_length_and_self_loop_mutants
  ac_id: FR-015-AC-106
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::actual_second_role_carrier_and_native_snapshot_preserve_both_relationships
  ac_id: FR-015-AC-106
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::tc_025_real_kani_wrong_role_dispatch_falsifies_the_same_authored_clause
  ac_id: FR-015-AC-106
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::actual_field_producer_has_no_population_link_and_location_refuses_first
  ac_id: FR-015-AC-107
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::repeated_occurrences_keep_their_paths_and_equal_inputs_regenerate_identically
  ac_id: FR-015-AC-107
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::refused_graph_sibling_leaves_supported_artifact_byte_identical
  ac_id: FR-015-AC-107
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::authored_pre_post_and_invariant_forms_keep_the_clause_kind
  ac_id: FR-015-AC-107
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::the_same_reachability_under_two_actual_clauses_has_distinct_identities
  ac_id: FR-015-AC-107
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::a_collection_condition_and_a_clause_without_reachability_refuse_missing_claim
  ac_id: FR-015-AC-107
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::actual_field_producer_has_no_population_link_and_location_refuses_first
  ac_id: FR-015-AC-108
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::authored_unbound_relationship_refuses_unavailable_without_inventing_a_binding
  ac_id: FR-015-AC-108
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::the_locked_producer_refuses_an_unbounded_relationship_before_ir_admission
  ac_id: FR-015-AC-108
  trace: wrong
  finding_ref: SR-14100/FND-002
```

## Metadata correction 1

Metadata correction only for SR14100 FND001 and FND002 (including its SR14101 reference). Original Findings, Verdict, artifact prefixes and private review comments are retained unchanged. Both original records paired a test-source path with requirement text. The records below correct that pairing with actual test-source excerpts and separately locate each original requirement excerpt at its spec path. No finding is resolved or substantively changed; FND001/FND002 remain open. The correction records supersede only the excerpt/line metadata for these original finding ids.

```yaml
artifact_id: SR-14100
correction_round: 1
method: code-review
model: gpt-6.1-sol
run: 31a87b74-b0d0-4a76-ac4c-945939d0f1e2
original_review_run: 231a982b-f9b9-436b-b6fa-bbd9315d14e5
checked_head: 4495d32538ba2b9c0915d915bc0f7c8231ccdc60
findings_owner: SR-14100
metadata_only: true
findings_and_verdict_unchanged: true
corrections:
- fnd: FND-001
  severity: medium
  confidence: high
  method: code-review
  check: test-intent
  artifact_id: FR-015-AC-106
  related_ids:
  - FR-015-AC-98
  - FR-015-AC-105
  - TC-025
  path: tests/it/kani_obligations_symbolic_graphs.rs
  lines: 582-593
  excerpt: "fn tc_025_real_kani_graph_relationship_claim_verifies_and_negated_claim_falsifies() {\n    use quire_contract_codegen::KaniRunOutcome;\n\
    \    for edge in [\"step\", \"kids\", \"litter\"] {\n        let healthy = fixture_data(\n            edge,\n\
    \            \"reaches(self, self, EDGE) or not reaches(self, self, EDGE)\",\n            3,\n            true,\n\
    \        );\n        retain_authored(&healthy, &format!(\"{edge}-healthy\"));\n        let healthy_harness =\
    \ harness(&healthy.package, &[0]);\n        let verified = run_graph_kani(&healthy_harness, &format!(\"{edge}-healthy\"\
    ));"
  finding: The relationship-side implementation has no qualified installed-backend controls for authored population
    extent +1/-1 or edge upper +1, which AC106 explicitly requires. The three new wrappers exercise authored negation,
    zero-length/self-loop evaluator faults and wrong-role dispatch only. The pure-bound oracle question on IR805
    comment10882823 remains unanswered; 40/40 PASS therefore does not discharge this current owned qualification
    gap. Define the intended mutation/original-domain observation with the requirement owner and add the corresponding
    failing controls, or obtain an explicit tracked deferral before merging the partial slice. Do not substitute
    traversal faults, unrelated assertions, safety failures or out-of-domain playbacks.
  requirement_path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  requirement_excerpt: 'each seeded mutant yields `Falsified` with an in-domain playback: reachability that allows
    a zero-length path (so an isolated `start = target` reads reachable), one that rejects a self-loop, one following
    another field or role, a population extent raised or lowered by one, an edge upper bound raised by one, and
    a negated claim.'
  recorded_head: 4495d32538ba2b9c0915d915bc0f7c8231ccdc60
  corrected_fields:
  - excerpt
  - lines
  correction_reason: The original excerpt was requirement text despite recording a test-source path. This appended
    correction supplies test-source text at that path; the original requirement excerpt is retained under its actual
    spec path. Finding id, severity, confidence, substance and verdict are unchanged.
- fnd: FND-002
  severity: medium
  confidence: high
  method: code-review
  check: trace
  artifact_id: FR-015-AC-108
  related_ids:
  - TC-025
  path: tests/it/kani_obligations_symbolic_graphs.rs
  lines: 925-945
  excerpt: "fn the_locked_producer_refuses_an_unbounded_relationship_before_ir_admission() {\n    let refusal =\
    \ try_fixture_source(\n        \"wild\",\n        \"not reaches(self,self,EDGE)\",\n        3,\n        true,\n\
    \        \"invariant\",\n        \"\",\n    )\n    .err()\n    .expect(\"the locked producer does not emit an\
    \ unbounded relationship claim\");\n    assert!(matches!(\n        refusal,\n        qsl_replay::CallSiteRefusal::Compile\
    \ {\n            code: qsl_replay::Code::UnsupportedConstruct,\n            ..\n        }\n    ));\n    assert!(refusal.to_string().contains(\"\
    expression-form\"));\n}\n"
  finding: The AC108 trace on the_locked_producer_refuses_an_unbounded_relationship_before_ir_admission names a
    generator-disposition criterion, but this test only asserts QSL CallSiteRefusal::Compile/UnsupportedConstruct
    before IR admission and never calls graph negotiation. It supplies upstream capability context and verifies
    no AC108 generator disposition. Remove that AC tag while retaining the producer observation and its TC025 context;
    do not fabricate a package or RequiresBound result.
  test_id: tests/it/kani_obligations_symbolic_graphs.rs::the_locked_producer_refuses_an_unbounded_relationship_before_ir_admission
  claimed_ac: FR-015-AC-108
  correct_ac: none (upstream producer capability context only)
  requirement_path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  requirement_excerpt: A graph claim settles by the disposition sequence in the Description (the paragraph "Disposition
    sequence for a graph claim"), with no harness, domain reduction or caller `max_expansions` substitute.
  recorded_head: 4495d32538ba2b9c0915d915bc0f7c8231ccdc60
  corrected_fields:
  - excerpt
  - lines
  correction_reason: The original excerpt was requirement text despite recording a test-source path. This appended
    correction supplies test-source text at that path; the original requirement excerpt is retained under its actual
    spec path. Finding id, severity, confidence, substance and verdict are unchanged.
```

## Dispositions

### Round 1

Round1 independently inspects4495→023b: only the producer-only test trace changes. FND002 is fixed; FND001 remains still-open, so PR474 remains draft/not mergeable. Field success dependencies remain unchanged. No new finding, owner answer, waiver, deferral or partial-merge approval exists. Original Findings, Verdict, metadata corrections and all prior private comments remain unchanged. Gap-method outcomes below reference findings owned once by SR14100.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | still-open | Still open at023b4b5: the only change removes the producer-only AC108 trace. No installed-backend population extent +1/-1 or edge-upper +1 control was added. Owner clarification IR805 comment10882823 has no recorded answer, and there is no waiver, deferral or partial-merge approval. Pure sampler narrowing removes traces; widening preserves the original evaluator on original-domain inputs and only adds out-of-domain graphs. The existing traversal/role controls do not qualify these authored-bound mutants. Defining the intended mutation/observation and supplying qualifying controls remains necessary; no speculative substitute receives credit. |
| FND-002 | fixed | Actual source removes only AC108; TC025 and producer assertions remain. |

Evidence inspected: review-fix-fmt.log (confined fmt-check exit0), review-fix-producer-test.log (exact1PASS), review-fix-trace.json (one canonical TC025 claim, noAC108), review-fix-matrix.json (same866 statuses512/286/18/50; exactly one binder removed and none added), coder-report review-fix section and actual git diff. Historical author40/40 remains attributed to4495; no compiler/backend/aggregate freshness run was required. The original-domain pure-bound qualification remains unresolved.

```yaml
repo: agent-ix/quire-contract-codegen
ticket: IR-805
artifact_id: SR-14100
method: code-review
round: 1
pr: quire-contract-codegen#474
checked_head: 023b4b579a82f3bcba8fbaeb21fb71f8ad169bbd
previous_head: 4495d32538ba2b9c0915d915bc0f7c8231ccdc60
original_review_run: 231a982b-f9b9-436b-b6fa-bbd9315d14e5
model: gpt-6.1-sol
run: f16cec43-8c12-458c-8d16-9b1a0a8b401a
dispositions_owner: SR-14100
new_findings: []
dispositions:
- fnd: FND-001
  outcome: still-open
  reason: 'Still open at023b4b5: the only change removes the producer-only AC108 trace. No installed-backend population
    extent +1/-1 or edge-upper +1 control was added. Owner clarification IR805 comment10882823 has no recorded answer,
    and there is no waiver, deferral or partial-merge approval. Pure sampler narrowing removes traces; widening
    preserves the original evaluator on original-domain inputs and only adds out-of-domain graphs. The existing
    traversal/role controls do not qualify these authored-bound mutants. Defining the intended mutation/observation
    and supplying qualifying controls remains necessary; no speculative substitute receives credit.'
- fnd: FND-002
  outcome: fixed
  after_excerpt: "#[trace(\"TC-025\")]\n#[test]\nfn the_locked_producer_refuses_an_unbounded_relationship_before_ir_admission()\
    \ {\n    let refusal = try_fixture_source(\n        \"wild\",\n        \"not reaches(self,self,EDGE)\",\n  \
    \      3,\n        true,\n        \"invariant\",\n        \"\",\n    )\n    .err()\n    .expect(\"the locked\
    \ producer does not emit an unbounded relationship claim\");\n    assert!(matches!(\n        refusal,\n    \
    \    qsl_replay::CallSiteRefusal::Compile {\n            code: qsl_replay::Code::UnsupportedConstruct,\n   \
    \         ..\n        }\n    ));"
  path: tests/it/kani_obligations_symbolic_graphs.rs
  lines: 923-942
  reason: 'Inspected actual4495..023b diff: only the FR015AC108 tag is removed; TC025 and producer refusal assertions
    remain. Exact test1PASS, confined fmt-check0, traceTC025-only and matrix binder deletion corroborate the source
    fix.'
reviewed_findings:
- fnd: FND-001
  severity: medium
  confidence: high
  method: code-review
  check: test-intent
  artifact_id: FR-015-AC-106
  related_ids:
  - FR-015-AC-98
  - FR-015-AC-105
  - TC-025
  path: tests/it/kani_obligations_symbolic_graphs.rs
  lines: 582-593
  excerpt: "fn tc_025_real_kani_graph_relationship_claim_verifies_and_negated_claim_falsifies() {\n    use quire_contract_codegen::KaniRunOutcome;\n\
    \    for edge in [\"step\", \"kids\", \"litter\"] {\n        let healthy = fixture_data(\n            edge,\n\
    \            \"reaches(self, self, EDGE) or not reaches(self, self, EDGE)\",\n            3,\n            true,\n\
    \        );\n        retain_authored(&healthy, &format!(\"{edge}-healthy\"));\n        let healthy_harness =\
    \ harness(&healthy.package, &[0]);\n        let verified = run_graph_kani(&healthy_harness, &format!(\"{edge}-healthy\"\
    ));"
  finding: The relationship-side implementation has no qualified installed-backend controls for authored population
    extent +1/-1 or edge upper +1, which AC106 explicitly requires. The three new wrappers exercise authored negation,
    zero-length/self-loop evaluator faults and wrong-role dispatch only. The pure-bound oracle question on IR805
    comment10882823 remains unanswered; 40/40 PASS therefore does not discharge this current owned qualification
    gap. Define the intended mutation/original-domain observation with the requirement owner and add the corresponding
    failing controls, or obtain an explicit tracked deferral before merging the partial slice. Do not substitute
    traversal faults, unrelated assertions, safety failures or out-of-domain playbacks.
  requirement_path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  requirement_excerpt: 'each seeded mutant yields `Falsified` with an in-domain playback: reachability that allows
    a zero-length path (so an isolated `start = target` reads reachable), one that rejects a self-loop, one following
    another field or role, a population extent raised or lowered by one, an edge upper bound raised by one, and
    a negated claim.'
  recorded_head: 4495d32538ba2b9c0915d915bc0f7c8231ccdc60
  corrected_fields:
  - excerpt
  - lines
  correction_reason: The original excerpt was requirement text despite recording a test-source path. This appended
    correction supplies test-source text at that path; the original requirement excerpt is retained under its actual
    spec path. Finding id, severity, confidence, substance and verdict are unchanged.
- fnd: FND-002
  severity: medium
  confidence: high
  method: code-review
  check: trace
  artifact_id: FR-015-AC-108
  related_ids:
  - TC-025
  path: tests/it/kani_obligations_symbolic_graphs.rs
  lines: 925-945
  excerpt: "fn the_locked_producer_refuses_an_unbounded_relationship_before_ir_admission() {\n    let refusal =\
    \ try_fixture_source(\n        \"wild\",\n        \"not reaches(self,self,EDGE)\",\n        3,\n        true,\n\
    \        \"invariant\",\n        \"\",\n    )\n    .err()\n    .expect(\"the locked producer does not emit an\
    \ unbounded relationship claim\");\n    assert!(matches!(\n        refusal,\n        qsl_replay::CallSiteRefusal::Compile\
    \ {\n            code: qsl_replay::Code::UnsupportedConstruct,\n            ..\n        }\n    ));\n    assert!(refusal.to_string().contains(\"\
    expression-form\"));\n}\n"
  finding: The AC108 trace on the_locked_producer_refuses_an_unbounded_relationship_before_ir_admission names a
    generator-disposition criterion, but this test only asserts QSL CallSiteRefusal::Compile/UnsupportedConstruct
    before IR admission and never calls graph negotiation. It supplies upstream capability context and verifies
    no AC108 generator disposition. Remove that AC tag while retaining the producer observation and its TC025 context;
    do not fabricate a package or RequiresBound result.
  test_id: tests/it/kani_obligations_symbolic_graphs.rs::the_locked_producer_refuses_an_unbounded_relationship_before_ir_admission
  claimed_ac: FR-015-AC-108
  correct_ac: none (upstream producer capability context only)
  requirement_path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  requirement_excerpt: A graph claim settles by the disposition sequence in the Description (the paragraph "Disposition
    sequence for a graph claim"), with no harness, domain reduction or caller `max_expansions` substitute.
  recorded_head: 4495d32538ba2b9c0915d915bc0f7c8231ccdc60
  corrected_fields:
  - excerpt
  - lines
  correction_reason: The original excerpt was requirement text despite recording a test-source path. This appended
    correction supplies test-source text at that path; the original requirement excerpt is retained under its actual
    spec path. Finding id, severity, confidence, substance and verdict are unchanged.
scope:
- id: FR-015-AC-7
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Every generated harness contains exactly one non-vacuity cover; a precondition harness covers that the
    precondition holds within the IR bounds, and a contract harness''s cover stands after the contract call, so
    a run that satisfies every check without satisfying the cover is not a proof. The cover is the last statement
    of the harness, after every assertion the harness carries: the state-clause and frame-effect harnesses and the
    scalar harness place it after their assertions, and a contract harness''s checks are inside the contract call
    its cover follows. A cover that preceded an assertion'
- id: FR-015-AC-98
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A graph claim (one reachability application of either form above, at its `state_clause` node id and argument
    path) generates a harness when the admitted package gives its covering object type `E` a finite population `[0,
    N]` through `select_population` and the selected edge has finite multiplicity (for `quire.op.model.reaches_field`
    the field's `Reference`, `Option` or bounded `Collection` member; for `quire.op.model.reaches` the destination
    end's finite multiplicity). Every symbolic object and edge is bounded inclusively by those declarations, including
    zero and maximum where admitted.
- id: FR-015-AC-100
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: For the collection family, changing the selected field's authored cardinality or element bound, admitted
    body claim, customer subject or unwind changes the harness identity; equal admitted inputs regenerate byte-identically.
    The identity records every symbolic collection domain and the solver and complete ordered option vector, and
    FR-017 run evidence records the actual Kani, Rust and solver versions, unwind and every option used by the installed
    backend. For the graph family (AC-98), changing the claim's clause id or argument path, the reachability form,
    relationship or field, its role
- id: FR-015-AC-101
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A request whose symbolic collection domain exceeds the generator's representational or resource ceiling
    settles `unsupported` with a typed reason naming the failing domain or ceiling and no harness; it never tightens
    the model domain to make a proof tractable. A missing or malformed selected declaration likewise refuses with
    its typed reason and no harness. For a graph claim, a population or edge domain whose symbolic object or edge
    count exceeds the ceiling likewise settles `unsupported` naming the domain, with no harness and no reduced population
    or multiplicity. PARTIAL (IR-268, CG
- id: FR-015-AC-102
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'An admitted collection body claim is settled through the FR-015 V2 obligation path, one disposition
    per requested item; supported items alone emit harnesses. The finite-input lowerer and hand-built corpus of
    AC-86 to AC-95 neither supply the symbolic bounds nor classify the proof verdict. Graph items take the same
    V2 path (IR-805), one disposition per requested item, and are not implementation or proof evidence for IR-268.
    IMPLEMENTED for the collection family (IR-268, CG #396 fe20ecc): `tests/it/kani_obligations_symbolic_domains.rs`'
- id: FR-015-AC-105
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'For a graph claim the harness draws an object count in `[0, N]` of the one covering population and symbolic
    objects within it; for every object, the selected edge as its shape states: a `Reference` field has exactly
    one target, an `Option` field an absent or one target, a `Collection` field or finite multi-valued relationship
    end a count in `[lower, upper]` (a unique end admits no repeated target, a non-unique end repeats, and `lower
    > 0`, such as `{2,3}`, forces at least `lower` targets) and every target is an object of the population. Start
    and target are drawn from that population and'
- id: FR-015-AC-106
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'The independent native oracle replays the same authored clause over each concrete graph through QSL''s
    replay (`qsl_replay`), and does not substitute a separately authored reachability. With the installed backend,
    a QSL-emitted, IR-admitted healthy case yields `Verified` with its cover satisfied for one field-edge claim
    and one relationship-end claim over a `{0,3}` unique and a `{0,3}` non-unique end; each seeded mutant yields
    `Falsified` with an in-domain playback: reachability that allows a zero-length path (so an isolated `start =
    target` reads reachable), one that rejects a self-loop,'
- id: FR-015-AC-107
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Graph claims are located by `state_clause` node id and argument path: one reachability written twice
    in one condition yields two items at positions 0 and 1; one node under two clauses yields two items with different
    identities; a field-edge and a relationship-end reachability yield separate items; a collection claim, a clause
    with no reachability and a path that does not name a reachability application each settle `unsupported` naming
    the missing claim with no harness. A mixed request settles one disposition per item and a supported item''s
    harness is unchanged by a refused sibling.'
- id: FR-015-AC-108
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A graph claim settles by the disposition sequence in the Description (the paragraph "Disposition sequence
    for a graph claim"), with no harness, domain reduction or caller `max_expansions` substitute. While the population
    selection for the claim is `Unavailable` (no available population selection for this admitted claim), the claim
    settles `unsupported` naming it. Given a selection, `Unbounded` or `UnknownClosureOpenExtent`, that is a claim
    whose population has no finite authored bound (once a selection is available), settles `requires_bound` naming
    the population; an unbounded
- id: FR-017-AC-2
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: An absent launcher is refused with a typed reason naming its path before anything runs.
- id: FR-017-AC-4
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A run is verified only when the process exited successfully, the report states success, and every cover
    property is reported satisfied; a successful run with an unsatisfied cover is cover-unsatisfied with its satisfied
    and total counts; a report of success from an unsuccessfully exited process, a report with no cover property
    and a report with no successful check are each inconclusive with their own reason; console banners never decide
    the verdict.
- id: FR-017-AC-5
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A failed non-unwinding check with a concrete playback is falsified carrying that playback verbatim and
    never the playback of a satisfied cover; a failure with no playback is inconclusive for that reason; a failed
    unwinding assertion is inconclusive as an exhausted bound rather than falsified, and a succeeded unwinding check
    in a results listing is not a failure.
- id: FR-017-AC-6
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The execution evidence of a single run carries the obligation kind, the harness path, the launcher path,
    the complete argument vector, the unwind bound, the solver, the exit code and the outcome.
- id: FR-015
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'When a caller selects Kani generation for complete-V1 contracts whose scalar

    oracles FR-014 generated, the code generator shall emit one bounded Kani

    obligation per precondition, postcondition, invariant and frame condition,

    each bounded by its model-domain bounds. It is the Kani

    backend''s one generator

    ([ADR-001](../../decisions/ADR-001-overlapping-generators-and-input-models.md)),

    and every item it lowers is a claim over an admitted `quire.checked-package/v2`

    package.'
- id: FR-017
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  excerpt: 'When a caller runs an FR-015, FR-022/FR-014 or FR-035 harness, the code generator shall invoke the

    installed Kani backend and retain the backend''s own reported outcome as typed

    execution evidence. A caller may hand the generator several harnesses to run together

    (FR-017-AC-21 to FR-017-AC-25); the generator then forms groups, runs each

    group in one launcher process, and retains one evidence record per harness. Unless a

    statement says "batch" or "group", it describes a run of one harness.'
- id: TC-025
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: context_only
  excerpt: 'Verify that bounded contracts produce one harness per obligation kind and

    refuse non-finite obligations. IR-268 covers the collection half of planned V2

    symbolic obligations: the claim is an admitted QSL clause/function body over a

    selected model collection field. Graph reachability is required by

    FR-015-AC-98 and FR-015-AC-105 to FR-015-AC-108 (IR-805); it is not IR-268

    evidence. The finite-input collection and graph corpus is not proof evidence

    for either symbolic family.'
- id: AD-004
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: context_only
  excerpt: 'AD-001 states what the generator is and which seams it has. This AD states how `src/` is cut so

    that the code matches the subsystems in the Subsystem Registry of [spec.md](../spec.md): the

    directory tree, which module goes where, which way imports may point, and what is deleted. It is

    the design that IR-348 implements. It describes this repository only. Seams to IR, RT and QSL are

    AD-001''s. The replay seam (AD-002) and the evidence chain (AD-003) are the seam descriptions of

    IR-324; both are merged, and this AD references them.'
- id: StR-001
  path: spec/core/stakeholder/StR-001-traceable-generation.md
  role: context_only
  excerpt: 'Assurance engineers require that the generator shall derive executable, property-test, proof, and

    coverage artifacts reproducibly from one validated contract without hiding unsupported semantics,

    vacuity, rejection, or assumptions.'
- id: src/kani/generate/graph.rs
  path: src/kani/generate/graph.rs
  role: examined
  excerpt: "//! Source-bound symbolic reference graphs at state-clause argument paths (FR-015).\n//!\n//! Population\
    \ extent and edge multiplicity come only from admitted owner projections.\nuse quire_contract_model::selected_population::{\n\
    \    select_population, CheckedSelectedPopulation, CheckedSelectedPopulationOutcome as Population,\n    CheckedSelectedPopulationSource,\n\
    };\nuse quire_contract_model::{\n    CheckedCollectionKind, CheckedMemberType, CheckedNodeId, CheckedPackageV2,\n\
    \    CheckedReachesPopulationError, CheckedRelationshipEndError, CheckedSemanticNodeV2,\n};\nuse serde_json::Value;\n\
    \nuse"
- id: src/kani/generate/mod.rs
  path: src/kani/generate/mod.rs
  role: examined
  excerpt: '//! The one Kani generator: the request and result vocabulary, the negotiation entry and the

    //! family lowerers.


    // Census validation; interim, see the file header.

    pub(crate) mod census_validation;

    // The V1 clause lowering the precondition and contract families share.

    pub(crate) mod clause;

    // Bounded collection claims over admitted V2 model fields.

    pub(crate) mod collection;

    // Symbolic graph claims at exact state-clause argument paths.

    pub(crate) mod graph;

    // The contract family.

    pub(crate) mod contract;

    // The bounded-Kani corpus family.

    pub(crate) mod corpus;

    // The state-frame'
- id: src/kani/generate/negotiate.rs
  path: src/kani/generate/negotiate.rs
  role: examined
  excerpt: '//! Separate bounded Kani obligations with per-item backend negotiation (FR-015).

    //!

    //! Every requested item is accounted for before any harness bytes are exposed: each receives

    //! exactly one [`ObligationRecord`] whose disposition is `supported`, `requires_bound`,

    //! `unsupported` or `invalid_request` (Contract IR FR-036). One `invalid_request` rejects the

    //! whole request and no harness is returned. Otherwise one harness is returned per supported

    //! item and never one for any other disposition.

    //!

    //! Each supported item is one IR clause lowered to one obligation — a'
- id: src/kani/generate/outcome.rs
  path: src/kani/generate/outcome.rs
  role: examined
  excerpt: "//! The request and result vocabulary of Kani obligation negotiation (FR-015): the items a\n//! request\
    \ names, the records it returns and the reasons an item has no harness. A leaf inside\n//! `generate/`: it imports\
    \ no family.\n\nuse quire_contract_model::{\n    BoundPackage, CheckedModelFieldsError, CheckedNodeId, CheckedNodeTag,\
    \ CheckedPackageIncomplete,\n    CheckedPackageLimit, CheckedPackageRefusal, CheckedPackageV2, CheckedSourceMapEntry,\n\
    \    ClauseKind, ClauseRef, ContractPackageV1, DependencyIdentity, SourceSpan,\n};\nuse serde::{Serialize, Serializer};\n\
    \nuse crate::{\n   "
- id: src/kani/identity.rs
  path: src/kani/identity.rs
  role: examined
  excerpt: "//! The harness and identity record types of a generated Kani obligation.\n//!\n//! The generators build\
    \ these records and the runner, the witness join and the replay read\n//! them, so they live below both: a harness\
    \ is run from its identity and its source text, and\n//! no reader imports a generator.\n\nuse std::{num::NonZeroU64,\
    \ time::Duration};\n\nuse quire_canonical::FixedShape;\nuse quire_contract_model::{CheckedNodeId, ClauseRef,\
    \ DependencyIdentity, SourceSpan};\nuse serde::{Deserialize, Serialize};\n\nuse crate::{\n    core::artifact::Artifact,\n\
    \    core::identity::{HarnessPath, HarnessSymbol,"
- id: src/kani/run/harness.rs
  path: src/kani/run/harness.rs
  role: examined
  excerpt: "//! The generated harness of any kind this crate can run, and exactly what execution reads from\n//!\
    \ it (FR-017).\n\nuse crate::{\n    core::artifact::Artifact,\n    kani::abi::KaniSolver,\n    kani::identity::{\n\
    \        CollectionElementIdentity, KaniCollectionObligationHarness, KaniGraphObligationHarness,\n        KaniObligationHarness,\
    \ KaniScalarObligationHarness, ObligationKind, StateFrameHarness,\n        StateFrameProperty, SymbolicArgumentBounds,\
    \ SymbolicBounds,\n    },\n};\n\n/// A generated harness of either kind this module can run.\n///\n/// The two\
    \ kinds carry their own identity types but"
- id: src/lib.rs
  path: src/lib.rs
  role: examined
  excerpt: '//! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.


    #![forbid(unsafe_code)]

    #![deny(missing_docs)]


    // Integration fixture modules name this crate as an external dependency.

    #[cfg(test)]

    extern crate self as quire_contract_codegen;


    // Private integration control needs internal generation and execution in the library test binary.

    #[cfg(test)]

    #[path = "../tests/it/kani_render_private.rs"]

    mod kani_render_private;


    #[cfg(test)]

    #[path = "../tests/it/kani_scalar_native_private.rs"]

    mod kani_scalar_native_private;


    #[cfg(all(test, target_os ='
- id: tests/it/kani_obligations.rs
  path: tests/it/kani_obligations.rs
  role: examined
  excerpt: '//! FR-015 separate bounded Kani obligations, FR-017 Kani execution, and Contract IR FR-036

    //! backend negotiation.

    //!

    //! The default lane checks negotiation, refusal and harness shape without running Kani. The

    //! `kani` lane (`make kani`, `#[ignore]` here) measures the installed backend, runs real

    //! verified and seeded-failing harnesses one at a time, and writes execution evidence under

    //! `CARGO_TARGET_TMPDIR/kani-obligation-evidence`.


    // package.rs holds a process-global `application_registry()` static keyed by small integer

    // fixture codes that this file and'
- id: tests/it/kani_obligations_symbolic_domains.rs
  path: tests/it/kani_obligations_symbolic_domains.rs
  role: examined
  excerpt: "//! Source-backed symbolic collection and graph obligation fixtures.\n//!\n//! QSL emits every admitted\
    \ package through its public `call_site` facade from\n//! native source and a test-authored domain model document.\
    \ No Contract IR\n//! package wire is constructed in this module.\n\nuse crate::scratch_crate::{runtime_dependency,\
    \ write_manifest, TemporaryDirectory};\nuse ix_trace_rs::trace;\nuse qsl_replay::{\n    call_site, object_reference_witness,\
    \ replay, write_object_environment, ByteDigest, Category,\n    DeclarationKey, DependencyInput, DigestDomain,\
    \ DigestRecord, DomainPackageRef,\n   "
- id: tests/it/kani_obligations_symbolic_graphs.rs
  path: tests/it/kani_obligations_symbolic_graphs.rs
  role: examined
  excerpt: "//! Authored QSL symbolic graph controls (FR-015).\nuse ix_trace_rs::trace;\nuse qsl_replay::{call_site,\
    \ DependencyInput, Identifier, QualifiedName, SourceIdentity};\nuse quire_contract_model::{\n    CheckedPackageEvidence,\
    \ CheckedPackageReadLimits, CheckedPackageV2, CheckedPackageV2ReadResult,\n};\nuse serde_json::{json, Value};\n\
    \nfn fixture(edge: &str) -> CheckedPackageV2 {\n    fixture_condition(edge, \"not reaches(self, self, EDGE)\"\
    , 3, true)\n}\n\nfn fixture_condition(edge: &str, condition: &str, maximum: u64, bounded: bool) -> CheckedPackageV2\
    \ {\n    fixture_data(edge, condition, maximum,"
- id: tests/it/layout.rs
  path: tests/it/layout.rs
  role: examined
  excerpt: '//! AD-004 layout decisions L-1 and L-2 and FR-024-AC-10, checked over `src/`.

    //!

    //! L-1: `src/` holds the eight subsystem directories and `lib.rs`, and every module the AD''s

    //! module-to-subsystem map names sits in the directory the map names.

    //!

    //! L-2: no file imports an item through the crate root, an import names a directory the

    //! importing directory may reach, the module graph has no cycle (`#[cfg(test)]` modules

    //! included, because the scan reads whole files), and the order inside `kani/` holds.

    //!

    //! The scan reads `crate::` paths in code. Comments and string and'
- id: tests/it/main.rs
  path: tests/it/main.rs
  role: examined
  excerpt: '//! IR-237: the integration test crate. Cargo builds a `tests/<name>/main.rs` directory as one

    //! test target automatically (no `[[test]]` entry needed), so every former top-level `tests/*.rs`

    //! file below is a module of the single `it` binary instead of its own executable -- each still

    //! links the crate and its dependencies once per `cargo test` invocation rather than once per

    //! former file.

    //!

    //! `common`, `composite_equality_support`, `exact_scalar_support`, `exact_function_support` and

    //! `fixtures` stay at `tests/`, one level up from here; the modules that use them reach'
bindings:
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::authored_relationship_domains_use_population_and_typed_destination_multiplicity
  ac_id: FR-015-AC-98
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::same_authored_clause_native_replay_distinguishes_isolation_self_loop_and_selected_role
  ac_id: FR-015-AC-98
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::authored_pre_post_and_invariant_forms_keep_the_clause_kind
  ac_id: FR-015-AC-98
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::actual_second_role_carrier_and_native_snapshot_preserve_both_relationships
  ac_id: FR-015-AC-98
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::a_subtype_operand_uses_the_near_ends_covering_population_without_a_subtype_cap
  ac_id: FR-015-AC-98
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::distinct_operation_operands_are_drawn_independently_from_the_same_population
  ac_id: FR-015-AC-98
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::repeated_occurrences_keep_their_paths_and_equal_inputs_regenerate_identically
  ac_id: FR-015-AC-100
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::graph_identity_binds_subject_unwind_solver_and_complete_ordered_options
  ac_id: FR-015-AC-100
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::zero_population_is_retained_and_inadequate_resources_refuse_without_domain_reduction
  ac_id: FR-015-AC-101
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::refused_graph_sibling_leaves_supported_artifact_byte_identical
  ac_id: FR-015-AC-102
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::authored_relationship_domains_use_population_and_typed_destination_multiplicity
  ac_id: FR-015-AC-105
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::zero_population_is_retained_and_inadequate_resources_refuse_without_domain_reduction
  ac_id: FR-015-AC-105
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::a_subtype_operand_uses_the_near_ends_covering_population_without_a_subtype_cap
  ac_id: FR-015-AC-105
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::distinct_operation_operands_are_drawn_independently_from_the_same_population
  ac_id: FR-015-AC-105
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::same_authored_clause_native_replay_distinguishes_isolation_self_loop_and_selected_role
  ac_id: FR-015-AC-106
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::tc_025_real_kani_graph_relationship_claim_verifies_and_negated_claim_falsifies
  ac_id: FR-015-AC-106
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::tc_025_real_kani_positive_length_evaluator_rejects_zero_length_and_self_loop_mutants
  ac_id: FR-015-AC-106
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::actual_second_role_carrier_and_native_snapshot_preserve_both_relationships
  ac_id: FR-015-AC-106
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::tc_025_real_kani_wrong_role_dispatch_falsifies_the_same_authored_clause
  ac_id: FR-015-AC-106
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::actual_field_producer_has_no_population_link_and_location_refuses_first
  ac_id: FR-015-AC-107
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::repeated_occurrences_keep_their_paths_and_equal_inputs_regenerate_identically
  ac_id: FR-015-AC-107
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::refused_graph_sibling_leaves_supported_artifact_byte_identical
  ac_id: FR-015-AC-107
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::authored_pre_post_and_invariant_forms_keep_the_clause_kind
  ac_id: FR-015-AC-107
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::the_same_reachability_under_two_actual_clauses_has_distinct_identities
  ac_id: FR-015-AC-107
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::a_collection_condition_and_a_clause_without_reachability_refuse_missing_claim
  ac_id: FR-015-AC-107
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::actual_field_producer_has_no_population_link_and_location_refuses_first
  ac_id: FR-015-AC-108
  trace: correct
- test_id: tests/it/kani_obligations_symbolic_graphs.rs::authored_unbound_relationship_refuses_unavailable_without_inventing_a_binding
  ac_id: FR-015-AC-108
  trace: correct
removed_binding:
  test_id: tests/it/kani_obligations_symbolic_graphs.rs::the_locked_producer_refuses_an_unbounded_relationship_before_ir_admission
  ac_id: FR-015-AC-108
  outcome: removed by actual source fix for SR14100/FND002; TC025 retained
current_mergeability: 'BLOCKED: SR14100/FND001 still-open; no owner deferral/waiver/partial-merge approval'
whole_repo_verdict: not assessed under code-review
new_static_delta: no new gap; one incorrect binding removed
resource: No reviewer heavy reservation acquired or compiler/fmt/Kani/cargo command run. Parent reports IR17 coder
  now owns sole heavy reservation including gaps; any final guard processes must be attributed to that owner, not
  treated as reviewer leakage.
```
