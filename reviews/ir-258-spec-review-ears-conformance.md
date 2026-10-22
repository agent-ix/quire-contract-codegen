---
id: SR-15644
title: "IR-258 PR 541 ears-conformance review"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen; IR-258; PR #541; spec/kani/functional/FR-046-rational-function-predicate-harnesses.md; spec/kani/matrix/TC-060-rational-function-predicate-harnesses.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-046
    type: references
---

## Summary

Exact PR head 5555193bf57b4d8fb14e75ccf8b26323860cf976 against base 5e7c1b7b99bef38b87db517c17be0367633c0688. The changed When/SHALL body statement has a named actor and observable response. The revised AC and test step specify matching initial state, denied prefixes, typed outcomes and exclusions without weakening the predicate.

## Verdict

PASS for this method on the two-file spec diff. This does not discharge planned implementation, proof or replay acceptance.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Checked the exact diff and QSL owner source at locked 485ccaa3bd289ae94baa74e96a976adce99fd0d8: qsl-replay/src/execute.rs:665-733 creates the evaluation Meter after supplied conversion; qsl-eval/src/value/expression/family.rs:150-165,209-225 charges one FunctionCall on the same Meter before the body. FR-046-AC-1/4/5/6 and TC-060 step 4 were read as surrounding constraints. No source or tests changed. No applicable AssuranceProfile was found.

## Examined scope

```yaml
scope:
  - {id: "FR-046-AC-3", path: "spec/kani/functional/FR-046-rational-function-predicate-harnesses.md", role: examined, excerpt: "Generated complete-body evaluation over already-admitted arguments preserves the authored full tree and declaration identity, exact owner arithmetic, intermediate values outside the input domains, arbitrary-precision continuation, full typed outcomes, laziness and the body's Meter charge/stop order from entry onward. Native comparison against an independent reference using public owner body operations starts from matching entry Meter state and limits and checks those observations, including each denied body-charge prefix and zero/exact/short budgets. The application's single pre-body FunctionC"}
  - {id: "FR-046", path: "spec/kani/functional/FR-046-rational-function-predicate-harnesses.md", role: examined, excerpt: "When evaluating the body, the generated oracle SHALL use the actual owner's\nexact operations and Meter with the original expression order, typed outcomes,\ncharge ordering, stops and laziness from body entry over already-admitted\narguments. The comparison starts the generated body and independent owner body\noperations from matching entry Meter state and limits. The application's single\npre-body `FunctionCall` charge is outside this predicate harness boundary;\nconversion and supplied-argument admission state are also outside the body.\nFull-call equivalence remains planned under IR-595 / IR-583. "}
  - {id: "TC-060-step-3", path: "spec/kani/matrix/TC-060-rational-function-predicate-harnesses.md", role: examined, excerpt: "3. Generate the actual complete-body oracle over already-admitted arguments and\n   compare its full native observations with an independently constructed\n   reference using public owner body operations for the same admitted function.\n   Start both bodies from matching entry Meter state and limits. Compare the\n   body's charge/stop order from entry onward, preserving all typed outcomes,\n   laziness, ordered Meter charges, counters and stops at every body\n   occurrence-denial prefix and zero/exact/short budgets. Exclude the application's\n   single pre-body `FunctionCall` charge at entry; do not "}
  - {id: "FR-046-AC-1", path: "spec/kani/functional/FR-046-rational-function-predicate-harnesses.md", role: context_only, excerpt: "The genuine source function `rational_commutative(a: Rational[-9,9;1,9], b: Rational[-9,9;1,9]): Boolean` with body `a + b = b + a` selects its whole declaration and complete body through the public QSL admission, strict IR reader/lowering and CG route. Its artifact proves that predicate, not only validity or parity of either addition. Replacing the body by `a + b = a + a` selects the same authored mutation with unchanged parameter domains."}
  - {id: "FR-046-AC-4", path: "spec/kani/functional/FR-046-rational-function-predicate-harnesses.md", role: context_only, excerpt: "Generation introduces no assumption on completion, truth or an absence of undefined/refused/incomplete results. Controls producing each reachable noncompletion retain that typed outcome and cannot become a successful predicate proof; an outcome-based assume mutation is rejected. A completion assertion failure without a replayed false value is not labelled a confirmed semantic false counterexample."}
  - {id: "FR-046-AC-5", path: "spec/kani/functional/FR-046-rational-function-predicate-harnesses.md", role: context_only, excerpt: "The healthy original function receives `Verified` only from an actual successful proof with all required checks and its completion cover satisfied. The wrong-addend mutation produces a substantive failed predicate assertion with actual solver playback of canonical in-domain arguments, and replay of that same authored function completes with Boolean false. A hypothetical witness or cover witness is not substituted for that playback."}
  - {id: "FR-046-AC-6", path: "spec/kani/functional/FR-046-rational-function-predicate-harnesses.md", role: context_only, excerpt: "Each emitted predicate harness has exactly one final completion cover after all predicate assertions, including harnesses with representation constraints. Unsatisfied or unreported completion cover, failed unwind, timeout, memory exhaustion or missing verdict cannot yield `Verified`; classifications retain FR-017 and FR-029 distinctions."}
  - {id: "TC-060-step-4", path: "spec/kani/matrix/TC-060-rational-function-predicate-harnesses.md", role: context_only, excerpt: "4. Run the healthy harness through the approved bounded production path with its\n   actual complete ordered options and no stubbing or fixture edits. Verify all\n   required checks and exactly one final completion cover. Run the wrong-addend\n   mutation over the same entire domain. Retain its actual failed predicate check\n   and solver playback; validate and replay its arguments through the owner route\n   for that exact declaration and assert completed Boolean false. For comparison,\n   `a = 1/2, b = 1/3` mathematically gives `5/6 != 1`, but this example is not a\n   solver counterexample until a"}
```
