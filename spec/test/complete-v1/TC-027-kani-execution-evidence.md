---
id: TC-027
title: "Verify Kani obligation execution and its evidence"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: verifies
---
# TC-027: Verify Kani obligation execution and its evidence

## Description

Verify that a bounded Kani obligation runs against the installed backend, that
its outcome is read from the backend rather than defaulted, and that the
returned evidence describes the invocation that actually happened.

## Test Procedure

Classification, with the backend's own recorded output as fixtures: a
successful run whose covers are all satisfied; a successful run whose cover is
unsatisfied; a successful run with a partial cover count; a failed run with a
concrete playback for the failed assertion and another for a satisfied cover; a
failed run with only a cover playback; a failed build with no verdict; success
text from a process that exited unsuccessfully; a successful run with no cover
summary, with a zero-total summary, and with an unreadable summary; a failed
unwinding assertion with playbacks present; and a results listing in which an
unwinding check succeeded.

Refusals: request a run against an installation whose launcher is absent.

Routed scalar harness (FR-017-AC-11): build a routed exact-scalar harness with `generate_routed`
(`x + 1` over `Int[0, 9]`) and assemble the crate the way the driver does — the returned
`Cargo.toml` as the manifest and the harness's `rust.contents` as `src/lib.rs`, never the returned
`src/lib.rs`. Run it through `execute_kani_obligation`: a crate whose `src/lib.rs` lacks the harness
is `HarnessNotInCrate`, and the harness's covers classify a run the same way a contract harness's
do. In the `make kani` lane, generated with unwind 3, run it for real and confirm it is `Verified`
and its evidence carries `None` for obligation kind. Then narrow the same harness's checked domain
to `[0, 5]`, below the bound the oracle enforces, and confirm the run is `Falsified` with a
counterexample.

Generation/execution boundary: negotiate obligations that refuse before a harness exists —
unbounded, non-finite, model-dependent, frame and definedness-bearing items, and whole-request
refusals (`KaniObligationError`) such as an empty request, an unparsable subject path, an
out-of-range unwind bound and too many items — and confirm none of them produces a
`KaniObligationHarness`.

In the `make kani` lane, with a real installation: run the precondition, postcondition and
invariant harnesses of a healthy subject; run the postcondition harness against a seeded defect;
run a contract harness whose requires are jointly unsatisfiable; and run a harness against a crate
whose library source does not contain it.

## Expected Results

Only the all-covers-satisfied success is verified. The unsatisfied and partial
covers are cover-unsatisfied with their counts. The failed run with an
assertion playback is falsified carrying that playback and not the cover
playback; the failure with only a cover playback, the build failure, the
unsuccessfully exited process, and the three unreadable cover summaries are
inconclusive with their own reasons. The failed unwinding assertion is
inconclusive as an exhausted bound and not falsified, and the succeeded
unwinding check in a listing is verified.

The absent launcher is a typed tool refusal naming the launcher and its path.

The routed scalar harness's crate missing its `src/lib.rs` source is `HarnessNotInCrate`; its
cover classifies identically to a contract harness's; and, in the `make kani` lane, it runs with
its evidence carrying `None` for obligation kind.

Every generation-time refusal above exposes no harness, so `execute_kani_obligation` — which
takes a `KaniObligationHarness` — has nothing to run for it: a generation-time classification can
never surface as one of FR-017's execution outcomes because no code path converts one into the
other.

In the `make kani` lane the three healthy harnesses are verified, each with exit code zero and an
argument vector equal after the subcommand to its harness identity's options; the seeded defect is
falsified with a concrete counterexample naming its harness symbol and a nonzero exit code; the
jointly unsatisfiable contract is cover-unsatisfied rather than verified; and the crate that does
not contain the harness is refused with no run.

## Implementation

`src/kani_execution.rs` unit tests for classification, and `tests/it/kani_obligations.rs` for the refusals, the
generation/execution boundary and the `make kani` lane. The lane is `#[ignore]`d and runs through
`make kani` under a host-wide lock, because Kani and CBMC are memory-heavy and must run one harness
at a time.

## Blocked

- Timed-out runs: the run carries a caller-declared wall-clock budget and a timed-out state is
  observable (agent-ix/quire-contract-codegen#58, with its own hermetic unit coverage in
  `src/kani_execution.rs`). No test in this repository exercises a timed-out state under
  FR-007-AC-3: that criterion names the corpus path's own `KaniOutcomeKind` vocabulary, distinct
  from this module's `KaniInconclusiveReason`, which FR-017-CON-2 forbids converting between. No
  case is written here because FR-017's own acceptance criteria (AC-4, AC-5) enumerate the
  inconclusive reasons they cover by name, and timed-out is not among them; adding it is
  agent-ix/quire-contract-codegen#55.
