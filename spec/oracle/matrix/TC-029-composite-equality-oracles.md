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

9. Emitted-source scan (FR-018-AC-17). Read the `src/lib.rs` that
   `generate_composite_equality_oracles` returns for the step 1 package and assert
   zero occurrences of the panic tokens of step 11, no `.ok()`, `.unwrap_or(` or
   `.unwrap_or_default(`, and no `[` index or slice expression directly after an
   identifier character, `)` or `]`. Assert every reconstruction helper returns
   `Result<_, ReconstructionError>`, every call of one is followed by `?` or a
   `match` whose `Err` arm yields `EnvironmentError::Reconstruction` or
   `Outcome::Refused(Refusal::CheckedInvariant)`, and `EnvironmentError` has the
   variants `Declaration(InvalidDeclaration)` and `Reconstruction(ReconstructionError)`,
   `ReconstructionError` has the unit variants `Integer`, `Interval`, `Rational`,
   `Decimal`, `Text` and `Cardinality`, and no call of `IntegerInterval::new`,
   `RationalDomain::new`, `TextType::new`, `CardinalityBound::new`,
   `DecimalType::new` or an integer `.parse()` appears outside a reconstruction
   helper. Assert each of `rebuild_integer`, `rebuild_interval`, `rebuild_rational`,
   `rebuild_decimal`, `rebuild_text` and `rebuild_cardinality` fails with exactly
   its own `ReconstructionError` variant through one `map_err`, with no other
   variant and no `or_else`, `or`, `ok`, `unwrap_or`, `map_or` or `match`, and show
   by hand-written snippets (a wrong variant, a silent widen, a swallowed failure)
   that the check names each departure. No valid request makes a reconstruction fail, so the failure path is verified
   structurally here and not by execution.
10. Render unit test (FR-018-AC-18). In the `#[cfg(test)]` tests of
    `src/oracle/equality/mod.rs`, call `render_value_type` with
    `ValueType::Quantity` and with `ValueType::Reference` and assert each returns
    `Err(RenderError::UnsupportedValueType { family })` with `family`
    `"quantity"` and `"reference"` respectively. Then feed a `RenderError` through
    the item-boundary mapping and assert it yields
    `CompositeEqualityRefusal::Unsupported` with `node_tag` equal to `family` and
    `unsupported_node_id` equal to the item's expression node id (for an operand
    type and for a composite declaration alike), with the
    item's siblings unchanged. Feed a `RenderError::Generation` carrying
    `UnknownRuntimeVariant` through the same mapping and assert the whole call
    fails with it. A public-API request cannot reach either arm: a
    quantity operand is refused earlier as `CompositeEqualityRefusal::Unsupported`
    and a reference operand by Contract IR, which AC-7 already covers.
11. Generator-source scan (FR-018-AC-19). Read the non-test text of
    `src/oracle/equality/mod.rs` (comments and every `#[cfg(test)]` item removed
    wherever the item sits, string literals kept), assert the scan removed the test
    module and nothing before it, and assert zero panic tokens as FR-018-AC-19
    defines them: `unwrap`, `expect`, `unwrap_unchecked`, `unwrap_err`,
    `expect_err`, `unwrap_err_unchecked`, `panic_any` and `resume_unwind` however
    written (including `Option::unwrap` and `.unwrap ()`), the macros `panic`,
    `unreachable`, `todo`, `unimplemented`, `assert`, `assert_eq`, `assert_ne`,
    `debug_assert`, `debug_assert_eq` and `debug_assert_ne` in any delimiter form,
    and `abort` anywhere but as a method call. The scan is shown to name each spelling by
    the TC-024 panic-scan snippet test, which shares it.
12. Lowering byte-ceiling failure (FR-018-AC-20, IR-547). Request a healthy node and a node whose
    preimage is over the ceiling the package was read under, at a ceiling the package fits under:
    only the over-ceiling item is refused `LoweringByteLimitExceeded` with the lowering record's
    `limit` and `consumed` (Contract IR FR-038-AC-95), and the healthy item's claim-map entry equals
    the entry of the same call with the other removed. Request every node at a ceiling one byte below
    the package's canonical length: every item is refused `LoweringByteLimitExceeded` with one shared
    `limit` and `consumed`, none `LoweringWorkExhausted`, and none generates. A work-ceiling failure
    is still `LoweringWorkExhausted`, and a `failed` record for each of the five other limit kinds is
    `LoweringLimitUnrecognised` with no panic.

## Expected Results

The source scans find zero panic sites, and the render function returns a typed
error for a quantity or reference type rather than panicking.

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
