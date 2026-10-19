---
id: "SR-13131"
title: "CG PR 451 gap analysis: FR-019-AC-10 repeated backend identity"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@e2703b15779aac1585db8d4928c666a3d928d63b; spec/routed/functional/FR-019-capability-settlement.md (AC-10, AC-4, AC-7, Behavior), spec/routed/matrix/tests.md, src/routed/capability.rs, src/routed/generate.rs (RoutedGenerationItem::from_descriptor, generate_routed), tests/it/capability_settlement.rs (diff origin/main...HEAD, base 87db2f7e)"
relationships:
  - type: references
    target: "ix://agent-ix/quire-contract-codegen/FR-019"
---

# SR-13131: CG PR 451 gap analysis

## Summary

Ticket: IR-887 (parent IR-878). PR: agent-ix/quire-contract-codegen#451 at e2703b15.
Plan completion: not assessed. The run is planless and scoped to the PR diff. Semantic review was
run for FR-019-AC-10 only, because the dispatching brief asked for it.

Matrix measurement. The reviewer ran `quire matrix --scope <worktree> --format tsv` (quire 0.36.2,
engine 0.50.2) and `quoin matrix --repo <worktree> --json` (quoin 0.28.3) the same way on
origin/main 87db2f7e and on the PR head:

| | criteria | tagged | method-without-symbol | tagged-by-ignored-test | untagged | backed |
| --- | --- | --- | --- | --- | --- | --- |
| origin/main | 854 | 481 | 48 | 18 | 307 | 529 |
| PR head | 854 | 482 | 48 | 18 | 306 | 530 |

The only status change is FR-019-AC-10, from untagged to tagged, which confirms the author's
529/854 to 530/854 (+1). The quoin evidence status is "no run evidence" for all 854 criteria on
both trees. Zero contradicted therefore holds, but vacuously: there is no evidence store to
contradict anything. The earlier figure of 548/906 for CG main does not reproduce on 87db2f7e with
these tool versions. The difference (52 criteria) comes from a different measurement, either an
older main, another quire/module set, or a mode that counts tagged-by-ignored as backed
(529+18=547). It is not material to the +1 delta.

Reverse gap: the new production code (the duplicate check) is owned by FR-019 (`// Implements:
FR-019`), so there is no unspecified code. No stubs or coverage inflation were found in the diff.

## Verdict

**CONDITIONAL.** FR-019-AC-10 is now tagged, and the tag backs its second clause: a registry
repeating an identity routes nothing in either order. It does not back the first clause, and the
matrix row overstates coverage (FND-001). The refusal semantics are recorded in SR-13130 FND-001
and are not repeated here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-019-AC-10 has two clauses: "Only an item settled `supported` routes" and "a registry repeating a backend identity routes nothing". The tagged test `tc_030_repeated_backend_identity_refuses_in_both_orders` asserts the second, and for the first asserts only the settlement dispositions of single-descriptor manifests (`Supported`, `Unsupported(UnboundedExtent)`). No test shows that an item settled other than `supported` is not routed. Nothing in CG enforces it either: `RoutedGenerationItem::from_descriptor` and `generate_routed` take no settlement. The matrix row still marks AC-10 "✅ Covered" | spec/routed/functional/FR-019-capability-settlement.md:397, tests/it/capability_settlement.rs:1148-1186, spec/routed/matrix/tests.md:14, src/routed/generate.rs:77-101 |

## Coverage

- Criteria examined: FR-019-AC-10 (now tagged, partially backed, FND-001). FR-019-AC-4 and
  FR-019-AC-7 remain tagged and green (47/47 in the capability_settlement filter).
- Strict delta: +1 tagged (FR-019-AC-10). Contradicted: 0, vacuously, because no run evidence
  exists.
- Semantic review: run for FR-019-AC-10 only.
- Plan completion: not assessed

## Dispositions

Round 1, against head 5c07c891b630056da636f5925de496def23b7f75.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | The merged FR-019 amendment (#453) split AC-10. AC-10 is now "Only an item settled `supported` routes.", verified by Analysis against quire-driver FR-001-AC-3 (TC-003) and QSL FR-057-AC-6 (TC-155). The repeat clause moved to AC-45. The PR's tests carry only AC-45 and AC-46 tags, and the matrix row lists AC-10 as Analysis, so the over-claimed AC-10 tag is gone |
