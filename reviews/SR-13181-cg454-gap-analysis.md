---
id: "SR-13181"
title: "CG PR 454 gap analysis: FR-039-AC-5 owner evidence and coverage delta"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@af121888fa410c17f7a7a9f635f73de24ae949db; spec/core/functional/FR-039-contract-package-consumption.md:188-220, spec/core/matrix/tests.md:25,63; owner evidence agent-ix/quire-contract-ir@4213b980b9298566070a76aa3eef76b832b93186 FR-035 and tests/it/complete_v1_contract_package.rs (diff origin/main...HEAD)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-039
    type: reviews
---

# SR-13181: CG PR 454 gap analysis

## Summary

Ticket: IR-888. Plan completion: not assessed.

This review checks each refusal class the PR cites against the sources on Contract IR origin/main
at 4213b98:

| Class | IR criterion text that states it | Tagged TC-047 test |
| --- | --- | --- |
| Wrong version | FR-035-AC-10 | `reader_refuses_wrong_string_version_before_shape_identity_and_nodes` |
| Incomplete | FR-035-AC-11 | `reader_incomplete_reports_each_exhausted_meter_and_pointer` |
| Invalid/incomplete source map | FR-035-AC-5 | `reader_requires_total_source_correspondence_and_request_order` |
| Stale own identity | FR-035-AC-2 ("identity ... mutation") | `reader_refuses_malformed_outer_shape_and_stale_own_identity` (tagged AC-5) |
| Stale selected dependency | FR-035-AC-2, FR-035-AC-7 | `selected_dependencies_are_bound_by_library_identity_before_graph_checks` (tagged AC-5) |
| Malformed wire | none | `reader_refuses_malformed_outer_shape_and_stale_own_identity`, `reader_locates_malformed_nested_source_value` (tagged AC-5) |
| Noncanonical wire | none | `duplicate_member_and_noncanonical_bytes_refuse_before_identity` (tagged AC-5) |
| Invalid accounting | none | `reader_requires_total_source_correspondence_and_request_order` (`/feature_accounting`, tagged AC-5) |

Every class has a tagged TC-047 test. Three classes have no IR criterion whose text states them.

Strict coverage delta, `quire coverage --scope . --strict` (quire 0.36.2), base cfb3890 to head:

- backed stays 549/906, and `status_lies` stays 0;
- the obligation FR-039-AC-5 changes method from Test to Analysis, and nothing else flips;
- `no_symbol_rows` rises 48 to 50: the AC-5 verification row, and the tests.md row as an FR-level
  Analysis row;
- `unbacked_rows` rises 229 to 230: the same tests.md row.

Adding an FR-level Analysis row to tests.md matches the existing convention in
`spec/routed/matrix/tests.md`. `make spec` passes.

## Verdict

CHANGES REQUESTED, with two medium findings and one low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | tests.md says "Contract IR FR-035-AC-2, AC-5, AC-10 and AC-11 have tagged TC-047 reader evidence", and the FR section says the tests are in `tests/it/complete_v1_contract_package.rs`. On IR main 4213b98, FR-035-AC-2's only tagged tests are TC-044, in `tests/it/complete_v1_checked_package.rs:195,323`. IR's own tests.md marks TC-044 "PLANNED / UNRUN for FR-330; existing reader/lowerer tests target the earlier IR package". The AC-2 evidence claim is false as written | spec/core/matrix/tests.md:25; spec/core/functional/FR-039-contract-package-consumption.md:213-219 |
| FND-002 | medium | FR-039-AC-5 cites FR-035-AC-5 as owner evidence for malformed and noncanonical wires and for invalid accounting. FR-035-AC-5's criterion text states only package emission, digest scope, and invalid/incomplete `source_map` refusal. The FR-035 statement does not state those three classes either. Tests tagged AC-5 exercise them, but no IR criterion owns them for the FR-330 reader. The citation should say this, and an IR ticket should add the criterion | spec/core/functional/FR-039-contract-package-consumption.md:210-213 |
| FND-003 | low | The analysis lists four public consumers. FR-039 governs the Kani paths, but the analysis omits the crate-internal `&ContractPackageV1` consumers: `src/oracle/scalar/mod.rs` `generate_contract_scalar_oracles` and `preflight_contract_scalar_claim`, and `src/kani/generate/negotiate.rs` `classify_scalar_form` and `classify_node`. The by-type claim still holds for them, but a checker following the list would not examine them | spec/core/functional/FR-039-contract-package-consumption.md:195-200 |
