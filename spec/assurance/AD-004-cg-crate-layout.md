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
AD-001's, and the replay seam and evidence chain are the seam descriptions of IR-324.

## System Boundary

In scope: every file under `src/`, the import edges between them, the public entry points that
`lib.rs` re-exports, and the helper, identity, node-access and version-profile code the modules
share. Also in scope are the two layout exceptions recorded in `spec/spec.md` (FR-005 and the
SuiteRegistry file), which the Subsystem Registry defers to this AD and which this AD settles.

Out of scope: behaviour. Every requirement keeps its id and its criteria. No requirement id is
minted here; the labels `L-1` and onward are local to this document, as the labels in the other
ADs are. Also out of scope: IR's and RT's own layouts, QSL's types, the error envelope, the
`unreachable!` arms on RT enums and the test conventions. They are listed under Risks as
uncovered.

## Views

The layout is described as what is there now, the target tree, the map from every module to its
subsystem, the dependency direction, the one Kani generator, the Kani run and output edge, the one
input model, the shared core, and the two recorded exceptions.

### Current state (measured)

Measured on this repository after the subsystem restructure (PR 213), by reading `src/lib.rs` and
grepping `src/`. Counts exclude `#[cfg(test)]` modules unless said so.

- `src/` holds 26,687 lines in 27 modules, all flat. `lib.rs` declares them, and only
  `bound_strategy` is a directory. The largest are `exact_scalar` (4170 lines), `kani_obligations`
  (2697), `composite_equality` (1702), `kani_execution` (1492), `bounded_kani_corpus` (1480),
  `exact_function` (1384), `oracle` (1306) and `state_frame` (1156).
- The directories do not match the spec subsystems. The registry names seven subsystems and
  `spec/` has a directory for each, but `src/` has none of them.
- **Import cycles.** The ticket names two. Re-measured, neither is a cycle in non-test code.
  `kani_execution` imports `kani_obligations`, `kani_transcript` and `state_frame`, and none of
  those imports it back; `kani_obligations` imports no `kani_execution` item. The one back-edge in
  the pair `kani_execution` and `kani_transcript` is `use crate::kani_execution::{classify_kani_run, ...}`
  inside `kani_transcript`'s own `#[cfg(test)]` module (`src/kani_transcript.rs:261`). No
  strongly connected component exists among the non-test `use crate::` edges. The structure that
  produces cycles is still there, for two reasons:
  - 17 files import through the crate root (`use crate::{Artifact, GenerationErrorCode, ...}` and
    `use crate::SourceProbe`), so the real edge is hidden behind `lib.rs`'s `pub use` list and an
    import of a root item can close a loop with no module name in sight.
  - Utility modules sit in the wrong place. The V1 `kani` module is a helper library for the V2
    path (`kani_obligations`, `state_frame`, `kani_witness_join` and `bounded_kani_corpus` import
    `adapter_options`, `i64_literal`, `readable_component`, `KaniBindingRole`, `KaniSolver` and
    `deterministic_json` from it), and `exact_scalar` is a walker library (`kani_obligations` and
    `state_frame` import `bound_members`, `aggregate_members`, `operand_ranges`, `literal` and the
    `*_MEMBERS` tables from it).
- **Harness generators.** Five renderers emit Kani source, not three:
  1. `src/kani.rs` `generate_kani_bundle` (`:314`; template at `:814`), a `proof_for_contract`
     harness. It emits no `kani::cover!`. Nothing in `src/` calls it; it is public, and
     `tests/it/kani_generation.rs` drives it.
  2. `src/kani_obligations.rs` `render_scalar` (`:2016`, template `:2057`, cover `:2082`), the
     precondition harness (`:2187`, cover `:2190`) and the contract harness (`:2285`, cover
     `:2288`). One file, three templates.
  3. `src/state_frame.rs` (`:1075` comparison harness, `:1099` assertion harness), both with a
     cover, reached by `generate_state_frame_obligations`, a second public entry beside
     `negotiate_kani_obligations`.
  4. `src/bounded_kani_corpus.rs` (`:588`), a plain `#[kani::proof]`. It has no `src/` caller.
  The cites in IR-344 (`kani.rs:314`, `kani_obligations.rs:2016`, `state_frame.rs:1075` and `:1099`)
  are correct. The corpus is the fourth; IR-464 already names the first and fourth as the two
  that emit no cover.
