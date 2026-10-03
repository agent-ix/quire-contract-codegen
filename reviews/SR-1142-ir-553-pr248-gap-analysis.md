---
id: SR-1142
title: "gap analysis of quire-contract-codegen PR 248 (IR-553 function-path obligation identity)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@360d58dae87a4486b4bcad945b8b1f4461e6e5f6; FR-016-AC-21, FR-016-AC-22, FR-016-AC-23, TC-026, AD-002, AD-003 E-1; src/replay/obligation.rs, src/replay/function.rs, tests/it/skeleton_spine.rs"
review_set: subset
---

## Summary

Ticket: IR-553. Manual acceptance-criteria-to-tests check, scoped to the PR diff, with
test-oracle strength judged per assertion. Plan completion: not assessed.

Examined: FR-016-AC-21 (tc_026_the_request_slot_is_the_recomputed_function_contract_digest,
tc_026_an_i64_extreme_domain_is_encodable), FR-016-AC-22
(tc_026_the_identity_separates_functions_kinds_and_domains_and_ignores_text_and_span,
tc_026_a_two_conjunct_function_has_one_identity_per_kind), FR-016-AC-23
(tc_026_an_unrelated_declaration_does_not_change_the_identity,
tc_026_only_the_four_members_decide_the_identity,
tc_026_the_identity_ignores_the_declared_order_of_the_site_parameters). Every test carries a
`Trace:` tag that resolves. `quire coverage --strict`: 66 unbacked, 0 contradicted (baseline);
FR-016 20/23 backed (AC-6, AC-7, AC-12 planned). `quire validate --scope . "spec/**/*.md"`:
only the existing `semantic.`/Duplicate warnings.

Clean: the AC-21 oracle hand-writes the RFC 8785 text in sorted key order and hashes it with
`sha2`, so it fails if the encoder is bypassed (serde_json struct order), a member is renamed
or dropped, the digest gains a label, or the domain spelling changes for integer ranges; it
also reads the node ids from QSL's own `call_site`, not CG's package. AC-22 separates
functions, kinds and a narrowed domain against a real recompile and a comment-only recompile.
AC-23 covers the unrelated declaration, the site-parameter reorder and the
function/declaration perturbations.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-21 is about the function path's request, but no test observes the slot `replay_counterexample` actually sends: the AC-21 test builds the request itself with `package.request(obligation, ..)`, and QSL does not check the slot, so replacing `obligation` with any fixed value at function.rs:536-545 leaves every test green. The production wiring the AC claims is untested | tests/it/skeleton_spine.rs:698-719 |
| FND-002 | low | No test discriminates "ascending by identifier" from another canonical order such as ascending parameter node id: every unit fixture gives identifier order equal to node order (`a`=node 1, `b`=node 2), the reorder tests assert invariance only, and the AC-21 fixture's two parameters are already ascending in declared and harness order, so only the incidental hash order of their node ids could catch a wrong sort key | src/replay/obligation.rs:240-265 |
| FND-003 | low | The AC-21 oracle is not independent for two spellings: it takes the kind string from CG's own `Serialize` (`serde_json::to_value(kind)`), and it covers no Boolean argument, so the `{"type":"boolean"}` domain spelling is checked by no golden text; a change to either alters every identity of that shape with no failing test (NFR-001 regeneration stability) | tests/it/skeleton_spine.rs:653-679 |

## Verdict

AC-21 to AC-23 are backed by real tests with strong oracles for the preimage bytes. FND-001
should be fixed in this PR (expose or assert the slot of the request `replay_counterexample`
builds, for example through a crate-level unit test of the closure, or by a test seam that
returns the wire). FND-002 and FND-003 are cheap additions to the golden test.

## Dispositions

Round 1, reviewed at 5aee2cf161b3b1068c3d70d4de895de0cba8a377. `quire coverage --strict`: 66
unbacked, 0 contradicted; FR-016 20/23. `cargo test tc_026`: 5 + 26 passed.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 5aee2cf |
| FND-002 | fixed | 5aee2cf |
| FND-003 | fixed | 5aee2cf |
