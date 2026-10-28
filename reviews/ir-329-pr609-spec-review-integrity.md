---
id: SR-609003
title: 'spec-review integrity analysis of IR-329 CG PR #609'
type: SpecReview
analysis: integrity
scope: 'agent-ix/quire-contract-codegen; PR #609 IR-329; spec/spec.md (reverse inventory), spec/kani/functional/FR-015-bounded-kani-obligations.md (IR-464 status paragraph); context spec/kani/functional/FR-041-claimed-module-proof-gate.md'
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-015
  type: references
- target: ix://agent-ix/quire-contract-codegen/FR-041
  type: references
---

## Summary

Ticket: IR-329. Integrity (completeness and consistency) analysis of the reconciled ownership map,
the new reverse inventory, and the rewritten FR-015 status prose at frozen head
`450a380b235630d46a71c37ff39e9712a31a0ee2`, checked against the computed matrix (`quoin matrix --json`).

## Verdict

PASS WITH LOW FINDINGS — the map is consistent with FR-015's AC statuses: AC-38 to AC-43, AC-46,
AC-48/49 PLANNED; AC-47 tagged but PLANNED, described as "full AC-47 remain planned or partial";
AC-96/99 PARTIAL; AC-110/111 PLANNED and untagged. The sentence that matrix tags are not proof
verdicts guards against reading tagged as done. Two low inconsistencies remain.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Reverse inventory groups FR-041 with planned-only requirements and names no code home, though FR-041-AC-1 to AC-5 are tagged and live in scripts/cg_gates.rs, scripts/kani_claims.json and tests/it/kani_generation_claims.rs / kani_gate.rs outside src/kani | spec/spec.md:149, tests/it/kani_gate.rs:388, tests/it/kani_generation_claims.rs:511 |
| FND-002 | low | The FR-015 "Implemented (IR-464)" paragraph says AC-53 to AC-58 "retain the original seven-kind cover obligations", yet AC-54/56 are RETIRED and AC-53/58 are OPEN for surviving kinds only | spec/kani/functional/FR-015-bounded-kani-obligations.md:46-51, spec/kani/functional/FR-015-bounded-kani-obligations.md:803-808 |

## Examined Scope

```yaml
scope:
- {id: spec/spec.md#reverse-inventory, path: spec/spec.md, role: examined, excerpt: "FR-034/040, FR-041 and FR-046 retain planned obligations without a completed implementation claim here."}
- {id: FR-015, path: spec/kani/functional/FR-015-bounded-kani-obligations.md, role: examined, excerpt: "FR-015-AC-53 to FR-015-AC-58 retain the original seven-kind cover obligations and the guard against an emitted harness without one. The V1 bundle and contract kinds have retired; the surviving V2 kinds still require their own evidence."}
- {id: FR-041, path: spec/kani/functional/FR-041-claimed-module-proof-gate.md, role: context_only, excerpt: "A stubbed executor, predetermined outcome or build-only run produces no qualifying module evidence and cannot pass the gate."}
```
