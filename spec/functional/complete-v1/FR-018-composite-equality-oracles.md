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
application awaits agent-ix/quire-contract-runtime#34 — the pinned runtime
publishes composite, collection and equality operators and no function
application surface at all, so an oracle over it has nothing to call.

The runtime carries no structural equality on `Value`: the FR-149 relation
reached through `check_equality`, `plan_equality` and `CheckedEquality::evaluate`
is the only equality, and it is the only relation that is metered, that is
correct for signed zero, for decimal values retained at differing scales and for
cross-universe references, and that the pinned quire-spec-language authority
decides. Generated oracles therefore call it and decide nothing themselves.

A `CheckedEquality` has no public constructor: it is obtained only from
`TypeEnvironment::check_equality`. The static stage therefore runs twice — once
in the generator, which refuses the item if it does not admit, and once in the
generated function, whose refusal is unreachable for an admitted item and is
reported as `Outcome::Refused(Refusal::CheckedInvariant)` rather than unwrapped.

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
- Each emitted oracle function shall call `check_equality` with the item's
  descriptor and then `CheckedEquality::evaluate` on the caller's two values and
  `Meter`, returning its `Outcome<bool>` unchanged. It shall contain no `unwrap`,
  `expect`, index or arithmetic that can panic: a `check_equality` refusal
  becomes `Outcome::Refused(Refusal::CheckedInvariant)`.
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
  as blocked on quire-spec-language#120: a `Reference<T>` value is terminal and
  carries a `(UniverseIdentity, object-type key, ObjectIdentity)` triple that
  only the model graph supplies, so no admitted operand can be constructed for it
  here.
- If the node belongs to the function family, or is an expression of form `call`,
  then the generator shall refuse it as blocked on quire-contract-runtime#34; the
  state, temporal and protocol families as blocked on quire-spec-language#121.
- If a node is unlowered, invalid, over its work limit, of a form other than
  `binary`, or disagrees with its descriptor's arity or operand types, then the
  generator shall refuse the item with a typed reason.
- If a node id appears more than once in the request, then the generator shall
  refuse every copy.
- The generator shall order output by node id (digest domain, then digest) so
  that equal requests in any order produce identical bytes.
- If the generated source exceeds its size ceiling, then the generator shall
  return a typed error and no partial output.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-018-AC-1 | Every requested item receives exactly one generated or typed-refused disposition; a refused item contributes no function, no environment constructor and no symbol to the generated crate, while its siblings are generated unchanged. | Test (TC-029) |
| FR-018-AC-2 | On every vector of the composite-equality corpus, the generated oracle's `Outcome<bool>`, its admitted charge sequence and its consumed counters are equal to those of `quire_contract_runtime::exact::TypeEnvironment::check_equality` plus `CheckedEquality::evaluate` invoked directly on an independently constructed environment, operands and fresh `Meter`, and equal to `quire_spec_language::value` under the pinned authority. | Test (TC-029) |
| FR-018-AC-3 | For a `Plan`-schedule item, the number of `equality.pair` charges the executed oracle admits equals `EqualityPlan::pair_events()` from `plan_equality` on the same operands, and two operand pairs that differ only in DAG sharing yield the same outcome and the same pair count. | Test (TC-029) |
| FR-018-AC-4 | An item whose common comparison type is top-level text, enum or quantity runs the text, enum or quantity schedule, and every other admitted common type runs `equality.plan-form`, `equality.plan`, one `equality.pair` per planned pair and `equality.result-retain`, each identified by the admitted charge sequence rather than by the generated text. | Test (TC-029) |
| FR-018-AC-5 | A descriptor with a `convert<T>` operand outside `admits_equality_conversion`, distinct text profiles, distinct enum declarations, incompatible dimensions, distinct units, or no common type is refused at generation time with its `IllTypedCause`, emits no code, and admits no charge on any `Meter`. | Test (TC-029) |
| FR-018-AC-6 | An operand type bearing an IEEE value at any depth — including a `float64` field of a record nested inside a `sequence` — is refused as `OperatorIneligible`, and the refusal agrees with `TypeEnvironment::contains_ieee` on the reconstructed type at every depth in the corpus. | Test (TC-029) |
| FR-018-AC-7 | A `reference` composite form or an operand reaching one, and model and relation nodes, are refused as blocked on quire-spec-language#120; function nodes and `call` expressions as blocked on quire-contract-runtime#34; state, temporal and protocol nodes as blocked on quire-spec-language#121 — each with its own distinct typed blocker, and none reported as generated. | Test (TC-029) |
| FR-018-AC-8 | A declaration closure `TypeEnvironment::new` refuses is refused at generation time carrying that `DeclarationCause`, including both recursion passes and a duplicate record field; and no generated crate contains `unwrap`, `expect` or a panicking index, so a run-time `check_equality` refusal is reported as `Outcome::Refused(Refusal::CheckedInvariant)`. | Test (TC-029) |
| FR-018-AC-9 | Every generated oracle function returns `Outcome<bool>`, never `bool`: with a denial injected at each of `equality.plan-form`, `equality.plan`, `equality.pair`, `equality.result-retain` and each conversion charge point in turn, the oracle yields `Outcome::Incomplete` naming that point with every counter unchanged, and never a completed Boolean. | Test (TC-029) |
| FR-018-AC-10 | Generated bytes are identical across repeated runs and across permutations of the request order, and claim-map entries are ordered by node id (digest domain, then digest); the committed golden crate is compiled and executed under AC-2, so a re-blessed golden that changed an emitted operator, operand order or descriptor fails AC-2 rather than passing. | Test (TC-029) |
| FR-018-AC-11 | Every claim marks its operation `caller_declared`, the claim map carries the blocked item "operation identity not carried by CheckedPackage V2", and two requests over one node differing only in `EqualityOperator` both generate with that mark and produce complementary outcomes on a vector whose operands differ. | Test (TC-029) |
| FR-018-AC-12 | The generated crate declares `publish = false`, pins the runtime revision with the `exact` feature, contains no charge amount and no planned pair count (every charge and every pair comes from runtime metering), and compiles. | Test (TC-029) |
| FR-018-AC-13 | Every claim-map entry carries the node id, IR id, package id, source map, claims, reconstructed declaration keys and selected schedule of its item, and its declaration keys equal `NodeKey::from_hex` of the V2 node id digests its operand types reach. | Test (TC-029) |

