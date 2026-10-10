---
id: SR-6306
title: IR-494 scope-boundary review
type: SpecReview
analysis: scope-boundary
scope: agent-ix/quire-contract-codegen@d9955f18171edc58c4f3db1f8efbb715277b465f; spec/decisions/ADR-001-overlapping-generators-and-input-models.md, spec/decisions/ADR-006-caller-text-admission-boundary.md, spec/oracle/functional/FR-035-caller-text-admission.md
review_set: subset
---

## Summary

Ticket: IR-494. Caller-owned validated payloads cross into CG-generated admission; Contract Runtime owns the exact admission semantics, while CG owns oracle generation and bounded proof shape. Checked-package comparison remains FR-014. The omitted production proof executor is already recorded in SR-6301; no further boundary defect was found.

## Reviewed units

| Unit | Role | Excerpt |
| --- | --- | --- |
| ADR-001 Q3 | examined | Every checked-expression or contract-claim generator reads an admitted `quire.checked-package/v2` package through Contract IR's `CheckedPackageV2`. |
| ADR-006 Decision | examined | The caller Text-admission oracle and its proof form a separate, explicitly caller-declared ingress route under FR-035. |
| FR-035 Inputs | examined | A runtime `TextPayload` made by the public `TextPayload::from_utf8(&[u8])` constructor, and a caller-supplied `Meter`. |
| FR-035 Outputs | examined | A deterministic generated admission oracle and its caller-ingress claim identity, separate from FR-014's per-node claim map. |
| FR-035-AC-6 | examined | A request outside the declared finite proof class has a typed unsupported or requires-bound disposition and no harness, and no caller-ingress claim is marked `ir_confirmed` or assigned a checked-package node id. The six attempted text-operand `numeric.convert` nodes remain Contract IR refusals and do not count as passing caller-admission coverage. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

No additional findings in this method.
