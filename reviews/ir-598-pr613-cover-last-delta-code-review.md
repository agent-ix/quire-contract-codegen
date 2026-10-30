---
id: SR-613009
title: 'Code and Rust delta review — IR-598 PR #613 cover-last family census'
type: SpecReview
analysis: code-review
scope: 'agent-ix/quire-contract-codegen; PR #613 IR-598 8156aacfb1c7c9a2f7a3acdf00e435c847eb1832..6e58b1eaf4911f49a96c915f3bec92b736f3d881; tests/it/cover_last.rs, tests/it/bounded_kani_corpus.rs, tests/it/kani_obligations.rs, tests/it/kani_obligations_state_frame.rs, tests/it/kani_obligations_symbolic_domains.rs, src/kani/generate/corpus/bounded_kani_corpus.rs'
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-015
  type: references
- target: ix://agent-ix/quire-contract-codegen/TC-025
  type: references
---

## Summary

Ticket: IR-598. Independent Opus review of the one-file regression fix on exact PR #613 head `6e58b1eaf4911f49a96c915f3bec92b736f3d881`. The diff removes the retired corpus collection family from the expected cover-last census. Earlier whole-PR reviews and dispositions are in SR-613001, SR-613002 and SR-613008; this review covers only the new delta.

## Verdict

PASS for the reviewed delta. The corpus request enum and test driver are graph-only, so the deleted family has no emitted source. The remaining six expected families match the six driver results. The two-way family equality, proof-function parser, malformed-cover controls, and proof-template source scan still guard the live emitters. Trace tags are unchanged. This read-only review did not run builds, tests, Kani, or the final gate; it does not claim the full IR-598 ticket is complete.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined Scope

```yaml
scope:
- id: FR-015-AC-7
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Every generated harness contains exactly one non-vacuity cover; a precondition harness covers that the precondition holds within the IR bounds, and a contract harness''s cover stands after the contract call, so a run that satisfies every check without satisfying the cover is not a proof.'
- id: FR-015-AC-55
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'While a family''s corpus arm remains (deleted by FR-015-AC-109 for arithmetic, FR-015-AC-110 for collection, FR-015-AC-111 for graph), a bounded-corpus harness of that family ends with exactly one `kani::cover!` after its `assert!` of the case''s oracle, which witnesses that the harness runs to its end past that assertion.'
- id: FR-015-AC-58
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A test generates a harness through every emitting entry point (the precondition, V1 contract, scalar, state-clause and frame-effect families, `generate_kani_bundle`, and the corpus generator for each of its families whose arm remains, per FR-015-AC-109 to FR-015-AC-111), parses each emitted source, and fails for any function attributed `#[kani::proof]` or `#[kani::proof_for_contract]` whose body does not contain exactly one `kani::cover!`, which must be the last statement, with no assertion after it and no other cover.'
- id: TC-025
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: examined
  excerpt: 'Step 18''s inspection remains required for surviving emitters, and FR-015-AC-7/53/58 remain open for their surviving harness families.'
bindings:
- test_id: tc_025_every_emitted_harness_ends_with_exactly_one_cover
  ac_id: FR-015-AC-58
  trace: correct
- test_id: tc_025_the_cover_inspection_rejects_a_misplaced_duplicated_or_missing_cover
  ac_id: FR-015-AC-58
  trace: correct
- test_id: tc_025_no_proof_attribute_is_spelled_outside_what_the_inspection_drives
  ac_id: FR-015-AC-58
  trace: correct
```
