---
id: SR-12606
title: "IR-210 gap analysis of CG PR 435"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR 435, 73aa48a2..3df9d299; src/publication/publish.rs and ten test bindings"
review_set: subset
---

# IR-210 gap analysis of CG PR 435

## Summary

All ten removed doc-line ID lists are retained in order on the same publication test functions. The test tagged `NFR-001-M-2` checks prior or absent destination bytes after each injected failure, and its `NFR-001-M-3` check compares the adjacent developer-owned file bytes. The diff does not change any assertion or production behavior, so it adds no evidence gap, stub, or unsupported implementation claim.

## Verdict

**PASS for the PR diff** — base and head coverage and normalized computed matrix are equal.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No new traceability gap (placeholder). | - |

## Coverage

Independent `quire coverage --scope . --strict` runs at base `73aa48a2` and head `3df9d299` each report 499/855 backed rows, 221 unbacked, and zero contradicted statuses. The JSON matrix has 812 criterion records on each side, and criterion IDs, statements, statuses, and binders match after source line and column normalization. Thirteen raw criterion records differ only in source positions. Strict exits 1 on both sides because of existing unbacked rows. Plan completion: not assessed. Optional repository-wide semantic review: not run.

## Examined test bindings

| Test function | Retained IDs |
| --- | --- |
| `publication_is_order_independent_and_replaces_only_its_destination` | `TC-001`, `TC-002`, `FR-005-AC-1`, `FR-005-AC-2`, `NFR-001-AC-1`, `NFR-001-AC-2`, `NFR-001-AC-3` |
| `every_injected_failure_preserves_old_and_developer_owned_bytes` | `TC-002`, `FR-005-AC-1`, `NFR-001-AC-2`, `NFR-001-AC-3`, `NFR-001-M-2`, `NFR-001-M-3`, `FR-005-AC-7` |
| `failed_rollback_reports_unknown_and_preserves_complete_recovery_bundles` | `TC-002`, `FR-005-AC-1`, `NFR-001-AC-2`, `NFR-001-AC-3`, `FR-005-AC-9` |
| `post_commit_cleanup_failure_reports_that_the_new_bundle_is_published` | `TC-002`, `FR-005-AC-1`, `NFR-001-AC-2`, `NFR-001-AC-3` |
| `real_backup_cleanup_failure_reports_published_with_both_new_artifacts` | `TC-002`, `FR-005-AC-8` |
| `unsafe_and_duplicate_artifact_paths_are_refused` | `TC-002`, `FR-005-AC-1`, `FR-005-AC-6`, `FR-005-AC-10`, `NFR-001-AC-3` |
| `bundle_construction_refuses_more_than_the_bounded_artifact_count` | `TC-002`, `FR-005-AC-5` |
| `bundle_construction_refuses_an_artifact_over_the_bounded_size` | `TC-002`, `FR-005-AC-5` |
| `bundle_construction_refuses_a_complete_bundle_over_the_bounded_size` | `TC-002`, `FR-005-AC-5` |
| `cleanup_removes_a_sibling_symlink_without_following_it` | `TC-002`, `FR-005-AC-1`, `NFR-001-AC-2`, `NFR-001-AC-3` |
