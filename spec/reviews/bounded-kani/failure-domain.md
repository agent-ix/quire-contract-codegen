---
id: SR-023
title: "Failure-domain review of bounded Kani codegen expansion"
type: SpecReview
analysis: failure-domain
scope: "FR-007, TC-023, Contract IR FR-029 through FR-031"
review_set: all
---
# SR-023: Failure-domain review of bounded Kani codegen expansion

## Summary

The reviewed requirement rejects adverse finite populations before harness
generation, retains object/reference identity and graph topology failures, and
preserves unavailable or mismatched native replay as non-Boolean. The generator
does not own native semantic execution and cannot use an assumption to erase a
failed input boundary.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No failure-domain gap remains: absent, dangling, foreign, wrong-type, unavailable, over-bound, cyclicly bounded, and replay-adverse paths have a typed non-generation response. | FR-007, TC-023 |
