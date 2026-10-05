---
id: "SR-1542"
title: "IR-624 criterion-strength analysis (CG #285)"
type: SpecReview
analysis: criterion-strength
review_set: subset
scope: "agent-ix/quire-contract-codegen@82ba95ff370dc5b6caac85d1623f7764a0f2ac1d; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, spec/tests.md (PR #285 diff against main 13fc2d0)"
---

# SR-1542: IR-624 criterion-strength analysis

## Summary

Ticket: IR-624. I judged the eight new criteria (FR-015-AC-77 to AC-80, FR-024-AC-31 to AC-34)
and their mutation rows on whether each can fail. Each criterion asserts concrete observable
values: ranges 0..1000, the five causes, `state_fields` order, scope ids equal to call_site's,
and the settlements. None is vacuous. FR-015-AC-77's "different body range" clause is strong
but asserts the unsound behaviour in SR-1540 FND-002, so it is not repeated here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Three mutation rows name equivalent mutants that the criterion cannot detect. "Rebase the scope onto the ids call_site names" (FR-015-AC-79, FR-024-AC-32) and "keep Twin::aligned" (FR-024-AC-34) do nothing to a harness generated from the emitted package, whose ids already are call_site's, so every test still passes. FR-024-AC-34's distinguishing clause ("no Twin::aligned call in either test") is a constraint on test code, checkable only by inspection. Replace these with mutations that change behaviour, such as generating from the hand-built fixture (already listed) or emitting the fixture's anchor id. Make the no-alignment requirement an assertion: the generated identity's scope equals call_site's before any replay. | FR-024-AC-34, FR-015-AC-79, FR-024-AC-32 |

## New findings (disposition pass 1)

Reviewed at 2fac5ac.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | FR-015-AC-77's last clause ('over an emitted package whose object body is made non-empty, the reads are not used in place of the body') cannot be exercised. Reproduced: the emitted package with a member added to the model object's body, package_id recomputed, is refused by IR (invalid_package/stale-node-key at the object). No admitted CheckedPackageV2 has that shape, and field_range only receives admitted packages. Drop the clause, or restate it over a non-model object, which AC-78's MemberDisagreesWithRead already covers. | FR-015-AC-77, spec/kani/functional/FR-015-bounded-kani-obligations.md:646 |

## Dispositions

Round 1, reviewed at 2fac5ac6394887b1d617c64350d7886e057f2e4c.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 2fac5ac: The equivalent mutants are replaced ('emit the fixture's anchor or frame id'). The scope is asserted on the generated identity before replay, and a source scan for Twin::aligned is added. |
| FND-002 | fixed | 8fc949f: The unreachable clause is dropped from AC-77 and from TC-025 step 37. The AC-77 mutation row still holds: 'trust a read's result_type for an object with body members' is caught by AC-78's MemberDisagreesWithRead. |
