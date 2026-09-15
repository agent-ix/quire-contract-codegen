---
id: SR-028
title: "Scope-boundary review of bounded Kani codegen expansion"
type: SpecReview
analysis: scope-boundary
scope: "FR-007, Contract IR FR-029 through FR-031, AD-001"
review_set: all
---
# SR-028: Scope-boundary review of bounded Kani codegen expansion

## Summary

Contract IR owns bounded-profile meaning, finite ABI validation, typed outcomes,
and the native replay boundary. Codegen owns deterministic artifact generation,
generated strategies, Kani harnesses, proof graphs, and corpus parity. Kani is
an external version-pinned executable, and its result cannot become a native or
release decision by itself.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No ownership collision remains: the specification preserves the one-way Contract IR-to-codegen dependency and does not assign TL or Protocol work to this campaign. | FR-007-CON-1, FR-007-CON-2, AD-001 |
