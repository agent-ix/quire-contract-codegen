---
id: SR-1500
title: "Criterion strength review of quire-contract-codegen PR 276 (IR-460 state-clause replay spec)"
type: SpecReview
analysis: criterion-strength
scope: "agent-ix/quire-contract-codegen@d22cc5d849db3edd7bba68c3638bb9c4995ac723; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, spec/assurance/AD-002-cg-qsl-replay-seam.md (PR #276 diff vs origin/main 7345463; QSL qsl-replay locked at c8f0c28)"
review_set: subset
---

## Summary

Ticket: IR-460. Judged by hand whether each of FR-024-AC-11..17 can fail (Jev was not run). AC-12 is failable: equal ClauseSite node and occurrence, and an undeclared name refused by call_site's UnknownClause. AC-13 is failable (changing a playback value changes the pre digest). AC-15 is failable both ways (mutated settles violation with evaluated false; unmutated settles inconclusive with Verdicts), and its names match QSL's WitnessSettlement::ReproducedWithEvaluatedWitness and DisagreementCause::Verdicts. AC-16 is failable at both endpoints. AC-17 is gated honestly as real-Kani and planned, with no backing counted. Two ACs are weak.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-14's "typed `Incomplete` result naming that field" names no type. QSL's Incomplete (TerminalValue::Incomplete with IncompleteCause TimedOut, Cancelled or ResourceExhausted, and FR-331's IncompleteInput under declined) names no field, and FR-029 reads Incomplete as a resource outcome. A playback of CG's own generated harness that lacks a declared field is a CG defect, by FR-029's rule for a playback outside the proof bound. Two implementers would build different types and settle them differently, and a test can pass against either. | FR-024-AC-14, FR-029 |
| FND-002 | low | AC-11's second sentence ("a stub in place of replay_state_clause changes the result and fails the test") is a claim about test design, with no seam to put a stub in. The function path has replay_counterexample_through for this, and the state-clause path has no counterpart in the spec. A stub that returns QSL's own result would not fail, so the clause is only as strong as an unspecified executor injection. | FR-024-AC-11, TC-035 step 9 |

## Dispositions

Round 1, reviewed at 02a5f67616fed74c0c014d546e2938e555791968.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 02a5f67616fed74c0c014d546e2938e555791968: FR-024-AC-15 returns StateClauseReplayError::MissingField naming the field, and FR-029-AC-16 maps it to Failed, explicitly never Incomplete. |
| FND-002 | fixed | 02a5f67616fed74c0c014d546e2938e555791968: FR-024-AC-11 now uses the replay_through seam. A sentinel executor's value must come back as is, and replay must equal a direct replay_state_clause call for both a violating and a respecting run, with the two results differing, so a composed or constant verdict fails. |

Round 2, reviewed at 6df9b318b612af8b7544f34e8c2d9e87ce65590c.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-003 | fixed | 6df9b318b612af8b7544f34e8c2d9e87ce65590c: FR-024-AC-14 now compares the builder's bytes and digest with core::canonical's over a vector with unsorted members, JSON escapes, a large integer, a fractional number and an exponent form; quire-canonical encodes all of these to RFC 8785. A hand-rolled encoder diverges on at least the number forms. The source ban adds ByteDigest::of, member sorting and hand-written escaping. |


## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | FR-024-AC-14 cannot fail for the defect it targets, a second encoder. FR-106 snapshot integers are JSON strings ({"integer": "<decimal>"}, SnapshotValue::Integer(String)), so the "integer above 2^53" probe exercises no number canonicalisation. The source assertion bans only quire_canonical's encoder functions and sha2. A hand-rolled sorted-key encoder (like native_twin's canonical()) hashing with qsl_replay::ByteDigest::of, which qsl-replay re-exports as SHA-256, passes every clause. Assert that the bytes and digest come from core::canonical's functions, or ban every other encoder and hash in the module. | FR-024-AC-14, TC-035 step 12 |
