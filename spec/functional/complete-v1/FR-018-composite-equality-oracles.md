---
id: FR-018
title: "Generate exact complete-V1 composite equality oracles"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: depends_on
  - target: ix://agent-ix/quire-contract-runtime/FR-008
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-036
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-038
    type: references
---
# FR-018: Generate exact complete-V1 composite equality oracles

## Description

When a caller supplies an admitted `quire.checked-package/v2` package and a set
of equality expression nodes, the code generator shall emit a deterministic Rust
oracle crate whose functions decide each node by reconstructing the item's
`TypeEnvironment` and calling the pinned Contract Runtime
`TypeEnvironment::check_equality` and `CheckedEquality::evaluate` with a
caller-supplied `Meter`. It never compares two values structurally, and never
copies a resource charge or a planned pair count.

This is issue #48, the composite/structural equality slice of complete-V1 oracle
generation. [FR-014](./FR-014-exact-scalar-oracles.md) is its scalar sibling.
The remaining families of #48 are not this requirement's work and are refused by
it: model and relation oracles await agent-ix/quire-spec-language#120, temporal
and protocol oracles await agent-ix/quire-spec-language#121, and function
application awaits agent-ix/quire-contract-runtime#34 — the Contract Runtime
publishes composite, collection and equality operators and no function
application surface at all, so an oracle over it has nothing to call.

Two upstream re-pins are prerequisites of this requirement, not consequences of
it, and both are satisfied. This repository pins Contract Runtime `4e33052` and
quire-spec-language `21c507e`, which is what makes the composite, collection and
equality surface this requirement calls visible here at all, and what gives
AC-2's third leg a `quire_spec_language::value` publishing `check_equality`,
`CheckedEquality` and `plan_equality` to agree with. At the revisions this
repository pinned before agent-ix/quire-contract-codegen#75 neither surface
existed, so no item of this requirement could have been generated and AC-2 could
not have been discharged at all.

The runtime carries no structural equality on `Value`: the FR-149 relation
reached through `check_equality`, `plan_equality` and `CheckedEquality::evaluate`
is the only equality, and it is the only relation that is metered, that is
correct for signed zero, for decimal values retained at differing scales and for
cross-universe references, and that the pinned quire-spec-language authority
decides. Generated oracles therefore call it and decide nothing themselves.

A `CheckedEquality` has no public constructor: it is obtained only from
`TypeEnvironment::check_equality`. The static stage therefore runs twice — once
in the generator, which refuses the item if it does not admit, and once in the
generated function, whose refusal is reported as
`Outcome::Refused(Refusal::CheckedInvariant)` rather than unwrapped.

That second stage must include `TypeEnvironment::check_type` on both operand
comparison types, before `check_equality`. `check_equality` consults the
environment only through `contains_ieee`, which treats a composite key absent
from the environment as bearing no IEEE value and returns `false`; and
`CheckedEquality::evaluate` takes no environment at all, comparing the caller's
two values directly. Without `check_type` the emitted function's
`&TypeEnvironment` parameter therefore decides almost nothing, and an environment
that declares none of the item's composites still completes. `check_type` is the
runtime's own check that every named declaration exists, that every
`Reference<T>` names a model object type, and that no set, bag or ordered set has
an IEEE-bearing element type; requiring it is what makes the second stage a stage
rather than a formality.

## Inputs

- An admitted `CheckedPackageV2` read through Contract IR's strict reader.
- A request of items, each naming one checked `expression` node of semantic form
  `binary` and one typed equality descriptor: an `EqualityOperator`
  (`Equal` or `NotEqual`) and two `EqualityOperand`s, each either
  `EqualityOperand::typed(source)` or `EqualityOperand::converted(source,
  target)`.
- The declaration closure reachable from each operand's `semantic_type`:
  `composite_type` nodes of form `record`, `tuple`, `option`, `sequence`, `set`,
  `bag` and `ordered_set`, their `scalar_type` leaves, and the `bounded_domain`
  nodes those types need.
- The pinned Contract Runtime revision with the `exact` feature.

As in FR-014, the V2 transport carries only an operator class (`binary`) and not
the operation law, so it does not say whether a node's operator is `=` or `!=`.
The law stays a caller-declared typed descriptor: every claim marks its operation
`caller_declared`, and the claim map carries the typed blocked item "operation
identity not carried by CheckedPackage V2". Downstream obligations must not treat
a caller-declared operation as checked.

A declaration is read from the `composite_type` node's body. Its body is an
`aggregate`; this encoding is defined by this generator, not by V2:

| Form | Members |
|------|---------|
| `record` | one `binding` per field in declaration order: its `name` is the field name, its value a `reference` to the field's type node; a field whose bound node is an `option` composite type is `Presence::Optional`, every other field `Presence::Required` |
| `tuple` | one `reference` per position, in declaration order |
| `option` | one `reference` to the payload type node |
| `sequence`, `set`, `bag`, `ordered_set` | a `reference` to the element type node, then a `reference` to the `collection_bounds` domain node |
| `collection_bounds` | two canonical decimal `integer` literals: minimum, maximum |

The runtime `NodeKey` of a declaration is `NodeKey::from_hex` of its V2 node id
digest, whose domain is `NODE_KEY_DOMAIN`.

## Outputs

- A generated crate: `Cargo.toml` (`publish = false`, runtime pinned by revision
  with the `exact` feature) and `src/lib.rs` holding, per generated item, one
  environment constructor returning `Result<TypeEnvironment, InvalidDeclaration>`
  and one oracle function
  `(&TypeEnvironment, &Value, &Value, &mut Meter) -> Outcome<bool>`.
- A typed claim map with one entry per requested item: node id, Contract IR
  semantic id, package id, semantic type, source map, claims, the reconstructed
  declaration closure and its keys, the selected `EqualitySchedule`, operation
  identity with its `caller_declared` provenance, and either the generated
  symbols or the typed refusal; plus the map-level blocked items.

## Behavior

- When generation begins, the generator shall lower every requested node through
  Contract IR's complete lowering with an equality tag profile and select exactly
  one generated or refused disposition per item.
- When a lowered node is a `binary` expression whose operand `semantic_type`s
  reconstruct to a declaration closure that `TypeEnvironment::new` admits, and
  whose descriptor `TypeEnvironment::check_equality` admits, the generator shall
  emit that item's environment constructor and oracle function.
- Each emitted oracle function shall call `TypeEnvironment::check_type` on each
  of its descriptor's two operand comparison types, then `check_equality` with
  the item's descriptor, then `CheckedEquality::evaluate` on the caller's two
  values and `Meter`, returning its `Outcome<bool>` unchanged. It shall contain
  no `unwrap`, `expect`, index or arithmetic that can panic: an `IllTyped` from
  either `check_type` or `check_equality` becomes
  `Outcome::Refused(Refusal::CheckedInvariant)`, before any charge.
- If `TypeEnvironment::new` refuses the item's declaration closure — duplicate
  key, duplicate member, unknown declaration, an ill-typed member, or either
  recursion pass — then the generator shall refuse the item with that
  `DeclarationCause` and emit no code for it, without changing any other item's
  output.
- If `check_equality` refuses the descriptor — a `convert<T>` operand outside
  `admits_equality_conversion`, distinct text profiles, distinct enum
  declarations, incompatible dimensions, distinct units, no common type, or an
  operand type that bears an IEEE value at any depth — then the generator shall
  refuse the item with that `IllTypedCause` and emit no code for it.
- If an operand type reaches a `composite_type` of form `reference`, or the node
  belongs to the model or relation families, then the generator shall refuse it
  as blocked on quire-spec-language#120. The reason is input-side, not runtime
  capability: the runtime implements reference equality and `ObjectReference::new`
  over `UniverseIdentity::new` and `ObjectIdentity::new` constructs a reference
  from arbitrary canonical bytes, but CheckedPackage V2 carries no form that
  yields a reference operand, and the identity binding that would give one
  meaning is quire-spec-language#120.
- If the node belongs to the function family, or is an expression of form `call`,
  then the generator shall refuse it as blocked on quire-contract-runtime#34; the
  state, temporal and protocol families as blocked on quire-spec-language#121.
- If a node is unlowered, invalid, over its work limit, of a form other than
  `binary`, or disagrees with its descriptor's arity or operand types, then the
  generator shall refuse the item with a typed reason.
- If one node id appears more than once in the request under one descriptor, then
  the generator shall refuse every copy. One node id under two descriptors is not
  a duplicate: the operation law is caller-declared and V2 does not carry it, so
  two descriptors over one node are two distinct items and each receives its own
  disposition and symbol.
- The generator shall order output by the item's **descriptor key** so that equal
  requests in any order produce identical bytes. Node id alone is not a total
  order, because one node id carries one item per descriptor. The descriptor key
  is the sequence: the expression node's id, then the operator's rank (`Equal`
  before `NotEqual`), then the V2 node ids of the left operand's source type and
  conversion target, then the right's, an absent conversion target ranking before
  every present one. Every node id in the key is compared by digest domain, then
  digest — the same rule this requirement already uses for node ids, and the only
  total order available: the runtime's `ValueType` derives `Clone`, `Debug`, `Eq`
  and `PartialEq` and no `Ord`, `EqualityOperator` derives no `Ord`, and the
  runtime publishes no canonical encoding of a `ValueType`, while `NodeKey`
  derives `Ord` over its digest.
