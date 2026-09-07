---
id: REV-008
title: "Kani lowering and proof-dependency preimplementation review"
type: SpecReview
analysis: gap-analysis
scope: "issue #2 bounded Kani contracts, proof harnesses, and dependency graph"
review_set: subset
---

# Kani lowering and proof-dependency preimplementation review

## Summary

FR-003, TC-003, TC-005, TC-007, interface-001, and Task-005 bound the first Kani adapter to the
installed 0.67.0 backend and the accepted Boolean-oracle semantics.

## Verdict

**PASS to implement the bounded first slice.** This authorizes a draft implementation; it does not
classify a proof complete, promote TM-001, or close issue #2. The exact adapter is cargo-kani 0.67.0
with function contracts enabled. Generation always records proof execution as `not_run`.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-501 | high | Reuse the same validated typed clauses and exact generated Boolean-oracle predicates; do not create a second predicate interpreter. | FR-003-AC-2, TC-007 |
| FND-502 | high | Derive ready, conditional, or incomplete from the complete dependency census; missing/failed required edges can never become ready. | FR-003-AC-1, TC-005 |
| FND-503 | high | Isolate experimental syntax and options behind `kani-0.67.0-function-contracts-v1`; reject every other requested backend version. | FR-003-AC-3, FR-003-AC-4 |
| FND-504 | high | Generate every assume/stub source site from one typed edge and require a bijection with graph edges. | FR-003-AC-1, TC-005 |
| FND-505 | medium | Bound the first slice to one Boolean input, one Boolean state, and an explicit `fn(bool, bool) -> bool` subject path. | interface-001 |
| FND-506 | medium | Emit one Quoin ProofAttestationV1 body per output and keep artifact-generation success distinct from Kani proof execution. | MP-001, interface-001 |

## Decision

Proceed locally. Verification must include schema mutation probes, exact predicate reuse, generated
crate compilation, and representative execution by the pinned Kani backend. Hosted CI remains off.
