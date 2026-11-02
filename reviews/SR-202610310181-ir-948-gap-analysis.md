---
id: SR-202610310181
title: "IR-948 PR710 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-948 PR710; five changed Rust files at 9ce333bf8e53b16793e284321807dcb3c4d08bf8"
review_set: subset
---

## Summary

Ticket: IR-948. Frozen source reviewed against 2256d0e4cb864c38b748ff757eed25034802c59a. Code-review includes the Rust lane. No applicable AssuranceProfile exists. No source, normative specification, dependency, CI or gate edits were made by the reviewer.

## Verdict

PASS (scoped generation slice only). The invariant renderer preserves ordered oracle inputs and original inclusive i64 domains, embeds the selected oracle bytes, asserts completed true (or the direct Boolean constant), and places one domain cover last with no subject call. Native selection and source-bound typed identities remain intact. Arithmetic invariant refusal is appropriate for this incremental slice because it lacks independent arithmetic probes; this is still unfinished AC-41 behavior, not an accepted narrowing of that criterion. Existing pre/postcondition paths remain available.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed

Review is limited to this PR's generation/native-oracle slice. The computed repository matrix was read; FR-015-AC-38/39/40/41/46/49 are tagged. Tagging is not evidence that broad planned criteria are complete. Unchanged repository debt and unrelated criteria were not reclassified as regressions. Reverse tracing maps the new production behavior to FR-015-AC-38/39/40/46/49; no new unspecified production behavior or stubs were found. Optional separate gap semantic sweep was not invoked; ordinary code-review requirement/test alignment was performed.

The actual /tmp/ir948-qualification/focused.log records 14 passed, including constant true/false, two distinct input domains, unbounded refusal, deterministic native identity, native/general emitted oracle compilation and healthy/mutated-QSL/two-field native execution. clippy.log records all-target/all-feature workspace Clippy success; fmt.log records touched Rust formatting success. The intended red logs fail for missing invariant admission; the initial compile-only failure is excluded. Reviewer ran no Cargo or backend jobs as directed; parent owns the aggregate. No Kani execution or whole-ticket acceptance credit is given. Real four-kind backend controls, impossible-precondition CoverUnsatisfied and invariant mutant backend counterexample remain open. The inherited tc_025_model_member_types_without_i64_ranges_remain_unranged failure is baseline debt, not attributed to this PR.

## Examined units

```yaml
scope:
- id: FR-015-AC-38
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V2 clause claim naming a `state`/`state_clause` node of an admitted `CheckedPackageV2`
    whose `operation.member.clause` is `precondition`, `postcondition` or `invariant`
    yields one harness of that obligation kind, separate from the harness of every
    other clause of the package; the kind is read from the node and no caller value
    changes it. For a collection-valued operation clause, the one operation-clause
    harness carries a generated `CgObjectState` with every admitted model field as
    an exact `Value`, the selected one being `Value::Collection`; the customer subject
    receives one mutable exact-va
- id: FR-015-AC-39
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
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
  role: examined
  excerpt: A harness for a V2 clause whose condition, inline or referenced from an
    admitted Boolean expression node, contains Boolean connectives and bounded-integer
    comparisons (`and`, `or`, `not`, `implies`, `eq`, `ne` and the six integer comparisons)
    embeds an FR-014 oracle of that condition byte-identical to the oracle crate's
    clause-identified function, as FR-015-AC-20 states for a scalar claim; the clause-identified
    oracle needs FR-014-AC-38. PLANNED (IR-489).
- id: FR-015-AC-41
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A V2 clause whose body holds integer arithmetic (add, subtract, multiply,
    negate) as an operand of a comparison lowers, and its harness keeps the independent
    native assertion of FR-015-AC-37 for each arithmetic subterm (the oracle''s outcome
    against the operation evaluated natively in `i128` over the same operands, `Refused`
    assumed away nowhere): a postcondition `amount < 1000 implies amount + 1 <= 1000`
    over `amount: Int[0, 1000]` yields one contract harness. With the installed backend
    the unmutated clause verifies with its cover satisfied. In separate unconstrained
    installed-Kani runs of tha'
- id: FR-015-AC-46
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Every V2 clause harness contains exactly one non-vacuity cover, placed
    after every assumption, after the subject call and after every assertion the harness
    carries (the last statement, as FR-015-AC-7 states), that states the family''s
    reachability, as FR-015-AC-7 does: a precondition harness covers that the precondition
    holds within the bounds; a postcondition harness covers, after the subject call
    returns, that the requires and the bounds are jointly satisfiable; an invariant
    harness covers that the drawn state satisfies its domain assumptions. The subject
    call is the call of the customer subj'
- id: FR-015-AC-47
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Regeneration of a V2 clause harness from equal inputs is byte-identical,
    and changing the unwind bound or the customer subject changes the harness identity
    record (as FR-015-AC-10 states). PLANNED (IR-489).
- id: FR-015-AC-49
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V2 invariant claim yields a harness that draws its state and parameters
    within their declared domains (FR-015-AC-39) and asserts the invariant clause;
    its subject is the clause itself, with no subject call. Preservation of the invariant
    under an operation is not specified by this criterion. PLANNED (IR-489).
- id: FR-015-AC-125
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Authentic QSL-emitted, IR-admitted bounded integer scalar and clause inputs
    generate kernel-owned native carriers and checked operation plans for all intermediates,
    with original graph identities/domains retained. Add/Subtract/Negate/ordering
    retain their admitted signed-i128 capacities; Multiply is eligible exactly when
    both whole operand intervals fit signed32 and uses the exact four-corner signed64
    image retained in i128. A Multiply operand interval outside signed32 makes the
    whole requested arithmetic component native-ineligible, even with a zero opposite
    operand, fitting actual payload or
```

## Disposition round 1 verdict

PASS for the scoped generation/native-oracle slice at 08e8c69b24f7316a6d87b91713f6442ca169e249. No new findings. The fix reuses existing scratch helpers; dependencies resolve from the seeded root lock, generated oracle bytes and independent checks remain unchanged, and child Cargo uses its isolated target without reacquiring the outer build lock. Actual variant-red.log and variant-green.log demonstrate failure then success with two genuine runtime rlibs retained. fix-fmt.log and fix-clippy.log pass. Reviewer inspected the source, scratch helper and logs; no builds or backend jobs were run. The parent still owns the aggregate native integration check; broader IR-948 installed-backend acceptance remains open. Original findings and original verdict text above are preserved as review history.