- Each generated symbol shall be disambiguated by that same descriptor key: the
  symbol carries a digest over the key's node id digests and operator rank, in
  key order. It is never derived from a declaration name, a field name or any
  other rendered label, because two distinct items can share every name they
  display.
- If the generated source exceeds its size ceiling, then the generator shall
  return a typed error and no partial output.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-018-AC-1 | Every requested item receives exactly one generated or typed-refused disposition; a refused item contributes no function, no environment constructor and no symbol to the generated crate, while its siblings are generated unchanged. | Test (TC-029) |
| FR-018-AC-2 | On every vector of the composite-equality corpus, the generated oracle's `Outcome<bool>`, its admitted charge sequence and its consumed counters are equal to those of `quire_contract_runtime::exact::TypeEnvironment::check_equality` plus `CheckedEquality::evaluate` invoked directly on an independently constructed environment, operands and fresh `Meter`, and equal to `quire_spec_language::value` under the pinned authority. | Test (TC-029) |
| FR-018-AC-3 | Each generated item's claim-map entry records the descriptor the request supplied — operator, both operand source types and both conversion targets — and it is equal to the request's, so the independent native run of AC-2 is driven from the request and never read back out of the generated crate. | Test (TC-029) |
| FR-018-AC-4 | Each generated item's claim-map entry records an `EqualitySchedule` equal to `CheckedEquality::schedule()` of the `CheckedEquality` its descriptor admits, for a top-level text, enum and quantity pair and for each `Plan`-schedule composite and collection shape. | Test (TC-029) |
| FR-018-AC-5 | A descriptor with a `convert<T>` operand outside `admits_equality_conversion`, distinct text profiles, distinct enum declarations, incompatible dimensions, distinct units, or no common type is refused at generation time with its `IllTypedCause`, emits no code, and admits no charge on any `Meter`. | Test (TC-029) |
| FR-018-AC-6 | An operand type bearing an IEEE value at any depth — including a `float64` field of a record nested inside a `sequence` — is refused as `OperatorIneligible`, and the refusal agrees with `TypeEnvironment::contains_ieee` on the reconstructed type at every depth in the corpus. | Test (TC-029) |
| FR-018-AC-7 | A `reference` composite form or an operand reaching one, and model and relation nodes, are refused as blocked on quire-spec-language#120; function nodes and `call` expressions as blocked on quire-contract-runtime#34; state, temporal and protocol nodes as blocked on quire-spec-language#121 — each with its own distinct typed blocker, and none reported as generated. | Test (TC-029) |
| FR-018-AC-8 | A declaration closure `TypeEnvironment::new` refuses is refused at generation time carrying that `DeclarationCause`, including both recursion passes and a duplicate record field; and each generated oracle calls `TypeEnvironment::check_type` on both operand comparison types before `check_equality`, so calling it with an empty environment, one omitting a key its operand types reach, or one declaring that closure under a different key, yields `Outcome::Refused(Refusal::CheckedInvariant)` with no charge admitted, rather than a panic or a completed Boolean. | Test (TC-029) |
| FR-018-AC-9 | Every generated oracle function returns `Outcome<bool>`, never `bool`: with a denial injected at each of `equality.plan-form`, `equality.plan`, `equality.pair`, `equality.result-retain` and each conversion charge point in turn, the oracle yields `Outcome::Incomplete` naming that point, the denied charge is not applied — every counter equals those of the same run stopped immediately before that point — and never a completed Boolean. | Test (TC-029) |
| FR-018-AC-10 | Generated bytes are identical across repeated runs and across permutations of the request order, including a request carrying two descriptors over one node id, and claim-map entries are ordered by the descriptor key — expression node id, operator rank, then each operand's source-type and conversion-target node ids, every node id compared by digest domain then digest; the committed golden crate is compiled and executed under AC-2 against a native run driven from the request's own descriptor (AC-3), so a re-blessed golden that changed an emitted operator, operand order or descriptor fails AC-2 rather than passing. | Test (TC-029) |
| FR-018-AC-11 | Every claim marks its operation `caller_declared`, the claim map carries the blocked item "operation identity not carried by CheckedPackage V2", and two items over one node differing only in `EqualityOperator` both generate with that mark, under distinct symbols, each a digest over its descriptor key's node id digests and operator rank and over no rendered name, and produce complementary outcomes on a vector whose operands differ. | Test (TC-029) |
| FR-018-AC-12 | The generated crate declares `publish = false`, pins the runtime revision with the `exact` feature, contains no charge amount and no planned pair count (every charge and every pair comes from runtime metering), and compiles. | Test (TC-029) |
| FR-018-AC-13 | Every claim-map entry carries the node id, IR id, package id, source map, claims, reconstructed declaration keys and selected schedule of its item, and its declaration keys equal `NodeKey::from_hex` of the V2 node id digests its operand types reach. | Test (TC-029) |

