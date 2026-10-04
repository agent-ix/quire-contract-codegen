---
id: TC-023
title: "Verify bounded Kani profile corpus parity"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: verifies
---
# TC-023: Verify bounded Kani profile corpus parity

## Description

Verify that the complete selected bounded-Kani corpus preserves one profile
disposition and typed outcome across native execution, generated oracle,
strategy, Kani harness and proof graph for arithmetic/definedness, graph, and
collection cases. Replay of a retained counterexample is not part of what the
current tests back: the corpus retains no counterexample packet, and the replay
clauses below are unbacked until each corpus case carries a real QSL source
replayed through QSL's replay facade.

## Test Procedure

For each semantic family, execute canonical valid boundary cases and malformed,
incomplete, unavailable, refused, inconclusive, exhausted, and counterexample
cases. Generate all codegen artifacts from the same validated Contract IR
selection, run the generated oracle/strategy/Kani cases where the profile is
supported, compare classifications with native execution, and replay every
retained counterexample through the public QSL replay boundary.
Inspect the resolved Cargo graph and generated manifests for a reverse Contract
IR-to-codegen dependency.

## Expected Results

Every supported case has matching typed classification across backends. Every unsupported or adverse case has its original
typed non-Boolean result and no partial artifact or proof claim. Every retained
counterexample either reproduces native false or reports a typed replay
non-success (unbacked). The dependency graph keeps Contract IR below codegen. Every
generated `#[kani::proof]` symbol carries its corpus case's name: the family label
plus the SHA-256 of the case's canonical content (construct, every request field, the finite
input with its objects and references in sorted order, the profile selection and the
normalized dependency census), the same name its artifact paths carry. The same request
names the same artifacts in any emission order and any run, and the same graph offered in
another object or reference order names the same ones. Varying any single one of those
fields names different artifacts. A request emitted twice through one registry is refused
as `kani_corpus_identity_collision` and emits nothing. A
declared proof-dependency census that is empty or duplicate-identity,
kind/state/path-inconsistent, or names any non-`Required` kind is refused
with a typed `InvalidInput` `kani_corpus_dependency_invalid` result and
leaves no artifact. A finite input validated under a different profile
selection than the profile offered with it is refused with a typed
`InvalidInput` `kani_profile_input_mismatch` result naming the request's
source id, leaves no artifact and records no case identity in the registry
(FR-015-AC-51). The context of every typed outcome and refusal the corpus
returns (proved, counterexample, dependency-invalid, identity-collision,
a lowering refusal and the mismatch refusal) is the revision of the profile
selection (FR-015-AC-52).

Implemented (IR-464): the harness every corpus case emits, of each of the arithmetic, graph
and collection families, ends with exactly one `kani::cover!` after its assertion of the
case's oracle, and the installed backend classifies a case of each family whose oracle is true
`Verified`, and a graph or collection case whose oracle is false (an arithmetic case's
oracle is always true) `Falsified` with the assertion's empty-valued
playback, which its cover after the assertion keeps from being the one Kani prints
(FR-015-AC-55, FR-015-AC-57). `tc_023_kani_falsifies_the_generated_false_collection_harness`
and `tc_023_kani_verifies_a_true_collection_and_falsifies_a_false_graph_harness` run the false
cases through `classify_kani_run`, and the arithmetic, graph and true collection cases are
classified `Verified` the same way. A corpus case has no symbolic input and no
precondition, so no vacuous corpus run exists to classify.
