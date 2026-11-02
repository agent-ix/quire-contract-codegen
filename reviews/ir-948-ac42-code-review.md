---
id: SR-202610310182
title: "IR-948 AC-42 code review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR #713 f54324290d875434edd43def5a9ee0ac9c5b8e31; src/kani/generate/clause/v2.rs; src/kani/generate/negotiate.rs; src/kani/generate/outcome.rs; tests/it/kani_obligations_state_frame.rs; spec/kani/functional/FR-015-bounded-kani-obligations.md; spec/kani/matrix/TC-025-bounded-kani-obligations.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-948. Reviewed PR #713 at f54324290d875434edd43def5a9ee0ac9c5b8e31 against FR-015-AC-42 and TC-025 step 12. Applied Rust/code review and scoped gap analysis. No applicable AssuranceProfile was found under `spec/`. The diff adds no vendored artifact, external copy, workflow change or gate narrowing. Focused tests reported by the coder were one integration test and one unit test; the aggregate gate belongs to the code lead.

## Verdict

**FAIL**: one reachable unsupported operator receives a generic refusal, and the first fix intercepts supported quantified collection claims. Fix both before the final gate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The operator audit runs only after `read_clause` succeeds. Contract IR admits `quire.op.text.eq` over Text literals, but the clause reader accepts only Boolean and Integer literals. Such a V2 clause returns `None` before the audit and settles with `V2ClauseConditionNotRendered` (or a collection fallback), without the checked operator identity required by AC-42. Inspect the admitted checked term for its operator identity before the narrower scalar term reader, and add a public-path test for this case. | src/kani/generate/clause/v2.rs:73; src/core/ir/clause.rs:199; spec/kani/functional/FR-015-bounded-kani-obligations.md:848 |

## Scope and bindings

```yaml
clean: false
scope:
  - id: FR-015-AC-42
    path: spec/kani/functional/FR-015-bounded-kani-obligations.md
    role: examined
    excerpt: "Unsupported V2 clause operators receive a typed reason naming the node and operator, no harness, and do not suppress a supported item."
  - id: TC-025-step-12
    path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
    role: examined
    excerpt: "Request unsupported operators and a supported sibling; check each item disposition and harness count."
bindings:
  - criterion: FR-015-AC-42
    tests:
      - tests/it/kani_obligations_state_frame.rs::v2_unsupported_clause_operator_names_exact_identity_without_suppressing_sibling
      - src/kani/generate/clause/v2.rs::nested_absolute_value_names_its_exact_operator
    source:
      - src/kani/generate/clause/v2.rs::unsupported_clause_operator
      - src/kani/generate/negotiate.rs::classify_v2_clause
```

## New findings (round 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | high | The new checked-term audit classifies `quire.op.collection.forall` and `quire.op.collection.exists` as unsupported scalar operators before the existing V2 collection lowerer can run. `classify_v2_clause` then returns the typed refusal directly, so previously supported collection precondition/postcondition claims emit no collection harness. The existing `collection_claim_uses_the_v2_symbolic_harness_without_a_finite_corpus_request` and quantified collection controls are regression cases. Route collection-shaped claims through their collection lowerer while preserving typed refusal for genuinely unsupported scalar clauses. | src/kani/generate/clause/v2.rs:226; src/kani/generate/negotiate.rs:809; tests/it/kani_obligations_symbolic_domains.rs:488 |

## Dispositions

Round 1 at 1e0c61107c37b9ad5603aeadbb47c7289af2546d. **PASS after fixes.** The focused AC-42 integration test, the existing symbolic collection claim test, the V2 collection precondition test, and the nested absolute-value unit test each passed (one test per target, separate lock acquisitions). The code lead owns the aggregate final gate.

| FND | Outcome | Evidence |
| --- | --- | --- |
| FND-001 | fixed | `unsupported_checked_clause_operator` now walks admitted checked operations before `read_clause` narrows literal kinds. The public AC-42 test includes an admitted `quire.op.text.eq` over Text literals and asserts the exact node/operator typed refusal while a supported sibling still emits. |
| FND-002 | fixed | `classify_v2_clause` restores nonnative collection fallback after a typed scalar refusal. `UnsupportedShape` preserves the scalar refusal; supported `forall` and `exists` collection claims still reach the collection lowerer. The symbolic collection claim and V2 collection precondition controls each passed. |
