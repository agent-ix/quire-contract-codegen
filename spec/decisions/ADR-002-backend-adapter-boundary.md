---
id: ADR-002
title: "Backend adapter boundary"
type: ADR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-001
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: relates_to
  - target: ix://agent-ix/quire-spec-language/ADR-013
    type: depends_on
---
# ADR-002: Backend adapter boundary

## Status

Proposed. Questions Q0 to Q4 are open until the owner rules, and their recommendations decide
nothing. The only settled input is the owner's ruling of 2026-09-28, recorded under Context.

## Context

Kani is the one backend registered under the FR-331 contract. `BackendKind` has one variant, `Kani`
(`src/capability.rs`). Settlement (FR-019) and routed generation (FR-022) dispatch over it
exhaustively. Everything else that is specific to Kani lives in modules that name Kani directly and
share no stated contract:

| Kani-specific fact | Where it lives |
|---|---|
| Backend version `0.67.0` | `KANI_BACKEND_VERSION`, `src/kani.rs` |
| FR-003 profile identity | `KANI_ADAPTER_PROFILE` (`kani-0.67.0-function-contracts-v2`), `src/kani.rs` |
| FR-015 profile identity | `KANI_OBLIGATION_PROFILE` (`kani-0.67.0-separate-obligations-v1`), `src/kani_obligations.rs`, with the version repeated inside the string |
| Committed installation pins | `KaniToolPins::pinned`, `src/kani_execution.rs` |
| Printed-output wording | `src/kani_transcript.rs`, the only non-test copy (FR-017-AC-10) |
| Real transcript captures | `tests/fixtures/kani-0.67.0/` |
| Playback typing | `src/kani_witness_join.rs` |
| QSL witness rendering | `src/spine_replay.rs` |

On 2026-09-28 the owner ruled that QSL owns `Witness`, `ReplaySource`, the counterexample
envelope, the FR-331 terminal record and `ObligationIdentity` in `qsl-replay`, that Contract IR
deletes its copies, and that the Kani transcript parser stays in CG as part of its backend adapter.
[AD-001](../assurance/AD-001-codegen-architecture.md) states the resulting ownership. The QSL
ADR-013 and QSpec AD-016 amendments that match the ruling had not landed when this record was
written.

### Coverage of the adapter's intents

A requirement is added only where no existing criterion covers the intent. An intent that only
changes where code lives is a refactor and gets none.

| Intent | Existing criterion | Test | Gap |
|---|---|---|---|
| The backend kind set is closed and dispatch is exhaustive | FR-019-AC-9, FR-022-AC-1 | Analysis (compiler) | none |
| Settlement happens at one point | FR-019-AC-5 | TC-030 | none |
| A run is refused unless the installation equals the committed pins | FR-017-AC-1, FR-017-AC-3 | TC-027 | none |
| One module reads the backend's printed wording | FR-017-AC-10 | TC-027 | none |
| The harness identity records the profile, pins and option vector | FR-015-AC-2, FR-015-AC-9 | TC-025 | none |
| A native counterexample reaches QSL only as QSL's backend-witness transcript | none | none | FR-024 |
| A run outcome is reported as QSL's FR-331 terminal record | none | none | Q3, owner undecided |
| One value holds the backend's version profile | none | none | refactor, Q2 |
| A per-backend adapter contract | none | none | refactor, Q1 |

## Decision

Nothing is decided beyond the owner ruling. The owner rules on each question below.

### Q0: What does a backend adapter own?

The ruling places the Kani transcript parser in CG's backend adapter. It does not say what else the
adapter owns.

Options:

1. **Five parts per kind.** A backend adapter is the unit that owns everything specific to one
   `BackendKind`, and for its kind it holds exactly the five parts listed after these options.
2. **The parser only.** The adapter holds the transcript parser. Generation, execution and witness
   rendering stay in modules shared across kinds.

Recommendation: option 1. Under option 2, adding a second backend means editing shared modules
with no compile-time check that each kind supplies every part.

The five parts of option 1:

