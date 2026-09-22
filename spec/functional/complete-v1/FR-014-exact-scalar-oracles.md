---
id: FR-014
title: "Generate exact complete-V1 scalar oracles"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-196
    type: references
  - target: ix://agent-ix/quire-specification/FR-333
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-036
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-038
    type: references
---
# FR-014: Generate exact complete-V1 scalar oracles

## Description

When a caller supplies an admitted `quire.checked-package/v2` package and a set
of scalar expression nodes, the code generator shall emit a deterministic Rust
oracle crate whose functions evaluate each node by calling the pinned
Contract Runtime `exact` operators and metering. It never approximates a
scalar family through another family and never copies a resource charge.

This is issue #48, the scalar slice of complete-V1 oracle generation. Its
composite/structural equality sibling is
[FR-018](./FR-018-composite-equality-oracles.md).

## Inputs

- An admitted `CheckedPackageV2` read through Contract IR's strict reader.
- A request of items, each naming one checked node id and one typed exact
  scalar operation descriptor.
- The pinned Contract Runtime revision with the `exact` feature.

The V2 transport carries each application term's `operation` member — its
catalogued identity, its laws and its mode — so the operation law (for example
which division law or which comparison) is present in the IR. This generator
classifies a body from its `term`, `operator` and `arguments` together with the
request item's own descriptor, and checks every descriptor parameter the IR
carries (integer, rational and decimal ranges, float rounding, text bounds)
against the node's reachable `bounded_domain` nodes. It then reads the node's
own `operation.identity` and compares it against the catalogued identity the
descriptor implies, together with the operation mode's value.

A claim whose node lowers and whose descriptor agrees with that identity marks
its operation `ir_confirmed` and carries no blocked item: quire-contract-ir
validated the identity against the closed
`quire.checked-operation-catalog/v1` at package admission, before this
generator saw the package, so a consumer may treat the operation identity, and
nothing else about the oracle, as checked. A descriptor naming a different
catalogued operation over the same bounds is not confirmed, even where the two
share one operand shape.

A claim this generator never confirms marks its operation `caller_declared` and
carries the typed blocked item: the identity the claim map reports is the
request item's own descriptor-derived identity, and downstream obligations must
not treat it as checked. A claim goes unconfirmed for one of three reasons:
this generator never inspected the node; it inspected the node and refused the
item with a typed reason; or it lowered the node and the descriptor and the
node disagreed — a descriptor naming a different catalogued operation, a
descriptor naming an operation the catalogue has no entry for, or one whose
implied identity matches while its law definition or its mode value is absent
from the node's own `operation.laws` and `operation.mode`. Only the third case
still generates an oracle; it is the confirmation that is withheld, not the
code.

A bound is read from the one reachable `bounded_domain` node on the node's
result type whose form matches the descriptor. Its body is an `aggregate` of
literals: integers as canonical decimal `integer` literals, spellings as `text`
literals. This encoding is defined by this generator, not by V2:

| Form | Members |
|------|---------|
| `integer_range` | lower, upper |
| `rational_range` | numerator lower, numerator upper, denominator lower, denominator upper |
| `decimal_range` | lower, upper, minimum scale, maximum scale, rounding |
| `float_rounding` | rounding |
| `text_bounds` | minimum scalars, maximum scalars, profile |

## Outputs

- A generated crate: `Cargo.toml` (`publish = false`, runtime pinned by
  revision with the `exact` feature) and `src/lib.rs` with one oracle function
  per supported item.
- A typed claim map with one entry per requested item: node id, Contract IR
  semantic id, package id, semantic type, source map, claims, bounds, the
  bounds its parameters were checked against, operation identity with its
  `ir_confirmed` or `caller_declared` provenance, and either the generated
  symbol or the typed refusal; plus the map-level blocked items.

## Behavior

- When generation begins, the generator shall lower every requested node
  through Contract IR's complete lowering with a scalar tag profile and select
  exactly one generated or refused disposition per item.
- When a requested item passes every check `check_item` makes against its
  descriptor, the generator shall emit an oracle function that calls the
  runtime operator named by the descriptor with a caller-supplied `Meter`.
- If a requested item fails any check `check_item` makes, then the generator
  shall return that item's typed refusal and emit no code for it, without
  changing any other item's output.
- If a node lowers, then the generator shall compare the node's own
  `operation.identity` against the catalogued identity its descriptor implies,
  and shall compare the descriptor's law definition and mode value against the
  node's `operation.laws` and `operation.mode`.
- If every one of those comparisons agrees, then the generator shall mark that
  claim's operation `ir_confirmed`.
- If any of those comparisons disagrees, then the generator shall generate the
  oracle the descriptor names and shall mark that claim's operation
  `caller_declared` with a typed blocked item, withholding the confirmation
  rather than the code.
