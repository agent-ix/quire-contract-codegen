---
id: SR-17940486709564847047979
title: "IR-805 PR #832 code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #832 at ec0646b1, git diff origin/main...HEAD; IR-805 slice D; src/kani/generate/graph.rs, src/kani/generate/outcome.rs, src/kani/identity.rs, src/replay/state_clause/graph.rs, src/lib.rs, tests/it/kani_graph_field_private.rs, tests/it/kani_graph_private.rs, tests/it/kani_obligations_symbolic_graphs.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
---

## Summary

Ticket: IR-805 slice D. This is a code review of PR #832 at ec0646b1, with the rust-review lane
folded in. I read the whole diff and ran tests on the committed lock (IR c8dca8c6, QSL 434869e8).
Every cargo run used my own target dir and ran under `flock /tmp/cargo-slot.lock`.

**On the committed lock no field-edge code runs.** IR c8dca8c6 `reaches_application` accepts only
`quire.op.model.reaches` (`relationship_end.rs:71`). So `reaches_population` returns
`NotReachesApplication` for every `reaches_field` node, and `domain_of` exits before
`field_member` or `field_edge` runs.

Test results at the head:

- **Focused suites pass:** `--lib kani_graph` gave 12 passed and 28 ignored;
  `--test it kani_obligations_symbolic_graphs` gave 23 passed and 4 ignored.
- **The ignored control fails as expected:**
  `real_kani_negated_field_claim_falsifies_and_qsl_replay_reproduces_it` panics at
  `kani_graph_private.rs:225` (harness refused) before Kani starts. The other five ignored controls
  call the same `harness()` and must fail the same way.

Mutation check, in a scratch copy of the head (the branch was not touched):

| Mutant | Change | Suites run | Result |
| --- | --- | --- | --- |
| M1 | `Option<Reference>` edge lower bound 0 changed to 1 (`graph.rs`, Option arm) | full `--lib` and full `--test it` | **SURVIVED.** it: 1035 passed. lib: 569 passed; the one failure is `production_refinement_live_solver_handoff_reaches_qsl_checker`, which needs `QUIRE_CVC5_BIN` (environment, unrelated) |
| M3 | `#[serde(skip_serializing_if)]` removed from `GraphEdgeIdentity.field` | same run as M1 | **SURVIVED.** This serializes `"field":null` into every relationship-end identity and changes every relationship-end harness symbol/digest |
| M2 | replay: a followed `Option` with no edge rendered as `present(n0)` | `--lib kani_graph_field` | **KILLED** by `qsl_replay_judges_a_concrete_field_graph_by_the_authored_clause` |

## Verdict

**FAIL: one high finding.**

What is sound:

- **The lowering reads the right inputs:**
  - the population comes from `reaches_population` + `select_population`;
  - the edge comes from `model_object_fields(declaration).field(name).member_type()`.
- **Member types map correctly:**
  - Reference is 1..1;
  - `Option<Reference>` is 0..1;
  - a bounded Collection takes its bounds;
  - Set/OrderedSet are unique and Sequence/OrderedSet ordered;
  - an unbounded Collection gives `upper: None`, which becomes `RequiresBound(declaration)` in
    `lower_graph`.
- **Type checks:** a declaring type or target that is not the population's object type gives
  `PopulationTypeMismatch`.
- **`Shape` compares `field`,** so a condition over two fields is refused.
- **The replay snapshot uses the right spellings.** It carries `reference`, `present`/`absent` and
  `sequence`/`set`/`bag`/`ordered-set`, which match QSL's snapshot decoder (`qsl-replay`
  `spine/request.rs:587`, `witness/value.rs:493-496`).
- **The replay seam is exercised:** the non-ignored test covers the Reference, Option and Sequence
  forms on the committed lock.