- **Input models.** `negotiate_kani_obligations` takes `ObligationItem::BoundClause` (V1,
  `BoundPackage`) and `ScalarClaim` (V2) and refuses a mix. `oracle`, `harness`, `strategy`,
  `bound`, `bound_strategy`, `bound_coverage` and the first renderer read V1.
- **Helpers.** `fn artifact(path, contents)` is defined 9 times and each is a one-line
  call to `Artifact::new` (`oracle`, `kani`, `harness`, `strategy`, `bound_strategy/generation`,
  `kani_obligations`, `exact_scalar`, `composite_equality`, `exact_function`).
  `deterministic_json` is defined twice with two error types (`oracle.rs:1111`, `kani.rs:1045`).
  Hex encoding is written twice (`composite_equality.rs:1293` and a `{:x}` format of QSL's
  `ByteDigest` in the corpus). No `sha2` call exists in `src/`; digests go through
  `qsl_replay::ByteDigest` (corpus, `spine_replay`). Two test helpers named `sha256_hex` are
  identical.
- **Raw IR walking.** Four modules walk checked-package body terms by hand over
  `serde_json::Value`: `exact_scalar` (23 `.get("..")` calls), `state_frame` (29),
  `composite_equality` (9) and `exact_function` (2), 63 in all, plus `.as_str()`, `.as_u64()` and
  `.as_object()` calls on the results. `CheckedNodeTag` is used for tag comparison, and
  `CheckedNodeKind` is used nowhere in `src/`. Operator identity is a string kept in three tables
  (`exact_scalar`, `kani_obligations`, `state_frame`).
- **Identity types.** `module_symbol` and `harness_symbol` are `pub String` on three
  identity structs (`kani_obligations.rs:478` and `:535`, `state_frame.rs:218`), and
  `generate_routed` pairs harnesses to records by that string.
- **Version profile.** The emitted oracle crate's `Cargo.toml` template, with its runtime dependency
  spelling and `RUNTIME_REVISION` (`oracle.rs:13`), is written out in three emitters
  (`exact_scalar.rs:2817`, `composite_equality.rs:1573`, `exact_function.rs:1378`). Contract
  spellings (`quire.backend-provider/v1`, `quire.capability-kind/v1`, the bundle, proof-graph,
  corpus and coverage schema ids) are constants in six modules, and the request contract
  spellings are literals in `spine_replay.rs`.

### Target directory tree

```text
src/
  lib.rs                      declares directories, re-exports the public API, holds no logic
  core/                       leaf: depends on nothing else in the crate
    artifact.rs               Artifact, ArtifactBundle and its size limits (no I/O)
    canonical.rs              deterministic JSON, the one content digest, hex encoding
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
  strategy/                   FR-002, FR-008 to FR-013
    harness.rs                tri-state harness (was harness)
    campaign.rs               enum and i64 campaigns (was strategy)
    bound/                    was bound and bound_strategy; V1 input, see One input model
  evidence/                   FR-004
    vacuity.rs                LLVM coverage parse and clause classification
    bound_coverage.rs         V1 input until rebased
  kani/                       FR-015, FR-017, FR-025, FR-028 to FR-030
    abi.rs                    binding roles, primitive types, integer bounds, KaniSolver, options
    identity.rs               one harness identity record (ceilings, options, bounds, symbols)
    generate/                 the one generator
      spec.rs                 HarnessSpec
      render.rs               the one renderer: the only text that holds kani::proof or kani::cover!
      negotiate.rs            negotiate_kani_obligations, the one public generation entry
      scalar.rs  precondition.rs  contract.rs  frame.rs   family lowerers; each returns a HarnessSpec
      corpus/                 corpus family; renders through render.rs (see Decisions taken from the planner)
    output/                   the one reader of Kani output
      report.rs               typed report parse (--export-json)
      playback.rs             typed extraction of the concrete-playback block
    classify.rs               KaniRunOutcome, KaniInconclusiveReason, vacuity rule
    run/                      launch, capture, timeout, execute_kani_obligation
    terminal.rs               KaniRunOutcome and KaniOutcome to QSL TerminalValue (FR-029, FR-030)
  replay/                     FR-016, FR-024
    witness.rs                types extracted playback entries against persisted bindings
    function.rs               was spine_replay
    frame.rs                  was frame_replay
  routed/                     FR-019, FR-022, FR-026
    capability.rs             was capability
    generate.rs               was routed_generation
    adapter.rs                the adapter trait and its one Kani implementation
  publication/                FR-005
    publish.rs                write_bundle_atomic, destination state, publication diagnostics
```

