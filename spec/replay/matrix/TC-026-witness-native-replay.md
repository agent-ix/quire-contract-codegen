---
id: TC-026
title: "Verify witness decoding and native replay"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-219
    type: references
---
# TC-026: Verify witness decoding and native replay

## Description

Verify that retained counterexamples are decoded, domain-checked and replayed
natively before any failure is reported.

## Test Procedure

Read a harness's persisted obligation arguments as the decode schema, including
one non-argument binding, and compare the schema's order with the harness's
emitted symbolic arguments. Decode a real falsifying transcript
against the persisted schema, then against that schema with a binding dropped
and with a binding retyped. Then replay a reproducing witness, a malformed
witness, a witness bound to another harness identity, a witness over the decode
size limit, an out-of-domain witness, a witness whose native value or charges
disagree with the harness, a witness whose native replay is unavailable, and
a witness whose replay agrees in a category other than `violation`.

For FR-016-AC-21 to AC-23, build the replay request for a function with two parameters declared
in an order that differs from their ascending identifier order. Recompute the O-09 digest from
the `FunctionSite`'s `function` and `declaration`, the requested kind and the arguments ascending
by identifier, through `quire-canonical`, and compare it with the request's `obligation_identity`
slot (AC-21). Then build the request for a second function with the same parameters, recompile
the first with comments and blank lines inserted, change its harness source span, replay it under
a different `ObligationKind`, and change one argument domain, comparing slots each time. Replay a
function whose body has two conjuncts under one kind and count its identities (AC-22). Rebuild the
identity with every other node of the compiled package perturbed and the `FunctionSite` members
held fixed, then with `function` and then `declaration` perturbed (AC-23).

## Expected Results

The schema follows the persisted argument order, which is the emission
order the harness uses, and refuses the non-argument binding with a typed
schema refusal (code `cg_witness_schema_non_argument_binding`) that
reports no failure and is none of the five replay results.
The real transcript decodes to values named by their bindings, and the dropped
and retyped schemas refuse by arity and width. Only the reproducing witness is
reported as a failure; the others yield malformed (three cases), out-of-domain,
mismatch and unavailable results respectively. The disagreeing witnesses are
never reported unavailable, and the unavailable replay is never reported as a
mismatch. The witness that agrees in a category other than `violation` is a
mismatch and never a failure.
The slot equals the recomputed digest (AC-21). The two functions' identities differ, comments and
blank lines and the span change leave one unchanged, the kind change and the domain change each
alter it, and the two-conjunct function has one identity (AC-22). The identity is unchanged by the
other-node perturbation and changed by each `FunctionSite` perturbation (AC-23).

## Status

FR-016-AC-21 to AC-23 are implemented and tested in the default suite
(`tests/it/skeleton_spine.rs`, and unit tests in `src/replay/obligation.rs` and
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
adapter refusal is its own typed error. The ignored Kani lane (`make kani`, not
part of `make ci`) replays a real prover counterexample the same way.

The verdict is tied to the exact witness values, not to any violating input. A twin whose clause
is false at a single point is reproduced only by the transcript carrying that point, and
neighbouring points settle `inconclusive` (default suite,
`tc_026_the_replay_verdict_is_decided_by_the_exact_witness_values`). In the Kani lane the point is
read from Kani's own printed block, by a reader that shares no code with `decode_falsification`
(the `// value` comment lines and, separately, the little-endian byte vectors, which must agree
with each other and with the decoder), and a neighbour control must not reproduce. Replacing the
real transcript with a constant, a decoder that adds one to each value, one that swaps the two
values, and one that returns a fixed in-domain pair each turned the Kani lane red when run on a
throwaway copy (IR-29).

What stays unbacked: the native twin is hand-mirrored QSL source, not derived from the contract or
the Rust subject, and the input package is hand-built, so the contract-to-Contract-IR step does not
run. The Kani lane is `#[ignore]` and outside `make ci`, so the real-prover mutations above are
not gated in CI; the default suite pins the replay end and the decoder against a synthetic
transcript only. FR-016-AC-6, AC-7 and AC-12 are planned.

FR-016-AC-8 is implemented and tested in the default suite: schema
order and naming by position (`src/replay/witness.rs` unit tests) and the
harness emission order (`tests/it/kani_argument_order.rs`); the real-backend
decode runs in the ignored Kani lane (`tests/it/kani_witness_join.rs`).

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
decode size limit (AC-6), the charge, counter and limit comparison (AC-7) and the
unavailable result for an executor fault (AC-12) are planned.

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
