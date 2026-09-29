---
id: SR-042
title: "Code review — quire-contract-codegen PR #189 remove ceremony"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@759c5d20ab3a4472b3916a59e5245cef898d4c56; src/, tests/, examples/, schemas/, build.rs, Cargo.toml, Makefile, README.md, CLAUDE.md, .github/workflows/ci.yml (read only)"
review_set: subset
---

## Summary

Code review with the Rust lane of PR #189 (branch `cg-remove-ceremony`, 13 commits, 102 files, +3242/-24339) against `origin/main` 6e0c518. No ticket id is derivable from the branch or title. The removal itself is mostly clean: the goldens are replaced by generate, compile and execute agreement suites that really run, determinism is checked by regeneration, and the qsl-replay bump to 1a368fa is QSL main. One change inside the PR breaks real Kani proving for routed scalar harnesses, and the replacement "authored" base package is the deleted vendored fixture re-spelled.

## Verdict

**FAIL**. FND-001 is a functional regression introduced by commit 61b8358 in this PR. The author said the routed scalar failures also happen on base 967c1ab and so were not caused by this change. That is not true: 967c1ab is itself inside the PR. On `origin/main` 6e0c518, and on the PR's first commit 5379cec, both `tc_027_a_routed_scalar_harness_*` tests pass. They fail from 61b8358 on. FND-002 is a P0 duplication finding.

