---
id: SR-4896
title: "IR-496 PR 340 composite equality code review"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@d8cb477ba86482e7cbf3a197be03b5a93bc65890; src/lib.rs, src/oracle/claim.rs, src/oracle/equality/mod.rs, src/oracle/equality/resolution.rs, src/oracle/function/mod.rs, src/oracle/scalar/mod.rs, tests/it/composite_equality_generation.rs, tests/it/exact_scalar_generation.rs; FR-018-AC-28..31, TC-029"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-018
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-029
    type: references
---

## Summary

Ticket: IR-496. Reviewed PR #340 against `main` at the frozen head above. The Rust review lane is included here. The changed generator accepts caller work and source limits, resolves type nodes with explicit frames, renders nested values as flat statements, and partitions declaration constructors into bounded modules. The public error mapping, source and publication checks, sibling handling, and tests were examined against FR-018-AC-28 through AC-31 and TC-029.

## Verdict

**PASS.** No code or test defect was found in this diff. No source files or CI workflows were edited by this review. The author's pre-PR `make ci` and selected Kani/replay results were reported on the PR at this exact head; this independent review inspected code and computed trace bindings, without repeating the full gates or heavy generated-crate test.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

- FR-018-AC-28 examined: 100,001-entry acyclic chain, default and selected work limits, generated oracle and shallow result on a 512 KiB stack; `src/oracle/equality/mod.rs:2720`.
- FR-018-AC-29 examined: charged repeated entry, cycle and work precedence, healthy sibling disposition; `src/oracle/equality/mod.rs:2583`.
- FR-018-AC-30 examined: source limit at default, measured exact size and one byte below, bounded generated modules, compilation and execution; `src/oracle/equality/mod.rs:2693` and `src/oracle/equality/mod.rs:2720`.
- FR-018-AC-31 examined: public rejection of `u64::MAX` before lowering and `u64::MAX - 1` counter seam; `tests/it/composite_equality_generation.rs:141` and `src/oracle/equality/mod.rs:2671`.
- Rust idioms, panic surface, test seams, duplication, public API and changed-file test alignment examined. No applicable AssuranceProfile was found under `spec/`. `.github/workflows` is unchanged.
