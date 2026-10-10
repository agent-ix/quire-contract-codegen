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
generated code depends on `Evaluation.location`/`.losses` becoming non-empty, since Contract
Runtime never populates either field.

It also verifies that the emitted function-oracle source has no panicking path (no `.unwrap(`,
`.expect(` or panic macro), that an unknown `rt::Outcome` variant is returned as
`Refused(CheckedInvariant)`, and that a `Negate` body is refused rather than reaching a panic
(FR-021-AC-19 to AC-21).

It also verifies that declarations sharing one declaring node id are refused with
`DuplicateDeclaringNode` before Stage 1 (FR-021-AC-22), that a failed lowering record is refused
as its own byte-ceiling or work refusal (FR-021-AC-23), and that two different unknown function
names on one call node are two `UnknownFunction` entries, and two members of a duplicate-node pair
are two `DuplicateDeclaringNode` entries, in byte order of the function name when they are equal
on the earlier key fields and in declaring-node-id order otherwise (FR-021-AC-24).

The authority-agreement leg (FR-021-AC-18) is 🚧 Planned.

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
   forms; and (g) a 100,000-node nested-`call` chain whose lowered graph is
   supplied at the generator seam, independently of upstream reader limits.
2. Generate twice and with a permuted request; compare bytes with each other, and inspect claim-map ordering by the `call` node id, then the applied function's
   declaring node id, then each argument operand's source node id, every node id compared in
   node-id order.
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
   result, and that the crate manifest declares `publish = false` and the runtime dependency (git source,
   `branch = "main"`, no pinned `rev`) with the `exact` feature.
4. Generate the main and chain corpus crates at test time, compile them with the agreement cases as
   their integration test, and execute the generated oracle for each
   admitted `call` item on the corpus vectors. For each vector compare the `Outcome<Value>`, the
   admitted charge sequence and the consumed counters against a direct call to
   `CheckedPackage::call` on a package, arguments and fresh `Meter` constructed independently in the
   test from the request, not by or from the generator — the applied function's identity is the
   thing an emitter mutation changes, so reading it back out of the generated crate would make this
   leg follow the mutation and the comparison vacuous. Assert each claim-map entry's recorded
   `Origin::Body { function, index }` equals the request's own declared-function ordering.
   **Agreement against `quire_spec_language::value::expression::CheckedPackage::call`** (FR-021-AC-18)
   is 🚧 Planned.
5. Re-execute the corpus with a denial injected at the `function.call` charge point; confirm
   `Outcome::Incomplete` naming that point and that the denied charge was not applied — every
   counter equal to those of the same run stopped immediately before that point.
6. Analyze and emit the deep chain of step 1(g) on a 512 KiB stack with a
   caller-selected generation work limit equal to the measured node, edge and
   instruction charges. Confirm complete flat assignments, no depth setting
   or fixed depth refusal, deterministic source and location-map paths.
   Set the limit one unit lower and confirm `GenerationWorkExhausted` names
   that limit and its first denied unit, with no emitted symbol or location
   entry for the refused function while a healthy sibling remains unchanged.
   Independently make Contract IR lowering fail for work and confirm its
   existing lowering refusal wins before any generation charge. Inspect that
   `check` refuses `CheckMode::Kernel` unconditionally; no corpus oracle applies
   such a package. Execution of this deep chain remains planned against the
   QSL-owned iterative evaluator and the RT FR-275 migration.
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
8. Count `.unwrap(`, `.expect(`, `unreachable!`, `panic!`, `todo!` and `unimplemented!`, the macros in
   any delimiter form, in the emitted `src/lib.rs` of `main_oracles()` (it holds the scalar
   `add_fn`, the equality `eq_fn` and the nested-call `call_fn`) and of `chain_oracles()`, and
   assert zero (FR-021-AC-19). Assert that the emitted bodies of `add_fn` and `eq_fn` match
   `Ok(Completed)` (rewrapped), `Ok(Undefined)`, `Ok(Refused)`, `Ok(Incomplete)` and `Err(refusal)`,
   then end with an `Ok(_)` arm valued `Outcome::Refused(Refusal::CheckedInvariant)`
   (FR-021-AC-20). Request a `Negate` body and assert
   `ExactFunctionRefusal::UnsupportedOperator` and that the function is absent from the emitted
   `checked_package()`; count invocations of `unreachable!`, `panic!`, `todo!` and `unimplemented!`, comment lines not counted, in
   `src/oracle/function/mod.rs` and assert zero (FR-021-AC-21). The unknown variant itself cannot
   be built from a test crate because the runtime enum is `#[non_exhaustive]`, so the arm's text
   and the zero counts are the evidence.
