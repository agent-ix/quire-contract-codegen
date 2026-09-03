---
id: REV-012
title: "Atomic bundle publication preimplementation review"
type: SpecReview
analysis: gap-analysis
scope: "issue #6 library-first atomic publication boundary"
review_set: subset
---

# Atomic bundle publication preimplementation review

## Summary

FR-005 and NFR-001 require complete staged validation and no developer-owned edits. Portable Rust
cannot atomically replace a non-empty directory in one syscall, so the bounded publisher uses a
staged sibling plus a rollback-protected directory swap and makes the ownership boundary explicit.

## Verdict

**PASS to implement the library publisher.** The publisher may create a previously absent destination
or replace a directory whose marker, complete file census, and content digests all prove it is wholly
generator-owned. Any unmarked, malformed, symlinked, extra-file, or modified boundary is refused.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-901 | high | Validate every artifact path, digest, uniqueness constraint, count, and size before filesystem mutation. | FR-005-AC-1, NFR-001-AC-2 |
| FND-902 | high | Existing output may be replaced only when a marker plus full recursive census proves every entry was generated and remains digest-identical. | FR-005-AC-1, NFR-001-AC-3 |
| FND-903 | high | A failed staged write or failed swap must restore the old directory and remove staging residue. | TC-002, NFR-001-AC-2 |
| FND-904 | medium | Bundle identity must be independent of input order and staging names. | FR-005-AC-2, TC-001 |
| FND-905 | medium | The portable replacement is rollback-atomic for reported I/O failures, but not process-crash atomic across the two directory renames; retain that boundary explicitly. | NFR-001, MP-001 |
| FND-908 | medium | Publication requires caller-serialized writers; a concurrent process mutating the destination or generated sibling names is outside this bounded API contract. | interface-001, NFR-001 |

## Decision

Proceed with a validated `ArtifactBundle`, deterministic ownership marker, sibling staging directory,
rollback on every injected failure, and tests that hash an adjacent developer-owned file before and
after each path. The serialized-package CLI remains a separate design gate because accepted IR
packages do not bind clauses to `TypedExpression`.
