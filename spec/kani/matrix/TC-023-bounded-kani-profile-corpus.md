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

Verify the current bounded-profile classifier, graph and collection finite-input
lowerers and corpus emitter against FR-015-AC-82 through
FR-015-AC-95, FR-015-AC-123/AC-124, and the retirement slices FR-015-AC-109 to FR-015-AC-111. Before AD-004 step 4g, the hand-built path's generated
`KaniOutcome` is a case classification; after step 4g, generation returns no
`KaniOutcome`. Only FR-017's installed-backend run provides proof evidence.
Each successful case retains a QSL source unit and its imported dependency
source. The real-Kani lane passes the run's actual falsification playback
immediately to the replay facade; playback is run evidence, not a persisted
case artifact. Current
source tests carry `TC-023` trace tags. FR-015-AC-94 retains graph/collection canonical-identity controls.
FR-015-AC-109 is traced by the admitted division refusal and routed remainder
refusal tests. FR-015-AC-95 and FR-015-AC-110 to FR-015-AC-111 remain planned.
A trace tag alone does not establish installed-backend proof evidence.

## Test Procedure

IR-621 direct-error procedure under FR-015-AC-123: from an external consumer
test, explicitly assign the collection operation's result to
`Result<CollectionLowering, KaniOutcome>` until FR-015-AC-110; assign the graph
and corpus operations' results to `Result<GraphLowering, KaniOutcome>` and
`Result<BoundedCorpusCase, KaniOutcome>` until FR-015-AC-111. Exercise each retained
family's actual profile/dispatch refusal, graph malformed-input and expansion-bound
refusals, collection bound exhaustion, and the corpus identity/dependency/serialization
failure controls. Assert the original exact kind and typed code, source identity,
selected profile/bound context, `boolean_claim() == None`, and absence of partial
artifacts or premature identity claims. Keep successful family outputs and corpus
artifacts/identities unchanged while each family remains. After FR-015-AC-110,
inspect that the collection operation and its public request/result types have no
exported replacement alias or wrapper; after FR-015-AC-111, apply the same check
to graph and corpus. Inspect that retained consumer paths call the matching
infallible IR constructor and that the retired public error names have no exported
type, alias or replacement wrapper. Replace the manufactured Boolean-kind
`OutcomeConstruction` wrapper test with these reachable public consumer controls;
IR's broad-constructor negative controls remain owned by IR. Preserve the IR-615
ordered profile census, strict-subset/late-entry and legacy-disposition rejection
controls. This procedure does not include the terminal cancellation adapter or
AD-004's independently gated generator retirement and invokes no backend merely
to establish the public API/refusal change.

Request supported and refused constructs from one selected profile in a
reordered subset; compare the returned modules and cause codes with the selected
entries. Submit empty, duplicate and unknown names. For the IR-615
cleanup, at Contract IR's owning
profile reader, submit the removed `inconclusive` disposition with a cause code;
require rejection before a profile can be classified or lowered. Inspect the
retained profile classifier and lowerers to confirm that no profile disposition
or profile-supplied code constructs an `Inconclusive` run outcome. This check
does not replace installed-backend proof-vacuity classification.
For each retained CG-owned semantic family, exercise the admitted boundary and each
typed refusal of FR-015-AC-86 through FR-015-AC-89. Generate the oracle,
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

Retirement procedure (IR-598), per slice, only after IR-858 (step 4b) has landed. Arithmetic (FR-015-AC-109): require a div claim to settle `Unsupported(OperationNotRendered)` with no harness and a rem claim to stay a refused identity; search `src/` and `tests/` for each deleted arithmetic name, and confirm the corpus replay is still present; confirm the AC-84/85 divide and remainder loss is recorded as a Rust checked-arithmetic literal check and points to IR-854. Collection (FR-015-AC-110): require step 4h landed and a recorded `make kani` or `make kani-gate` run on the slice PR's head with the AC-99 controls green, cited in the PR, then search for the deleted collection names. Graph and final (FR-015-AC-111): require IR-805's graph claims backed (CG integration of the merged IR-851/IR-852 accessors and QSL-820 real-package population emission, after QSL-819) and the same recorded run with FR-015-AC-106's real-Kani graph replay green, then search for every deleted name, the `BoundedCorpus*` types, the corpus replay and `generate_bounded_kani_corpus_case`, and confirm no generation path returns a `KaniOutcome`. For each slice, inspect that no `interface-001` operation was added and no `src/` item converts a finite-input request type into a V2 generator call.
Inspect the resolved Cargo graph for a reverse Contract IR-to-codegen dependency.

Replay procedure (IR-453; live until the final slice FR-015-AC-111 drops corpus replay, which is not re-homed; the real-Kani replays of FR-015-AC-99 and FR-015-AC-106 replace it): for each falsifiable family, compile the case's
retained QSL unit and locked dependency through `call_site`, run the generated
harness under Kani with concrete playback enabled, then pass the actual
`Falsified` playback and the case's typed harness/site identity through the
Witness replay arm. Require a typed reproduced violation. A mismatched harness
or QSL package/site is evidence failure. The corpus does not model state/frame
operations; their source-backed replay remains covered by the separate
state/frame facade tests, not by a corpus case.

## Expected Results

