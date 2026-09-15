---
id: SR-026
title: "Evidence review of bounded Kani codegen expansion"
type: SpecReview
analysis: evidence
scope: "FR-007, TC-023"
review_set: all
---
# SR-026: Evidence review of bounded Kani codegen expansion

## Summary

TC-023 is the declared integration evidence for all five FR-007 acceptance
criteria and requires real native, oracle, strategy, Kani, provenance, and
replay observations. The installed Quoin advisor could not read the available
Quire CLI version contract, so no unsupported automated recommendation is
claimed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Evidence-advisor output is inconclusive because `quoin advise --json` cannot determine a compatible Quire CLI JSON version; the authored Test and Inspection methods remain an explicit human judgement. | FR-007-AC-1 through FR-007-AC-5, TC-023 |
