---
id: SR-14111
title: "IR-917 PR 477 spec-review/base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; FR-015-AC-119, TC-025 step 49, context FR-015-AC-38/39/40/41/118, FR-014-AC-38, IT-011-SC-02"
review_set: subset
---

## Summary

Ticket IR-917. Exact PR #477 head f9b12a8d43d42ee336da5cf08e195de7aa4af734; examined new FR-015-AC-119 and TC-025 step 49 against FR-015-AC-38/39/40/41/118, FR-014-AC-38 and IT-011-SC-02. Method: spec-review/base. No additional defect found under this lens.

## Verdict

**PASS**. The review is restricted to the changed spec diff. The planned criterion does not claim implemented proof evidence.

## Examined scope

```yaml
scope:
- id: FR-015-AC-119
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-917; depends on FR-015-AC-40 and FR-015-AC-41). From QSL source
    authoring the V2 postcondition `amount < 1000 implies amount + 1 <= 1000`, the
    admitted `CheckedPackageV2` yields a clause-identified FR-014-AC-38 oracle and
    an FR-015 postcondition harness for that same clause, not a scalar implication
    harness. The harness embeds the oracle byte-identically as FR-015-AC-40 requires
    and independently asserts the clause's implication using native `!left || right`
    over the same drawn values; neither that expectation nor the oracle calls the
    runtime implication helper. The compiled subjec
- id: TC-025-step-49
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: examined
  excerpt: "49. Author the postcondition `amount < 1000 implies amount + 1 <= 1000`\
    \ in\n    QSL source. Emit its exact `CheckedPackageV2` through QSL's public facade,\n\
    \    admit it through Contract IR, and select the postcondition by its clause\n\
    \    node identity. Check that the FR-014-AC-38 oracle and FR-015 harness name\n\
    \    that same clause, that the oracle is embedded byte for byte, and that the\n\
    \    harness independently checks native `!left || right` over the clause's\n\
    \    drawn values. Inspect the compiled subject to require a call to\n    `quire_contract_runtime::operators::implies_short_circuit`\
    \ and ins"
- id: FR-015-AC-38
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: A V2 clause claim naming a `state`/`state_clause` node of an admitted `CheckedPackageV2`
    whose `operation.member.clause` is `precondition`, `postcondition` or `invariant`
    yields one harness of that obligation kind, separate from the harness of every
    other clause of the package; the kind is read from the node and no caller value
    changes it. PLANNED (IR-489).
- id: FR-015-AC-39
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'A V2 clause harness draws one nondeterministic value per drawn input:
    the pre-state of each state field of `self` the body reads, whether bare or under
    `pre(...)` (the subject''s input state; how it reaches the subject is FR-025''s),
    and each operation parameter (for a precondition or invariant, each bare state
    field read, since the state is the one judged). Each carries an inclusive assumption
    equal to its declared domain: the accessor''s `IntRange` values for a selected
    model declaration state field (FR-015-AC-77), and the IR `bounded_domain` of a
    parameter''s type. A non-model or admitted unsel'
- id: FR-015-AC-40
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: A harness for a V2 clause whose condition, inline or referenced from an
    admitted Boolean expression node, contains Boolean connectives and bounded-integer
    comparisons (`and`, `or`, `not`, `implies`, `eq`, `ne` and the six integer comparisons)
    embeds an FR-014 oracle of that condition byte-identical to the oracle crate's
    clause-identified function, as FR-015-AC-20 states for a scalar claim; the clause-identified
    oracle needs FR-014-AC-38. PLANNED (IR-489).
- id: FR-015-AC-41
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'A V2 clause whose body holds integer arithmetic (add, subtract, multiply,
    negate) as an operand of a comparison lowers, and its harness keeps the independent
    native assertion of FR-015-AC-37 for each arithmetic subterm (the oracle''s outcome
    against the operation evaluated natively in `i128` over the same operands, `Refused`
    assumed away nowhere): a postcondition `amount < 1000 implies amount + 1 <= 1000`
    over `amount: Int[0, 1000]` yields one contract harness. With the installed backend
    the unmutated clause verifies with its cover satisfied, and the same clause with
    `+` mutated to return `left'
- id: FR-015-AC-118
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: For an admitted V2 scalar `implies` claim over two Boolean operands, the
    generated harness embeds its FR-014 oracle as FR-015-AC-20 requires and draws
    both operands symbolically. It calls `quire_contract_runtime::operators::implies_short_circuit`
    with `left` and a closure returning `right` as the subject expression, asserts
    that subject result equals an independent native `!left || right` expectation
    over the same operands, and separately asserts that the oracle's `Outcome<bool>`
    is `Completed` with that expectation; neither subject nor oracle computes the
    native expectation. With installed Ka
- id: FR-014-AC-38
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: context_only
  excerpt: The Boolean condition in the third argument of a `state`/`state_clause`
    node's `quire.op.state.clause` body, addressed by the clause node id, derives
    a descriptor and generates an `ir_confirmed` oracle over its connectives, integer
    comparisons and integer add, subtract, multiply and negate, returning `Outcome<bool>`.
- id: IT-011-SC-02
  path: agent-ix/quire-integration/spec/integration/IT-011-function-application-kani-exemplar.md
  role: context_only
  excerpt: all three steps pass. A generation-time Boolean, a missing real Kani run,
    a verdict without a satisfied healthy cover, a refused report, a counterexample
    outside the admitted domain or a mutation that leaves the claim verified does
    not satisfy the scenario.
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
