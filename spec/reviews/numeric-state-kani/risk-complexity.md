---
id: SR-019
title: "Risk and complexity review of numeric and state Kani lowering"
type: SpecReview
analysis: risk-complexity
scope: "StR-001, FR-003, NFR-001, NFR-002 and pinned Kani/IR/runtime boundaries"
review_set: subset
---

## Summary

FR-003's numeric/state increment is high technical risk and medium external-contract volatility: it
introduces a generalized symbolic ABI on unstable pinned Kani syntax and must preserve exact oracle
semantics. The narrow comparison grammar, v2 adapter, actual cargo-kani runs, and independent corpus
checks are the named mitigations.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No unmitigated high-risk or high-volatility requirement remains in the reviewed slice. | - |

## Risk register

| Requirement | Technical risk | Volatility | Drivers | Mitigation |
| --- | --- | --- | --- | --- |
| StR-001-VC-2 | medium | low | Cross-backend semantic agreement can be tautological when renderers are shared | Reuse exact predicates but independently evaluate the bounded corpus and retain downstream native replay |
| FR-003-AC-2/5/6/7 | high | medium | First generalized Boolean/i64 Kani ABI; function-contract and concrete-playback syntax is unstable | Pin cargo-kani 0.67.0 and executable digest, isolate v2 adapter, compile generated crates, run healthy and falsifying exact harnesses |
| FR-003-AC-3 | medium | low | Unsupported typed shapes could fall through to unconstrained values | Closed grammar, stable refusal mapping, exact SourceSpan, negative mutation corpus, no partial bundle |
| FR-003-AC-4/8 | medium | low | Typed binding order, options or schema identity can drift nondeterministically | Canonical dependency key/order, v2 schemas, full option vector, repeated byte comparison, preserve v1 files |
| NFR-001-AC-1 | low | low | Additional tuple/domain metadata can perturb bytes | Normalized-order permutation and byte-identical regeneration |
| NFR-002-AC-1/3 | medium | low | A ready graph or generation attestation could be mistaken for a proof result | Always retain `proofExecutionState: not_run`; keep execution observation and graph readiness distinct |

## Top hazards

1. Kani quantifies over machine-wide `i64` or a strategy range instead of the exact checked IR
   domain. TC-014 inspects assumptions and separates endpoints from adjacent outside controls.
2. Post-state tuple ordering differs from oracle dependency ordering. The full checked identity is
   canonical and source/census permutations must reproduce the same bytes.
3. Shared rendering hides a common semantic defect. The suite evaluates predicates independently,
   and SL IT-010 later replays the concrete counterexample through native runtime execution.
4. Concrete-playback syntax changes. The exact 0.67.0 executable/profile/options are evidence inputs;
   unbound output cannot be promoted or replayed.

The failure-domain review contains the corresponding framing, extension-point, and topology controls.
