---
id: SR-12380
title: "IR-865 code and Rust review of CG PR 428"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR 428, 836b298f..eee965cc; all 18 changed Rust test files"
review_set: subset
---

# IR-865 code and Rust review of CG PR 428

## Summary

The PR converts 353 `/// Trace:` lines on 351 test functions in 18 files to `#[trace(...)]` attributes, and adds `ix-trace-rs` to the generated agreement crate's dev dependencies. A per-function comparison found identical ID lists and test placement in every converted function. The only code tokens changed outside trace attributes and imports are the intended scratch-crate dependency declaration. Two documentation quality findings remain.

## Reviewed scope

The diff covers `tests/composite_equality_support/agreement_cases.rs`, `tests/exact_scalar_support/agreement_cases.rs`, and these `tests/it/` files: `bound_census.rs`, `bound_coverage.rs`, `bound_populations.rs`, `bound_strategy_generation.rs`, `capability_settlement.rs`, `composite_equality_generation.rs`, `composite_parity_converter.rs`, `exact_function_generation.rs`, `exact_scalar_agreement.rs`, `exact_scalar_generation.rs`, `harness_generation.rs`, `oracle_arithmetic.rs`, `oracle_generation.rs`, `routed_generation.rs`, `skeleton_spine.rs`, and `strategy_generation.rs`. The CI workflow diff is empty. No production Rust, Kani proof, or copied file was added.

## Verdict

**CONDITIONAL** — the trace migration and scratch dependency are faithful, with two low severity doc-comment findings to fix in this PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Style note: 119 converted doc blocks now end with an empty `///` line directly before `#[trace]`. The separator belonged before the removed `Trace:` line; leaving it creates a trailing blank line in the rendered test docs. Remove the trailing empty doc lines in the eight affected files. | `tests/it/oracle_arithmetic.rs:765`; `tests/it/skeleton_spine.rs`; `tests/exact_scalar_support/agreement_cases.rs` |
| FND-002 | low | Some criterion qualifiers were detached from the IDs they explained. For example, the old `FR-021-AC-1, TC-031 (duplicate handling, matching FR-014/FR-018)` line became a standalone doc sentence beginning `(duplicate handling, matching FR-014/FR-018)`, above an attribute with both IDs; a reader can no longer tell which criterion the qualifier narrows. Prefix such notes with their owning criterion ID and make the sentence stand alone. | `tests/it/exact_function_generation.rs:2437`; `tests/it/composite_equality_generation.rs:1446`; `tests/composite_equality_support/agreement_cases.rs:390` |

## Evidence

- A diff-scoped script compared removed doc IDs with added attribute IDs by test function, including both doc lines on `tc_017_bound_admission_uses_the_public_clause_and_domain`: 351/351 migrations match, and every new attribute precedes a test function.
- No added `/// Trace:` or `/// Tracing:` line appears in the diff. The three files reserved for PR #416 are unchanged.
- After removing doc comments, trace attributes, and the added imports, tokenized test code is identical in 17 files; the remaining file differs only in `AGREEMENT_DEV_DEPENDENCIES`, whose `ix-trace-rs` spelling matches `Cargo.toml` and the package in `Cargo.lock`.
- `git diff --check` passes. The author reports local cargo gates in the PR body; this reviewer did not rerun them while higher-priority shared cargo slots were occupied. Coverage and matrix were measured independently in SR-12381.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Commit `54256e7` removes exactly 119 empty `///` lines immediately before converted trace attributes. No final migrated test doc block ends with an empty `///`. |
| FND-002 | fixed | The same commit rewrites all 26 orphaned qualifier openings as criterion-labelled prose. Each new leading criterion ID appears in that test's unchanged `#[trace]` ID list; for example, the duplicate-handling note now begins `FR-021-AC-1`. |

The final base-to-head trace audit still finds 351/351 function ID lists identical, with attributes on tests only. The fix diff contains only the 119 blank-line deletions and 26 prose-label replacements; it adds no new trace doc line or executable code change. `git diff --check` passes.
