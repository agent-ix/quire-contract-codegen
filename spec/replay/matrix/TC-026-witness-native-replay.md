---
id: TC-026
title: "Verify bounded witness decoding and native replay"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-219
    type: references
---
# TC-026: Verify bounded witness decoding and native replay

## Description

Verify that retained counterexamples are decoded, domain-checked and replayed
natively before any failure is reported.

## Test Procedure

Read a harness's persisted obligation arguments as the decode schema, including
one non-argument binding, and compare the schema's order with the harness's
emitted symbolic arguments. Decode a real falsifying transcript
against the persisted schema, then against that schema with a binding dropped
and with a binding retyped. Then replay a reproducing witness, a malformed
witness, a witness bound to another harness identity, and witnesses at and one over
each caller-supplied byte and value-count bound. Supply both required bounds through
the replay inputs and directly to every public witness-decoding entry point. For each
over-bound control, place a poisoned byte or value just past the bound that would cause
a distinct decode error if read. Verify by inspection that neither bound can be omitted
or defaulted at any decoding entry point. Also replay an out-of-domain witness,
a witness whose native value disagrees with the harness, a witness whose
native replay returns `ReplayRefusal::Fault`, and a witness whose replay agrees
in a category other than `violation`. Inspect the replay error for the fault
(FR-016-AC-12).

For the installed-Kani spine, observe the launcher's complete stdout independently of
`KaniRunOutcome::Falsified.counterexample` before the execution adapter classifies the run.
After a healthy subject verifies and an injected violation falsifies, scan the observed stdout
in print order. Select the first fenced playback for that same harness whose test invokes
`kani::concrete_playback_run` and is not a cover playback. Exclude blocks headed for other
harnesses and satisfied-cover blocks; do not choose a later assertion block if more than one
eligible block is printed for this harness. Use neither the production playback extractor nor
the witness decoder for this selection. Trim only surrounding whitespace from the complete text
between the fences, then compare its bytes with the retained counterexample before decoding or
replay. Replace the returned counterexample at the classification boundary with another
well-formed, self-consistent playback as a mutation control: the comparison must fail even though
that replacement's comments, byte vectors,
decoder and replay all agree with each other. Continue the existing point and neighbouring-point
replay checks against the original process output. This exercises FR-017-AC-5 together with
FR-016-AC-9 and AC-10. The comparison is the test oracle over independently observed stdout;
production replay receives only the retained counterexample and performs no second stream check.
In a separate selection control, put a cover block, a block headed for another harness and two
eligible assertion blocks for the target harness in one stdout stream. The oracle selects the first
eligible target block in print order, then refuses the second target block as a substituted
counterexample.

For FR-016-AC-21 to AC-23, build the replay request for a function with two parameters declared
in an order that differs from their ascending identifier order. Recompute the O-09 digest from
the `FunctionSite`'s `function` and `declaration`, the requested kind and the arguments ascending
by identifier, through `quire-canonical`, and compare it with the request's `obligation_identity`
slot (AC-21). Then build the request for a second function with the same parameters, recompile
the first with comments and blank lines inserted, change its harness source span, replay it under
a different `ObligationKind`, and change one argument domain, comparing slots each time. Replay a
function whose body has two conjuncts under one kind and count its identities (AC-22). Rebuild the
identity with every other node of the compiled package perturbed and the `FunctionSite` members
held fixed, then with `function` and then `declaration` perturbed (AC-23). Build a replay
package from a lock that selects one library identity twice and has no other defect (AC-24).

## Expected Results

The schema follows the persisted argument order, which is the emission
order the harness uses, and refuses the non-argument binding with a typed
schema refusal (code `cg_witness_schema_non_argument_binding`) that
reports no failure and is none of the replay results.
The real transcript decodes to values named by their bindings, and the dropped
and retyped schemas refuse by arity and width. Only the reproducing witness is
reported as a failure. The malformed and wrong-identity witnesses yield malformed
results; each one-over-bound witness yields the over-bound malformed cause, not the
poisoned-suffix decode error. The at-bound witnesses decode, and both required bounds
are present at every decoding entry point without a default. The remaining witnesses
yield out-of-domain and mismatch results. The witness that agrees in a category other
than `violation` is a mismatch and never a reproduced failure. The executor fault
remains a typed `SpineReplayError::Refused(ReplayRefusal::Fault)`; FR-029-AC-10
owns its terminal `Failed` reading.
The retained falsification is byte-identical to the independently selected assertion
playback in that run's stdout. A valid replacement after classification fails the comparison
before replay; healthy verification, the original witness-point replay and its neighbour
control still pass. In the multi-block selection control, the cover, other-harness and later
same-harness assertion blocks cannot replace the first eligible target assertion block.
The slot equals the recomputed digest (AC-21). The two functions' identities differ, comments and
blank lines and the span change leave one unchanged, the kind change and the domain change each
alter it, and the two-conjunct function has one identity (AC-22). The identity is unchanged by the
other-node perturbation and changed by each `FunctionSite` perturbation (AC-23).
A lock whose only defect is a repeated dependency identity is refused with
`DependencyLockError::Input`, code `invalid_package`, cause `conflicting-definition`, and the
error has no `Duplicate` variant (AC-24, planned).

