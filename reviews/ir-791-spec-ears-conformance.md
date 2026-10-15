---
id: "SR-11250"
title: "EARS analysis — IR-791 attested scalar"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/matrix/TC-027-kani-execution-evidence.md; spec/replay/functional/FR-032-routed-scalar-replay-binding.md"
review_set: all
---

## Summary

The changed FR-017 behavior statements use an event trigger or an explicit system subject. `quire validate` reported only an existing warning at the unchanged line 226 and another warning in untouched FR-014; no new EARS finding applies to this diff.

## Verdict

**PASS** — no method-specific defect was found in the reviewed changed scope. The base checklist findings are recorded in SR-11243.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
