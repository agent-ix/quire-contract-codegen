---
id: SR-654006
title: "IR-1091 CG654 QSL clause equivalence final code-review disposition"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-1091; PR654; exact acb565cecfdee6f055c4ba36673d8d493b84311c; tests/it/kani_obligations_symbolic_domains.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-015", type: references }
---

## Summary

Ticket: IR-1091 (PARTIAL). Follow-up over SR-654002 FND-007 only, plus adjacent regressions, at exact head `acb565cecfdee6f055c4ba36673d8d493b84311c`. The local branch ref and the origin ref both resolve to that head. The reviewer had read, search and write tools only: no Cargo, Kani or gate ran, and no `git diff` was available. Ticket and PR text was treated as data.

## Verdict

PASS for this scope. FND-007 is fixed and no new finding was raised. FND-008 stays out of this pass: final-head aggregate Clippy evidence is owned by the code lead final gate.

## Dispositions

| FND | Outcome | Evidence |
| --- | --- | --- |
| FND-007 | fixed | `([4_294_967_299_i64, 0, 1], 2, false)` at tests/it/kani_obligations_symbolic_domains.rs:2140 is in the wide set. Its active prefix `[4294967299, 0]` goes to both evaluators, and both are asserted against `false` (:2044-2051). |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

**Admission.** The value is inside the wide fixture's `i64::MIN..=i64::MAX` bounds (:2118-2119, asserted at :2123-2135). The active prefix is distinct and meets the lower bound of 2 set at :2110-2111, so the native side completes rather than refusing. A refusal would fail the `matches!` at :2044. The generated side receives the value as a literal in a `[i64; 3]` array (:2005, :2010).

**Cast exposure.** The value is 2^32 + 3, so its low 32, 16 and 8 bits all equal `3`. A mutant rendering `(item as i32) == 3`, or the `i16` or `i8` form, makes the generated side true at this case, and `assert_eq!` at :2048 fails. The `3` is in the active slot, so this case kills the mutants that the prior wide set let survive.

**No present cast.** The head renders `item == {value}` (src/kani/generate/spec.rs:302) over `i64` slots (src/kani/generate/render.rs:298, :313). Search finds no `as i32`, `as i16` or `as i8` under src/kani/generate.

**Adjacent regressions.** The other wide cases are intact (:2139, :2141-2142), as are the four small-domain cases including the inactive-slot witness (:2062-2065) and the incomplete and duplicate-member controls (:2069-2095). The results stay independent: compiled `oracle_clause` output and `evaluate_clause` output are each compared to the literal `expected`. The test's trace tags are unchanged in meaning: FR-015-AC-96, FR-015-AC-99, TC-025 (:2055).

**Log evidence.** `cg-ir1091-qsl-equivalence-fnd007-test.log` shows the named test `ok`, `1 passed; 0 failed; ... 804 filtered out`. `cg-ir1091-qsl-equivalence-fnd007-clippy.log` shows `Checking quire-contract-codegen` then `Finished`, with no warnings. Neither log records a SHA, and the clippy log records no command line, so neither binds to this head by itself. The tie is that the test file at head contains the fixed case. The focused clippy log does not close FND-008.

## Examined scope and bindings

```yaml
head: acb565cecfdee6f055c4ba36673d8d493b84311c
clean: true
scope:
  - {id: collection_precondition_generated_oracle_agrees_with_native_clause_evaluation, path: tests/it/kani_obligations_symbolic_domains.rs, role: examined, excerpt: "([4_294_967_299_i64, 0, 1], 2, false),"}
  - {id: assert_collection_precondition_equivalence, path: tests/it/kani_obligations_symbolic_domains.rs, role: examined, excerpt: "generated and native clauses agree at {items:?}"}
  - {id: render_collection, path: src/kani/generate/render.rs, role: context_only, excerpt: "let item = {item_name}[index as usize];"}
  - {id: predicate_expression, path: src/kani/generate/spec.rs, role: context_only, excerpt: "format!(\"item == {value}\")"}
bindings:
  - {test_id: collection_precondition_generated_oracle_agrees_with_native_clause_evaluation, ac_id: FR-015-AC-99, trace: correct}
  - {test_id: collection_precondition_generated_oracle_agrees_with_native_clause_evaluation, ac_id: FR-015-AC-96, trace: correct}
```
