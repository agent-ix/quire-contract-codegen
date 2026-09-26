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

A bound is read from a `bounded_domain` node whose form matches the descriptor,
and each role a node uses is identified by what the node itself names, not by a
package-wide "the one bound of this form". A `reference` operand whose target is
typed by a `bounded_domain` has that domain as its own bound. The result bound
is the domain typing the node's result when the node's `semantic_type` is a
`bounded_domain`; when it is a scalar type, the result bound is the one reachable
bound of the form over that type, and where several are reachable, the one that
is not the own bound of an operand. Bounds that no operand and no result names
never make the node ambiguous. Its body is an `aggregate` of
`binding` members, `{term: binding, name, value: <literal>}` (QSpec
`proposals/checked-package-v2/fixtures/positive-operation-identities.json`),
each carrying a canonical decimal `integer` literal or, for a spelling, a `text`
literal. A member is looked up by its `name`, in any order; the body holds
exactly the names of its form, once each:

| Form | Member names |
|------|--------------|
| `integer_range` | `min`, `max` |
| `rational_range` | `numerator_min`, `numerator_max`, `denominator_min`, `denominator_max` |
| `decimal_range` | `coefficient_min`, `coefficient_max`, `scale_min`, `scale_max`, `rounding` (text) |
| `float_rounding` | `rounding` (text) |
| `text_bounds` | `min`, `max`, `text_profile` (text) |
| `collection_bounds` | `min`, `max` |

A bare literal member, a missing or duplicate name and an unlisted name are each
an unreadable bound.

### Deriving the descriptor from the node

`derive_exact_scalar_items` builds each item's descriptor from the package
alone, so a caller that holds only node ids need not declare one. For each
node id it returns one `ExactScalarItem` or one `ExactScalarRefusal`, in
input order. It first checks that the node is in the graph and lowers, and
returns the refusal `generate_exact_scalar_oracles` gives such a node
(`InvalidInput`, `BlockedOnUpstream`, `Unsupported`, `RequiresBound`), so a
consumer classifies it as it always did. It then reads the node's
`operation.identity` and maps it to an operator by an exhaustive match over
identity strings. The descriptor's own identity check is the opposite
exhaustive match over descriptors; the two are kept aligned by a round-trip
test that walks every descriptor through both and cannot omit an
`ExactScalarOperation` variant. Its other sources are:

- the operand scalar forms, for the identities whose operator depends on them
  (`rational.div`: `IntegerDivide` when both operands are integers, `Divide`
  when both are rationals; the IEEE comparisons: the operand width);
- `operation.laws`, whose `definition.identity` selects the division profile
  of `integer.div`;
- `operation.mode`, whose rounding value must equal the rounding of the bound
  the operation is read against, and which supplies the rounding of an
  integer `quantity.convert` target;
- the one reachable bound of the needed form on the node's result type, read by
  the same selector the descriptor's parameter check uses: the integer, rational
  or decimal domain, the IEEE rounding, the text bounds or the decimal target.

The derivable operations are integer `add`, `sub`, `mul`, `negate`, `div` and
`mod`; rational `add`, `sub`, `mul`, `negate` and `div` (both operands
integers, or both rationals); `lt`, `le`, `gt` and `ge` over integers,
rationals and decimals; decimal `add`, `sub`, `mul`, `div` and `negate`;
`numeric.convert_rounding` to a decimal; IEEE `float32` and `float64` `add`,
`sub`, `mul` and `div`, `numeric_equal`, `total_order`, `bit_identical`,
`to_float32` and `to_float64`; every `text` and `enum` comparison;
`quantity` `add`, `sub`, `mul`, `div`, `pow`, every comparison and `convert`;
and `numeric.convert` to a text type. Every other operation identity is
refused as `OperationNotDerivable`. The catalogued identities refused this way
are `integer.eq`, `integer.ne`, `rational.eq`, `rational.ne`, `decimal.eq`,
`decimal.ne`, `integer.rem`, `ieee.float32.sqrt`, `ieee.float64.sqrt`,
`ieee.float32.fma`, `ieee.float64.fma`, `ieee.from_exact`, `ieee.to_rational`,
`numeric.narrow`, `rational.narrow`, `text.size` and every `boolean` identity
(each read against the operation catalog).

