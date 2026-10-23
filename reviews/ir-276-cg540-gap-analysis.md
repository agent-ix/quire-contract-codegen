---
id: SR-15681
title: "Gap analysis of routed lookup batching"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; Cargo.lock, src/oracle/equality/mod.rs, src/oracle/scalar/mod.rs, src/routed/capability.rs, src/routed/contract_package_tests.rs"
review_set: subset
---

## Summary

Ticket IR-276; PR #540 at 583a6d443c512445915cb7b8555db49e4534d909 against main 6c79e985. Compared the changed batching path with FR-019 and TC-030, its new test, and the locked IR FR-038-AC-309 independent-record contract.

## Verdict

**CONDITIONAL** — The new performance control proves one graph/index build per package and exact refusal order for outside forms, but it does not exercise a successful scalar-family cache result. A batch association defect could survive this test.

## Reviewed units

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-019-AC-25 | examined | Each item is identified by its typed IR tag/form, family, operation identity and obligation role as applicable. |
| FR-019-AC-27 | examined | With one Kani candidate, a typed form outside the normative set in FR-019-AC-25 settles `unsupported`, warned, with `unsupported_projection`/`unsupported-requested-capability` naming its typed tag/form and Kani; no artifact is emitted and no generator is invoked. |
| TC-030 | examined | Verify that every requested capability claim is settled by exactly one `negotiate_*` arm over a closed backend kind, that each FR-290 rule settles its own row, including whole-envelope refusal for a repeated backend identity. |
| FR-038-AC-309 | context_only | `CheckedPackageV2::lower_independent_records` returns one record per requested key in input order, each byte-for-byte equal to the record from `lower` called with only that key and the same profile and retained byte ceiling. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new batch test uses only non-scalar forms, so it cannot detect wrong cached scalar families or target association | src/routed/contract_package_tests.rs:1250 |

## Coverage

Plan completion: not assessed

The new test's five requests select `call` and `pure_function` nodes from two packages. Every expected disposition has `KaniFormFamily::OutsideSet`, so all scalar derivation results are `None`; the visit counter checks traversal count but no non-`None` family result. A regression in the new `scalar_families` map that attaches a successful result to the wrong `CheckedNodeId`, or drops all successful results, would not fail this control. Existing single-item scalar settlement tests and IR's independent-record equivalence test do not exercise this CG batch association. Add a same-package batch with two different derivable scalar families and assert the exact per-item typed identity or refusal in both request orders.

The source of `lower_independent_records` in locked IR 8810495e and its IR #517 controls establish one-key record equivalence, including union-byte-ceiling failure. The CG method selection matches that contract. The computed Test Matrix is repository-wide and was inspected only for this diff's TC-030 binding; no plan was supplied and semantic review was not separately opted into.

## Dispositions

At PR #540 head a15eddbd9fbdafef6c11c0cc61edf7ac48a9c9d6, the same-reviewer narrow check is **PASS**. The new test selects a Boolean and an Integer operation from one admitted package, asserts each exact family against its checked node ID in both lookup orders, and requires one scalar graph walk. Its focused test passed 1/1 on that head. The addition is test-only and follows the repository's existing fixture inclusion pattern.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The added two-family cache test directly checks both successful `Some` results against exact node IDs in both orders and one shared graph walk. |