Clean, and recorded as examined:
- The agreement suites (`tests/it/exact_scalar_agreement.rs:33-96` and the composite and function equivalents) write each generated crate, run `cargo test --offline` on it, and assert that the passed count equals the number of cases and is greater than 0. None is ignored.
- The 1596-line chain golden is still covered by `tc_031_ac7_nested_call_chain`.
- Determinism is checked by regenerating from fresh and permuted inputs.
- The FR-016-AC-8 witness join still keys on `{module_symbol}::{harness_symbol}`, and `kani-obligations/{module}.json` is still emitted as `{identity, rustPath}`. Nothing read the removed digests.
- The spine test now uses `qsl_replay::call_site`, with no `qsl-foundation` or `quire-exact` dev-dependencies.
- Gates at head: `cargo fmt --check` 0, `make lint` 0, `make test` 0 (lib 82, it 221 passed with 5 ignored, doc 7).
- The Kani lane at head: `skeleton_spine` passes. Four ignored Kani tests fail. Two of them (`tc_025_real_kani` VacuousProof and `kani_witness_join` tc_026 NoVerdict) also fail on main, so they predate the PR. The other two are FND-001.

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | high | Routed scalar Kani harnesses no longer verify or falsify. The scalar module and harness symbols went from 32 to 64 hex characters when commit 61b8358 dropped the re-hash. Both `tc_027_a_routed_scalar_harness_verifies` and `..._violating_its_bound_is_falsified` now return `Inconclusive{NoVerdict}`. They pass on main 6e0c518 and on 5379cec and fail from 61b8358. The clause path still truncates to 32 characters because "Kani derives object-file names from these symbols; keep them bounded". The PR's claim that these tests were already failing is wrong. | src/kani_obligations.rs:1336-1340; src/kani_obligations.rs:941 |
| FND-002 | high | Duplication: `base_package()` is the deleted vendored fixture `positive-nominal-identities.json` (QSpec I04 vector) transliterated into `json!`. It keeps the same four upstream node digests, the placeholder artifacts, the preimages and the members. Reworking a copy is not removing it. The same enum digest is copied again in `codes.rs`, whose doc still says "The vendored fixture's". Remedy: derive the node ids from the preimages the way `wire()` already derives the package id, or depend on the upstream vector. | tests/checked_package_support/base.rs:12-15; tests/composite_equality_support/codes.rs:10-14 |
| FND-003 | medium | The spine test lost its obligation-module mutation control. It used to change `*post_state >= 0_i64` to `>= 1_i64` and expect `Falsified`, which was the only check that the generated `ensures` bound carries weight and is not vacuous. That is functional, not part of the claimed-module gate, and nothing replaces it. | tests/it/skeleton_spine.rs:374 |
| FND-004 | medium | Dead code after the removal. `KaniToolError::Failed` and `::UnexpectedOutput` are never constructed. `KaniInstallation.kani_home` is never read, yet `discover_from` still refuses when neither `KANI_HOME` nor `HOME` is set, so a caller with no HOME is refused for a value nothing uses. | src/kani_execution.rs:80-92; src/kani_execution.rs:122; src/kani_execution.rs:169 |
| FND-005 | low | `tc_001_boolean_oracle_bundle` used to execute the generated oracle and its identity constants. It now asserts substrings of the generated source. `A -> false` is still executed by the TC-002 differential corpus, but the identity constants are no longer executed anywhere. | tests/it/oracle_generation.rs:445-464 |
| FND-006 | low | `ItemSettlement::routed` and `RoutedItem` are public API whose only purpose was the removed tool probe. Only tests call them. | src/capability.rs:692-727 |
| FND-007 | low | Stale doc comments still describe removed pins and attestations: "the pinned version", "pinned lane", "first pinned adapter", "attested source-size limit", "Pinned runtime revision", and v2 schema "fixes adapterProfile/backendVersion". There are also stale test-doc lines ("pin drift", "real pinned backend", "pinned prover"), and a test name mentions attestation. | src/kani_transcript.rs:3; src/kani_execution.rs:1200; src/kani.rs:139; src/strategy.rs:132; src/bounded_kani_corpus.rs:31; src/generation.rs:57; tests/it/kani_obligations.rs:56; tests/it/skeleton_spine.rs:8; tests/it/harness_generation.rs:761 |
| FND-008 | low | The PR body's gate numbers are stale: it says lib 85, it 240, doc 8, and head measures 82/221/7. Its Kani gate filtered only `kani_obligations` and `skeleton_spine` and reported 0, but at head that filter fails 3 tests. | PR #189 body |
| FND-009 | high | Remaining ceremony (case 1, source-reading tests, and count assertions): tests that read `src/` text and ban substrings or count occurrences, including the `sites.len() >= 12` floor and the interface-001 table and variant counting. | tests/it/kani_obligations.rs:1618; tests/it/kani_obligations.rs:1695; tests/it/routed_generation.rs:205; tests/it/capability_settlement.rs:473; tests/it/capability_settlement.rs:504; tests/it/interface_001.rs:268; tests/it/interface_001.rs:364 |
| FND-010 | high | Remaining duplication and digests (cases 2 and 3): law-role catalog digests copied "verbatim from quire-contract-ir dfd8bd78 operation-catalog.json". This predates the PR. | tests/exact_scalar_support/package.rs:298-340 |
| FND-011 | high | Remaining pin (case 3): `pub const IR_CANDIDATE_REVISION` is unused and restates the Cargo.toml rev. | src/oracle.rs:14; src/lib.rs:156 |
| FND-012 | high | Remaining pins (case 3): orphaned schemas `kani-proof-graph-v1` and `generated-rust-kani-v1` still hold Kani 0.67.0 adapter and profile constants and backend executable and source digests. Nothing references them. | schemas/kani-proof-graph-v1.schema.json; schemas/generated-rust-kani-v1.schema.json:9 |
| FND-013 | high | Remaining digests and provenance (case 2): the bound coverage report records `export_sha256`, `artifacts[].sha256`, `expression_sha256`, `declaration_sha256`, `provenance: "unqualified"` and `bound_sha256`, which is recorded but never compared. `LlvmCoverage::export_sha256` is consumed only by a length check. The owner decides whether `bound_sha256` is a binding key. | src/bound_coverage.rs:60-93; src/bound_coverage.rs:344-353; src/bound_coverage.rs:484-485; src/vacuity.rs:169-179; schemas/bound-coverage-observations-v1.schema.json |
| FND-014 | high | Remaining provenance and pins (cases 2 and 3): the bounded Kani corpus emits a `.provenance` artifact with the profile's executable digest, options digest, ABI and census digest. It also folds `executable_digest` and `options_digest` into each case's `identity_preimage`, so case names change when the Kani executable digest changes. | src/bounded_kani_corpus.rs:510-535; src/bounded_kani_corpus.rs:644-672 |
| FND-015 | high | Remaining pins (case 3): `manifest_digest` on `BackendDescriptor` and `Candidate` adds no uniqueness, since duplicate identities are already refused, and only rejects a changed manifest. `ClaimMap.runtime_revision` and `version` in `claim-map.json` repeat what the generated Cargo.toml says. The test asserts `rev = "{RUNTIME_REVISION}"` in the generated manifest. | src/capability.rs:170-198; src/capability.rs:605-612; src/generation.rs:57-58; tests/it/exact_scalar_generation.rs:1337 |
| FND-016 | high | Digests that may be product identity; the owner decides (case 2). `Artifact.sha256` and `bundleSha256` are stored and re-validated self-consistency fields. The publication ownership marker, which refuses to overwrite hand-edited generated files, is real behaviour. Symbol naming uses SHA-256: `oracle_symbol`, the clause `kob_` digest, `kani_symbol`, `harness_symbol`, and the strategy and population suffixes. The agreement cases hard-code `oracle_<64-hex>` names. | src/oracle.rs:168; src/oracle.rs:1044-1062; src/publication.rs:355; src/publication.rs:397-406; src/publication.rs:520-565; src/kani_obligations.rs:930-956; src/kani.rs:1036-1045; src/harness.rs:1060; tests/composite_equality_support/agreement_cases.rs:70 |
| FND-017 | high | Remaining assurance-chain leftovers (case 4): the conformance example keeps a `PROTOCOL` identifier and per-row `trace_ids` built for the deleted Quoin intake. CI still pins `quoin@0.23.1` and `ix-flow@0.0.4` and cites the deleted `assurance/pins.json` and `check_shared_pins.py`. The workflow was not touched, and editing it needs owner clearance. | examples/generation_conformance.rs:32-38; examples/generation_conformance.rs:59; .github/workflows/ci.yml:36-50 |
| FND-018 | high | Remaining pins and counts (case 3): the Makefile comment "selects exactly the 20 tests … 19 default-lane plus this one" is stale, since `kani_obligations` now holds 21 tests and 3 ignored. `MSRV := 1.98.1` and the `msrv` lane repeat `rust-toolchain.toml`, which also says 1.98.1. Cargo.toml says "exactly one file under examples/". deny.toml carries Rust 1.75/1.88 advisory prose. | Makefile:11; Makefile:57-64; Cargo.toml:16-22; deny.toml:62-65; CLAUDE.md:13 |
| FND-019 | high | README "Generated artifacts" still says every artifact carries a Quoin `ProofAttestationV1` that "Quoin seals". Attestations were removed by this PR. It also says "The two documents under schemas/", and there are 8. Remaining README lines ask for remote runs to be "retained when used as evidence". | README.md:18-31 |

