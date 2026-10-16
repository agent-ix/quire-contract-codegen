---
id: FR-014
title: "Generate exact complete-V1 scalar oracles"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
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
oracle crate whose functions evaluate each node by calling the
Contract Runtime `exact` operators and metering. It never approximates a
scalar family through another family and never copies a resource charge.

FR-014 is the one oracle generator for every family it covers, the Boolean
connectives and the bounded-integer comparisons included
([ADR-001](../../decisions/ADR-001-overlapping-generators-and-input-models.md)
Q2). Its composite/structural equality sibling is
[FR-018](./FR-018-composite-equality-oracles.md).

FR-014 owns the integer and Boolean `eq` and `ne` nodes. Their oracles call the
runtime's one equality relation, `TypeEnvironment::check_equality` followed by
`CheckedEquality::evaluate`, as FR-018's oracles do, and return its
`Outcome<bool>` unchanged; no oracle compares two values itself. A Boolean
connective's oracle calls `exact::evaluate_boolean` with the matching
`BooleanConnective` when both operands are decided values, and
`exact::evaluate_boolean_short_circuit` when its right operand is itself an
oracle that may stop, and returns the runtime's `Outcome<bool>` unchanged.

The checked-package expression route includes catalogued text comparisons. Text
admission is a separate caller-ingress operation: a caller constructs a
`TextPayload` and admits it under a `TextType` before any comparison. No checked
`quire.op.numeric.convert` node represents this operation; that identity has an
`exact_numeric` operand. A generated comparison oracle does not certify how its
caller obtained either `Text` operand.

## Inputs

- An admitted `CheckedPackageV2` read through Contract IR's strict reader.
- A request of items, each naming one checked node id and one typed exact
  scalar operation descriptor.
- Contract Runtime with the `exact` feature, as this repository's `Cargo.toml`
  names it.

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
`bounded_domain`; when it is a scalar type and only narrowing conversions consume
the node, the result bound is the one `bounded_domain` they narrow it to, and a
node that a narrowing conversion and any other node both consume has no result
bound, since the narrow's bound does not bound the other use. QSL emits an
expression in a bounded context this way: the arithmetic node's own type is the
plain scalar type, and the declared bound types a `conversion` node whose body
is a `convert` application with identity `quire.op.numeric.narrow` and exactly
one argument, a `reference` to the node. Otherwise the result bound is the one
reachable bound of the form over that type, and where several are reachable,
the one that is not the own bound of an operand. The descriptor is compared with the result
bound only; each operand's own bound is recorded after it and ranges that
operand in a Kani harness, and need not lie inside the result bound. A
`reference` operand typed by a plain scalar type owns no bound: where another
operand owns one, or the result is typed by one, it is `RequiresBound` naming
its type, and a literal operand, being a constant, is never `RequiresBound`. A
`reference` to a `value` node whose body is a literal is a literal operand: QSL
emits a literal operand that way, typed by the plain scalar type, and it is the
referenced node's body that decides it, not its type or form. A literal operand
owns no bound: a Kani harness constrains it to exactly its own value (FR-015),
and one that is not a canonical `integer` literal is refused for an integer
operation whatever the referenced node's type. A bound's body is an `aggregate` of
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

For an integer operand that references an `expression` application with a plain
`Integer` result, the generator may establish a finite operand interval
independently of a result-domain node. It walks only `quire.op.integer.add`,
`sub`, `mul` and `negate` application arguments. A parameter is a leaf only
when its own type is an `integer_range` `bounded_domain`; an inline canonical
integer literal or a reference to a `value` node with a canonical integer
literal body is a singleton leaf. An eligible application is evaluated
recursively. The interval conservatively contains every mathematical result
for the referenced application; addition and subtraction use endpoint sums and
differences, multiplication uses the minimum and maximum of all four endpoint
products, and negate maps `[a, b]` to `[-b, -a]`. Shared or correlated leaves
may make the interval wider than the set of reachable results (for example,
`x - x` over `[0, 9]` has inferred interval `[-9, 9]`). QSL's negate application
over the literal `1` therefore has interval `[-1, -1]`. Negation is exact
before any `i64` conversion: `-0` yields `[0, 0]` and a canonical positive
literal `9223372036854775808` yields the singleton
`[-9223372036854775808, -9223372036854775808]`, which fits `i64` after
negation. An inferred interval
is recorded by operand position with its source node id and endpoints, not as
a fictitious `bounded_domain` or checked bound id.

