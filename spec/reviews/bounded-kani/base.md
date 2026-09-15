---
id: SR-022
title: "Base review of bounded Kani codegen expansion"
type: SpecReview
analysis: base
scope: "spec/index.md, FR-007, TC-023, and Contract IR FR-029 through FR-031"
review_set: all
---
# SR-022: Base review of bounded Kani codegen expansion

## Summary

The reviewed expansion links the new codegen corpus requirement to StR-001,
the existing strategy and Kani requirements, Contract IR's accepted bounded
profile, and one concrete integration test case. Every new acceptance criterion
has a declared verification method and TC-023 retains planned status until the
implementation corpus exists.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No base-review gap remains after adding explicit profile, adverse-input, replay, and dependency-direction criteria. | FR-007, TC-023 |