- If the bound a descriptor parameter needs is missing, repeated, unreadable or
  unequal to the parameter, then the generator shall refuse the item with a
  typed reason.
- If an operand is neither a literal nor a reference, or a literal stands where
  a quantity is required, then the generator shall refuse the item with a typed
  reason.
- If a node belongs to the function family, then the generator shall refuse
  it as blocked on quire-contract-runtime#34; model and relation families as
  blocked on quire-spec-language#120; composite, collection, state, temporal
  and protocol families as unsupported.
- If a node id appears more than once in the request, then the generator shall
  refuse every copy.
- The generator shall order output by node id (digest domain, then digest) so
  that equal requests in any order produce identical bytes.
- If the generated source exceeds its size ceiling, then the generator shall
  return a typed error and no partial output.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-014-AC-1 | Every requested item receives exactly one generated or typed-refused disposition, and a refused item contributes no generated function while its siblings are generated unchanged. | Test (TC-024) |
| FR-014-AC-2 | Integer (add, subtract, multiply, negate, truncating/floor/Euclidean division, modulo), rational, ordering, decimal, IEEE arithmetic/comparison/width conversion, text admission/comparison, enum comparison, and quantity arithmetic/comparison/conversion descriptors each generate a function calling the matching runtime `exact` operator. | Test (TC-024) |
| FR-014-AC-3 | A descriptor whose arity, result or operand scalar type disagrees with the lowered node, a node that is not a lowered scalar expression, a node that is unbounded, has an invalid or incomplete body, or exhausts lowering work, or a node id requested more than once, is refused with a typed reason. | Test (TC-024) |
| FR-014-AC-4 | Generated bytes are identical across repeated runs and across permutations of the request order, match the committed golden output, and order claim-map entries by node id (domain, then digest). | Test (TC-024) |
| FR-014-AC-5 | Every claim-map entry carries the node id, IR id, package id, source map, claims, bounds and operation identity of its item. | Test (TC-024) |
| FR-014-AC-6 | Executing generated oracles on the QSpec TC-185, TC-186, TC-187, TC-192 and TC-193 vectors yields outcomes, charges and consumed counters equal to direct runtime execution and to the QSL value authority. | Test (TC-024) |
| FR-014-AC-7 | Composite, collection, function, model, relation, state, temporal and protocol nodes are refused with their blocked or unsupported reason and never reported as generated. | Test (TC-024) |
| FR-014-AC-8 | The generated crate declares `publish = false`, pins the runtime revision with the `exact` feature, contains no charge amount (every charge comes from runtime metering), and compiles. | Test (TC-024) |
| FR-014-AC-9 | Generated oracle functions do not panic: an invalid generated constant, including a decimal target, stops as `InvalidConstant`, an operand of the wrong width stops before any charge, and generated source over its ceiling is a typed error with no output. | Test (TC-024) |
| FR-014-AC-10 | A descriptor parameter whose bound is missing, repeated, unreadable or unequal to the reachable `bounded_domain` node, an operand that is neither a literal nor a reference, and a literal quantity operand, are each refused with a typed reason. | Test (TC-024) |
| FR-014-AC-11 | A claim whose descriptor passes every check `check_item` makes, and whose descriptor agrees with the node's catalogued `operation.identity`, law definition and mode value, marks its operation `ir_confirmed`. An item refused by any of those checks is `caller_declared` even where its operation agrees. | Test (TC-024) |
| FR-014-AC-12 | A descriptor naming a different catalogued operation than the node's own is not confirmed, including where the two share one operand shape and one checked bound; nor is a descriptor naming an operation the catalogue has no entry for. | Test (TC-024) |
| FR-014-AC-13 | A descriptor whose implied identity matches the node's but whose law definition is absent from that node's `operation.laws`, or whose mode value disagrees with that node's `operation.mode`, is not confirmed; the item still lowers and still generates the oracle its descriptor names, marked `caller_declared` with a typed blocked item. | Test (TC-024) |
| FR-014-AC-14 | A claim whose node this generator never inspected — a duplicate copy, or a record that never lowered — or inspected and refused with a typed reason, marks its operation `caller_declared` with a typed blocked item and reports the request item's own descriptor-derived identity. | Test (TC-024) |

## Dependencies

- **Upstream**: [FR-001](../FR-001-deterministic-oracles.md), Contract IR
  FR-036/FR-038 (CheckedPackage V2 lowering), Contract Runtime FR-007 (exact
  scalar operators), QSpec FR-196.
- **Downstream**: [TC-024](../../test/complete-v1/TC-024-exact-scalar-oracles.md),
  [FR-015](./FR-015-bounded-kani-obligations.md).
