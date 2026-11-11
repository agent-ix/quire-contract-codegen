---
id: SR-17940486709564847047680
title: "IR-1502 PR #786 FR-014 IEEE sqrt and fma spec dependency review"
type: SpecReview
analysis: dependency
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #786 @ 06dcdaf1; IR-1502; spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/tests.md; context spec/kani/functional/FR-053-float-scalar-harnesses.md, spec/kani/matrix/TC-067-float-scalar-harnesses.md, spec/routed/matrix/TC-030-capability-settlement.md, spec/spec.md, spec/kani/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-053
    type: reviews
---

## Summary

Ticket: IR-1502. The ticket says IR-1502 is FR-053's prerequisite P3 and blocks the sqrt/fma part of IR-260. The ticket has no comments. To find statements made stale by moving sqrt/fma to FR-014's derivable list, `spec/` was searched for the whole words sqrt, fma, SquareRoot and FusedMultiplyAdd, and for OperationNotDerivable. FR-047 has no IEEE or sqrt/fma statement. The prerequisites in the diff are named with ticket numbers: IR-1502, IR-842 (QSL value-authority agreement) and FR-053/TC-067 (the Kani row). The PR adds no new `relationships:` edges.

## Verdict

FAIL ON MEDIUMS. Most mentions elsewhere describe what happens until IR-1502 is delivered, and they become true when the code lands. Those can wait for the code PR:
- TC-030:398 'stay `OperationNotDerivable` until then'
- TC-067:16, 69 and 145
- spec.md:93
- spec/kani/matrix/tests.md:13
- FR-053 Status (386-393)
- FR-053-AC-6 'without P3'

One statement describes FR-014's requirement rather than its implementation, and this PR makes it false at merge, so it must change in this PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-053 Dependencies says that FR-014 'owns the IEEE oracle semantics and the derivable operation list (add, sub, mul, div, the three comparisons and the two width conversions)' and that 'It refuses `sqrt` and `fma` as `OperationNotDerivable`, which is why P3 is needed'. The P3 bullet (FR-053:79-85) likewise says that 'FR-014 refuses these four identities as `OperationNotDerivable` today'. After this PR, FR-014's requirement puts sqrt and fma on the derivable list, so the two specs contradict each other at merge. Update it in this PR: FR-014 lists them as derivable (FR-014-AC-72..74, planned under IR-1502), and only the code still refuses them. | spec/kani/functional/FR-053-float-scalar-harnesses.md:366-369 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-053 Dependencies (FR-014) | examined | spec/kani/functional/FR-053-float-scalar-harnesses.md:366-369 | FR-014 owns the IEEE oracle semantics and the derivable operation list (add, sub, mul, div, the three comparisons and the two width conversions); its oracle agreement is not this proof. It refuses `sqrt` and `fma` as `OperationNotDerivable`, which is why P3 is needed. |
| FR-053 P3 prerequisite | examined | spec/kani/functional/FR-053-float-scalar-harnesses.md:78-85 | P3 (ticket IR-1502) is the FR-014 extension that derives and generates an oracle for `quire.op.ieee.float32.sqrt`, `.float64.sqrt`, `.float32.fma` and `.float64.fma`. FR-014 refuses these four identities as `OperationNotDerivable` today, so no oracle exists to prove. |
| FR-053-AC-6 | examined | spec/kani/functional/FR-053-float-scalar-harnesses.md:354 | A `sqrt` or `fma` item without P3 keeps `OperationNotDerivable`. |
| TC-030 IEEE-float row | examined | spec/routed/matrix/TC-030-capability-settlement.md:398 | `sqrt` and `fma` also need the FR-014 derivation of IR-1502 and stay `OperationNotDerivable` until then |
| TC-067 prerequisites | examined | spec/kani/matrix/TC-067-float-scalar-harnesses.md:16 | and, for `sqrt` and `fma`, the FR-014 derivation of IR-1502; none is delivered |
| TC-067 step (no P3) | examined | spec/kani/matrix/TC-067-float-scalar-harnesses.md:69 | Present `sqrt` and `fma` without the FR-014 extension and assert ... |
| spec.md FR-053 row | examined | spec/spec.md:93 | `sqrt` and `fma` also on IR-1502 |
| TM FR-053 row | examined | spec/kani/matrix/tests.md:13 | and `sqrt`/`fma` the FR-014 `sqrt`/`fma` derivation (IR-1502) |
| TM FR-014-AC-72..74 row | examined | spec/oracle/matrix/tests.md:24 | Planned (IR-1502); `ieee.float32/64.sqrt` and `.fma` are `OperationNotDerivable` today; the TC-024 section has not run |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | verified at be0daf1b |