Every successful corpus case has the oracle, strategy, Kani harness, QSL unit,
locked dependency source and proof graph derived from its admitted request.
Focused source tests compile each family through QSL's call-site API. The
real-Kani false graph and collection cases replay their actual playback against
the selected source package and settle as reproduced typed violations;
wrong-harness and wrong-package evidence is refused. This does not claim
state/frame corpus coverage. Every profile request has one ordered disposition per construct, or one typed
request refusal. The census preserves each admitted `Supported` module and
`Refused` cause code exactly. The removed `inconclusive` profile disposition is
rejected by the owning reader, with no admitted profile or manufactured run
outcome. Full `unsupported` matrix entries, capability-kind warnings and
no-artifact negotiation semantics remain planned under Contract IR FR-029-AC-2;
this cleanup does not establish that coverage. The IR-615 producer and consumer
cleanup is implemented: native reader/census and both retained family preparation
controls pass; the actual run classifier retains its zero-SUCCESS vacuity rule.
This classification check includes a recorded real-Kani report, without invoking
a backend as part of the profile cleanup. Each successful
lowering returns the exact family result and
each adverse case its original typed non-Boolean refusal, with no partial
artifact or proof claim. One admitted corpus case has all generated artifacts and
one generated case classification until AD-004 step 4g retires that path;
after step 4g, the symbolic V2 cases render through the shared path and the
artifacts have no generation-time `KaniOutcome` or proof verdict. A refused
case has no artifacts and claims no identity. The dependency graph keeps Contract
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
(FR-015-AC-51). The context of every successful expectation and typed refusal the corpus
returns (assertion-expected-to-pass, assertion-expected-to-fail, dependency-invalid, identity-collision,
a lowering refusal and the mismatch refusal) is the revision of the profile
selection while the hand-built path remains (FR-015-AC-52). The expectation
retains one or zero predicted SUCCESS checks for its one assertion and cannot
enter the public terminal mapper as a run-authenticated `KaniOutcome`
(FR-015-AC-91).

Implemented (IR-464; arithmetic corpus retired by IR-598 AC-109): the harness
every retained graph and collection corpus case emits, ends with exactly one `kani::cover!` after its assertion of the
case's oracle, and the installed backend classifies a case of each family whose oracle is true
`Verified`, and a graph or collection case whose oracle is false
`Falsified` with the assertion's
playback, which its cover after the assertion keeps from being the one Kani prints
(FR-015-AC-55, FR-015-AC-57). `tc_023_kani_falsifies_the_generated_false_collection_harness`
and `tc_023_kani_verifies_a_true_collection_and_falsifies_a_false_graph_harness` run the false
cases through `classify_kani_run`, and the graph and true collection cases are
classified `Verified` the same way. A corpus case has no symbolic input and no
precondition, so no vacuous corpus run exists to classify.


Arithmetic retirement status (IR-598, FR-015-AC-109 only): the finite-request
arithmetic API, corpus arm and renderer are deleted. Shared canonical-preimage,
dependency, refusal, collision, emission-order, ceiling and exact wide `i128`
checks use retained collection requests. Graph/collection source-backed replay,
cover controls, schemas, census validation and V1 generation remain. The admitted
`quire.op.integer.div` test requires `Unsupported(OperationNotRendered)` with no
harness; the separate `quire.op.integer.rem` test requires an `Underived`, refused
claim with no generated scalar family or harness.

Coverage retired from FR-015-AC-84/85 includes finite-input `i128` checked
add/subtract/multiply/divide/remainder admission and range/refusal behavior,
including requests outside `i64`. Divide/remainder had only ground Rust
`checked_div`/`checked_rem` checks for exact result, nonzero divisor and range,
with no mutation control. Their retained QSL text used `/` and `%`; this slice
claims no QSL division/modulo proof and passes none to IR-854. V2 supports symbolic
`i64` add/subtract/multiply/negate with independent `i128` assertions; it does not
replace the old finite-request API. IR-858's differential reference renderer,
authentic QSL-emitted AC-103 inputs, real healthy Add/Add-to-Subtract mutant and
routed healthy/bound-violation controls remain; this does not claim dedicated
real Subtract/Multiply/Negate mutants.

IR-598 and IR-348 remain open. Collection retirement (AC-110) waits for released
step 4h and actual AC-99 controls. Graph/final retirement (AC-111) also waits for
AC-110 and IR-805/QSL-820. Held steps 4c–4f, 5 and 6 and the separately gated
IR-854 division/modulo replacement remain unchanged.

IR-621 direct-error adoption is implemented. The external consumer tests assign
all three public results to their exact `Result<Success, KaniOutcome>` types and
exercise supported results, profile/dispatch precedence, malformed graph inputs,
collection/graph bounds, corpus dependency and profile mismatch ordering,
preimage refusal, collision, and a same-request retry after dispatch failure.
They retain the exact non-success kind, typed code, source and profile context.
The original identity and no-partial-artifact controls remain, and the real
artifact renderer retains its over-ceiling serialization refusal control.
That serializer control is a private rendering-boundary test, not an observed
public-generator typed serialization failure: the public generator's earlier
bounded preimage check limits which dependency graphs can reach rendering.
The public serialization-failure branch still uses the exact infallible
`refused` constructor and original cause/source/context.
Focused native family/corpus tests and doctests pass; the eight separately named
Kani-lane integration tests remain ignored in that run. This adoption claims no
new backend proof qualification. Existing family/corpus behavior and the separate
AC-111 retirement prerequisites remain in force.
