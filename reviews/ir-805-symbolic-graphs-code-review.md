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

### Round 2

Round2 independently audits frozen ebdea against landed main22a and the two domain-control commits after preserved prior custody. FND001 is fixed by actual independent D/G conformance/completeness controls under the clarified AC106; FND002 remains fixed. No new current finding. The available relationship/domain-oracle slice is review-clear, subject to root final gates and exact trailing custody. Field success remains missing, so no full AC106/IR805, parent or legacy-corpus retirement acceptance is given. All original Findings/Verdicts, prefixes, metadata corrections and private parts remain unchanged.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Independent authored D and observed sampler G now qualify all three pure bound mutations for unique/nonunique profiles under landed IR939, with actual installed/backend boundary evidence. |

FND002 was already fixed in round1 and remains fixed; no new disposition row is needed. No new findings were discovered.

The new test seam is cfg(test) only: it lowers the same admitted original package/path, changes only population.maximum+1/-1 or edge.upper+1 and calls the same renderer. Package, claim, carrier and positive-length evaluator stay unchanged; arrays/unroll dimensions follow the changed domain. D reads the retained authored QSL N4 bound and original model0..3 multiplicity/uniqueness before any generator call, cross-checking actual reaches_population/select_population/relationship_end projections. G parses actual Rust inputs, all pre-call assumptions, degree/edge initializers, positional arguments, helper parameter array capacities and actual active-row/slot guards. Capacity rejection precedes any row/slot access. Dormant padding normalizes to zero only after active interpretation/range recognition; for this actual emitted guard set each inactive range admits zero and inactive constraints are disabled. Source-corruption native controls reject changed guards/capacity and observe narrowed ranges/deleted uniqueness while D remains original. G is not reconstructed from identity bounds.

The5x4 carrier has28 independently drawn nonnegative scalar ranges: count0..5, operands0..4, five degrees0..4 and20 targets0..4. No D/G, active-domain or uniqueness assumptions filter candidates. Every active graph in the healthy and three mutated domains is representable; inactive padding is not an object/edge. D covers zero population without live operands, operand equality, active degree/target validity and uniqueness. Healthy runs universally prove both implications and separately cover N4/degree0/degree3. Actual mutant runs fail only the named domain assertions; separate installed probe runs cover pop+ count5/zeroedges (Gtrue/Dfalse), pop- count4/zeroedges despite3-row capacity (Dtrue/Gfalse), and edge+ count4/one degree4 row/others empty, with0,1,2,3 unique targets or repeated0 nonunique. All18 raw reports, consoles, sampler/oracle sources, original QSL/model/package/provenance and typed witnesses were read and independently reconciled;12Verified/6Falsified, one satisfied final cover per run, no safety failure. Tool evidence records actual Kani0.68.0/rustc1.100.0-nightly/CBMC6.11.0/CaDiCaL3.0.0, unwind8, ordered argv and per-check data. The before/after16GiB/noSwap observations are enclosing-test-scope evidence, not isolated production bounded-execution memory attestations. Mutant-only domain witnesses receive no original-domain clause-replay credit.

Original same-clause zero-length/self-loop/wrong-role/negated-claim controls and FND002'sTC025-only observation remain preserved. Shared model_document's function body is an exact move; the original graph fixture still uses incoming upper9, while the new N4/U3 domain fixture uses20 to accommodate every union-carrier inverse end. No copied checked wire, alternate generator, pin/lock/spec/CI/scanner change or resource-driven domain reduction exists. The test-only sampler observer is an independent property of actual emitted source, not a new public graph schema.

L2 scrutiny includes graph.rs inline cfg(test), new library-included orchestration, its include! predicate and path-included fixture/model/sampler plus all other Rust roots and the shared std-only core scratch allocator. The actual layout scanner walks src and inline test code; its8PASS alone does not inspect external path-included library test files. Those files and their actual module/import graph were manually inspected here: orchestration is crate-root test code importing kani generate/identity/classify/output/test_support; fixture/model use external owners; sampler uses only parent CandidateGraph and parser/standard dependencies. No root re-export import or oracle/publication-to-kani backedge is introduced. Canonical trace attributes are on exactly the two real test wrappers, not production/helper/proof functions.

