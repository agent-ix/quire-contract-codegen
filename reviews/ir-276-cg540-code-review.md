---
id: SR-15680
title: "Code and Rust review of routed lookup batching"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; Cargo.lock, src/oracle/equality/mod.rs, src/oracle/scalar/mod.rs, src/routed/capability.rs, src/routed/contract_package_tests.rs"
review_set: subset
---

## Summary

Ticket IR-276; PR #540 at 583a6d443c512445915cb7b8555db49e4534d909 against main 6c79e985. Reviewed the complete five-file diff, including Rust idioms, typed settlement, the locked IR interface, and the changed test. The new independent-record call retains each target's one-key byte and work outcome while reusing IR's graph and closure context.

## Verdict

**PASS** — No code or Rust defect found in the changed surface. `CapabilityLookups` keys both caches by the admitted package identity; the checked-node path uses the existing ordered index. `CheckedNodeTag::Extension` is classified explicitly as unsupported in both affected oracle matches. Aggregate gates were outside this review request.

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
| FND-001 | low | No findings (placeholder) | - |

## Review notes

The previous grouped `lower` call could fail every record when only the union package exceeded its byte ceiling. The new locked IR revision exposes `lower_independent_records`, which assembles one request at a time and has a direct equivalence test at IR #517. The changed CG helper invokes this method and preserves input order through its zip. No new production panic, unsafe block, unchecked numeric conversion, or catch-all enum arm was added. `git diff --check` passed. No aggregate gate was run for this independent review.