On the derived path the descriptor is built from the node's own identity, laws
and mode, so the comparison of the descriptor with the node always agrees and a
derived claim is never `caller_declared` by disagreement. One derived descriptor
can still fail its parameter check: `quantity.convert` to a rational result
derives `QuantityTarget::Exact`, for which the descriptor declares no domain,
and generation refuses it as `BoundMismatch`.

A claim for a node derivation refused has operation provenance `underived`: no
descriptor was built, so no operation was declared or confirmed. Its identity is
the node's own `operation.identity` when the node carries one, and empty
otherwise.

## Outputs

- A generated crate: `Cargo.toml` (`publish = false`, runtime pinned by
  revision with the `exact` feature) and `src/lib.rs` with one oracle function
  per supported item.
- `derive_exact_scalar_items`: one `Result<ExactScalarItem, ExactScalarRefusal>`
  per requested node id, in input order. A refusal is a lowering or bound
  refusal `generate_exact_scalar_oracles` also gives, or `NoDerivableClaim`
  carrying a `ClaimDerivationRefusal`, a tagged `code` enum:
  `not_application`, `missing_operation_identity`, `operation_not_derivable`,
  `operand_forms_not_derivable`, `law_not_derivable` and `mode_not_derivable`.
- A typed claim map with one entry per requested item: node id, Contract IR
  semantic id, package id, semantic type, source map, claims, bounds, the
  bounds its parameters were checked against, operation identity with its
  `ir_confirmed` or `caller_declared` provenance, and either the generated
  symbol or the typed refusal; plus the map-level blocked items.

## Behavior

- When generation begins, the generator shall lower every requested node
  through Contract IR's complete lowering with a scalar tag profile and select
  exactly one generated or refused disposition per item.
- When a requested item is requested exactly once, passes every check this
  requirement states, and its node carries the catalogued `operation.identity`
  the admission guarantees, the generator shall emit an oracle function that
  calls the runtime operator named by the descriptor with a caller-supplied
  `Meter`.
- If a requested item fails any of the checks this requirement states, then
  the generator shall return the `ExactScalarRefusal` variant naming that
  failure — see `ExactScalarRefusal` for the closed set of checks and their
  reasons — and emit no code for it, without changing any other item's
  output.
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
- Where an integer node's result is typed by a `bounded_domain`, or its
  operands are typed by distinct `integer_range` `bounded_domain` nodes, the
  generator shall compare the descriptor's domain with the result bound only.
- Where an integer operand is typed by an `integer_range` `bounded_domain`, the
  generator shall record that bound among the claim's checked bounds after the
  result bound.
- If the result of an integer node is typed by a `bounded_domain` of a form
  other than the descriptor's, or an operand is typed by one, then the
  generator shall refuse the item as `MissingBound`.
- If an operand's own bound is not contained in the descriptor's domain, then
  the generator shall refuse the item as `BoundMismatch` naming that operand's
  bound.
- If a node with a scalar-typed result reaches two or more bounds of the form
  and not exactly one of them is outside the operands' own bounds, then the
  generator shall refuse the item as `AmbiguousBound`.
- Where a `reference` operand's target is typed by a `bounded_domain` node, the
  generator shall classify the operand by that domain's own `semantic_type`, its
  base scalar type; QSL emits a bounded domain directly over its scalar base.
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
- When a node id is passed to `derive_exact_scalar_items`, the generator shall
  read the node's `operation.identity`, `operation.laws`, `operation.mode`
  and operand forms and the one result bound, and return the descriptor they
  determine.
- If a node is absent, does not lower or has a refused bound, then the
  generator shall return the `ExactScalarRefusal` `generate_exact_scalar_oracles`
  gives it, and no descriptor.
- If a node has no derivable descriptor for another reason, then the generator
  shall return `NoDerivableClaim` carrying the `ClaimDerivationRefusal` naming
  why (a node that is not an application, an absent identity, an identity
  outside the derivable set, operand forms that do not select an operator, or a
  law or mode that does not select a parameter), and no descriptor.
- If the generated source exceeds its size ceiling, then the generator shall
  return a typed error and no partial output.
