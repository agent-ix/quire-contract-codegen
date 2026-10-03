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
execution and with the quire-spec-language value authority.

The discriminating evidence is the agreement of step 4. No generated output is
committed: step 4 generates the corpus crate at test time and executes it
against independently constructed native runs whose descriptor comes from the
request, so a generator change to an emitted operator, operand order or
descriptor fails step 4. Determinism is checked by regeneration in step 2.

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
   operand types, one node id repeated under one descriptor, and a request that
   exceeds the lowering work limit.
2. Generate twice and with a permuted request that includes both descriptors of
   step 7 over one node id; compare bytes with each other, and inspect claim-map ordering by the descriptor key — expression node
   id, operator rank, then each operand's source-type and conversion-target node
   ids, every node id compared in node-id order; the expression node
   id alone ties those two entries — reconstructed declaration keys against
   `NodeKey::from_hex` of the V2 node ids, and `caller_declared` provenance.
   Assert each entry's recorded descriptor is equal to the one the request
   supplied — operator, both operand source types, both conversion targets — and
   its recorded `EqualitySchedule` equal to `CheckedEquality::schedule()` of the
   `CheckedEquality` that descriptor admits, both compared against values the
   test holds, never against values parsed out of the generated source.
3. Inspect each refusal: its typed cause, that the item's symbols are absent
   from the generated source, that its siblings are unchanged, and that the
   three upstream blockers (quire-spec-language#120, quire-contract-runtime#34,
   quire-spec-language#121) are distinct values rather than one reason. Assert
   the generated source contains no `unwrap`, `expect`, panicking index or
   charge or pair-count literal, and that the crate manifest declares
   `publish = false` and the runtime dependency (git source, `branch = "main"`, no pinned `rev`)
   with the `exact` feature.
4. Generate the corpus crate at test time, compile it with the agreement cases
   as its integration test, and execute it on the corpus vectors. For each vector compare the `Outcome<bool>`, the admitted
   charge sequence and the consumed counters against (a) a direct call to
   `TypeEnvironment::check_equality` and `CheckedEquality::evaluate` on an
   environment, operands **and descriptor** constructed in the test from the
   request, not by or from the generator, and (b) the same call through
   `quire_spec_language::value`. The descriptor
   is the thing an emitter mutation changes, so reading it back out of the
   generated crate would make leg (a) follow the mutation and the comparison
   vacuous; it is held by the test.
5. Re-execute every vector with a denial injected at each of
   `equality.plan-form`, `equality.plan`, `equality.pair`,
   `equality.result-retain` and each conversion charge point in turn; confirm
   `Outcome::Incomplete` naming that point and that the denied charge was not
   applied — every counter equal to those of the same run stopped immediately
   before that point, not to the counters at entry.
6. Call each generated oracle whose operand types reach a declaration key with a
   `TypeEnvironment` other than the one its environment constructor returns — an
   empty environment, and one omitting a key its operand types reach, and one
   declaring that closure under a different key; confirm `Outcome::Refused(Refusal::CheckedInvariant)`, no panic, and no charge
   admitted on the `Meter`. Assert that this is the emitted `check_type` calls
   deciding it, by confirming the same environments pass `check_equality`: that
   call consults the environment only through `contains_ieee`, which returns
   `false` for a composite key the environment does not hold, and
   `CheckedEquality::evaluate` takes no environment at all — so without
   `check_type` these vectors complete with a Boolean.
7. Request one node twice in one request under descriptors differing only in
   `EqualityOperator`; confirm both generate under distinct readable symbols,
   each built from its own operator's stem, that both are
   marked `caller_declared`, and
   that their outcomes are complementary on a vector whose operands differ. This
   is not the duplicate case of step 1: a duplicate is one node id under one
   descriptor.
8. Generate an equality over a tuple whose member types are `bounded_domain`
   nodes (FR-018-AC-16): the text position names a `text_bounds` node, and the
   generated declaration is `Text(0, 16, Nfc)`; integer, decimal and rational
   `bounded_domain` members generate source identical to members naming their
   base scalars; with a second `text_bounds` node over the same text scalar,
   naming either reads that node's own bounds. Then name a `text_bounds` node
   over an integer scalar (refused as missing `integer_range`), and
   `bounded_domain` nodes over a boolean scalar, over the base package's enum
   declaration and over a record (each refused as unsupported `bounded_domain`).
   Name a `float_rounding` node over a `float64` scalar as a member: it reads as the
   float and the equality is refused as `OperatorIneligible`.

9. Source-level scan (FR-018-AC-17, FR-018-AC-18). Read the emitted `src/lib.rs`
   of this corpus and of the FR-014 scalar corpus, and the non-test text of
   `src/oracle/equality/mod.rs` and `src/oracle/scalar/mod.rs` (everything before
   the `#[cfg(test)]` module, comment lines dropped, string literals kept), and
   assert zero occurrences of `.unwrap(`, `.expect(`, `unreachable!`, `panic!`,
   `todo!` and `unimplemented!`, the macros in any delimiter form. Then request an
   equality whose operand type is rendered with a `ValueType::Quantity` and one
   with a `ValueType::Reference`, and assert each is refused with a typed
   `OracleGenerationError` and absent from the emitted crate.

## Expected Results

The source scan finds zero panic sites, and a quantity or reference operand is a
typed refusal rather than a generator panic.

Every admitted shape and schedule is generated; every refused item is absent
from the source and carries its own typed reason; bytes are identical across
runs and orderings; each entry's recorded descriptor and schedule equal the
request's and `CheckedEquality::schedule()`; all three executions agree on every
vector's outcome, charges and counters; every injected denial yields
`Incomplete` at its point without applying that charge; a foreign environment
refuses as `CheckedInvariant` without panicking and without charging, decided by
the emitted `check_type` calls; the two operators generate under distinct
symbols with complementary results and one `caller_declared`
mark; and the generated crate
compiles with `publish = false` and no charge or pair-count literal.

Operand construction is the corpus's work, not the generator's: the composite
and collection values compared are built by the runtime's own FR-008
constructors, so a set's or bag's retention order is never a choice the
generated code makes. For the same reason the schedule the runtime selects and
the pair count `plan_equality` forms are not separately asserted: the generated
function is a delegation to `CheckedEquality::evaluate`, so both are fixed inside
the runtime and step 4's charge-sequence comparison already covers them. What
this test asserts about them is what the emitter chooses — the descriptor and
schedule the claim map records, in step 2.

No vector produces `Refusal::ForeignReference`: reference operands are excluded,
so `CheckedInvariant` from step 6 is the only run-time `Refused` a generated
oracle here can yield.
