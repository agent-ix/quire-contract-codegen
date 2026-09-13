---
id: SR-012
title: "Risk and complexity review of numeric and state oracle lowering"
type: SpecReview
analysis: risk-complexity
scope: "FR-001 numeric/state slice and its pinned IR/runtime boundaries"
review_set: subset
---
## Summary

The slice is medium technical risk and low product volatility. The dominant hazards are collapsing
invalid arithmetic into false and deriving parameter types or spans from the wrong IR node; the
reviewed grammar and differential controls directly constrain both.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No issues found after the undefined-result and locus dispositions | - |

## Risk register

| Requirement | Technical risk | Volatility | Drivers | Mitigation |
| --- | --- | --- | --- | --- |
| FR-001-AC-2/8 | medium | low | First typed i64 dependency/signature lowering and six comparison operators | Exhaustive operator corpus, typed-node mapping, generated-rust compilation, independent evaluator |
| FR-001-AC-4 | medium | low | Fail-closed grammar and source-span identity across nested expressions | Unconditional arithmetic refusal, deterministic preorder locus, negative corpus with exact spans |
| NFR-001-AC-1 | low | low | New parameter types can perturb deterministic bytes | Repeat each numeric/state generation and compare every artifact byte |

## Top hazards

1. A checked arithmetic failure is unwrapped or defaulted into Boolean false. The grammar prohibits
   every arithmetic and numeric-negation node until a versioned invalid-result API exists.
2. A dependency's `bool`/`i64` type or pre/current/post observation is taken from its name rather
   than the matching public typed node. Compilation and mixed-signature cases expose that error.
3. An unsupported nested node reports the clause or a neighboring node. TC-003 checks the declared
   preorder winner's exact source identity and endpoints.

The SL/codegen IR revision reconciliation remains explicitly assigned to the later SL pin-bump step
in issue #83; this crate implements only its already pinned public IR contract.
