---
id: SR-616002
title: IR-1091 CG PR 616 feasible OrderedSet uniqueness gap analysis
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; PR 616 (stacked on PR 613); diff 731029ed..0d685342; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, src/kani/generate/collection.rs, src/kani/generate/render.rs, tests/it/kani_obligations_symbolic_domains.rs"
review_set: subset
---

## Summary

Ticket: IR-1091. Planless gap analysis of frozen head 0d6853429fe91b4f663edcf8f51b5e32cde622e4 against stacked base 731029ed4fb11f4b9dbacea74a5329e44b6ebb30. Plan completion: not assessed. Examined FR-015-AC-96, -97, -99, -100, -101, -102 and TC-025 against the PR's tagged tests and production code. The PR changes no `spec/` file; draft PR 615 (max greater than capacity) is held for the owner and was not treated as authority.

## Verdict

CONDITIONAL. Code and tests back the feasible OrderedSet clause of FR-015-AC-96 (Boolean and IntRange, max at most distinct capacity): three AC-96-tagged tests (`feasible_ordered_set_emits_symbolic_ordered_unique_slots`, `boolean_ordered_set_keeps_both_values_and_distinct_active_slots`, `unique_capacity_uses_the_complete_domain_without_integer_overflow`) plus the Kani-lane `tc_025_real_kani_ordered_set_claim_and_mutant_keep_distinct_active_values`. AC-100 kind-in-identity is backed by the extended `every_identity_component_changes_the_rendered_harness_path` and the `assert_ne` in the feasible test. No AC was narrowed and no over-capacity path emits a harness. The tags are justified; none is a false positive. The gaps are spec status drift and an AC-99 oracle gap for the newly supported kind.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-015-AC-96 status prose still states that the uniqueness clause is unbacked and that ordered and unique shapes are refused, not modelled. At this head the code models feasible ordered sets, and AC-96-tagged tests back them, so the spec status contradicts the code. It needs an owner-approved status update, separate from the held over-capacity wording. | spec/kani/functional/FR-015-bounded-kani-obligations.md:846 |
| FND-002 | low | FR-015-AC-102 (and AC-96) cite `ordered_set_shape_is_refused_without_erasing_its_uniqueness_semantics` (:1461) as backing. The PR renamed that test to `ordered_set_over_capacity_keeps_the_existing_typed_refusal_without_a_harness` and removed its AC-97/AC-102 tags, so the citation dangles. AC-102 stays backed by the two other tagged tests. | spec/kani/functional/FR-015-bounded-kani-obligations.md:852 |
| FND-003 | medium | FR-015-AC-99 requires the independent native oracle to replay the same authored function through `qsl_replay::replay`. OrderedSet claims now settle Supported, but nothing replays them natively: the new Kani-lane test omits `assert_native_collection_result`. The dispatch brief reports that the QSL replay helper refuses ordered_set with UnknownRequiredFeature; the reviewer did not reproduce that. The PR does not tag AC-99, so nothing is falsely claimed, but a supported kind has no oracle parity. | tests/it/kani_obligations_symbolic_domains.rs:2210 |

## Coverage

Examined FR-015-AC-96 (uniqueness and order clause; Boolean and IntRange endpoints), AC-97 (unsupported collection form, no finite-ceiling substitution), AC-99 (symbolic draw, cover after assertions, native oracle, healthy and mutant controls), AC-100 (identity includes the collection kind), AC-101 (typed refusal; never tightens the model domain) and AC-102 (one disposition per item; only supported items emit harnesses). TC-025 and AD-004 (shared `render.rs` path) were read as context. The renderer change stays in the shared `HarnessSpec`/`render.rs` path, which fits AD-004.

AC-101: the capacity check uses the full admitted element domain and refuses rather than narrowing. `collection_i64` refuses out-of-range integers rather than clamping them. Over-capacity requests get a typed refusal with no harness, so no vacuous proof is emitted. The refusal reuses `UnsupportedFieldType`, which does not name the infeasible cardinality. That is left to the owner's PR 615 decision and is not raised here.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | At 9f6a3b087e64c39147946ae59f3a7b4ef138e2de the FR-015-AC-96 row status reads PARTIAL (IR-268, IR-1091): feasible OrderedSet domains up to the distinct-value capacity are backed by AC-96-tagged tests, and the over-capacity branch is pending the owner ruling (IR-1091 / CG #615). The normative AC text is byte-identical, it makes no full-AC claim, and `quire trace --id FR-015-AC-96` resolves all four cited tests. The other status sites that still contradict this are new findings in SR-616003. |
| FND-002 | fixed | The FR-015-AC-102 row no longer cites the renamed test; it stays IMPLEMENTED on `collection_negotiation_accounts_for_each_supported_or_refused_item` and `unsupported_collection_predicate_is_refused_without_harness`, both still tagged AC-102. The remaining dangling citation in spec/kani/matrix/tests.md is SR-616003 FND-003. |
| FND-003 | fixed | QSL moves to 579d4f40 (QSL-814 #1480, the review-fix merge after #1478 19a1d3a6). The native replay helper now takes a `CollectionKind` and builds `SnapshotValue::collection(kind, ...)`. The OrderedSet Kani-lane control replays the same authored healthy function natively on [0, 1] (true) and the mutant's real Kani playback (false), so the oracle and Kani playback agree. Existing Sequence callers pass `CollectionKind::Sequence` unchanged. |
| FND-004 | fixed | Round 2, head bc3d50c93cfe9ad44974b762606477bc8069799a: `native_collection_replay_evaluates_the_authored_healthy_and_mutated_functions` (regular lane, tagged FR-015-AC-99) now replays the same authored `forall(item >= 0)` over an OrderedSet field natively, using `SnapshotValue::collection(CollectionKind::OrderedSet, [0, 1])`, and expects true. Default `cargo test` therefore reaches the QSL-814 OrderedSet snapshot path. The ignored Kani-lane mutant replay of real playback (false) remains at :2272. Reviewer ran it at this head: 23 passed, 3 ignored. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | Native OrderedSet replay through `SnapshotValue::collection(CollectionKind::OrderedSet, ...)` runs only inside `#[ignore]` Kani-lane tests. Default `cargo test` never reaches the QSL-814 snapshot path, so a QSL regression there shows up only on `make kani-gate`. The healthy [0, 1] native check needs no Kani and could join `native_collection_replay_evaluates_the_authored_healthy_and_mutated_functions`. | tests/it/kani_obligations_symbolic_domains.rs:2249 |

## Capacity follow-up review

Reviewed exact head `03f5d74afe349ea8a7c95f0c673118a75ce746de`, commits `efd0bd8d..03f5d74a`. CLEAN: no new findings. AC-96's newly merged capacity clause is covered by tagged authentic QSL→IR controls for IntRange max equal to capacity, IntRange max above capacity, and Boolean minimum above capacity, plus a tagged independent wide-arithmetic property test and full-i64 interval unit test. The shared assertion checks one `Unsupported` item, exact `UniqueElementCapacityExceeded { capacity, declared_maximum }`, and no collection harness. Existing feasible OrderedSet controls still cover unique active slots and preserved order; no AC was narrowed. Installed-backend proof remains explicitly PARTIAL in the spec status.