## Dispositions

Round 1, reviewed at c0cc093bb657a92d280159e16424518e5440fd83. Gates at that head:
- `cargo fmt --check` 0, `make lint` 0, `make test` 0 (77 / 209 passed with 5 ignored / 7).
- Kani lane: `tc_027_a_routed_scalar_harness_verifies` ok, `tc_027_..._violating_its_bound_is_falsified` ok, `kani_witness_join` tc_026 ok (it fails on main), spine tc_026 ok.
- `tc_025_real_kani` fails with `Inconclusive{VacuousProof}` on the precondition, the same way on main 6e0c518, so it predates this PR.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 87963e9: `kob_scalar_{op12}_{index}` (src/kani_obligations.rs:1345); both routed scalar Kani tests pass at c0cc093 |
| FND-002 | fixed | 87963e9: node ids come from `quire_contract_ir::NominalIdentityPreimage::digest` (tests/checked_package_support/base.rs:57-62); the copied enum digest is gone from codes.rs. See FND-020 |
| FND-003 | fixed | 87963e9: tests/it/skeleton_spine.rs:380-390 tightens `ensures` to `>= 1_i64` and asserts `Falsified`; passes under Kani |
| FND-004 | fixed | 87963e9: `KaniToolError::Failed`/`UnexpectedOutput`, `KaniHome` and `kani_home` removed |
| FND-005 | fixed | 87963e9: tests/it/oracle_generation.rs:442-456 compiles and runs the oracle (`f(false)`, `!f(true)`) |
| FND-006 | fixed | 87963e9: `RoutedItem` and `ItemSettlement::routed` deleted |
| FND-007 | fixed | 87963e9: no pin or attestation wording left in src/ doc comments |
| FND-008 | fixed | PR body gates now match the measured 77/209/7 |
| FND-009 | fixed | 87963e9: source-reading and count tests removed, including all of interface_001.rs |
| FND-010 | fixed | 87963e9: catalog read from `quire_verification_contracts::operation_catalog::CHECKED_OPERATION_CATALOG_V1` (tests/exact_scalar_support/package.rs:324-336) |
| FND-011 | fixed | 87963e9: `IR_CANDIDATE_REVISION` removed |
| FND-012 | fixed | 87963e9: orphan schemas deleted |
| FND-013 | fixed | 87963e9: bound coverage digests and provenance removed; the schema has no digest fields. See FND-022 |
| FND-014 | fixed | 87963e9: corpus `.provenance` removed; the identity preimage is replaced by a counter |
| FND-015 | fixed | 87963e9: `manifest_digest`, ClaimMap `version`/`runtime_revision` removed |
| FND-016 | fixed | 87963e9, by owner ruling: `Artifact.sha256`/`bundleSha256` and the ownership marker removed; symbols are readable text plus a counter |
| FND-017 | fixed | 87963e9: `PROTOCOL`/`trace_ids` removed; ci.yml pins and the `pins.json` comment removed |
| FND-018 | fixed | d82ac87: Makefile count, Cargo.toml count and deny.toml prose removed |
| FND-019 | fixed | d82ac87: README describes the schemas without attestations |

