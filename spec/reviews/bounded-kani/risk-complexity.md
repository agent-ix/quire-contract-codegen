---
id: SR-027
title: "Risk and complexity review of bounded Kani codegen expansion"
type: SpecReview
analysis: risk-complexity
scope: "FR-007, TC-023"
review_set: all
---
# SR-027: Risk and complexity review of bounded Kani codegen expansion

## Summary

FR-007 is high technical risk because it combines an external proof backend,
three semantic families, generated artifacts, and independently implemented
native replay. Volatility is medium because the profile and backend adapter are
versioned external contracts. The requirement mitigates this with one public
profile pin, modular strategy/oracle/harness lanes, exact corpus parity, and
typed adverse outcomes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The high-risk scope is explicitly isolated behind the reviewed public profile pin and one corpus integration test; implementation must keep semantic-family lanes separate until TC-023 can join them. | FR-007, TC-023 |
