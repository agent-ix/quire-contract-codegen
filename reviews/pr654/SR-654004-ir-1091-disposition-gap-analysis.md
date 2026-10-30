---
id: SR-654004
title: "IR-1091 CG654 QSL clause equivalence gap-analysis disposition"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-1091; PR654; exact dbfcc961e66bb26d5d5c2cae47659feeb41594b0 on base d91bf1bd75636d8ed98ac8bc9ceee401b26e94ae; spec/kani/functional/FR-015-bounded-kani-obligations.md, tests/it/kani_obligations_symbolic_domains.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-015", type: references }
---

## Summary

Ticket: IR-1091 (PARTIAL). Planless gap-analysis disposition of SR-654001 FND-003 at exact head `dbfcc961e66bb26d5d5c2cae47659feeb41594b0`, merged base `d91bf1bd75636d8ed98ac8bc9ceee401b26e94ae`. Plan completion: not assessed. `quire trace` was not run, because the reviewer had read, search and write tools only; tag bindings were read from source.

## Verdict

CLEAN for this method's own scope. FND-003 is fixed at the row it named, and no proof credit is overclaimed. The same stale claim in the sibling AC-96 row is SR-654005 FND-006, which stays open and blocks a fully clean PR.

## Dispositions

| FND | Outcome | Evidence |
| --- | --- | --- |
| FND-003 | fixed | The FR-015-AC-99 status at spec/kani/functional/FR-015-bounded-kani-obligations.md:901 now says the receiver-only precondition independent-oracle control compares the compiled generated oracle with `qsl_replay::evaluate_clause` in `collection_precondition_generated_oracle_agrees_with_native_clause_evaluation` (CG #654), and that the proof-evidence and operation-context rows remain partial. The row stays PARTIAL and no longer says the control awaits QSL-1302. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder); see SR-654005 FND-006 for the AC-96 status sentence. | - |

## Coverage

**False-credit check.** Clean. The row claims only the receiver-only precondition control, and its claim of an inactive-slot witness and full-width i64 elements matches the cases at tests/it/kani_obligations_symbolic_domains.rs:2064 and :2136-2143. It does not credit the postcondition leg, the extra-parameter preconditions, or installed-Kani proof. The test changes no proof state, identity, readiness or status.

**Bindings.** `#[trace("FR-015-AC-96", "FR-015-AC-99", "TC-025")]` is at :2055. The AC-99 binding is correct. The AC-96 tag is new since SR-654001 and is justified: the test asserts cardinality `(2, 3)`, the `OrderedSet` kind and the full `i64` element identity at :1962-1972 and :2123-2135.

**Status sites.** A repository search finds no remaining `QSL-1302` outside FR-015:898 and the historical SR-654001 row. The status paragraph at FR-015:133-147, TC-025:210 and spec/kani/matrix/tests.md:40 and :115 describe source-backed totality and satisfiability without claiming the native comparison is pending, so they are not false. FR-029's "awaiting independent native evaluation" concerns settlement of a totality falsification, which this PR does not change.

**Log evidence.** `cg-ir1091-qsl-equivalence-review-fix-spec.log` and `review-artifact-spec.log` show `quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/**/*.md'` with module warnings and four grammar warnings, none on rows 898 or 901, and no error text. Neither log records a SHA or exit status.

## Examined scope and bindings

```yaml
head: dbfcc961e66bb26d5d5c2cae47659feeb41594b0
base: d91bf1bd75636d8ed98ac8bc9ceee401b26e94ae
clean: true
scope:
  - {id: FR-015-AC-99, path: spec/kani/functional/FR-015-bounded-kani-obligations.md, role: examined, excerpt: "the receiver-only precondition independent-oracle control compares the compiled generated oracle with `qsl_replay::evaluate_clause`"}
  - {id: collection_precondition_generated_oracle_agrees_with_native_clause_evaluation, path: tests/it/kani_obligations_symbolic_domains.rs, role: examined, excerpt: "#[trace(\"FR-015-AC-96\", \"FR-015-AC-99\", \"TC-025\")]"}
bindings:
  - {test_id: collection_precondition_generated_oracle_agrees_with_native_clause_evaluation, ac_id: FR-015-AC-96, trace: correct}
  - {test_id: collection_precondition_generated_oracle_agrees_with_native_clause_evaluation, ac_id: FR-015-AC-99, trace: correct}
```
