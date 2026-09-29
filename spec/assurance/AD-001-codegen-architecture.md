---
id: AD-001
title: Contract codegen architecture
type: ArchitectureDescription
status: accepted
owner: codegen-maintainers
system: quire-contract-codegen
relationships:
  - target: ix://agent-ix/quire-contract-codegen/ADR-001
    type: references
  - target: ix://agent-ix/quire-contract-codegen/ADR-002
    type: references
  - target: ix://agent-ix/quire-contract-codegen/ADR-003
    type: references
  - target: ix://agent-ix/quire-contract-codegen/ADR-004
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-011
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-013
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-015
    type: references
  - target: ix://agent-ix/quire-specification/AD-016
    type: references
---
# Contract codegen architecture

## System Boundary

The code generator turns an admitted contract package into Rust oracles, property-test harnesses,
bounded Kani obligations and their evidence, runs one Kani obligation on the installed backend,
and hands every counterexample to QSL for native replay. It is the Kani backend adapter of the
AD-016 pipeline: it selects the runtime operation per IR node at arrow 3, settles capabilities at
arrow 4's single negotiation point and generates oracles there, generates harnesses at arrow 5, and
owns the execution half of arrow 6 and the reconstruction half of arrow 7.

The generator owns:

- oracle generation: the exact complete-V1 oracles of every scalar family, the Boolean connectives
  and the bounded-integer comparisons included (FR-014), composite and structural equality oracles
  (FR-018) and function-application oracles (FR-021);
- property-test strategies and harnesses (FR-002, FR-008 to FR-013);
- Kani obligation generation, by FR-015 alone;
- capability settlement at one `negotiate_*` point over the closed `BackendKind` (FR-019) and
  generation of the items QSL `route` routed (FR-022);
- the Kani backend adapter, one implementation of the adapter trait (FR-026): execution under the
  harness identity's ceilings (FR-017, FR-028), the one
  Kani transcript parser (`src/kani_transcript.rs`), the map from a run outcome to QSL's terminal
  value (FR-029), the witness join (FR-016), the counterexample
  submission to QSL (FR-024) and the generated subject ABI (FR-025);
- publication and vacuity evidence (FR-004, FR-005).

Outside the boundary, each a dependency `Cargo.toml` names:

- Contract IR reads and lowers the package through the `CheckedPackageV2` reader and lowering.
- Contract Runtime publishes the `exact` surface every generated oracle calls, and co-owns the
  bounded shadow model and its refinement obligation for its kernel types.
- QSL owns the replay and proof types in `qsl-replay` and the executor behind them.
- Kani and CBMC prove; the Rust compiler and proptest build and run what is generated.

## Views

Five views answer the concerns of this boundary: what is generated from which input, which runtime
surface generated code calls, how one obligation runs, how a counterexample reaches native replay,
and which failure states stay distinct.

### Generation view

One input model enters the generator: an admitted `quire.checked-package/v2` package, read through
Contract IR's `CheckedPackageV2` (ADR-001). Every generator reads it.

The routed path runs in four steps:

1. The driver passes the FR-331 backend provider envelope to `negotiate_backend_provider`
   (FR-019). Each item settles once, in the `negotiate_*` arm of its `BackendKind`.
2. QSL `route` routes each item that settled `supported`. The generator reads the candidate set
   and computes none.
3. `generate_routed` (FR-022) runs the generation arm of each routed item's `BackendKind`, keyed
   by request index. It settles nothing again.
4. The Kani arm calls `negotiate_kani_obligations` (FR-015) with one `ScalarClaim` item per routed
   item. Each harness embeds the FR-014 oracle of its claim and one non-vacuity cover.

FR-015 is the one Kani generator. Each of its harnesses embeds the FR-014 oracle of its claim,
records the ceilings it runs under and names its family (FR-028).

### Runtime `exact` surface view

Every generated oracle crate calls `quire_contract_runtime::exact` and interprets no expression
itself:

- FR-014 scalar oracles call the kernel scalar operations over `rt::Integer` and the other exact
  scalar types.
