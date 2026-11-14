---
id: SR-17940486709564847047980
title: "IR-805 PR #832 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #832 at ec0646b1, git diff origin/main...HEAD; IR-805 slice D; FR-015-AC-98, -100, -105, -106, -107, -108, TC-025; src/kani/generate/graph.rs, src/replay/state_clause/graph.rs, tests/it/kani_graph_field_private.rs, tests/it/kani_obligations_symbolic_graphs.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-805 slice D. Plan completion: not assessed. I took the test bindings from
`quire trace --file` / `--id` and judged each against the committed lock, where IR c8dca8c6
refuses `reaches_field` in `reaches_population` (`NotReachesApplication`).

| AC | Bound field-edge tests | What runs on the committed lock |
| --- | --- | --- |
| AC-98 / AC-105 | `field_edge_domains_use_the_population_and_the_member_type_multiplicity` | refusal branch only; no CG field-edge code |
| AC-100 | `the_followed_field_changes_the_harness_identity_and_equal_inputs_regenerate_it` | refusal branch for `next`, then `return` |
| AC-106 | 1 native replay test plus 6 ignored `real_kani_*` | replay test runs and is mutation-sensitive; the 6 controls panic at `harness()` |
| AC-107 | `a_field_edge_claim_is_located_first_and_one_field_per_claim`, `a_field_edge_and_a_relationship_end_reachability_are_separate_items` | location-first refusal at path `[1]` is real; the two-field `UnsupportedClaimShape` branch is skipped |
| AC-108 | the two AC-107 tests plus `qsl_refuses_a_scalar_collection_field_edge_before_a_package_exists` | no field-edge typed refusal (`FieldNotAdmitted`, `FieldEdgeNotReference`, `PopulationTypeMismatch`, unbounded `requires_bound`, `PopulationUnavailable`) is produced by any test |

## Verdict

**FAIL: coverage inflation on AC-98, AC-105 and AC-100.** The spec's status words are honest
(PARTIAL; verdicts from an uncommitted composition). But three ACs carry computed-matrix bindings
from tests that, on this tree, assert only the upstream IR refusal.

The wrong-field clause, evidenced by the multiplicity claim (`maybe` vs `next` for
"every object has an edge"), is acceptable and honestly stated. With a one-edge harness, a
harness-edit wrong-field mutant cannot be expressed, and the multiplicity claim does show that the
member type is read, not assumed. Its Kani verdict, however, is one of the non-reproducible ones.

The ignored installed-Kani controls are meaningful as written: real mutants, decoded playback, and
replay over the authored clause. Gating them with `#[ignore]` matches the rest of the Kani lane.
But on the committed lock they cannot pass, only fail.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Coverage inflation. `field_edge_domains_use_the_population_and_the_member_type_multiplicity` is bound to AC-98 and AC-105, and `the_followed_field_changes_the_harness_identity_and_equal_inputs_regenerate_it` to AC-100. On the committed lock both return early after asserting `NotReachesApplication`, which the IR produces before any CG field-edge code runs. The matrix shows these ACs as covered by field-edge tests that verify nothing about field-edge lowering. Mutation evidence is in SR-17940486709564847047979 FND-001 (M1 survives). Scenario: a matrix reader sees AC-98/105/100 field-edge bindings as live coverage. Fix: add the committed-lock `field_edge` seam test from that finding and bind it; or tag the dual-branch tests so the matrix does not count the refusal branch as AC-98/105/100 evidence. | tests/it/kani_obligations_symbolic_graphs.rs:376-431 |
| FND-002 | medium | The AC-108 field-edge refusals are "covered only by inspection", but they are testable. The spec says `FieldNotAdmitted`, `FieldEdgeNotReference` and an unbounded field-edge Collection are "reachable only through a package QSL does not emit, so they have no test". That is true of the end-to-end path, not of the mapping. `field_edge` takes a package, a population, a declaration and a name, and can be driven on the existing fixture, which already declares a scalar `items` field and bounded and unbounded shapes are authorable in the domain document. Scenario: a wrong reason (for example `FieldEdgeNotReference` swapped with `FieldNotAdmitted`) ships unnoticed. Fix: a seam test asserting each reason. | src/kani/generate/graph.rs:385-439; spec/kani/functional/FR-015-bounded-kani-obligations.md:943,953 |
| FND-003 | low | The six AC-106 `real_kani_*` field controls are bound in the matrix but fail deterministically on the committed lock (verified: `real_kani_negated_field_claim_falsifies_and_qsl_replay_reproduces_it` panics at `kani_graph_private.rs:225` in 0.03 s, before Kani). Scenario: whoever runs the documented `cargo test --lib kani_graph_field_private -- --ignored` on this tree sees six failures with no message saying why. Fix: have `harness()` in this module fail with an explicit "requires QSL-1904 population link" message, or skip-with-reason when `links_population` is false, and say so in AC-106. | tests/it/kani_graph_field_private.rs:1-9,172-292 |

## Examined scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-015-AC-98 | examined | field-edge population from `reaches_population`/`select_population`, edge from the member type; PARTIAL on QSL-1904 |
| FR-015-AC-100 | examined | followed field changes the harness identity |
| FR-015-AC-105 | examined | field-edge draw from member type; PARTIAL |
| FR-015-AC-106 | examined | installed-Kani field controls measured on an uncommitted composition; PARTIAL; wrong-field as multiplicity claim |
| FR-015-AC-107 | examined | field-edge located like any other; two-field condition `UnsupportedClaimShape` when linked |
| FR-015-AC-108 | examined | typed refusals; `PopulationUnavailable`; `NotReachesApplication` on older IR |
| TC-025 | context_only | slice D delivers field-edge healthy and mutant controls against QSL-1904 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | Leftover from FND-001. `the_followed_field_changes_the_harness_identity_and_equal_inputs_regenerate_it` is still bound to FR-015-AC-100, and on the committed lock it still asserts the refusal for `next` and returns. The new seam tests check `Edge.field` but not `GraphEdgeIdentity`, so the field-edge part of AC-100 (the followed field changes the identity) has no executed evidence here. Scenario: dropping `field: end.field` from the identity in `lower_graph` merges green. Fix: state AC-100's field-edge part as PARTIAL on QSL-1904 in the spec; or drop the AC-100 tag until the link lands. | tests/it/kani_obligations_symbolic_graphs.rs:431-440; src/kani/generate/graph.rs:289-292 |

## Dispositions

Round 1 (head 91b274b2).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AC-98 and AC-105 are now bound to `the_member_type_decides_the_field_edge_domain`, which runs the member-type mapping on the committed lock (M1 and M4 die). The AC-100 leftover is tracked as FND-004. |
| FND-002 | fixed | `field_edge_refusals_are_typed_and_name_their_input` asserts `RequiresBound(declaration)`, `FieldNotAdmitted`, `FieldEdgeNotReference` (Boolean, Option of Boolean, Collection of Boolean) and `PopulationTypeMismatch` (a target of another type, and a population of another type). |
| FND-003 | fixed | The local `harness()` asserts the population link with the message "this control needs QSL to link reaches_field to its population (QSL-1904)", and AC-106 says the controls stop at generation without the link. |
