---
id: TC-027
title: "Verify pinned Kani obligation execution and its evidence"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: verifies
---
# TC-027: Verify pinned Kani obligation execution and its evidence

## Description

Verify that a bounded Kani obligation runs only under the committed backend
pins, that its outcome is read from the backend rather than defaulted, and that
the retained evidence describes the invocation that actually happened.

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

Pins: compare the committed pins with themselves, and with a copy differing in
each of the six fields in turn.

Refusals: request a run against an installation whose launcher is absent; separately, request a run
whose harness identity's pins differ from the committed pins against an installation that does not
exist, so that measuring the backend at all before the identity comparison would surface as an
absent-tool refusal rather than as pin drift.

Aggregate verdict and retained evidence: census `src/kani_execution.rs` — the surface FR-017 owns —
for any function signature that takes more than one run's evidence or outcome, and for any
filesystem write.

Generation/execution boundary: negotiate obligations that refuse before a harness exists —
unbounded, non-finite, model-dependent, frame and definedness-bearing items, and whole-request
refusals (`KaniObligationError`) such as an empty request, an unparsable subject path, an
out-of-range unwind bound and too many items — and confirm none of them produces a
`KaniObligationHarness`.

Under the pinned lane, with a real installation: measure the backend and
compare it with the committed pins; run the precondition, postcondition and
invariant harnesses of a healthy subject; run the postcondition harness against
a seeded defect; run a contract harness whose requires are jointly
unsatisfiable; run a harness against a crate whose `Cargo.lock` is a directory
so the backend runs but the post-run digest read fails; run a harness whose
identity names another driver digest; and run a harness against a crate whose
library source does not contain it.

## Expected Results

Only the all-covers-satisfied success is verified. The unsatisfied and partial
covers are cover-unsatisfied with their counts. The failed run with an
assertion playback is falsified carrying that playback and not the cover
playback; the failure with only a cover playback, the build failure, the
unsuccessfully exited process, and the three unreadable cover summaries are
inconclusive with their own reasons. The failed unwinding assertion is
inconclusive as an exhausted bound and not falsified, and the succeeded
unwinding check in a listing is verified.

Equal pins report no difference; each of the six altered fields is reported as
that field with its expected and observed values. The absent launcher is a
typed tool refusal naming the launcher and its path. A harness identity pin
difference is reported as pin drift even against a nonexistent installation,
proving the backend is measured only after the identity's own pins already
match the committed pins.

The census finds no function taking a collection of runs' evidence or outcomes, and no filesystem
write, in `src/kani_execution.rs`.

Every generation-time refusal above exposes no harness, so `execute_kani_obligation` — which
takes a `KaniObligationHarness` — has nothing to run for it: a generation-time classification can
never surface as one of FR-017's execution outcomes because no code path converts one into the
other.

In the pinned lane the measured backend equals the committed pins; the three
healthy harnesses are verified, each with the measured pins, exit code zero, an
argument vector equal after the subcommand to its harness identity's options,
its oracle digest and a lockfile digest; the seeded defect is falsified with a
concrete counterexample naming its harness symbol and a nonzero exit code; the
jointly unsatisfiable contract is cover-unsatisfied rather than verified; the
harness run against a `Cargo.lock` directory reports evidence — not a refusal —
carrying the backend's own verdict (the healthy subject verified) and no
lockfile digest, because the backend already ran and a lockfile read failure is
missing evidence about that run, never grounds to discard its verdict; the
drifted driver digest is refused as pin drift on that field with no target
directory created; and the crate that does not contain the harness is refused
with no run.

## Implementation

`src/kani_execution.rs` unit tests for classification and pin comparison, and
`tests/kani_obligations.rs` for the refusals, the generation/execution
boundary, the aggregate-verdict/retained-evidence census, and the pinned lane.
The lane is `#[ignore]`d and runs through `make kani` under a host-wide lock,
because Kani and CBMC are memory-heavy and must run one harness at a time.

## Blocked

- Timed-out runs: the run now carries a caller-declared wall-clock budget and
  a timed-out state is observable (agent-ix/quire-contract-codegen#58, closed
  at the code level; traced from `src/kani_execution.rs`'s own unit test as
  TC-023 under FR-007-AC-3). No case is written here because FR-017 itself
  does not yet state timed-out behaviour as a requirement; adding that
  requirement is agent-ix/quire-contract-codegen#55.