`kani/terminal.rs` is added by the parked Kani PR 210, which is not at this base; the layout
reserves its place. Directory names equal the registry's subsystem names, which makes ADR-0056
rule 3 (one module maps to exactly one subsystem) true by construction.

### Module-to-subsystem map

Every module at this base, with its target. "Retire" means deleted by the step named under
Migration order, not moved.

| Module today | Subsystem | Target | Fate |
| --- | --- | --- | --- |
| `lib` | core | `lib.rs` | stays; re-exports by explicit list, no logic |
| `oracle` (shared parts: `Artifact`, diagnostics, `RUNTIME_REVISION`, naming, source regions) | core | `core/artifact.rs`, `core/diagnostic.rs`, `core/naming.rs`, `core/profile.rs` | split |
| `oracle` (V1: `generate_boolean_oracle`, `analyze_node`, `render_node`) | oracle | `oracle/boolean_v1.rs` | moved, then retired with V1 |
| `publication` | publication | `publication/publish.rs`; bundle type and limits to `core/artifact.rs` | split |
| `generation` | oracle | `oracle/claim.rs` | moved |
| `exact_scalar` | oracle | `oracle/scalar/`; walkers to `core/ir/` | split |
| `composite_equality` | oracle | `oracle/equality/` | moved |
| `exact_function` | oracle | `oracle/function/` | moved |
| `harness` | strategy | `strategy/harness.rs` | moved; V1 input |
| `strategy` | strategy | `strategy/campaign.rs` | moved |
| `bound` | strategy | `strategy/bound/` | moved; V1 input |
| `bound_strategy/*` (5 files) | strategy | `strategy/bound/` | moved; V1 input |
| `vacuity` | evidence | `evidence/vacuity.rs` | moved |
| `bound_coverage` | evidence | `evidence/bound_coverage.rs` | moved; V1 input |
| `kani` (types, `adapter_options`, `i64_literal`, `readable_component`) | kani | `kani/abi.rs` | split |
| `kani` (`generate_kani_bundle`, `KaniArtifactBundle`, `ProofDependency*`, validation) | kani | none | retired (step 4f), unless the planner keeps a proof-dependency graph (Risks) |
| `kani` (`deterministic_json`, `artifact`) | core | `core/canonical.rs` | merged into the one helper |
| `kani_obligations` | kani | `kani/generate/{negotiate,scalar,precondition,contract}.rs`, `kani/identity.rs` | split |
| `state_frame` | kani | `kani/generate/frame.rs` | moved; one entry with `negotiate` |
| `bounded_kani_corpus` | kani | `kani/generate/corpus/` | moved; its package lowerer retired after QSL-353 (step 4g) |
| `bounded_kani_profile`, `bounded_collections`, `definedness_arithmetic`, `finite_reference_graphs` | kani | `kani/generate/corpus/` | moved with the corpus, their only `src/` caller; they forward to IR lowerings |
| `kani_execution` | kani | `kani/run/`, `kani/classify.rs` | split |
| `kani_transcript` | kani | `kani/output/` | moved; PR 210 replaces its parse |
| `kani_witness_join` | replay | `replay/witness.rs`; text scanning to `kani/output/playback.rs` | split |
| `spine_replay` | replay | `replay/function.rs` | moved |
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
evidence    --> core
oracle      --> core
core        --> (nothing in this crate)
lib         --> everything (re-exports only)
```

Rules, each checkable:

- A directory imports only the directories to its right. `strategy`, `evidence` and `kani` are peers
  and import none of each other. `kani` and `replay` import nothing from `routed`, and the adapter
  trait is defined in `routed/adapter.rs` with the Kani implementation beside it. `routed` is the one
  place that calls into both `kani` and `replay`.
- Imports use a module path (`use crate::core::artifact::Artifact`), never an item re-exported
  from the crate root. `lib.rs` is the only file that names a root re-export.
- Inside `kani/` the order is `abi`, `identity`, then `generate` and `output`, then `classify`,
  `run`, `terminal`. A file imports only earlier names in that order. `output` imports `identity`
  and `abi` and nothing from `generate`, `classify` or `run`. `run` imports `identity`, `output` and
  `classify`, not `generate`, so a harness is run from its identity and its source text.
- A `#[cfg(test)]` module obeys the same direction as the file it sits in. The edge at
  `kani_transcript.rs:261` is moved to the `classify` tests.
