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
writes, marker-consistent destination boundaries, and rollback after injected staging or swap failures. It does
not manufacture the missing serialized-package semantics needed by the planned CLI.

## Current verdict

The publication slice is ready for independent current-head review. It is not a verdict on the CLI,
cross-backend parity, Task-006 as a whole, TM-001 promotion, or source release.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-901 | high | Closed locally: every artifact path, digest, uniqueness constraint, count, and size is checked before the destination is mutated. | FR-005-AC-1, NFR-001-AC-2 |
| FND-902 | high | Closed locally: replacement requires a valid ownership marker, exact bundle digest, complete recursive entry census, regular files, and unchanged content digests. | FR-005-AC-1, NFR-001-AC-3 |
| FND-903 | high | Closed locally for failures with successful rollback: every artifact and marker staging point plus both swap boundaries preserve the old bundle and adjacent developer-owned bytes without staging residue. Failed rollback instead preserves recovery bundles and reports `unknown`. | TC-002, NFR-001-AC-2 |
| FND-904 | medium | Closed locally: bundle identity and published bytes are invariant under artifact input order. | FR-005-AC-2, TC-001 |
| FND-905 | medium | Retained platform boundary: portable non-empty-directory replacement is rollback-atomic for pre-commit failures, not process-crash atomic between two renames. | NFR-001, MP-001 |
| FND-906 | high | Open upstream design gate: accepted serialized contract packages bind `ReferenceBody` metadata but no executable `TypedExpression`, while IR's expression decoder is private. A truthful serialized-package CLI cannot be implemented locally. | FR-005, interface-001, issue #6 |
| FND-907 | medium | Open process gate: independent exact-head review and full-repository verification are required before landing. | issue #6, MP-001 |
| FND-908 | medium | Retained concurrency boundary: callers must serialize publishers and other destination/sibling writers during the call. | interface-001, NFR-001 |
| FND-909 | high | Closed locally: publication originally discarded every staging/backup cleanup error, so it could report success with backup residue or hide residue behind an earlier failure. Cleanup now returns structured `io_failed`, distinguishes absent paths from inspection failures, and removes a raced sibling symlink itself without following it. | FR-005-AC-1, NFR-001-AC-2, TC-002 |
| FND-910 | high | Closed locally: a backup-cleanup error happens after the new destination is committed, so the original diagnostic overclaimed rollback. Diagnostics distinguish `unchanged`, `published`, and `unknown`, and the revised controls now assert all three states against observable filesystem contents. | FR-005-AC-1, NFR-001-AC-2, TC-002 |

## PR #26 review dispositions

These are local remediation/disposition statements against the review of `bfa7086`, not an
independent acceptance verdict for this revised candidate.

| Finding | Disposition |
|---|---|
| P26-01 | Repaired: a replacement-and-rollback failure exercises `Unknown`; the destination is absent and both complete bundles remain under stage/backup siblings. Every injected initial-publication or successful-rollback failure now asserts `Unchanged`. The post-commit control still asserts `Published`. |
| P26-02 | Repaired: destination metadata, marker reads, artifact metadata, and artifact reads preserve I/O failures as `IoFailed`/`Unchanged` with the original error. Each point has an injected permission-denial control. Observed missing marker/artifact inputs remain `DestinationNotOwned`; malformed marker JSON remains an observed refusal rather than an I/O error. |
| P26-03 | Repaired: canonical path validation rejects every `.` or `..` segment before mutation, including interior/trailing dots and the `a/b` plus `a/./b` alias pair. |
| P26-04 | Clarified boundary: the writable marker proves complete content consistency, not authenticated authorship or destination-specific authority. Marker ordering is still not independently enforced; destination naming/provenance is not added to this slice. |
| P26-05 | Retained resource gap: input bundles are bounded, but disk-side marker/artifact reads occur before or without independent bounds, and directory census depth/count is unbounded. Do not claim bounded verification of arbitrary on-disk trees. |
| P26-06 | Retained fixture-depth gap: existing refusal cases pin public outcomes but not every internal refusal branch. Extra-entry/empty-directory controls still share the census guard, and the traversal's non-file/symlink branches need independent staging controls. |
| P26-07 | Open integration gate: this publisher has no generator-output adapter or CLI producer yet; generator output paths have not been qualified through it. This is separate from the IR executable-package blocker. |
| P26-08 | Retained diagnostic-quality gap: cleanup failure can replace the original staging/swap error; destination state remains conservative, but the causal chain is incomplete. |
| P26-09 | Documented platform boundary: files are synchronized but directories are not; a successful return does not guarantee persistence after power loss. |
| P26-10 | Partially repaired: initial-publication fault paths are now exercised with `Unchanged` and absent destination; its `DuringSwap` injection models refusal before the sole publication rename. `Published` diagnostics still require the caller to read the marker for bundle identity. |
| P26-11 | Review/registry integration remains open: focused SUITE-010 is documented as a local command without a structured result producer; cross-branch review-document homes and identifiers remain coordinator-owned. |

## Verification performed

`cargo test --lib publication` covers deterministic order, initial publication, owned replacement,
all modeled pre-commit failure points, failed rollback with preserved recovery bundles, four ownership
I/O points, the post-commit cleanup boundary, modified files, unmarked and extra
entries, empty directories, file plus destination symlinks, and cleanup of a raced sibling symlink
without following it. The revised seven publication tests passed with locked, offline dependencies
on stable and exact Rust 1.75.0. All-target Clippy with denied warnings, rustfmt, and Quire document
validation passed; Quire emitted existing duplicate module/inverse-edge warnings. These are focused
local checks. A clean-commit full-repository run and independent review remain required; no native
publication-result producer, shared receipt campaign, or complete CLI acceptance is claimed here.
