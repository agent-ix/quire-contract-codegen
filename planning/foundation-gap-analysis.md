---
id: SR-001
title: "Codegen foundation gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "codegen requirements, assurance, dependency pins, foundation evidence, and release gates"
review_set: subset
---

# Foundation gap analysis

## Summary

The specification and assurance foundation is internally consistent and locally validated. Semantic
work remains blocked by the unavailable authoritative IR corpus and by provisional upstream review,
remote checks, and human decisions.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | The authoritative IR schema and conformance corpus have no candidate revision. | IR #10, FR-001 |
| FND-002 | high | PGM-01 review findings and accepted governance identity remain open. | PGM-01, REV-003 |
| FND-003 | medium | Runtime review, deliberate remote CI, CODEOWNER approval, and human release remain pending. | runtime PR #5, AA-001, REV-004 |

Reviewed source revision: `8a4537d32002e1f2e984508ec8d89b300215855d`

Retained evidence: `evidence/foundation-8a4537d32002-20260830T225332Z/sha256sums.txt`

## Requirement and evidence matrix

| Requirement | Foundation disposition | Semantic verification | Remaining gap |
|---|---|---|---|
| StR-001 | specified and traced to FR-001 through FR-005 | demonstration pending | reconcile all released upstream identities |
| FR-001 | deterministic oracle, manifest, diagnostic, and identity behavior specified | TC-001 through TC-003 pending | authoritative IR corpus and runtime API unavailable |
| FR-002 | tri-state and shaped-strategy behavior specified | TC-004 pending | executable IR semantics unavailable |
| FR-003 | Kani lowering and dependency-closure behavior specified | TC-003, TC-005, and TC-007 pending | proof mapping and authoritative corpus unavailable |
| FR-004 | vacuity, rejection, observation, and source-map behavior specified | TC-006 pending | coverage-region schema and executable outputs unavailable |
| FR-005 | atomic CLI and cross-backend parity behavior specified | TC-001, TC-002, and TC-007 pending | semantic backends not implemented |
| NFR-001 | zero-difference and zero-partial-publication thresholds specified | repeated golden and fault-injection runs pending | implementation unavailable |
| NFR-002 | identity, license, qualification, and explicit-state boundaries specified | schema and corpus inspection pending | released dependency identities unavailable |
| interface-001 | provisional input, operation, output, diagnostic, and evidence contract specified | compatibility tests pending | reconcile with accepted IR and runtime interfaces |
| TestMatrix | 29 acceptance criteria mapped to seven planned semantic cases; placeholder tests explicitly unbound | all semantic cases pending | authoritative IR corpus and runtime release unavailable |
| AP/AD/CAC/MP/AA-001 | intended use, architecture, contract, measurements, and open claim specified | independent and human review pending | governance, dependency, CI, and release evidence open |
| PLAN-001 | typed foundation, dependency, semantic, parity, and human-release tasks with explicit statuses | Task-001/002 done; Task-003 in progress; Task-004 through Task-007 not started | IR candidate, reviews, implementation, and human authority remain open |

## Foundation evidence result

The retained bundle records a clean source revision and successful specification validation,
formatting, Clippy, unit/integration tests, explicit Rust 1.75 compatibility, license policy,
unsafe-code audit, metadata capture, and warning-denied documentation generation. The local input
schema now rejects unknown tool and dependency fields and requires exact PGM/runtime identities.
All 39 checksum entries verify; the local schemas, exact PGM-01 candidate schema, and custom
validator all accept their respective records with zero errors.

The current placeholder crate tests establish only scaffold health. They do not satisfy any semantic
test case or close the assurance claim.

## Open dependencies and remote evidence

- PGM-01 PR #12 is pinned at `e928ce3703d238856544c65bc4aa7c30ea16dac1` and provisionally
  reconciled; review, merge, and final identity reconciliation remain open.
- The authoritative IR schema/corpus (`agent-ix/quire-contract-ir#10`) has no candidate revision.
- Runtime PR #5 is provisionally pinned at
  `5bdb26feb0b310405bf7ac7f94bf128869386898`; current remote checks, review, final
  reconciliation, and release remain open.
- The manual-only CI workflow change is provided by codegen PR #8. This branch is stacked on that
  exact commit and must be rebased onto `main` after PR #8 merges.
- Default-branch protection was observed and retained in
  `evidence/foundation-remote/branch-protection.md`: strict `Rust Checks` and `License Check`, one
  CODEOWNER approval, administrator enforcement, conversation resolution, and no force push or
  deletion. A deliberately dispatched remote CI run, approval, and human source-release decision
  remain pending.

## Conclusion

The 39-document foundation specification, typed plan bundle, and local evidence procedure are ready
for draft review. Semantic implementation and release claims are not ready. Rebase and dependency
reconciliation are required before any semantic child leaves draft.