## Status

FR-016-AC-24 is planned (IR-465): the test that exercises it,
`tc_026_a_lock_repeating_a_dependency_is_refused`, today expects the `Duplicate` error and changes
with the code, which deletes the builder's own pre-check and the `Duplicate` variant so QSL's
constructor refuses. Because QSL checks libraries in supply order (for each, an empty identity,
then a repeated identity, then a shared source owner), a lock with several defects reports
whichever QSL meets first; AC-24 is therefore limited to a lock whose only defect is the repeated
identity.

FR-016-AC-21 to AC-23 are implemented and tested in the default suite
(`tests/it/skeleton_spine.rs` with a direct test-only identity fixture, and unit tests in `src/replay/obligation.rs` and
`src/replay/function.rs`): `ReplayPackage::obligation_identity` builds the O-09 function-contract
digest in `core::canonical` over the `FunctionSite`'s `function` and `declaration`, the harness's
kind and its arguments ascending by identifier, and `replay_counterexample` puts it in the
request's `obligation_identity` slot. The slot equals a digest recomputed in the test from
hand-written RFC 8785 text, with a Boolean argument and an identifier order that differs from the
node-id order, and the request `replay_counterexample` sends is captured at the executor seam
(`replay_counterexample_through`) and carries it; a harness that is not exactly the function's
bound parameters is refused (AC-21). The identity separates functions, kinds and domains and
ignores comments, blank lines and the harness's source span (AC-22), and it ignores an unrelated
declaration and the order of `FunctionSite.parameters` while a changed `function` or
`declaration` changes it (AC-23).

Partial. FR-016-AC-9 through AC-11 are implemented and tested in the default suite
(`tests/it/skeleton_spine.rs`): a falsifying input replays through
`qsl_replay::replay` and settles the violation, a witness at which the function
holds and a healthy twin settle `inconclusive` naming both verdicts, and each
adapter refusal is its own typed error. Its former ignored Kani lane used the retired V1
`BoundClause` generator and no longer supplies installed-prover evidence. IR-1047 owns the
genuine V2 generated harness and real-Kani counterexample control for AC-9 and AC-10.

The verdict is tied to the exact witness values, not to any violating input. A twin whose clause
is false at a single point is reproduced only by the transcript carrying that point, and
neighbouring points settle `inconclusive` (default suite,
`tc_026_the_replay_verdict_is_decided_by_the_exact_witness_values`). In the retired V1 Kani control the point was
read from Kani's own printed block, by a reader that shares no code with `decode_falsification`
(the `// value` comment lines and, separately, the little-endian byte vectors, which must agree
with each other and with the decoder), and a neighbour control must not reproduce. On a throwaway
copy each of these turned the Kani lane red (IR-29): replacing the transcript handed to the replay
with a constant after the printed witness is read, a decoder that adds one to each value, one that
swaps the two values, and one that returns a fixed in-domain pair. The witness check pins the
replay to the transcript it is given. A constant substituted at the source, right after the run is
classified `Falsified` and before anything reads it, stays green, because the independent reader,
the decoder and the replay then all see the same self-consistent block. The link from the Kani run
to that transcript rests on the `Verified` then `Falsified` outcome assertions. The independent
process-stdout test oracle described above is planned under IR-604; its mutation control must
turn the lane red for a replacement made immediately after `Falsified` classification.

