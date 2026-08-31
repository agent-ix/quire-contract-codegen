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

The specification and assurance foundation is locally validated, but it is not release-ready. An IR
schema/corpus is accepted at IR PR #19 and pinned exactly. The runtime source is merged
and its exact reviewed tree is reconciled, while its human source-release decision remains open.

## Verdict

**FAIL.** Semantic TestMatrix rows remain planned and the human source-release decision is pending.
This verdict cannot become PASS from local foundation gates alone.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | Closed: the authoritative IR schema and conformance corpus is accepted and pinned to the exact PR #19 merge. | IR PR #19, FR-001 |
| FND-003 | medium | Runtime PR #5 is merged with the exact reviewed tree; hosted CI is deferred and the human source-release decision remains pending. | runtime PR #5, AA-001, REV-004 |
| FND-004 | low | Stable rustfmt omits nightly-only import grouping options, so import grouping is not enforced. | `rustfmt.toml`, L-1 |
| FND-005 | low | License, unsafe, MSRV, and scaffold-test gates are intentionally low-signal while the crate has no dependencies and only placeholder code. | `Makefile`, L-4 |

FND-001 is closed at accepted IR PR #19 merge
`5c49ebfd1c87415f74420ad047392bd03b1bd202`. FND-002 was retired in commit `e1463f3f7719` as a stale duplicate; its identity is retained
in this history note and is not reused. Hosted CI clauses in FND-003 are deferred by operator
direction; the human decision remains open.

Source identity is recorded once in each immutable evidence bundle's `source-revision.txt` and
manifest. This mutable review document deliberately does not duplicate a source hash or record path
that would become stale on its own next commit.

## Coverage

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
| TestMatrix | all 27 FR acceptance criteria and both StR validation criteria map to planned tests or inspections; placeholder tests explicitly unbound | all semantic cases pending | IR candidate review and runtime release unavailable |
| AP/AD/CAC/MP/AA-001 | intended use, architecture, contract, measurements, and open claim specified | independent and human review pending | governance, dependency, CI, and release evidence open |
| PLAN-001 | typed foundation, dependency, semantic, parity, and human-release tasks with explicit statuses | Task-001/002 done; Task-003 in progress; Task-004 through Task-007 not started | IR candidate, reviews, implementation, and human authority remain open |

## Foundation evidence result

Round 2 invalidated the former authoritative record set: its checksums proved that retained bytes had
not changed, but the verifier did not independently derive outcome values or bind the set of records.
Those six records are now under `evidence/historical/retired-pre-derivation-verifier/` and cannot
support a current claim. The current procedure requires a committed `evidence/ANCHORS` census,
re-derives outcome values and the envelope result, binds every validator transcript, validates JSON
Schema formats with exact dependencies, and seals the complete historical and remote trees.

Because upstream coverage status classification currently reports that it could not perform the
measurement, the coverage command is retained as `inconclusive` even when its process exits zero.
Consequently, the current foundation result is truthfully `pending`, not `conclusive`; local compile,
test, audit, documentation, and schema lanes may pass without upgrading that unavailable measurement.

The foundation evidence toolchain is owned by MP-001 and tested independently of the semantic
TestMatrix. Its vendored PGM-01 schema digest must match the executable pin; its PGM/runtime pins must
agree with the planning reviews; every declared outcome is derived from a retained exit status; and
fixture tests cover envelope identities, roles, digests, extensions, fail-closed pin mismatch,
failure/inconclusive truthfulness, and local validator acceptance/rejection. Semantic coverage remains
zero because these tests make no code-generation claim.

## Review Round 2 disposition

