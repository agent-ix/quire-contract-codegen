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
grepping `src/`. Counts exclude `#[cfg(test)]` modules unless said so.

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
  inside `kani_transcript`'s own `#[cfg(test)]` module (`src/kani_transcript.rs:261`). No
  strongly connected component exists among the non-test `use crate::` edges. The structure that
  produces cycles is still there, for two reasons:
  - 12 files import at least one item through the crate root (`use crate::{..., OracleRequest, ...}`
    beside module paths, and `use crate::OperationClaim`), at the time of writing, measured after
    step 2c (an earlier draft of this AD said 17). The 12 are `bound`, `bound_coverage`,
    `bounded_kani_corpus`, `harness`, `kani`, `kani_obligations`, `routed_generation`,
    `state_frame`, `strategy`, `bound_strategy/census`, `bound_strategy/generation` and
    `bound_strategy/population`; the count includes `#[cfg(test)]` modules, and six further files
    use `use crate::{...}` with module paths only. So the real edge is hidden behind `lib.rs`'s
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
- **A tool digest exists.** `ReplayInputs::backend_manifest` is "the digest of the tool manifest of
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
  oracle/                     FR-014, FR-018, FR-021
    claim.rs                  ClaimMap, ClaimDisposition, OracleGenerationError (was generation)
    scalar/                   was exact_scalar, split along derivation, lowering and rendering
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
      spec.rs                 HarnessSpec
      render.rs               the one renderer: the only code that emits Kani attributes and macros
      negotiate.rs            negotiate_kani_obligations, the one public generation entry
      scalar.rs  precondition.rs  contract.rs  frame.rs   family lowerers; each returns a HarnessSpec
      lower/                  the Kani family lowerings CG takes over from IR, and the profile admission
      corpus/                 corpus family; renders through render.rs
    output/                   the one reader of Kani output
      report.rs               typed report parse (--export-json)
      playback.rs             typed extraction of the concrete-playback block
    classify.rs               KaniRunOutcome, KaniInconclusiveReason, vacuity rule
    run/                      launch, capture, timeout, execute_kani_obligation
    terminal.rs               the terminal-value maps (FR-029, FR-030) and the public C-09 entry the
                              driver calls, with the replay-outcome input type it defines
  replay/                     FR-016, FR-024
    witness.rs                types extracted playback entries against persisted bindings
    function.rs               was spine_replay
    frame.rs                  was frame_replay
  routed/                     FR-019, FR-022, FR-026
    capability.rs             was capability
    generate.rs               was routed_generation
    adapter.rs                the adapter trait and its one Kani implementation
  publication/                FR-005
    publish.rs                write_bundle_atomic, published identity
