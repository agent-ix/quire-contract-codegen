---
id: SR-17940486709564847047973
title: "IR-805 PR #830 code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #830 at 5a16e153, git diff origin/main...HEAD; IR-805 slice E; src/kani/generate/graph.rs, src/kani/generate/negotiate.rs, src/kani/generate/outcome.rs, src/kani/identity.rs, src/replay/state_clause/graph.rs, tests/it/kani_graph_private.rs, tests/it/kani_obligations_symbolic_graphs.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
---

## Summary

Ticket: IR-805 slice E. Code-review with the rust-review lane folded in. PR #830 at 5a16e153.
I read the whole diff: the `(node, occurrence)` claim key, the move to document-held ids through
`resolved_graph`, the quantified `Claim` lowering (`Shape`), the oracle renderer and the replay
occurrence guard. I also judged the strength of the three real-Kani oracles and re-ran the new tests
under `flock` (results in the Verdict).

## Verdict

**PASS with findings (no high).** The occurrence keying is correct. The location check requires the
document-held node id and an occurrence that the node carries (`graph.rs:176-181`). An absent
occurrence settles `MissingClaim`. The occurrence is in `ItemIdentity::Graph`, so a repeated
`(node, occurrence, path)` is rejected as `InvalidRequest`, and that is tested. It is also in the
harness preimage and in `GraphObligationIdentity`, and the source map is filtered by role and
ordinal. Binders resolve by name and level. A binder's type must be a reference to the population's
object type (`references_population_type`). The quantifier collection must be `all_instances` of that
same type, and depth is capped at 4. The unwind rule `max(n, upper) + 1` still covers the new
quantifier loops, which run over `0..n`, so unwind 8 is consistent with `UnwindBelowDomain`.

The oracles are not vacuous. The healthy transitivity run is `Verified`, which `classify.rs:279`
grants only when all covers are satisfied, so `object_count == N` was reached. The no-closure mutant
asserts that the decoded witness contains a two-edge chain with no direct edge, and that QSL holds the
unchanged clause. The reversed-claim mutant asserts that QSL independently reaches `Violation` on the
decoded graph. QSL evaluates the authored clause itself rather than the harness's `paths`, so it is a
real independent oracle for clause truth. The settlement is `Inconclusive` with a `Witness` cause, and
that is asserted and stated honestly. The findings are an untested replay guard that compares the
ordinal only, a one-sided depth-cap boundary, and one `as` cast.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new replay guard compares only `origin().ordinal()` against `CheckedOccurrence.ordinal` and ignores `role`, though `CheckedOccurrence` is `{role, ordinal}` ("stable ordinal for repeated roles") and `negotiate.rs` filters the source map on both. No test exercises it: every replay uses `occurrences[0]` under clause name `Graph`, and none replays a second-occurrence harness or asserts `ClauseSelectionMismatch` for a mismatched occurrence. Scenario: IR's per-role occurrence ordinal and QSL's call-site origin ordinal number differently for the two-declaration package. Then a valid replay of occurrence 1 is refused as `ClauseSelectionMismatch`, or a harness keyed by role X ordinal 0 is accepted under a clause whose origin is role Y ordinal 0, and no test notices. Compare role too, and add a two-occurrence replay test (accept the matching name, refuse the other). | src/replay/state_clause/graph.rs:353-358; src/kani/generate/negotiate.rs:1225-1231 |
| FND-002 | low | The quantifier depth cap is tested only on the reject side (five binders). Transitivity uses three, so changing `nesting >= MAX_QUANTIFIER_DEPTH` to `>`, or `MAX_QUANTIFIER_DEPTH` to 5, goes undetected. Add a four-binder claim that settles `Supported`. | src/kani/generate/graph.rs:443; tests/it/kani_obligations_symbolic_graphs.rs:1060-1090 |
| FND-003 | low | `(self.parameters.len() + nesting) as u64` uses an `as` integer conversion where the repo idiom is `u64::try_from`. It is lossless on 64-bit, but it is the pattern rust-style flags. | src/kani/generate/graph.rs:448 |

## Examined scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| lower_graph location check | examined | `package.resolved_graph().nodes.iter().any(|node| &node.node_id == clause && ... && node.occurrences.contains(occurrence))` |
| Shape::operand / references_population_type | examined | binder resolved by `(name, level)`; type must be `Aggregate[Reference(object_type)]` |
| Shape::quantifier | examined | collection must be `quire.op.model.all_instances` of the population object type; depth cap 4 |
| Claim::rust / oracle helper | examined | quantifiers loop `0..maximum` guarded by `q < count`; closure loop unchanged |
| render_graph unwind rule | examined | `required = n.max(upper) + 1`; `UnwindBelowDomain` otherwise |
| classify_graph / ItemIdentity::Graph | examined | occurrence in identity; source_map filtered by role and ordinal |
| StateClauseReplay occurrence guard | examined | `site.occurrence.origin().ordinal() != occurrence.ordinal` -> `ClauseSelectionMismatch` |
| real_kani_healthy_transitivity_verifies_over_a_zero_to_three_end | examined | asserts `KaniRunOutcome::Verified` (all covers satisfied) |
| real_kani_transitivity_without_closure_mutant_falsifies_and_qsl_replay_disagrees | examined | asserts a 2-chain without a direct edge in the witness and `assert_clause_holds` |
| real_kani_reversed_transitivity_claim_falsifies_and_qsl_replay_agrees_on_the_verdict | examined | asserts `Inconclusive`, `Violation`, `DisagreementCause::Witness{given: None, derived: Some}` |
| one_node_under_two_clauses_is_two_items_with_different_identities | examined | two occurrences -> two identities; duplicate -> InvalidRequest; ordinal 99 -> MissingClaim |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | The lower-bound-dropped mutant test asserts only `Falsified`. Its doc comment and AC-106 say QSL replay is skipped because the decoder refuses a degree below the end's lower bound, but the test never calls `decode_falsification` to assert that refusal, so the stated reason is unverified by any test. Non-vacuity still holds by construction: the healthy harness differs only in `degree_0 >= 2` and verifies, so the falsifying draw must have `degree_0 < 2`. Scenario: a decoder regression that admits a degree below `lower` goes unnoticed, and the spec's skip rationale silently stops being true. Assert `decode_falsification` returns `GraphWitnessNotInDomain` on this playback. | tests/it/kani_graph_private.rs:936-948 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | `names_occurrence` compares role and ordinal (src/replay/state_clause/graph.rs:303-315). A unit test covers the role, and the real-Kani test `real_kani_each_occurrence_replays_only_under_its_own_clause_name` checks both occurrences against both names. |
| FND-002 | fixed | `four_nested_binders_are_the_deepest_supported_claim` emits a harness at four binders; the five-binder test still refuses. |
| FND-003 | fixed | `u64::try_from(self.parameters.len() + nesting).map_err(|_| unsupported())?` (graph.rs:446). |