| Finding | Disposition |
|---|---|
| N-1 | Closed: the verifier re-runs outcome derivation and compares complete name/status values plus the derived result and limitations. |
| N-2 | Closed: anchored checksum manifests bind all validator status files and transcripts outside each record. |
| N-3 | Closed: exact evidence requirements and required `date-time`, `uri`, and `uri-reference` format implementations fail closed. |
| N-4 | Closed: `evidence/ANCHORS` is a set-equality boundary; missing/empty verification has a distinct unavailable exit. Historical and remote trees are sealed recursively. |
| N-5 | Closed: any skipped outcome forces `pending` and a named limitation. |
| N-6/N-7 | Closed locally: structured coverage output gates status lies and emits a real inconclusive marker when classification is unavailable. |
| N-8 | Closed locally: ignored Rust tests carrying nearby trace identities fail the coverage-status gate. |
| N-9 | Closed: MP-001 specifies each evidence script and verifier limits; the ownership test requires the actual filename in the plan. |
| N-10 | Closed: every record's externally captured PGM-01 schema digest must match the vendored schema used by verification. |
| N-11 | Partially closed: format validation uses the strict upstream pattern; ecosystem consolidation remains PGM-02 work. |
| N-12 | Closed: checksum census diagnostics distinguish unlisted entries from absent declared entries. |
| N-13 | Closed: security-relevant Make command/toolchain variables use non-overridable exact values. |
| N-14 | Closed: transcript contradiction derives a durable failed outcome instead of aborting assembly before sealing. |
| N-15 | Closed: the dependency is reconciled to accepted IR PR #19 merge `5c49ebfd…`. |
| N-16 | Closed: reviewer-of-record is an explicit collection input; the extension states that this role is not GitHub approval. |
| N-17 | Deferred by operator direction: hosted CI and workflow ownership belong to the separate manual-CI agent; local gate documentation is synchronized. |
| N-18 | Closed: any nested entry changes the recursively anchored historical tree, while any directory inside an authoritative flat record violates its checksum census. |

The current placeholder crate tests establish only scaffold health. They do not satisfy any semantic
test case or close the assurance claim.

The 24 pre-exit-status records under `evidence/historical/untrusted-pre-exit-status/` are deliberately
quarantined and are not authoritative: their historical `conclusive` claims were not derived from
retained numeric statuses. The historical record
`evidence/historical/foundation-a7790d225746-20260831T154248Z` is likewise deliberately
quarantined: its PGM-01 custom-validator lane failed closed because the invoking Python environment
lacked the pinned RFC 3339 validator. It is retained as failure evidence and is not a candidate pass.
The failed first hardening record
`evidence/historical/foundation-374a6a3060ad-20260831T163004Z` is also quarantined: all three PGM-01
lanes rejected an invalid external-validator schema version (`3` instead of `v3`). The schema caught
the defect, the collector exited nonzero, and the corrected record supersedes this attempt.

## Open dependencies and remote evidence

- PGM-01 PR #12 is merged at `7dac9d8c19952412b56a0347387666e2ca81e01d`. Its tree is
  byte-identical to reviewed head `d8d376d887c40255e87ef9656bc0faf79216b321`; the complete merged-main
  release check passes, and the exact merged identity and schema digest are reconciled locally.
- The codegen IR pin is accepted PR #19 merge
  `5c49ebfd1c87415f74420ad047392bd03b1bd202`, pinned exactly in planning and the evidence builder.
- Runtime PR #5 is merged at `e360dad8a3e0e54f9b8457ff7f3748be0f2acdb3`. Its tree is
  byte-identical to the round-10-reviewed head `4e0edec972c7e1431cf0d81ed8346a0ab8817af7`;
  every finding is closed, and its 25 executed local outcomes plus both merged-PGM validators pass
  in a 91/91-checksum record. Hosted CI is deferred; the human runtime source-release decision
  remains open.
- Manual-only CI PR #8 is merged into `main`. This branch also changes the required `Rust Checks`
  definition by adding MSRV and specification steps; those workflow changes have never executed on
  this branch. Hosted CI is intentionally deferred by operator direction.
- Default-branch protection was observed and retained in
  `evidence/foundation-remote/branch-protection.md`: strict `Rust Checks` and `License Check`, one
  CODEOWNER approval, conversation resolution, and no force push or deletion. Administrator
  enforcement is disabled, so an authorized administrator can merge without required checks or an
  approval. Hosted CI, independent approval, and the human source-release decision remain pending.

## Conclusion

The 39-document foundation specification, typed plan bundle, and local evidence procedure are ready
for draft review. Semantic implementation and release claims are not ready. The runtime merged-tree
pin is reconciled; the runtime release decision and IR dependency reconciliation remain required
before any semantic child leaves draft.
