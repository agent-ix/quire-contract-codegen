# Foundation gap analysis

Reviewed source revision: `c1b8990a159732619ba6533deed583dfd27cd86a`

Retained evidence: `evidence/foundation-candidate/sha256sums.txt`

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
| AP/AD/CAC/MP/AA-001 | intended use, architecture, contract, measurements, and open claim specified | independent and human review pending | governance, dependency, CI, and release evidence open |

## Foundation evidence result

The retained bundle records a clean source revision and successful specification validation,
formatting, Clippy, unit/integration tests, license policy, unsafe-code audit, metadata capture, and
warning-denied documentation generation. Its checksum manifest verifies all 22 captured files.

The current placeholder crate tests establish only scaffold health. They do not satisfy any semantic
test case or close the assurance claim.

## Open dependencies and remote evidence

- PGM-01 (`agent-ix/quire-contract-ir#3`) remains open.
- The authoritative IR schema/corpus (`agent-ix/quire-contract-ir#10`) has no candidate revision.
- Runtime PR #5 is provisionally pinned at
  `4392a2385f95defdeef2ee883fcc8024cab1d168`; review and release remain open.
- The manual-only CI workflow change is provided by codegen PR #8. This branch is stacked on that
  exact commit and must be rebased onto `main` after PR #8 merges.
- A deliberately dispatched remote CI run, branch-protection record, independent review, and human
  source-release decision remain pending.

## Conclusion

The foundation specification and local evidence procedure are ready for draft review. Semantic
implementation and release claims are not ready. Rebase and dependency reconciliation are required
before any semantic child leaves draft.
