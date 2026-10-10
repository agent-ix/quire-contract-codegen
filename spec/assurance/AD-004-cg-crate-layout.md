---
id: AD-004
title: "CG crate layout: subsystem directories, one input model, one Kani generator"
type: ArchitectureDescription
status: proposed
owner: codegen-maintainers
system: quire-contract-codegen src layout (module tree, dependency direction, the one Kani generator, the Kani run and output edge, shared helpers, identities, IR node access)
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-001
    type: references
  - target: ix://agent-ix/quire-contract-codegen/AD-002
    type: references
  - target: ix://agent-ix/quire-contract-codegen/AD-003
    type: references
  - target: ix://agent-ix/quire-contract-codegen/ADR-001
    type: references
  - target: ix://agent-ix/quire-contract-codegen/ADR-002
    type: references
  - target: ix://agent-ix/quire-contract-codegen/ADR-003
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-005
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: references
  - target: ix://agent-ix/quire-contract-ir/ADR-0056
    type: references
---
# CG crate layout

AD-001 states what the generator is and which seams it has. This AD states how `src/` is cut so
that the code matches the subsystems in the Subsystem Registry of [spec.md](../spec.md): the
directory tree, which module goes where, which way imports may point, and what is deleted. It is
the design that IR-348 implements. It describes this repository only. Seams to IR, RT and QSL are
AD-001's. The replay seam (AD-002) and the evidence chain (AD-003) are the seam descriptions of
IR-324; both are merged, and this AD references them.

## System Boundary

In scope: every file under `src/`, the import edges between them, the public entry points that
`lib.rs` re-exports, and the helper, identity, node-access and version-profile code the modules
share. Also in scope are the two layout exceptions recorded in `spec/spec.md` (FR-005 and the
SuiteRegistry file), which the Subsystem Registry defers to this AD and which this AD settles.

Out of scope: behaviour. Every requirement keeps its id and its criteria. No requirement id is
minted here; the labels `L-1` and onward are local to this document, as the labels in the other
ADs are. Two places record the design intent of a behaviour change that its own requirement ticket
carries, and say so where they do: step 4c (a V2 input for the contract families) and step 5 (the
terminal map). They add no behaviour claim of their own. Also out of scope: IR's and RT's own
layouts, QSL's types, the error envelope, the `unreachable!` arms on RT enums and the test
conventions. They are listed under Risks as uncovered.

## Views

The layout is described as what is there now, the target tree, the map from every module to its
subsystem, the dependency direction, the one Kani generator, the Kani run and output edge, the one
input model, the shared core, and the two recorded exceptions.

### Current state (measured)

Measured on this repository after the subsystem restructure (PR 213), by reading `src/lib.rs` and
grepping `src/`. Counts exclude `#[cfg(test)]` modules unless said so. The line numbers cited in
this section were read then and are not kept current; the item names are the reference. The step
2f item map below is keyed by item name, never by line.

- `src/` holds 26,687 lines in 27 modules, all flat. `lib.rs` declares them, and only
  `bound_strategy` is a directory (and the one `pub mod`, at `lib.rs:49`). The largest are
  `exact_scalar` (4170 lines), `kani_obligations` (2697), `composite_equality` (1702),
  `kani_execution` (1492), `bounded_kani_corpus` (1480), `exact_function` (1384), `oracle` (1306)
  and `state_frame` (1156).
- The directories do not match the spec subsystems. The registry names seven subsystems and
  `spec/` has a directory for each, but `src/` has none of them.
- **Import cycles.** The ticket names two. Re-measured, neither is a cycle in non-test code.
  `kani_execution` imports `kani_obligations`, `kani_transcript` and `state_frame`, and none of
  those imports it back; `kani_obligations` imports no `kani_execution` item. The one back-edge in
  the pair `kani_execution` and `kani_transcript` is `use crate::kani_execution::{classify_kani_run, ...}`
  inside `kani_transcript`'s own `#[cfg(test)]` module, which also imports `KaniInconclusiveReason`
  and `KaniRunOutcome`. Step 2f dissolves it by moving the tests that classify to
  `kani/classify.rs` (see the step 2f item map). No
  strongly connected component exists among the non-test `use crate::` edges. The structure that
  produces cycles is still there, for two reasons:
  - 12 files import at least one item through the crate root (`use crate::{..., OracleRequest, ...}`
    beside module paths, and `use crate::OperationClaim`), at the time of writing, measured after
    step 2c (an earlier draft of this AD said 17). The 12 are `bound`, `bound_coverage`,
    `bounded_kani_corpus`, `harness`, `kani`, `kani_obligations`, `routed_generation`,
    `state_frame`, `strategy`, `bound_strategy/census`, `bound_strategy/generation` and
    `bound_strategy/population`; the count includes `#[cfg(test)]` modules, and six further files
    use `use crate::{...}` with module paths only. Two more files reach a root item by an inline
    path, not a `use` line (`crate::GenerationDiagnostic` in `kani` and `kani_obligations`). So the real edge is hidden behind `lib.rs`'s
    `pub use` list and an import of a root item can close a loop with no module name in sight.
  - Utility modules sit in the wrong place. The V1 `kani` module is a helper library for the V2
    path (`kani_obligations`, `state_frame`, `kani_witness_join` and `bounded_kani_corpus` import
    `adapter_options`, `i64_literal`, `readable_component`, `KaniBindingRole`, `KaniSolver` and
    `deterministic_json` from it), and `exact_scalar` is a walker library (`kani_obligations` and
    `state_frame` import `bound_members`, `aggregate_members`, `operand_ranges`, `literal` and the
    `*_MEMBERS` tables from it).
- **Edges that would break a downward-only rule.** A rename cannot remove them, so the plan removes
  them first (step 2):
  - `bound_coverage` imports `BoundOracleGeneration` and `GeneratedBoundOracles` from `bound`
    (`bound_coverage.rs:8-11`). `bound` generates oracles, so it belongs to the oracle subsystem;
    FR-004 (evidence) already `depends_on` FR-014 (oracle).
  - `bound` imports `PublicationDiagnostic` (`bound.rs:9`, `:94`) and the publication limits, and
    `ArtifactBundle::new` returns `PublicationDiagnostic` (`publication.rs:86`). The bundle, its
    limits and its diagnostic are one unit that every generator builds, so they belong to `core`.
  - `kani_execution` imports the harness types `KaniObligationHarness`,
    `KaniScalarObligationHarness` and `ObligationKind` from `kani_obligations`, and
    `StateFrameHarness` and `StateFrameProperty` from `state_frame` (`kani_execution.rs:45-52`);
    its tests import the same `state_frame` harness types (`:903`, `:947`). `kani_witness_join` and `spine_replay`
    import `ObligationBinding` and `KaniObligationIdentity` from `kani_obligations`. These are
    harness and identity record types, and they belong below both the generators and the runner.
- **Harness generators.** Four modules hold seven Kani source templates:
  1. `src/kani.rs` `generate_kani_bundle` (`:314`; template at `:814`), a `proof_for_contract`
     harness over V1 (`KaniRequest` takes `TypedExpression`, `kani.rs:156-176`). It emits no
     `kani::cover!`. Nothing in `src/` calls it; it is public, and `tests/it/kani_generation.rs`
     drives it.
  2. `src/kani_obligations.rs` `render_scalar` (`:2016`, template `:2057`, cover `:2082`), the
     precondition harness (`:2187`, cover `:2190`) and the contract harness (`:2285`, cover
     `:2288`). One file, three templates.
  3. `src/state_frame.rs` (`:1075` comparison harness, `:1099` assertion harness), both with a
     cover, reached by `generate_state_frame_obligations`, a second public entry beside
     `negotiate_kani_obligations`.
  4. `src/bounded_kani_corpus.rs` (`:588`), a plain `#[kani::proof]`. It has no `src/` caller.
  The cites in IR-344 (`kani.rs:314`, `kani_obligations.rs:2016`, `state_frame.rs:1075` and `:1099`)
  are correct. The first and fourth emit no cover, as IR-464 says.
- **Which families are V2.** The scalar family (`ScalarClaim`, `lower_scalar_claim`, `render_scalar`)
  and the frame family (`state_frame`, which reads `CheckedPackageV2` nodes) are V2. The
  precondition, postcondition and invariant families, the contract family, are V1 only:
  `lower_clause` takes `&BoundClause` (`kani_obligations.rs:1069`), `ObligationKind::Precondition`,
  `Postcondition` and `Invariant` come only from V1 `ClauseKind` (`:1060-1066`), and the
  precondition and contract templates render that V1 lowering. No V2 contract arm exists.
- **Input models.** `negotiate_kani_obligations` takes `ObligationItem::BoundClause` (V1,
  `BoundPackage`) and `ScalarClaim` (V2) and refuses a mix. The V1 readers are `oracle` (the
  Boolean oracle), `harness`, `bound`, `bound_strategy`, `bound_coverage`, and in `kani` the bundle
  and the `BoundClause` arm. `strategy` (the enum and `i64` campaigns) reads no `BoundPackage`.
