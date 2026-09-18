---
id: TC-029
title: "Verify composite equality oracle generation and three-way agreement"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-018
    type: verifies
  - target: ix://agent-ix/quire-contract-runtime/TC-026
    type: references
---
# TC-029: Verify composite equality oracle generation and three-way agreement

## Description

Verify that composite equality oracles generated from an admitted
CheckedPackage V2 cover every admitted composite and collection shape, refuse
every non-generated item with its own typed reason, are byte-deterministic, and
agree — outcome, admitted charges and consumed counters — with direct runtime
execution and with the pinned quire-spec-language value authority.

The discriminating evidence is the three-way agreement of step 4, not the
committed golden. The golden is blessable with `QUIRE_CODEGEN_BLESS=1`, so it
pins nothing on its own; it earns its place only because the golden crate is the
crate step 4 compiles and executes against independently constructed native
runs. A blessed golden whose emitted operator, operand order or descriptor
changed fails step 4.

## Test Procedure

1. Build an admitted V2 package holding `binary` expression nodes over: record,
   tuple and option types; `sequence`, `set`, `bag` and `ordered_set` collection
   types with their `collection_bounds` domains; nested composites inside
   collections and collections inside records; scalar leaves of every equality
   schedule (text, enum, quantity, and the `Plan` types); admitted `convert<T>`
   operands from each `admits_equality_conversion` row exercised; plus operands
   with distinct text profiles, distinct enum declarations, incompatible
   dimensions, distinct units, no common type, a `convert<T>` outside the table,
   a record with a `float64` field nested inside a `sequence`, a `reference`
   composite form, model, relation, function, `call`, state, temporal and
   protocol nodes, a duplicate record field, both recursion-rule cycles, an
   unlowered node, a non-`binary` form, a descriptor disagreeing with its
   operand types, a duplicate node id, and a request that exceeds the lowering
   work limit.
2. Generate twice and with a permuted request; compare bytes with each other and
   with the committed golden, and inspect claim-map ordering by node id (digest
   domain, then digest), reconstructed declaration keys against
   `NodeKey::from_hex` of the V2 digests, selected `EqualitySchedule` and
   `caller_declared` provenance.
3. Inspect each refusal: its typed cause, that the item's symbols are absent
   from the generated source, that its siblings are unchanged, and that the
   three upstream blockers (quire-spec-language#120, quire-contract-runtime#34,
   quire-spec-language#121) are distinct values rather than one reason. Assert
   the generated source contains no `unwrap`, `expect`, panicking index or
   charge or pair-count literal, and that the crate manifest declares
   `publish = false` and the pinned runtime revision with the `exact` feature.
4. Compile the golden oracle crate into the test crate and execute it on the
   corpus vectors. For each vector compare the `Outcome<bool>`, the admitted
   charge sequence and the consumed counters against (a) a direct call to
   `TypeEnvironment::check_equality` and `CheckedEquality::evaluate` on an
   environment and operands constructed in the test, not by the generator, and
   (b) the same call through `quire_spec_language::value` at the runtime's
   pinned authority. Compare the admitted `equality.pair` count against
   `EqualityPlan::pair_events()` from `plan_equality`, and repeat each vector
   with an operand pair that differs only in DAG sharing.
5. Re-execute every vector with a denial injected at each of
   `equality.plan-form`, `equality.plan`, `equality.pair`,
   `equality.result-retain` and each conversion charge point in turn; confirm
   `Outcome::Incomplete` naming that point with every counter unchanged.
6. Request one node twice under descriptors differing only in
   `EqualityOperator`; confirm both generate, both are marked `caller_declared`,
   and their outcomes are complementary on a vector whose operands differ.

## Expected Results

Every admitted shape and schedule is generated; every refused item is absent
from the source and carries its own typed reason; bytes are identical across
runs and orderings; all three executions agree on every vector's outcome,
charges and counters; the pair count equals the planned count and does not
change under sharing; every injected denial yields `Incomplete` at its point
with counters unchanged; the two operators generate with complementary results
under one `caller_declared` mark; and the generated crate compiles with
`publish = false` and no charge or pair-count literal.

Operand construction is the corpus's work, not the generator's: the composite
and collection values compared are built by the runtime's own FR-008
constructors, so a set's or bag's retention order is never a choice the
generated code makes.
