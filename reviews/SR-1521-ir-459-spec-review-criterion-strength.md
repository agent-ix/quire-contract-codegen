---
id: SR-1521
title: "criterion-strength review of quire-contract-codegen#280 (FR-024-AC-1, AC-20..30)"
type: SpecReview
analysis: criterion-strength
scope: "agent-ix/quire-contract-codegen@70f5e08ac399d0880b50c1610592220c15298e09; spec/replay/functional/FR-024-counterexample-envelope-intake.md FR-024-AC-1, FR-024-AC-20..30; spec/replay/matrix/TC-035-counterexample-envelope-intake.md steps 18-27 (diff only)"
review_set: subset
---

## Summary

Ticket: IR-459. I judged whether each new or edited AC can fail, reading the spec text against
main's code. I did not use Jev; this is a manual reading.

These criteria are direct, failable assertions with adverse cases: AC-1, AC-22, AC-23, AC-25
(once its row parses, see SR-1518 FND-001), AC-26, AC-27, AC-28 and AC-29.

- AC-26 tests the range endpoints and the values one outside them.
- AC-27 replays with the playback's own pre state and with another pre state, and it closes
  SR-647 FND-001 from #209.
- AC-28 checks the operation before `call_site`, which closes SR-647 FND-002.

AC-30's adverse half cannot be met (SR-1518 FND-002). TC-035 steps 18 to 27 are concrete and
testable, apart from step 27's granted half. The weak clauses are listed below.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-024-AC-20 says "the source of `src/replay/obligation.rs` names none of ... `sha2`", but it also requires a hand-written-vector test. That test exists today inside that file's `#[cfg(test)]` module, and it uses `sha2::{Digest, Sha256}` (`src/replay/obligation.rs:343`, `tc_026_arguments_are_ordered_by_identifier_not_by_node_id`). As written, the source check fails on the AC's own vector test unless that test moves, and the AC does not say it must. The clause should read "non-test source" or name the test's new home. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:286 |
| FND-002 | low | FR-024-AC-24's scan for "a repeated-byte digest literal (`[1; 32]`, `[2; 32]`, `[3; 32]` or the like)" has two gaps. "Or the like" is unbounded. "Non-test source under `src/`" leaves `#[cfg(test)]` modules inside `src/` undecided: `src/oracle/equality/mod.rs:2171` has `NodeKey::from_bytes([1; 32])` inside one. The scan is a weak proxy. The AC's behavioural clauses already make it failable: request and envelope both equal the minted identity, and both change with a grant. Bound the pattern and state the cfg(test) rule, or drop the scan. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:290 |
| FND-003 | low | In FR-024-AC-21, "two operations of one object whose frames are equal text have different identities" holds even with no `anchor` member, because FR-104 gives the two operations different frame occurrences. "The anchor changes it alone" can only be shown with synthetic inputs, since in a real compile the anchor and the frame occurrence change together. So no criterion shows that the anchor member does anything (see SR-1519 FND-001). | spec/replay/functional/FR-024-counterexample-envelope-intake.md:287 |

## Dispositions

Round 1, reviewed at b094aaca23d924831ee362d8865d31b74acbb753.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | b094aac |
| FND-002 | fixed | b094aac |
| FND-003 | fixed | b094aac |
