---
id: SR-12842
title: "IR-118 CG PR 441 code and Rust review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; PR 441 tests/it/vacuity_primitives.rs, src/evidence/vacuity.rs, FR-004-AC-5, TC-006"
review_set: subset
---

## Summary

Ticket: IR-118. Reviewed the five new vacuity integration tests at PR 441 head d17b872e and the production branches they exercise. The diff changes only `tests/it/vacuity_primitives.rs`; the original production source is unchanged.

## Verdict

**PASS** — the five tests have real parser/observer oracles, assert only the trace IDs they claim, and distinguish all ten previously missed mutations by direct code-path reasoning. This is a PR-diff review, not a claim that the planned aggregate TC-006 obligation is complete.

## Mutation and gate evidence

The ten named pilot misses map to these failing assertions if reintroduced; this review reasons from the exact source conditions and does not claim a new cargo-mutants run:

| Mutated source | Newly failing test and reason |
| --- | --- |
| `MAX_COVERAGE_BYTES` first `*` to `+` | Exact 16 MiB raw export is larger than the reduced limit, so admission fails. |
| `MAX_COVERAGE_BYTES` second `*` to `+` | The same exact raw export is larger than the other reduced limit. |
| `LlvmCoverage::observe` line-zero guard `||` to `&&` | Isolated zero line is no longer `InvalidPath`. |
| `LlvmCoverage::observe` start-column-zero guard `||` to `&&` | Isolated zero start column is no longer `InvalidPath`. |
| `LlvmCoverage::observe` end-before-start guard `||` to `&&` | Isolated empty span is no longer `InvalidPath`. |
| `parse_llvm_coverage` byte guard `>` to `>=` | Exact 16 MiB export is wrongly refused. |
| `parse_llvm_coverage` root guard `||` to `&&` | Relative root with a matching relative manifest is wrongly admitted. |
| `parse_llvm_coverage` file guard `>` to `==` | Exact 4096-file export is wrongly refused. |
| `parse_llvm_coverage` file guard `>` to `>=` | Exact 4096-file export is wrongly refused. |
| `parse_llvm_coverage` segment guard `>` to `>=` | Exact 250000-segment export is wrongly refused. |

The original source passes all five new integration tests in the review's `make ci` run. That command exited 0: its MSRV and default-toolchain test phases each passed 321 library, 608 integration, and 5 doc tests (36 real-Kani lane integration tests remained ignored by each normal run); formatting, specification validation, all-target lint, deny, unsafe audit, and rustdoc also passed. `bash scripts/kani_scope.sh` reports `not required`; the changed path does not enter its real-Kani reachability set. No CI workflow, production file, lint setting, dependency, or unsafe surface changed. The review used an isolated target and an independent QSpec fixture worktree.

## Examined scope

```yaml
scope:
  - id: FR-004-AC-5
    path: spec/evidence/functional/FR-004-vacuity-evidence.md
    role: examined
    excerpt: "Malformed, summary-only, identity-mismatched, path-ambiguous, unsupported, or missing observation inputs retain structured non-success outcomes without invented classifications."
  - id: TC-006
    path: spec/evidence/matrix/TC-006-vacuity.md
    role: examined
    excerpt: "Verify actual generated-oracle probes against native LLVM export, and ultimately keep measured coverage, runtime accounting, generated-campaign execution, QSL clause-run outcome, and obligation discharge distinct."
  - id: src/evidence/vacuity.rs
    path: src/evidence/vacuity.rs
    role: examined
    excerpt: "if bytes.len() > MAX_COVERAGE_BYTES {"
  - id: tc_006_exact_raw_export_limit_is_admitted_and_next_byte_refused
    path: tests/it/vacuity_primitives.rs
    role: examined
    excerpt: 'parse_llvm_coverage(&bytes, "/fixture").expect("valid export at the raw-byte limit");'
  - id: tc_006_invalid_probe_components_refuse_independently
    path: tests/it/vacuity_primitives.rs
    role: examined
    excerpt: 'coverage.observe(path, token).unwrap_err().code'
  - id: tc_006_relative_source_root_refuses_even_with_matching_manifest
    path: tests/it/vacuity_primitives.rs
    role: examined
    excerpt: 'parse_llvm_coverage(&bytes, "fixture").unwrap_err().code'
  - id: tc_006_exact_file_census_limit_is_admitted
    path: tests/it/vacuity_primitives.rs
    role: examined
    excerpt: 'parse_llvm_coverage(&bytes, "/fixture").expect("valid export at the file-count limit");'
  - id: tc_006_exact_segment_census_limit_is_admitted
    path: tests/it/vacuity_primitives.rs
    role: examined
    excerpt: 'parse_llvm_coverage(&bytes, "/fixture").expect("valid export at the segment-count limit");'
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
