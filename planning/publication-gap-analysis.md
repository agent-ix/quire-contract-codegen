---
id: REV-013
title: "Atomic publication implementation gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "issue #6 atomic generated-boundary publication slice"
review_set: subset
---

# Atomic publication implementation gap analysis

## Summary

The bounded library publisher implements deterministic artifact-bundle identity, complete staged
writes, verified ownership boundaries, and rollback after injected staging or swap failures. It does
not manufacture the missing serialized-package semantics needed by the planned CLI.

## Current verdict

The publication slice is ready for independent current-head review. It is not a verdict on the CLI,
cross-backend parity, Task-006 as a whole, TM-001 promotion, or source release.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-901 | high | Closed locally: every artifact path, digest, uniqueness constraint, count, and size is checked before the destination is mutated. | FR-005-AC-1, NFR-001-AC-2 |
| FND-902 | high | Closed locally: replacement requires a valid ownership marker, exact bundle digest, complete recursive entry census, regular files, and unchanged content digests. | FR-005-AC-1, NFR-001-AC-3 |
| FND-903 | high | Closed locally for injected failures: every artifact and marker staging point plus both swap boundaries preserve the old bundle and adjacent developer-owned bytes without staging residue. | TC-002, NFR-001-AC-2 |
| FND-904 | medium | Closed locally: bundle identity and published bytes are invariant under artifact input order. | FR-005-AC-2, TC-001 |
| FND-905 | medium | Retained platform boundary: portable non-empty-directory replacement is rollback-atomic for pre-commit failures, not process-crash atomic between two renames. | NFR-001, MP-001 |
| FND-906 | high | Open upstream design gate: accepted serialized contract packages bind `ReferenceBody` metadata but no executable `TypedExpression`, while IR's expression decoder is private. A truthful serialized-package CLI cannot be implemented locally. | FR-005, interface-001, issue #6 |
| FND-907 | medium | Open process gate: independent exact-head review and full-repository verification are required before landing. | issue #6, MP-001 |
| FND-908 | medium | Retained concurrency boundary: callers must serialize publishers and other destination/sibling writers during the call. | interface-001, NFR-001 |
| FND-909 | high | Closed locally: publication originally discarded every staging/backup cleanup error, so it could report success with backup residue or hide residue behind an earlier failure. Cleanup now returns structured `io_failed`, distinguishes absent paths from inspection failures, and removes a raced sibling symlink itself without following it. | FR-005-AC-1, NFR-001-AC-2, TC-002 |
| FND-910 | high | Closed locally: a backup-cleanup error happens after the new destination is committed, so the original diagnostic overclaimed rollback. Diagnostics now distinguish `unchanged`, `published`, and `unknown`; a post-commit injected control requires `published`, a complete new bundle, unchanged developer bytes, and visible cleanup residue. | FR-005-AC-1, NFR-001-AC-2, TC-002 |

## Verification performed

`cargo test --lib publication` covers deterministic order, initial publication, owned replacement,
all pre-commit failure points, the post-commit cleanup boundary, modified files, unmarked and extra
entries, empty directories, file plus destination symlinks, and cleanup of a raced sibling symlink
without following it. `cargo
clippy --all-targets -- -D warnings` passes locally. A clean-commit full-repository run remains
required.