```

`kani/terminal.rs` is added by the parked Kani PR 210, which is not at this base; the layout
reserves its place. Directory names equal the registry's subsystem names, which makes ADR-0056
rule 3 (one module maps to exactly one subsystem) true by construction.

### Module-to-subsystem map

Every module at this base, with its target. "Retire" means deleted by the step named under
Migration order, not moved.

| Module today | Subsystem | Target | Fate |
| --- | --- | --- | --- |
| `lib` | core | `lib.rs` | stays; re-exports by explicit list, no logic. The one `pub mod bound_strategy` path leaves (step 2e); callers use the re-exported names |
| `oracle` (shared parts: `Artifact`, diagnostics, naming, `RUNTIME_REVISION`) | core | `core/artifact.rs`, `core/diagnostic.rs`, `core/naming.rs`, `core/profile.rs` | split; `RUNTIME_REVISION` deleted |
| `oracle` (naming helpers: `bounded_readable_component`, `readable_name_component`, `upper_camel`, `unique_names`, `unique_pair`, `oracle_symbol`, `reference_identifier`, and the private `rust_component` and `observation_name` they use) | core | `core/naming.rs` | split (step 2d-0). `dependency_parameters` and `typed_dependency_parameters` are not naming: they run the V1 expression analysis, so they stay with `oracle/boolean_v1.rs` |
| `oracle` (`MAX_GENERATED_SOURCE_BYTES`) | core | `core/artifact.rs` | split (step 2d-0): the one cap on a generated source, kept beside the bundle's size limits. Every user (the oracle, strategy, harness and Kani generators) sits above `core`, so rule 4 holds |
| `oracle` (`SourceProbe`, `SourceRegion`) | core | `core/source_map.rs` | split |
| `oracle` (V1: `generate_boolean_oracle`, `analyze_node`, `render_node`) | oracle | `oracle/boolean_v1.rs` | moved, then retired with V1 |
| `publication` (`ArtifactBundle`, limits, `PublicationDiagnostic`, `PublicationErrorCode`, `PublicationDestinationState`) | core | `core/artifact.rs` | split (step 2a); the destination state is a field of the diagnostic, so it moves with it |
| `publication` (writer, published identity) | publication | `publication/publish.rs` | split |
| `generation` | oracle | `oracle/claim.rs` | moved |
| `exact_scalar` | oracle | `oracle/scalar/`; walkers to `core/ir/` | split |
| `composite_equality` | oracle | `oracle/equality/` | moved |
| `exact_function` | oracle | `oracle/function/` | moved |
| `bound` | oracle | `oracle/bound_v1.rs` | moved; V1 input, so `evidence` and `strategy` import it downward |
| `harness` | strategy | `strategy/harness.rs` | moved; V1 input |
| `strategy` | strategy | `strategy/campaign.rs` | moved; no V1 input |
| `bound_strategy/*` (5 files) | strategy | `strategy/bound/` | moved; V1 input |
| `vacuity` | evidence | `evidence/vacuity.rs` | moved |
| `bound_coverage` | evidence | `evidence/bound_coverage.rs` | moved; V1 input |
| `kani` (types, `adapter_options`, `i64_literal`, `readable_component`) | kani | `kani/abi.rs` | split |
| `kani` (`ProofDependencyEdge`, `Kind`, `State`, `Request`, `ProofReadiness`, `normalize_dependencies`, `dependency_readiness`) | kani | `kani/census.rs` | split (step 2b); the FR-015 census input and the corpus use them |
| `kani` (`generate_kani_bundle`, `KaniArtifactBundle`, `ProofDependencyGraph`, bundle validation, except `validate_dependencies`) | kani | none | retired (step 4f); `validate_dependencies` stays until the corpus stops calling it (step 4g) |
| `kani` (`deterministic_json`, `artifact`) | core | `core/canonical.rs` | `deterministic_json` deleted; `Artifact::new` used directly |
| `kani_obligations` (harness and identity records, `ObligationKind`, `ObligationBinding`) | kani | `kani/identity.rs` | split (step 2b) |
| `kani_obligations` (the rest) | kani | `kani/generate/{negotiate,scalar,precondition,contract}.rs` | split |
| `state_frame` | kani | `kani/generate/frame.rs`; `StateFrameHarness`, `StateFrameProperty` to `kani/identity.rs` | moved; one entry with `negotiate` |
| `bounded_kani_corpus` | kani | `kani/generate/corpus/` | moved; its package lowerer retired after QSL-353 (step 4g) |
| `bounded_kani_profile`, `bounded_collections`, `definedness_arithmetic`, `finite_reference_graphs` | kani | `kani/generate/lower/` | moved; today thin callers of IR's Kani family lowerings. The planner's decision, relayed for IR-347 and not verified here: those lowerings (`lower_checked_arithmetic`, `lower_query`, `lower_reaches`, IR `src/kani/mod.rs:19`) move out of IR into CG and IR deletes them afterwards, so CG takes them over and the forwarders go. The AD fixes the destination only; IR-347 schedules it |
| `kani_execution` | kani | `kani/run/`, `kani/classify.rs` | split |
| `kani_transcript` | kani | `kani/output/` | moved; PR 210 replaces its parse |
| `kani_witness_join` | replay | `replay/witness.rs`; text scanning to `kani/output/playback.rs` | split |
| `spine_replay` | replay | `replay/function.rs` | moved; `backend_manifest` deleted at step 5 |
| `frame_replay` | replay | `replay/frame.rs` | moved |
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
  trait is defined in `routed/adapter.rs` with the Kani implementation beside it. The driver
  (`quire-driver`, outside this crate) pairs a Kani outcome with a replay result, as QSL's merged
  ADR-011 T-13 says; no module of this crate does.
- Imports use a module path (`use crate::core::artifact::Artifact`), never an item re-exported
  from the crate root. `lib.rs` is the only file that names a root re-export.
- Inside `kani/` the order is `abi`, `census`, `identity`, then `generate` and `output`, then
  `classify`, `run`, `terminal`. A file imports only earlier names in that order. `output` imports
  `identity` and `abi` and nothing from `generate`, `classify` or `run`. `run` imports `identity`,
  `output` and `classify`, not `generate`: the harness and identity record types it needs live in
  `kani/identity.rs` (step 2b), so a harness is run from its identity and its source text.
- A `#[cfg(test)]` module obeys the same direction as the file it sits in. A test that needs a
  generator as a fixture lives in `tests/it/`, not in the runner's file. The edge at
  `kani_transcript.rs:261` moves there.
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
  the subject call, the covers and the assertions, in that order.
- **Cover rule (IR-464).** A non-empty cover list is not a non-vacuity check. The rule is where the
  covers go: `render.rs` emits every cover after all assumptions and after the subject call, never
  before, so a cover is reachable only when the assumptions are satisfiable, and the cover states
  the property's own reachability (for the scalar family, that the oracle's `Completed` branch is
  reached; for the precondition family, that the precondition holds). The constructor refuses an
  empty cover list, and the order is fixed by the renderer, not by the family. A frame harness's
  `kani::cover!(true, ...)` after the subject call satisfies the rule, because it is placed after
  the assumptions. Test (L-4): a harness whose assumptions are unsatisfiable does not classify
  `Verified` under real Kani. The corpus today has no symbolic input and no cover; see step 4g.
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
| `bounded_kani_corpus` renderer | Keeps its own template until QSL-353 lands (an interim exception, no cover); then its hand-built package lowerer is retired and its cases render through `render.rs` (step 4g). It returns no `KaniOutcome` at generation time: a verdict comes only from a run. |

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
| Run classification and vacuity | `kani/classify.rs` | typed report only; no text |
| FR-029 map: `KaniRunOutcome` to a terminal value | `kani/terminal.rs` | typed run outcome only |
| C-09 map: IR's `KaniOutcome` with the replay outcome to `TerminalValue` | `kani/terminal.rs`, a public entry | typed outcome and the replay-outcome type it defines; total over the pair (step 5) |
| Pairing the two inputs | the driver, outside this crate | runs the replay and calls the C-09 entry with both |
| Building the replay request; converting CG errors and QSL's result into the replay outcome | `replay/` | builds the request the driver passes to `qsl_replay::replay`, and converts results into `kani/terminal.rs`'s input type |

Who calls `qsl_replay::replay`. QSL's merged ADR-011 E9 and T-13 say the driver calls it with the
request CG's adapter builds. Today CG's `replay/` modules (`spine_replay`, `frame_replay`) build the
request and also call `replay` themselves (AD-001's Replay view). That is a deviation from T-13,
recorded here: the layout keeps `replay/` calling `replay` until the driver takes the call, at which
point `replay/` only builds the request and converts results. This AD does not schedule that move.
| Witness decode | `replay/witness.rs` | typed playback entries only |

Today `kani_witness_join` also scans Kani's text (`check_clause`, `select_assertion_block`,
`read_block`, `concrete_entries`), which breaks AD-001's "Kani's printed wording is read in exactly one module".
The scanning moves to `kani/output/playback.rs`, and `replay/witness.rs` receives
`(name, rendered value)` entries, checks them against the persisted argument bindings and builds
`qsl_replay::WitnessValue`. Replay imports `kani`; `kani` never imports `replay`. That gives
one report parse and one playback parse. The removal of IR's witness-accessor re-parse (IR-277) is
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
  once step 7 lands the registry rows by directory, it reads the registry instead.
- L-2. The import graph is acyclic and follows the dependency direction above, `#[cfg(test)]`
  modules included, and no file imports an item through the crate root. Test: a layout test that
  reads `use crate::` lines. It lands at the end of step 2, after the edge-removal steps, so it can
  pass when it lands.
- L-3. `#[kani::proof]`, `#[kani::proof_for_contract]`, `kani::requires`, `kani::ensures`,
  `kani::any`, `kani::assume` and `kani::cover!` occur in string literals of non-test source in
  `kani/generate/render.rs` only. Test: a literal scan that ignores comments. It lands after 4f and
  4g. Until then the interim exceptions are the V1 arm and `generate_kani_bundle` (their own
  templates until 4f) and the corpus template (until 4g); the scan lists them by file and each
  entry is removed with its step.
- L-4. A `HarnessSpec` has at least one cover, and `render.rs` places every cover after all
  assumptions and the subject call. Test: the constructor's refusal, and a real-Kani test that a
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
  outside that directory.
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
  after the last V1 reader is replaced nowhere.

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
   `reference_identifier`, and the private `rust_component` and `observation_name` they use) and
   `MAX_GENERATED_SOURCE_BYTES` into `core/artifact.rs`, and points every importer at the new
   module path. Without it `core/naming.rs` is never created, because 2d moves `oracle.rs` whole,
   and L-1 cannot pass. It carries no behaviour change: the items move unchanged and the
   generated output is byte-identical. 2c to 2g are `git mv` plus path fixes, imports by module
   path, no logic change, one PR per subsystem in leaf order: 2c `core`, 2d `oracle` (with `bound`
   landing as `oracle/bound_v1.rs`), 2e `evidence` and `strategy` (the one `pub mod
   bound_strategy` path leaves here; its callers in `tests/it/` and any item reached only by
   that path are re-exported by name or made private in the same PR), 2f `kani` (including the
   `run`, `output`, `classify` split of `kani_execution` and `kani_transcript`, which is a
   verbatim item move: items move unchanged between files, with no logic edit and
   byte-identical output; and the test back-edges), 2g `replay`, `routed`, `publication`.
   The crate-root imports (L-2) are rewritten by the steps that move the files: each of 2d to
   2g rewrites every root-path import in the files it moves to a module path, and 2g-0, a sweep
   PR before the layout test lands, rewrites any that remain (`use crate::{..., Item}` and
   `use crate::Item`, in `#[cfg(test)]` modules too; `lib.rs` keeps its re-export list). 2g-0
   changes imports only. The layout test (L-1, L-2) lands with 2g.
3. **Typed node access.** `core/ir` with the operator enum, then `state_frame`, `exact_scalar`,
   `composite_equality` and `exact_function` onto it, one PR each. L-8 lands with the last.
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
     because it returns the V1 bundle's error type. Merged only after 4e has landed, the QSL follow-up has moved the exemplars and the control
     passes on the V2 contract arm (4c), and the V2 census input exists (4d). FR-015's census and
     FR-015-AC-22 and AC-25 stay verbatim; if the V2 side does not back them by this step, their
     matrix rows go to planned or unbacked, and nothing is deleted or rewritten.
   - 4g: the corpus. Until QSL-353 lands the corpus stays as it is, with its own template: it has
     no cover and no symbolic input (a ground `assert!` over literals), so `HarnessSpec`, which
     refuses an empty cover list, cannot render it. It is therefore an interim exception to L-3
     and outside L-4, and IR-464 stays open for it. When QSL-353 lands, the hand-built package
     lowerer is retired, the rows are backed from QSL-emitted packages built through the facade,
     and a corpus case with symbolic input is rendered through `render.rs` under the cover rule;
     a case with no symbolic input is not rendered as a proof. L-3 lands after 4f and 4g. The Kani family lowerings
     move in from IR when IR-347 schedules it (relayed).
5. **One output reader and the terminal map.** The playback scanning moves from `kani_witness_join`
   to `kani/output/playback.rs`; `replay/witness.rs` takes typed entries. Batching follows, in
   `run/` only. `ReplayInputs::backend_manifest` and the manifest members `spine_replay` builds
   from it are deleted here, with the tool pin QSL-351 drops. Step 5's reader and the C-09 map
   (`kani/terminal.rs`) produce `TerminalValue` (the FR-029 map from `KaniRunOutcome`, and the
   C-09 map from IR's `KaniOutcome` with the replay outcome), so both depend on QSL-351
   (`Inconclusive(cause)`, a `NonZero` `Proved`, the tool pin gone) and on the terminal value also
   taking the replay settlement. Neither merges before QSL-351.
   - The map follows QSL's merged ADR-013 C-09 and ADR-011 T-13 (QSL #550, QSL-354; checked at QSL
     `origin/main`: T-13 says the driver `quire-driver` owns the S6b run, the E9 replay
     (`qsl_replay::replay`) and the FR-331 terminal record, CG owns the C-09 map and settles
     dispositions, and parity is settled inside `replay`). It adds no behaviour claim of this AD;
     FR-029 is amended by its own ticket, and QSL-351 adds the two causes. The map is total over
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
     `SpineReplayError::{UnboundArgument, FieldDelimiter, Transcript, WrongArm}`, the envelope
     failure, and a Kani playback outside the harness proof bound, which CG checks before building
     the envelope. AD-001's Failure view keeps each a distinct typed state before the map.
   - QSL's answer on CG-side refusals (relayed; keyed on when the refusal happens, and consistent
     with the when-rule in AD-003's R-Q1 as in the follow-up PR 216, which is not edited here):
     a refusal before Kani runs, when the obligation's own input is refused (IR's `Refused`,
     `InvalidInput` and `IncompleteInput` outcomes; nothing was proved), is
     `Declined(ProofRefusalCause)`, mapped by the IR-outcome map. A caller-lock refusal is not
     that case: in CG it is `DependencyLockError`, reached through `ReplayPackageError::Dependencies`
     (interface-001; FR-016-AC-16 to AC-19), which happens in replay setup AFTER Kani refuted, so
     it is the after-Kani case and not a `Declined` candidate. A replay setup refused on data
     after a refuted Kani run (`CallSiteRefusal` `Compile` or `UnknownFunction`,
     `InvalidFunction`, `Name`, a dependency-selection refusal, `DependencyLockError`) is
     `Inconclusive(ReplayRefused)` with a QSL code catalogued in QSL-352, which lands with this
     step. A decode failure (`DecodeFailure`, `EvidenceFailureCause::Decode`) is not in that list:
     it is a CG defect, a playback that does not type against the bindings CG persisted, and maps
     to `Failed` (AD-003, link 7). Faults stay `Failed`. Until QSL-352's codes exist, these
     refusals map to `Failed` as the interim.
   - Layering. The C-09 map is a public entry in `kani/terminal.rs` that the driver calls; the
     driver runs the obligation and the replay and pairs the two, as QSL's merged T-13 says. Its
     first input is IR's `KaniOutcome` (ADR-013 C-09's `KaniOutcomeKind`); the FR-029 map from
     CG's `KaniRunOutcome` is the other entry. Its second input is a replay-outcome type that
     `kani/terminal.rs` defines from `qsl-replay` types (a QSL result, a QSL `ReplayRefusal`, a
     QSL fault) plus two CG-raised variants, so `kani` imports nothing from `replay`: a CG-origin
     defect, and a setup refusal on data (the after-Kani case above) that carries a QSL code from
     QSL-352. The setup-refusal variant needs QSL-352's codes to be constructible by CG, so it
     is an open item on QSL-352: until those codes exist the variant cannot be built, and `replay/`
     converts those refusals to the CG-defect variant (the `Failed` interim).
     `replay/` imports `kani` (downward) and owns the conversion: it turns its own errors
     (`SpineReplayError`, the envelope failure, the out-of-bound playback) and QSL's result into
     that type, so a CG-origin failure reaches the map as the CG-defect variant and becomes
     `Failed` there. `routed/` does not pair.
6. **V1 readers.** Each of `harness`, `strategy/bound`, `evidence/bound_coverage`,
   `oracle/bound_v1.rs` and `oracle/boolean_v1.rs` is replaced and deleted with its V2 criteria.
   IR-364 (the V2 strategy chain: FR-002, FR-004 and FR-008 to FR-013 over `CheckedPackageV2`) is
   owned by the IR team and is ordered before this step; no V1 reader is deleted until its
   criteria exist.
7. **Spec follows the code.** One spec PR: the registry rows by directory; FR-005 and
   TC-001, TC-002, TC-007 to `spec/publication/`; SUR-001 to `core/functional/`; `interface-001`
   and `tests.md` fixed. `git mv`, ids unchanged. When this step lands, the registry note in
   `spec/spec.md` that records the FR-005 exception becomes obsolete and is deleted in the same
   PR, as is the SUR-001 note. A separate follow-up, not edited here: AD-001's Current state and
   Risks are stale against this AD (it lists the corpus and profile modules as V1, and says Kani
   publishes no machine-readable verdict while the `--export-json` decision says otherwise). The
   spec PR of this step is the place to fix them.

## Risks

- A rename-only step that also touches imports is large in line count. It is reviewable only if
  it carries no logic change, so each of 2c to 2g (and 2g-0) must be refused if it does. The
  definition moves of 2a, 2b and 2d-0 and the verbatim item move of the `kani_execution` and
  `kani_transcript` split in 2f are the only edits beyond renames and path fixes; they relocate
  items unchanged and carry no behaviour change, and each is refused if its generated output is
  not byte-identical.
- The layout test reads `use` lines, not the compiler's graph. A path in a macro or a
  `super::` import would escape it. The measured edge list in this AD was made the same way
  and has the same blind spot.
- Step 4c needs a spec change and has no ticket. If it slips, 4e and 4f slip with it, because the
  V1 arm is the only contract family until then.
- Step 4f depends on a fact this AD could not verify: that QSL's exemplars can be moved onto the
  one public entry. The planner's record puts the arithmetic control on `generate_kani_bundle`. If
  the exemplars need something the contract family does not emit, 4f waits.
- Step 1a is blocked in CG itself: until QSL moves `quire-canonical` from its tag to
  `branch = "main"` (waiting on the owner), CG's lock would hold two entries and `make deny` fails.
- Batching with per-harness ceilings (FR-028) needs a rule for the batch's wall clock. This AD
  puts batching in `run/` and leaves the rule to FR-017 and IR-277.
- Typed node access depends on what IR exposes. If IR's decoder lands later than `core/ir`, the
  member names are spelled in CG once, which is still fewer than today's 70 call sites plus string
  tags, but not zero duplication with IR.
- Uncovered here, each already a finding in the CG design audit (SR-645): the error envelope (23
  public error-like types), the 37 `unreachable!` arms on RT enums, release-build truncation in
  `generate_routed`, and test conventions. A layout does not fix them; IR-348 carries them.
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
| Terminal map dependency | Step 5's reader and the C-09 map (`kani/terminal.rs`) depend on QSL-351 (`Inconclusive(cause)`, a `NonZero` `Proved`, the tool pin gone) and on the terminal value also taking the replay settlement. | QSL review of this PR |
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
- PR 210 was read as a description and file list, not built. It is not at this base.
- Whether IR exposes a typed body-term decoder today was not checked; IR's layout AD is a separate
  ticket. The IR-347 lowering move and IR's `BoundPackage` retirement are relayed and not
  verified. The C-09 terminal cases were checked against QSL's merged ADR-011 T-13; the closed-set
  rule for `ReplayRefused` is relayed.
- The `quire-canonical` API names (`to_vec`, `Limits`, `sha256`), QSL's tag pin and the driver's
  one-copy gate are relayed and were not checked.
