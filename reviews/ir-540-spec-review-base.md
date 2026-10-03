---
id: SR-1004
title: "CG PR 241 spec review: FR-021-AC-22 duplicate declaring node ids"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@b4309b85673304ed2ba9b580847af46e8fce338e; spec/oracle/functional/FR-021-function-application-oracles.md (AC-22, its mutation row, Behavior bullets), spec/oracle/matrix/TC-031-function-application-oracles.md (description, steps 9 and 10, Expected Results), spec/oracle/matrix/tests.md (FR-021-AC-22 row, TC-031 summary row), src/oracle/function/mod.rs (git diff origin/main...HEAD, base 85b8114)"
---
# SR-1004: CG PR 241 spec review

## Summary

Ticket: IR-540. PR: agent-ix/quire-contract-codegen#241 at b4309b8, base 85b8114. Spec-only.

Checked:

- FR-021-AC-22 is a direct assertion, not a "shall" statement. It is marked 🚧 Planned in the
  criteria cell, and the id cell is plain.
- The mutation row is present and names two mutations, one per symptom in the ticket.
- TC-031 old step 9 is now step 10. No spec, test or source file cites "TC-031 step 9". The only
  in-repo step citations are step 1(a) and step 8, and both are unchanged. Inside TC-031, "step 7"
  and "step 1(g)" still point at the right steps.
- tests.md has a new FR-021-AC-22 row marked 🚧 Planned. AC-22 is appended to the TC-031 Traces To
  list.
- `quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/**/*.md'` exits 0, and only
  module-level warnings are printed. `quire coverage --strict` reports 66 unbacked rows and 0
  contradicted statuses at both b4309b8 and base 85b8114, so there is no regression.
- The new variant `ExactFunctionRefusal::DuplicateDeclaringNode { node_id }` is justified. See
  SR-1005 for the code reading.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-021-AC-22 has no Behavior statement. The only duplicate rule in FR-021's Behavior section is item-level: "If one node id appears more than once in the request under one function binding... refuse every copy". No "shall" covers two declarations that share a declaring node id. FR-021-AC-19 to AC-21 each have their own Behavior bullet, but AC-22 asserts behavior that the requirement statement never requires. Add an EARS bullet, for example: "If two or more declared functions share one declaring node id, then the generator shall refuse each with DuplicateDeclaringNode before classification and refuse every item naming one of them with the same reason." | spec/oracle/functional/FR-021-function-application-oracles.md:210 |
| FND-002 | medium | AC-22 does not say which refusal wins when it overlaps the existing name-ambiguity refusal. Case one: two declarations share both a node id and a name. Case two: a name is shared between a duplicate-node declaration and a declaration with its own node id. AC-22 says every item naming such a function carries `DuplicateDeclaringNode`. But `item_disposition` checks `name_counts` first and returns `AmbiguousFunctionName` (mod.rs:1194-1206), and Stage 1 checks the name first too (mod.rs:862). Two implementers would order these differently. `AmbiguousFunctionName` is not named anywhere in the spec, so nothing settles it. State the precedence, and add a TC-031 step 9 fixture that shares both the node id and the name | spec/oracle/functional/FR-021-function-application-oracles.md:248 |
| FND-003 | low | TC-031 Expected Results was not updated for AC-22. The description and step 9 were added, but the Expected Results paragraph still ends at the `Negate` refusal and says nothing about duplicate declaring node ids being refused | spec/oracle/matrix/TC-031-function-application-oracles.md:115 |
| FND-004 | low | The tests.md TC-031 summary row now reads "✅ Covered; FR-021-AC-22 is 🚧 Planned", which names AC-22 as the only planned criterion. AC-15 and AC-18 are also 🚧 Planned in the coverage table. The preamble says that table governs, but the new annotation is incomplete as written. Name all three or drop the annotation | spec/oracle/matrix/tests.md:79 |
| FND-005 | low | The criterion text ends with review provenance, "(SR-880 FND-004)". That is not part of the testable property. It belongs in the tests.md status cell or a note, not in the AC that tests are tagged to | spec/oracle/functional/FR-021-function-application-oracles.md:248 |

## Verdict

Approve after fixes: two mediums and three lows. The AC is well formed, correctly marked Planned,
and backed by a non-vacuous mutation row and a planned TC step (see SR-1008). FND-001, the missing
Behavior statement, and FND-002, the unstated precedence against `AmbiguousFunctionName`, should be
fixed before the coder implements against the AC. Validation passes and the strict-coverage
baseline (66) is unchanged.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | The new Behavior bullet refuses "an item naming a name that any such declaration holds" as `DuplicateDeclaringNode`. It does not say what happens to a third declaration Z that has its own distinct node id but shares that name with one of the duplicate pair. Z is not a duplicate node, and the bullet keeps `AmbiguousFunctionName` only for "a name shared by declarations whose node ids are all distinct". AC-22's baseline clause excludes duplicate names. So nothing says whether Z is refused (with which reason) or enters `checked_package()`. Two implementers could differ. State Z's disposition, and add it to step 9 fixture (iii) | spec/oracle/functional/FR-021-function-application-oracles.md:212 |
| FND-007 | low | The bullet says `UnknownCallee` is "the existing reason for a callee that itself failed to classify (FR-021-AC-12)". AC-12 says nothing about that. It covers only "a nested `call` naming a function absent from the request", and so does the older Behavior bullet. The "failed to classify" wording comes from the `UnknownCallee` rustdoc and code (mod.rs:909-913), not from the spec. The chosen reason matches the code path. Only the citation is wrong: drop "(FR-021-AC-12)", or cite the rustdoc instead | spec/oracle/functional/FR-021-function-application-oracles.md:220 |

## Dispositions

Reviewed at agent-ix/quire-contract-codegen@08a7f0366888bad18e710b6b9287b543fff0db61 (fix-round delta b4309b8..08a7f03). `quire validate` exit 0; `quire coverage --strict` 66 unbacked, 0 contradicted (baseline unchanged).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 08a7f03 |
| FND-002 | fixed | 08a7f03 |
| FND-003 | fixed | 08a7f03 |
| FND-004 | fixed | 08a7f03 |
| FND-005 | fixed | 08a7f03 |
