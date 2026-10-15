---
id: "SR-11245"
title: "Integrity analysis — IR-791 attested scalar"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/matrix/TC-027-kani-execution-evidence.md; spec/replay/functional/FR-032-routed-scalar-replay-binding.md"
review_set: all
---

## Summary

The changed ACs map to TC-027, the procedure states the implementation tests are still unrun, and validation found no duplicated new IDs or conflicting requirements. The test-vector and identity-scope gaps are recorded in the base review.

## Verdict

**PASS** — no method-specific defect was found in the reviewed changed scope. The base checklist findings are recorded in SR-11243.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
