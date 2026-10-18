---
id: SR-12843
title: "IR-118 CG PR 441 trace and gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; PR 441 tests/it/vacuity_primitives.rs, src/evidence/vacuity.rs, FR-004-AC-5, TC-006"
review_set: subset
---

## Summary

Ticket: IR-118. Compared the PR's five new test oracles and seven `#[trace]` bindings to FR-004-AC-5, TC-006, the unchanged vacuity parser and observer, and Quire's computed base/head matrix.

## Verdict

**PASS for this PR diff** — the new bindings resolve to actual behavioral tests and introduce no untested production behavior or inflated criterion status. TC-006 and FR-004 remain planned for aggregate native-run binding and obligation discharge; these primitive tests do not claim that work is complete.

## Coverage

Plan completion: not assessed.

The exact PR base and head each have strict Quire coverage of 516/904 backed rows, 226 unbacked rows, and 0 contradicted statuses. The computed matrix status counts are also identical: 454 tagged, 337 untagged, 48 method-without-symbol, and 14 tagged only by ignored tests. FR-004-AC-5 is `tagged` on both trees; its direct binder count increases from seven to nine through the two new structured-refusal tests. `quire trace --file tests/it/vacuity_primitives.rs --json` resolves all seven new attribute bindings: five TC-006 and two FR-004-AC-5. The pre-existing global untagged and ignored-only rows are outside this test-only PR's delta.

The tests call `parse_llvm_coverage` or `LlvmCoverage::observe` on constructed input and assert an actual `Ok`/typed refusal result; no test uses a stubbed parser, a self-comparison, or an assertion skipped behind a conditional. Both MSRV and default-toolchain `make ci` test phases passed 321 library, 608 integration, and 5 doc tests; the five new tests passed in each integration phase. Their narrow exact-boundary assertions are partial TC-006 evidence and do not replace the planned producer authentication, campaign binding, or consuming obligation controls.

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
