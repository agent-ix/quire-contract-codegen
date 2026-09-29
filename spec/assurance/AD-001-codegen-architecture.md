---
id: AD-001
title: Contract codegen architecture
type: ArchitectureDescription
status: proposed
owner: codegen-maintainers
system: quire-contract-codegen
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AP-001
    type: realizes
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
bounded Kani obligations and their evidence, runs one Kani obligation under committed backend pins,
and hands every counterexample to QSL for native replay. It is the Kani backend adapter of the
AD-016 pipeline: it selects the runtime operation per IR node at arrow 3, settles capabilities at
arrow 4's single negotiation point and generates oracles there, generates harnesses at arrow 5, and
owns the execution half of arrow 6 and the reconstruction half of arrow 7.

The generator owns:

- oracle generation: the V1 Boolean and bounded-integer oracles (FR-001), the exact complete-V1
  scalar oracles (FR-014), composite and structural equality oracles (FR-018) and
  function-application oracles (FR-021);
- property-test strategies and harnesses (FR-002, FR-008 to FR-013);
- Kani obligation generation (FR-003, FR-007, FR-015);
- capability settlement at one `negotiate_*` point over the closed `BackendKind` (FR-019) and
  generation of the items QSL `route` routed (FR-022);
- the Kani backend adapter: pinned execution (FR-017), the one Kani transcript parser
  (`src/kani_transcript.rs`), the claimed-module gate (FR-023), the witness join (FR-016), the
  counterexample submission to QSL (FR-024) and the generated subject ABI (FR-025);
- publication and vacuity evidence (FR-004, FR-005).

Outside the boundary, each version-identified by the revision `Cargo.toml` pins:

- Contract IR reads and lowers the package: the V1 `BoundPackage` projection, the
  `CheckedPackageV2` reader and lowering, and the `kani-bounded/1` profile.
- Contract Runtime publishes the `exact` surface every generated oracle calls.
- QSL owns the replay and proof types in `qsl-replay` and the executor behind them.
- Kani and CBMC prove; the Rust compiler and proptest build and run what is generated.
- Quoin retains, seals and verifies evidence; the generator retains none.

## Views

Five views answer the concerns of this boundary: what is generated from which input, which runtime
surface generated code calls, how one obligation runs, how a counterexample reaches native replay,
and which failure states stay distinct.

### Generation view

Two input models enter the generator. A V1 `BoundPackage` feeds FR-001, FR-002, FR-003, FR-007 and
the `BoundClause` arm of FR-015. An admitted `quire.checked-package/v2` package, read through
Contract IR's `CheckedPackageV2`, feeds FR-014, FR-018, FR-021, FR-022 and the `ScalarClaim` arm of
FR-015. ADR-001 asks which of these survives.

The routed path runs in four steps:

1. The driver passes the FR-331 backend provider envelope to `negotiate_backend_provider`
   (FR-019). Each item settles once, in the `negotiate_*` arm of its `BackendKind`.
2. QSL `route` routes each item that settled `supported`. The generator reads the candidate set
   and computes none.
3. `generate_routed` (FR-022) runs the generation arm of each routed item's `BackendKind`, keyed
   by request index. It settles nothing again.
4. The Kani arm calls `negotiate_kani_obligations` (FR-015) with one `ScalarClaim` item per routed
   item. Each harness embeds the FR-014 oracle of its claim and one non-vacuity cover.

The Boolean and numeric FR-003 lowering and the FR-007 corpus stay direct entry points over the V1
package.

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
operations the native oracle runs. ADR-003 records what that costs the prover.

### Execution view

`execute_kani_obligation` (FR-017) runs these steps in order:

1. Compare the harness identity's pins with the committed pins, before any process starts.
2. Measure the installed `cargo-kani`, `kani-driver`, CBMC, toolchain and target, and compare the
   six measured pins with the committed pins.
3. Check that the crate contains the harness source byte for byte.
4. Launch the backend under the caller's wall-clock budget.
5. Read the output once, in `src/kani_transcript.rs`, into a typed transcript.
6. Classify the run from that transcript into one `KaniRunOutcome`.

The caller retains the returned evidence. The claimed-module gate (FR-023) reads the same typed
transcript and passes only when every claimed module holds a `SUCCESS` check.

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

QSL recompiles the digest-addressed source and evaluates the selected function, so the verdict is
QSL's evaluation and never a value the generator supplies. A frame counterexample replays through
QSL's `replay_frame`. A corpus counterexample that has no backend transcript takes the `Input` arm
and settles `reproduced-without-witness`, which is never backend evidence.

### Failure view

Invalid input, unsupported semantics, requires-bound, a backend that cannot be measured, pin drift,
I/O failure, an incomplete proof, vacuity, a timed-out run, falsification, a malformed or
out-of-domain witness and a replay disagreement each stay a distinct typed state. None has a
success fallback, and no requirement converts one into another.

## Seams

