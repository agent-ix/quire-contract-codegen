---
id: "SR-11247"
title: "Evidence analysis — IR-791 attested scalar"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/matrix/TC-027-kani-execution-evidence.md; spec/replay/functional/FR-032-routed-scalar-replay-binding.md"
review_set: all
---

## Summary

`quoin advise` found no authored Test-method mismatch for FR-017-AC-27..30. The computed matrix currently marks those four new ACs untagged because the implementation tests do not exist on this spec-only head; TC-027 explicitly reserves those tests for the implementation PR.

## Verdict

**PASS** — no method-specific defect was found in the reviewed changed scope. The base checklist findings are recorded in SR-11243.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