- `serde_json::Value` is named only in `core/ir`, `core/canonical` and the report parser. No other
  file reads a field of a body term.
- `BoundPackage` and `BoundClause` are named only in `strategy/`, `evidence/`, `oracle/boolean_v1.rs`
  and the V1 arm that step 4f deletes. After step 4f none is named in `kani/`, `routed/` or
  `replay/`.

### The one Kani generator

FR-015 is the one generator (ADR-001 Q1, AD-001). In code that means one renderer, one spec type,
one identity record, one cover rule and one entry, not one function:

- `kani/generate/spec.rs` defines `HarnessSpec`: the proof attribute (`proof` or
  `proof_for_contract`), `HarnessPath`, the argument bindings with their bounds, the assumptions,
  the subject call, the assertions and the covers. Its constructor refuses an empty cover list, so
  a harness with no non-vacuity cover cannot be built. That closes IR-464 for every family at
  once.
- `kani/generate/render.rs` is the only function that turns a `HarnessSpec` into text. It is the only
  file whose source contains `kani::proof`, `kani::any`, `kani::assume` or `kani::cover!`. A layout
  test greps for that.
- Four family lowerers (`scalar`, `precondition`, `contract`, `frame`) each take lowered IR claims
  and return `HarnessSpec`s. They do not format Rust source. `generate_state_frame_obligations`
  becomes a frame arm of `ObligationItem`, so `negotiate_kani_obligations` is the one public
  generation entry; ADR-004 owns what the frame arm asserts.
- `ScalarObligationIdentity`, `KaniObligationIdentity` and `StateFrameIdentity` collapse into one
  `kani/identity.rs` record with the family's own members as a typed variant, so the argument
  vector, ceilings, symbols and bounds are spelled once. AD-003's obligation digest is computed
  from that record by one function.

What happens to the other generators:

| Generator | Fate |
| --- | --- |
| `generate_kani_bundle` | Deleted at step 4f, after step 4e and the QSL-owned move of the QI exemplars. Its `proof_for_contract` harness is the contract family's output, and the contract family already emits a cover. Its public entry leaves `interface-001`. FR-003's optional stubbing was dropped by the IR-311 ruling. |
| `kani_obligations` scalar, precondition, contract renderers | Become family lowerers; the template text moves into `render.rs`. |
| `state_frame` renderers | Become the frame lowerer. |
| `bounded_kani_corpus` renderer | Renders through `render.rs` as a corpus family until QSL-353 lands, then its hand-built package lowerer is retired (step 4g). It returns no `KaniOutcome` at generation time: a verdict comes only from a run. |

**Requirement (regression test the one generator must keep passing).** QSL's real-Kani arithmetic
control, in QSL's exemplar tests (4 of 4) and recorded as QSL-342 QI #10, stays green on the
surviving generator at every step of the migration. A lowered `+` mutated to return
`left + right + 1` must classify `Falsified`, and `decode_falsification` must name
`amount_current = 999`. The unmutated control must prove with every cover satisfied. The planner
reports that the control currently passes through `generate_kani_bundle`, the path step 4f
deletes. Step 4a therefore lands the control as a CG-side real-Kani test on the FR-015 path, in the
`make kani` lane, before any generator is touched, and step 4f is not merged until the QSL
exemplars have moved onto the one public entry. The control's package comes from QSL's facade
(`call_site(...).package`, or the source plus `qsl_replay`), not from a copied QI test or a
QSL-emitted package file. This AD did not run the control.

