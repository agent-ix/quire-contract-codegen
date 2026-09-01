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

The bounded Kani adapter is implemented with shared oracle semantics, fail-closed dependency
classification, version-pinned syntax/options, structured diagnostics, and pending-only generation
evidence. Clean-head evidence and independent exact-head review remain open.

## Scope reviewed

This analysis compares the first `cargo-kani 0.67.0` adapter implementation against FR-003,
interface-001, TC-003, TC-005, TC-007, REV-008, and issue #2. It does not promote TestMatrix rows,
classify a generated proof complete, or approve the draft for merge.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-601 | high | Closed in implementation: missing or failed required dependencies derive `incomplete`, while assumptions and stubs derive `conditional`; generation cannot claim proof completion. | FR-003-AC-1, TC-005 |
| FND-602 | high | Closed in implementation: generated Kani predicates are the exact executable-oracle sources and representative bundles execute under the pinned backend. | FR-003-AC-2, TC-007 |
| FND-603 | high | Closed in implementation: assume/stub source sites and graph edges are bijective and kind/state combinations fail closed. | FR-003-AC-1, REV-008 |
| FND-604 | medium | Open at this draft stage: clean-head retained evidence and independent exact-head review are still required. | MP-001, issue #2 |
| FND-605 | medium | Open beyond this bounded slice: wider function signatures, non-Boolean domains, and future Kani versions require separate adapter profiles. | FR-003-AC-3, interface-001 |

## Implemented controls

| Requirement | Implementation and verification | Disposition |
|---|---|---|
| FR-003-AC-1 | A validated, sorted dependency census derives `ready`, `conditional`, or `incomplete`; missing/failed required edges and assumed/stubbed edges have direct fixtures. | implemented, review pending |
| FR-003-AC-2 | Kani embeds the exact generated executable-oracle source for every supported Boolean operator, and the pinned backend executes representative unconditional and conditional proofs. | implemented, review pending |
| FR-003-AC-3 | Backend version, identity, subject binding, dependency combinations, manifest status, and unwind bounds fail with structured terminal diagnostics. | implemented, review pending |
| FR-003-AC-4 | The graph and PGM-01 manifest retain adapter/version, exact harness selector, unwind, solver, conditional stubbing flag, dependency/source-site census, and artifact identities. | implemented, review pending |
| REV-008 FND-504 | Every generated assume/stub site is derived from one validated edge, uses a digest-bound source-site identity, and is checked for source/graph bijection. | implemented, review pending |

## Residual gaps and boundaries

- The adapter intentionally supports only one Boolean input, one Boolean pre/post state, and a
  customer function with signature `fn(bool, bool) -> bool`; other bindings are explicit
  `unsupported` diagnostics.
- Dependency `ready` is not proof completion. Generation manifests remain `pending` with
  `proofExecutionState: not-run`; execution evidence and dependency closure must be combined by a
  later evidence consumer before a complete-proof claim.
- Kani function contracts and stubbing remain version-pinned experimental surfaces. Any syntax or
  behavior change requires a new adapter profile rather than silent reuse of this one.
- The full Task-005 backend scope is not closed: LLVM vacuity work proceeds in a separate ticket,
  and the TestMatrix remains planned until independent review and program-level parity closure.
- Retained clean-head evidence and exact-head code review remain open at this draft stage.

## Verdict

The bounded implementation covers issue #2's generation controls without a known false-completeness
path. Keep the PR in draft until clean-head local evidence is retained and an independent exact-head
review clears all findings.