- FR-018 equality oracles call `check_equality` and `CheckedEquality::evaluate`.
- FR-021 function-application oracles admit a `PackageDeclarations` through
  `PackageDeclarations::check` and apply one function through `CheckedPackage::call`.

Each oracle takes a caller-supplied `Meter` and returns the runtime `Outcome` unchanged. A Kani
harness embeds its FR-014 oracle, so the prover proves the generated code through the same `exact`
operations the native oracle runs. A family whose production harness cannot verify within its
ceilings proves its claim over a bounded shadow instead, together with a refinement obligation that
the production code agrees with the shadow on the bounded domain (ADR-003, FR-028).

### Execution view

`execute_kani_obligation` (FR-017) runs these steps in order:

1. Check that the crate contains the harness source byte for byte.
2. Launch the installed backend under the memory and wall-clock ceilings the harness identity records
   (FR-028).
3. Read the output once, in `src/kani_transcript.rs`, into a typed transcript.
4. Classify the run from that transcript into one `KaniRunOutcome`. A run that exceeds a ceiling is
   `inconclusive` with that ceiling's own reason.
5. Map the outcome to exactly one QSL terminal value in one total match (FR-029). A vacuous or cover-unsatisfied run maps to
   `Proved { success_checks: 0 }`, QSL's inconclusive vacuity record.

The caller retains the returned evidence.

### Replay view

A falsified run's concrete playback goes through these steps:

1. The witness join types the playback against the harness's persisted argument bindings (FR-016).
2. The adapter checks each decoded value against its declared domain before any replay (FR-016,
   FR-024).
3. The adapter renders the values into QSL's backend-witness transcript, admitted by
   `Witness::parse`. It builds QSL's counterexample envelope around that `ReplaySource`, keyed by
   QSL's `ObligationIdentity` (FR-024).
4. The adapter builds the replay request from the envelope and the proving run's package
   reference, byte provision and limits, and calls `qsl_replay::replay`.

QSL recompiles the provided source and evaluates the selected function, so the verdict is
QSL's evaluation and never a value the generator supplies. A frame counterexample replays through
QSL's `replay_frame`. A corpus counterexample that has no backend transcript takes the `Input` arm
and settles `reproduced-without-witness`, which is never backend evidence.

### Failure view

Invalid input, unsupported semantics, requires-bound, an absent backend, I/O failure, an incomplete proof, vacuity, a timed-out run, falsification, a malformed or
out-of-domain witness and a replay disagreement each stay a distinct typed state. None has a
success fallback, and no requirement converts one into another.

## Seams

| Seam | What crosses | Type owner | CG side |
|---|---|---|---|
| IR → CG | `CheckedPackageV2`, lowered claims and `bounded_domain` bounds | Contract IR | Reads them and never re-derives a bound from a caller descriptor. |
| CG → RT | Generated calls into `quire_contract_runtime::exact`: kernel scalar operations, `check_equality`, `PackageDeclarations::check`, `CheckedPackage::call`, `Meter`, `Outcome` | Contract Runtime | Emits calls and charges nothing itself. |
| QSL → CG | The FR-331 envelope and its `candidates` | QSpec wire; QSL `route` computes candidates | FR-019 reads the envelope, and FR-022 generates for what was routed. |
| CG → QSL | `Witness`, `ReplaySource`, the counterexample envelope `WitnessEnvelope`, `ObligationIdentity`, the replay request, `replay` and `replay_frame` | QSL `qsl-replay` | Builds them and calls the facade. Target: no copy of these types in CG or in Contract IR; see Current state. |
| CG → QSL | The FR-331 terminal value of a run | QSL `qsl-replay` | The Kani adapter maps `KaniRunOutcome` to `TerminalValue` in one total match, one value per run (FR-029). An item settled `unsupported` at negotiation has no terminal value. |
| CG ↔ Kani | The option vector in, the printed transcript and concrete playback out | Kani | Only `src/kani_transcript.rs` reads the text. |

## Decisions

