---
id: "SR-648"
title: "CG PR 208 gap analysis: scalar Kani harness native-relation property"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@2bea27d5504126a0c32ab5d37fef02c39f9bf0f2; src/kani_obligations.rs, tests/it/kani_obligations.rs, tests/it/routed_generation.rs, spec/functional/complete-v1/FR-015-bounded-kani-obligations.md, spec/functional/complete-v1/FR-017-kani-execution-evidence.md, spec/test-matrix.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: references
---

# SR-648: CG PR 208 gap analysis

## Summary

Ticket: IR-458. PR: agent-ix/quire-contract-codegen#208 at 2bea27d. Planless and scoped to the PR
diff: requirement ownership of the changed behaviour, and the trace tags of the new and renamed
tests. Plan completion: not assessed.

## Method

I read the FR-015 acceptance criteria (AC-1 to AC-21), the FR-017 acceptance criteria, and the
FR-015 and FR-017 rows of `spec/test-matrix.md`. For each test the PR adds or changes, I checked
that every AC it cites exists and that the test exercises it. I looked for an AC that owns the
rendered scalar harness's assertion, the behaviour IR-458 changes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No acceptance criterion owns the property this PR establishes. FR-015 says what a scalar harness assumes (AC-11, AC-16), what it refuses (AC-17, AC-18) and that it has one cover (AC-7). No AC says what it must assert. The renamed text test and the new Kani mutation control both trace to FR-015-AC-7, the cover criterion. A regression to a tautological assertion, the defect IR-458 fixed, would break no requirement. Only text checks tagged to an unrelated AC would catch it. The PR says "No spec requirement changes", and that is the gap. An FR-015 AC should state that a scalar harness asserts the oracle's outcome equals the clause's operation evaluated independently of the oracle: completed with that value when it lies in the result bound, refused otherwise, nothing else accepted. It should say that an oracle computing a different operation is falsified, with a TC-025 row in the matrix. | spec/functional/complete-v1/FR-015-bounded-kani-obligations.md:155-180, tests/it/kani_obligations.rs:764, tests/it/kani_obligations.rs:1998 |
| FND-002 | low | Wrong trace on the new test. `tc_025_scalar_harness_falsifies_a_mutated_oracle_arithmetic` cites FR-017-AC-7: "A crate whose library source does not contain the harness source byte for byte is refused, and no backend runs." The test never builds a crate without its harness. It looks copied from `tc_027_a_routed_scalar_harness_verifies`, which does test that. FR-017-AC-11 (a routed scalar harness runs through `execute_kani_obligation`) is closer to what it exercises, alongside the FR-015 AC that FND-001 asks for. | tests/it/kani_obligations.rs:1998 |

## Verdict

Not clean. FND-001 should be fixed in this PR, as a spec AC plus a matrix row with the tests
re-tagged to it, because IR-458's whole point is that this property was missing. FND-002 is a
one-line re-tag. The code-to-test side is reviewed in SR-647.

Clean units examined: FR-015-AC-7 still holds. There is exactly one `kani::cover!(completed, ..)`,
checked by tc_025 and by `tc_027_a_routed_scalar_harness_run_classifies_like_a_contract_harness`.
FR-015-AC-11 and FR-015-AC-16 still hold. tc_025 checks the operand assumes, and both tc_033
tests now check the result bound through the `admitted` line. FR-017-AC-11 is still exercised by
the routed verify, falsify and classification tests. The test matrix needs no change for the
renamed test, because it lists TC ids, not function names.
