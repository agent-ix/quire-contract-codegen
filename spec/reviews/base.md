---
id: SR-008
title: "Base review of numeric and state oracle lowering"
type: SpecReview
analysis: base
scope: "FR-001, interface-001, TC-002, TC-003, TM-001 at 393c6a1 plus review dispositions"
review_set: subset
---
## Summary

The base review checked the issue #4 numeric/state amendment for identity, traceability, concrete
interfaces, adverse behavior, boundary coverage, and honest pre-implementation status. Two gaps were
resolved: deterministic numeric regeneration is now explicit, and expression failures now have a
code-specific source-span contract.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-002 did not explicitly repeat numeric/state generation even though FR-001-AC-8 requires deterministic artifacts. The procedure now requires byte-identical Rust, source-map, and attestation outputs. | FR-001-AC-8; TC-002 |
| FND-002 | medium | interface-001 described the IR source span as optional without stating which expression failures require it. The interface now defines the four span-bearing codes and deterministic locus selection. | FR-001-AC-4; interface-001; TC-003 |

## Checks

FR-001 directly satisfies StR-001 and implements interface-001; this library specification contains
no US artifacts, so the user-story checklist does not create a missing local relationship. Every
FR-001 criterion names TC-001, TC-002, TC-003, or TC-006. TC-002 covers every comparison operator,
Boolean and integer parameter types, current/pre/post observations, domain endpoints and adjacent
outside values. TC-003 covers obligations, scalar roots, all numeric arithmetic/negation, indirect
dependencies, object/graph reads, and every other node. There are no option combinations or state
transitions in this slice.

TM-001 keeps FR-001-AC-8 and TC-002 planned until implementation evidence exists. The deliberate
TC-011 gap is documented as a removed pre-stable evidence identity and is not reused or moved.
Quire found no duplicate IDs, broken links, untracked symbols, or falsely complete status rows in
the reviewed scope.
