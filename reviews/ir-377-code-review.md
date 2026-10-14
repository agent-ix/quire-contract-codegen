---
id: SR-9000
title: "IR-377 PR 380 code review with Rust lane"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen; PR #380 head 6f0452d318c14c48964c864009ef9dc530b6162e against main 9675c0dc43ba4cc33dc2e274f2fe384849e09ba6; tests/checked_package_support/base.rs, tests/checked_package_support/rekey.rs, tests/composite_equality_support/package.rs, tests/exact_scalar_support/package.rs"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-037
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-052
    type: references
---

# SR-9000: IR-377 PR 380 code review

## Summary

Ticket: IR-377. Review of the four-file PR diff at the exact head above. The Rust lane of code-review inspected typed identity construction, source-map updates, candidate convergence, grouped recursion, nominal member identity, canonical package hashing, and the reader controls. No CI workflow changed. No Cargo gate was run in this reviewer-only pass.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for the inspected diff. Both builders use `ApplicationNodeDraft` and `derive_application_node_id` over complete nodes, and their new tests compare candidates with reader-admitted IDs. Grouped repair constructs `ApplicationIdentityContext` from the ordered graph; tests cover missing and mismatched context plus canonical byte-limit refusal. The base enum member uses `NominalIdentityPreimage::EnumMember(...).digest`, with a changed-case check. The existing `tests/it/exact_scalar_generation.rs` controls submit stale-key and body-mutation wires to the reader without rekey repair and assert `StaleNodeKey`. Remaining `sha256_hex` sites in exact-scalar support name fixture-only operand/bound keys or source bytes, not application or package identity. `refresh_identity` now hashes a typed package preimage through `quire-canonical`. The `u64::MAX` encoding allowance is confined to fixture construction for large-input refusal tests; the checked reader retains the requested input ceiling and the fixture resolver retains its cumulative byte/work caps. This is a static review; no runtime verdict is claimed.

## Coverage

- FR-037-AC-1: both builder implementations and positive reader agreement tests examined.
- FR-037-AC-2: grouped candidate test and rekey context/error tests examined.
- FR-037-AC-3: nominal member builder and changed-case test examined.
- FR-037-AC-4: positive admission and pre-existing direct reader negative controls examined.
- FR-037-AC-5: package, model and fixture-only hash sites classified by use.
- Rust checklist: no added production panic surface, unsafe, CI weakening, compatibility layer, vendored copy, or new unbounded reader loop in the diff.
