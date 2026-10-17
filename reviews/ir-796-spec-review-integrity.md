---
id: SR-12004
title: "spec-review/integrity review of IR-796 TC-050 proof-bound disposition"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; spec/oracle/matrix/TC-050-caller-text-admission.md, spec/oracle/functional/FR-035-caller-text-admission.md, spec/oracle/matrix/tests.md, spec/kani/matrix/tests.md @ f9164277dccc"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/TC-050
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-035
    type: references
---
# SR-12004: spec-review/integrity, IR-796 disposition (CG #412)

## Summary

Ticket: IR-796. PR quire-contract-codegen#412 at f9164277dcccf08c03d2a7d23adbae4aaa6b5fde, diff against codex/ir-494-caller-text-admission.

Internal consistency of TC-050 and FR-035 after the amendment. The new section contradicts the unchanged proof-boundary paragraph, and the next-owner slice it names is not tracked.

## Scope

- examined: TC-050 Proof Boundary and Current Disposition (spec/oracle/matrix/TC-050-caller-text-admission.md:90-92): IR-796 owns the next source-backed proof design or a separately reviewed spec disposition; neither an inconclusive run nor this status note establishes impossibility.
- examined: TC-050 Measured Proof-Bound Disposition (next owner) (spec/oracle/matrix/TC-050-caller-text-admission.md:148-155): The next owner is Contract Runtime: the proof cost of the exact Text admission and metering path (`prepare` scalar counting, `Meter::charge` and `Meter` equality) has to come within the published ceiling under `--unwind 8` before CG can regenerate the twelve harnesses and the mutation control. That change is a separate runtime slice, followed by a CG generator slice that rebinds TC-050 step 3 to the reduced runtime. A Kani or CBMC change that keeps the concrete payload length constant through `Box<str>` construction is an alternative upstream route. Neither route alters this test's criteria.
- examined: Oracle matrix FR-035 row (spec/oracle/matrix/tests.md): AC-4/5 are unrun and have no real-Kani proof credit. IR-796 measured that no criterion-preserving harness shape fits the published 1 GiB / `--unwind 8` bound, because one concrete `admit_text` call already exceeds it. TC-050 records the disposition and names Contract Runtime as next owner; twelve verified harnesses and the falsifying mutation control remain open.
- examined: Kani matrix FR-035 row (spec/kani/matrix/tests.md): IR-796 measured that per-literal, partitioned and byte-table shapes all exhaust the published bound, so TC-050 records a proof-bound disposition with Contract Runtime as next owner.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | TC-050:90-92 still says 'IR-796 owns the next source-backed proof design ... neither an inconclusive run nor this status note establishes impossibility', while the new section at 140-155 records a measured impossibility and a different next owner. Failure scenario: a reader of the normative boundary paragraph routes the work back to IR-796, which is already Done. | spec/oracle/matrix/TC-050-caller-text-admission.md:90-92 | wrong-requirement |
| FND-002 | medium | The disposition names a 'separate runtime slice' and a 'CG generator slice' but cites no ticket for either. No tracked issue was found (ix-board search 'TC-050' and 'IR-796'). AC-2 asks for an explicit next owner, and an untracked slice has no owner in practice. Failure scenario: IR-796 is closed, no ticket exists, and TC-050 stays planned with nobody assigned. | spec/oracle/matrix/TC-050-caller-text-admission.md:148-155 | missing-requirement |

## Verdict

Changes requested: reconcile TC-050:90-92 with the disposition and cite the tracked follow-up tickets.

## Dispositions

Round 1, reviewed at b94bdd4e7f8d15e1d317391ac0fe07fd93bc9a02.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | still-open | IR-837 (CG, Backlog) is now cited, which closes the CG half. The quire-exact half is still untracked. TC-050:137 and 153-155 name IR-705 as owner, but IR-705 is Done, and it has no AC-9: its acceptance list has four items. The open obligation that matches 'AC-9' is quire-exact FR-359-AC-9 (PLANNED/UNRUN), not an IR-705 criterion. IR-837 itself says the quire-exact fix is 'IR-705 option (c), awaiting Peter'. Still needed: cite an open quire-exact ticket (or reopen IR-705), and replace 'IR-705-AC-9' with quire-exact FR-359-AC-9 or the real criterion. |

Round 2, reviewed at bed849da9fbc39b9fcef4a4e654e6362d2baae65 (IR-705 reopened, In Progress).

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | fixed | - |