### The Kani run, report and witness (questions c and d)

Where the report file lives. The runner already owns the run's directory: it sets
`CARGO_TARGET_DIR` from `KaniExecutionRequest::target_directory` and runs in
`crate_directory`. The report is run evidence, not a generated artifact, so it is never part of an
`ArtifactBundle` and is never published. Decision: `kani/run` passes
`--export-json <target-dir>/quire-kani-report.json` (the path PR 210 uses), removes any file
there before launch (a stale report must never read as this run's verdict), reads it once after
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
| Terminal-value maps | `kani/terminal.rs` | typed outcomes only |
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
change. This AD follows it and keeps no V1 boundary as a design goal. It records only the
work order the ruling allows, because the code still has V1 readers:

- Kani side: V1 has no consumer once the contract family exists. Step 4f deletes the
  `BoundClause` arm of `ObligationItem`, `ObligationSubject::BoundClause`, `MixedBoundPackages` and
  `generate_kani_bundle`.
- Oracle, strategy and evidence side: `harness`, `strategy/bound`, `evidence/bound_coverage` and
  `oracle/boolean_v1.rs` read V1 and have no V2 criteria yet (FR-002, FR-004 and FR-008 to
  FR-013 are written over `BoundPackage`). They sit in their final directories, named V1 in
  the module map, and each is deleted in the PR that adds its V2 replacement. A layout test fails
  if a V1 type is named anywhere else. This is a rule about where the leftover may be, not a
  second supported path.

### Shared core: helpers, identities, node access, profile

- **One helper module** `core/canonical.rs`: `deterministic_json` (one error type), the one content
  digest, and lowercase hex. The nine `artifact` wrappers are deleted and callers use
  `Artifact::new`. The digest wraps `qsl_replay::ByteDigest`, QSL's type, and CG writes no hash
  routine of its own.
- **One digest identity.** The only digest CG mints for a proof is the obligation digest that
  binds a proof to its content (AD-003, E-1). `core/identity.rs` defines it as `ContentDigest`: 32
  bytes, rendered as lowercase hex, built only by `core::canonical`. Every other digest on the
  chain (`package_id`, byte digests of provided source) is QSL's or IR's and is carried, not
  recomputed. The corpus `CaseIdentity` digest goes with the corpus or becomes a `ContentDigest`
  of the same function. No tool, version or file digest is added.
- **Identity newtypes.** `ModuleSymbol` and `HarnessSymbol` are validated Rust identifiers, built
  once where a harness is generated. `HarnessPath` is the pair. `KaniSolver` replaces the `solver:
  String` field. `generate_routed` keys by `HarnessPath` and a duplicate is a typed error, not a
  silent `collect` overwrite. Bounds travel as typed integers and are not re-parsed from decimal
  text.
- **One typed node-access layer** `core/ir/`: the only code that opens a `CheckedPackageV2` body
  term. It exposes a node view with a typed tag, typed member accessors and typed errors, and
  carries the operator table (identifier, arity, reachable range) as one enum in place of the three
  string tables. The four walkers (63 `.get` calls at this base) migrate onto it, one module per
  PR, and the count outside `core/ir` goes to zero. IR owns the decoder it exposes (IR's layout AD);
  until IR exposes one, `core/ir` is the single place that spells the member names, and when IR
  does, it becomes a thin re-export of IR's. That is one access layer, not a bridge between two.
- **The version profile in one place** `core/profile.rs`: the emitted oracle crate's manifest
  template (written once, called by the three emitters), whose runtime dependency names the
  runtime source the way CG's own `Cargo.toml` does (git URL and branch), and every contract and
  schema spelling listed under Current state. `RUNTIME_REVISION` is deleted (see Decisions taken
  from the planner). A
  spelling QSL will export (AD-002's R-Q5) leaves this file when it does. The profile holds
  spellings this build emits or requires; it records no tool version and asserts no digest
  over a file.

### The two recorded exceptions

**FR-005 (CLI conformance; TC-001, TC-002, TC-007).** FR-005 is in `core` but depends on FR-002
(strategy), FR-015 (kani) and FR-004 (evidence), against ADR-0056 rule 4. Decision (the IR
planner, on IR-344): a `publication` subsystem, `spec/publication/`, owning FR-005 and its three
test cases, and the code directory `publication/`. There is no FR-005 exception. The move is a
follow-up PR (migration step 7), and the registry note in `spec/spec.md` that records the
exception becomes obsolete when it lands.

| | Publication subsystem (decided) | Accepted core exceptions (not taken) |
| --- | --- | --- |
| Benefit | Rule 4 holds everywhere. `core` shrinks to shared primitives, which is what rule 4 says it is. The requirement sits in the top layer where it depends on the lower ones, the same direction the code takes. The code needs no upward import: bundle construction and its limits move to `core/artifact.rs`, and only the atomic writer is `publication/`. | No spec movement. |
| Cost | One registry row, one directory, one matrix index row; `git mv` of FR-005 and TC-001, TC-002, TC-007 with ids unchanged (ADR-0056 identifier rules); the `Owning crates/modules` column changes for `publication`. FR-005-AC-5 describes bundle limits enforced in `core/artifact.rs` but lives in `publication/`, a split the criterion's verification (TC-002) already spans. | The exception stays for as long as the requirement spans subsystems, and "core depends on nothing" stops being checkable. The next cross-cutting requirement has a precedent. |

Splitting FR-005 into a core publication requirement and an integration requirement was not
taken, because it mints ids.

**`core/matrix/suites.md` (SUR-001, a SuiteRegistry).** ADR-0056 has no matrix slot for it.
Recommendation: `spec/core/functional/suites.md`, by analogy with ADR-0056 rule 5, which puts "a
registry every subsystem consumes" in `core/functional/`. SUR-001 lists the commands that
validate and run every subsystem (spec validation, coverage export, MSRV build, the Kani lane), so
every subsystem consumes it, and it depends on no subsystem's requirement. The alternative is to
amend ADR-0056 in `quire-contract-ir` to add a `matrix/suites.md` slot; that costs an edit to an
ADR three repositories reference, for one file. It can wait.

## Decisions

Statements a test can check. Local labels; the repository assigns requirement ids when a
requirement is authored.

- L-1. `src/` has the directories `core`, `oracle`, `strategy`, `evidence`, `kani`, `replay`,
  `routed` and `publication`, plus `lib.rs`, and every module in the map above lives in the one the
  map names. Test: a layout test lists `src/` and compares with the registry.
- L-2. The import graph is acyclic and follows the dependency direction above, `#[cfg(test)]`
  modules included, and no file imports an item through the crate root. Test: a layout test that
  reads `use crate::` lines.
- L-3. `kani::proof`, `kani::any`, `kani::assume` and `kani::cover!` appear as text in
  `kani/generate/render.rs` only. Test: a grep gate.
- L-4. A `HarnessSpec` with no cover cannot be constructed. Test: the constructor's refusal.
- L-5. QSL's arithmetic control passes on the surviving generator: the `left + right + 1` mutant
  is `Falsified` with `amount_current = 999`, and the unmutated control proves with every cover
  satisfied. Test: real Kani, in the `make kani` lane. This holds at every migration step.
- L-6. Kani output is read in `kani/output/` only, and the classifier, the terminal maps and the
  witness decode take typed values. Test: a grep for Kani's banner, check and playback wording
  outside that directory.
- L-7. The report path is `<target-dir>/quire-kani-report.json`, removed before launch and absent
  from every `ArtifactBundle`. Test: a stand-in launcher that leaves a previous report.
- L-8. `serde_json::Value` and `.get("` appear for body terms only in `core/ir`. Test: a grep
  gate.
- L-9. `deterministic_json`, `Artifact::new` and the content digest each have one definition.
  `fn artifact(` has none. Test: a grep gate.
- L-10. `module_symbol` and `harness_symbol` have no `String` field outside `core/identity.rs`, and
  a duplicate `HarnessPath` is a typed error. Test: a duplicate-key test on `generate_routed`.
- L-11. The oracle crate manifest template, the runtime dependency spelling and every contract
  and schema spelling are defined in `core/profile.rs` once.
- L-12. `BoundPackage` and `BoundClause` are named nowhere in `kani/`, `routed/` or `replay/`, and
  after the last V1 reader is replaced nowhere.

### Migration order

Each step is one PR (several for step 3), keeps main green and deletes what it replaces. Per the
ruling there is no compatibility layer anywhere in it. It is the order IR-348 files its
tickets in.

1. **Shared core, no moves.** 1a: `core/canonical` and deletion of the nine `artifact`
   wrappers and the two `deterministic_json` definitions. 1b: identity newtypes
   threaded through `kani_obligations`, `state_frame` and `routed_generation`, with the typed
   duplicate error. 1c: `core/profile`, with `RUNTIME_REVISION` deleted.
2. **Directories, rename-only.** Precondition: PR 210 has landed, after this AD is approved and
   with a local `make kani` transcript from its head. `git mv` plus path fixes, imports by module path, no logic change,
   one PR per subsystem in leaf order: 2a `core` (extract from `oracle.rs`), 2b `oracle`, 2c
   `evidence` and `strategy`, 2d `kani` (including the `run`, `output`, `classify` split of
   `kani_execution` and `kani_transcript`, and the test back-edge), 2e `replay`, `routed`,
   `publication`. The layout test (L-1, L-2) lands with 2e, when it can pass.
3. **Typed node access.** `core/ir` with the operator enum, then `state_frame`, `exact_scalar`,
   `composite_equality` and `exact_function` onto it, one PR each. L-8 lands with the last.
4. **The one generator.** 4a the CG-side arithmetic control (L-5), before anything else in this
   step. The test builds its package through QSL's facade (`call_site(...).package`, or the QSL
   source plus `qsl_replay`), never from a copied QI test or a QSL-emitted package file. 4b `HarnessSpec` and `render`, with the scalar family ported. 4c precondition and
   contract. 4d frame, and `generate_state_frame_obligations` becomes an `ObligationItem` arm.
   4e **CG's one public generator entry exists** (`negotiate_kani_obligations` with the contract
   and frame arms, ported in 4b to 4d, is the public path a caller can use for everything
   `generate_kani_bundle` served). This is the step the leader reports to the planner when it
   lands. A QSL-owned follow-up then moves QSL's quire-integration exemplars, which call
   `generate_kani_bundle` today, onto that entry; it is a QSL ticket, not CG work. 4f deletion of
   `generate_kani_bundle`, the V1 obligation arm and, unless the planner keeps it (see Decisions
   taken from the planner), the proof-dependency graph, with the matching `interface-001` edit.
   4f is merged only after 4e has landed and the QSL follow-up has moved the exemplars. 4g the
   corpus (see Decisions taken from the planner).
5. **One output reader.** Precondition: PR 210 has landed before the directory moves (it is a
   precondition of step 2, not of step 5), and it lands only after this AD is approved and with a
   local `make kani` transcript from its head. The playback scanning then moves from
   `kani_witness_join` to `kani/output/playback.rs`; `replay/witness.rs` takes typed entries.
   Batching follows, in `run/` only. Step 5's reader and the C-09 map (`kani/terminal.rs`) map
   `KaniRunOutcome` to `TerminalValue`, so both depend on QSL-351 (`Inconclusive(cause)`, a
   `NonZero` `Proved`, the tool pin gone) and on the rule that the terminal value also takes the
   replay settlement. Neither step 5's terminal map nor `kani/terminal.rs` merges before QSL-351.
6. **V1 readers.** Each of `harness`, `strategy/bound`, `evidence/bound_coverage` and
   `oracle/boolean_v1.rs` is replaced and deleted with its V2 criteria. IR-364 (the V2 strategy
   chain: FR-002, FR-004 and FR-008 to FR-013 over `CheckedPackageV2`) is owned by the IR team and
   is ordered before this step; no V1 reader is deleted until its criteria exist.
7. **Spec follows the code.** One spec PR: the registry rows by directory; FR-005 and
   TC-001, TC-002, TC-007 to `spec/publication/`; SUR-001 to `core/functional/`; `interface-001`
   and `tests.md` fixed. `git mv`, ids unchanged. When this step lands, the registry note in
   `spec/spec.md` that records the FR-005 exception becomes obsolete and is deleted in the same
   PR, as is the SUR-001 note.

## Risks

- A rename-only step that also touches imports is large in line count. It is reviewable only if
  it carries no logic change, so each of 2a to 2e must be refused if it does.
- The layout test reads `use` lines, not the compiler's graph. A path in a macro or a
  `super::` import would escape it. The measured edge list in this AD was made the same way
  and has the same blind spot.
- Step 4f depends on a fact this AD could not verify: that QSL's exemplars can be moved onto the
  one public entry. The planner's record puts the arithmetic control on `generate_kani_bundle`. If
  the exemplars need something the contract family does not emit, 4f waits.
- Batching with per-harness ceilings (FR-028) needs a rule for the batch's wall clock. This AD
  puts batching in `run/` and leaves the rule to FR-017 and IR-277.
- Typed node access depends on what IR exposes. If IR's decoder lands later than `core/ir`, the
  member names are spelled in CG once, which is still fewer than today's 63 calls plus string
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
| Proof-dependency graph | Retire it with the V1 bundle unless a current requirement names it. Checked: FR-015 does. Its Inputs and Behavior name an optional declared proof-dependency census per obligation, folded into the harness identity, and FR-015-AC-22 and AC-25 are the criteria TC-005 traces. So those requirements stay as planned and are not touched. Ask the planner: the census is a V2 request input, while `ProofDependencyGraph` is the V1 bundle's output type, so whether the graph type survives in `kani/` to carry the census or is replaced by identity members is the planner's to say. Step 4f leaves the type in place until it answers. | IR planner, IR-344 |
| `RUNTIME_REVISION` | Deleted, not spelled once. Checked: the constant is read by the three emitters' manifest templates (a `rev = "..."` on the runtime dependency) and by tests (failure messages and `manifest.contains`). Nothing else decides on it: no code compares it with the runtime CG itself builds against, and CG's own `Cargo.toml` names the runtime by branch with the lockfile recording the commit. The emitted manifest needs a runtime source, not a pin, so the template names it the way `Cargo.toml` does. The cost is that an emitted crate follows the runtime branch as CG does. I did not build an emitted crate to confirm. | IR planner, IR-344; QSL concurs; repository CLAUDE.md |
| PR 210 | Lands before the directory moves, only after this AD is approved, and with a local `make kani` transcript from its head. A precondition of migration step 2. | IR planner, IR-344 |
| V2 strategy criteria | FR-002, FR-004 and FR-008 to FR-013 over `CheckedPackageV2` are IR-364 (M3, the V2 strategy chain), IR team, ordered before step 6. | IR planner, IR-344, IR-364 |
| One public entry before deletion | Migration step 4e: the one public generator entry exists. A QSL-owned follow-up moves QSL's quire-integration exemplars, which call `generate_kani_bundle` today, onto it. Only then does step 4f delete `generate_kani_bundle`. | QSL review of this PR |
| Package source for tests | The arithmetic control (step 4a) and, after QSL-353, the corpus build packages through QSL's facade (`call_site(...).package`, or source plus `qsl_replay`). CG copies no QSL fixture and no QSL-emitted package file. | QSL review of this PR |
| Terminal map dependency | Step 5's reader and the C-09 map (`kani/terminal.rs`) depend on QSL-351 (`Inconclusive(cause)`, a `NonZero` `Proved`, the tool pin gone) and on the terminal value also taking the replay settlement. | QSL review of this PR |
| `ContentDigest` | Wraps QSL's `ByteDigest` through the facade. | QSL review of this PR |

Still open: the SuiteRegistry home (`spec/core/functional/suites.md` is the recommendation; the
planner has not ruled), and the planner's answer on the proof-dependency graph type.

### Not verified in this AD

- The arithmetic control was not run, and QSL's tests were not read. Which generator they route
  through is the planner's statement on IR-344.
- The 63-call count and the import edges are greps over `src/`, not the compiler's output, and no
  build or test was run for this AD.
- PR 210 was read as a description and file list, not built. It is not at this base.
- Whether IR exposes a typed body-term decoder today was not checked; IR's layout AD is a separate
  ticket.