## New findings (disposition pass 1)

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-020 | high | The test support re-implements Contract IR's private identity algorithms. It computes `package_id` as sha256 of `identity_preimage`, builds the identity projection by dropping `occurrences`, and hashes application-node preimages. IR has no public API for any of these (`digest_json` is `pub(super)`, `application_preimage` is private). This is two implementations of one rule. The fix belongs in IR: expose these helpers and call them from here. | tests/exact_scalar_support/package.rs:1000-1014; tests/exact_scalar_support/package.rs:566-577; tests/composite_equality_support/package.rs:414-424; tests/composite_equality_support/package.rs:476 |
| FND-021 | medium | Generated source writes node-id digests into comments nothing reads. The kani obligation header has `// Obligation: exact-scalar node {domain}/{digest}`. Oracle doc comments have ``/// Node `{digest}` ``. The `.call(...)` key in exact_function is `declaration.name`, not the digest, so only the doc comment carries the digest there. Also, 9 generator headers stamp `Generated by quire-contract-codegen {CARGO_PKG_VERSION}`, a version string nothing reads (low). | src/kani_obligations.rs:1987-1990; src/exact_scalar.rs:2373-2374; src/composite_equality.rs:1335; src/composite_equality.rs:1343; src/exact_function.rs:1246-1256 |
| FND-022 | high | Owner ruling, remove (self-equality): `generate_bound_oracles(package) != generated` compares a function's output with a second run of the same function on the same input. The analyzer has no callers outside tests. Remove the check, `CoverageErrorCode::BindingMismatch`, and their tests. | src/bound_coverage.rs:200-206 |
| FND-023 | medium | Routed harness names now depend on request position (`kob_scalar_{op}_{index}`, where the index is the position in the Kani group). FR-022 (line 214, AC-9) requires a harness whose bytes are "independent of the item's request index". Scenario: node N routed with a sibling M gets `_1` or `_0` depending on M's index. `tests/it/routed_generation.rs:394` only routes a node alone, so it cannot catch this. Fix the code or the FR in #186. | src/kani_obligations.rs:594; src/kani_obligations.rs:1345; spec/functional/complete-v1/FR-022-routed-generation.md:214 |
| FND-024 | medium | Cutting readable names to 24 or 12 characters, and joining with `_`, makes distinct ids collide, and the collision refuses the whole package. Example: clause ids equal in their first 24 characters. Another: (req `fr-1`, rev 2, clause `c`) and (req `fr`, rev 1, clause `2-c`) both give `oracle_fr_1_2_c`. No test exercises `NameCollision` or `readable_name_component`. | src/oracle.rs:1019-1062; src/bound.rs:156; src/kani_obligations.rs:943 |
| FND-025 | low | exact_scalar increments `generated_count` before the `catalogued_operation_identity` refusal, so a refused sibling still uses up a number. That contradicts the comment at :638, "a refused sibling never renames a generated one". | src/exact_scalar.rs:638-677 |
| FND-026 | low | `sha2` is still in `[dependencies]` but src/ no longer uses it; only tests do. Move it to `[dev-dependencies]`. | Cargo.toml:34 |
| FND-027 | low | In-repo duplication: `harness.rs` `to_upper_camel` duplicates the new `oracle::upper_camel`. | src/harness.rs:1066; src/oracle.rs:1040 |
| FND-028 | low | Stale test comments: a deleted fixture path, a "135-entry" count, and IR commit `dfd8bd78`. | tests/composite_equality_support/package.rs:171-172; tests/exact_scalar_support/package.rs:56 |

