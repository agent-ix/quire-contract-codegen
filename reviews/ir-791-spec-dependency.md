---
id: "SR-11246"
title: "Dependency analysis — IR-791 attested scalar"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/matrix/TC-027-kani-execution-evidence.md; spec/replay/functional/FR-032-routed-scalar-replay-binding.md"
review_set: all
---

## Summary

FR-017 depends on routed generation and exact scalar input; QSL replay and terminal settlement remain downstream and out of CG scope. IR-773’s private context wrapper can retain occurrence and replay request context without adding QSL fields to the CG receipt.

## Verdict

**PASS** — no method-specific defect was found in the reviewed changed scope. The base checklist findings are recorded in SR-11243.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
