---
id: SR-598006
title: "Gap analysis of IR-1044 V2 clause proof dependency census"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-1044; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md, spec/assurance/AD-004-cg-crate-layout.md, tests/it/kani_obligations_state_frame.rs; FR-015-AC-22/25/44/45"
review_set: subset
---

## Summary

Checked the computed Test Matrix and traced the four FR-015 census criteria through the changed V2 tests, production code, and authored matrix. All four criteria have static tags to real changed tests, but the persisted-record oracle is incomplete and the authored status has not been reconciled to the new evidence.

## Verdict

**CONDITIONAL** — Two medium evidence/documentation findings remain.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The positive V2 test does not verify census or readiness values in the persisted identity record | tests/it/kani_obligations_state_frame.rs:568 |
| FND-002 | medium | AD-004 and the authored test matrix still describe the V2 census criteria as planned or unbacked after this PR adds the supported path | spec/kani/matrix/tests.md:21 |

## Coverage

Plan completion: not assessed

Examined FR-015-AC-22, FR-015-AC-25, FR-015-AC-44, and FR-015-AC-45, plus TC-025 steps 4, 7, and 13. `quoin matrix --repo . --json` reports each of the four as statically tagged; it reports no run evidence bound to them. The new invalid test exercises empty, duplicate, state/path inconsistency, and non-Required inputs through the public V2 request and the Rejected outcome. The new positive test exercises sorted identity, order independence, ready/incomplete, and not_run on the in-memory identity. The record check only searches for the not_run field and tests record inequality between ready and incomplete cases, so it would pass if readiness were omitted from serialization or if the dependency list were incomplete. The authored matrix and AD-004 still call this work planned or unbacked. No explicit plan was supplied.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The V2 positive test now parses the persisted record and asserts the complete sorted dependency edges, readiness, and not_run for passed, missing, failed, and reordered declarations. |
| FND-002 | fixed | AD-004, FR-015, TC-005, TC-025, and the authored matrix now record the V2 generation-time evidence separately from V1 graph retirement, remaining V2 family work, and installed-Kani proof. |
