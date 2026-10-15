---
id: "SR-11248"
title: "Risk and complexity analysis — IR-791 attested scalar"
type: SpecReview
analysis: risk-complexity
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/matrix/TC-027-kani-execution-evidence.md; spec/replay/functional/FR-032-routed-scalar-replay-binding.md"
review_set: all
---

## Summary

This is a high technical-risk, low-volatility change because generated code execution, proof identity, sandboxing and typed native observation cross trust boundaries. The spec names private staging, write isolation, path/byte mutation controls, typed failure and a real Kani run; no additional requirements defect was found.

## Verdict

**PASS** — no method-specific defect was found in the reviewed changed scope. The base checklist findings are recorded in SR-11243.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
