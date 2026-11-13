---
id: SR-17940486709564847047936
title: "Gap analysis — CG PR 820 FR-055 real-Kani fixes"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-055-kani-mutations-target.md, spec/kani/matrix/TC-069-kani-mutations-target.md, spec/core/functional/interface-001-codegen-api.md, scripts/kani_mutations.rs, src/kani/run/scalar.rs, tests/it/kani_mutations.rs, tests/it/kani_obligations.rs; ticket IR-1437"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-055", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/TC-069", type: references }
---

## Summary

Planless gap analysis of PR #820, scoped to the diff, covering the chain from FR-055 to TC-069 to the tests and the code. Plan completion: not assessed. Ticket: IR-1437. Trace tags were read straight from `tests/it/kani_mutations.rs`.

## Verdict

**GAPS (minor).** The FR-055 outcome semantics are unchanged: `pass` still means the owning harness printed the marker. The PR only widens which Kani outcome counts as "Kani rejected the proof". The FR-055 and TC-069 Status lines are honest. AC-1 and AC-7 are still not claimed for a real run, which is right, because no real run has passed yet. The new fail-cause test backs FR-055-AC-5's `fail` causes, and it can fail. Two gaps remain. A spec statement about who reads `failureCause` now contradicts the code (FND-001). One new test is tagged to an AC it does not assert (FND-002). The high honesty defect in how `pass` is matched is recorded in the code-review SR (FND-001 there), not repeated here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | interface-001's `failed_without_counterexample_cause` entry says the cause "never feeds classification, refusal or the batch timeout rule, and the FR-029 terminal map alone reads it". `falsified_verdict` now reads it to decide whether to print the rejection marker, and the existing `proof_failure_class` (scalar.rs:393) already read it. The spec is wrong about the code. Either state the scalar verdict and class readers, or narrow the claim to "never feeds classification" | spec/core/functional/interface-001-codegen-api.md:374, src/kani/run/scalar.rs:305-320 |
| FND-002 | low | `every_shipped_mutant_still_parses_as_rust_once_injected` is tagged FR-055-AC-5. AC-5 says a mutant that does not compile is `fail`. This test asserts the opposite thing: every shipped declaration entry parses. It is a property of the declaration file (FR-055-AC-9: each entry holds file, anchor, replacement and owning proof), so it should be tagged AC-9 | tests/it/kani_mutations.rs:531-545 |

## Coverage

- FR-055-AC-5: `a_fail_row_names_its_cause_exit_status_and_the_tail_of_the_output` (new) and `absent_or_repeated_anchors_are_malformed_and_other_failures_are_fail`. Correct binding.
- FR-055-AC-1: `every_owning_proof_is_an_ignored_lane_test_that_prints_the_verdict_on_failure` (new) checks the source shape that AC-1's real run depends on. It is supporting evidence, not the run itself. The Status line correctly does not claim AC-1's real half.
- FR-055-AC-7: unchanged, still no real-Kani run, disclosed in the Status line.
- Code with no owning requirement: none. The `falsified_verdict` change implements FR-055's `pass` marker for the scalar harness.
