---
id: TC-031
title: "Verify function-application oracle generation, agreement, and static location tagging"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: verifies
  - target: ix://agent-ix/quire-contract-runtime/TC-194
    type: references
---
# TC-031: Verify function-application oracle generation, agreement, and static location tagging

## Description

Verify that function-application oracles generated from an admitted CheckedPackage V2 lower every
declared function's body, admit the assembled package, apply the requested function through the
runtime's FR-273 call surface with agreeing outcomes and charges, refuse every non-generated item
with its own typed reason, are byte-deterministic, and carry a static location map that round-trips
to the request's own expression trees without executing anything. It also verifies that no
generated code depends on `Evaluation.location`/`.losses` becoming non-empty, since the pinned
runtime revision never populates either field.

This test depends on this branch's own re-pin of `quire-contract-runtime` to `9f311692`
(`agent-ix/quire-contract-codegen` FR-021's own prerequisite): at the previously pinned `4e33052`,
the function-application surface step 4 calls (`PackageDeclarations`, `CheckedPackage`, `Frame`,
`Evaluation`) was not visible here at all.

The authority-agreement leg (FR-021-AC-18) is recorded here as 🚧 Planned, but not because the
authority surface is missing: `quire_spec_language::value::expression` already publishes
`PackageDeclarations`, `CheckedPackage::call` and `CheckedPackage::evaluate` at this repository's
current `21c507e` pin. It is planned because `21c507e` is not `ea39f91`, the revision
`quire-contract-runtime` FR-273-AC-5 names as its authority, and the 14 commits between them rewrite
`src/value/expression/` substantially. Asserting agreement against `21c507e` today would look green
while comparing against the wrong authority revision, which is the one failure this leg exists to
detect. See FR-021 Dependencies for the measurement and for the ruling that the re-pin is a separate
change.

## Test Procedure

1. Build an admitted V2 package holding: (a) two or more function declarations whose bodies are,
   respectively, a scalar expression (an admitted FR-014 form), a composite-equality expression (an
   admitted FR-018 form), and a nested `call` of another declared function in the same request; (b)
   a function whose declared parameter or result type reaches a `reference` composite form; (c) a
   function whose declared operator requirements name a capability no registered backend
   discharges; (d) a function whose body is an unlowerable node, a form none of the three
   classifiers admits, or a nested `call` naming a function absent from the request; (e) a `call`
   expression node applying an admitted function, with a duplicate copy of the same node id under
   the same binding; (f) one or more `call` nodes over model, relation, state, temporal and protocol
   forms; and (g) a nested-`call` chain deep enough to reach `MAX_CALL_DEPTH` when executed.
2. Generate twice and with a permuted request; compare bytes with each other and with the committed
   golden, and inspect claim-map ordering by the `call` node id, then the applied function's
   declaring node id, then each argument operand's source node id, every node id compared by digest
   domain then digest.
3. Inspect each refusal: its typed cause, that the item's symbols are absent from the generated
   source, that its siblings bound to an admitted package are unchanged, that the reference-typed
   function (b) is refused as blocked on quire-spec-language#120, that the model and relation nodes of
   (f) are refused as blocked on quire-spec-language#120 and its state, temporal and protocol nodes
   as blocked on quire-spec-language#121 — two distinct blockers, never collapsed into one reason —
   that the capability-gated function (c) is marked
   `unsupported` at generation time naming the capability and emits no oracle for any item naming
   it, and that the unlowerable function (d) refuses every item bound to it without changing an
   unrelated package's items. Assert the generated source contains no `unwrap`, `expect`, panicking
   index, charge-amount literal, or literal `Outcome`/`Value` constant standing in for a runtime
   result, and that the crate manifest declares `publish = false` and the pinned runtime revision
   with the `exact` feature.
4. Compile the golden oracle crate into the test crate and execute its generated oracle for each
   admitted `call` item on the corpus vectors. For each vector compare the `Outcome<Value>`, the
   admitted charge sequence and the consumed counters against a direct call to
   `CheckedPackage::call` on a package, arguments and fresh `Meter` constructed independently in the
   test from the request, not by or from the generator — the applied function's identity is the
   thing an emitter mutation changes, so reading it back out of the generated crate would make this
   leg follow the mutation and the comparison vacuous. Assert each claim-map entry's recorded
   `Origin::Body { function, index }` equals the request's own declared-function ordering.
   **Agreement against `quire_spec_language::value::expression::CheckedPackage::call`** (FR-021-AC-18)
   is 🚧 Planned: the call exists at the current `21c507e` pin, but `21c507e` is not the `ea39f91`
   authority revision FR-273-AC-5 names, so this leg is written and left unasserted until the re-pin
   lands rather than being run against the wrong authority (FR-021 Dependencies).
5. Re-execute the corpus with a denial injected at the `function.call` charge point; confirm
   `Outcome::Incomplete` naming that point and that the denied charge was not applied — every
   counter equal to those of the same run stopped immediately before that point.
6. Execute the deep nested-`call` chain of step 1(g); confirm `Refusal::CheckedInvariant` once
   `MAX_CALL_DEPTH` is exceeded, before any further charge, and that no vector in the corpus applies
   a package this generator checked under `CheckMode::Kernel` — inspect that `check` itself refuses
   `CheckMode::Kernel` unconditionally, which is what makes that true by construction rather than by
   sampling.
7. Walk the generated location map for the multi-form package of step 1(a): for every recorded
   entry, independently re-derive the `Location{origin, path}` by walking that same function's
   original request expression tree from its root (the function's own `Origin::Body { function,
   index }`) to the sub-expression that reaches the corresponding runtime call point, and assert
   field-for-field equality — a structural comparison against the request, executing nothing. Then
   confirm the `origin` half against the runtime itself (FR-021-AC-17): re-submit the same assembled
   package to `PackageDeclarations::check` with one function's measure left undischarged, read the
   `Origin::Body { function, index }` off the returned `CheckRefusal`, and assert it equals the
   `origin` the location map recorded for that function. The `path` half has no such counterpart —
   the runtime builds every `Location` through `location_at`, which always sets an empty `path` — so
   step 7's request-side re-derivation is the only check `path` can have, and this test states that
   rather than implying a runtime cross-check covers both fields.
8. Grep the generated crate's source and its claim map for any read of, branch on, or non-emptiness
   assertion against `Evaluation.location` or `Evaluation.losses`; confirm none exists, and that
   both fields are simply discarded by the emitted oracle function's return path, since the pinned
   runtime revision never populates either one regardless of what the applied body computed.

## Expected Results

Every admitted function's body lowers and every admitted package generates oracles for its `call`
items; a refused item is absent from the source with its own typed reason and its siblings are
unchanged; the reference-typed, capability-gated, unlowerable, and family-excluded functions are
each refused with their own distinct typed blocker rather than one collapsed reason; bytes are
identical across runs and orderings; the native `CheckedPackage::call` leg agrees on outcome,
charges and counters for every generated item, driven from the request rather than the generated
crate, while the authority leg stays 🚧 Planned pending the re-pin that moves this repository onto
the `ea39f91` revision FR-273 names; every injected denial yields `Incomplete` at its point
without applying that charge; the depth bound refuses `CheckedInvariant` once exceeded and no
generated oracle ever applies a `CheckMode::Kernel` package; the location map round-trips to the
request's own expression trees with no execution required; and no generated code reads or depends
on `Evaluation.location`/`.losses` becoming non-empty.

Function-body semantics beyond what FR-014's and FR-018's own oracles already verify are not
separately asserted here: a function body is a delegation to those same generators' lowering, so
step 4's agreement check already covers a scalar or equality sub-expression's own correctness
through its own family's corpus; this test's own new surface is admission, application, charge
sequencing, capability negotiation, depth bounding, and the static location map.
