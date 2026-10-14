---
id: SR-9001
title: "IR-377 PR 380 gap analysis"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen; PR #380 head 6f0452d318c14c48964c864009ef9dc530b6162e against main 9675c0dc43ba4cc33dc2e274f2fe384849e09ba6; FR-037-AC-1 through AC-5, TC-052, tests/checked_package_support/base.rs, tests/checked_package_support/rekey.rs, tests/composite_equality_support/package.rs, tests/exact_scalar_support/package.rs and bound tests/it controls"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-037
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-052
    type: references
---

# SR-9001: IR-377 PR 380 gap analysis

## Summary

Ticket: IR-377. Plan completion: not assessed. The computed `quoin matrix --repo . --json` records FR-037-AC-1 through AC-4 as tagged. AC-5 is Inspection and appears as `method-without-symbol`, which is expected for a non-test method. The four changed test-support files implement fixture construction and key repair owned by FR-037; no production source or spec changed. The existing direct reader tests cover stale application keys and body mutation without rekeying.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for the FR-037 and TC-052 diff scope. The candidate, grouped recursion, nominal-member, positive-admission, and negative-refusal obligations have corresponding test assertions. No new unowned behavior or stub was found in the changed files. This is a matrix and source inspection; it does not claim runtime gate results or repository-wide assurance.

## Coverage

- FR-037-AC-1: tagged positive candidate tests in both builders and convergence tests in rekey support.
- FR-037-AC-2: tagged grouped candidate agreement and typed context/limit refusal test.
- FR-037-AC-3: tagged admitted enum member and changed-case test.
- FR-037-AC-4: tagged positive admission and `tests/it/exact_scalar_generation.rs` direct stale-key and body-mutation refusal tests.
- FR-037-AC-5: inspection of hash ownership and typed package preimage; `method-without-symbol` is not a missing test for this method.
- Plan completion: not assessed.
- Optional semantic review: not requested; limited intent-to-assertion inspection is described above.