What fails: that test is the only field-edge test that runs real code on this lock. The lowering is
untested, and the relationship-end identity non-change is untested.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | On the committed lock the whole field-edge lowering has zero executed tests: `field_member`, `field_edge`, the member-type multiplicity mapping and every new typed refusal. `field_edge_domains_use_the_population_and_the_member_type_multiplicity` (AC-98/105) and `the_followed_field_changes_the_harness_identity_and_equal_inputs_regenerate_it` (AC-100) check `links_population`, which is false here, assert the IR's `NotReachesApplication`, and `return`. They pass vacuously as to CG behaviour. Mutant M1 (Option lower bound 0→1) survives the full lib and it suites. Scenario: a regression in the Option/Collection bounds, the unique/ordered flags or the refusal mapping merges green and only shows up after QSL-1904 lands and the lock bumps. The claim that `FieldNotAdmitted`, `FieldEdgeNotReference`, `PopulationTypeMismatch` and the unbounded `requires_bound` are "reachable only through a package QSL does not emit, so they have no test" mistakes package reachability for testability. `field_edge` is a pure function of (package, population, declaration, name). A `#[cfg(test)]` seam, as slice C's `render_with_bounds` already does, can call it on the QSL-emitted fixture: take the population from a sibling relationship-end application via `reaches_population`/`select_population`, and the declaration and name from the `reaches_field` node's `CheckedOperationMember::Field`. Fix: add that committed-lock test. It should cover Reference/Option/Sequence and at least one Set or OrderedSet, an unbounded Collection (`upper` None), a scalar field (`FieldEdgeNotReference`), a missing name (`FieldNotAdmitted`) and a mismatched population (`PopulationTypeMismatch`). | src/kani/generate/graph.rs:326-329,334-349,385-439; tests/it/kani_obligations_symbolic_graphs.rs:349-372,379-389,426-431 |
| FND-002 | medium | The dual-branch gate makes tests change behaviour silently. `emitted_field` accepts either `PopulationUnavailable` or `NotReachesApplication` whenever `links_population` is false, and the AC-107 two-field and separate-items assertions skip on the same test. Scenario: after the lock carries QSL-1904, a regression that drops or breaks the population link (QSL stops emitting it, or the IR accessor refuses again) sends every field-edge test back to the refusal branch, and they stay green. The suite cannot tell "not linked yet" from "link broke". Fix: assert the side the committed lock is on (here exactly `NotReachesApplication`). Make the switch explicit: a named constant, or a test that fails when `links_population` becomes true, forcing the flip in the lock-bump PR. | tests/it/kani_obligations_symbolic_graphs.rs:349-372,463-468,916-926 |
| FND-003 | medium | The PR and the identity doc say the field is "omitted for relationship ends so their identities are unchanged", but no test pins it. Mutant M3 (always serialize `field`, giving `"field":null`) changes every relationship-end harness identity and digest, and it survives the full suites. Scenario: a later serde change re-keys every relationship-end harness and invalidates recorded evidence, and nothing fails. Fix: add a golden check that a relationship-end `GraphEdgeIdentity` serializes without a `field` key, or pin a relationship-end harness symbol/digest. | src/kani/identity.rs:737-740 |
| FND-004 | low | `qsl_refuses_a_scalar_collection_field_edge_before_a_package_exists` asserts only `!error.is_empty()`. Any refusal passes it: a source typo, a renamed fixture field, an unrelated admission failure. So it does not show that QSL refuses the scalar field edge, which the AC-108 prose relies on. Fix: assert the error names the field-edge/reachability type refusal. | tests/it/kani_obligations_symbolic_graphs.rs:477-491 |
| FND-005 | low | `field_member` returns `None` when a `quire.op.model.reaches_field` application's `operation.read()` fails or its member is not `Field`. `domain_of` then falls back to `relationship_edge`, so a malformed field-edge application is refused with a relationship-end accessor reason, not `FieldNotAdmitted`. Scenario: the diagnostic misnames the cause for a field-edge claim. Fix: once the identity is `reaches_field`, map a read failure or a non-field member to `FieldNotAdmitted`. | src/kani/generate/graph.rs:334-349 |
| FND-006 | low | Stale or misleading text and errors. The `GraphEdgeIdentity.declaration` doc still reads "Relationship declaration that owns the selected end", but for a field edge it is the object type. The field-edge snapshot path maps `serde_json::to_value` failures of the identity and the model to `GraphWitnessNotInDomain`, which is not a witness-domain fault. Fix: update the doc, and map those to the document-encode error. | src/kani/identity.rs:735; src/replay/state_clause/graph.rs:507,511 |

