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

FR-003, TC-003, TC-005, TC-007, interface-001, and Task-005 define the proof-backend slice. The
accepted Boolean oracle boundary supplies the canonical clause semantics. The first Kani adapter is
bounded to installed `cargo-kani 0.67.0`; function-contract syntax remains experimental and is
enabled only through an exact, recorded adapter profile.

## Verdict

**PASS to implement the bounded first slice.** This verdict authorizes a draft implementation; it
does not classify any proof complete, promote a TestMatrix row, or close issue #2. A generated proof
is complete only after the exact backend executes successfully and its required dependency closure
is independently derived from the retained graph.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-501 | high | Closed by design: Kani requests carry the same validated typed clauses and reuse the Boolean lowering/profile, preventing an independent proof-predicate interpretation. | FR-003-AC-2, TC-007 |
| FND-502 | high | Closed by design: graph classification is derived from a complete dependency census; missing/failed required edges force `incomplete`, while assumptions/stubs force `conditional`. | FR-003-AC-1, TC-005 |
| FND-503 | high | Closed by design: one exact `kani-0.67.0-function-contracts-v1` adapter owns attributes/options; any other requested version/profile is non-generated. | FR-003-AC-3, FR-003-AC-4 |
| FND-504 | high | Closed by design: every generated assume/stub site is created from a typed dependency edge and the source/graph censuses must be bijective. | FR-003-AC-1, TC-005 |
| FND-505 | medium | Closed for the first slice: the request takes an explicit validated subject path and Boolean bindings; no package-level or customer-body adapter is inferred. | interface-001, accepted IR PR #19 |
| FND-506 | medium | Open beyond generation: execution evidence must retain `cargo-kani --version`, options, status, and output before any proof classification is accepted. | FR-003-AC-4, MP-001 |

## Coverage

| Requirement | First-slice verification |
|---|---|
| FR-003-AC-1 / TC-005 | Complete, missing, failed, assumed, and stubbed dependency fixtures with mutation checks against classification laundering. |
| FR-003-AC-2 / TC-007 | Boolean literal/reference/operator corpus evaluated by the executable oracle and generated Kani predicate plan; exact normalized agreement required. |
| FR-003-AC-3 / TC-003 | Unsupported clauses, bindings, dependency identities, and adapter versions return structured non-generated terminal states. |
| FR-003-AC-4 / Inspection | Source, proof graph, manifest, backend version, toolchain profile, options, assumptions, and output identities are retained. |

## Decision

Proceed on a parallel draft based on the exact current oracle branch. Keep framing, binding,
contract, harness, graph, and evidence portions separately inspectable. Do not use unrecorded
`kani::assume`, `kani::stub`, `kani::stub_verified`, or caller-supplied completion flags.
