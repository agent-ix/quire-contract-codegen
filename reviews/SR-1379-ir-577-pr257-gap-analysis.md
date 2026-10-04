---
id: "SR-1379"
title: "CG PR 257 gap analysis: NFR-005-AC-6 to AC-8, TC-042 steps 6 to 8, FR-022 and interface-001 against the code and tests"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@4b5490b1292b963fd89b2bc804b660e4878c9e60; spec/core/non-functional/NFR-005-no-generation-panics.md (Statement, behavior table IR-577 rows, AC-6 to AC-8, Verification), spec/core/matrix/TC-042-no-generation-panics.md (steps 6 to 8, Expected Results), spec/routed/functional/FR-022-routed-generation.md (DuplicateItem refusal), spec/core/functional/interface-001-codegen-api.md (generate_routed, invalid_generated_syntax), spec/core/matrix/tests.md, spec/tests.md, src/routed/generate.rs, src/evidence/bound_coverage.rs, src/oracle/boolean_v1.rs, src/core/diagnostic.rs, tests/it/no_generation_panics.rs (diff origin/main...HEAD)"
---

# SR-1379: CG PR 257 gap analysis

## Summary

Ticket: IR-577. PR: agent-ix/quire-contract-codegen#257 at 4b5490b. Plan completion: not
assessed.

Each planned-to-implemented flip, checked against a test or the code:

- NFR-005 Statement bullets 2 and 3, and the four IR-577 behavior-table rows: backed by the code
  at each site (`group.get`, slice pattern, `checked_sub` then `get`, struct update) and by the
  AC-8 body scan, which fails when any of the four sites is restored (mutation, SR-1378).
- NFR-005-AC-6 / TC-042 step 6: `tc_042_ac6_a_duplicate_position_outside_the_group_is_a_typed_refusal`
  hands the group length (3), one beyond (4) and `usize::MAX`, asserting the variant with both
  fields, and the last valid position (2) rewritten to request index 12.
- NFR-005-AC-7 / TC-042 step 7: `tc_042_ac7_a_map_shorter_than_two_regions_is_a_census_mismatch_not_a_panic`
  covers 0 and 1 regions, each with and without an export, asserting the exact diagnostic, no
  classification, `evaluation_count` `None` and no consequents; and two probed regions with an
  export, asserting no consequents and no `MapMismatch`.
- NFR-005-AC-8 / TC-042 step 8: `tc_042_ac8_the_four_ir_577_bodies_hold_no_index_or_subtraction`
  locates the four bodies in literal-free non-test code, panics if one is missing, and scans
  them; `tc_042_ac8_the_body_check_flags_index_and_subtraction_and_passes_their_lookalikes`
  asserts every positive and negative case step 8 names.
- FR-022 DuplicateItem refusal and interface-001 `KaniDuplicatePositionOutOfRange{first_index
  usize, items usize}`: the variant exists with those fields; the refusal is propagated by `?` in
  `generate_kani` (see SR-1378 FND-002 for its test strength).
- interface-001 `invalid_generated_syntax` and the `InvalidGeneratedSyntax` doc: the doc comment
  reads "Generated source did not parse, or did not match its own source map", the refusal goes
  through `single_diagnostic`, and the stable code is unchanged. NFR-005 Verification states this
  refusal has no fixture; that is the merged spec's choice.
- Matrix rows: core `tests.md` merges the two NFR-005 rows into one covered row and marks TC-042
  covered; `spec/tests.md` drops the AC-6 to AC-8 planned note. No `PLANNED (IR-577)` marker is
  left in `spec/`. `quire coverage --strict` reports 66 unbacked rows and 0 contradicted statuses
  on head, NFR-005 8/8; the three `uncatalogued-verification-method` notes on NFR-005's metric
  table predate this PR. `make spec` exits 0.

No production code in the diff lacks an owning requirement: every changed `src/` hunk is one of
the four NFR-005 sites, the new variant (FR-022, interface-001) or the widened doc
(interface-001).

## Verdict

Mergeable. Every flip is backed by a test or by code that a test fails without. No gap beyond the
low test-strength items recorded in SR-1378.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