## Examined scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| domain_of / field_member / field_edge | examined | `let edge = match field_member(package, application) { Some((declaration, name)) => field_edge(...)?, None => relationship_edge(...)? };` |
| relationship_edge | examined | unchanged end checks; `Edge { declaration: end.relationship, field: None, ... }` |
| Shape::same edge check | examined | `|| end.field != self.end.field` |
| GraphUnsupportedReason FieldNotAdmitted / FieldEdgeNotReference | examined | new typed reasons |
| GraphEdgeIdentity.field | examined | `#[serde(skip_serializing_if = "Option::is_none")] pub field: Option<Box<str>>` |
| replay field_values / snapshot branch | examined | followed field holds drawn targets; other reference fields neutral; non-reference field refused |
| tests/it/kani_graph_field_private.rs | examined | 1 native replay test (runs) + 6 `#[ignore]` installed-Kani controls (panic at `harness()` on the committed lock) |
| tests/it/kani_obligations_symbolic_graphs.rs field-edge tests | examined | `emitted_field` / `links_population` dual-branch gate |
| IR c8dca8c6 reaches_application | context_only | `(operation.identity() == Some("quire.op.model.reaches")).then_some(operation)` |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | The new `field_member` refusal is untested. A `reaches_field` member that cannot be read as a field now returns `Err(FieldNotAdmitted)` instead of falling through to `relationship_edge`, but no test reaches that branch: `field_member` is private, and on the committed lock `domain_of` exits earlier. In round 1, mutant M5 (that arm reverted to `Ok(None)`, the old fall-through) survives both the focused lib and `it` suites. Scenario: a regression back to the fall-through, which gives a relationship-end accessor reason for a field-edge claim, merges green. Fix: make `field_member` reachable through the same seam and assert `FieldNotAdmitted` on an application whose member is not a `Field`; or accept it as covered by inspection. | src/kani/generate/graph.rs:340-356 |

## Dispositions

Round 1 (head 91b274b2, `git diff ec0646b1..91b274b2`). The mutants were run in a scratch copy of
the head with my own target dir under flock, against `--lib kani_graph` and
`--test it kani_obligations_symbolic_graphs`.

- **Baseline:** 14 + 23 tests pass.
- **Killed:**
  - M1 (Option lower bound 0→1), by `the_member_type_decides_the_field_edge_domain`;
  - M4 (OrderedSet not unique), by `the_member_type_decides_the_field_edge_domain`;
  - M3 (`field` always serialized), by `relationship_domains_use_the_population_and_the_typed_destination_multiplicity`;
  - M2 (empty Option replayed as present), by `qsl_replay_judges_a_concrete_field_graph_by_the_authored_clause`.
- **Survived:** M5 (FND-007).

I checked by reading that the generated harness text is unchanged. Rendering code is untouched, and
every successful `field_edge` returns the same `Edge` values as at ec0646b1. The changes only move
or add refusals: an unbounded Collection still yields `RequiresBound(declaration)`, now raised in
`field_edge` instead of `lower_graph`. So the earlier local-composition Kani verdicts are not
invalidated by this round. I could not confirm it empirically, because the committed lock cannot
generate a field-edge harness.

Relationship-end identity bytes are unchanged: `relationship_edge` is untouched, and the new
no-`field`-key assertion kills M3.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | `field_edge` is `pub(crate)` and takes the object type. `the_member_type_decides_the_field_edge_domain` runs the mapping on the committed lock for Reference, Option, Sequence, Set, OrderedSet and Bag. M1 and M4 die. |
| FND-002 | fixed | `emitted_field` and the separate-items test now pin exactly `NotReachesApplication`, so a change of reason fails. |
| FND-003 | fixed | `relationship_domains_use_the_population_and_the_typed_destination_multiplicity` asserts that a relationship-end identity has no `field` key. M3 dies. |
| FND-004 | fixed | The test asserts `IllTyped` and `operator-ineligible`. |
| FND-005 | fixed | `field_member` returns `Err(FieldNotAdmitted)` for an unreadable or non-field `reaches_field` member. The new branch is untested; see FND-007. |
| FND-006 | fixed | The doc is updated. The snapshot identity and model encode failures map to `Document(Encode)` via `preimage_value`. |
| FND-007 | fixed | Round 2 (head 606e3221). The branch is split into the pure `pub(crate) fn field_of(identity, member)` (graph.rs:362). `a_reaches_field_without_a_field_member_is_refused_not_read_as_another_form` asserts `FieldNotAdmitted` for a RelationshipEnd member and for no member, a field for a Field member, and `None` for `reaches`. Mutant M5 (that arm set to `Ok(None)`) is now killed by it. `Edge::identity(upper)` copies every field one to one into `GraphEdgeIdentity`, exactly as the inline literal it replaces did, so harness text and successful `Edge` values are unchanged. The earlier local-composition verdicts stand (checked by reading). Relationship-end bytes are unchanged; M3's guard still passes. Mutant M6 (`field: None` in `identity`) is killed by `the_followed_field_is_part_of_the_edge_identity`. |