What stays unbacked: the native twin is hand-mirrored QSL source, not derived from the contract or
the Rust subject, and the input package is hand-built, so the contract-to-Contract-IR step does not
run. The Kani lane is `#[ignore]` and outside `make ci`; NFR-006 instead requires `make kani-gate`
before opening and merging a Kani-touching PR. The default suite pins the replay end and the
decoder against a synthetic transcript only. FR-016-AC-6 is covered by tagged witness unit,
spine, frame, state-clause and bounded-corpus tests; the bounded-corpus test runs in the Kani lane.
Former FR-016-AC-7 is retired under IR-604 because Kani supplies no scalar-charge,
consumed-counter or replay-limit counterpart to QSL's accounting result.
FR-016-AC-12 is covered by
`tc_026_an_executor_fault_remains_a_typed_replay_refusal`. FR-029-AC-10 already
tests the terminal mapping for a fault wrapped in `SpineReplayError::Refused`.

FR-016-AC-8 remains a generic persisted argument order and positional naming requirement.
The `src/replay/witness.rs` unit tests continue to check schema order, positional naming and
non-argument refusal. The surviving V2
`state_clause_kani_playback_adapter_refuses_wrong_evidence` test checks a generated state-frame
postcondition's persisted draw order against playback and refuses mismatched assertion evidence;
it does not prove emitted symbolic argument order or a real-Kani decode. The
`tests/it/kani_argument_order.rs` emission-order test and the ignored
real-backend `tests/it/kani_witness_join.rs` test construct V1 `BoundClause` harnesses; their
evidence retires with that path under the 2026-10-27 IR-348 no-legacy ruling. AC-8 is therefore
partially backed and Open until a genuine V2 producer persists argument bindings and a matching
emitted harness is checked for emission order and real-Kani decode. The surviving decoder unit
tests and V2 playback adapter alone do not prove that producer-to-persisted-schema coupling.

FR-016-AC-25 stays Planned/Open under IR-604. The ignored
`tc_026_installed_kani_falsification_is_bound_to_process_stdout` test uses a V1 `BoundClause`
harness and retires with that producer. It genuinely compared the retained counterexample
byte-for-byte with the first eligible assertion playback in independently captured stdout,
excluding cover blocks, and rejected a valid substitute chosen after classification. The
observing-launcher helper and the other three AC-25-tagged tests in
`tests/it/kani_stdout_capture.rs` are independent of V1 and survive. IR-604 still owes this
stdout binding and substitution control on a surviving generated harness; no AC-25 completion
credit transfers from the retired V1 test.

FR-016-AC-1 through AC-5 and AC-13 are implemented and tested in the default suite
through `replay_counterexample`: a transcript that decodes to nothing, has the wrong
value count or byte width, or names another harness is a decode evidence failure
carrying the decoder's cause code and is never replayed (AC-1, AC-5); a value one
past either end of its argument's bounds, or at an `i64` extreme, is a domain
evidence failure (AC-2, `src/replay/witness.rs` unit tests); an in-domain
counterexample the native twin falsifies is reproduced (AC-3); a twin that holds
the clause, or any settlement other than a reproduced violation, is a verdict
evidence failure (AC-4, AC-13, `src/replay/function.rs` unit tests). Decode, domain
and verdict mismatches are one evidence-failure verdict with a typed cause. The
caller-supplied byte and value-count decode limits (AC-6) are covered by tagged tests in
`src/replay/witness.rs`, `tests/it/skeleton_spine.rs`,
`tests/it/kani_obligations_state_frame.rs`,
`tests/it/kani_obligations_state_clause_replay.rs` and `tests/it/bounded_kani_corpus.rs`.
Replay-layer propagation of an executor fault (AC-12) is covered by
`tc_026_an_executor_fault_remains_a_typed_replay_refusal`; the terminal mapping is
covered by FR-029-AC-10.

FR-016-AC-14 through AC-20 are implemented and tested in the default suite
(`tests/it/skeleton_spine.rs`): `ReplayPackage::new` hands the lock's dependency
selections to `qsl_replay::call_site` as its dependency input, and
`ReplayPackage::request` fills the request's `package.dependencies` from the same
lock. A unit that imports a locked dependency replays through `qsl_replay::replay`
to a reproduced violation, and the same unit at an input the imported function
holds settles `inconclusive`, so the dependency is evaluated and not assumed. An
import with no lock selection is refused at the call site, a lock recording another
`package_id` is refused by QSL as a dependency identity mismatch, and a lock
selecting a library the unit does not import is refused by QSL as unselected.
Libraries sharing a source owner are refused before the call site, and a library
sharing the unit's owner is refused by the call site.
