---
id: "SR-11249"
title: "Scope-boundary analysis — IR-791 attested scalar"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/matrix/TC-027-kani-execution-evidence.md; spec/replay/functional/FR-032-routed-scalar-replay-binding.md"
review_set: all
---

## Summary

CG owns staging, proof/native execution, measured artifact identity and the opaque receipt. IR-773 owns wrapping it with QSL occurrence, complete claim/replay request and ReplayLimits before QSL evaluation; settlement and terminal creation remain outside this CG requirement. No dependency boundary conflict was found.

## Verdict

**PASS** — no method-specific defect was found in the reviewed changed scope. The base checklist findings are recorded in SR-11243.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
