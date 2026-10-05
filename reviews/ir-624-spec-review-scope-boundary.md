---
id: "SR-1543"
title: "IR-624 scope-boundary analysis (CG #285)"
type: SpecReview
analysis: scope-boundary
review_set: subset
scope: "agent-ix/quire-contract-codegen@82ba95ff370dc5b6caac85d1623f7764a0f2ac1d; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, spec/tests.md (PR #285 diff against main 13fc2d0)"
---

# SR-1543: IR-624 scope-boundary analysis

## Summary

Ticket: IR-624. Checked the CG, Contract IR, QSL and QSpec boundary the design rests on. Two
things are right. CG reads no domain document and re-implements no resolution, which matches
the no-copy rule. The ask for a typed IR accessor (Q-5) is the right owner for the field set
and the ranges of unread fields. IR-625 (the state-playback decoder) should take its draw order
from the same `state_fields` that FR-024-AC-31 adds. That is a note for IR-625's spec, not a
defect here. IR-241 touches FR-015-AC-69 to AC-76 only, with no overlap.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Q-5 names no owning ticket and asks IR only for the field set and the ranges of unread fields. The design also depends on two IR properties that IR's contract does not give today. One is that a `bounded_domain` node's bounds are bound to its key (SR-1540 FND-001). The other is that `result_type` is checked only for model declaration nodes (SR-1540 FND-002). The accessor would close both if it returned the bounds as values. Q-5 should say so and cite an IR ticket. The QSL alternative ("emit a read-free declaration of each state field's type") would add a node form that QSpec FR-322 does not define and that model nodes may not carry. It belongs to the QSpec owner, not to QSL. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:432-446 |

## Dispositions

Round 1, reviewed at 2fac5ac6394887b1d617c64350d7886e057f2e4c.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 2fac5ac: Q-5 now cites IR-628, names IR-627 as the admission gap and the value-bounds closure, and routes the read-free declaration to the QSpec owner. |
