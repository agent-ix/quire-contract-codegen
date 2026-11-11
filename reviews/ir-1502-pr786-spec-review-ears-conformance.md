---
id: SR-17940486709564847047679
title: "IR-1502 PR #786 FR-014 IEEE sqrt and fma spec EARS-conformance review"
type: SpecReview
analysis: ears-conformance
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #786 @ 06dcdaf1; IR-1502; spec/oracle/functional/FR-014-exact-scalar-oracles.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: reviews
---

## Summary

Ticket: IR-1502. The quire grammar lint on the PR head reports no warning for the new rows, because they are not parsed (see the base review, FND-001). To lint them, the blank line at FR-014:722 was removed in a local scratch copy and `quire validate --scope . spec/oracle/functional/FR-014-exact-scalar-oracles.md` was run. The copy was then reverted, and the worktree is clean. Results: `ac:non-singular` errors at lines 724 and 725 (AC-72 and AC-73), and `ac:non-canonical-shape` (obligation-shaped) warnings at 724, 725 and 726 (AC-72..74). The document then fails structural validation. The Behavior bullet ('For ..., the generator shall derive ...') is a ubiquitous EARS statement with a subject and passes.

## Verdict

FAIL (not mergeable). The new criteria do not conform once the tool can see them. AC-72 and AC-73 are each a structural error.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-72 bundles derivation, descriptor naming, `ir_confirmed` marking and a code-structure clause. AC-73 bundles the call shape and the outcome passthrough. With the AC table parsed, quire reports `ac:non-singular` (an error) on both, and `ac:non-canonical-shape` on AC-72..74, because they are written as 'shall' obligations. The neighbouring ACs use direct outcome assertions ('Each generated ... accepts ...'). Split AC-72 and AC-73 into single assertions, and restate AC-72..74 as verification statements. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:725-727 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-014-AC-72 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md:725 | When a strict-reader-admitted application node carries `operation.identity` `quire.op.ieee.float32.sqrt` ... then `derive_exact_scalar_items` shall return an item for it and its descriptor shall name the identity's operation and width, and `generate_exact_scalar_oracles` shall mark its operation `ir_confirmed`. ... |
| FR-014-AC-73 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md:726 | The generated oracle for each of the four identities of FR-014-AC-72 shall call `evaluate_ieee` ... and shall return the owner's result, ordered admitted charges, consumed counters and typed stops unchanged. |
| FR-014-AC-74 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md:727 | If a `sqrt` or `fma` node of FR-014-AC-72 has the wrong operand count, ... then derivation shall return the same typed refusal IEEE `add` returns for that defect ... |
| FR-014 Behavior sqrt/fma bullet | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md:351-363 | For `quire.op.ieee.float32.sqrt`, `quire.op.ieee.float64.sqrt`, `quire.op.ieee.float32.fma` and `quire.op.ieee.float64.fma` (IR-1502), the generator shall derive the descriptor and generate the oracle exactly as for IEEE `add`, with these differences. ... |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | verified at be0daf1b |
