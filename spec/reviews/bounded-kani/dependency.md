---
id: SR-025
title: "Dependency review of bounded Kani codegen expansion"
type: SpecReview
analysis: dependency
scope: "FR-002, FR-003, FR-007, Contract IR FR-029 through FR-031"
review_set: all
---
# SR-025: Dependency review of bounded Kani codegen expansion

## Summary

The dependency graph is acyclic: accepted Contract IR bounded-profile interfaces
and the reviewed public release pin enable codegen strategy/Kani integration;
existing FR-002 and FR-003 enable FR-007; TC-023 verifies the completed corpus.
The corpus does not make Contract IR depend on codegen.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No cycle is specified. The required reviewed public Contract IR profile pin is an explicit enablement prerequisite, not a codegen-to-IR reversal. | FR-007-CON-1, FR-007-CON-2, FR-007-CON-4 |
