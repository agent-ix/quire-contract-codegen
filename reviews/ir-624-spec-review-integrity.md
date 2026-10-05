---
id: "SR-1541"
title: "IR-624 integrity analysis (CG #285)"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@82ba95ff370dc5b6caac85d1623f7764a0f2ac1d; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, spec/tests.md (PR #285 diff against main 13fc2d0)"
---

# SR-1541: IR-624 integrity analysis

## Summary

Ticket: IR-624. Checked: numbering, the matrix and test-case rows, the strict delta, edits to
merged criteria, and the code the renamed causes imply.

Measured with quire 0.36.1 (engine 0.50.1):

- `quire coverage --strict` reports 44 unbacked and 0 contradicted on both main 13fc2d0 and head.
- Totals go from 483 to 491 and criteria from 440 to 448. FR-015 goes from 45/76 to 45/80 and
  FR-024 from 20/30 to 20/34. All as claimed.
- `quire validate 'spec/**/*.md'` gives 6 warnings and 0 errors on both, with the same set.
- No collision: FR-015-AC-77 to AC-80 and FR-024-AC-31 to AC-34 appear on no other remote
  branch, no open PR (#286, #222, #209) and not on main.
- `git merge-tree` of #285 with #286 (the AC-10 fault halves, which also edit FR-024's Current
  state and kani tests.md) is clean.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new FR-024-AC-31 to AC-34 coverage row sits after the blank line that ends the Functional table, so it is not part of the table. Measured: a nonsense status in that row draws no diagnostic, while the same edit in the AC-20 row and in the orphan row with the blank line removed each draws "classes as nothing". These four criteria have no functional coverage row that quire reads. Delete the blank line. | spec/replay/matrix/tests.md:19-20, FR-024-AC-31 |
| FND-002 | medium | Merged criteria marked ✅ Covered are rewritten in place to describe the code after IR-624. FR-015-AC-63 now names `ConflictingFieldReads` (covered "through a negotiated request") and `FieldNotRead`, and neither variant exists in `BoundNotResolvedCause` (src/kani/generate/outcome.rs:679-699). FR-024-AC-15 points at a `state_fields` input that `StateClauseReplayInputs` does not have. The kani tests.md AC-61..65 row stays ✅ Covered and still names `MemberAbsent`. Strict reports 0 contradicted only because no test row is checked against the text. Keep the merged text until the code lands (AC-78 and FR-024-AC-31 already carry the new behaviour), or mark the edited criteria planned. The code change then owes: rename `MemberAbsent` to `FieldNotRead`, drop `ValueNotReference`, add `ConflictingFieldReads` with both nodes, and redocument `UnboundedType.target` as the `result_type`. That touches the mapping arms (outcome.rs:849-850), the AC-66 mapping-test rows (outcome.rs:1104, 1108), the engine-cause test and the AC-63 negotiated case for `ValueNotReference` (tests/it/kani_obligations_state_frame.rs:1143, 1681), the `Member::Literal` fixture, the public re-export in lib.rs:150 (an API break), and the retirement of tc_035_the_generator_reads_no_field_range_from_the_object_shape_qsl_emits. | FR-015-AC-63, FR-015-AC-61, FR-024-AC-15, spec/kani/matrix/tests.md:24 |

## New findings (disposition pass 1)

Reviewed at 2fac5ac.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | The GATED marking is inconsistent and leaves trust-dependent criteria closable. FR-015's prose says AC-77 to AC-80 are GATED, but the AC-78 row carries no GATED and the kani matrix gates only AC-77, AC-79 and AC-80. AC-78's last sentence, AC-81 and FR-024-AC-35 each generate a harness from the emitted package, with the clause field's range read from a read's result_type. By FR-024's own rule ('the criteria that read a range from an emitted package are GATED') they must be gated too. Also, GATED limits closing a criterion, not shipping the model-declaration path. Say whether the IR-624 code may emit harnesses from that path before IR-627 or IR-628 (an accepted, stated risk) or must refuse it until then. | FR-015-AC-78, spec/kani/functional/FR-015-bounded-kani-obligations.md:451, 647, 650; spec/kani/matrix/tests.md:33; spec/replay/functional/FR-024-counterexample-envelope-intake.md:321, 386-414 |
| FND-004 | low | FR-024-AC-32 requires that a source scan of tests/ finds no Twin::aligned. The merged, Covered FR-024-AC-30 says its test rebases with Twin::aligned, and that test does so today. Implementing AC-32 makes AC-30's text false. Say that AC-30 (and its test) is edited or superseded when AC-32 lands. | FR-024-AC-32, spec/replay/functional/FR-024-counterexample-envelope-intake.md:316, 318 |

## Dispositions

Round 1, reviewed at 2fac5ac6394887b1d617c64350d7886e057f2e4c.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 2fac5ac: Re-measured: a nonsense status in the AC-31..35 row now draws the classes-as-nothing diagnostic, so the row is inside the table. |
| FND-002 | fixed | 2fac5ac: FR-015-AC-27, AC-61, AC-63 and AC-29 and FR-024-AC-13, AC-15 and AC-23 are byte-identical to main 13fc2d0. No cause is renamed. The AC-66 row's rename note is removed. FR-024-AC-30 gains only a pointer to AC-34. |
| FND-003 | still-open | Round 2 (8fc949f). Most of it is fixed. Every AC row FR-015-AC-77 to AC-81 and FR-024-AC-31 to AC-35 says GATED on IR-627 or IR-628. Both matrix rows say all are GATED. TC-025 steps 37 to 41 are GATED. FR-015's prose states that the code shall not emit from the model declaration path until IR-627 lands or IR-628's accessor is in use. IR #295 is OPEN and holds AC-123 to AC-127 as planned, as cited. Two stragglers remain. (1) TC-035 step 32 (FR-024-AC-35) is the only TC-035 IR-624 step without 'GATED on IR-627 or IR-628'. (2) The gate is 'IR-627 or IR-628', but FR-015's prose and TC-025's closing paragraph also say the code change 'is ordered after IR-627 and IR-628'. Fix: mark step 32, and make the ordering 'or'. |
| FND-004 | still-open | Round 2 (8fc949f). FR-024-AC-32, the FR-024 matrix row and TC-025 now say that the covered FR-024-AC-30 test is edited or superseded when AC-32 lands. Good. But the same sentences claim 'AC-30's text stays as merged', and that is false. AC-30 differs from main 13fc2d0 by an added clause: 'state); FR-024-AC-34 (planned, IR-624) states the case.' in place of 'state).'. Revert that clause so AC-30 is byte-identical to main, or drop the claim. |
| FND-003 | fixed | 1b05c69 (round 3): TC-035 step 32 is now GATED. 'IR-627 or IR-628' is used everywhere: no 'IR-627 ... and IR-628' remains in spec/. The planned order (IR-627 first, then IR-628) is a separate note in FR-015 and TC-025. |
| FND-004 | fixed | 1b05c69 (round 3): Diffed the FR-024-AC-30 row against main 13fc2d0: byte-identical. The 'AC-30's text stays as merged' claims in FR-024-AC-32 and the replay matrix row are now true. |
