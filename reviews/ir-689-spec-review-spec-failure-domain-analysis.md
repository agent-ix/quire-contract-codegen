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

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | The retained-gate scenario's binding timing is not allocated. The writer now reaches O only after Armed, through the C/O receive cursor. The receive-state feasibility gate covers only the birth case ("while I lives"). Nothing requires O to accept the binding before the retained-gate cancellation's actual I confirmation and gate close. Nothing establishes that O's gated-startup receive state can consume the new envelope at all. If the cancellation wins the race, InitConfirmed and GateClosed are missing and the fixture fails closed, so the result is not vacuous. AC-54's retained-gate witness, however, then has no specified construction. | spec/kani/functional/FR-034-caller-death-ownership.md:1467-1472; spec/kani/functional/FR-034-caller-death-ownership.md:1499-1503; spec/kani/functional/FR-034-caller-death-ownership.md:1330-1332 |

## Dispositions

Round 1 was re-checked at the branch's round-1 fix head (subject 'Resolve IR689 observation selection and independent evidence obligations'; head named in the Linear marker only). It was checked against the newer published guardian review-source backup ref: C already has a separate pre-encoded C/O phase-send path. The check was static and read-only, and make spec passes.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | fix commit 'Resolve IR689 observation selection and independent evidence obligations': O latches full, EPIPE or other write failure, stops further writes and continues the same production path. Observation failure never causes cancellation, I signalling, early exit or escalation. Proven-safe feature-only SIGPIPE suppression is a CODE gate that no inherited disposition can satisfy (FR-034:1556-1564; AC-90). ExactDeath and ignored-inner-EOF scenarios use no writer (FR-034:1485). |
| FND-002 | fixed | fix commit 'Resolve IR689 observation selection and independent evidence obligations': O has no writer before Armed because C keeps both ends until authenticated Armed and transfer. The harness rejects pre-transfer bytes and checks endpoint transfer history without inferring a write epoch (FR-034:1488-1492; AC-78, AC-83). This is decidable: C holds the read end and can check that the pipe is empty at transfer. |
| FND-003 | fixed | fix commit 'Resolve IR689 observation selection and independent evidence obligations': ordinary feature-on execution without an authenticated binding creates no endpoint and emits nothing, and a missing binding is not a startup failure (FR-034:1483-1485; AC-91). IR-649 stays named as required (FR-034:1590). |
| FND-004 | fixed | fix commit 'Resolve IR689 observation selection and independent evidence obligations': the CODE author shall extend the mechanical feature-off consumer/helper checks to the binding decoder, frame/right mapping and all observer items, beyond public-operation absence (FR-034:1585-1589; AC-82; TC step 29e). |

Round 2 was re-checked at the branch's round-2 fix head: six commits ahead of main, with subject 'Clarify stage-two parent observations and binding prerequisites'. The head is named in the Linear marker only. The check was static and read-only. make spec exits 0, and its only warnings are the two older ones at FR-017 line 174. The computed matrix has 631 records: the 16 additions plus the intended AC-24 text amendment, which stays untagged. No 7+ hex string, local path or timestamp appears in the committed diff.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-005 | fixed | fix commit 'Clarify stage-two parent observations and binding prerequisites': O shall accept and authenticate the binding while the original gate is held, before the genuine cancellation-initiating operation and before InitConfirmed and GateClosed are captured. A completed C send does not prove acceptance. The held-gate receive and the IR-639 bounded cancellation under the existing AC-8, AC-10 and AC-54 obligations are separate PLANNED CODE prerequisites, with no new kill, control, ACK, pause, channel or timer (FR-034:1510-1521; TC step 29f and the AC-83 Expected row). The AC-54 witness construction is now specified. |