- **Helpers.** `fn artifact(path, contents)` is defined 9 times and each is a one-line
  call to `Artifact::new` (`oracle`, `kani`, `harness`, `strategy`, `bound_strategy/generation`,
  `kani_obligations`, `exact_scalar`, `composite_equality`, `exact_function`).
  `deterministic_json` is defined twice with two error types (`oracle.rs:1111`, `kani.rs:1045`),
  both plain `serde_json::to_vec` plus a newline, not RFC 8785. Hex encoding is written twice
  (`composite_equality.rs:1293` and a `{:x}` format of QSL's `ByteDigest` in the corpus). No `sha2`
  call exists in `src/`; digests go through `qsl_replay::ByteDigest` (corpus, `spine_replay`). Two
  test helpers named `sha256_hex` are identical.
- **Raw IR walking.** Four modules walk checked-package body terms by hand over
  `serde_json::Value`: `exact_scalar` (25 `.get("..")` calls), `state_frame` (31),
  `composite_equality` (11) and `exact_function` (3), 70 call sites in all (63 lines), plus
  `.as_str()`, `.as_u64()` and `.as_object()` calls on the results. `CheckedNodeTag` is used for tag
  comparison, and `CheckedNodeKind` is used nowhere in `src/`. Operator identity is a string kept
  in three tables (`exact_scalar`, `kani_obligations`, `state_frame`).
- **Identity types.** `module_symbol` and `harness_symbol` are `pub String` on three
  identity structs (`kani_obligations.rs:478` and `:535`, `state_frame.rs:218`), and
  `generate_routed` pairs harnesses to records by that string.
- **Version profile.** The emitted oracle crate's `Cargo.toml` template, with its runtime dependency
  spelling and `RUNTIME_REVISION` (`oracle.rs:13`), is written out in three emitters
  (`exact_scalar.rs:2817`, `composite_equality.rs:1573`, `exact_function.rs:1378`). Contract
  spellings (`quire.backend-provider/v1`, `quire.capability-kind/v1`, the bundle, proof-graph,
  corpus and coverage schema ids) are constants in six modules, and the request contract
  spellings are literals in `spine_replay.rs`.
- **A tool digest existed, and is deleted.** `ReplayInputs::backend_manifest` was "the digest of the tool manifest of
  the backend that found the counterexample" (`spine_replay.rs:197-199`, plumbed at `:360-361`),
  a public field.

### Target directory tree

```text
src/
  lib.rs                      declares directories, re-exports the public API, holds no logic
  core/                       leaf: depends on nothing else in the crate
    artifact.rs               Artifact, ArtifactBundle, its size limits and the generated-source cap
                              (`MAX_GENERATED_SOURCE_BYTES`), PublicationDiagnostic with
                              its code and PublicationDestinationState (no I/O)
    source_map.rs             SourceProbe, SourceRegion
    canonical.rs              the one caller of quire-canonical's encoder and digest
    identity.rs               HarnessSymbol, ModuleSymbol, HarnessPath, ContentDigest
    ir/                       the one typed node-access layer over CheckedPackageV2
    profile.rs                the version profile: every spelling this build emits or requires
    diagnostic.rs             GenerationErrorCode, GenerationDiagnostic, GenerationTerminalState
    naming.rs                 bounded readable components, unique names, symbol derivation
  oracle/                     FR-014, FR-018, FR-021, FR-031 (boolean_v1.rs)
    claim.rs                  ClaimMap, ClaimDisposition, OracleGenerationError (was generation)
    scalar/                   was exact_scalar; step 2d moves it whole, step 3 splits it
      mod.rs                  public scalar API and generation orchestration
      derive.rs               descriptor derivation from admitted nodes
      lower.rs                checked-node access, bounds and operation validation
      render.rs               generated oracle source and artifact construction
    equality/                 was composite_equality
    function/                 was exact_function
    boolean_v1.rs             V1 Boolean oracle; deleted with V1 (see One input model)
    bound_v1.rs               V1 bound oracle generation (was bound); deleted with V1
  strategy/                   FR-002, FR-008 to FR-013
    harness.rs                tri-state harness (was harness); V1 input
    campaign.rs               enum and i64 campaigns (was strategy); no V1 input
    bound/                    was bound_strategy; V1 input
  evidence/                   FR-004
    vacuity.rs                LLVM coverage parse and clause classification
    bound_coverage.rs         V1 input until rebased
  kani/                       FR-015, FR-017, FR-025, FR-028 to FR-030
    abi.rs                    binding roles, primitive types, integer bounds, KaniSolver, options
    census.rs                 the proof-dependency census types (the FR-015 census input)
    identity.rs               the harness and identity record types, ObligationKind, ObligationBinding
    generate/                 the one generator
      spec.rs                 HarnessSpec (step 4b; not created by 2f)
      render.rs               the one renderer: the only code that emits Kani attributes and macros
                              (step 4b; not created by 2f)
      negotiate.rs            negotiate_kani_obligations, the one public generation entry, with the
                              negotiation passes over a request's items (classify, name settlement,
                              assumption resolution) and the clause renderer's dispatch
      outcome.rs              the request and result vocabulary the entry and every family speak
                              (ObligationItem, UnsupportedObligation, ObligationRecord, ...); a leaf
                              inside generate/: it imports no family
      scalar.rs  precondition.rs  contract.rs  frame.rs   family lowerers; each returns a HarnessSpec
                              once its family is ported (scalar at 4b, precondition and contract at
                              4c, frame at 4d). At step 2f each holds what its family lowers and
                              renders today, moved verbatim
      clause.rs               the V1 clause lowering that precondition.rs and contract.rs share
                              (ClauseOracle, the subject ABI, slots); interim, reshaped by step 4c
      record.rs               the validated harness path and the persisted per-harness record every
                              family's render step builds last; holds today's items only, and later
                              steps may move them
      v1_bundle.rs            INTERIM: `generate_kani_bundle` and what only it uses; deleted whole by
                              step 4f
      census_validation.rs    INTERIM: `validate_dependencies`, the V1 bundle's error type it returns,
                              and `deterministic_json`; the corpus calls them. Survives 4f; deleted
                              with the corpus lowerer at 4g or by the V2 census input, and
                              `deterministic_json` by step 1a
      lower/                  the Kani family lowerings CG takes over from IR, and the profile admission
                              (at 2f: bounded_kani_profile.rs, bounded_collections.rs,
                              definedness_arithmetic.rs, finite_reference_graphs.rs, old names kept)
      corpus/                 corpus family (at 2f: bounded_kani_corpus.rs, old name kept, unsplit);
                              renders through render.rs from step 4g
    output/                   the one reader of Kani output
      report.rs               typed report parse (--export-json)
      playback.rs             the concrete-playback block: the console-text extraction
                              (`counterexample_playback`) and, from step 2f, the block scan
                              `kani_witness_join` held, with `DecodeFailure`; typed entries from step 5
    classify.rs               KaniRunOutcome, KaniInconclusiveReason, vacuity rule
    run/                      launch, capture, timeout, execute_kani_obligation (at 2f: tool.rs,
                              harness.rs, launch.rs, report_file.rs, execute.rs)
    test_support.rs           `#[cfg(test)]` only: the test helpers that tests of more than one
                              kani/ file share (step 2f)
    terminal.rs               the terminal-value maps (FR-029, FR-030) and the public C-09 entry the
                              driver calls, with the replay-outcome input type it defines
                              (step 5; not created by 2f)
  replay/                     FR-016, FR-024
    witness.rs                types extracted playback entries against persisted bindings
    function.rs               was spine_replay
    frame.rs                  was frame_replay
    state_clause.rs           StateClauseReplay: the postcondition state-clause replay (FR-024-AC-11 to AC-19, IR-460)
  routed/                     FR-019, FR-022, FR-026
    capability.rs             was capability
    generate.rs               was routed_generation
    adapter.rs                RESERVED, not created by any step of Migration order: the adapter trait
                              and its one Kani implementation (the adapter contract of ADR-002; no
                              code of that shape exists today)
  publication/                FR-005
    publish.rs                write_bundle_atomic, published identity
```

`kani/terminal.rs` is created by step 5 (the terminal map); the layout reserves its place and step
2f does not create it, nor `generate/spec.rs` or `generate/render.rs` (step 4b). `routed/adapter.rs`
is reserved the same way but no step creates it: nothing in this crate is the adapter trait today,
so step 2g does not create it and the layout test (L-1) does not require it. Directory names equal the registry's subsystem names, which makes ADR-0056
rule 3 (one module maps to exactly one subsystem) true by construction.

### Module-to-subsystem map

Every module at this base, with its target. "Retire" means deleted by the step named under
Migration order, not moved.

| Module today | Subsystem | Target | Fate |
| --- | --- | --- | --- |
| `lib` | core | `lib.rs` | stays; re-exports by explicit list, no logic. The one `pub mod bound_strategy` path leaves (step 2e); callers use the re-exported names |
| `oracle` (shared parts: `Artifact`, diagnostics, `RUNTIME_REVISION`) | core | `core/artifact.rs`, `core/diagnostic.rs`, `core/profile.rs` | split; `RUNTIME_REVISION` deleted |
| `oracle` (naming helpers: `bounded_readable_component`, `readable_name_component`, `upper_camel`, `unique_names`, `unique_pair`, `oracle_symbol`, `reference_identifier`, `observation_name`, and the private `rust_component`; with their tests) | core | `core/naming.rs` | split (step 2d-0). `observation_name` becomes `pub(crate)`, because `reference_key` (V1 identity key, with `dependency_key`) stays in `oracle/boolean_v1.rs` and calls it; those two are expression bookkeeping, not naming. `dependency_parameters` and `typed_dependency_parameters` are not naming either: they run the V1 expression analysis and stay in `oracle/boolean_v1.rs` at step 2d. Because the V2 `kani_obligations` and `kani` import `typed_dependency_parameters` (and `generate_boolean_oracle`), that dependency predates this AD and step 6 must move or replace it before it deletes `boolean_v1.rs` |
| `oracle` (`MAX_GENERATED_SOURCE_BYTES`) | core | `core/artifact.rs` | split (step 2d-0): the one cap on a generated source, kept beside the bundle's size limits. Every user (the oracle, strategy, harness and Kani generators) sits above `core`, so rule 4 holds |
| `oracle` (`SourceProbe`, `SourceRegion`) | core | `core/source_map.rs` | split |
| `oracle` (V1: `generate_boolean_oracle`, `analyze_node`, `render_node`) | oracle | `oracle/boolean_v1.rs` | moved, then retired with V1 |
| `publication` (`ArtifactBundle`, limits, `PublicationDiagnostic`, `PublicationErrorCode`, `PublicationDestinationState`) | core | `core/artifact.rs` | split (step 2a); the destination state is a field of the diagnostic, so it moves with it |
| `publication` (writer, published identity) | publication | `publication/publish.rs` | split |
| `generation` | oracle | `oracle/claim.rs` | moved |
| `exact_scalar` | oracle | `oracle/scalar/{mod,derive,lower,render}.rs`; shared walkers to `core/ir/` | moved whole to `oracle/scalar/mod.rs` at step 2d; split with typed node access at step 3 |
| `composite_equality` | oracle | `oracle/equality/` | moved |
| `exact_function` | oracle | `oracle/function/` | moved |
| `bound` | oracle | `oracle/bound_v1.rs` | moved; V1 input, so `evidence` and `strategy` import it downward |
| `harness` | strategy | `strategy/harness.rs` | moved; V1 input |
| `strategy` | strategy | `strategy/campaign.rs` | moved; no V1 input |
| `bound_strategy/*` (5 files) | strategy | `strategy/bound/` | moved; V1 input |
| `vacuity` | evidence | `evidence/vacuity.rs` | moved |
| `bound_coverage` | evidence | `evidence/bound_coverage.rs` | moved; V1 input |
| `kani` (`KaniBindingRole`, `KaniPrimitiveType`, `KaniIntegerBounds`, `KaniSolver`, `adapter_options`, `i64_literal`, `readable_component`) | kani | `kani/abi.rs` | split (step 2f). `KaniSubjectBinding` is not an `abi` item: only the V1 bundle uses it, so it goes with the bundle below |
| `kani` (`ProofDependencyEdge`, `Kind`, `State`, `Request`, `ProofReadiness`, `normalize_dependencies`, `dependency_readiness`) | kani | `kani/census.rs` | split (step 2b); the FR-015 census input and the corpus use them |
| `kani` (`generate_kani_bundle`, `KaniArtifactBundle`, `ProofDependencyGraph`, `KaniRequest`, `KaniSubjectBinding`, bundle validation and rendering, except `validate_dependencies` and what it returns) | kani | `kani/generate/v1_bundle.rs` until step 4f; then none | retired (step 4f). Step 2f moves them verbatim to the one interim file `v1_bundle.rs`, which 4f deletes whole. Still live: `tests/it/kani_generation.rs` drives the bundle, and it is the path QSL's control runs through until the V2 contract arm (4c) |
| `kani` (`validate_dependencies`, `KaniDiagnostic`, `KaniErrorCode`, the identity and path helpers it uses) | kani | `kani/generate/census_validation.rs` | interim home (step 2f); stays until the corpus stops calling it (step 4g) or the V2 census input replaces it |
| `kani` (`deterministic_json`, `artifact`) | core | `core/canonical.rs` | `deterministic_json` deleted and `Artifact::new` used directly by step 1a, which is held. Until it lands, `deterministic_json` stays in `kani/generate/census_validation.rs` (the file that survives 4f, because the corpus calls it) and the private `artifact` wrapper stays beside its only caller in `v1_bundle.rs` |
| `kani_obligations` (harness and identity records, `ObligationKind`, `ObligationBinding`) | kani | `kani/identity.rs` | split (step 2b) |
| `kani_obligations` (the rest) | kani | `kani/generate/{negotiate,outcome,record,clause,scalar,precondition,contract}.rs` | split (step 2f), item by item as the step 2f item map says; it adds `outcome.rs`, `record.rs` and `clause.rs` to the four files this row first named, because the items the families share need a home below all of them |
| `state_frame` | kani | `kani/generate/frame.rs`; `StateFrameHarness`, `StateFrameProperty` to `kani/identity.rs` | moved whole at step 2f, unsplit; one entry with `negotiate` at step 4d |
| `bounded_kani_corpus` | kani | `kani/generate/corpus/bounded_kani_corpus.rs` | moved whole at step 2f, old file name kept, no split; its package lowerer retired after QSL-353 (step 4g) |
| `bounded_kani_profile`, `bounded_collections`, `definedness_arithmetic`, `finite_reference_graphs` | kani | `kani/generate/lower/{bounded_kani_profile,bounded_collections,definedness_arithmetic,finite_reference_graphs}.rs` | moved whole at step 2f, old file names kept; CG owns the three semantic-family lowerings and their request/result types. Its corpus consumes them directly and retains IR-owned profile, dispatch, validated finite input and typed outcome interfaces. IR-347 removes the former IR exports after the CG consumer move |
| `kani_execution` | kani | `kani/run/{tool,harness,launch,report_file,execute}.rs`, `kani/classify.rs` | split (step 2f): classification to `classify.rs`, everything that launches or reads a file to `run/` |
| `kani_transcript` | kani | `kani/output/report.rs`, `kani/output/playback.rs` | split (step 2f): `counterexample_playback` and its four `PLAYBACK_*` constants to `playback.rs`, the typed report parse to `report.rs`. PR 210 (merged) put the report parse here |
| `kani_witness_join` | replay | `kani/output/playback.rs` (the block scan and `DecodeFailure`, step 2f); `replay/witness.rs` (the decode, step 2g) | split across two steps; the item map says which item goes where |
| `spine_replay` | replay | `replay/function.rs` | moved; `backend_manifest` deleted (IR-465) |
| `frame_replay` | replay | `replay/frame.rs` | moved |
| (new, IR-460) | replay | `replay/state_clause.rs` | the state-clause replay; no earlier file |
| `capability` | routed | `routed/capability.rs` | moved |
| `routed_generation` | routed | `routed/generate.rs` | moved |

The registry rows change to match in the spec PR of step 7: Core owns `lib` and `core`, Oracle owns
`oracle`, and so on, one directory per row.

### Dependency direction

```text
publication --> core
routed      --> replay, kani, strategy, oracle, core
replay      --> kani, core
kani        --> oracle, core
strategy    --> oracle, core
evidence    --> oracle, core
oracle      --> core
core        --> (nothing in this crate)
lib         --> everything (re-exports only)
```

Every arrow has a basis in code or spec. `evidence --> oracle` is FR-004's `depends_on` FR-014 and
the `bound_coverage` edge. `strategy --> oracle` is `bound_strategy` and `harness` calling oracle
generation. `kani --> oracle` is the scalar family embedding the FR-014 oracle. `replay --> kani`
is the identity and binding types the witness decode and replay read. `routed --> ...` is the
`BackendKind` match calling each generator.

Rules, each checkable:

- A directory imports only the directories to its right. `strategy`, `evidence` and `kani` are peers
  and import none of each other. `kani` and `replay` import nothing from `routed`, and the adapter
  trait, when it exists, is defined in the reserved `routed/adapter.rs` with the Kani
  implementation beside it. The driver
  (`quire-driver`, outside this crate) pairs a Kani outcome with a replay result, as QSL's merged
  ADR-011 T-13 says; no module of this crate does.
- Imports use a module path (`use crate::core::artifact::Artifact`), never an item re-exported
  from the crate root. `lib.rs` is the only file that names a root re-export.
- Inside `kani/` the order is `abi`, `census`, `identity`, then `generate` and `output`, then
  `classify`, `run`, `terminal`. A file imports only earlier names in that order. `output` imports
  `identity` and `abi` and nothing from `generate`, `classify` or `run`. `run` imports `identity`,
  `output` and `classify` (and `abi`, for `KaniSolver`), not `generate`: the harness and identity
  record types it needs live in `kani/identity.rs` (step 2b), so a harness is run from its identity
  and its source text.
- Inside `kani/generate/` the order is `outcome`, `record`, `census_validation`, then the families
  (`scalar`, `clause`, then `precondition` and `contract`, `frame`, `lower`, `corpus`, `v1_bundle`),
  then `negotiate`, which imports `outcome`, `record`, `scalar`, `clause`, `precondition`, `contract`
  and `frame` (the `StateFrame` arm of step 4d calls the frame family's role function; none of
  `lower`, `corpus`, `v1_bundle` or `census_validation`) and is imported by none. A family file may import `outcome` and `record` and never `negotiate`: the passes that read the whole request
  (classification, name settlement, assumption resolution) are `negotiate`'s, and what lowers or
  renders one item is its family's. `corpus` imports `lower` and `census_validation`; `v1_bundle`
  imports `census_validation`; `frame` imports `outcome` only. `kani/test_support.rs` is
  `#[cfg(test)]`, imports `identity` and `abi` and no generator, and may be imported by any test
  module under `kani/`.
- A `#[cfg(test)]` module obeys the same direction as the file it sits in. A test that needs a
  generator as a fixture lives in `tests/it/`, not in the runner's file. The back-edge in the pair
  `kani_execution` and `kani_transcript` (the `kani_transcript` tests that import
  `classify_kani_run`, `KaniInconclusiveReason` and `KaniRunOutcome`) is not moved to `tests/it/`:
  it needs no generator, it is a classification test, and classification is a later file than the
  report parse, so those tests move to `kani/classify.rs`, which may import `output`. The tests
  that need only the report parse stay with `output/report.rs`. The step 2f item map decides each
  test by name.
- One interim exception to the typed-values target, recorded as the code has it. `classify_kani_run`
  takes the console text beside the typed report, and `classify_report` calls
  `counterexample_playback` on it for a falsifying check, so `kani/classify.rs` imports
  `kani/output/playback.rs`. The Kani run table below records the target and today's reading side
  by side. The edge points in the allowed direction (`classify` is after `output`), so no order rule
  is broken; the exception is to L-6 (the classifier takes typed values) and is listed there. It
  holds before this AD, and step 2f keeps it as it is. Step 5 removes it by changing the classifier's
  input from the text to the playback `output/playback.rs` extracts; that signature change is public
  (`classify_kani_run` is `pub` so tests can reach the production classifier) and is step 5's to
  design.
- `serde_json::Value` is named only in `core/ir`, `core/canonical` and the report parser. No other
  file reads a field of a body term.
- `BoundPackage` and `BoundClause` are named only in `strategy/`, `evidence/`, `oracle/boolean_v1.rs`,
  `oracle/bound_v1.rs` and the V1 arm that step 4f deletes. After step 4f none is named in `kani/`,
  `routed/` or `replay/`.

### The one Kani generator

FR-015 is the one generator (ADR-001 Q1, AD-001). In code that means one renderer, one spec type,
one identity record, one cover rule and one entry, not one function:

- `kani/generate/spec.rs` defines `HarnessSpec`: the proof attribute (`proof` or
  `proof_for_contract`), `HarnessPath`, the argument bindings with their bounds, the assumptions,
  the subject call, the assertions and the covers, in that order.
- **Cover rule (IR-464).** A non-empty cover list is not a non-vacuity check. The rule is where the
  covers go: `render.rs` emits every cover after all assumptions, after the subject call and after
  every assertion, as the last statement of the body, never before, so a cover is reachable only
  when the assumptions are satisfiable and no failing assertion shares its valuation, and the cover states
  the property's own reachability (for the scalar family, that the oracle's `Completed` branch is
  reached; for the precondition family, that the precondition holds). The constructor refuses an
  empty cover list, and the order is fixed by the renderer, not by the family. A frame harness's
  `kani::cover!(true, ...)` satisfies the rule once it follows the subject call and the frame
  assertions, as FR-015-AC-7 states. Test (L-4): a harness whose assumptions are unsatisfiable does not classify
  `Verified` under real Kani. The corpus has no symbolic input; FR-015-AC-55 gives its harness a cover after its assertion (IR-464), and FR-015-AC-58's inspection of emitted text guards every emitter, and stays beside this constructor once a family renders through it; see step 4g.
- `kani/generate/render.rs` is the only code that emits `#[kani::proof]`, `#[kani::proof_for_contract]`,
  `kani::requires`, `kani::ensures`, `kani::any`, `kani::assume` and `kani::cover!`. A layout test
  checks string literals in non-test source (not comments, so doc prose that names them passes).
- Four family lowerers (`scalar`, `precondition`, `contract`, `frame`) each take lowered IR claims
  and return `HarnessSpec`s. They do not format Rust source. `generate_state_frame_obligations`
  becomes a frame arm of `ObligationItem`, so `negotiate_kani_obligations` is the one public
  generation entry; ADR-004 owns what the frame arm asserts.
- `ScalarObligationIdentity`, `KaniObligationIdentity` and `StateFrameIdentity` collapse into one
  `kani/identity.rs` record with the family's own members as a typed variant, so the argument
  vector, ceilings, symbols and bounds are spelled once. The obligation digest of AD-003
  is computed from that record by one function.

**The contract families need a V2 input first.** The precondition, postcondition and invariant
families exist only over V1 (see Current state). Deleting the V1 arm without a V2 replacement would
delete the families that step 4e makes the public entry. So the V2 input is a step of its own,
before 4e (step 4c). Its design intent: a clause claim names a precondition, postcondition or
invariant clause node of an admitted `CheckedPackageV2`, and the contract lowering reads that node
and embeds the FR-014 oracle of its Boolean connectives, bounded-integer comparisons and the
integer arithmetic expressions in the clause body (the control's shape is a population-rule
postcondition, `amount < 1000` implies `amount + 1 <= 1000`, so arithmetic in a clause is in
scope, or the control cannot move onto this arm), as the scalar family embeds the oracle of its
claim and as `state_frame` already reads postcondition clause nodes from V2. The exact input type is the spec change's to define. That change is an
FR-015 amendment in this repository's spec lane. IR-364 does not carry it: IR-364 covers the V2
strategy chain (FR-002, FR-004, FR-005, FR-008 to FR-013), not FR-015. I found no ticket for the
FR-015 V2 contract input; the planner should file one.

What happens to the other generators:

| Generator | Fate |
| --- | --- |
| `generate_kani_bundle` | Deleted at step 4f, after step 4e, the QSL-owned move of the QI exemplars and the control passing on the V2 contract arm (4c). Its public entry leaves `interface-001`. FR-003's optional stubbing was dropped by the IR-311 ruling. |
| `kani_obligations` scalar, precondition, contract renderers | Become family lowerers; the template text moves into `render.rs`. Until 4c the precondition and contract families keep their V1 input. |
| `state_frame` renderers | Become the frame lowerer. |
| `bounded_kani_corpus` renderer | Keeps its own template until QSL-353 lands (an interim exception to L-3; its cover is FR-015-AC-55, IR-464); then its hand-built package lowerer is retired and its cases render through `render.rs` (step 4g). It returns no `KaniOutcome` at generation time: a verdict comes only from a run. |

**Requirement (regression test the one generator must keep passing).** QSL's real-Kani arithmetic
control, recorded as QSL-342 QI #10, passes at every step of the migration, on the path that
serves it at that step. A lowered `+` mutated to return `left + right + 1` must classify
`Falsified`, and `decode_falsification` must name `amount_current = 999`. The unmutated control
must prove with every cover satisfied. The control already exists in quire-integration, the
repository above both CG and QSL (`tests/qsl_kani_exemplar.rs`, 4 of 4 exemplar tests; relayed,
not checked here). That test is the regression requirement: it runs in a repository above both, so
it needs no CG-side copy and no copied fixture, and each step's PR runs it against CG's branch. The
planner reports that it currently passes through `generate_kani_bundle`, a V1 path. Until the V2
contract arm exists (4c), that path stays and is the one the control runs through. The QSL-owned
follow-up after 4e moves the exemplars onto the one public entry, so the control then runs on the
V2 contract arm and must pass there; only then may 4f delete the V1 path. A CG-side copy of the control is described only if the `qsl-replay` facade offers a way to
build the package: CG may depend on `qsl-replay` only (`Cargo.toml`, QSL arch-lint T12-A), and the
V1 control's package is built by `quire_spec_language::lowering::lower_for(.., IntegerIrV1)` in
QSL's root crate (as the SR-662 review found; not checked here), which CG cannot call. If the
facade offers no way, a CG-side control is a need routed to QSL, not a fixture. This AD did not
run the control.

### The Kani run, report and witness (questions c and d)

Where the report file lives. The runner already owns the run's directory: it sets
`CARGO_TARGET_DIR` from `KaniExecutionRequest::target_directory` and runs in
`crate_directory`. The report is run evidence, not a generated artifact, so it is never part of an
`ArtifactBundle` and is never published. Decision: `kani/run` passes
`--export-json <target-dir>/quire-kani-report-<pid>-<seq>.json` (a name unique to the launch, as
PR 210 implements and FR-017 states), removes only that file before launch and after reading it
(a report left by another run must never read as this run's verdict, and runs sharing a target
directory never touch each other's file), reads it once after
the process exits, and hands the bytes to `kani/output/report.rs`. The evidence record carries the
parsed typed result, never the path. A report that is absent after a clean exit, unreadable, over
the size cap or of an unknown schema is a typed refusal and never `Inconclusive`, as PR 210
specifies. With harness batching the file stays one per Kani process in that process's
`target_directory`; the parser returns one typed result per harness keyed by `HarnessPath`, and
the runner refuses a batch whose report lacks or duplicates a requested harness. PR 210's
"exactly one harness result" check becomes "exactly one result per requested harness".

Where the runner, report parser and witness decode sit. One module reads Kani's output.

| Concern | Home | Reads |
| --- | --- | --- |
| Launch, capture cap, timeout, process-group kill, harness-in-crate check | `kani/run/` | process only |
| Report parse | `kani/output/report.rs` | the JSON file, typed |
| Playback extraction | `kani/output/playback.rs` | the printed block, as a payload; Kani's report carries no concrete playback |
| Run classification and vacuity | `kani/classify.rs` | the typed report; TARGET: nothing else. Today also the run's console text (`classify_kani_run`'s `text` parameter), which it hands to `output/playback.rs`'s `counterexample_playback` for a falsifying check; an interim exception removed by step 5 (see Dependency direction and L-6) |
| Witness decode | `replay/witness.rs` | TARGET: typed playback entries only. Until step 5 it reads the block through `output/playback.rs`'s scan and compares Kani's decoded-value comment itself (`boolean_comment`) |
| FR-029 map: `KaniRunOutcome` to a terminal value | `kani/terminal.rs` | typed run outcome only |
| C-09 map: IR's `KaniOutcome` with the replay outcome to `TerminalValue` | `kani/terminal.rs`, a public entry | typed outcome and the replay-outcome type it defines; total over the pair (step 5) |
| Pairing the two inputs | the driver, outside this crate | runs the replay and calls the C-09 entry with both |
| Building the replay request; converting CG errors and QSL's result into the replay outcome | `replay/` | builds the request the driver passes to `qsl_replay::replay`, and converts results into `kani/terminal.rs`'s input type |

Who calls `qsl_replay::replay`. QSL's merged ADR-011 E9 and T-13 say the driver calls it with the
request CG's adapter builds. Today CG's `replay/` modules (`spine_replay`, `frame_replay`) build the
request and also call `replay` themselves (AD-001's Replay view). That is a deviation from T-13,
recorded here: the layout keeps `replay/` calling `replay` until the driver takes the call, at which
point `replay/` only builds the request and converts results. This AD does not schedule that move.

Today `kani_witness_join` also scans Kani's text (`check_clause`, `select_assertion_block`,
`read_block`, `concrete_entries`, with `unescaped_quote` and `bracketed`), which breaks AD-001's
"Kani's printed wording is read in exactly one module". Two steps fix it, and the first is a
verbatim move. Step 2f moves the scan items, the two marker constants, the `Playback` and
`CheckKind` types and `DecodeFailure` (the error the scan raises, so `kani` need not import
`replay`) to `kani/output/playback.rs`, unchanged but for visibility; the decode half stays in the
flat `kani_witness_join.rs`. Step 2g moves that half to `replay/witness.rs`. Step 5 then changes
the interface: `playback.rs` returns typed `(name, rendered value)` entries and its own typed
refusal, and `replay/witness.rs` receives them, checks them against the persisted argument
bindings and builds `qsl_replay::WitnessValue`. Replay imports `kani`; `kani` never imports
`replay`. That gives one report parse and one playback parse. The removal of IR's witness-accessor re-parse (IR-277) is
on IR's side: CG imports no IR witness type already (AD-001), so nothing in CG depends on those
accessors. Batching (IR-277) touches `run/` only, plus the keyed parse above.

### One input model

The IR-311 ruling is: `CheckedPackageV2` is the one input model, the V1 `BoundPackage` is retired
entirely, no side-by-side path and no adapter, and each V1 path is replaced and deleted in the same
change. This AD follows it and keeps no V1 boundary as a design goal. IR deletes its own
`BoundPackage` after CG stops naming it (IR's layout AD, pending; relayed, not read here). The AD
records only the work order the ruling allows, because the code still has V1 readers:

- Kani side: the V1 arm and the bundle are deleted at step 4f, only after step 4c has given the
  contract families a V2 input. V1 has no consumer once that exists; before it, the V1 arm is the
  contract family.
- Oracle, strategy and evidence side: `harness`, `strategy/bound`, `evidence/bound_coverage`,
  `oracle/bound_v1.rs` and `oracle/boolean_v1.rs` read V1 and have no V2 criteria yet (FR-002,
  FR-004 and FR-008 to FR-013 are written over `BoundPackage`). They sit in their final
  directories, named V1 in the module map, and each is deleted in the PR that adds its V2
  replacement. A layout test fails if a V1 type is named anywhere else. This is a rule about where
  the leftover may be, not a second supported path. `strategy/campaign.rs` reads no V1 type.

### Shared core: helpers, identities, node access, profile

- **One canonical module** `core/canonical.rs` is the one place CG calls `quire-canonical` (its own
  repository, `agent-ix/quire-canonical`; RFC 8785): `to_vec(value, Limits)` and `sha256`. CG
  depends on that crate directly, spelled `branch = "main"`, with no `qsl-replay` re-export (QSL's
  ruling, relayed by the IR planner; not verified here). AD-003 requires the
  same: the obligation preimage is encoded by that encoder and never by `serde_json`.
  Every JSON CG emits, artifact files and digest preimages alike, goes through it. Both
  `serde_json`-based `deterministic_json` copies are deleted, not kept; this changes the bytes of
  emitted JSON artifacts once, and regeneration stays byte-identical (NFR-001). The nine `artifact`
  wrappers are deleted and callers use `Artifact::new`. The corpus case digest moves to this module
  too. CG writes no hash or encoding routine of its own. Coupling: CG's own `Cargo.lock` already
  resolves `quire-canonical` from QSL's tag `quire-canonical-v0.3.0`, through `qsl-replay`
  (`Cargo.lock:1345-1347`). A direct `branch = "main"` dependency adds a second entry, and CG's own
  `make deny` one-copy check (`scripts/check_one_copy.awk`) fails on it. The same holds for any lock
  that pulls QSL together with CG or IR on `main` (quire-integration, the driver), the second
  consumers. See the step 1a precondition.
- **One digest identity.** The only digest CG mints for a proof is the obligation digest that
  binds a proof to its content (AD-003, E-1). `ContentDigest` in `core/identity.rs` is a CG type over
  `quire-canonical`'s digest; it is built only by `core::canonical`, over the RFC 8785 preimage.
  It does not wrap `qsl-replay`'s `ByteDigest`, except at a QSL API surface that requires one. It
  is not a second digest beside QSL's `ObligationIdentity`: it is CG's typed holder of the value
  that identity is built from. Every other digest on the chain (`package_id`, byte digests of provided
  source) is QSL's or IR's and is carried, not recomputed. No tool, version or file digest is
  added, and the one that exists, `ReplayInputs::backend_manifest`, is deleted at step 5 together
  with the tool pin QSL-351 removes, with the manifest members it feeds in `spine_replay`.
- **Identity newtypes.** `ModuleSymbol` and `HarnessSymbol` are validated Rust identifiers, built
  once where a harness is generated. `HarnessPath` is the pair. `KaniSolver` replaces the `solver:
  String` field. `generate_routed` pairs a record with its harness by the harness symbol, which is all
  the record's `Supported` disposition names, and two harnesses with one symbol are a typed error
  (`DuplicateHarness`, carrying the second one's `HarnessPath`), not a silent `collect` overwrite. Bounds travel as typed integers and are not re-parsed from decimal
  text.
- **Model items come from `quire-contract-model` directly.** IR's merged AD-006 (the codegen
  consumption seam; checked at IR `origin/main`) has codegen declare `quire-contract-model` for
  model items and the IR root crate only for the rest, because the root crate re-exports model
  items through a `pub use quire_contract_model::*` glob that IR removes (AD-006's R3-C2: codegen
  first adds the direct dependency, then IR removes the glob). In this layout that touches
  `core/ir`, `oracle/` and `kani/generate/`, the modules that name model types, and it is step 1d.
  CG's lock then holds one `quire-contract-model` and one `quire-contract-ir` (`make deny`).
- **One typed node-access layer** `core/ir/`: the only code in CG that opens a `CheckedPackageV2` body
  term. It exposes a node view with a typed tag, typed member accessors and typed errors, and
  carries the operator table (identifier, arity, reachable range) as one enum in place of the three
  string tables. The four walkers (70 `.get` call sites at this base) migrate onto it, one module per
  PR, and the count outside `core/ir` goes to zero. If IR exposes a typed decoder, the callers
  switch to it and `core/ir` is deleted in the same change; no forwarding layer is kept.
- **The version profile in one place** `core/profile.rs`: the emitted oracle crate's manifest
  template (written once, called by the three emitters), whose runtime dependency names the
  runtime source the way CG's own `Cargo.toml` does (git URL and branch), and every contract and
  schema spelling listed under Current state. `RUNTIME_REVISION` is deleted (see Decisions taken
  from the planner). A spelling QSL will export (AD-002's R-Q5) leaves this file
  when it does. The profile holds spellings this build emits or requires; it records no tool
  version and asserts no digest over a file.

### The two recorded exceptions

**FR-005 (CLI conformance; TC-001, TC-002, TC-007).** FR-005 is in `core` but depends on FR-002
(strategy), FR-015 (kani) and FR-004 (evidence), against ADR-0056 rule 4. Decision (the IR
planner, on IR-344): a `publication` subsystem, `spec/publication/`, owning FR-005 and its three
test cases, and the code directory `publication/`. There is no FR-005 exception. The move is a
follow-up PR (migration step 7), and the registry note in `spec/spec.md` that records the
exception becomes obsolete when it lands.

| | Publication subsystem (decided) | Accepted core exceptions (not taken) |
| --- | --- | --- |
| Benefit | Rule 4 holds everywhere. `core` shrinks to shared primitives, which is what rule 4 says it is. The requirement sits in the top layer where it depends on the lower ones, the same direction the code takes. The code needs no upward import: the bundle, its limits and its diagnostic move to `core/artifact.rs` (step 2a), and only the atomic writer is `publication/`. | No spec movement. |
| Cost | One registry row, one directory, one matrix index row; `git mv` of FR-005 and TC-001, TC-002, TC-007 with ids unchanged (ADR-0056 identifier rules); the `Owning crates/modules` column changes for `publication`. FR-005-AC-5 describes bundle limits enforced in `core/artifact.rs` but lives in `publication/`, a split the criterion's verification (TC-002) already spans. | The exception stays for as long as the requirement spans subsystems, and "core depends on nothing" stops being checkable. The next cross-cutting requirement has a precedent. |

Splitting FR-005 into a core publication requirement and an integration requirement was not
taken, because it mints ids.

**`core/matrix/suites.md` (SUR-001, a SuiteRegistry).** ADR-0056 has no matrix slot for it.
Decision (the IR planner, on IR-344): `spec/core/functional/suites.md`, by analogy with ADR-0056
rule 5, which puts "a registry every subsystem consumes" in `core/functional/`. SUR-001 lists the
commands that validate and run every subsystem (spec validation, coverage export, MSRV build, the
Kani lane), so every subsystem consumes it, and it depends on no subsystem's requirement.
ADR-0056 is not amended. The move is part of migration step 7.

## Decisions

Statements a test can check. Local labels; the repository assigns requirement ids when a
requirement is authored.

- L-1. `src/` has the directories `core`, `oracle`, `strategy`, `evidence`, `kani`, `replay`,
  `routed` and `publication`, plus `lib.rs`, and every module in the map above lives in the one the
  map names. Test: a layout test lists `src/` and compares with a directory list held in the test;
  once step 7 lands the registry rows by directory, it reads the registry instead. The scalar
  file split is an additional step 3 exit check: `oracle/scalar/` declares `derive`, `lower` and
  `render` from `mod.rs`, and the scalar-specific logic resides in those files as the target tree
  assigns. The step 2 layout test does not require those files before step 3.
- L-2. The import graph is acyclic and follows the dependency direction above, `#[cfg(test)]`
  modules included, and no file imports an item through the crate root. Test: a layout test that
  reads `use crate::` lines and flags a bare `crate::<Item>` path in code (inline types, calls;
  not string literals or comments, so intra-doc links are rewritten by step 2g-0 but not checked). It lands at the end of step 2, after the edge-removal steps, so it can
  pass when it lands.
- L-3. `#[kani::proof]`, `#[kani::proof_for_contract]`, `kani::requires`, `kani::ensures`,
  `kani::any`, `kani::assume` and `kani::cover!` occur in string literals of non-test source in
  `kani/generate/render.rs` only. Test: a literal scan that ignores comments. It lands after 4f and
  4g. Until then the interim exceptions are the V1 arm and `generate_kani_bundle` (their own
  templates until 4f, in `kani/generate/v1_bundle.rs`) and the corpus template (until 4g, in
  `kani/generate/corpus/bounded_kani_corpus.rs`); the scan lists them by file and each entry is
  removed with its step.
- L-4. A `HarnessSpec` has at least one cover, and `render.rs` places every cover after all
  assumptions, the subject call and every assertion, as the last statement. Test: the constructor's refusal, and a real-Kani test that a
  harness with an unsatisfiable assumption does not classify `Verified`.
- L-5. QSL's arithmetic control passes at every migration step, on the path that serves it: the
  `left + right + 1` mutant is `Falsified` with `amount_current = 999`, and the unmutated control
  proves with every cover satisfied. Test: quire-integration's exemplar tests (4 of 4), real Kani,
  run against CG's branch by each step's PR as a local run with a `[patch]` of CG into
  quire-integration, with the transcript in the PR (as step 2 requires for PR 210); a CG-side copy
  only if the facade allows. Until the QSL-owned follow-up after 4e moves the exemplars, the path is
  the V1 bundle; after it, the V2 contract arm.
- L-6. Kani output is read in `kani/output/` only, and the classifier, the terminal maps and the
  witness decode take typed values. Test: a grep for Kani's banner, check and playback wording
  outside that directory, over non-test source: the test modules that embed real Kani captures as
  fixtures (`kani/classify.rs`, the decode tests) are not scanned. Interim exceptions, listed by
  file and each removed with its step: `kani/classify.rs` takes the console text and calls
  `counterexample_playback` (removed by step 5), and the flat `kani_witness_join.rs`, then
  `replay/witness.rs`, still compares Kani's decoded-value comment in `decode_values` and
  `boolean_comment` (removed by step 5, when the typed entries carry the rendered value).
- L-7. The report path is `<target-dir>/quire-kani-report-<pid>-<seq>.json`, unique to the launch,
  removed only by its own run before launch and after reading, and absent from every
  `ArtifactBundle`. Test: a stand-in launcher that leaves another run's report, and concurrent
  runs in one target directory.
- L-8. `serde_json::Value` and `.get("` appear for body terms only in `core/ir`. Test: a grep
  gate.
- L-9. The canonical encoder and the content digest are called from `core/canonical.rs` only, and
  `deterministic_json` and `fn artifact(` have no definition. Test: a grep gate.
- L-10. `module_symbol` and `harness_symbol` have no `String` field outside `core/identity.rs`, and
  two harnesses sharing a harness symbol are a typed error in `generate_routed`. Test: a duplicate-key
  test on its pairing helper, because unique name assignment makes the duplicate unreachable from
  the public entry.
- L-11. The oracle crate manifest template, the runtime dependency spelling and every contract
  and schema spelling are defined in `core/profile.rs` once.
- L-12. `BoundPackage` and `BoundClause` are named nowhere in `kani/`, `routed/` or `replay/`, and
  after the last V1 reader is replaced nowhere. Until step 4f the interim exceptions, listed by
  file, are the V1 arm: `kani/generate/outcome.rs` (`ObligationItem::BoundClause` holds a
  `&BoundPackage`), `negotiate.rs` (including its test fixture) and `clause.rs`; each entry is
  removed with its step.

### Migration order

Each step is one PR (several for steps 2, 3 and 4), keeps main green and deletes what it replaces.
Per the ruling there is no compatibility layer anywhere in it. It is the order IR-348 files its
tickets in. The planner tracks these numbers: 4e is the public generator entry, 5 is the terminal
map, 6 is the V1 reader deletions and 7 is the publication move.

1. **Shared core, no moves.** Precondition for 1a, a testable condition: QSL's `quire-canonical`
   dependency and CG's resolve to ONE lock entry, so `make deny` passes with the direct
   dependency. That needs QSL to move `quire-canonical` to `branch = "main"`, which waits on the
   owner. 1a (and IR-274) does not merge before it; the driver's lock is the second consumer to
   check. 1a:
   `core/canonical` calling `quire-canonical` directly (RFC 8785, `to_vec` and `sha256`; AD-003),
   deletion of both `deterministic_json` definitions and the nine
   `artifact` wrappers, and the corpus case digest moved onto it. 1b: identity newtypes
   threaded through `kani_obligations`, `state_frame` and `routed_generation`, with the typed
   duplicate error. 1c: `core/profile`, with `RUNTIME_REVISION` deleted. 1d: declare
   `quire-contract-model` directly and import model items from it, before IR removes the root
   glob (IR's AD-006, R3-C2).
2. **Edge removal, then directories.** Precondition: PR 210 has landed, after this AD is approved
   and with a local `make kani` transcript from its head. 2a to 2b are definition moves in the flat
   layout, each removing an edge a rename cannot: 2a moves `ArtifactBundle`, its limits,
   `PublicationDiagnostic`, `PublicationErrorCode` and `PublicationDestinationState` (a field of the
   diagnostic), `SourceProbe` and `SourceRegion` out of `publication` and `oracle` into the modules
   that become `core`, together with what those types hold so that `core` imports nothing: `Artifact`
   and the generation diagnostic types (`GenerationTerminalState`, `GenerationErrorCode`,
   `GenerationDiagnostic`). In the flat layout they are `artifact`, `diagnostic` and `source_map`.
   2b moves the proof-dependency census types (including `ProofDependencyState`, a field of the
   edge and the request) to `census` (flat `kani_census`), and the harness and identity record
   types, flat `kani_identity` because `identity` is already `core/identity.rs` (`KaniObligationHarness`,
   `KaniScalarObligationHarness`, `ObligationKind`, `ObligationBinding`,
   `KaniObligationIdentity`, `ScalarObligationIdentity`, `ScalarObligationArgument`,
   `EmbeddedOracle`, `StateFrameHarness`, `StateFrameProperty`, `StateFrameIdentity`,
   `StateFrameScope`, `StateFieldDomain`, `StateComparison`) to `identity`, since the harness
   records hold them and `identity` would otherwise import `generate`, so
   `kani_execution`, `kani_witness_join` and `spine_replay` stop importing the generators.
   2d-0 is the third definition move and lands after 2c, before 2d: it moves the shared naming
   helpers out of `oracle` into `core/naming.rs` (`bounded_readable_component`,
   `readable_name_component`, `upper_camel`, `unique_names`, `unique_pair`, `oracle_symbol`,
   `reference_identifier`, `observation_name` and the private `rust_component`) and
   `MAX_GENERATED_SOURCE_BYTES` into `core/artifact.rs`, and points every importer at the new
   module path. `observation_name` becomes `pub(crate)` because `reference_key`, which stays in
   `oracle`, calls it; that visibility is the only edit to a moved item. The tests of
   `unique_names` and `oracle_symbol` (in `oracle.rs`, traced to FR-022-AC-9 and TC-033) move to
   `core/naming.rs` with the items, their trace tags unchanged. Without this step
   `core/naming.rs` is never created, because 2d moves `oracle.rs` whole, and L-1 cannot pass.
   It carries no behaviour change: the items otherwise move unchanged and the generated output
   is byte-identical. 2c to 2g are `git mv` plus path fixes, imports by module
   path, no logic change, one PR per subsystem in leaf order: 2c `core`, 2d `oracle` (with `bound`
   landing as `oracle/bound_v1.rs`), 2e `evidence` and `strategy` (the one `pub mod
   bound_strategy` path leaves here; its callers in `tests/it/` and any item reached only by
   that path are re-exported by name or made private in the same PR), 2f `kani` (every flat `kani*` file, `state_frame` and the four `bounded_*`,
   `definedness_arithmetic` and `finite_reference_graphs` files, with the `run`, `output`, `classify`
   split of `kani_execution` and `kani_transcript`, the split of `kani.rs` and `kani_obligations.rs`
   and the scan half of `kani_witness_join`; a verbatim item move: items move unchanged between
   files, with no logic edit and byte-identical output; and the test back-edge, which dissolves
   rather than moves to `tests/it/`. The step 2f item map below names the destination file, the
   visibility and the owning step of every item, so the PR has nothing left to decide), 2g
   `replay`, `routed`, `publication` (with the decode half of `kani_witness_join`).
   The crate-root imports (L-2) are rewritten by the steps that move the files: each of 2d to
   2g rewrites every root-path import in the files it moves to a module path, and 2g-0, a sweep
   PR before the layout test lands, rewrites any that remain (`use crate::{..., Item}` and
   `use crate::Item`, in `#[cfg(test)]` modules too; `lib.rs` keeps its re-export list). The
   sweep also covers inline paths in code, `crate::Item` outside a `use` line, and intra-doc
   links that name a root item; it leaves string literals (the template text `"crate::State"`
   and the like) alone. At the time of writing there are two inline code paths,
   `crate::GenerationDiagnostic` in `kani.rs` and in `kani_obligations.rs`, and ten intra-doc
   links naming a root item, in six files (`generation`, `kani_obligations`, `spine_replay`,
   `kani_witness_join`, `kani_identity`, `oracle`). 2g-0 changes paths only. The layout test
   (L-1, L-2) lands with 2g; it reads `use` lines and also flags `crate::<Item>` with no module
   segment in code outside string literals. It does not read comments, so the doc links are
   rewritten by 2g-0 and not checked by the test.
3. **Typed node access and the scalar split.** `core/ir` with the operator enum, then
   `state_frame`, `exact_scalar`, `composite_equality` and `exact_function` onto it, one PR each.
   The `exact_scalar` PR starts from the step 2d result, where the complete implementation is in
   `oracle/scalar/mod.rs`. It moves shared checked-node walkers into `core/ir`, moves descriptor
   derivation to `oracle/scalar/derive.rs`, node lowering, bounds and operation checks to
   `oracle/scalar/lower.rs`, and source and artifact rendering to `oracle/scalar/render.rs`.
   `mod.rs` keeps the public scalar types and entry points and orchestrates those stages; it does
   not retain their implementations. File-local helpers and tests move with the behavior they
   exercise. The exit check inspects those module responsibilities and the step 3 scalar file
   layout, then runs the existing scalar generation and agreement tests plus the repository gate
   on the code PR. The public API, claim/refusal outcomes and emitted artifacts remain equivalent;
   this step introduces no old-path forwarding module or other compatibility layer. L-8 lands
   with the last typed-node-access PR.
4. **The one generator.**
   - 4a: confirm the regression requirement (L-5) before anything else in this step: the
     quire-integration exemplars pass against CG's branch on the V1 path that serves the control
     today. A CG-side control is added only if the `qsl-replay` facade offers a way to build the
     package (`call_site(...).package`, or QSL source plus `qsl_replay`); otherwise that is a need
     routed to QSL. No copied QI test and no QSL-emitted package file.
   - 4b: `HarnessSpec`, `render` with the cover rule, and the scalar family ported.
   - 4c: the V2 contract arm. First a spec PR in this repository's spec lane amends FR-015 with the
     V2 clause input (design intent above; no ticket found, the planner should file one). Then
     the code: the precondition and contract families take the V2 input and render through
     `HarnessSpec`. The control cannot move in this step: only the QSL-owned follow-up after 4e
     can change the quire-integration test to route through the V2 arm. The V1 arm and the bundle
     keep their own templates until 4f.
   - 4d: the V2 census input (FR-015's census and FR-015-AC-22 and AC-25), carried on the harness
     identity and backed by the types in `kani/census.rs`; and the frame family ported, with
     `generate_state_frame_obligations` becoming an `ObligationItem` arm.
   - 4e: **CG's one public generator entry exists** (`negotiate_kani_obligations` with the scalar,
     contract and frame arms, the public path for everything `generate_kani_bundle` served). This
     is the step the leader reports to the planner when it lands. A QSL-owned follow-up then moves
     QSL's quire-integration exemplars, which call `generate_kani_bundle` today, onto that entry;
     it is a QSL ticket, not CG work.
   - 4f: deletion of `generate_kani_bundle`, `KaniArtifactBundle`, `ProofDependencyGraph`, the V1
     obligation arm and bundle validation, with the matching `interface-001` edit. The census types
     in `kani/census.rs` stay: the V2 census input needs them, and the corpus imports them until
     4g. `validate_dependencies` (census validation, and the `KaniDiagnostic` it returns with the
     identity and path helpers it uses) is not bundle validation for this purpose: the corpus calls
     it until 4g (`bounded_kani_corpus.rs`), so 4f leaves it in place and it is deleted with the
     corpus lowerer at 4g, or when the V2 census input replaces it. It is not in `kani/census.rs`
     because it returns the V1 bundle's error type. Step 2f put it, with `KaniDiagnostic`,
     `KaniErrorCode`, the helpers and `deterministic_json`, in `kani/generate/census_validation.rs`,
     and everything else of the bundle in `kani/generate/v1_bundle.rs`, so 4f deletes `v1_bundle.rs`
     whole and edits nothing in `census_validation.rs` (the V1-only variants of `KaniErrorCode` it
     leaves unused stay there until 4g or the V2 census input removes the file). Merged only after 4e has landed, the QSL follow-up has moved the exemplars and the control
     passes on the V2 contract arm (4c), and the V2 census input exists (4d). FR-015's census and
     FR-015-AC-22 and AC-25 stay verbatim; if the V2 side does not back them by this step, their
     matrix rows go to planned or unbacked, and nothing is deleted or rewritten.
   - 4g: the corpus. Until QSL-353 lands the corpus stays as it is, with its own template: it has
     no symbolic input (a ground `assert!` over literals), and its cover after that assertion
     (FR-015-AC-55, IR-464) is reachability past the assertion only. It is an interim exception
     to L-3 and, until it renders through `HarnessSpec`, outside L-4; FR-015-AC-58's inspection
     covers its text meanwhile. When QSL-353 lands, the hand-built package
     lowerer is retired, the rows are backed from QSL-emitted packages built through the facade,
     and a corpus case with symbolic input is rendered through `render.rs` under the cover rule;
     a case with no symbolic input is not rendered as a proof. L-3 lands after 4f and 4g. The Kani family lowerings
     are implemented in CG by IR-347; IR removal follows this consumer change.
5. **One output reader and the terminal map.** The playback scanning already sits in
   `kani/output/playback.rs` (step 2f moved it verbatim); this step changes the interface:
   `playback.rs` returns typed entries and its own typed refusal, `replay/witness.rs` takes them
   and `DecodeFailure` is built from that refusal, and `classify_kani_run` takes the extracted
   playback in place of the console text, which removes the interim edge from `kani/classify.rs`
   to `counterexample_playback`. Batching follows, in
   `run/` only. `ReplayInputs::backend_manifest` and the manifest members `spine_replay` builds
   from it are deleted here, with the tool pin QSL-351 drops. Step 5's reader and the C-09 map
   (`kani/terminal.rs`) produce `TerminalValue` (the FR-029 map from `KaniRunOutcome`, and the
   C-09 map from IR's `KaniOutcome` with the replay outcome). The `Inconclusive(cause)` types
   are merged in QSL 02530e7 (QSL has ruled, relayed on IR-465, vacuity stays `Proved{0}`, no
   `NonZero`; the tool pin is gone, #551), and the FR-029 map is built on them over the pair
   (outcome, replay settlement). The IR-outcome map (FR-030) is built the same way as
   `ir_outcome_terminal_value`, with `Declined` carrying IR's `Std001Code` as `DeclineCode::Std001`
   (QSL #634).
   - The map follows QSL's merged ADR-013 C-09 and ADR-011 T-13 (QSL #550, QSL-354; checked at QSL
     `origin/main`: T-13 says the driver `quire-driver` owns the S6b run, the E9 replay
     (`qsl_replay::replay`) and the FR-331 terminal record, CG owns the C-09 map and settles
     dispositions, and parity is settled inside `replay`). It adds no behaviour claim of this AD;
     FR-029 is amended by its own ticket, and the two causes are merged in QSL. The map is total over
     (Kani outcome, QSL replay result): a reproduced replay gives `Refuted`; a replay
     disagreement, or one that completes no value, gives
     `Inconclusive(InconclusiveCause::ReplayParity)`; a non-fault `ReplayRefusal` (identity
     mismatch, decode refusal, stale dependency, limit reached) gives
     `Inconclusive(InconclusiveCause::ReplayRefused)` carrying that refusal's catalog code; a
     fault (QSL `InternalFault`) gives `TerminalValue::Failed`. A refuted Kani outcome never
     becomes `Refuted` without a reproduced replay. The spellings are `replay_parity` and
     `replay_refused`.
   - Closed-set rule (QSL's ruling, relayed by the IR planner; design intent following QSL's
     merged C-09): `ReplayRefused` carries only QSL `ReplayRefusal` codes, a closed set QSL owns.
     CG-origin defects map to `TerminalValue::Failed`, with no CG code inside `ReplayRefused`:
     `SpineReplayError::{UnboundArgument, FieldDelimiter, Transcript, WrongArm, Identity}`,
     `FrameReplayError::{Transcript, Envelope, Name}`, `ReplayPackageError::InvalidFunction` (none
     carries a QSL code), the envelope failure, and a Kani playback outside the harness proof bound, which CG checks before building
     the envelope. AD-001's Failure view keeps each a distinct typed state before the map.
   - QSL's answer on CG-side refusals (relayed; keyed on when the refusal happens, and consistent
     with the when-rule in AD-003's R-Q1 as in the follow-up PR 216, which is not edited here):
     a refusal before Kani runs, when the obligation's own input is refused (IR's `Refused`,
     `InvalidInput` and `IncompleteInput` outcomes; nothing was proved), is
     `Declined(ProofRefusalCause)`, mapped by the IR-outcome map. A caller-lock refusal is not
     that case: in CG it is `DependencyLockError`, reached through `ReplayPackageError::Dependencies`
     (interface-001; FR-016-AC-16 to AC-19), which happens in replay setup AFTER Kani refuted, so
     it is the after-Kani case and not a `Declined` candidate. A replay setup refused on data
     after a refuted Kani run, by a QSL refusal that carries a code (`CallSiteRefusal` `Compile` or
     `UnknownFunction`, a dependency-selection refusal, and `DependencyLockError::Input`, which is CG
     applying the same rule), is
     `Inconclusive(ReplayRefused)` with that QSL code. QSL ruled this (relayed on IR-465): `Declined`
     is only for a refusal before any backend run, and QSL amends its FR-121 to say so.
     `InvalidFunction` and `Name` carry no QSL code and map to `Failed`;
     CG has no `DependencyLockError::Duplicate` (FR-016-AC-24, built): the lock admission dropped its own duplicate
     pre-check and builds QSL's dependency input, so QSL refuses a repeated identity as
     `invalid_package` and it arrives as `DependencyLockError::Input` (QSL ruling, relayed on IR-465,
     a QSL ruling recorded by the planner; AD-003 R-Q1). A decode failure (`DecodeFailure`, `EvidenceFailureCause::Decode`) is not in that list:
     it is a CG defect, a playback that does not type against the bindings CG persisted, and maps
     to `Failed` (AD-003, link 7). Faults stay `Failed`. `CallSiteRefusal::code()` and
     `DependencyInputRefusal::code()` are already in QSL `main`, so no code is missing;
     `Inconclusive(ReplayRefused)` is in QSL's types.
   - Layering. The C-09 map is a public entry in `kani/terminal.rs` that the driver calls; the
     driver runs the obligation and the replay and pairs the two, as QSL's merged T-13 says. Its
     first input is IR's `KaniOutcome` (ADR-013 C-09's `KaniOutcomeKind`); the FR-029 map from
     CG's `KaniRunOutcome` is the other entry. Its second input is a replay-outcome type that
     `kani/terminal.rs` defines from `qsl-replay` types (a QSL result, a QSL `ReplayRefusal`, a
     QSL fault) plus two CG-raised variants, so `kani` imports nothing from `replay`: a CG-origin
     defect, and a setup refusal on data (the after-Kani case above) that carries a QSL code from
     QSL-352; those codes already exist in QSL `main`. The setup-refusal variant carries the code of a
     `CallSiteRefusal` or a `DependencyLockError::Input`, which includes a repeated dependency identity (AD-003 R-Q1).
     `replay/` imports `kani` (downward) and owns the conversion: it turns its own errors
     (`SpineReplayError`, the envelope failure, the out-of-bound playback) and QSL's result into
     that type, so a CG-origin failure reaches the map as the CG-defect variant and becomes
     `Failed` there. `routed/` does not pair.
6. **V1 readers.** Each of `harness`, `strategy/bound`, `evidence/bound_coverage`,
   `oracle/bound_v1.rs` and `oracle/boolean_v1.rs` is replaced and deleted with its V2 criteria.
   IR-364 (the V2 strategy chain: FR-002, FR-004 and FR-008 to FR-013 over `CheckedPackageV2`) is
   owned by the IR team and is ordered before this step; no V1 reader is deleted until its
   criteria exist. `typed_dependency_parameters` and `generate_boolean_oracle`, which the V2
   `kani` and `kani_obligations` import, are moved or replaced before `boolean_v1.rs` is deleted
   (see the `oracle` naming and V1 rows of the module map).
7. **Spec follows the code.** One spec PR: the registry rows by directory; FR-005 and
   TC-001, TC-002, TC-007 to `spec/publication/`; SUR-001 to `core/functional/`; `interface-001`
   and `tests.md` fixed. The same PR repoints every spec document that cites a flat source path
   deleted by steps 2a to 2g (`src/spine_replay.rs`, `src/kani_witness_join.rs`,
   `src/frame_replay.rs`, `src/capability.rs`, `src/routed_generation.rs`, `src/kani_transcript.rs`,
   `src/kani_execution.rs`, `src/oracle.rs`, `src/exact_scalar.rs` and the like), and the
   registry's module column. The list measured at step 2g, by a grep of `spec/` for each deleted
   flat file name: FR-016, FR-017, FR-021, FR-024, TC-026, TC-027, TC-035, ADR-002, AD-001,
   AD-002, AD-003, `interface-001` and `spec/oracle/matrix/tests.md`. Paths that name another
   repository's files (AD-003's `src/kani/outcome.rs` is IR's; FR-021's
   `src/exact/expression.rs` is RT's) are not stale. The step re-greps before it starts, because
   later steps delete more flat paths. `git mv`, ids unchanged. When this step lands, the registry note in
   `spec/spec.md` that records the FR-005 exception becomes obsolete and is deleted in the same
   PR, as is the SUR-001 note. A separate follow-up, not edited here: AD-001's Current state and
   Risks are stale against this AD (it lists the corpus and profile modules as V1, and says Kani
   publishes no machine-readable verdict while the `--export-json` decision says otherwise). The
   spec PR of this step is the place to fix them.

### Step 2f item map

Step 2f is a verbatim item move, but the first form of this AD named files, not items, and
several files (`kani.rs`, `kani_obligations.rs`, `kani_execution.rs`, `kani_transcript.rs`,
`kani_witness_join.rs`) are split across the new files. A coder cannot move an item whose
destination the AD does not state, and the first coder for 2f stopped for that reason. This map
states the destination of every item of every file step 2f touches. It was read from `src/` at
`origin/main` after step 2e (PR 228), by item name; no build was run for it, so the visibilities
below are the narrowest that a read of the call sites says the new file boundaries need, and the
coder confirms them by compiling.

**Counting rule.** An item is a column-0 `fn`, `struct`, `enum`, `trait`, `const`, `static`,
`type`, `impl` or `macro_rules!` (any `pub` form) outside the `mod tests` block, and a
four-space-indented item of those kinds inside it. `use` lines and the `mod tests` line are not
items. An `impl` block is one item. Methods, nested items, variants and fields belong to their
parent item. Every item of the files listed below is assigned to exactly one new file in these
tables; each table states the count it covers, and the per-destination rows add up to it.

| Source file | Production items | Test items | Total |
| --- | --- | --- | --- |
| `kani.rs` | 38 | 0 | 38 |
| `kani_obligations.rs` | 70 | 12 | 82 |
| `kani_execution.rs` | 46 | 37 | 83 |
| `kani_transcript.rs` | 31 | 21 | 52 |
| `kani_witness_join.rs` | 19 | 15 | 34 |
| `state_frame.rs` | 36 | 2 | 38 |
| `bounded_kani_corpus.rs` | 22 | 31 | 53 |
| `bounded_kani_profile.rs` | 3 | 5 | 8 |
| `bounded_collections.rs` | 1 | 2 | 3 |
| `definedness_arithmetic.rs` | 1 | 2 | 3 |
| `finite_reference_graphs.rs` | 1 | 0 | 1 |
| `kani_census.rs`, `kani_identity.rs` (step 2b files, renamed only) | 8, 16 | 0 | 24 |

#### What the move may and may not change

- An item's text is unchanged: no logic, name, signature, attribute, doc comment or string literal
  edit. The string `"kani_execution::classify_kani_run"` that `classify_success` hands to IR's
  `KaniOutcome::proved_from_checks` stays exactly as written: it is data, not a path, and changing
  it would be a behaviour edit.
- What may change: `use` lines and module paths; an item's visibility, widened to the narrowest form
  its new boundary needs (`pub(super)` for use inside the directory, `pub(crate)` for use outside
  it), as the Visibility column states, with only the fields the row names widened; the
  `//!` header of a split file, divided between its successors; the INTERIM header sentence that
  `v1_bundle.rs` and `census_validation.rs` each carry (the step that deletes the file); and
  intra-doc link paths. A row that
  says "unchanged" widens nothing.
- Files created: every file this map names, and the `mod.rs` of `kani/`, `kani/generate/`,
  `kani/generate/lower/`, `kani/generate/corpus/`, `kani/output/` and `kani/run/`. A `mod.rs` holds
  only module declarations and their comments, as the 2e `mod.rs` files do. Files not created:
  `generate/spec.rs` and `generate/render.rs` (step 4b) and `terminal.rs` (step 5). Template text
  stays where it is today (`render_scalar`, `render_precondition`, `render_contract`, and the
  state-frame and corpus templates); where it goes later is for the steps that own it.
- `lib.rs` replaces the flat `mod` lines of the moved files with `mod kani;` and points its
  `pub use` list at the new module paths (`kani::generate::outcome::...`, `kani::run::...` and so
  on). The set of re-exported names does not change, so no test code under `tests/` changes (comment lines and assertion or failure message strings that cite a deleted source path may be updated, paths only).
- Files outside the move whose imports of moved items are rewritten to the new module paths:
  `spine_replay.rs` (`kani_identity` and the `DecodeFailure` import), `routed_generation.rs`
  (`kani_identity`, and the test module's `kani::KaniSolver`, which becomes `kani::abi::KaniSolver`),
  the inline `crate::kani::validate_dependencies` calls in the corpus, the flat `kani_witness_join.rs` (`kani::abi`, `kani::identity`,
  `kani::output::playback`), and the intra-doc links in `oracle/scalar/mod.rs` that name
  `kani_obligations::render_scalar`.
- `kani_census.rs` becomes `kani/census.rs` and `kani_identity.rs` becomes `kani/identity.rs`,
  whole, as 2b prepared; the latter imports `kani::abi` where it imports `kani` today.

#### Decisions

- **D-1. The two halves of `kani_witness_join`.** Step 2f moves the scan half, which is Kani's
  text and so belongs in `kani/output/`; step 2g moves the decode half to `replay/witness.rs`. The
  scan half is `HARNESS_MARKER`, `CHECK_MARKER`, `check_clause`, `select_assertion_block`,
  `read_block`, `unescaped_quote`, `bracketed`, `concrete_entries`, the `Playback` and `CheckKind`
  types, and `DecodeFailure` with its `new`. `DecodeFailure` goes with the scan because the scan
  raises it in every refusal and `kani` must not import `replay`; `decode_falsification` raises
  it too and imports it from `kani/output/playback.rs`, the downward direction. The decode half
  is `WitnessSchemaError`, `argument_types`, `byte_width`, `decode_falsification`, `decode_values`,
  `boolean_comment` and `first_out_of_domain`. `boolean_comment` is not scan: its one caller is
  `decode_values`, which cross-checks a value against Kani's decoded-value comment, and the target
  design has `replay/witness.rs` compare the rendered value; it stays with the decode half and is an
  interim exception (below). After 2f the flat `kani_witness_join.rs` holds the decode half and its
  whole test module, and imports the scan half from `kani::output::playback`. All 15 of its tests
  stay with the decode half, because they exercise `decode_falsification`; the one that calls
  `read_block` and reads the `check_text` of a `Playback` does so through the `pub(crate)` the scan
  half gains, which points downward.
- **D-2. `kani.rs`.** `kani/abi.rs` holds only what `abi` is named for: binding roles, primitive
  types, integer bounds, `KaniSolver`, `i64_literal`, `adapter_options` and `readable_component`.
  `KaniSubjectBinding` is not an `abi` item: only the V1 bundle uses it (as a field of
  `ProofDependencyGraph` and in the bundle's ABI), so it goes with the bundle. The bundle is
  retired at 4f but is still live (`tests/it/kani_generation.rs` drives it, and the planner records
  that QSL's V1 control runs through it; relayed), so it needs a home that exists and that 4f
  deletes whole: `kani/generate/v1_bundle.rs`, a file with an INTERIM header that says so.
  `validate_dependencies` must outlive 4f (the corpus calls it), and it returns `KaniDiagnostic`, so
  `KaniDiagnostic`, `KaniErrorCode` and the three helpers `validate_dependencies` uses
  (`validate_plain_identity`, `validate_path`, `single_diagnostic`) go to a second interim file,
  `kani/generate/census_validation.rs`, which 4f does not touch. `deterministic_json` also goes
  there, not into `v1_bundle.rs`: the corpus calls it, step 1a (held) is what deletes it, and 4f
  must not strand the corpus if 4f merges first. The private `artifact` wrapper has one caller (the
  bundle) and goes with it; step 1a deletes it with the other eight.
- **D-3. `kani_obligations.rs`: where the shared items live.** The file does five jobs: the
  request and result vocabulary, the negotiation passes over a request's items, the scalar family,
  the V1 clause lowering the precondition and contract families share, and each family's
  renderer. The first form of this AD named four files (`negotiate`, `scalar`, `precondition`,
  `contract`) and no home for the items several families use. The split is by dependency, not by
  count, because the family files and `negotiate.rs` must not import each other (L-2): `Outcome`
  holds the lowered clause form and the lowered scalar form (`Lowered` and `LoweredScalar`), the
  family files build those forms, so if `Outcome` lived in a family the family files and
  `negotiate.rs` would form a cycle. `Outcome` itself is `negotiate.rs`'s; `outcome.rs` holds the
  vocabulary the families name. Three
  rules settle every item. (1) What the entry returns or takes, and so what every family names
  (`UnsupportedObligation`, `DerivedDomain`, `ObligationItem`, the dispositions, the limits), is a
  leaf, `outcome.rs`. (2) What reads the whole request is the entry's, in `negotiate.rs`: the
  classifiers (`classify`, `classify_clause`, `classify_node`, `classify_claim` and its guard
  `refuse_unknown_node_kind`), `assign_names`, `reject_duplicates_and_mixtures`,
  `resolve_assumptions` and `unify_subject_signatures`, and the clause renderer's dispatch
  `render`, which picks `precondition.rs` or `contract.rs` and so imports both, which `clause.rs`
  cannot. (3) What lowers or renders one item is its family's: the scalar family (including
  `derive_domain` and `unsatisfiable`, which only `classify_claim` and the scalar tests call, so
  they are not shared across families) in `scalar.rs`; the V1 clause lowering that precondition and
  contract both use (`lower_clause`, `ClauseOracle`, the subject ABI, slots, `call`,
  `contract_contexts`, `symbolic_arguments`) in `clause.rs`; `render_precondition` in
  `precondition.rs` and `render_contract` in `contract.rs`. `clause.rs` is justified by the AD
  itself: step 4c gives the precondition and contract families a V2 input and says the V1
  lowering takes `BoundClause`, so the shared V1 lowering is its own file that 4c reshapes and 4f
  trims. The three items that `render_scalar` and `render` both call (`harness_path`, `record` and
  `artifact`) are in `record.rs`, below both files: in `negotiate.rs` they would be imported by
  `scalar.rs` against the order, and in `scalar.rs` they would be a helper of one family that the
  other's dispatch borrows. `record.rs` holds today's items only; later steps may move them.
  `generate/mod.rs` holds declarations only.
- **D-4. `kani_execution.rs`.** Classification is `kani/classify.rs` (`KaniInconclusiveReason`,
  `KaniRunOutcome`, `ClassifiedRun`, `classify_kani_run`, `classify_report`, `classify_success`,
  `inconclusive`). `run/` holds five files: `tool.rs` (the backend and its location), `harness.rs`
  (the three-kind harness view), `launch.rs` (spawn, capture cap, timeout, process-group kill),
  `report_file.rs` (the unique report path, stale removal and bounded read, all of L-7) and
  `execute.rs` (the request, refusal and evidence types, `execute_kani_obligation`, the launch
  command, `launch_evidence` and the harness-in-crate file read). `classify` imports `output` and
  `identity`; `run` imports `classify`, `output`, `identity` and `abi`; `launch.rs` imports nothing
  of the crate. `read_file` stays in `execute.rs`, its one caller, though it returns
  `KaniToolError`.
- **D-5. `kani_transcript.rs`.** `output/report.rs` is everything but `counterexample_playback`
  and the four `PLAYBACK_*` constants, which are `output/playback.rs`, beside the scan half of D-1.
  The split follows what each reads: the report is JSON, the playback is console text.
- **D-6. The test back-edge, per test.** The AD said both that the edge `kani_transcript`'s
  tests have into `kani_execution` lives in `tests/it/` and that it moves to the classify side.
  It is the second, and no test goes to `tests/it/`: the edge needs no generator, it is a
  classification test, and `kani/classify.rs` may import `output`. The decision is per test, by
  what the test calls. A test that calls `classify_kani_run`, `classified`, `KaniRunOutcome` or
  `KaniInconclusiveReason`, or that uses the real-capture fixtures (`Capture`, `capture!`,
  `mutated`), moves to a `real_capture` test module in `kani/classify.rs`, which may import
  `output`; a test that needs only the report parse and no fixture moves to `output/report.rs`; the
  one playback test moves to `output/playback.rs`. The fixtures are `include_str!`d by a path
  relative to the file that holds the macro, so they change with the file: from
  `src/kani_transcript.rs` the path is `../tests/fixtures/kani-report/<name>.{json,stdout,exit}`;
  from `src/kani/classify.rs` it is `../../tests/fixtures/kani-report/<name>.{json,stdout,exit}`.
  Only `classify.rs` includes them. Three parse-only tests call `mutated`, which reads a fixture,
  so they go with the fixtures, not with `report.rs`:
  `tc_027_a_report_without_exactly_one_harness_is_refused`,
  `tc_027_an_unknown_line_is_none_and_a_non_numeric_line_is_refused` and
  `tc_027_an_unnamed_check_category_is_a_property`. `real_capture` thus holds 13 tests and 5
  helpers, `output/report.rs` 2 tests, `output/playback.rs` 1.
- **D-7. Test helpers shared by several new files.** `kani_execution`'s tests share helpers with each
  other across what become five files and `classify.rs`. A helper used by the tests of one new file
  stays in that file's test module. A helper used by the tests of more than one goes to
  `kani/test_support.rs`, declared `#[cfg(test)]` in `kani/mod.rs`, with its visibility widened to
  `pub(crate)`: `state_frame_harness`, `report`, `PASSED`, `COVER_OK`, `COVER_NO` and
  `discover_scratch`. It imports `identity` and `abi` and no generator. `classify.rs` holds two test
  modules, `real_capture` (D-6) and `synthetic` (the four `kani_execution` classification tests
  with `classify`, `COVER_PLAYBACK` and `ASSERTION_PLAYBACK`), because both define a helper named
  `report` or `classify`/`classified` with a different signature; neither is renamed.
  `a_precondition_harness_with_no_checks_is_decided_by_its_cover_not_the_zero_checks_rule`
  calls `launch_evidence`, so it goes to the `execute.rs` tests, not to `classify.rs`, which may not
  import `run`.
- **D-8. Unsplit files.** `state_frame.rs` is `generate/frame.rs`; `bounded_kani_corpus.rs` is
  `generate/corpus/bounded_kani_corpus.rs`; the four thin files keep their names under
  `generate/lower/`, as the 2e directories kept theirs. The corpus is not split at 2f: its
  lowerer is retired at 4g and splitting 1,484 lines first only moves code that is about to
  change. `frame.rs` imports `MAX_OBLIGATION_UNWIND` from `outcome.rs`, not from the old crate-root
  path; the corpus imports `lower::*`, `census_validation` and `kani::census`. The inherent
  `impl StateComparison` in `state_frame.rs` is an impl for a type that lives in
  `kani/identity.rs`; it stays in `frame.rs`, unchanged, because moving it is a logic-free but
  non-verbatim edit and 4d reshapes the frame family anyway.

#### Interim exceptions

Each is a place where the target of this AD is not yet true, stated so that no step stops for it.

| Exception | Where | Removed by |
| --- | --- | --- |
| The classifier takes console text and calls `counterexample_playback`, so `classify` imports `output/playback` | `kani/classify.rs` | step 5 (the classifier takes the extracted playback); L-6's listed exception |
| The decode half compares Kani's decoded-value comment (`decode_values`, `boolean_comment`) | flat `kani_witness_join.rs`, then `replay/witness.rs` | step 5 (typed entries carry the rendered value); L-6's listed exception |
| `DecodeFailure` is defined in `kani/output/playback.rs`, where the scan raises it | `kani/output/playback.rs` | step 5 (the reader gets its own typed refusal; `DecodeFailure` moves to `replay/witness.rs`) |
| The V1 bundle generator, still live | `kani/generate/v1_bundle.rs` | step 4f deletes the file |
| `validate_dependencies`, the V1 error type it returns and `deterministic_json` | `kani/generate/census_validation.rs` | 4g or the V2 census input for the first two; step 1a for `deterministic_json` |
| The shared V1 clause lowering and the V1 arm (`ObligationItem::BoundClause` in `outcome.rs`, `classify_clause` in `negotiate.rs`) | `kani/generate/clause.rs`, `outcome.rs`, `negotiate.rs` | step 4c reshapes the lowering for the V2 input; 4f deletes the V1 arm |
| The persisted-record helpers | `kani/generate/record.rs` | not scheduled; `artifact` by step 1a |
| Each family renders its own template; the corpus keeps its own, with its cover after the assertion (FR-015-AC-55) | `scalar.rs`, `precondition.rs`, `contract.rs`, `frame.rs`, `corpus/bounded_kani_corpus.rs` | 4b, 4c, 4d; 4g for the corpus |
| The three semantic-family forwarding modules | `kani/generate/lower/{definedness_arithmetic,bounded_collections,finite_reference_graphs}.rs` | IR-347 replaces the forwarders with CG-owned lowerings and request/result types; IR removes its former exports afterwards |
| `generate_state_frame_obligations` is a second public entry beside `negotiate_kani_obligations` | `kani/generate/frame.rs` | step 4d |

#### Item tables

Visibility: "unchanged" widens nothing; "pub(super)" and "pub(crate)" name the form a private item
(or a private field, where the cell says fields) is widened to. A test row's visibility is "n/a".

**`kani.rs` (38 items).**

| Old items | Count | New file | Visibility | Note |
| --- | --- | --- | --- | --- |
| `KaniBindingRole`, `KaniPrimitiveType`, `impl KaniPrimitiveType`, `KaniIntegerBounds`, `KaniSolver`, `impl KaniSolver` | 6 | `kani/abi.rs` | unchanged | `pub` types and `pub(crate)` methods |
| `i64_literal`, `adapter_options`, `readable_component` | 3 | `kani/abi.rs` | unchanged | `pub(crate)` |
| `KaniErrorCode`, `impl KaniErrorCode`, `KaniDiagnostic` | 3 | `kani/generate/census_validation.rs` | unchanged | `pub`; `validate_dependencies` returns them (D-2) |
| `validate_dependencies` | 1 | `kani/generate/census_validation.rs` | unchanged | `pub(crate)`; the corpus calls it |
| `validate_plain_identity`, `validate_path`, `single_diagnostic` | 3 | `kani/generate/census_validation.rs` | pub(super) | the bundle's validation and rendering call them |
| `deterministic_json` | 1 | `kani/generate/census_validation.rs` | unchanged | `pub(crate)`; the corpus and the bundle call it; step 1a deletes it |
| `KaniSubjectBinding`, `KaniRequest`, `ProofDependencyGraph`, `KaniArtifactBundle` | 4 | `kani/generate/v1_bundle.rs` | unchanged | `pub`; re-exported until 4f |
| `generate_kani_bundle` | 1 | `kani/generate/v1_bundle.rs` | unchanged | `pub` |
| `SubjectAbi`, `KaniSource`, `validate_request`, `derive_subject_abi`, `subject_binding`, `binding_matches_value_type`, `predicate_arguments`, `result_access`, `render_kani_source`, `render_framing`, `result_type`, `render_symbolic_arguments`, `render_result_bounds`, `map_clause_diagnostics`, `kani_symbol`, `artifact` | 16 | `kani/generate/v1_bundle.rs` | unchanged | private, used only inside the bundle |

Rows add to 6 + 3 + 3 + 1 + 3 + 1 + 4 + 1 + 16 = 38.

**`kani_obligations.rs` (70 production items, 12 test items).**

| Old items | Count | New file | Visibility | Note |
| --- | --- | --- | --- | --- |
| `MAX_OBLIGATION_ITEMS`, `MAX_OBLIGATION_UNWIND`, `ObligationItem`, `KaniObligationRequest`, `KaniObligationError`, `ObligationSubject`, `DerivedDomain`, `UnsupportedObligation`, `InvalidObligationItem`, `ObligationDisposition`, `ObligationRecord`, `KaniObligationOutcome`, `impl KaniObligationOutcome` | 13 | `kani/generate/outcome.rs` | unchanged | `pub`, re-exported by `lib.rs`; the vocabulary of the entry (D-3 rule 1) |
| `negotiate_kani_obligations` | 1 | `kani/generate/negotiate.rs` | unchanged | `pub`, the one entry |
| `validate_request`, `ItemState`, `ItemIdentity`, `Outcome`, `impl Outcome`, `supported_without_harness`, `NameKey`, `clause_key` | 8 | `kani/generate/negotiate.rs` | unchanged | private; the negotiation state |
| `classify`, `classify_clause`, `classify_node`, `refuse_unknown_node_kind`, `classify_claim` | 5 | `kani/generate/negotiate.rs` | unchanged | private; the classifiers map an item to an `Outcome` (rule 2). `classify_clause` is the V1 arm |
| `assign_names`, `named_oracle_source`, `reject_duplicates_and_mixtures`, `anchor_operation`, `resolve_assumptions`, `SubjectGroup`, `unify_subject_signatures` | 7 | `kani/generate/negotiate.rs` | unchanged | private; passes over every item of the request |
| `kind_name`, `render` | 2 | `kani/generate/negotiate.rs` | unchanged | private; `render` is the clause renderer's dispatch (rule 2); `kind_name` has `render` as its one caller |
| `harness_path`, `record`, `artifact` | 3 | `kani/generate/record.rs` | pub(super) | `render_scalar` and `render` call them |
| `LoweredClause`, `ClauseOracle`, `Symbols` | 3 | `kani/generate/clause.rs` | pub(super), fields pub(super), except `ClauseOracle.parameters`, which stays private | the lowered V1 form; `negotiate`, `precondition` and `contract` read the fields |
| `Parameter` | 1 | `kani/generate/clause.rs` | unchanged | private; only `abi` and `call`, in `clause.rs`, read it, so a private field of private type compiles |
| `obligation_kind`, `lower_clause`, `clause_stem`, `symbols`, `contract_contexts`, `abi`, `call`, `symbolic_arguments` | 8 | `kani/generate/clause.rs` | pub(super) | called from `negotiate`, `precondition` and `contract` |
| `SlotContext`, `Abi`, `impl Abi` | 3 | `kani/generate/clause.rs` | pub(super), fields and `access` pub(super) | `negotiate` and `contract` read them |
| `oracle_function_symbol`, `slot` | 2 | `kani/generate/clause.rs` | unchanged | private, one file's helpers |
| `LoweredScalarClaim` | 1 | `kani/generate/scalar.rs` | pub(super), fields `node_id`, `operation_identity`, `module_symbol` and `harness_symbol` pub(super), the rest private | `negotiate`'s `Outcome::LoweredScalar` and `assign_names` read those four; widening `operation` would expose the private `ScalarOperation` (`private_interfaces`) |
| `scalar_stem`, `ScalarLoweringRefusal`, `lower_scalar_claim`, `derive_domain`, `unsatisfiable`, `render_scalar` | 6 | `kani/generate/scalar.rs` | pub(super) | `classify_claim`, `assign_names` and `negotiate` call them |
| `ScalarOperation`, `impl ScalarOperation`, `not_symbolic`, `count_bounds` | 4 | `kani/generate/scalar.rs` | unchanged | private to the scalar family |
| `render_precondition` | 1 | `kani/generate/precondition.rs` | pub(super) | called by `render` |
| `render_contract` | 1 | `kani/generate/contract.rs` | pub(super) | called by `render` |
| `CONTRACT_COVER` | 1 | `kani/generate/contract.rs` | unchanged | private; its one user is `render_contract` |
| Tests: `tc_025_every_clause_kind_maps_to_at_most_one_obligation_kind` | 1 | `clause.rs` tests | n/a | calls `obligation_kind` |
| Tests: `tc_025_derive_domain_reads_binding_shaped_range_members`, `tc_025_range_members_are_read_by_name_not_position`, `tc_025_inverted_ranges_are_unsatisfiable_and_ordered_ranges_are_not`, `tc_026_a_domain_outside_i64_is_a_typed_refusal_not_a_panic` | 4 | `scalar.rs` tests | n/a | call `derive_domain`, `unsatisfiable`, `lower_scalar_claim` |
| Tests: `render_probe_package`, `render_probe_clause`, `render_probe_request`, `render_probe_lowered`, `render_refuses_a_generated_source_over_the_byte_ceiling`, `render_refuses_a_generated_source_that_fails_to_parse`, `render_refuses_a_frame_as_not_a_clause_oracle` | 7 | `negotiate.rs` tests | n/a | call `classify` and `render`; they build a `BoundPackage` fixture, which `kani/` may name until 4f |

Rows add to 13 (`outcome.rs`) + 23 (`negotiate.rs`: 1 + 8 + 5 + 7 + 2) + 3 (`record.rs`) + 17
(`clause.rs`: 3 + 1 + 8 + 3 + 2) + 11 (`scalar.rs`: 1 + 6 + 4) + 1 (`precondition.rs`) + 2 (`contract.rs`) =
70 production items, and 1 + 4 + 7 = 12 tests.

**`kani_execution.rs` (46 production items, 37 test items).**

| Old items | Count | New file | Visibility | Note |
| --- | --- | --- | --- | --- |
| `KaniTool`, `KaniToolError`, `impl fmt::Display for KaniToolError`, `impl std::error::Error for KaniToolError`, `KaniInstallation`, `impl KaniInstallation` | 6 | `kani/run/tool.rs` | unchanged | `discover_from` stays private; its tests move with it |
| `KaniExecutableHarness`, `impl From<&KaniObligationHarness> for KaniExecutableHarness`, `impl From<&KaniScalarObligationHarness> for KaniExecutableHarness`, `impl From<&StateFrameHarness> for KaniExecutableHarness` | 4 | `kani/run/harness.rs` | unchanged | `pub` |
| `HarnessView`, `impl KaniExecutableHarness` (`view`) | 2 | `kani/run/harness.rs` | pub(super), fields and `view` pub(super) | `execute.rs` reads the view |
| `LaunchOutcome`, `run_launcher_with_timeout` | 2 | `kani/run/launch.rs` | unchanged | `pub` |
| `LAUNCHER_POLL_INTERVAL`, `CAPTURE_LIMIT`, `STOP_DRAIN_LIMIT`, `wait_until`, `spawn_capture`, `capture_tail`, `kill_process_tree` | 7 | `kani/run/launch.rs` | unchanged | private; the capture tests move with them |
| `REPORT_FILE_STEM`, `REPORT_SEQUENCE`, `REPORT_LIMIT` | 3 | `kani/run/report_file.rs` | unchanged | private |
| `fresh_report_path`, `remove_stale_report`, `read_report` | 3 | `kani/run/report_file.rs` | pub(super) | `execute.rs` calls them |
| `KaniExecutionRequest`, `KaniExecutionRefusal`, `impl fmt::Display for KaniExecutionRefusal`, `impl std::error::Error for KaniExecutionRefusal`, `impl From<KaniReportRefusal> for KaniExecutionRefusal`, `impl From<KaniToolError> for KaniExecutionRefusal`, `KaniExecutionEvidence` | 7 | `kani/run/execute.rs` | unchanged | `pub` |
| `execute_kani_obligation`, `kani_launch_command`, `launch_evidence` | 3 | `kani/run/execute.rs` | unchanged | `pub` |
| `launch_command`, `read_file` | 2 | `kani/run/execute.rs` | unchanged | private |
| `KaniInconclusiveReason`, `KaniRunOutcome`, `ClassifiedRun`, `classify_kani_run` | 4 | `kani/classify.rs` | unchanged | `pub` |
| `classify_report`, `classify_success`, `inconclusive` | 3 | `kani/classify.rs` | unchanged | private |
| Tests: `state_frame_harness`, `report`, `PASSED`, `COVER_OK`, `COVER_NO`, `discover_scratch` | 6 | `kani/test_support.rs` | pub(crate) | shared by the tests of several files (D-7) |
| Tests: `COVER_PLAYBACK`, `ASSERTION_PLAYBACK`, `classify`, `tc_027_run_classification_never_defaults_to_verified`, `a_report_with_no_successful_check_is_inconclusive_not_verified_even_with_every_cover_satisfied`, `tc_027_an_exhausted_unwind_bound_is_inconclusive_not_falsified`, `tc_027_an_unreadable_or_missing_report_is_refused_never_inconclusive` | 7 | `classify.rs` tests, `synthetic` module | n/a | call `classify_kani_run` only |
| Tests: `tc_027_a_state_frame_harness_reports_the_kind_of_what_it_proves` | 1 | `harness.rs` tests | n/a | calls `view` |
| Tests: `tc_027_the_launcher_resolves_through_cargo_home_then_path` | 1 | `tool.rs` tests | n/a | calls `discover_from` |
| Tests: `tc_027_the_report_is_read_bounded_and_refused_not_truncated` | 1 | `report_file.rs` tests | n/a | calls `read_report`, `remove_stale_report` |
| Tests: `a_precondition_harness_with_no_checks_is_decided_by_its_cover_not_the_zero_checks_rule`, `run_stand_in`, `run_stand_in_into`, `tc_027_execution_reads_only_the_report_its_own_run_exported`, `tc_027_the_launch_exports_the_report_after_the_harness_options`, `tc_027_concurrent_runs_in_one_target_directory_keep_their_own_reports`, `a_timed_out_launch_carries_no_exit_code_into_the_evidence` | 7 | `execute.rs` tests | n/a | call `launch_evidence`, `execute_kani_obligation`, `kani_launch_command` |
| Tests: `a_run_exceeding_its_budget_kills_a_real_grandchild_not_only_the_direct_child`, `process_gone_within`, `GRANDCHILD_REAP_WAIT`, `GRANDCHILD_KILL_TIMEOUT_LADDER`, `a_process_orphaned_just_before_the_kill_does_not_block_this_calls_own_return`, `a_run_finishing_within_its_budget_reports_its_own_exit_status_and_output`, `a_stream_longer_than_the_capture_limit_keeps_only_its_tail`, `capture_within`, `a_capture_thread_told_to_stop_returns_what_is_already_in_the_pipe`, `a_capture_thread_blocked_on_an_open_idle_pipe_stops_when_the_flag_is_set`, `a_capture_thread_stops_reading_when_its_drain_limit_has_passed`, `a_capture_thread_stops_within_the_drain_limit_while_a_straggler_keeps_writing`, `a_launcher_printing_more_than_the_limit_completes_with_bounded_text`, `a_timeout_of_duration_max_never_elapses_and_does_not_panic` | 14 | `launch.rs` tests | n/a | call `run_launcher_with_timeout`, `capture_tail` |

Rows add to 6 + 4 + 2 + 2 + 7 + 3 + 3 + 7 + 3 + 2 + 4 + 3 = 46 production items, and
6 + 7 + 1 + 1 + 1 + 7 + 14 = 37 tests.

**`kani_transcript.rs` (31 production items, 21 test items).**

| Old items | Count | New file | Visibility | Note |
| --- | --- | --- | --- | --- |
| `SUPPORTED_REPORT_VERSION`, `COVER_CATEGORY`, `UNWIND_CATEGORY`, `UNKNOWN_LOCATION`, `RawCheck`, `RawLocation`, `RawHarness`, `RawReport`, `RawMetadata`, `RawResults` | 10 | `kani/output/report.rs` | unchanged | private |
| `KaniReportRefusal`, `impl fmt::Display for KaniReportRefusal`, `impl std::error::Error for KaniReportRefusal`, `KaniHarnessStatus`, `KaniCheckStatus`, `KaniCheckClass`, `OtherCheckClass`, `impl OtherCheckClass`, `impl From<String> for KaniCheckClass`, `impl From<KaniCheckClass> for String`, `KaniCheckLocation`, `KaniCheckResult`, `impl TryFrom<RawCheck> for KaniCheckResult`, `KaniHarnessReport`, `impl TryFrom<RawHarness> for KaniHarnessReport`, `impl KaniHarnessReport` | 16 | `kani/output/report.rs` | unchanged | `pub` or `pub(crate)` as today; `KaniHarnessReport`'s fields are already `pub(crate)` |
| `PLAYBACK_HEADER`, `PLAYBACK_FENCE`, `PLAYBACK_ENTRY_POINT`, `PLAYBACK_COVER_MARKER` | 4 | `kani/output/playback.rs` | unchanged | private |
| `counterexample_playback` | 1 | `kani/output/playback.rs` | unchanged | `pub(crate)`; `classify.rs` calls it (interim) |
| Tests: `Capture`, `capture` (the `macro_rules!`), `classified`, `report`, `mutated` | 5 | `classify.rs` tests, `real_capture` module | n/a | the fixtures and their helpers (D-6) |
| Tests: `tc_027_real_kani_success_with_a_satisfied_cover_is_verified`, `tc_027_real_kani_failure_carries_the_assertion_playback_not_the_cover_one`, `tc_027_real_kani_unwinding_failure_is_inconclusive_not_falsified`, `tc_027_real_kani_a_run_with_no_successful_check_is_a_vacuous_proof`, `tc_027_real_kani_partly_satisfied_covers_are_cover_unsatisfied`, `tc_027_real_kani_success_without_a_cover_is_inconclusive`, `tc_027_the_console_banner_never_decides_the_verdict`, `tc_027_a_report_that_changed_shape_is_refused_not_classified`, `tc_027_a_report_without_exactly_one_harness_is_refused`, `tc_027_a_success_report_listing_a_failed_check_is_refused_never_verified`, `tc_027_real_kani_the_per_check_view_carries_id_class_location_and_status`, `tc_027_an_unknown_line_is_none_and_a_non_numeric_line_is_refused`, `tc_027_an_unnamed_check_category_is_a_property` | 13 | `classify.rs` tests, `real_capture` module | n/a | each calls `classify_kani_run` or `classified`, or uses `mutated` or `capture!`; the last two are parse tests that call `mutated`; include path `../../tests/fixtures/kani-report/` |
| Tests: `tc_027_a_class_spelled_cover_or_unwind_is_never_other`, `tc_027_the_per_check_view_has_one_serialized_wire_shape` | 2 | `output/report.rs` tests | n/a | report types only, no fixture and no `mutated` |
| Tests: `tc_027_playback_scanning_returns_the_property_block_and_stops_at_an_unterminated_fence` | 1 | `output/playback.rs` tests | n/a | calls `counterexample_playback` |

Rows add to 10 + 16 + 4 + 1 = 31 production items, and 5 + 13 + 2 + 1 = 21 tests.

**`kani_witness_join.rs` (19 production items, 15 test items).**

| Old items | Count | New file | Step | Visibility | Note |
| --- | --- | --- | --- | --- | --- |
| `HARNESS_MARKER`, `CHECK_MARKER`, `CheckKind`, `check_clause`, `unescaped_quote`, `bracketed`, `concrete_entries` | 7 | `kani/output/playback.rs` | 2f | unchanged | private to the scan |
| `Playback` | 1 | `kani/output/playback.rs` | 2f | pub(crate), fields pub(crate) | the decode half reads `harness`, `check_text` and `entries` |
| `select_assertion_block`, `read_block` | 2 | `kani/output/playback.rs` | 2f | pub(crate) | `decode_falsification` and one test call them |
| `DecodeFailure`, `impl DecodeFailure` | 2 | `kani/output/playback.rs` | 2f | `DecodeFailure` unchanged (`pub`); `new` pub(crate) | D-1; the decode half builds it too |
| `WitnessSchemaError`, `argument_types`, `byte_width`, `decode_values`, `boolean_comment` | 5 | flat `kani_witness_join.rs`; `replay/witness.rs` | 2g | unchanged | private to the decode |
| `decode_falsification`, `first_out_of_domain` | 2 | flat `kani_witness_join.rs`; `replay/witness.rs` | 2g | unchanged | `pub`, and `pub(crate)` for `spine_replay` |
| Tests: `argument`, `argument_types_preserve_order_and_type`, `argument_types_refuse_a_non_argument_binding`, `synthetic_transcript`, `two_value_transcript`, `decode_falsification_refuses_arity_and_width_mismatches`, `decode_falsification_decodes_a_matching_transcript`, `decode_falsification_refuses_a_transcript_whose_harness_symbol_disagrees`, `decode_falsification_names_each_value_by_its_binding_position`, `first_out_of_domain_is_inclusive_at_both_bounds`, `decode_falsification_refuses_a_disagreeing_comment_and_an_invalid_boolean_byte`, `decode_falsification_selects_exactly_one_assertion_block`, `decode_falsification_reads_a_multi_line_check_text`, `decode_falsification_decodes_negative_and_extreme_integers`, `decode_falsification_refuses_each_malformed_transcript_by_code` | 15 | flat `kani_witness_join.rs`; `replay/witness.rs` | 2g | n/a | all exercise `decode_falsification` |

Rows add to 7 + 1 + 2 + 2 + 5 + 2 = 19 production items, and 15 tests. The `pub use` line for
`decode_falsification` and `DecodeFailure` in `lib.rs` splits into two at 2f, one per module.

**Files moved whole.** Each file moves to its destination with every item, test items included,
assigned to that one file. The names are the full item list.

- `state_frame.rs` to `kani/generate/frame.rs` (36 production items, 2 tests; visibility unchanged).
  Production: `LOWERING_WORK_LIMIT`, `STATE_FRAME_LOWERING_TAGS`, `StateFrameRequest`,
  `StateFrameObligations`, `impl StateComparison`, `Side`, `UnsupportedFrameEffect`,
  `StateFrameRefusal`, `impl std::fmt::Display for StateFrameRefusal`,
  `impl std::error::Error for StateFrameRefusal`, `generate_state_frame_obligations`,
  `validate_request`, `is_identifier`, `short`, `lower_clause`, `Graph`, `impl Graph`, `Located`,
  `node_id`, `application`, `ClauseShape`, `Condition`, `impl ClauseShape`, `Observation`,
  `read_scope`, `integer_range`, `state_domains`, `render`, `RecordView`, `Abi`, `symbolic_state`,
  `assertion_message`, `HARNESS`, `Postcondition`, `postcondition_body`, `frame_body`. Tests: `scope`,
  `tc_025_an_operation_name_with_braces_cannot_break_an_assertion`.
- `bounded_kani_corpus.rs` to `kani/generate/corpus/bounded_kani_corpus.rs` (22 production items,
  31 tests; visibility unchanged). Production: `CORPUS_PROOF_GRAPH_SCHEMA`, `BoundedCorpusFamily`,
  `impl BoundedCorpusFamily`, `BoundedCorpusRequest`, `impl BoundedCorpusRequest`,
  `BoundedCorpusArtifacts`, `CorpusProofDependencyGraph`, `EmittedCorpusIdentities`,
  `impl EmittedCorpusIdentities`, `CaseIdentity`, `RequestIdentity`, `QueryKindIdentity`,
  `impl From<&BoundedCorpusRequest> for RequestIdentity`, `InputIdentity`,
  `impl From<&FiniteInput> for InputIdentity`, `impl CaseIdentity`, `BoundedCorpusCase`,
  `generate_bounded_kani_corpus_case`, `checked_method`, `render_arithmetic_oracle`,
  `render_graph_oracle`, `render_artifacts`. Tests: `InputEdit`, `Fixture`, `fixture`,
  `fixture_with`, `arithmetic`,
  `tc_023_generates_deterministic_complete_artifacts_for_every_supported_family`,
  `tc_023_declared_required_dependency_appears_in_the_graph`,
  `tc_023_missing_required_dependency_yields_incomplete_readiness`,
  `tc_023_duplicate_dependency_identity_is_refused_and_claims_no_identity`,
  `tc_023_assumed_dependency_kind_is_refused_and_claims_no_identity`,
  `tc_023_non_success_emits_no_partial_artifacts_or_boolean_claim`,
  `tc_023_unreachable_graph_request_classifies_as_false`,
  `tc_023_admitted_zero_arithmetic_is_a_proof_not_a_false_verdict`,
  `tc_023_provable_arithmetic_with_an_out_of_i64_range_operand_still_generates`,
  `tc_023_collection_oracle_evaluates_the_selected_ordered_population`, `case_name`, `emit`,
  `collection`, `tc_023_case_identity_is_independent_of_emission_order_and_run`,
  `tc_023_distinct_requests_get_distinct_names_paths_and_proof_symbols`,
  `tc_023_a_request_emitted_twice_is_refused_as_an_identity_collision`, `identity_of`,
  `assert_each_variation_changes_identity`, `checked`, `reach`,
  `tc_023_every_arithmetic_request_field_changes_the_identity`,
  `tc_023_every_graph_request_field_changes_the_identity`,
  `tc_023_every_collection_request_field_changes_the_identity`,
  `tc_023_every_input_and_profile_field_changes_the_identity`,
  `tc_023_identity_is_canonical_over_the_input_population_order`, `proof_symbol`.
- `bounded_kani_profile.rs` to `kani/generate/lower/bounded_kani_profile.rs` (3 production, 5
  tests). Production: `BoundedKaniProfile`, `impl BoundedKaniProfile`,
  `classify_bounded_kani_profile`. Tests: `profile`, `entry`, `superset_matrix`,
  `tc_023_census_reports_every_construct_including_ones_after_an_early_refusal`,
  `tc_023_missing_construct_is_rejected_with_kani_capability_missing`.
- `bounded_collections.rs` to `kani/generate/lower/bounded_collections.rs` (1 production, 2
  tests). Production: `prepare_bounded_collection_query`. Tests: `fixture`,
  `tc_023_collection_order_duplicates_and_bounds_remain_exact`.
- `definedness_arithmetic.rs` to `kani/generate/lower/definedness_arithmetic.rs` (1 production, 2
  tests). Production: `prepare_checked_arithmetic`. Tests:
  `tc_023_division_by_zero_remains_a_typed_refusal`,
  `tc_023_checked_domain_admits_exact_values_and_refuses_outside_results`.
- `finite_reference_graphs.rs` to `kani/generate/lower/finite_reference_graphs.rs` (1 production,
  no tests). Production: `prepare_finite_graph_reaches`.
- `kani_census.rs` to `kani/census.rs` (8 items): `ProofDependencyKind`, `ProofDependencyState`,
  `ProofReadiness`, `ProofDependencyRequest`, `ProofDependencyEdge`, `normalize_dependencies`,
  `dependency_readiness`, `dependency_site`.
- `kani_identity.rs` to `kani/identity.rs` (16 items): `ObligationKind`, `ObligationBinding`,
  `EmbeddedOracle`, `KaniObligationIdentity`, `KaniObligationHarness`, `ScalarObligationArgument`,
  `ScalarObligationIdentity`, `KaniScalarObligationHarness`, `impl KaniObligationIdentity`,
  `impl ScalarObligationIdentity`, `StateFrameHarness`, `StateComparison`, `StateFrameProperty`,
  `StateFieldDomain`, `StateFrameScope`, `StateFrameIdentity`.

**What a reviewer checks for 2f.** For each row, the item's text at its new path equals its text at
the old path, apart from `use` lines, paths and the visibility the row states
(`git diff --color-moved` shows it); the generated output of every existing test is unchanged and
`make test` passes with no test code under `tests/` edited, path-only updates to comments and to assertion or failure message strings aside (the root re-exports keep their names); the
six `mod.rs` files hold declarations only; none of `spec.rs`, `render.rs` or `terminal.rs` exists;
and each interim file carries a header naming the step that deletes it.

## Risks

- A rename-only step that also touches imports is large in line count. It is reviewable only if
  it carries no logic change, so each of 2c to 2g (and 2g-0) must be refused if it does. The
  definition moves of 2a, 2b and 2d-0 and the verbatim item moves of step 2f (the splits of
  `kani`, `kani_obligations`, `kani_execution`, `kani_transcript` and the scan half of
  `kani_witness_join`) are the only edits beyond renames and path fixes; they relocate items
  unchanged and carry no behaviour change, and each is refused if its generated output is not
  byte-identical. The one edit a 2f move may make to an item is the narrowest visibility
  widening its new file boundary requires, as the item map states per item.
- The layout test reads `use` lines and bare `crate::<Item>` paths in code, not the compiler's graph. A path in a macro, a
  `super::` import or an intra-doc link (comments are not read) would escape it. The measured edge list in this AD was made the same way
  and has the same blind spot.
- Step 4c needs a spec change and has no ticket. If it slips, 4e and 4f slip with it, because the
  V1 arm is the only contract family until then.
- Step 4f depends on a fact this AD could not verify: that QSL's exemplars can be moved onto the
  one public entry. The planner's record puts the arithmetic control on `generate_kani_bundle`. If
  the exemplars need something the contract family does not emit, 4f waits.
- Step 1a is blocked in CG itself: until QSL moves `quire-canonical` from its tag to
  `branch = "main"` (waiting on the owner), CG's lock would hold two entries and `make deny` fails.
- Batching with per-harness ceilings (FR-028) needs a rule for the batch's wall clock. This AD
  puts batching in `run/`. FR-028-AC-12 states the rule (Kani's own per-harness timeout T per
  member so a slow member keeps the others' results, an outer bound of N times T for a wedged
  backend, batches formed only from harnesses with equal options and equal T, FR-017-AC-21) as a
  default the owner may change; FR-017-AC-22 and FR-017-AC-23 own the per-harness split and the
  refusals.
- Typed node access depends on what IR exposes. If IR's decoder lands later than `core/ir`, the
  member names are spelled in CG once, which is still fewer than today's 70 call sites plus string
  tags, but not zero duplication with IR.
- Uncovered here, each already a finding in the CG design audit (SR-645): the error envelope (23
  public error-like types), the `unreachable!` arms on RT enums (37 at the audit, converted by #232;
  the remaining arms are tracked by IR-352), release-build truncation in
  `generate_routed`, and test conventions. A layout does not fix them; IR-348 carries the rest.
  The typed operator enum is in this AD because the audit assigned it to IR-344, though the
  ticket text does not list it.

### Decisions taken from the planner

The IR planner answered the questions this AD first raised. Each is now a decision, with its
source. The planner's answers and QSL's review of this PR are inputs the AD adopted after checking
what it could against the code; the checks are stated.

| Topic | Decision | Source |
| --- | --- | --- |
| Publication subsystem | Yes. FR-005 with TC-001, TC-002 and TC-007 moves to `publication`; no FR-005 exception. The move is migration step 7, and the spec.md registry note becomes obsolete when it lands. | IR planner, IR-344 |
| Corpus | CG's hand-built package lowerer is retired once QSL-353 (QSL's emission-to-admission corpus) lands, and the corpus rows are backed from QSL-emitted packages (IR-453). No requirement row is removed; until QSL-353 lands the rows and the corpus stay as they are. CG obtains those packages by calling QSL's facade on source, never from QSL test fixtures or copied package files. | IR planner, IR-344, IR-453, QSL-353 |
| Proof-dependency graph | `ProofDependencyGraph`, the V1 bundle's output type, retires at step 4f with `generate_kani_bundle`. Checked: FR-015's Inputs and Behavior name an optional declared proof-dependency census per obligation, folded into the harness identity, and FR-015-AC-22 and AC-25 are the criteria TC-005 traces. They stay verbatim as requirements on the V2 request input, and are backed by a V2 census input (step 4d), not by keeping the V1 graph type. The census request and readiness types are not the graph: they move to `kani/census.rs` (step 2b) and survive 4f, because the V2 census input needs them and the corpus imports them until 4g. Rule: if the V2 side does not back the requirements by 4f, their matrix rows go to planned or unbacked; nothing is deleted and no criterion is rewritten. | IR planner, IR-344 |
| `RUNTIME_REVISION` | Deleted, not spelled once. Checked: the constant is read by the three emitters' manifest templates (a `rev = "..."` on the runtime dependency) and by tests (failure messages and `manifest.contains`). Nothing else decides on it: no code compares it with the runtime CG itself builds against, and CG's own `Cargo.toml` names the runtime by branch with the lockfile recording the commit. The emitted manifest needs a runtime source, not a pin, so the template names it the way `Cargo.toml` does. The cost is that an emitted crate follows the runtime branch as CG does. I did not build an emitted crate to confirm. | IR planner, IR-344; QSL concurs; repository CLAUDE.md |
| PR 210 | Lands before the directory moves, only after this AD is approved, and with a local `make kani` transcript from its head. A precondition of migration step 2. | IR planner, IR-344 |
| V2 strategy criteria | FR-002, FR-004 and FR-008 to FR-013 over `CheckedPackageV2` are IR-364, IR team, ordered before step 6. | IR planner, IR-344, IR-364 |
| One public entry before deletion | Migration step 4e: the one public generator entry exists. A QSL-owned follow-up moves QSL's quire-integration exemplars, which call `generate_kani_bundle` today, onto it. Only then does step 4f delete `generate_kani_bundle`. | QSL review of this PR |
| Package source for tests | After QSL-353 the corpus builds packages through QSL's facade (`call_site(...).package`, or source plus `qsl_replay`), and the arithmetic control (step 4a) is quire-integration's existing test, with a CG-side copy only if the facade allows. CG copies no QSL fixture and no QSL-emitted package file. | QSL review of this PR |
| Terminal map dependency | Step 5's reader and the C-09 map (`kani/terminal.rs`) depend on QSL's `Inconclusive(cause)` types, merged in 02530e7 (QSL has ruled, relayed on IR-465, vacuity stays `Proved{0}`, no `NonZero`; the tool pin is gone, #551), and on the terminal value also taking the replay settlement; the FR-029 map and the IR-outcome map (FR-030, `DeclineCode::Std001`, QSL #634) are built. | QSL review of this PR |
| `ContentDigest` and the canonical encoding | CG depends on `quire-canonical` directly (`branch = "main"`, no `qsl-replay` re-export). `ContentDigest` is a CG type over its digest, with no `ByteDigest` wrapper except where a QSL API requires one. The obligation preimage is encoded by `quire-canonical` (RFC 8785), and `core::canonical` is the one place that calls it; the `serde_json` `deterministic_json` copies are deleted. QSL still pins it by tag, so step 1a waits on CG's lock resolving it to one entry. Cites AD-003, which is merged and not edited here. | QSL; relayed by the IR planner |

SuiteRegistry (SUR-001): moves to `spec/core/functional/suites.md` in step 7; ADR-0056 is not
amended (IR planner, IR-344). No question remains open in this AD except the missing ticket for the
FR-015 V2 contract input (step 4c).

### Not verified in this AD

- The arithmetic control was not run, and QSL's tests were not read. Which generator they route
  through, and whether the control's clause is a contract-family clause or a scalar claim, is the
  planner's statement on IR-344 and was not determined here.
- The 70-call count and the import edges are greps over `src/`, not the compiler's output, and no
  build or test was run for this AD. The crate-root import count (12 files) is a grep over
  `use crate::` blocks, not the compiler's output.
- PR 210 was read as a description and file list, not built, when this AD was first written. It
  has since merged: its report parse is `kani_transcript` and its runner changes are in
  `kani_execution`, and the step 2f item map reads them there.
- The step 2f item map was read from `src/` at `origin/main` after step 2e, by item name and by
  reading call sites; no build or test was run for it. The visibilities, the one-caller claims and
  the acyclic file order inside `kani/generate/` are the coder's to confirm by compiling, and a
  disagreement is fixed in this AD, not worked around in the code.
- Whether IR exposes a typed body-term decoder today was not checked; IR's layout AD is a separate
  ticket. The IR-347 CG lowering move is implemented here; IR export removal and its `BoundPackage`
  retirement remain separate work. The C-09 terminal cases were checked against QSL's merged ADR-011 T-13; the closed-set
  rule for `ReplayRefused` is relayed.
- The `quire-canonical` API names (`to_vec`, `Limits`, `sha256`), QSL's tag pin and the driver's
  one-copy gate are relayed and were not checked.

## IR-364 V2 strategy replacement gate

The strategy replacement consumes IR FR-038-AC-202 through AC-209's public clause-context
comparison accessor (IR-703, planned/unrun) over `CheckedPackageV2`, a checked clause id and an
authentic claim occurrence. Contract IR owns operand provenance, observation, identity and exact
i128 bounds. `strategy/bound` owns CG's narrower equal-domain i64 admission, constructive
population and census. `strategy/harness` owns campaign execution; `oracle` must add the
planned six-operator V2 comparison oracle for the selected claim or its typed refusal. The existing exact scalar generator
does not cover integer eq/ne. The selected claim is the admission unit even when a sibling
claim is refused; generated output remains atomic for the selected request. A real QSL-produced,
IR-admitted ConfigVersion direct Post/Pre comparison is required before claiming this code path.

The same code change that introduces the V2 public strategy entry removes the V1
`BoundPackage`/`ClauseRef` strategy entry and its V1-only harness reader. This gate does not delete
V1 readers in `evidence`, `oracle/boolean_v1.rs`, `oracle/bound_v1.rs` or Kani before their own
replacement gates. The caller-constraint `strategy/campaign.rs` is unaffected.