- If lowering a requested node's closure consumes more than 65,536 metered work
  units, then the generator shall refuse the item as `LoweringWorkExhausted`,
  naming the ceiling and the counter at the failed charge, contribute no
  generated function for it, and leave every other item's disposition
  unaffected.

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
| FR-014-AC-11 | A claim whose descriptor passes every check this requirement states, and whose descriptor agrees with the node's catalogued `operation.identity`, law definition and mode value, marks its operation `ir_confirmed`. An item refused by any of those checks is `caller_declared` even where its operation agrees. | Test (TC-024) |
| FR-014-AC-12 | A descriptor naming a different catalogued operation than the node's own is not confirmed, including where the two share one operand shape and one checked bound; nor is a descriptor naming an operation the catalogue has no entry for. | Test (TC-024) |
| FR-014-AC-13 | A descriptor whose implied identity matches the node's but whose law definition is absent from that node's `operation.laws`, or whose mode value disagrees with that node's `operation.mode`, is not confirmed; the item still lowers and still generates the oracle its descriptor names, marked `caller_declared` with a typed blocked item. | Test (TC-024) |
| FR-014-AC-14 | A claim whose node this generator never inspected — a duplicate copy, or a record that never lowered — or inspected and refused with a typed reason, marks its operation `caller_declared` with a typed blocked item and reports the request item's own descriptor-derived identity. | Test (TC-024) |
| FR-014-AC-15 | A requested item whose node closure exceeds the 65,536-unit lowering work ceiling is refused as `LoweringWorkExhausted`, naming the ceiling and the consumed counter, contributes no generated function, and leaves every other item's disposition unaffected. | Test (TC-024) |
| FR-014-AC-16 | A bound is read from `binding` members looked up by name in any order (`min`/`max`, `numerator_min` through `denominator_max`, `coefficient_min` through `rounding`, `rounding`, `min`/`max`/`text_profile`); a bare literal member, a missing, duplicate or unlisted name is refused as an unreadable bound. | Test (TC-024) |
| FR-014-AC-17 | A `reference` operand whose target is typed by a `bounded_domain` node is classified by that domain's base scalar type, so `x + 1` over a parameter `x` of type `Int[0, 9]` generates, and a bounded text parameter is still a text operand. | Test (TC-024) |
| FR-014-AC-18 | `derive_exact_scalar_items` returns, per node id in input order, the descriptor determined by the node's `operation.identity`, `operation.mode`, `operation.laws`, operand forms and the one bound of the needed form on its result type; a node that is not an application, has no identity, names an identity outside the derivable set, has operand forms, a law or a mode that select no parameter, is refused with the matching `ClaimDerivationRefusal` and no descriptor; a node that is absent, does not lower or has a refused bound (missing, repeated or unreadable) is refused with the `ExactScalarRefusal` generation gives it. | Test (TC-024) |
| FR-014-AC-19 | A derived item generates an oracle whose operation is `ir_confirmed`, and for every item of the golden corpus the derived descriptor equals the descriptor the fixture declares. | Test (TC-024) |
| FR-014-AC-20 | An integer node over two distinct bounded parameters, `Int[0, 9]` and `Int[10, 20]`, each typed by its own `integer_range` `bounded_domain`, with its result typed by a third `integer_range` `[0, 29]`, generates an `ir_confirmed` oracle whose descriptor is compared with the result bound only; the claim's checked bounds are the result bound, then each operand's own bound; derivation returns the descriptor over `[0, 29]`. | Test (TC-024) |
| FR-014-AC-21 | An operand bound not contained in the descriptor's domain is refused as `BoundMismatch` naming the operand's bound; an operand or result typed by a `bounded_domain` of another form is refused as `MissingBound`; a scalar-typed result reaching two bounds of the form that no operand types is `AmbiguousBound`, as is one whose two reachable bounds are both operand bounds. | Test (TC-024) |

## Dependencies

- **Upstream**: [FR-001](../FR-001-deterministic-oracles.md), Contract IR
  FR-036/FR-038 (CheckedPackage V2 lowering), Contract Runtime FR-007 (exact
  scalar operators), QSpec FR-196.
- **Downstream**: [TC-024](../../test/complete-v1/TC-024-exact-scalar-oracles.md),
  [FR-015](./FR-015-bounded-kani-obligations.md).