1. **Version profile.** One value naming the backend version, the committed installation pins,
   the profile identities its harness identities record and the option vector template. Every
   other part reads the version from this value.
2. **Generation arm.** The per-kind arm FR-022 dispatches to.
3. **Execution.** Measure, compare with the version profile's pins, launch under the caller's
   budget, and classify (FR-017).
4. **Transcript parser.** The one module that reads the backend's native output, its
   counterexample included, into a typed transcript. No other non-test source reads that output
   (FR-017-AC-10).
5. **Witness renderer.** The one function that renders QSL's backend-witness transcript from the
   decoded values, admitted by `qsl_replay::Witness::parse` (FR-024).

Under option 1, the closed `BackendKind` enum stays the registry, and an adapter is reached only
through an exhaustive match on it.

### Q1: Is the adapter contract a Rust trait?

Options:

1. **A trait implemented once per kind, reached by the closed-enum match.** The trait's associated
   types and functions are Q0's five parts. The compiler then checks that every adapter has
   every part, and the enum keeps the kind set closed.
2. **No trait.** Each kind keeps free functions, and the five parts are a documented convention.
3. **Trait objects registered at run time.** This contradicts FR-019's closed kind set.

Recommendation: option 1. It is a refactor with no new behaviour, so no requirement is added for it.

### Q2: Where does the version profile live?

Options:

1. **One constant value in the adapter module.** `KANI_BACKEND_VERSION`, both profile identities
   and `KaniToolPins::pinned` are derived from it, so the version string is written once.
2. **A checked-in data file read at build time.** This adds a reader and a failure mode that no
   other consumer needs.

Recommendation: option 1. Neither option changes behaviour, so this is also a refactor with no
requirement.

### Q3: Who maps a run outcome to QSL's FR-331 terminal record?

QSL ADR-013 O-24 names Contract IR's `KaniOutcomeKind` as the source of that map. Under the owner
ruling, QSL owns the terminal record and Contract IR deletes its copies. CG's own run outcome is
`KaniRunOutcome` (FR-017).

Options:

1. **The CG adapter.** One total match from `KaniRunOutcome` and its inconclusive reasons to
   `qsl_replay::TerminalRecord`, with no catch-all arm.
2. **QSL.** QSL maps from a CG-exported outcome type. This needs a CG → QSL type edge, which QSL
   ADR-013 T-7 and FB-05 exclude.
3. **Contract IR.** IR keeps `KaniOutcomeKind` and its map, and CG converts into it first. That
   keeps an IR copy of a backend outcome after the ruling.

Recommendation: option 1. It needs a QSL ADR-013 O-24 amendment, which is QSL's to make. Once the
owner rules, a requirement is added for it.

### Q4: How does a second backend register?

Proposed procedure:

1. Its descriptor appears in the FR-331 provider manifest. That manifest is QSpec-authored, and QSL
   converts it (QSL ADR-013 T-7).
2. A new `BackendKind` variant is added. The settlement, routed-generation, execution and
   terminal-record matches then fail to compile until each has an arm.
3. The new adapter module supplies the five parts, with real transcript captures of its own under
   `tests/fixtures/<backend>-<version>/`.
4. FR-019, FR-022 and the test matrix gain the kind's rows.

Open within Q4: does each adapter own its own execution evidence schema, or do all adapters share
one schema with a per-backend payload? Recommendation: each adapter owns its own schema identity,
for example `quire.codegen.kani-execution/v1` for Kani. Pins and outcomes differ per backend, and a
shared schema would need an untyped extension point.

## Consequences

FR-024 adds the witness-renderer obligation. Q0 decides whether the other four parts move into
per-kind adapters. Q1 and Q2 change no behaviour, so the spec does not
change for them. Q3 adds a requirement once the owner rules. Q4 becomes FR-019 and FR-022 rows when
a second backend is actually added.

## Alternatives Considered

- **A backend-neutral transcript model shared by all adapters.** Each backend prints different
  facts, so the shared model would be either lossy or a union of every backend. QSL's
  backend-witness transcript is already the one neutral form that matters, and it is the one the
  adapter renders into.
