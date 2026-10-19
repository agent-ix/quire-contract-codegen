---
id: SR-13246
title: "IR-878 CG PR 456 spec-review integrity: replay matrix rows and matrix index"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@943f4c268ba9288ee8a71f3677e6afcb55b2c6f8 vs origin/main fb380d29; spec/replay/matrix/tests.md, spec/tests.md"
review_set: subset
---

## Summary

Ticket: IR-878. This review checked the two spec files the PR changes:

- The FR-024 matrix row split: AC-1, AC-5 and AC-7 to AC-9 stay Planned, and AC-2 to AC-4 and AC-6 move to Covered.
- The Replay line of the matrix index.

The rows partition FR-024-AC-1 to FR-024-AC-9 with no gap or overlap. The index line agrees with the matrix rows. Every row the PR marks Covered has a tagged test that `quire coverage` credits. `make spec` exits 0. Its only warnings are pre-existing ones in files this PR does not touch. The stale FR-024 "Current state" prose is recorded in the gap analysis (SR-13245 FND-001), not repeated here.

## Examined criteria

- FR-024 matrix rows, spec/replay/matrix/tests.md:16-17 (examined): "FR-024-AC-1, FR-024-AC-5, FR-024-AC-7 through FR-024-AC-9 / 🚧 Planned" and "FR-024-AC-2 through FR-024-AC-4, FR-024-AC-6 / ✅ Covered".
- Replay index line, spec/tests.md:20 (examined): "FR-024-AC-2 to AC-4, AC-6 and AC-10 are covered by focused intake and source-layout tests".

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

**PASS.** The matrix rows and the index are consistent with each other and with the measured coverage.