### Mutations these criteria detect

Each criterion is recorded with the mutation it exists to catch; a criterion
without one is not written.

| ID | Mutation that breaks it |
|----|-------------------------|
| FR-018-AC-1 | Abort the whole request on the first refusal, or emit a stub function for a refused item. |
| FR-018-AC-2 | Emit `EqualityOperator::Equal` where the descriptor says `NotEqual`, or apply the right operand's conversion before the left's. |
| FR-018-AC-3 | Emit a comparison that visits a DAG-shared node once instead of per occurrence. |
| FR-018-AC-4 | Select `EqualitySchedule::Plan` for a top-level text pair; the charge sequence changes while the generated text still compiles. |
| FR-018-AC-5 | Generate the item and let the runtime refuse at evaluation time instead of refusing at generation time. |
| FR-018-AC-6 | Check `contains_ieee` only at the top level, so a nested `float64` field generates. |
| FR-018-AC-7 | Collapse the three blockers into one "unsupported" reason, or generate an oracle over a `reference` operand. |
| FR-018-AC-8 | Emit `TypeEnvironment::new(..).unwrap()` or `check_equality(..).expect(..)` into the generated crate. |
| FR-018-AC-9 | Emit closures returning plain `bool`, as `src/oracle.rs` does for Boolean connectives; a denial then has no representable result. |
| FR-018-AC-10 | Order claim-map entries by request order, or bless a golden whose emitted operator was changed. |
| FR-018-AC-11 | Mark the operation checked rather than `caller_declared`, or refuse the second descriptor as a duplicate. |
| FR-018-AC-12 | Emit the plan's pair count or a charge amount as a literal constant in the generated source. |
| FR-018-AC-13 | Key declarations by request ordinal instead of by the V2 node id digest. |

## Dependencies

- **Upstream**: [FR-001](../FR-001-deterministic-oracles.md),
  [FR-014](./FR-014-exact-scalar-oracles.md), Contract IR FR-036/FR-038
  (CheckedPackage V2 lowering), Contract Runtime FR-008 (composite, collection
  and equality families), quire-spec-language at the runtime's pinned authority.
- **Downstream**: [TC-029](../../test/complete-v1/TC-029-composite-equality-oracles.md).

## Out of Scope

- Function application oracles: the pinned runtime publishes no function
  application surface (agent-ix/quire-contract-runtime#34).
- Model graph, identity and reachability oracles
  (agent-ix/quire-spec-language#120), which FR-019 will own.
- Temporal and protocol oracles (agent-ix/quire-spec-language#121), which FR-020
  will own.
- Composite and collection *construction* oracles. This requirement generates the
  equality relation over completed operands; `construct_collection`,
  `form_collection`, `evaluate_record` and `evaluate_tuple` are the runtime's
  FR-008 constructors and are called by the corpus, not emitted.