9. Request two declarations sharing one declaring node id (FR-021-AC-22) in each of six
   fixtures: (i) both bodies admissible; (ii) one body refused in Stage 1 and its same-node
   sibling admissible; (iii) the two also sharing one name, plus a third declaration with its own
   distinct node id and the same name (absent from `checked_package()` and the location map, while
   items naming the shared name are `DuplicateDeclaringNode`); (iv) a third declaration whose
   nested `call` names one of the pair, plus a distinct-node-id function, with an item naming each
   function; (v) one name held by two duplicate groups on different node ids, with an item naming
   it; (vi) two items on one `call` node naming the two members of a pair, and one such item
   requested twice. Generate each fixture under every permutation of the declaration order, which
   includes both orders of the pair. Assert, in every case and order: neither declaration of the
   pair appears in the emitted `checked_package()`, source or `location-map.json`; every item
   naming one of them is refused with `ExactFunctionRefusal::DuplicateDeclaringNode` carrying the
   shared node id (never `UnknownFunction`, never `DuplicateRequest`, and in fixture (iii) never
   `AmbiguousFunctionName`), and in fixture (v) the smaller of the two node ids; no claim-map
   entry records another function's name, symbol or `Origin::Body` index; the third declaration of
   (iv) is refused as `UnknownCallee`; and each item over a distinct-node-id function has a
   claim-map entry equal to the entry the same request produces with the duplicate declarations
   removed. The tests are `tc_031_ac22_fixture_i_both_bodies_admissible`,
   `tc_031_ac22_fixture_ii_one_body_refused_and_sibling_admissible`,
   `tc_031_ac22_fixture_iii_shared_name_takes_the_node_id_refusal`,
   `tc_031_ac22_fixture_iv_nested_call_to_a_refused_duplicate_is_unknown_callee`,
   `tc_031_ac22_fixture_v_two_duplicate_groups_report_the_smallest_node_id` and
   `tc_031_ac22_fixture_vi_same_call_node_items_over_a_pair_each_keep_the_node_refusal`. The comments in `src/oracle/function/mod.rs` (module doc
   "Package assembly" and the `own_shape`/`bodies` comment) no longer claim node-id keying alone
   prevents collapse, which does not hold for duplicate node ids.
10. Grep the generated crate's source and its claim map for any read of, branch on, or non-emptiness
   assertion against `Evaluation.location` or `Evaluation.losses`; confirm none exists, and that
   both fields are simply discarded by the emitted oracle function's return path, since the
   runtime never populates either one regardless of what the applied body computed.
11. Lowering byte-ceiling failure (FR-021-AC-23, IR-547). Build a package whose two lowered
    contract packages of the call (bodies, and requested `call` nodes) are both longer than its
    checked package and whose body and call-node lowerings give different `consumed` under one
    ceiling (read each alone to confirm). Read it under a byte ceiling that admits the checked
    package and is one byte below the shorter of the two lowered packages' canonical lengths:
    every function is absent from `checked_package()` and `location-map.json`, and every item is
    refused `LoweringByteLimitExceeded` with `limit` equal to that ceiling and the call-node
    lowering's `consumed` (Contract IR FR-038-AC-95), none `LoweringWorkExhausted`; this asserts the
    call-node-first order. In the `#[cfg(test)]` module the code change adds to the function
    module, hand `lowered_binary_body` hand-built records: `work` gives `LoweringWorkExhausted`,
    `bytes` gives `LoweringByteLimitExceeded` with the record's `limit` and `consumed`, and each of
    `nodes`, `edges`, `occurrences` and `diagnostics` gives `LoweringLimitUnrecognised`
    with that snake_case `limit_kind`, with no panic and never `LoweringWorkExhausted`. Per-function
    isolation stays with FR-021-AC-12's `tc_031_ac12_*` tests.