- QSL owns the replay and proof types in `qsl-replay`: `Witness`, `ReplaySource`, the
  counterexample envelope, the FR-331 terminal record and `ObligationIdentity`. Contract IR holds
  no copy of them. CG uses QSL's `ObligationIdentity` and QSL's envelope, and keeps the Kani
  transcript parser as part of its backend adapter. The authority is the `qsl-replay` API, which
  defines all five types.
- CG builds QSL's `ObligationIdentity` from every `KaniObligationIdentity` member except
  `source_span` (AD-016 arrow 5), and builds its envelopes as QSL's `WitnessEnvelope` (FR-024).
- CG replays only through `qsl_replay::replay`, the layer-6 facade (QSL ADR-013 TK-01), and frame
  counterexamples only through `qsl_replay::replay_frame` (QSL FR-116). No replay path takes a
  caller-supplied executor (QSL ADR-011 FB-07).
- `CheckedPackageV2` is the one input model. FR-015 is the one Kani generator, and FR-014 the one
  oracle generator for every family it covers (ADR-001).
- The Kani transcript parser belongs to CG's backend adapter. Kani's printed wording is read in
  exactly one module, and a backend's native counterexample reaches QSL only as QSL's
  backend-witness transcript.
- A backend adapter is one implementation of one adapter trait, reached only through an exhaustive
  match on the closed `BackendKind` enum, and it owns its own execution evidence type (ADR-002,
  FR-026). Adding a kind without an arm is a compile error (FR-019, FR-022).
- Every harness is bounded: inclusive bounds from the declared model domains, and a memory ceiling
  and a wall-clock ceiling in its identity. A family proves the production code where it verifies
  within them, and a bounded shadow with a refinement obligation elsewhere. Coverage is reportable
  per family, and a tightened bound is always recorded (ADR-003, FR-028).
- The generator settles each capability once. Generation, execution and replay then take the
  settlement as given.
- Generation and execution are separate claims. A generated harness records proof execution as
  `not_run`, and only an observed run with every cover satisfied is `verified` (FR-015, FR-017).
- Separate-obligation harnesses are lowered against `cadical` with no stubbing option, and every
  symbolic argument is assumed inside its IR `bounded_domain` (FR-015).
- State reaches a generated subject by `&mut` reference to a harness-owned value. The frame harness
  is written over AD-016's frame subject (ADR-004, FR-025).
- `quote`, `syn` and the bounded renderer emit Rust syntax. Stable ordering and path-independent
  names make regeneration byte-identical (NFR-001).

## Current state

The views and decisions above state the target. At this revision:

- `src/kani_witness_join.rs` decodes the playback through Contract IR's `Witness`.
  `src/bounded_kani_corpus.rs` imports Contract IR's `ReplaySource`. `src/bounded_kani_replay.rs`
  replays through Contract IR's `replay_counterexample` with a caller-supplied native evaluator,
  and its test module imports Contract IR's `ReplaySource`.
- Step 2 of the replay view, the domain check before replay, is not built. No `WitnessEnvelope` is
  built, and only the skeleton spine renders a QSL transcript (`src/spine_replay.rs`).
- The V1 paths are still present: `src/oracle.rs`, `src/kani.rs`, `src/bounded_kani_corpus.rs`,
  `src/bounded_kani_profile.rs` and `src/bounded_kani_replay.rs`, and the `BoundClause` arm of
  FR-015's `ObligationItem`. The FR-002 and FR-008 to FR-013
  strategy generators still read the V1 `BoundPackage`.
- No adapter trait exists, a run is held to a caller-declared wall-clock budget with no memory
  ceiling, and no outcome maps to QSL's terminal
  value.
- FR-014, FR-015, FR-024 to FR-026, FR-028, FR-029 and the test matrix record which criteria each of these leaves
  planned.

## Risks

- Kani publishes no machine-readable verdict, so a Kani release that changes its wording changes
  what the transcript parser recognises.
- Kani tractability over production data structures is unmeasured beyond single scalar operations,
  so which families need a bounded shadow is known only once their runs are measured (ADR-003).
- The frame lowering waits on QSpec (ADR-004).
- Platform formatting and path behaviour can threaten byte reproducibility.
- The runtime release decision remains a human decision.
