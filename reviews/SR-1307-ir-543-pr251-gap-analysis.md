---
id: "SR-1307"
title: "CG PR 251 gap analysis: NFR-005-AC-1 to AC-5 and TC-042 against the tests and the code"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@0b11c77e0408242377eb86e5aac29a9bbac9f07f; spec/core/non-functional/NFR-005-no-generation-panics.md (Statement, Scope, Behavior table, NFR-005-AC-1 to AC-5), spec/core/matrix/TC-042-no-generation-panics.md, spec/core/matrix/tests.md, spec/tests.md, spec/core/functional/interface-001-codegen-api.md:92,139-140, spec/routed/functional/FR-022-routed-generation.md:204-207, spec/oracle/functional/FR-021-function-application-oracles.md:211-216, spec/kani/matrix/TC-025-bounded-kani-obligations.md:159-167, against the src/ and tests/ diff (origin/main...HEAD, base dc4166a)"
---

# SR-1307: CG PR 251 gap analysis

## Summary

Ticket: IR-543. PR: agent-ix/quire-contract-codegen#251 at 0b11c77. This is a planless gap
analysis (plan completion: not assessed). It maps each NFR-005 AC and each TC-042 step to a
tagged test and to the code, and re-measures the strict coverage figure.

| AC | What the spec asks | Covering test | Status |
| --- | --- | --- | --- |
| NFR-005-AC-1 | Token list identical to FR-014-AC-39's. Comments, literals and `#[cfg(test)]` items removed. `src/kani/test_support.rs` exempt because `src/kani/mod.rs` declares it under `#[cfg(test)]`. All required files read. At least 60 files scanned (69 here). Exactly one `expect` allowed, located inside `fn digest` of `impl CaseIdentity<'_>`, and the rest of that file clean | `no_generation_panics.rs` tests `tc_042_ac1_src_*` and `tc_042_ac1_the_literal_free_scan_*` | Covered. Mutation-checked: an `unwrap` in a clean file, a second `expect` in `digest`, an `expect` in `render_artifacts`, and `digest`'s `expect` changed to `unwrap` all fail the scan |
| NFR-005-AC-2 | Empty `checked_bounds`, and a first bound with no `IntegerRange`, both return `NoRenderer`; `classify_claim` reports `OperationNotRendered` for each | Unit test `scalar.rs::tc_042_ac2_*`, with three shapes. Integration test `tc_042_ac2_*` through `negotiate_kani_obligations`, which first asserts the unmodified claim is `Supported` as a baseline | Covered. A panic in either branch fails both tests |
| NFR-005-AC-3 | `warning()` returns `None` for the six `invalid_capability` causes and `Some` for the two `unsupported_projection` causes (all eight) | `tc_042_ac3_*` | Covered. Each cause's `code()` is also asserted |
| NFR-005-AC-4 | `MapMismatch` "missing semantic probe" with no classification; `evaluation_count` is `None` for a missing evaluation probe; `UnavailableObservation` with no export | `bound_coverage.rs::tc_042_ac4_*`, with a complete-map baseline | Covered |
| NFR-005-AC-5 | Fewer records, more records, and an equal count | `routed/generate.rs::tc_042_ac5_*` | Covered |

Spec and matrix edits:

- The core matrix rows for NFR-005 and TC-042 move to Covered, and `spec/tests.md` drops TC-042
  from the planned list.
- `quire coverage --scope . --strict` gives 66 unbacked rows at head and 72 at base dc4166a,
  re-measured.
- The seven `uncatalogued-verification-method` warnings are identical at base and head. The two
  on NFR-005's Measurement table came from the spec PR (#250), not this one.
- `make spec` passes.

## Verdict

Every AC has a test that fails on a wrong implementation. One low trace finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `tc_042_ac1_operands_of_each_operation_have_its_own_arity_and_reach_exact_extremes` is tagged `Trace: NFR-005-AC-1, TC-042`. NFR-005-AC-1 is the lexical scan, and NFR-005's Verification section says the `ScalarOperation::reachable` arm "is verified by NFR-005-AC-1 alone". This test is a parity regression test of the `ScalarOperands` refactor and does not exercise the scan, so the tag credits AC-1 with a test that does not test it. Retag it as a parity test of the site row (for example `Trace: TC-042` plus a note), or name the Behavior row it guards | src/kani/generate/scalar.rs:783-787 |

## Dispositions

Round 1, reviewed at 8216fe4cdda057953e1191dbdd15118a5dad7c9d.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed 8216fe4 | 8216fe4cdda057953e1191dbdd15118a5dad7c9d |
