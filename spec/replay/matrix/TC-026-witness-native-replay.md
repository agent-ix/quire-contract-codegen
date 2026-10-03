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

For FR-016-AC-21 to AC-23, build the replay request for two functions with the same parameters
and compare the `obligation_identity` slots, recompile one with comments and blank lines inserted,
change its harness source span, then change its obligation kind and one argument domain in turn,
and scan `src/` for any computation of a function node id or occurrence key.

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
The two functions' identities differ, comments and blank lines and the span change leave one
unchanged, the kind change and the domain change each alter it, and `src/` computes no node id or
occurrence key (FR-016-AC-21 to AC-23).

## Status

FR-016-AC-21 to AC-23 are planned: the function path still passes the transcript digest, and
QSL's `FunctionSite` `function` and `declaration` members are not yet in the `qsl-replay` CG
builds against.

Partial. FR-016-AC-9 through AC-11 are implemented and tested in the default suite
(`tests/it/skeleton_spine.rs`): a falsifying input replays through
`qsl_replay::replay` and settles the violation, a witness at which the function
holds and a healthy twin settle `inconclusive` naming both verdicts, and each
adapter refusal is its own typed error. The ignored Kani lane (`make kani`, not
part of `make ci`) replays a real prover counterexample the same way. The native
twin is hand-mirrored QSL source and the input package is hand-built, so the
contract-to-Contract-IR step does not run.

FR-016-AC-8 is implemented and tested in the default suite: schema
order and naming by position (`src/kani_witness_join.rs` unit tests) and the
harness emission order (`tests/it/kani_argument_order.rs`); the real-backend
decode runs in the ignored Kani lane (`tests/it/kani_witness_join.rs`).

FR-016-AC-1 through AC-5 and AC-13 are implemented and tested in the default suite
through `replay_counterexample`: a transcript that decodes to nothing, has the wrong
value count or byte width, or names another harness is a decode evidence failure
carrying the decoder's cause code and is never replayed (AC-1, AC-5); a value one
past either end of its argument's bounds, or at an `i64` extreme, is a domain
evidence failure (AC-2, `src/kani_witness_join.rs` unit tests); an in-domain
counterexample the native twin falsifies is reproduced (AC-3); a twin that holds
the clause, or any settlement other than a reproduced violation, is a verdict
evidence failure (AC-4, AC-13, `src/spine_replay.rs` unit tests). Decode, domain
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
