---
id: "SR-11243"
title: "Base spec review — IR-791 attested scalar proof/native receipt"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/matrix/TC-027-kani-execution-evidence.md; spec/replay/functional/FR-032-routed-scalar-replay-binding.md"
review_set: all
---

## Summary

Base checklist reviewed the added FR-017 ACs and TC-027 procedure, validation and computed matrix. Two acceptance gaps remain: binary operand order is optional in the test procedure, and the canonical identity scope does not name all staged executable inputs.

## Verdict

**FAIL** — the proof/native receipt acceptance does not require a binary order-sensitive case and its canonical identity omits staged executable inputs.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-027 makes a two-operand order check conditional, so the required single positive fixture can be unary and cannot prove ordered native invocation. | spec/kani/matrix/TC-027-kani-execution-evidence.md:165-184 |
| FND-002 | high | The canonical identity excludes staged adapter and resolved lock bytes, despite both being executed, so it does not uniquely bind the execution inputs used for the Kani/native observation. | spec/kani/functional/FR-017-kani-execution-evidence.md:101-104 |