The walk shall visit at most 4,096 distinct application/value nodes, descend
at most 64 application levels, and keep every input and calculated endpoint
at most 4,096 bits in magnitude (`0` has bit width zero; a negative
endpoint's bit width is that of its absolute value). Exceeding a limit is the typed
`OperandRangeLimitExceeded` refusal naming the operand node, limit kind,
ceiling and consumed amount. The limit kinds are `depth`, `nodes` and
`endpoint_bits`; `consumed` is the first count or bit width over the ceiling.
The walk never truncates an interval. A plain `Integer` parameter at any
depth is `RequiresBound` naming its type; a wrong-kind literal is
`OperandTypeMismatch` naming the outer argument position and literal kind;
an unsupported nested operation is `OperandUnsupported` naming the outer
argument position and nested operation identity. A malformed literal or
reference, a missing node, or a cycle keeps the typed IR admission or
lowering refusal before inference; an inference seam presented with one of
these malformed shapes returns `InvalidBody` naming its first offending
node. No conversion
that consumes the referenced application supplies this operand interval. A
conversion's result may instead be ranged by its own `bounded_domain` when
that conversion itself is the operand; a direct reference to the raw
application is ranged only from independently bounded leaves. The
`AmbiguousBound` rule for an application consumed by both a narrowing
conversion and another node remains in force when that application itself is
requested. Inference does not replace the requested outer node's result bound
or relax its descriptor comparison.

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
  the operation is read against;
- the one reachable bound of the needed form on the node's result type, read by
  the same selector the descriptor's parameter check uses: the integer, rational
  or decimal domain, the IEEE rounding, the text bounds or the decimal target.

The derivable operations are integer `add`, `sub`, `mul`, `negate`, `div`,
`mod`, `eq` and `ne`; the Boolean `and`, `or`, `not`, `implies`, `eq` and `ne`;
rational `add`, `sub`, `mul`, `negate` and `div` (both operands
integers, or both rationals); `lt`, `le`, `gt` and `ge` over integers,
rationals and decimals; decimal `add`, `sub`, `mul`, `div` and `negate`;
`numeric.convert_rounding` to a decimal; IEEE `float32` and `float64` `add`,
`sub`, `mul` and `div`, `numeric_equal`, `total_order`, `bit_identical`,
`to_float32` and `to_float64`; every `text` and `enum` comparison;
`quantity` `add`, `sub`, `mul`, `div`, `pow` and every comparison.
`quire.op.quantity.convert` is not derivable: it has its own refusal,
`QuantityConversionUnsupported`, which the Behavior section and FR-014-AC-2
state. Every other operation identity is refused as `OperationNotDerivable`. The catalogued identities refused this way
are `rational.eq`, `rational.ne`, `decimal.eq`, `decimal.ne`, `integer.rem`,
`ieee.float32.sqrt`, `ieee.float64.sqrt`, `ieee.float32.fma`,
`ieee.float64.fma`, `ieee.from_exact`, `ieee.to_rational`, `numeric.narrow`,
`rational.narrow` and `text.size` (each read against the operation catalog).

On the derived path the descriptor is built from the node's own identity, laws
and mode, so the comparison of the descriptor with the node always agrees and a
derived claim is never `caller_declared` by disagreement.

A claim for a node derivation refused has operation provenance `underived`: no
descriptor was built, so no operation was declared or confirmed. Its identity is
the node's own `operation.identity` when the node carries one, and empty
otherwise.

## Outputs

- A generated crate: `Cargo.toml` (`publish = false`, the Contract Runtime
  dependency this repository's `Cargo.toml` names, with the `exact` feature) and `src/lib.rs` with one oracle function
  per supported item.
- A coverage-probe source map for the generated crate.
- `derive_exact_scalar_items`: one `Result<ExactScalarItem, ExactScalarRefusal>`
  per requested node id, in input order. A refusal is a lowering or bound
  refusal `generate_exact_scalar_oracles` also gives, or `NoDerivableClaim`
  carrying a `ClaimDerivationRefusal`, a tagged `code` enum:
  `not_application`, `missing_operation_identity`, `operation_not_derivable`,
  `operand_forms_not_derivable`, `law_not_derivable`, `mode_not_derivable` and
  `quantity_conversion_unsupported`.
- A typed claim map with one entry per requested item: node id, Contract IR
  semantic id, package id, semantic type, source map, claims, bounds, the
  bounds its parameters were checked against, operation identity with its
  `ir_confirmed` or `caller_declared` provenance, and either the generated
  symbol or the typed refusal; plus the map-level blocked items.

## Behavior

- Before production generation begins, the driver shall lower the full request
  once through Contract IR with CG's public routed lowering profile and admit
  the resulting FR-330 package. The generator shall consume that package's
  ordered accounting and reached graph, selecting exactly one generated or
  refused disposition per requested item without lowering again (FR-039).
  A `lowered` package record may still receive this generator's typed
  `RequiresBound` refusal for an unbounded scalar operand.
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
- If a requested node's `operation.identity` is `quire.op.quantity.convert`,
  then every entry point that can receive it shall refuse it with
  `ClaimDerivationRefusal::QuantityConversionUnsupported`, naming the operation
  identity and `QUANTITY_CONVERSION_CAPABILITY`, and shall produce no
  descriptor, no oracle function and no harness for it. The entry points are
  `derive_exact_scalar_items`; `generate_exact_scalar_oracles` and
  `plan_exact_scalar_oracles` under any descriptor the caller declares; the
  routed contract-package path of `generate_routed`;
  `negotiate_kani_obligations`; and backend capability settlement. The refusal
  precedes every check against a caller's descriptor. `generate_exact_function_oracles` and the function schedules
  (`generate_function_schedules` and `generate_function_schedules_with_limits`)
  are entry points of this statement: a conversion node reached from a declared
  function body, from a requested call, or as an argument of a call is refused
  with the capability, whatever the declared body kind, parameter types or
  result type; the function generator records it on the item's claim and the
  schedules record it on the refused function or the refused item. The
  composite-equality generator is the one exemption: it refuses a conversion
  node it is asked to generate as `FormMismatch` naming `conversion`, and a
  quantity equality that reads a conversion node as an operand (FR-018-AC-15)
  as `Unsupported` with node tag `quantity`, each before any artifact. The capability text
  states that quantity conversion is not supported because the runtime converts
  a quantity only to a scalar target and has no conversion whose result is a
  quantity, and that real support is tracked as IR-818.
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
  result bound, whether or not it lies inside the result bound.
- If the result of an integer node is typed by a `bounded_domain` of a form
  other than the descriptor's, or an operand is typed by one, then the
  generator shall refuse the item as `MissingBound`.
- If a node with a scalar-typed result reaches two or more bounds of the form
  and not exactly one of them is outside the operands' own bounds, then the
  generator shall refuse the item as `AmbiguousBound`.
- If an integer operand is a `reference` typed by a plain scalar type and
  another operand owns a bound or the result is typed by one, then the
  generator shall refuse the item as `RequiresBound` naming the operand's type.
- Where a `reference` operand's target is a `value` node whose body is a
  literal, the generator shall treat the operand as a literal.
- If a literal operand of an integer operation, inline or referenced, is not a
  canonical `integer` literal, then the generator shall refuse the item as
  `OperandTypeMismatch` naming the operand's position and its literal kind.
- Where a node's result is typed by a scalar type and narrowing conversions
  consume the node, the generator shall take the one bound they narrow it to as
  the result bound, record it first among the checked bounds, and compare the
  descriptor with it.
- If the narrowing conversions consuming a node narrow it to two or more
  distinct bounds, or a narrowing conversion and a node that is not one both
  consume it, then the generator shall refuse the item as `AmbiguousBound`; if
  they narrow it to one bound of another form, or of the form over another
  scalar type, then as `MissingBound`.
- Where a `reference` operand's target is typed by a `bounded_domain` node, the
  generator shall classify the operand by that domain's own `semantic_type`, its
  base scalar type; QSL emits a bounded domain directly over its scalar base.
- Where an integer operand references an eligible plain-Integer application,
  the generator shall compute its inclusive interval from independently
  bounded leaves by the conservative endpoint rules above and record that interval
  by operand position for FR-015. It shall neither borrow a consuming
  narrowing conversion's domain for a direct reference to the application nor
  classify a plain-Integer parameter as a bounded application.
- If interval inference reaches an unbounded parameter, an unsupported or
  cyclic application, a missing referenced node, a wrong-kind or noncanonical
  literal, or an inference resource ceiling, then the generator shall refuse
  with the typed reason assigned above and emit no oracle for the item.
- If an operand is neither a literal nor a reference, or a literal stands where
  a quantity is required, then the generator shall refuse the item with a typed
  reason.
- If a node belongs to the function family, then the generator shall refuse
  it as blocked on quire-contract-runtime#34; model and relation families as
  blocked on quire-spec-language#120; composite, collection, state, temporal
  and protocol families as unsupported.
- When an admitted `temporal`/`formula`, `temporal`/`fairness` or
  `expression`/`case` node is requested directly, the scalar generator shall
  retain that requested node id in one claim-map entry and refuse the item as
  `ExactScalarRefusal::Unsupported` without emitting its oracle. The refusal
  retains the first unsupported reachable node and tag returned by IR
  lowering; that cause can differ from the requested root. If a `case` root
  lowers despite its reachable types, the scalar generator shall refuse that
  root as unsupported with the root id and `expression` tag, rather than
  treating its `case` form as a scalar body mismatch. Derivation and routed
  Kani shall keep their existing typed underivable or unsupported outcomes
  for the same requested node.
- If a node id appears more than once in the request, then the generator shall
  refuse every copy.
- The generator shall order output by node id so
  that equal requests in any order produce identical bytes.
- When a node id is passed to `derive_exact_scalar_items`, the generator shall
  read the node's `operation.identity`, `operation.laws`, `operation.mode`
  and operand forms and the one result bound, and return the descriptor they
  determine.
- If a node is absent, does not lower or has a refused bound, then the
  generator shall return the `ExactScalarRefusal` `generate_exact_scalar_oracles`
  gives it, and no descriptor.
- If a node has no derivable descriptor for another reason, then the
  generator shall return `NoDerivableClaim` carrying the `ClaimDerivationRefusal`
  naming why (a node that is not an application, an absent identity, an identity
  outside the derivable set, operand forms that do not select an operator, or a
  law or mode that does not select a parameter), and no descriptor.
- If the generated source exceeds its size ceiling, then the generator shall
  return a typed error and no partial output.
- If lowering a requested node's closure consumes more than 65,536 metered work
  units, then the generator shall refuse the item as `LoweringWorkExhausted`,
  naming the ceiling and the counter at the failed charge, contribute no
  generated function for it, and leave every other item's disposition
  unaffected. `LoweringWorkExhausted` is the refusal of the work ceiling and of
  nothing else.
- Lowering a requested node can also return a `failed` record whose `limit_kind`
  is `bytes`. Contract IR's FR-038-AC-95 fixes when it does, what its `limit` and
  `consumed` are (`consumed` saturating at `u64::MAX`) and whether one node or
  every requested node fails. The generator shall refuse each item whose record
  is such a failure as `LoweringByteLimitExceeded`, carrying the record's `limit`
  and `consumed` unchanged, and contribute no generated function for it; an item
  whose record did not fail keeps its own disposition.
- If a `failed` record's `limit_kind` is neither `work` nor `bytes`, then the
  generator shall refuse the item as `LoweringLimitUnrecognised`, carrying the
  kind as `limit_kind`, a `&'static str` that is the snake_case of the
  `CheckedPackageLimit` variant's name (`nodes`, `edges`, `occurrences`
  or `diagnostics`; the enum has no wire name of its own) and serialises as that
  string, with the record's `limit` and `consumed`, and shall not panic, not
  report the record as work exhaustion and not drop the item.
- The generator shall read a `failed` record's `limit_kind` in exactly one place,
  `classify_lowering_failure` of the shared `oracle` module, which FR-018 and FR-021 also call, so
  the three generators cannot disagree about which kind is which. Each generator
  carries the classifier's three outcomes as `LoweringWorkExhausted`,
  `LoweringByteLimitExceeded` and `LoweringLimitUnrecognised` variants of its own
  refusal enum, with the same fields.
- How `negotiate` reports an `ExactScalarRefusal` of either new kind is FR-015's
  (FR-015-AC-50).
- When a node is a Boolean connective (`and`, `or`, `not`, `implies`), the
  generator shall derive its descriptor and generate an oracle that calls
  `exact::evaluate_boolean` or, where the right operand may stop,
  `exact::evaluate_boolean_short_circuit`, and returns its `Outcome<bool>`.
- When a node is an integer or Boolean `eq` or `ne`, the generator shall derive
  its descriptor and generate an oracle that calls
  `TypeEnvironment::check_equality` and `CheckedEquality::evaluate` and returns
  its `Outcome<bool>`.
- The generator shall emit, for each oracle, one evaluation-entry probe on its
  function-entry line and one entry-token probe inside the exact source region
  of each implication consequent, and shall declare the consequent count
  derived from the node, independent of the emitted region list.
- Planned (IR-489, IR-815): where FR-015 names a V2 clause claim, or FR-002 (IR-815) selects a V2
  precondition or postcondition for its harness, the generator shall derive a
  descriptor and generate an oracle for the Boolean condition of that
  `state`/`state_clause` node, an inline term with no node id of its own, addressed by
  the clause node id, without admitting the `state` node itself as a claimed node
  (FR-014-AC-7 stands). Its field reads are `project(deref(self), field)` terms and
  `pre(...)` of them, and FR-014-AC-10 stays in force for every other operand
  (FR-014-AC-38).
- The scalar generator, `src/oracle/scalar/mod.rs`, and every crate it emits shall
  contain no panic token in non-test code: `unwrap`, `expect`, `unwrap_unchecked`,
  `unwrap_err`, `expect_err`, `unwrap_err_unchecked`, `panic_any` or `resume_unwind`
  however written, the macros `panic`, `unreachable`, `todo`, `unimplemented`, `assert`,
  `assert_eq`, `assert_ne`, `debug_assert`, `debug_assert_eq` or `debug_assert_ne`, or
  `abort` anywhere but as a method call, so `process::abort` is a panic token
  (FR-014-AC-39).

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-014-AC-1 | Every requested item receives exactly one generated or typed-refused disposition, and a refused item contributes no generated function while its siblings are generated unchanged. | Test (TC-024) |
| FR-014-AC-2 | Integer (add, subtract, multiply, negate, truncating/floor/Euclidean division, modulo), rational, ordering, decimal, IEEE arithmetic/comparison/width conversion, text comparison, enum comparison, and quantity arithmetic/comparison descriptors for admitted checked-package expression nodes each generate a function calling the matching runtime `exact` operator. Each entry point of the quantity-conversion Behavior statement refuses a `quire.op.quantity.convert` node with `QuantityConversionUnsupported` carrying `QUANTITY_CONVERSION_CAPABILITY` and produces no descriptor, oracle function or harness, so the artifact set for that node is empty (real support is tracked as IR-818). The function-application generator and the function schedules refuse a conversion reached from a body, a call or a call argument with `QuantityConversionUnsupported` carrying the capability (`ExactFunctionRefusal` on the item's claim; `ScheduleRefusal` on the refused function or item), producing no oracle, schedule or location entry. The composite-equality generator never receives the operation as a requested root or as an operand: it refuses it earlier, as `FormMismatch` naming `conversion` for a root and `Unsupported` with node tag `quantity` for an operand, before any artifact. The Kani frame comparison classifier (`StateComparison::from_operation`), the collection predicate reader (`resolve_predicate`), the replay composite-equality builder (`UnsupportedNodeOperation`) and the Kani scalar renderer (`ScalarOperation::of`) each read a closed set of operation identities and refuse a conversion before rendering (`a_quantity_conversion_is_not_a_state_comparison`, `a_quantity_conversion_predicate_is_an_unsupported_shape`, `a_quantity_conversion_is_an_unsupported_node_operation` and `a_quantity_conversion_has_no_scalar_harness_renderer`, in-module unit tests asserting `None`, `UnsupportedShape`, `UnsupportedNodeOperation` and `None`). The QSpec TC-187 vectors that convert a quantity (U01, U04, U07, U08, U09b, U10, U15 to U18, U22, U25, U26 and U29) have no oracle under this criterion; the tests exercise the three corpus conversion nodes, not the individual vectors. | Test (TC-024: `tc_024_quantity_conversion_is_refused_with_its_named_capability` for `derive_exact_scalar_items`; `tc_024_caller_declared_quantity_conversion_is_refused_and_generates_nothing` for `generate_exact_scalar_oracles` and `plan_exact_scalar_oracles`; `tc_024_function_generator_refuses_a_function_whose_body_is_a_quantity_conversion`, `tc_024_function_generator_refuses_a_call_passing_a_quantity_conversion_as_an_argument`, `tc_024_function_schedules_refuse_a_quantity_conversion_body_with_the_named_capability` and `tc_024_function_schedule_item_with_a_conversion_argument_is_refused_with_the_named_capability` for the function generator and the schedules; `tc_024_equality_generator_refuses_a_quantity_conversion_node_before_any_artifact` and `tc_024_equality_over_a_conversion_operand_is_refused_as_an_unsupported_quantity` for the exempt equality generator; `a_selected_quantity_conversion_settles_as_unsupported_with_its_named_capability` for capability settlement. TC-033: `tc_033_a_quantity_conversion_is_unsupported_with_its_named_capability_and_no_harness` for `generate_routed`. TC-025: `tc_025_a_quantity_conversion_claim_is_unsupported_with_its_named_capability_and_no_harness` for `negotiate_kani_obligations`.) |
| FR-014-AC-3 | A descriptor whose arity, result or operand scalar type disagrees with the lowered node, a node that is not a lowered scalar expression, a node that is unbounded, has an invalid or incomplete body, or exhausts lowering work, or a node id requested more than once, is refused with a typed reason. | Test (TC-024) |
| FR-014-AC-4 | Generated bytes are identical across repeated runs and across permutations of the request order; the generated crate is exactly `Cargo.toml`, `src/lib.rs` and `claim-map.json`, its `claim-map.json` equals the returned claim map, every generated claim's oracle symbol is defined in `src/lib.rs`, and claim-map entries are ordered by node id. | Test (TC-024) |
| FR-014-AC-5 | Every claim-map entry carries the node id, IR id, package id, source map, claims, bounds and operation identity of its item. | Test (TC-024) |
| FR-014-AC-6 | Executing generated oracles on the QSpec TC-185, TC-186, TC-192 and TC-193 vectors, and on these QSpec TC-187 vectors, yields outcomes, charges and consumed counters equal to direct runtime execution: U03 (the dimension mismatch), U13 and U21 (powers), U19 and U24 (same-unit sums), U23 (comparison tuples) and U28 (the compound product), each adapted to the generated oracle's limits tuple, plus the generated grid of quantity arithmetic, comparisons and powers. The other TC-187 vectors are not covered by this criterion: the vectors that convert a quantity (U01, U04, U07, U08, U09b, U10, U15 to U18, U22, U25, U26 and U29) have no oracle because FR-014-AC-2 refuses them (real support is tracked as IR-818), and U02, U05, U06, U09, U11, U12, U14, U20 and U27 are unrun (IR-823). Agreement with the QSL value authority is not claimed by this criterion; IR-823 adds it. A vector or row of this criterion that yields no agreement fails the test. | Test (TC-024) |
| FR-014-AC-7 | Composite, collection, function, model, relation, state, temporal and protocol nodes are refused with their blocked or unsupported reason and never reported as generated. | Test (TC-024) |
| FR-014-AC-8 | The generated crate declares `publish = false`, names the Contract Runtime dependency this repository's `Cargo.toml` names with the `exact` feature, contains no charge amount (every charge comes from runtime metering), and compiles. | Test (TC-024) |
| FR-014-AC-9 | Generated oracle functions do not panic: an invalid generated constant, including a decimal target, stops as `InvalidConstant`, an operand of the wrong width stops before any charge, and generated source over its ceiling is a typed error with no output. | Test (TC-024) |
| FR-014-AC-10 | A descriptor parameter whose admitted bound is missing, repeated or unequal to the reachable `bounded_domain` node, an operand that is neither a literal nor a reference, and a literal quantity operand, are each refused by CG with a typed reason. A malformed bound with a noncanonical integer endpoint such as `05` is refused by Contract IR's strict checked-package reader before CG receives an admitted item; CG retains its defensive typed `UnreadableBound` refusal for an admitted bound it cannot decode. Contract IR's FR-038 "Mode" reader rule also refuses a bare Float32 addition authored with the non-exact `nearest-even` mode and no `float_rounding` type pin as `invalid_package`/`operation-mode-type-mismatch` at `operation.mode/value`, before CG can classify a missing rounding bound. | Test (TC-024) |
| FR-014-AC-11 | A claim whose descriptor passes every check this requirement states, and whose descriptor agrees with the node's catalogued `operation.identity`, law definition and mode value, marks its operation `ir_confirmed`. An item refused by any of those checks is `caller_declared` even where its operation agrees. | Test (TC-024) |
| FR-014-AC-12 | A descriptor naming a different catalogued operation than the node's own is not confirmed, including where the two share one operand shape and one checked bound; nor is a descriptor naming an operation the catalogue has no entry for. | Test (TC-024) |
| FR-014-AC-13 | A descriptor whose implied identity matches the node's but whose law definition is absent from that node's `operation.laws`, or whose mode value disagrees with that node's `operation.mode`, is not confirmed; the item still lowers and still generates the oracle its descriptor names, marked `caller_declared` with a typed blocked item. | Test (TC-024) |
| FR-014-AC-14 | A claim whose node this generator never inspected — a duplicate copy, or a record that never lowered — or inspected and refused with a typed reason, marks its operation `caller_declared` with a typed blocked item and reports the request item's own descriptor-derived identity. | Test (TC-024) |
| FR-014-AC-15 | A requested item whose node closure exceeds the 65,536-unit lowering work ceiling is refused as `LoweringWorkExhausted`, naming the ceiling and the consumed counter, contributes no generated function, and leaves every other item's disposition unaffected. | Test (TC-024) |
| FR-014-AC-16 | A bound is read from `binding` members looked up by name in any order (`min`/`max`, `numerator_min` through `denominator_max`, `coefficient_min` through `rounding`, `rounding`, `min`/`max`/`text_profile`); a bare literal member, a missing, duplicate or unlisted name is refused as an unreadable bound. | Test (TC-024) |
| FR-014-AC-17 | A `reference` operand whose target is typed by a `bounded_domain` node is classified by that domain's base scalar type, so `x + 1` over a parameter `x` of type `Int[0, 9]` generates, and a bounded text parameter is still a text operand. | Test (TC-024) |
| FR-014-AC-18 | `derive_exact_scalar_items` returns, per node id in input order, the descriptor determined by the node's `operation.identity`, `operation.mode`, `operation.laws`, operand forms and the one bound of the needed form on its result type; a node that is not an application, has no identity, names an identity outside the derivable set (FR-014-AC-2 owns the `quire.op.quantity.convert` refusal), has operand forms, a law or a mode that select no parameter, is refused with the matching `ClaimDerivationRefusal` and no descriptor; a node that is absent, does not lower or has a refused bound (missing, repeated or unreadable) is refused with the `ExactScalarRefusal` generation gives it. | Test (TC-024) |
| FR-014-AC-19 | A derived item generates an oracle whose operation is `ir_confirmed`, and for every item of the corpus the derived descriptor equals the descriptor the fixture declares. | Test (TC-024) |
| FR-014-AC-20 | An integer node over two distinct bounded parameters, `Int[0, 9]` and `Int[10, 20]`, each typed by its own `integer_range` `bounded_domain`, with its result typed by a third `integer_range` `[0, 29]`, generates an `ir_confirmed` oracle whose descriptor is compared with the result bound only; the claim's checked bounds are the result bound, then each operand's own bound; derivation returns the descriptor over `[0, 29]`. | Test (TC-024) |
| FR-014-AC-21 | A node whose result is typed by a scalar type and whose reachable bounds are its operands' own and exactly one other takes that other as its result bound. | Test (TC-024) |
| FR-014-AC-22 | A node whose result is typed by a scalar type and whose two or more reachable bounds of the form are not exactly one outside the operands' own bounds is refused as `AmbiguousBound`, in generation and in derivation. | Test (TC-024) |
| FR-014-AC-23 | An operand, or the result, typed by a `bounded_domain` of a form other than the descriptor's is refused as `MissingBound`. | Test (TC-024) |
| FR-014-AC-24 | An operand's own bound need not lie inside the result bound: `a + wide` over `[0, 9]` and `[0, 50]` with a `[0, 29]` result, `-e` over `[1, 9]` with a `[-9, -1]` result and `e * f` over `[1, 9]` and `[100, 200]` with a `[100, 1800]` result each generate, derive the same descriptor, and record each operand's own bound. | Test (TC-024) |
| FR-014-AC-25 | A `reference` operand to a plain scalar parameter or a non-literal `value` node is refused as `RequiresBound` naming its type, in generation and in derivation, where another operand owns a bound or the result is typed by one (`x + y`, `y + y`, and `x + y` over a scalar-typed result); a literal operand is never refused as `RequiresBound`. An eligible application reference follows FR-014-AC-45 instead. | Test (TC-024) |
| FR-014-AC-26 | A `reference` operand whose target is a `value` node with a literal body is a literal operand: `x + 1` over `x: Int[0, 9]` narrowed to `Int[0, 10]`, with the literal as a reference to its own plain-Integer-typed `value` node, generates with checked bounds `[0, 10]` then `[0, 9]` and derives the descriptor over `[0, 10]`. | Test (TC-024) |
| FR-014-AC-27 | A node whose result is typed by a scalar type and that only narrowing conversions (`quire.op.numeric.narrow`, exactly one argument referencing the node) consume takes the bound they narrow it to as its result bound: `x + y` over `Int[0, 9]` and `Int[10, 20]` narrowed to `Int[10, 29]` generates with checked bounds `[10, 29]`, `[0, 9]`, `[10, 20]` and derives the descriptor over `[10, 29]`, a descriptor over `[0, 9]` is `BoundMismatch` against `[10, 29]`, and `-z` over `Int[0, 9]` narrowed to `Int[-9, 0]` checks `[-9, 0]` then `[0, 9]`. | Test (TC-024) |
| FR-014-AC-28 | The referenced node's body, not its form or type, makes an operand a literal: `x + n`, where `n` is a plain-Integer-typed `value` node of form `literal` whose body is not a literal, is refused as `RequiresBound` naming Integer, in generation and in derivation. | Test (TC-024) |
| FR-014-AC-29 | `x + p` narrowed to `Int[0, 29]`, where `p` is a parameter typed by the plain Integer type, is refused as `RequiresBound` naming Integer, in generation and in derivation. | Test (TC-024) |
| FR-014-AC-30 | `q + q` narrowed to `Int[0, 29]`, where no operand owns a bound and `q`'s node reaches `[0, 29]`, is refused as `RequiresBound` naming Integer, in generation and in derivation: the narrowing bounds the result as a `bounded_domain` result type does. | Test (TC-024) |
| FR-014-AC-31 | A literal operand of an integer operation that is not a canonical `integer` literal is refused as `OperandTypeMismatch` naming its position and literal kind, whatever the referenced node's type: `x + "a"` with the text literal in a plain-Integer-typed `value` node is refused at position 1 with kind `text`, in generation and in derivation. | Test (TC-024) |
| FR-014-AC-32 | A node that narrowing conversions narrow to two distinct bounds (`[10, 29]` and `[0, 40]`) is refused as `AmbiguousBound`, in generation and in derivation. | Test (TC-024) |
| FR-014-AC-33 | A node narrowed to a `text_bounds` bound over Integer, or to an `integer_range` over Rational, is refused as `MissingBound` naming Integer and `integer_range`, in generation and in derivation. | Test (TC-024) |
| FR-014-AC-34 | A node that a narrowing conversion and a node that is not a narrowing conversion both consume (`x + 2` narrowed to `Int[0, 11]` and an operand of `(x + 2) + x`) is refused as `AmbiguousBound`, in generation and in derivation, while `x + 1`, consumed only by its narrowing, generates. | Test (TC-024) |
| FR-014-AC-35 | Boolean `and`, `or`, `not`, `implies`, `eq` and `ne` nodes and integer `eq`, `ne`, `lt`, `le`, `gt` and `ge` nodes each derive a descriptor and generate an `ir_confirmed` oracle; each connective's oracle calls `evaluate_boolean` or `evaluate_boolean_short_circuit`, and each `eq`/`ne` oracle calls `check_equality` and `CheckedEquality::evaluate`, each returning `Outcome<bool>`. | Test (TC-024) |
| FR-014-AC-36 | Every oracle's source map declares the node's implication-consequent count, one evaluation-entry probe on the function-entry line disjoint from every consequent region, and one entry-token probe inside the exact region of each consequent; a dropped or duplicated region does not change the declared count. | Test (TC-024) |
| FR-014-AC-37 | A differential corpus covering every node FR-014-AC-35 names compiles against the runtime alone, and each oracle's outcome, admitted charges and consumed counters equal direct runtime execution and the QSL value authority. | Test (TC-024) |
| FR-014-AC-38 | PLANNED (IR-489). The Boolean condition of a `state`/`state_clause` node, addressed by the clause node id, derives a descriptor and generates an `ir_confirmed` oracle over its connectives, integer comparisons and integer add, subtract, multiply and negate, returning `Outcome<bool>`. A `project(deref(self), field)` term is an operand that takes a value in the object member's `integer_range` as a state field read, and a `pre(...)` of one takes the pre-state value of that field. FR-014-AC-10 is not weakened: an operand that is neither a literal, a reference nor one of those two terms is still refused. The `state` node itself stays refused as FR-014-AC-7 states. | Test (TC-024) |
| FR-014-AC-39 | The generated `src/lib.rs` of every crate the TC-024 corpus generates, and the non-test code of `src/oracle/scalar/mod.rs` (comments and every `#[cfg(test)]` item removed wherever the item sits, string literals the generator emits counted), contain zero panic tokens: the identifiers `unwrap`, `expect`, `unwrap_unchecked`, `unwrap_err`, `expect_err`, `unwrap_err_unchecked`, `panic_any` and `resume_unwind` however written (called, with whitespace before the paren, named on a path such as `Option::unwrap`, or imported); the macros `panic`, `unreachable`, `todo`, `unimplemented`, `assert`, `assert_eq`, `assert_ne`, `debug_assert`, `debug_assert_eq` and `debug_assert_ne` in any delimiter form, with any whitespace before the `!` and any path prefix; and the identifier `abort` anywhere except as a method call (`.abort()`), so `process::abort` and a bare `abort` after an import are both counted. | Test (TC-024) |
| FR-014-AC-40 | A package whose lowering returns `failed` for the `bytes` limit (Contract IR FR-038-AC-95) is refused per item as `ExactScalarRefusal::LoweringByteLimitExceeded { limit, consumed }` with `limit` and `consumed` equal to the record's, never as `LoweringWorkExhausted`, and contributes no generated function. Through a whole call of `generate_exact_scalar_oracles` whose input is read under a byte ceiling that admits the checked package and is one byte below the canonical length of the lowered contract package of that call (the package `lower` returns for the call's requested nodes, not the checked package, which the fixture makes shorter), every requested item is refused so, each with `limit` equal to that ceiling and the same `consumed`, and no function is generated. A `Failed { limit_kind: Bytes }` record built by hand and given to the scalar `Failed` arm in the module's own `#[cfg(test)]` seam is refused so with the record's `limit` and `consumed` field for field (the per-node case of Contract IR FR-038-AC-95, which CG's public API cannot reach because a node preimage is shorter than any document it can read; the arm maps one record to one refusal, so it asserts no sibling isolation, which FR-014-AC-1 covers). | Test (TC-024) |
| FR-014-AC-41 | A `failed` record whose `limit_kind` is `work` is still refused as `LoweringWorkExhausted` with the record's `limit` and `consumed` (FR-014-AC-15 unchanged), and one whose `limit_kind` is neither `work` nor `bytes` (each of `nodes`, `edges`, `occurrences` and `diagnostics`) is refused as `ExactScalarRefusal::LoweringLimitUnrecognised` with `limit_kind` equal to that snake_case name and the record's `limit` and `consumed`, without a panic and without a `LoweringWorkExhausted` or `LoweringByteLimitExceeded`. Each of the two is asserted on hand-built `Failed` records given to the scalar `Failed` arm itself in the module's `#[cfg(test)]` seam, so a mapping of an unrecognised kind to `LoweringWorkExhausted` in this module fails it. | Test (TC-024) |
| FR-014-AC-42 | No `limit_kind` of a `failed` record is read in `src/oracle/scalar/mod.rs`, `src/oracle/equality/mod.rs` or `src/oracle/function/mod.rs` but through the one shared classifier, `classify_lowering_failure` of the shared `oracle` module: the non-test code of each of the three files, as FR-014-AC-39 defines it (comments and every `#[cfg(test)]` item removed wherever the item sits, taken with `non_test_code` and `comments_stripped` of `tests/common/panic_scan.rs`), names no `CheckedPackageLimit` and accesses no `.limit_kind`, has exactly one such direct-lowering `Failed` arm in `src/oracle/scalar/mod.rs`, which binds nothing (`Failed { .. }`) and calls `classify_lowering_failure`; the FR-330 equality and exact-function package paths have no direct-lowering `Failed` arm and retain their admitted `Failed` record through `PackageAccounting`. Each file names `limit_kind` only as the field of its own `LoweringLimitUnrecognised` refusal and in the conversion into it. | Test (TC-024) |
| FR-014-AC-44 | For each IR-admitted direct request naming a `temporal`/`formula`, `temporal`/`fairness` or `expression`/`case` node, the scalar claim map has exactly one entry with that requested node id and `ExactScalarRefusal::Unsupported`. For an IR `Unsupported` lowering record, the refusal's `unsupported_node_id` and `node_tag` equal that record's first unsupported reachable node and tag, even if that node is not the request root; for a lowered `case` root they name the root and `expression`. A lowered `case` is refused as `Unsupported`, not `FormMismatch`. Each refused item has no generated function or symbol, does not panic or disappear, and leaves a supported sibling's claim and source byte-identical to a run without the refused request. Derivation and routed Kani retain one typed unsupported or underivable outcome for each request without a harness. | Test (TC-024) |
| FR-014-AC-45 | For a QSL-shaped outer integer operation narrowed to an `integer_range`, a direct reference to a plain-Integer inner application over independently bounded operands supplies a conservative inclusive operand interval: `(x + 1) + 1` for `x: Int[0, 9]` supplies `[1, 10]` for its inner operand, and `-(x + 1)` supplies `[1, 10]` to the outer negate; `x * -1` for `x: Int[1, 9]` supplies `[-1, -1]` from a negate application over the canonical literal `1`. Each outer node derives and generates an `ir_confirmed` oracle, records the inferred range by argument position without inventing a checked bound id, and retains the outer result bound's descriptor check. | Test (TC-024) |
| FR-014-AC-46 | An inferred interval uses only the four named integer operations and canonical literal or `integer_range`-typed leaves. A direct reference to a raw application consumed by a separate narrowing conversion never takes that conversion's domain as its operand interval; where the raw application has another consumer, requesting that application itself still yields FR-014-AC-34's `AmbiguousBound`. A plain-Integer parameter at any depth remains `RequiresBound` naming Integer. A missing, cyclic or unsupported application, wrong-kind or noncanonical literal, and an inference depth, work or endpoint-bit ceiling each refuse with a typed cause and no generated oracle, without changing a supported sibling. | Test (TC-024) |

## Dependencies

- **Upstream**: Contract IR
  FR-036/FR-038 (CheckedPackage V2 lowering), Contract Runtime FR-007 (exact
  scalar operators), QSpec FR-196.
- **Downstream**: [TC-024](../matrix/TC-024-exact-scalar-oracles.md),
  [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md).