The new ignored private wrapper is deliberately invoked with --lib explicitly and is not selected by the unchanged make kani-gate --test it filters. Its actual18 runs qualify this separate observation; a later full lane is not claimed to have run this wrapper. No new lane coverage or whole AC106 completion is inferred. No applicable AssuranceProfile exists and no own spec edit requires spec-review methods. Broad optional whole-repo semantic review remains skipped; expressly assigned graph/domain semantics were inspected. Historical4495 full40/40 remains historical; current ebdea has retained focused formatting/lint/native1PASS/43affected4ignored/8layout evidence and exactly one new private wrapper1PASS101.69s. Current full author/aggregate/merger gates are not supplied by these focused results.


```yaml
repo: agent-ix/quire-contract-codegen
ticket: IR-805
pr: quire-contract-codegen#474
round: 2
checked_head: ebdea6ce9548a36ac35b3289afa7c16fd7238705
assigned_base: 22a6ccf5dfe9fe1b9a1c5fe90521285914fb17ef
previous_reviewed_head: 023b4b579a82f3bcba8fbaeb21fb71f8ad169bbd
original_review_run: 231a982b-f9b9-436b-b6fa-bbd9315d14e5
model: gpt-6.1-sol
run: 5a28ed0c-e957-4784-adef-7ecdc9142eeb
dispositions_owner: SR-14100
dispositions:
- fnd: FND-001
  outcome: fixed
  path: tests/it/kani_graph_domain_private.rs
  lines: 352-355
  after_excerpt: "if assertions {\n        body.push_str(\"assert!(!generated || d, \\\"generated graph domain conforms\
    \ to original authored domain\\\");\\nassert!(!d || generated, \\\"generated graph domain includes every original\
    \ authored graph\\\");\\n\");\n    }"
  reason: 'Fixed under the user-selected clarification landed by IR939/PR479 in base22a: actual production-domain
    construction is mutated at population+1/-1 or edge-upper+1 while D remains grounded in original authored QSL/model
    and typed projections. The independently parsed sampler G includes actual ranges, arrays, capacity rejection
    and active guards. One retained installed wrapper at clean ebdea executes18 runs across unique/nonunique profiles:12Verified
    and6expectedFalsified at the conformance/completeness assertions, with all six required mismatch boundary probes
    and native D/G agreement. Independent read-only audit decoded every raw backend witness, evaluated original
    D and actual-source G, and found no safety failure or D/G membership assumption. This is a substantive oracle/control
    fix, not a waiver or original-clause truth claim.'
  requirement_path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  requirement_excerpt: 'With installed Kani, the separate domain oracle asserts both `G(g) implies

    D(g)` (conformance) and `D(g) implies G(g)` (completeness) over a common

    bounded candidate carrier.'
previously_resolved:
- fnd: FND-002
  reason: TC025-only producer observation remains in the actual current source; no new disposition row is needed.
  latest_status: fixed
new_findings: []
open_owned_findings: []
scope_qualification: Available relationship integration and separate D/G oracle only. Field success remains independently
  missing under QSL820/owner API; full AC106/IR805, parent work and corpus retirement are not accepted.
mergeability: Review-clear for available relationship/domain-oracle scope; latest exact custody and root final integrated
  make ci/full Kani remain required. No merge performed.
resource: NO reviewer Cargo/rustc/fmt/Kani/CBMC/backend command or heavy reservation. Root owns sole IR631 make
  ci/full Kani reservation including gaps. No author/full aggregate/backend freshness rerun performed.
artifact_id: SR-14100
method: code-review
clean: true
clean_definition: No current owned or new finding in the reviewed available slice; inherited whole_repo FAIL and
  field qualification limits remain explicit.
whole_repo_verdict: not assessed by code-review
current_delta_verdict: PASS for available relationship/domain-oracle scope
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
  excerpt: 'The independent native oracle replays the same authored clause over each original-domain concrete graph
    through QSL''s replay (`qsl_replay`), and does not substitute a separately authored reachability. With the installed
    backend, a QSL-emitted, IR-admitted healthy case yields `Verified` with its cover satisfied for one field-edge
    claim and one relationship-end claim over a `{0,3}` unique and a `{0,3}` non-unique end. Each semantic mutant
    yields `Falsified` at the clause assertion with an original-domain playback and native replay of that same authored
    clause: reachability that allows a'
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
- id: NFR-006
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: context_only
  excerpt: "- The repository shall define the **Kani-touching set** as the changed paths matching\n  `src/kani/**`,\
    \ `src/oracle/**`, `src/routed/**`, `src/replay/**`, `src/core/**`,\n  `src/publication/**`, `tests/it/kani_*.rs`,\
    \ `tests/it/skeleton_spine.rs`,\n  `tests/it/bounded_kani_corpus.rs`, `tests/it/scratch_crate.rs`,\n  `tests/exact_scalar_support/package.rs`,\
    \ `tests/checked_package_support/base.rs`,\n  `tests/checked_package_support/rekey.rs`,\n  `tests/state_frame_support/**`,\
    \ `schemas/kani-*.schema.json`,\n  `schemas/generated-rust-kani-*.schema.json`, `Cargo.toml` and\n  `Cargo.lock`.\n\
    - The `make"
- id: FR-041
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: context_only
  excerpt: 'When the repository claims proof coverage for a generated-harness module, its existing

    [`make kani-gate`](../non-functional/NFR-006-real-kani-lane-gating.md) SHALL accept that claim only

    after a real Kani run discharges a proposition in the module and an executed mutation inside the

    module falsifies a proof check. The gate applies the proof-stage acceptance rule of QSL ADR-011

    §2.3 to CG''s claimed modules. It makes no claim about a module absent from its declared list or

    about a semantic stage the test input did not traverse.'
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


    #[cfg(test)]

    #[path ='
- id: tests/graph_support/domain_predicate.rs
  path: tests/graph_support/domain_predicate.rs
  role: examined
  excerpt: "// Independent original-domain membership over verification storage, never production arrays.\n#[derive(Clone,\
    \ Debug)]\nstruct CandidateGraph {\n    count: i64,\n    start: i64,\n    target: i64,\n    degrees: [i64; 5],\n\
    \    edges: [[i64; 4]; 5],\n}\n\n#[derive(Clone, Copy, Debug)]\nstruct AuthoredDomain {\n    maximum: i64,\n\
    \    lower: i64,\n    upper: i64,\n    unique: bool,\n    same_operand: bool,\n}\n\nfn authored_admits(g: &CandidateGraph,\
    \ d: AuthoredDomain) -> bool {\n    if g.count < 0 || g.count > d.maximum {\n        return false;\n    }\n\
    \    if g.count == 0 {\n        return true;\n    }\n    if"
- id: tests/graph_support/fixture.rs
  path: tests/graph_support/fixture.rs
  role: examined
  excerpt: "//! Authored QSL emission and strict IR admission shared by graph controls.\nuse qsl_replay::{call_site,\
    \ DependencyInput, Identifier, SourceIdentity};\nuse quire_contract_model::{\n    CheckedPackageEvidence, CheckedPackageReadLimits,\
    \ CheckedPackageV2, CheckedPackageV2ReadResult,\n};\nuse serde_json::{json, Value};\npub(crate) struct AuthoredGraph\
    \ {\n    pub(crate) package: CheckedPackageV2,\n    pub(crate) emitted_bytes: Vec<u8>,\n    pub(crate) source:\
    \ String,\n    pub(crate) model_bytes: Vec<u8>,\n    pub(crate) model_digest: [u8; 32],\n    pub(crate) site:\
    \ qsl_replay::ClauseSite,\n    pub(crate)"
- id: tests/graph_support/model.rs
  path: tests/graph_support/model.rs
  role: examined
  excerpt: "//! The shared authored domain document used by collection and graph QSL fixtures.\nuse serde_json::{json,\
    \ Value};\nconst NODE_ID: &str = \"ix://ir268.test/domain/Node\";\nconst SMALL_ID: &str = \"ix://ir268.test/domain/SmallInt\"\
    ;\nconst POPULATION_ID: &str = \"ix://ir268.test/domain/CollectionNodes\";\nconst SOURCE_ID: &str = \"ix://ir268.test/spec\"\
    ;\nconst ZERO_SHA256: &str = \"sha256:0000000000000000000000000000000000000000000000000000000000000000\";\n\n\
    pub(crate) fn model_document() -> Value {\n    let field = |name: &str,\n                 type_ref: &str,\n\
    \                 lower: u64,\n                "
- id: tests/graph_support/sampler.rs
  path: tests/graph_support/sampler.rs
  role: examined
  excerpt: "//! Observe the actual emitted sampler. Unrecognized admission or storage syntax refuses.\nuse super::CandidateGraph;\n\
    use quote::ToTokens;\nuse std::collections::{BTreeMap, BTreeSet};\nuse syn::{Expr, Item, Pat, Stmt};\n\n#[derive(serde::Serialize)]\n\
    pub(super) struct SamplerObservation {\n    pub(super) population_capacity: usize,\n    pub(super) row_capacities:\
    \ Vec<usize>,\n    pub(super) scalar_bindings: BTreeMap<String, String>,\n    pub(super) assumptions: Vec<String>,\n\
    \    #[serde(skip)]\n    predicates: Vec<Expr>,\n    pub(super) input_names: Vec<String>,\n    pub(super) scalar_ranges:"
- id: tests/it/kani_graph_domain_private.rs
  path: tests/it/kani_graph_domain_private.rs
  role: examined
  excerpt: "//! Private orchestration of the AC106 original-domain / actual-sampler controls.\n//! No production\
    \ API is exposed for the carrier or bound mutations.\n#[path = \"../graph_support/fixture.rs\"]\nmod fixture;\n\
    #[path = \"../graph_support/model.rs\"]\nmod model;\ninclude!(\"../graph_support/domain_predicate.rs\");\n#[path\
    \ = \"../graph_support/sampler.rs\"]\nmod sampler;\nuse crate::kani::{\n    classify::KaniRunOutcome,\n    generate::{\n\
    \        graph::{bound_mutant_for_test, BoundMutation},\n        negotiate::negotiate_kani_obligations,\n  \
    \      outcome::{KaniObligationOutcome, KaniObligationRequest,"
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
  excerpt: "//! Authored QSL symbolic graph controls (FR-015).\nuse ix_trace_rs::trace;\nuse qsl_replay::{Identifier,\
    \ QualifiedName};\nuse quire_contract_model::CheckedPackageV2;\nuse serde_json::{json, Value};\n\nfn fixture(edge:\
    \ &str) -> CheckedPackageV2 {\n    fixture_condition(edge, \"not reaches(self, self, EDGE)\", 3, true)\n}\n\n\
    fn fixture_condition(edge: &str, condition: &str, maximum: u64, bounded: bool) -> CheckedPackageV2 {\n    fixture_data(edge,\
    \ condition, maximum, bounded).package\n}\n#[path = \"../graph_support/fixture.rs\"]\nmod graph_fixture;\nuse\
    \ graph_fixture::AuthoredGraph;\nfn fixture_data(edge:"
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
- id: Original::new
  path: tests/it/kani_graph_domain_private.rs
  role: examined
  excerpt: "fn new(role: &str) -> Self {\n        let fixture = fixture::try_fixture_source(\n            fixture::ModelInput\
    \ {\n                document: model::model_document(),\n                incoming_upper: 20,\n            },\n\
    \            role,\n            \"reaches(self,self,EDGE) = reaches(self,self,EDGE)\",\n            4,\n   \
    \         true,\n            \"invariant\",\n            \"\",\n        )\n        .unwrap();\n        // D\
    \ is fixed from the original authored inputs and checked projections BEFORE the\n        // generator (healthy\
    \ or mutant) is invoked. No harness metadata supplies D.\n        let decla"
- id: oracle_source::domain_assertions
  path: tests/it/kani_graph_domain_private.rs
  role: examined
  excerpt: "if assertions {\n        body.push_str(\"assert!(!generated || d, \\\"generated graph domain conforms\
    \ to original authored domain\\\");\\nassert!(!d || generated, \\\"generated graph domain includes every original\
    \ authored graph\\\");\\n\");\n    }\n    body.push_str(&format!(\n        \"kani::cover!({cover}, \\\"domain\
    \ carrier boundary witness\\\");\\n\"\n    ));\n    let source = format!(\"// SPDX-License-Identifier: MIT OR\
    \ Apache-2.0\\n// Private AC106 domain oracle; production reachability is not asserted here.\\n{}\\n{}\\n#[cfg(kani)]\\\
    nmod {} {{ use super::*; #[kani::proof] fn {}() {{ {body} }} }}\\n\",\n        inc"
- id: installed_domain_wrapper
  path: tests/it/kani_graph_domain_private.rs
  role: examined
  excerpt: "fn tc_025_real_kani_graph_domain_conformance_completeness_and_bound_mutants() {\n    for role in [\"\
    kids\", \"litter\"] {\n        let original = Original::new(role);\n        let healthy = original.generated(None);\n\
    \        let observed = SamplerObservation::parse(\n            &healthy.rust.contents,\n            healthy.identity.harness_symbol.as_str(),\n\
    \        );\n        let max = boundary(4, 0, original.domain.unique);\n        let label = format!(\"{role}-healthy-domain\"\
    );\n        retain(&original, &observed, &label);\n        let source = oracle_source(\n            &original,\n\
    \            &healthy,"
- id: authored_admits
  path: tests/graph_support/domain_predicate.rs
  role: examined
  excerpt: "fn authored_admits(g: &CandidateGraph, d: AuthoredDomain) -> bool {\n    if g.count < 0 || g.count >\
    \ d.maximum {\n        return false;\n    }\n    if g.count == 0 {\n        return true;\n    }\n    if g.start\
    \ < 0 || g.start >= g.count || g.target < 0 || g.target >= g.count\n        || (d.same_operand && g.start !=\
    \ g.target)\n    {\n        return false;\n    }\n    for row in 0..5 {\n        if row as i64 >= g.count {\n\
    \            continue;\n        }\n        let degree = g.degrees[row];\n        if degree < d.lower || degree\
    \ > d.upper {\n            return false;\n        }\n        for slot in 0..4 {"
- id: SamplerObservation::parse
  path: tests/graph_support/sampler.rs
  role: examined
  excerpt: "pub(super) fn parse(source: &str, proof_name: &str) -> Self {\n        let parsed = syn::parse_file(source).expect(\"\
    actual generated Rust parses\");\n        let proof = parsed\n            .items\n            .iter()\n    \
    \        .find_map(|item| {\n                let Item::Mod(module) = item else { return None };\n          \
    \      module.content.as_ref()?.1.iter().find_map(|item| {\n                    let Item::Fn(function) = item\
    \ else {\n                        return None;\n                    };\n                    (function.sig.ident\
    \ == proof_name).then_some(function)\n                })"
- id: SamplerObservation::values
  path: tests/graph_support/sampler.rs
  role: examined
  excerpt: "fn values(&self, g: &CandidateGraph) -> Option<BTreeMap<String, i64>> {\n        if g.count < 0 || g.count\
    \ > self.population_capacity as i64 {\n            return None;\n        }\n        for row in 0..usize::try_from(g.count).ok()?\
    \ {\n            if g.degrees[row] < 0 || g.degrees[row] > self.row_capacities[row] as i64 {\n             \
    \   return None;\n            }\n        }\n        // This uses the same observed position map as the emitted\
    \ predicate, with no bound\n        // or multiplicity inference. Values are assigned only after capacity rejection.\n\
    \        let mut values = BTreeMap::new();"
- id: SamplerObservation::source
  path: tests/graph_support/sampler.rs
  role: examined
  excerpt: "pub(super) fn source(&self) -> String {\n        let mut source = format!(\"fn generated_admits(g: &CandidateGraph)\
    \ -> bool {{\\nif g.count < 0 || g.count > {} {{ return false; }}\\n\", self.population_capacity);\n       \
    \ for (row, slots) in self.row_capacities.iter().enumerate() {\n            source.push_str(&format!(\"if g.count\
    \ > {row} && (g.degrees[{row}] < 0 || g.degrees[{row}] > {slots}) {{ return false; }}\\n\"));\n        }\n \
    \       for (symbol, binding) in &self.scalar_bindings {\n            source.push_str(&format!(\"let {symbol}:\
    \ i64 = {binding};\\n\"));\n        }\n        for predicate in &s"
- id: bound_mutant_for_test
  path: src/kani/generate/graph.rs
  role: examined
  excerpt: "pub(crate) fn bound_mutant_for_test(\n    request: &KaniObligationRequest<'_>,\n    package: &CheckedPackageV2,\n\
    \    clause: &CheckedNodeId,\n    path: &[usize],\n    mutation: BoundMutation,\n) -> KaniGraphObligationHarness\
    \ {\n    let mut lowered = match lower_graph(package, clause, path) {\n        Ok(lowered) => lowered,\n   \
    \     Err(_) => panic!(\"the original authored graph must lower before mutation\"),\n    };\n    match mutation\
    \ {\n        BoundMutation::PopulationPlus => {\n            lowered.population.maximum = lowered.population.maximum.checked_add(1).unwrap();\n\
    \        }\n        BoundMutation::"
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
- test_id: tests/it/kani_graph_domain_private.rs::actual_sampler_observation_preserves_capacity_guards_padding_and_authored_domain_independence
  ac_id: FR-015-AC-106
  trace: correct
- test_id: tests/it/kani_graph_domain_private.rs::tc_025_real_kani_graph_domain_conformance_completeness_and_bound_mutants
  ac_id: FR-015-AC-106
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

```
