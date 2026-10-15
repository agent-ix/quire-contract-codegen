---
id: "SR-11244"
title: "Failure-domain analysis — IR-791 attested scalar"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/matrix/TC-027-kani-execution-evidence.md; spec/replay/functional/FR-032-routed-scalar-replay-binding.md"
review_set: all
---

## Summary

Identity, extension/failure behavior, process boundaries and mutation cases were examined. The receipt binds scalar context and returns no receipt on typed execution/snapshot failure; the base review records the two acceptance gaps.

## Verdict

**PASS** — no method-specific defect was found in the reviewed changed scope. The base checklist findings are recorded in SR-11243.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
