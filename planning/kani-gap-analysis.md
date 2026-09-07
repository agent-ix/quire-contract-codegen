---
id: REV-009
title: "Kani lowering implementation gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "issue #2 bounded Kani contracts, proof harnesses, and dependency graph implementation"
review_set: subset
---

# Kani lowering implementation gap analysis

## Summary

The bounded adapter implements the pre-reviewed Kani generation surface using shared oracle
semantics and Quoin-owned proof attestations. Independent current-head review remains open.

## Current verdict

The bounded implementation is ready for independent current-head review. It is not a landing verdict,
proof-completion claim, TM-001 promotion, or issue closure.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-601 | high | Closed locally: missing/failed required dependencies derive `incomplete`; assumptions/stubs derive `conditional`; the graph always states `proofExecutionState: not_run`. | FR-003-AC-1, TC-005 |
| FND-602 | high | Closed locally: generated Kani predicates are the exact executable-oracle sources, and representative unconditional/conditional bundles execute under cargo-kani 0.67.0. | FR-003-AC-2, TC-007 |
| FND-603 | high | Closed locally: dependency kind/state/path combinations fail closed, graph edges are sorted, and assume/stub source-site markers are bijective. | FR-003-AC-1, REV-008 |
| FND-604 | high | Closed locally: the deprecated repository-local derivation envelope is absent; Rust and graph outputs each carry a Quoin ProofAttestationV1 body that seals through the real Quoin CLI. | MP-001, interface-001 |
| FND-605 | medium | Open process gate: independent exact-head code review and retained local evidence are required before landing. | issue #2, MP-001 |
| FND-606 | medium | Intentional boundary: wider signatures, non-Boolean domains, future Kani versions, and proof-result intake require separate work and adapter profiles. | FR-003-AC-3, interface-001 |

## Verification performed

`cargo test --test kani_generation` covers structured rejection, all readiness classes, schema
validation and mutation, exact oracle reuse across all five Boolean operators, generated crate
compilation, Quoin sealing, and actual pinned Kani execution. Full-repository verification remains a
current-head landing prerequisite.
