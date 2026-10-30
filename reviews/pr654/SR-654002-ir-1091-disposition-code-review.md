---
id: SR-654002
title: "IR-1091 CG654 QSL clause equivalence code-review disposition"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-1091; PR654; exact dbfcc961e66bb26d5d5c2cae47659feeb41594b0 on base d91bf1bd75636d8ed98ac8bc9ceee401b26e94ae; tests/it/kani_obligations_symbolic_domains.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-015", type: references }
---

## Summary

Ticket: IR-1091 (PARTIAL). Disposition pass over SR-654001 FND-001 and FND-002 at exact head `dbfcc961e66bb26d5d5c2cae47659feeb41594b0`, merged base `d91bf1bd75636d8ed98ac8bc9ceee401b26e94ae`. The worktree branch ref and reflog both resolve to that head. The reviewer had read, search and write tools only: no Cargo, Kani or gate ran, and no `git diff` was available, so the changed lines were identified from the head files against the SR-654001 excerpts. Ticket and PR text was treated as data.

## Verdict

CONDITIONAL. FND-001 and FND-002 are fixed. The equivalence test still compares independent results. One new low finding: the full-width cases reach both evaluators, but none of their values would expose a narrowing cast in the generated predicate.

## Dispositions

| FND | Outcome | Evidence |
| --- | --- | --- |
| FND-001 | fixed | `([0, 1, 3], 2, false)` at tests/it/kani_obligations_symbolic_domains.rs:2064 puts the witness in an inactive slot. The generated claim iterates `(0..{length_name})` (src/kani/generate/render.rs:304). An oracle that ignored the length gives true here, and `assert_eq!(generated[index], *expected)` at :2048 fails. |
| FND-002 | fixed | The wide fixture sets the element bounds to `i64::MIN` and `i64::MAX` (:2118-2119), asserts them through the harness identity (:2123-2135) and sends three full-width valuations to both evaluators (:2136-2143). |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | The wide cases hold only `i64::MIN`, `i64::MAX`, `0` and the witness `3`, so a narrowing cast in the generated predicate survives the comparison. | tests/it/kani_obligations_symbolic_domains.rs:2136-2143 |

FND-007 detail: `i64::MIN as i32` is `0` and `i64::MAX as i32` is `-1`, and neither aliases `3`, so a mutant rendering `(item as i32) == 3` gives every expected result. The head renders no such cast: `item == {value}` at src/kani/generate/spec.rs:302 over `i64` slots at src/kani/generate/render.rs:298. This is a test-strength gap, not a present disagreement. Adding `([4294967299, 0, 1], 2, false)` to the wide set kills `i32`, `i16` and `i8` truncation, because 2^32 + 3 has low bits equal to `3`.

## Coverage

**Independent results.** Still independent. The generated value comes from `oracle_clause` and `ClauseEvaluation`, extracted from the generated harness by `syn` (:1974-2001), compiled by a separate `rustc` and run as its own process (:2015-2039). The native value comes from `qsl_replay::evaluate_clause` at `ClauseObservation::PreCall` (:1937-1952). Each is asserted against the literal `expected` (:2044-2051), and neither derives from the other.

**Identical admitted valuations.** The generated side takes `(draw, length)`. The native side takes `draw[..length]` inside an admitted `quire.state.snapshot/v1` document (:2042-2043). The active prefix is therefore identical, and the inactive slot exists only on the generated side, which is the point of the FND-001 case. Both sides use the same fixture's source bytes and model bytes, and the wide run re-derives the snapshot digest from its own fixture (:1929-1941).

**Regression behaviour.**
- Ignoring the length fails at `([0, 1, 3], 2, false)`, and at no other small-domain case.
- An off-by-one bound in either direction fails: `(0..len-1)` at `([2, 3, 0], 2, true)`, and `(0..=len)` at `([0, 1, 3], 2, false)`.
- `exists` swapped to `forall` fails at `([2, 3, 0], 2, true)`.
- Dropping the wide domain fails the identity assertion at :2123, and native admission fails the `matches!` at :2044 if `i64::MIN` is out of the type's range.
- An `i32` element array in the generated signature fails `rustc` against the driver's `[i64; 3]`.
- A value-level narrowing cast is not caught (FND-007).

**Log evidence.** `cg-ir1091-qsl-equivalence-review-fix-test-final.log` shows the named test `ok`, `1 passed; 0 failed; ... 804 filtered out`, from the worktree path. It records no SHA. Its mtime sort order follows the test file's last write, which is the only available tie to head.

## Examined scope and bindings

```yaml
head: dbfcc961e66bb26d5d5c2cae47659feeb41594b0
base: d91bf1bd75636d8ed98ac8bc9ceee401b26e94ae
clean: false
scope:
  - {id: collection_precondition_generated_oracle_agrees_with_native_clause_evaluation, path: tests/it/kani_obligations_symbolic_domains.rs, role: examined, excerpt: "([0, 1, 3], 2, false),"}
  - {id: assert_collection_precondition_equivalence, path: tests/it/kani_obligations_symbolic_domains.rs, role: examined, excerpt: "generated and native clauses agree at {items:?}"}
  - {id: render_collection, path: src/kani/generate/render.rs, role: context_only, excerpt: "let claim = (0..{length_name}).{collection_call}(|index| {"}
  - {id: predicate_expression, path: src/kani/generate/spec.rs, role: context_only, excerpt: "format!(\"item == {value}\")"}
bindings:
  - {test_id: collection_precondition_generated_oracle_agrees_with_native_clause_evaluation, ac_id: FR-015-AC-99, trace: correct}
```