### Round 2

Reviewed at 476dbd1799cdd223b8a09fd79a11fd80e56dd837. Gates:
- `cargo fmt --check`: exit 0.
- `make lint`: exit 0.
- `make test`: exit 0 (lib 80, it 212 passed with 5 ignored, doc 0).
- Kani lane: routed scalar tc_027 (both), `kani_witness_join` tc_026 and spine tc_026 pass.
- `tc_025_real_kani` is still `Inconclusive{VacuousProof}`. It fails the same way on main 6e0c518.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-020 | deferred | This predates the PR: main 6e0c518 already has `sha256_hex` package-id and application-preimage hashing in tests/exact_scalar_support/package.rs:397,575,1011. The fix is for Contract IR to expose its `digest_json` and `application_preimage` helpers; that needs an IR ticket. |
| FND-021 | fixed | cba94dd: the header now reads ``// Obligation: exact-scalar `{operation}` ``, and the oracle doc comments no longer carry node digests (src/kani_obligations.rs:2084; src/composite_equality.rs; src/exact_scalar.rs; src/exact_function.rs). The version stamp is split out as FND-030. |
| FND-022 | fixed | cba94dd: the `BindingMismatch` check, the `CoverageErrorCode::BindingMismatch` variant and the `binding_mismatch` schema value are removed. |
| FND-023 | fixed | cba94dd: `assign_names` settles names through `oracle::unique_names`, ordered by full identity rather than request position. Covered by `tc_033_a_node_gets_identical_bytes_whatever_its_differently_named_siblings` and `tc_033_nodes_sharing_a_stem_take_stable_ordinals_in_node_order`. |
| FND-024 | fixed | cba94dd: coinciding stems get ordinals instead of being refused. `NameCollision` is removed from `bound.rs`. Covered by `readable_stems_that_coincide_still_yield_distinct_names`. |
| FND-025 | fixed | cba94dd: exact_scalar names pending oracles once all are known, so refused items take no ordinal. |
| FND-026 | fixed | cba94dd: `sha2` moved to `[dev-dependencies]`. |
| FND-027 | fixed | cba94dd: `harness.rs` `to_upper_camel` removed; only `oracle::upper_camel` remains. |
| FND-028 | fixed | cba94dd: the IR commit ids, the "135-entry" count and the deleted fixture path are gone from test comments. |

## New findings (disposition pass 2)

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-029 | medium | The harness refusal of identical precondition and postcondition clauses (`DuplicateClauseIdentity`) has lost its only test. That test was the `harness_rejects_invalid_input(true)` case in the conformance example, which cba94dd deleted. No test under tests/ exercises this refusal. It is a real check: a harness that compares a clause with itself proves nothing. Add one `tc_004` case to harness_generation. | src/harness.rs:141-147 |
| FND-030 | low | Nine generator headers stamp `Generated by quire-contract-codegen {CARGO_PKG_VERSION}`, a version string nothing reads. | src/oracle.rs:406; src/harness.rs:1025; src/kani.rs:820; src/kani_obligations.rs:1842; src/strategy.rs:271 |
