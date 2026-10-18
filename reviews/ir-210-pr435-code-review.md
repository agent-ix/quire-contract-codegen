---
id: SR-12605
title: "IR-210 code and Rust review of CG PR 435"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR 435, 73aa48a2..3df9d299; src/publication/publish.rs and ten test bindings"
review_set: subset
---

# IR-210 code and Rust review of CG PR 435

## Summary

The diff converts ten `/// Trace:` lines on publication unit tests to `ix-trace-rs` attributes. A function-by-function script compared every ordered ID list, including the `NFR-001-M-2` and `NFR-001-M-3` measurements, and found an identical `#[trace]` list on the same test function. Removing trace metadata and the new test-module import leaves the same nonblank source lines at base and head.

## Verdict

**PASS** — no code or Rust review finding in the PR diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder). | - |

## Evidence

- All ten changes are on `#[test]` functions, including the two Unix-only tests. No helper, production, or Kani proof function changed. Test bodies and assertions are unchanged.
- The prose suffix on `real_backup_cleanup_failure_reports_published_with_both_new_artifacts` remains a doc comment. No orphan blank `///` line or new `Trace:`/`Tracing:` line appears.
- `git diff --check` and `make fmt-check` pass. `make kani-scope` reports required for the changed file; the author reports a 34/34 real Kani gate. The reviewer has not repeated that gate.

## Gates

Independent `cargo test --locked -j2 --lib publication:: -- --test-threads=2` passed all ten tests. The author reports a full `make -k ci` pass at this head; the reviewer did not repeat the full suite.

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
