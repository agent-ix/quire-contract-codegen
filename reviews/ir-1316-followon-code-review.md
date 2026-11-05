---
id: SR-2026110283862
title: code-review review of IR-1316 follow-on
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; PR745 changed tests/ and spec/; selected IR-1316
  criteria
review_set: subset
---

## Summary

This is a PR/ticket subset review, not whole-repository assurance. Ticket: IR-1316; PR: quire-contract-codegen#745. Historical PR #743 findings and artifacts are separate and were not reused. No applicable AssuranceProfile is declared under spec/. The protected files are unchanged. No code, specification, lockfile, or branch mutation was performed by the reviewer.

## Verdict

**PASS** — No new defect in the scoped evidence changes. This is not a repository-wide assurance verdict.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

The four comparisons use independent Rust predicates for <, <=, > and >= with literal 1 in domain 0..=3. Each mirror is admitted from real QSL-authored source; both satisfying and violating membership sets are compared with the independent enumeration. All three populations for all eight authored forms are emitted into a scratch crate, then cargo test executes 24 named generated tests. The shared check_small walker rejects every unexpected valuation/tag and requires the reachable set to equal the expected set after seeded simplify/complicate walks. It does not call the generated oracle as its truth authority. A wrong operator, position, missing side member or tag can fail these assertions.

FR-011-AC-12 compares the complete Artifact vectors from two calls with identical admitted package/key/policy. Artifact derives structural Eq over path and contents; two artifacts are required, identified as Rust source and JSON map, parsed and checked. This establishes byte equality of both source and map; generated population/census consumer execution is additional evidence. The selected occurrence is authentic in the QSL-emitted/IR-admitted graph; no driver-export rejection coverage is inferred from that fact. The separate export/owner/identity mutation obligations remain Planned.

The only helper changes expose an existing common generated walker and factor source-condition replacement and admission setup. Searches of the existing tests found and reused check_small, with_field_ranges and the existing scratch-crate helpers; no second implementation or upstream codec/schema was vendored. No new unsafe, concurrency, external input or production API path is introduced. Test fixture assertion panics have explicit invariants. CI workflows and gates are unchanged.

The Rust lane read rust-style and its implementation, API, safety and testing references plus repository CLAUDE.md. Reviewed the frozen-head focused-test log (10 passed), formatter and locked Clippy logs; no aggregate, Kani, Cargo or cargo-deny lane was rerun, per dispatch. The root owns final scoped checks on the final head; this review does not certify an independently rerun build.

## Examined scope

```yaml
scope:
- id: FR-009-AC-7
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: For each noncommutative operator, a comparison written with the literal
    in the first position produces the same satisfying and violating sets as its mirrored
    read-first form, enumerated independently.
- id: FR-011-AC-12
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: Repeated numeric runner generation with the same admitted package, selected
    driver-bound FR-331 claim, checked clause, and campaign policy yields byte-identical
    source and map.
- id: tests/it/v2_numeric_population.rs
  path: tests/it/v2_numeric_population.rs
  role: examined
  excerpt: fn selected_v2_literal_mirrors_generate_exact_population_sets() {
- id: tests/it/bound_populations.rs
  path: tests/it/bound_populations.rs
  role: examined
  excerpt: 'pub(super) const GENERATED_SUPPORT: &str = r#"use std::{cell::Cell, collections::BTreeSet,
    fmt::Debug};'
- id: tests/it/v2_integer_comparison_admission.rs
  path: tests/it/v2_integer_comparison_admission.rs
  role: examined
  excerpt: pub(super) fn admit_twin(
- id: tests/state_frame_support/native_twin.rs
  path: tests/state_frame_support/native_twin.rs
  role: examined
  excerpt: '    pub fn with_balance_condition_in_range(condition: &str, range: (i64,
    i64)) -> Self {'
```

## Method selection

Code-review includes Rust; separate gap-analysis, spec base, integrity and evidence artifacts were produced. EARS is skipped because no behavioral requirement statement changed; dependency because no relationship edge changed; object, scope-boundary, risk-complexity and failure-domain as standalone methods because no object, allocation, architecture or failure behavior was introduced. Integrity included the failure-domain probes. Criterion-strength/Jev is not needed for unchanged AC semantics; no calibrated strength claim is made. No sub-reviewer was dispatched.
