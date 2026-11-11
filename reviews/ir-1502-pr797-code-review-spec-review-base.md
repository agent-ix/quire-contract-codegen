---
id: SR-17940486709564847047697
title: "IR-1502 PR #797 spec-review base (status and evidence edits)"
type: SpecReview
analysis: base
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen PR #797 at 8c0b9508 (base main 6e62186e); IR-1502; spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/tests.md, spec/kani/functional/FR-053-float-scalar-harnesses.md, spec/kani/matrix/TC-067-float-scalar-harnesses.md, spec/kani/matrix/tests.md, spec/routed/matrix/TC-030-capability-settlement.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-053
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-024
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-067
    type: reviews
---

## Summary

Ticket: IR-1502. This PR's spec edits are status and evidence only. The AC text of FR-014-AC-72..80 is unchanged from merged CG #786 apart from the removed `PLANNED (IR-1502).` prefixes.

Edits reviewed:
- FR-014 derivable-list prose and Behavior bullet: planned markers removed.
- FR-014 refused-identity list: the "until IR-1502" sentence dropped. It still lists `ieee.float32/64.negate`, `from_exact` and `to_rational`.
- TC-024 IR-1502 section: "planned" removed, test locations named.
- spec/oracle/matrix/tests.md: AC-72..80 row now reads Delivered.
- FR-053: P3 delivered, P1 and P2 still open.
  - Status: P2 undelivered, the sqrt/fma oracle exists but is unproved, all fourteen criteria still planned and untagged.
  - Dependencies and AC-6: a sqrt/fma item now derives and, without P1, settles as the TC-030 IEEE-float row.
- TC-067: step 1 and Status.
- spec/kani/matrix/tests.md: FR-053 row.
- TC-030 IEEE-float row: sqrt/fma derive under the delivered FR-014 derivation and settle `OperationNotRendered`. This is consistent with src/routed/capability.rs:612, which maps every `IeeeArithmetic` to `KaniFormFamily::IeeeFloat`.
- spec/spec.md registry row.

These statements match the code:
- P3 is delivered and P1 (IR-1446) and P2 (IR-69) remain open.
- FR-053 stays PLANNED and untagged.
- No Kani credit is claimed.

`quire validate` over spec, plan and reviews (`make spec`) reports 0 errors and 6 warnings on both origin/main and the head. The warning lists are identical, so the PR adds no warning. `quire matrix` lists FR-014-AC-72..80 as tagged.

The Delivered status in spec/oracle/matrix/tests.md overclaims one clause of AC-72, the fixture clause. That is recorded once, as SR-17940486709564847047696 FND-001 (gap-analysis), and is not repeated here.

## Verdict

The status and evidence edits are accurate apart from two low wording defects below and the AC-72 fixture clause tracked in the gap-analysis. With that medium fixed in this PR, the spec side is mergeable.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-053 reads 'The `sqrt` and `fma` parts of AC-1, AC-4, AC-5, AC-9, AC-10, AC-13 and AC-14 are were conditional on P3 (IR-1502), which is delivered; they remain conditional on P1.' The text has a doubled verb, 'are were'. Use 'were'. | spec/kani/functional/FR-053-float-scalar-harnesses.md:344 |
| FND-002 | low | The FR-053 row of spec/kani/matrix/tests.md reads '...the Contract IR float `requires_bound` ruling (IR-69), (the FR-014 `sqrt`/`fma` derivation, IR-1502, is delivered); TC-067 controls have not run.' The comma before the parenthesis leaves a dangling list. Write '...ruling (IR-69); the FR-014 `sqrt`/`fma` derivation (IR-1502) is delivered; TC-067 controls have not run.' | spec/kani/matrix/tests.md:13 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-014 derivable-list prose | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md:290-300 | IEEE float32 and float64 add, sub, mul, div, ... (and sqrt and fma at both widths: FR-014-AC-72..80) |
| FR-014 Behavior sqrt/fma bullet | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md:351-367 | When a requested node carries operation.identity quire.op.ieee.float32.sqrt ... |
| FR-014-AC-72..80 status prefixes | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md:729-737 | PLANNED (IR-1502) prefixes removed |
| TC-024 IR-1502 section | examined | spec/oracle/matrix/TC-024-exact-scalar-oracles.md:535-602 | IEEE square root and fused multiply-add (IR-1502) |
| oracle tests.md AC-72..80 row | examined | spec/oracle/matrix/tests.md:24 | Delivered (IR-1502) |
| FR-053 prerequisites, AC-6, Dependencies, Status | examined | spec/kani/functional/FR-053-float-scalar-harnesses.md:56-80, 344-391 | P3 (IR-1502) is delivered and P1 and P2 are not |
| TC-067 step 1 and Status | examined | spec/kani/matrix/TC-067-float-scalar-harnesses.md:16-34, 146 | for sqrt and fma, the FR-014 derivation of IR-1502, which is delivered |
| kani tests.md FR-053 row | examined | spec/kani/matrix/tests.md:13 | PLANNED (IR-1442) |
| TC-030 IEEE-float row | examined | spec/routed/matrix/TC-030-capability-settlement.md:398 | sqrt and fma derive under the delivered FR-014 derivation of IR-1502 |
| spec.md registry row | examined | spec/spec.md:93 | sqrt and fma use the delivered FR-014 derivation of IR-1502 |
