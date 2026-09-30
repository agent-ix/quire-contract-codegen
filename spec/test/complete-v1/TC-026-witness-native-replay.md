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

Build a harness's witness schema from its persisted obligation arguments,
including one non-argument binding, and compare the schema's order with the
harness's emitted symbolic arguments. Decode a real falsifying transcript
against the persisted schema, then against that schema with a binding dropped
and with a binding retyped. Then replay a reproducing witness, a malformed
witness, a witness bound to another harness identity, a witness over the decode
size limit, an out-of-domain witness, a witness whose native value or charges
disagree with the harness, a witness whose native replay is unavailable, and
a witness whose replay agrees in a category other than `violation`.

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

## Status

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

Filling the replay request's `package.dependencies` from the proved lock is
implemented (`ReplayPackage::request`) and its wire shape is tested, but no replay
of a unit that imports a locked dependency is reachable: `qsl_replay::call_site`
compiles a standalone unit, so QSL refuses a request naming a dependency as
unselected, which `tests/it/skeleton_spine.rs` asserts.
