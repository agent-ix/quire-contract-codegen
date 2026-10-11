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

Verify the current bounded-profile classifier, three CG-owned finite-input
lowerers and four-artifact corpus emitter against FR-015-AC-82 through
FR-015-AC-95. Before AD-004 step 4g, the hand-built path's generated
`KaniOutcome` is a case classification; after step 4g, generation returns no
`KaniOutcome`. Only FR-017's installed-backend run provides proof evidence.
The corpus retains no
counterexample packet, so native replay remains planned and unbacked. Current
source tests carry `TC-023` trace tags, but none binds FR-015-AC-82 through
FR-015-AC-95 yet; the computed matrix reports those criteria untagged.

## Test Procedure

Request supported, refused and inconclusive constructs from one selected
profile in a reordered subset; submit empty, duplicate and unknown names.
For each CG-owned semantic family, exercise the admitted boundary and each
typed refusal of FR-015-AC-84 through FR-015-AC-89. Generate the oracle,
finite strategy, Kani harness and proof graph from one validated Contract IR
finite input; compare their selected family, profile, case identity and oracle
truth value. Vary one case input at a time, reorder graph objects and edges,
repeat emission through one registry, and submit invalid dependency censuses.
Run the installed backend for FR-015-AC-57's healthy and falsifying cases.
At AD-004 step 4g, first verify that authentic QSL-emitted packages admit through
Contract IR and that the one V2 generator serves arithmetic, collection and
graph cases; collection and graph must meet FR-015-AC-96 to AC-102 with
model-derived symbolic bounds. Check that the shared renderer places the cover
after the assertions and that a case without symbolic input emits no proof
harness. Then check that neither successful nor refused generation returns a
`KaniOutcome`, and classify healthy and seeded-mutant cases only from installed
FR-017 backend runs (FR-015-AC-95; planned). An absent typed graph population
input leaves this replacement incomplete rather than substituting a caller
expansion ceiling.
Inspect the resolved Cargo graph for a reverse Contract IR-to-codegen dependency.

Planned replay procedure: once a corpus case retains a real QSL source and
counterexample packet, submit its counterexample through QSL's replay facade
and compare the typed result. No current test or corpus artifact establishes
that result.

## Expected Results

Every profile request has one ordered disposition per construct, or one typed
request refusal. Each successful lowering returns the exact family result and
each adverse case its original typed non-Boolean refusal, with no partial
artifact or proof claim. One admitted corpus case has all four artifacts and
one generated case classification until AD-004 step 4g retires that path;
after step 4g, the symbolic V2 cases render through the shared path and the
artifacts have no generation-time `KaniOutcome` or proof verdict. A refused
case has no artifacts and claims no identity. Native replay
remains unbacked. The dependency graph keeps Contract
IR below codegen. Every
finite-graph case expands identities in sorted depth-first order: a branch explored before the
target branch consumes its expansion budget, while a target on an edge of the current identity
is found without expanding the target. The generated graph oracle uses the same order and bound.
Every generated `#[kani::proof]` symbol carries its corpus case's name: the family label
plus the SHA-256 of the case's canonical content (run ceilings, construct, every request field, the finite
input with its objects and references in sorted order, the profile selection and the
normalized dependency census), the same name its artifact paths carry. The preimage
uses RFC 8785 canonical bytes without a trailing newline for the digest, bounded to 1 MiB;
signed `i128` request values are decimal strings in that preimage so no finite value is rounded.
The proof graph is RFC 8785 canonical JSON followed by one newline, bounded to 16 MiB including
the newline; an oversized graph refuses before its JSON tree is materialized. An unencodable
preimage (including one over the 1 MiB ceiling) refuses as `InvalidInput`
`kani_corpus_identity_unencodable` before lowering or claiming a registry identity, and an unencodable graph refuses as `Refused`
`kani_corpus_serialization_failed` without artifacts or a registry claim. The same request
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
selection while the hand-built path remains (FR-015-AC-52); its `Proved` and
`Counterexample` values classify generated cases rather than Kani runs.

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
