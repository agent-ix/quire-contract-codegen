---
id: SR-015
title: "Base review of numeric and state Kani lowering"
type: SpecReview
analysis: base
scope: "FR-003, interface-001 kani_slice, TC-003/005/007/014, TM-001, AD-001, MP-001, SUITE-008"
review_set: subset
---

## Summary

The base review checked the codegen #2 numeric/state Kani amendment for identity, traceability,
atomic requirements, concrete interfaces, refusal behavior, boundary coverage, and honest
pre-implementation status. The scoped requirements and tests are ready for implementation after the
recorded human acceptance gate; the repository-wide TestMatrix catalog mismatch remains visible.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | One behavior statement bound subject arguments and results with two SHALL clauses. It is now split into independently verifiable argument- and result-binding requirements. | FR-003 |
| FND-002 | medium | TC-014 did not explicitly exercise current-state inputs, mixed primitive types, the multiple-result tuple, normalized-order permutations, or an external subject-signature failure. Those cases are now required. | FR-003-AC-2; FR-003-AC-8; TC-014 |
| FND-003 | medium | The installed TestMatrix schema expects `Coverage Status` while this repository's shared coverage selector and TM-001 use `Status`; whole-spec structural validation therefore exits 1 even though the new rows are grammar-clean and parse into the coverage census. The existing catalog conflict is not represented as green. | TM-001; spec-artifacts-process#77 |

## Checks

FR-003 is directly covered by StR-001 and constrained by NFR-001 and NFR-002. This library spec has
no US artifacts, so the user-story checklist creates no missing local story edge. Every FR-003
criterion maps to TC-003, TC-005, TC-007, or TC-014; TC-014 covers Boolean and all six integer
comparisons, zero/one/multiple result shapes, direct current/pre/post observations, 0 and 1000 model
endpoints, -1 and 1001 controls, proof success, falsifying playback, dependency states, exact pins,
and unsupported inputs. Optional stubbing and non-stubbing configurations are both present.

TC-014 and all numeric/state FR-003 matrix rows remain Planned. Quire reports no status lies for the
scope; unbacked TC-014 rows are the expected pre-implementation state. TC-011 is a documented retired
pre-stable evidence identity and was neither reused nor moved.