| Seam | What crosses | Type owner | CG side |
|---|---|---|---|
| IR → CG | `BoundPackage`, `CheckedPackageV2`, lowered claims and `bounded_domain` bounds, the `kani-bounded/1` profile | Contract IR | Reads them and never re-derives a bound from a caller descriptor. |
| CG → RT | Generated calls into `quire_contract_runtime::exact`: kernel scalar operations, `check_equality`, `PackageDeclarations::check`, `CheckedPackage::call`, `Meter`, `Outcome` | Contract Runtime | Emits calls and charges nothing itself. |
| QSL → CG | The FR-331 envelope and its `candidates` | QSpec wire; QSL `route` computes candidates | FR-019 reads the envelope, and FR-022 generates for what was routed. |
| CG → QSL | `Witness`, `ReplaySource`, the counterexample envelope `WitnessEnvelope`, `ObligationIdentity`, the replay request, `replay` and `replay_frame` | QSL `qsl-replay` | Builds them and calls the facade. Target: no copy of these types in CG or in Contract IR; see Current state. |
| CG → QSL | The FR-331 terminal record of a run | QSL `qsl-replay` | ADR-002 Q3 asks who writes the map from `KaniRunOutcome`. |
| CG ↔ Kani | The option vector in, the printed transcript and concrete playback out | Kani 0.67.0 | Only `src/kani_transcript.rs` reads the text. |
| CG → Quoin | Proof-attestation bodies and structured producer results | Quoin | Supplies them and retains nothing. |

## Decisions

- QSL owns the replay and proof types in `qsl-replay`: `Witness`, `ReplaySource`, the
  counterexample envelope, the FR-331 terminal record and `ObligationIdentity`. Contract IR holds
  no copy of them. CG uses QSL's `ObligationIdentity` and QSL's envelope, and keeps the Kani
  transcript parser as part of its backend adapter. The authority is the `qsl-replay` API at the
  revision `Cargo.toml` pins, which defines all five types. QSL ADR-013 O-24 and O-25 and QSpec
  AD-016 still name Contract IR as the witness and packet owner and give it the terminal-record
  map; their amendment to match is pending upstream.
- CG computes the obligation-identity digest over every `KaniObligationIdentity` member except
  `source_span` (AD-016 arrow 5), carries it as QSL's `ObligationIdentity`, and builds its
  envelopes as QSL's `WitnessEnvelope` (FR-024).
- CG replays only through `qsl_replay::replay`, the layer-6 facade (QSL ADR-013 TK-01), and frame
  counterexamples only through `qsl_replay::replay_frame` (QSL FR-116). No replay path takes a
  caller-supplied executor (QSL ADR-011 FB-07).
- The Kani transcript parser belongs to CG's backend adapter. Kani's printed wording is read in
  exactly one module, and a backend's native counterexample reaches QSL only as QSL's
  backend-witness transcript. ADR-002 sets out the adapter boundary.
- The `BackendKind` enum is closed. Settlement and generation dispatch over it exhaustively, so
  adding a kind without an arm is a compile error (FR-019, FR-022).
- The generator settles each capability once. Generation, execution and replay then take the
  settlement as given.
- Generation and execution are separate claims. A generated graph records proof execution as
  `not_run`, and only an observed run with every cover satisfied is `verified` (FR-003, FR-017).
- Separate-obligation harnesses are lowered against `cadical` with no stubbing option, and every
  symbolic argument is assumed inside its IR `bounded_domain` (FR-015).
- `quote`, `syn` and the bounded renderer emit Rust syntax. Stable ordering and path-independent
  names make regeneration byte-identical (NFR-001).
- Retention, integrity checking, audit and attestation are Quoin's. Static specification facts are
  Quire's (FR-006).

## Current state

The views and decisions above state the target. At this revision:

- `src/kani_witness_join.rs` decodes the playback through Contract IR's `Witness`.
  `src/bounded_kani_corpus.rs` imports Contract IR's `ReplaySource`. `src/bounded_kani_replay.rs`
  replays through Contract IR's `replay_counterexample` with a caller-supplied native evaluator,
  and its test module imports Contract IR's `ReplaySource`.
- Step 2 of the replay view, the domain check before replay, is not built. No `WitnessEnvelope` is
  built, and only the skeleton spine renders a QSL transcript (`src/spine_replay.rs`).
- FR-024, FR-025 and the test matrix record which criteria each of these leaves planned.

This description claims no structural convergence across the program's Rust crates. Their
ownership markers, gate names, evidence layouts and architecture records differ, and converging
them is a separately reviewed cross-repository change that this repository's local controls cannot
claim.

## Risks

- Kani 0.67.0 publishes no machine-readable verdict, so a Kani release that changes its wording
  changes what the transcript parser recognises. The committed pins refuse any other installation
  before a run.
- Kani tractability over production data structures is unmeasured beyond single scalar operations.
  The ceiling and its terminal outcome are undecided (ADR-003).
- Three Kani generation paths, two oracle generators and two input models coexist (ADR-001).
- The subject ABI of the families other than Boolean and bounded integer, how state reaches the
  subject, and the frame harness are undecided (ADR-004).
- The committed pins are one x86_64 Linux installation, so another host is refused rather than
  qualified.
- Platform formatting and path behaviour can threaten byte reproducibility.
- The runtime release decision remains a human decision.
