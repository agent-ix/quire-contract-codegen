---
id: SR-3103
title: "IR-689 spec review (failure domain): production safety, authority and timing of the feature-only O observation transport"
type: SpecReview
analysis: failure-domain
review_set: subset
scope: "agent-ix/quire-contract-codegen branch spec/ir689-stage2-observation (frozen head named in the Linear marker); spec/kani/functional/FR-034-caller-death-ownership.md section 'Feature-only stage-2 observation transport' (lines 1453-1549) and AC-78..AC-82; spec/kani/matrix/TC-049-caller-death-ownership.md step 29; context: FR-034 opt-in fixture section (lines 512-541, 686-707), stage-2 section (lines 1262-1361), AC-24, AC-27, AC-29, AC-30; newer published guardian review-source backup ref (head commit 'Retain producer clock failure with borrowed outer setup custody'): Cargo.toml features (default empty, guardian-test-support empty), README feature section, src/kani/run/fixture.rs scenarios, fixture_execution.rs run, owned.rs cfg-gated fixture module, namespace.rs Command-based spawning; Linear IR-649 state (Backlog)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: references
---

## Summary

Ticket: IR-689. This pass looks for unstated failure modes, authority leaks, identity confusion and timing edges in the new transport, measured against the published guardian review source.

Recorded clean:

- **Default features.** In the guardian source, Cargo.toml has `default = []` and an empty `guardian-test-support`. Existing AC-27 and AC-29 require off-by-default, separate feature-off and feature-on invocations with no self dev-dependency unification, a feature-off compile check that the operation is absent, and mismatch refusal in both directions before Dispatch. A feature-off L/O receiving a feature-on binding envelope fails closed at bootstrap.
- **Reader influence on O.** C holds only the read end. O never waits on an ACK, pause or permission (FR-034:1535-1537), and a full pipe is a fixture failure, not a stall. Reading cannot pause, extend or kill O.
- **Child exclusion.** O starts M and I through `Command` (fork plus exec) in namespace.rs, so a CLOEXEC write end does not reach M, I or the backend.
- **Producer binding.** Exclusive transfer of the single write right to the Armed-bound O pin establishes the producer. Reported PIDs and labels are rejected as producer evidence (FR-034:1469-1473).
- **After-the-fact and timing.** Producer ordinals are fixed at the actual boundary, and receipt time or late fill establishes no order (FR-034:1503-1505). Order predicates, not wall-clock windows, judge the mutants, so the extra nonblocking write does not mask or create an order race.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | O's reaction to a write failure is unspecified, and so is the set of scenarios that create the pipe. "Unavailable/full/failed observation shall be a typed fixture failure followed by immediate unchanged cleanup" names no actor. Read as an O-side rule, O starts cleanup or cancellation when its own write fails. C creates the pipe unconditionally for the single operation (FR-034:1456-1457), which in the guardian source also serves ExactDeath scenarios where C self-kills. There the reader is gone, so O's next write returns EPIPE. If that EPIPE causes O-side cleanup, the ignored-inner-lease-EOF mutant, which must "never be ... rescued by outer cancellation" (FR-034:1333-1334), is rescued by an observation-failure cancellation that exists only in feature-on builds. That contradicts "fixture selection shall not change ... cancellation" (FR-034:1351-1352). The spec also leaves O's EPIPE handling dependent on an inherited SIGPIPE disposition it never states. | spec/kani/functional/FR-034-caller-death-ownership.md:1537-1538; spec/kani/functional/FR-034-caller-death-ownership.md:1456-1457; spec/kani/functional/FR-034-caller-death-ownership.md:1333-1334; spec/kani/functional/FR-034-caller-death-ownership.md:1351-1352 |
| FND-002 | medium | The pre-Armed exclusion cannot be checked by the reader. O receives the write right during bootstrap, before Armed (FR-034:1458-1462). C accepts observations "only after" Armed binding (FR-034:1469-1473). A pipe carries no write-time epoch, so a frame O wrote before Armed and a frame written after Armed look the same when C reads them after Armed. The record schema has no Armed-relative field, such as an ordinal floor fixed at Armed. TC step 29a's "unbound-before-Armed observations ... fail the fixture" therefore has no decidable predicate. A genuine pre-Armed write would be silently accepted, and the adverse control can pass vacuously. | spec/kani/functional/FR-034-caller-death-ownership.md:1469-1473; spec/kani/functional/FR-034-caller-death-ownership.md:1490-1491; spec/kani/matrix/TC-049-caller-death-ownership.md:948-949 |
| FND-003 | medium | The feature-on ordinary-execution mode is undefined. The binding and writer are specified only for "the documented single fixture operation". The spec does not say what a feature-on L or O does when ordinary public execution sends no Stage2ObservationBinding. Either it requires the envelope, which breaks the ordinary feature-on execution that the guardian-feature-on invocation still runs, or it treats the envelope as optional and must emit nothing without it. Production exclusion of the feature is owned downstream by IR-649, which is in Backlog with no gate yet. A transitively unified feature-on driver therefore passes the CG mismatch check, because library and helper match. With this slice it would also carry O-side emission branches. The spec should state that the feature-on O emits nothing and has no writer unless an authenticated binding was received. That keeps an accidental feature-on production build behaviourally identical. | spec/kani/functional/FR-034-caller-death-ownership.md:1455-1462; spec/kani/functional/FR-034-caller-death-ownership.md:703-707; spec/kani/functional/FR-034-caller-death-ownership.md:1854 |
| FND-004 | low | Feature-off absence (AC-82) rests only on CODE-author Analysis checked by the CODE reviewer. The existing mechanical feature-off check (AC-29: feature-off consumer compilation verifies absence of the fixture operation) covers only the public operation export. AC-82 does not extend any mechanical feature-off build check to the new L/O-side binding decoder, the O writer branches or the record types in the helper binary. A later edit that drops a `cfg` on an O emission site would compile and pass every feature-off Test until the next manual Analysis. | spec/kani/functional/FR-034-caller-death-ownership.md:1906; spec/kani/functional/FR-034-caller-death-ownership.md:1541-1545; spec/kani/functional/FR-034-caller-death-ownership.md:1853 |

## Verdict

**Changes requested (medium).** Specify O's side of a transport failure: latch it, keep control flow unchanged, ignore EPIPE without relying on an inherited disposition, and let C detect the missing or failed record. Bound the pipe to the stage-2 live scenarios, or state its behaviour under ExactDeath. Give records an Armed-relative field so the pre-Armed adverse case is decidable. State that the feature-on L/O emits nothing without an authenticated binding. Consider a mechanical feature-off check for the helper-side items.
