---
id: SR-13030
title: "IR-878 CG PR 449 spec review: FR-042 re-homed checked-invariant cause criterion"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@f69874b0a8d63bf6f2384656961ee15c105de642 vs origin/main 4fb3402; spec/oracle/functional/FR-042-generated-checked-invariant-causes.md, spec/oracle/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-878 (item 4, owner allocation of unbacked strict-coverage criteria). The PR adds CG FR-042 with one criterion, FR-042-AC-1, re-homed from quire-exact FR-369-AC-7. The FR-042-AC-1 text is byte-identical to FR-369-AC-7 on quire-exact origin/main `b83cab9`. The companion quire-exact PR #56 deletes exactly that row and links back to `ix://agent-ix/quire-contract-codegen/FR-042`. The criterion text is not weakened. The FR id is the next free one: FR-041 is the highest on main, and no other open CG PR adds FR-042.

CG owns the producer half of the criterion. Its generated Boolean, equality and function oracle bodies and the pre-check placeholder construct `rt::Refusal::CheckedInvariant`. There are 19 such sites: 4 in `src/oracle/boolean_v1.rs`, 8 in `src/oracle/function/mod.rs` (including the placeholder at line 2384) and 7 in `src/oracle/equality/mod.rs`. These match the 19 site citations in FR-369's CG table. Status is PLANNED, which is honest because no test is credited.

## Examined criteria

- FR-042-AC-1 (examined): "CG's native Boolean, equality, function and pre-check placeholder constructors use the exact nine CG-owned causes in the table, retaining `IllTypedCause` from `check_type` and `check_equality`; downstream replay consumers preserve the typed payload or match it explicitly, retain charge/failure behavior, and accept neither the former unit constructor nor an invented default."
- FR-042 Description/Behavior statements (examined).
- quire-exact FR-369-AC-7 on origin/main (context_only, origin of the text).
- CG FR-018-AC-8 and FR-018-AC-17 (context_only): they still assert the unit `Refusal::CheckedInvariant`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-042-AC-1 also requires that "downstream replay consumers" keep the typed payload, but FR-042 never says which consumers those are or which repo owns them. CG `src/replay` has no `CheckedInvariant` match at all. The live consumer is QSL-owned: qsl-replay `src/scalar.rs:617` matches `Refusal::CheckedInvariant { cause }`. So under the owner-allocation rule, part of the criterion names code outside CG. FR-042's Behavior prose should name the CG-owned consumer scope and point to QSL for the rest. The AC text itself stays verbatim | spec/oracle/functional/FR-042-generated-checked-invariant-causes.md:25,31 |
| FND-002 | low | FR-042-AC-1 says "the nine CG-owned causes in the table", but FR-042 contains no table; the table is the second table of quire-exact FR-369. The Behavior section names FR-369 and Dependencies links to it, so a reader can resolve it, but only indirectly | spec/oracle/functional/FR-042-generated-checked-invariant-causes.md:31 |
| FND-003 | low | FR-042 has no relationship to, or note on, CG FR-018/FR-021. FR-369 says those must migrate generated constructors and replay consumers. FR-018-AC-8 and FR-018-AC-17 still name the unit `Refusal::CheckedInvariant` that FR-042-AC-1 forbids, so an implementer meets two CG criteria that conflict, with no pointer between them | spec/oracle/functional/FR-042-generated-checked-invariant-causes.md:5-7 |

## Verdict

**PASS with findings.** The text is preserved, CG owns the producer code path, the id and status are correct, and `quire validate` is clean. FND-001 needs a clarification of the ownership boundary in the Behavior prose, without changing the AC text, before this allocation closes the IR-878 owner-allocation item cleanly.

## Dispositions

Disposition pass 1, against head 5d0c8f3c2f5731a7044e9ed4d30fef294683616f (fix commit 386aa47f). The FR-042-AC-1 text is still byte-identical to the FR-369-AC-7 row that quire-exact PR #56 head c58ab92 removes.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Behavior now states that CG owns the generated Boolean, equality, function and pre-check placeholder constructors and the payload they emit, and that QSL `qsl-replay/src/scalar.rs` owns the downstream consumer, tracked at that owner; the AC text is unchanged |
| FND-002 | fixed | Behavior now names "the second cause table of quire-exact FR-369", with an ix link; that table is present at quire-exact PR #56 head c58ab92 |
| FND-003 | fixed | Dependencies now link FR-018 and say that its AC-8 and AC-17 still name the former unit constructor and must align with FR-042-AC-1; FR-021's criteria name no unit CheckedInvariant |