AC-5 requires each listed condition to be refused with its `IllTypedCause`, not
that the six causes be distinct. Two of them are not: a `convert<T>` operand
outside `admits_equality_conversion` and two operand types with no common type
both refuse as `IllTypedCause::TypeMismatch`, because that is the cause the
authority names for both. AC-7 demands distinct blockers because those three are
distinct upstream tickets; AC-5 does not, because these are one authority cause.

### Mutations these criteria detect

Each criterion is recorded with the mutation it exists to catch; a criterion
without one is not written.

| ID | Mutation that breaks it |
|----|-------------------------|
| FR-018-AC-1 | Abort the whole request on the first refusal, or emit a stub function for a refused item. |
| FR-018-AC-2 | Emit `EqualityOperator::Equal` where the descriptor says `NotEqual`, or apply the right operand's conversion before the left's. |
| FR-018-AC-3 | Record the descriptor by reading it back out of the generated source, so a mutated emitter and a mutated claim map agree with each other and AC-2's native leg follows the mutation. |
| FR-018-AC-4 | Record a fixed `EqualitySchedule::Plan` on every entry, so a top-level text, enum or quantity item reports a schedule the runtime never selected. |
| FR-018-AC-5 | Generate the item and let the runtime refuse at evaluation time instead of refusing at generation time. |
| FR-018-AC-6 | Check `contains_ieee` only at the top level, so a nested `float64` field generates. |
| FR-018-AC-7 | Collapse the three blockers into one "unsupported" reason, or generate an oracle over a `reference` operand. |
| FR-018-AC-8 | Emit only `check_equality`, omitting the `check_type` calls; `check_equality` consults the environment solely through `contains_ieee`, which returns `false` for an absent composite key, so an empty environment completes with a Boolean instead of refusing. Or emit `.expect(..)`, which panics instead. |
| FR-018-AC-9 | Emit closures returning plain `bool`, as `src/oracle.rs` does for Boolean connectives; a denial then has no representable result. |
| FR-018-AC-10 | Order claim-map entries by the expression node id alone, which ties the two items of one node and lets a permuted request permute them; or bless a golden whose emitted operator was changed while the native leg reads its descriptor from that same golden; or swap which operand's runtime value or conversion target is evaluated as left versus right; or swap the emitted `left_source`/`right_source` descriptor itself. |
| FR-018-AC-11 | Mark the operation checked rather than `caller_declared`; refuse the second descriptor as a duplicate; or derive both symbols from the node's declaration name, so the two items collide. |
| FR-018-AC-12 | Emit the plan's pair count or a charge amount as a literal constant in the generated source. |
| FR-018-AC-13 | Key declarations by request ordinal instead of by the V2 node id digest. |

## Dependencies

- **Upstream**: [FR-001](../FR-001-deterministic-oracles.md),
  [FR-014](./FR-014-exact-scalar-oracles.md), Contract IR FR-036/FR-038
  (CheckedPackage V2 lowering), Contract Runtime FR-008 (composite, collection
  and equality families), quire-spec-language at the runtime's pinned authority.
- **Prerequisite, satisfied**: agent-ix/quire-contract-codegen#75, merged as
  `e74d592`, which re-pinned Contract Runtime `a04bd47` to `4e33052` and
  quire-spec-language `d9d5273` to `21c507e`. The first makes the equality
  surface this requirement calls visible from this repository; the second makes
  AC-2's third leg — agreement with `quire_spec_language::value` — a check with
  something to call.
- **Downstream**: [TC-029](../../test/complete-v1/TC-029-composite-equality-oracles.md).

## Out of Scope

- Function application oracles: the pinned runtime publishes no function
  application surface (agent-ix/quire-contract-runtime#34).
- Model graph, identity and reachability oracles
  (agent-ix/quire-spec-language#120), which FR-019 will own.
- Temporal and protocol oracles (agent-ix/quire-spec-language#121), which FR-020
  will own.
- `Refusal::ForeignReference`. It is the runtime's one substantive equality
  refusal — a reference pair whose universes differ, refused at plan time — and
  because the reference exclusion above is total, no operand this requirement
  admits can reach it. It is therefore named in no criterion and exercised by no
  vector, and the only run-time `Outcome::Refused` a generated oracle can produce
  is `Refusal::CheckedInvariant` (AC-8). That is a consequence of the exclusion,
  not an omission; it ends when quire-spec-language#120 admits reference
  operands.
- Composite and collection *construction* oracles. This requirement generates the
  equality relation over completed operands; `construct_collection`,
  `form_collection`, `evaluate_record` and `evaluate_tuple` are the runtime's
  FR-008 constructors and are called by the corpus, not emitted.
