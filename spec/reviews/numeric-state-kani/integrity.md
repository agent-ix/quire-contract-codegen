---
id: SR-017
title: "Integrity review of numeric and state Kani lowering"
type: SpecReview
analysis: integrity
scope: "StR-001, FR-003, NFR-001, NFR-002, interface-001, TC-003/005/007/014 and TM-001"
review_set: subset
---

## Summary

The integrity review reconciled the generalized ABI, exact semantic source, model-domain ownership,
proof-readiness vocabulary, diagnostics, evidence identity, and downstream replay boundary. Three
ambiguities were resolved without absorbing the sibling strategy work or the downstream native run.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-003's stakeholder, reproducibility, and provenance relationships were only inferable through FR-001 and matrix prose. StR-001, NFR-001, and NFR-002 now carry direct FR-003 edges. | StR-001; FR-003; NFR-001; NFR-002 |
| FND-002 | medium | Argument binding and post-state result binding were expressed as one non-atomic requirement. They are now separate SHALL statements with zero, one, and multiple result cases. | FR-003; TC-014 |
| FND-003 | medium | “Normalized dependency identity” did not say whether observation participated in uniqueness or how duplicate references behaved. interface-001 now names kind, normalized path, and observation as the key, unifies identical references, and refuses incompatible declarations. | interface-001; TC-003; TC-014 |

## Trace and consistency

| Stakeholder | Requirement | Constraints | Interface | Verification |
| --- | --- | --- | --- | --- |
| StR-001-VC-2 | FR-003-AC-2/5/6/7/8 | NFR-001 reproducibility; NFR-002 identity/refusal | interface-001 `kani_slice` | TC-003, TC-005, TC-007, TC-014; SUITE-008 |

The supported interpretation is singular: the adapter reuses the executable-oracle analyzer and
rendered predicates; direct current/pre values become ordered primitive arguments; direct post
values become an ordered primitive result; and checked IR domains alone constrain integers. Model
bounds are not proof dependency edges, graph readiness is not proof execution, and generation
attestations do not claim Kani success. Agent E's strategy interface supplies no Kani range, and SL
IT-010—not codegen—constructs and judges the native replay.
