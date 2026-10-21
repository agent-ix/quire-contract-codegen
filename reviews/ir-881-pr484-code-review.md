---
id: SR-14310
title: "Code and Rust review of IR-881 CG consumer rename"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; Cargo.lock, src/kani/terminal.rs, tests/it/terminal_map.rs"
review_set: subset
---

## Summary

Ticket: IR-881. Reviewed PR #484 at ab01e5eeeaa79ee310accd8bb986b393f41bf835. The changed Rust mapping, tagged test, and selective lockfile update agree with FR-030-AC-8 and the merged Contract IR constant.

## Verdict

**PASS** — No diff-scoped code or Rust defect found. The map preserves the distinct checker and backend absence outcomes, and the test asserts both plus an unrelated code.

## Scope examined

- FR-030-AC-8 (examined): `Unavailable` with cause `kani_checker_absent` maps to `Unsupported(SolverAbsent)`; with `kani_backend_absent` or any other cause it maps to `Unsupported(BackendAbsent)`.
- TC-041 step 6 and expected result 6 (examined): map checker absence, backend absence, and another cause to their stated results.
- Cargo.lock (examined): `quire-contract-ir` and `quire-contract-model` move together to merged revision 6ca8809093fe071bf23e1b1fec90ce37f5387e44; no other lock entry changes.

## Evidence

- `src/kani/terminal.rs:194-200` matches `Std001Code::KANI_CHECKER_ABSENT` before the fallback `BackendAbsent` arm. The surrounding match enumerates the closed outcome kinds.
- `tests/it/terminal_map.rs:1207-1226` carries `#[trace("FR-030-AC-8", "TC-041")]` and exact output assertions for all three inputs.
- The merged IR source at 6ca8809093fe071bf23e1b1fec90ce37f5387e44 defines `KANI_CHECKER_ABSENT`. No live non-review file in the CG worktree contains the old code spelling.
- `git diff --check origin/main...HEAD` passed. The author-reported full gate remains the gate evidence; this review did not rerun full builds while the shared build lock was held.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