12. Unknown function names, and duplicate-node pair members, on one call node (FR-021-AC-24,
    IR-545). For cases (i) to (v), over one declared `add_fn`, request on one `call` node with
    equal arguments: (i) `zz_unknown` once; (ii) `zz_unknown` and
    `aa_unknown`, in both request orders; (iii) `zz_unknown` twice; (iv) `add_fn` and `zz_unknown`;
    (v) `Zz_unknown` and `aa_unknown`, in both request orders; (vi) over declarations
    `m_multi` and `z_pair` sharing node id N2 and `m_multi` and `q_extra` sharing the smaller node
    id N1, items `m_multi` and `z_pair` on one `call` node in both request orders, and the same
    with `z_multi` and `a_pair` in place of `m_multi` and `z_pair`; (vii) over node ids
    N1 < N2 < N3, `a_pair` and `z_pair` declared on N2, `z_pair` and `q_extra` on N1 and `a_pair`
    alone on N3, items `a_pair` and `z_pair` on one `call` node in both request orders.
    Assert (i) one `UnknownFunction { name: "zz_unknown" }` entry; (ii) two entries, `aa_unknown`
    then `zz_unknown`, each `UnknownFunction` naming its own name, no `DuplicateRequest`, identical
    under both orders; (iii) one `DuplicateRequest` entry; (iv) two entries, the `UnknownFunction`
    one first, each equal to the entry that item gets when requested alone; (v) two entries,
    `Zz_unknown` then `aa_unknown` (byte order, case-sensitive), identical under both orders;
    (vi) two entries, `DuplicateDeclaringNode { N1 }` then
    `DuplicateDeclaringNode { N2 }` for `m_multi` and `z_pair`, and `{ N2 }` then `{ N1 }` for
    `z_multi` and `a_pair`, identical under both request orders (the name orders only items equal on
    the earlier key fields); (vii) two entries, `DuplicateDeclaringNode { N1 }` (for `z_pair`) then
    `DuplicateDeclaringNode { N2 }` (for `a_pair`), identical under both request orders: `a_pair`
    resolves to the larger N3 and `z_pair` to N2, so the declaring node id orders them before the
    smaller name `a_pair` could. The tests are
    `tc_031_ac24_case_i_one_unknown_name_is_one_entry`,
    `tc_031_ac24_case_ii_two_unknown_names_are_two_entries_in_name_order`,
    `tc_031_ac24_case_iii_the_same_unknown_name_twice_is_one_duplicate_request`,
    `tc_031_ac24_case_iv_known_and_unknown_order_unknown_first_and_match_solo`,
    `tc_031_ac24_case_v_names_order_by_bytes_case_sensitively`,
    `tc_031_ac24_case_vi_duplicate_node_pair_members_order_by_name` and
    `tc_031_ac24_case_vii_differing_declaring_node_ids_order_before_names`.

13. Flat-order denial (FR-021-AC-26, planned, IR-511). In a shallow body
    with observable left and right operand charges, deny the left charge and
    assert the right charge never occurs. Admit the left charge
    and deny the right; assert the exact admitted prefix and `Incomplete` at
    the right point. Compare a deep flat body and an equivalent shallow body
    for outcome and admitted charge order once the QSL-owned application
    authority can execute both.

## Expected Results

Every admitted function's body lowers and every admitted package generates oracles for its `call`
items; a refused item is absent from the source with its own typed reason and its siblings are
unchanged; the reference-typed, capability-gated, unlowerable, and family-excluded functions are
each refused with their own distinct typed blocker rather than one collapsed reason; bytes are
identical across runs and orderings; the native `CheckedPackage::call` leg agrees on outcome,
charges and counters for every generated item, driven from the request rather than the generated
crate, while the authority leg is 🚧 Planned; every injected denial yields `Incomplete` at its point
without applying that charge; deep analysis and flat emission obey the
caller-selected generation work limit without a fixed depth refusal and no
generated oracle ever applies a `CheckMode::Kernel` package; the location map round-trips to the
request's own expression trees with no execution required; and no generated code reads or depends
on `Evaluation.location`/`.losses` becoming non-empty; the emitted corpus source has zero
`.unwrap(`, `.expect(` and panic macros, every unknown `Outcome` variant refuses
`CheckedInvariant`, and a `Negate` body is refused with `UnsupportedOperator`; declarations sharing
a declaring node id are absent from the package, source and location map, every item naming one
is refused as `DuplicateDeclaringNode` independent of request order, and no claim or oracle symbol
is crossed (FR-021-AC-22); and two different unknown function names on one call node are two
`UnknownFunction` entries, while only the same name requested twice is a `DuplicateRequest`, and
two members of a duplicate-node pair are two `DuplicateDeclaringNode` entries ordered by function
name when equal on the earlier key fields and by declaring node id otherwise (FR-021-AC-24).

Function-body semantics beyond what FR-014's and FR-018's own oracles already verify are not
separately asserted here: a function body is a delegation to those same generators' lowering, so
step 4's agreement check already covers a scalar or equality sub-expression's own correctness
through its own family's corpus; this test's own new surface is admission, application, charge
sequencing, capability negotiation, charged iterative generation, and the static location map.
