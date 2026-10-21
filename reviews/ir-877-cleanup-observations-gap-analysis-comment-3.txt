<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15291 method=gap-analysis lang=rust pr=quire-contract-codegen#501 model=unavailable run=d9d507d8-42eb-4ddc-a8d0-3eed0b37342f -->

SR-15291 gap-analysis, part 3/18. FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-037 | high | coverage | Inherited FR-015-AC-42 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-038 | high | coverage | Inherited FR-015-AC-43 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-039 | high | coverage | Inherited FR-015-AC-44 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-040 | high | coverage | Inherited FR-015-AC-45 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-041 | high | coverage | Inherited FR-015-AC-46 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-042 | high | coverage | Inherited FR-015-AC-47 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-043 | high | coverage | Inherited FR-015-AC-48 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-044 | high | coverage | Inherited FR-015-AC-49 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-045 | high | coverage | Inherited FR-015-AC-51 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-046 | high | coverage | Inherited FR-015-AC-52 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-047 | high | coverage | Inherited FR-015-AC-56 is tagged-by-ignored-test at the frozen candidate: every bound test is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-048 | high | coverage | Inherited FR-015-AC-57 is tagged-by-ignored-test at the frozen candidate: every bound test is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-049 | high | coverage | Inherited FR-015-AC-69 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-050 | high | coverage | Inherited FR-015-AC-70 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-051 | high | coverage | Inherited FR-015-AC-71 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-052 | high | coverage | Inherited FR-015-AC-72 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-053 | high | coverage | Inherited FR-015-AC-73 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-054 | high | coverage | Inherited FR-015-AC-74 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |

+++ [reviewer data]

```yaml
clean: false
scope:
- id: FR-014-AC-10
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A descriptor parameter whose admitted bound is missing, repeated or unequal to the reachable
    `bounded_domain` node, an operand that is neither a literal nor a reference, and a literal quantity
    operand, are each refused by CG with a typed reason. A malformed bound with a noncanonical integer
    endpoint such as `05` is refused by Contract IR's strict checked-package reader before CG receives
    an admitted item; CG retains its defensive typed `UnreadableBound` refusal for an admitted bound it
    cannot decode. Contract IR's FR-038 "Mode" reader rule also refuses a bare Float32 addition authored
    with the
- id: FR-014-AC-11
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A claim whose descriptor passes every check this requirement states, and whose descriptor agrees
    with the node's catalogued `operation.identity`, law definition and mode value, marks its operation
    `ir_confirmed`. An item refused by any of those checks is `caller_declared` even where its operation
    agrees.
- id: FR-014-AC-12
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A descriptor naming a different catalogued operation than the node's own is not confirmed,
    including where the two share one operand shape and one checked bound; nor is a descriptor naming
    an operation the catalogue has no entry for.
- id: FR-014-AC-13
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A descriptor whose implied identity matches the node's but whose law definition is absent from
    that node's `operation.laws`, or whose mode value disagrees with that node's `operation.mode`, is
    not confirmed; the item still lowers and still generates the oracle its descriptor names, marked `caller_declared`
    with a typed blocked item.
- id: FR-014-AC-14
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A claim whose node this generator never inspected — a duplicate copy, or a record that never
    lowered — or inspected and refused with a typed reason, marks its operation `caller_declared` with
    a typed blocked item and reports the request item's own descriptor-derived identity.
- id: FR-014-AC-15
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A requested item whose node closure exceeds the 65,536-unit lowering work ceiling is refused
    as `LoweringWorkExhausted`, naming the ceiling and the consumed counter, contributes no generated
    function, and leaves every other item's disposition unaffected.
- id: FR-014-AC-16
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A bound is read from `binding` members looked up by name in any order (`min`/`max`, `numerator_min`
    through `denominator_max`, `coefficient_min` through `rounding`, `rounding`, `min`/`max`/`text_profile`);
    a bare literal member, a missing, duplicate or unlisted name is refused as an unreadable bound.
- id: FR-014-AC-17
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A `reference` operand whose target is typed by a `bounded_domain` node is classified by that
    domain's base scalar type, so `x + 1` over a parameter `x` of type `Int[0, 9]` generates, and a bounded
    text parameter is still a text operand.
- id: FR-014-AC-18
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: '`derive_exact_scalar_items` returns, per node id in input order, the descriptor determined
    by the node''s `operation.identity`, `operation.mode`, `operation.laws`, operand forms and the one
    bound of the needed form on its result type; a node that is not an application, has no identity, names
    an identity outside the derivable set, has operand forms, a law or a mode that select no parameter,
    is refused with the matching `ClaimDerivationRefusal` and no descriptor; a node that is absent, does
    not lower or has a refused bound (missing, repeated or unreadable) is refused with the `ExactScalarRefusal`'
- id: FR-014-AC-19
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A derived item generates an oracle whose operation is `ir_confirmed`, and for every item of
    the corpus the derived descriptor equals the descriptor the fixture declares.
- id: FR-014-AC-20
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: An integer node over two distinct bounded parameters, `Int[0, 9]` and `Int[10, 20]`, each typed
    by its own `integer_range` `bounded_domain`, with its result typed by a third `integer_range` `[0,
    29]`, generates an `ir_confirmed` oracle whose descriptor is compared with the result bound only;
    the claim's checked bounds are the result bound, then each operand's own bound; derivation returns
    the descriptor over `[0, 29]`.
- id: FR-014-AC-21
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A node whose result is typed by a scalar type and whose reachable bounds are its operands'
    own and exactly one other takes that other as its result bound.
- id: FR-014-AC-22
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A node whose result is typed by a scalar type and whose two or more reachable bounds of the
    form are not exactly one outside the operands' own bounds is refused as `AmbiguousBound`, in generation
    and in derivation.
- id: FR-014-AC-23
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: An operand, or the result, typed by a `bounded_domain` of a form other than the descriptor's
    is refused as `MissingBound`.
- id: FR-014-AC-24
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'An operand''s own bound need not lie inside the result bound: `a + wide` over `[0, 9]` and
    `[0, 50]` with a `[0, 29]` result, `-e` over `[1, 9]` with a `[-9, -1]` result and `e * f` over `[1,
    9]` and `[100, 200]` with a `[100, 1800]` result each generate, derive the same descriptor, and record
    each operand''s own bound.'
- id: FR-014-AC-25
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A `reference` operand to a plain scalar parameter or a non-literal `value` node is refused
    as `RequiresBound` naming its type, in generation and in derivation, where another operand owns a
    bound or the result is typed by one (`x + y`, `y + y`, and `x + y` over a scalar-typed result); a
    literal operand is never refused as `RequiresBound`. An eligible application reference follows FR-014-AC-45
    instead.
- id: FR-014-AC-26
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A `reference` operand whose target is a `value` node with a literal body is a literal operand:
    `x + 1` over `x: Int[0, 9]` narrowed to `Int[0, 10]`, with the literal as a reference to its own plain-Integer-typed
    `value` node, generates with checked bounds `[0, 10]` then `[0, 9]` and derives the descriptor over
    `[0, 10]`.'
- id: FR-014-AC-27
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A node whose result is typed by a scalar type and that only narrowing conversions (`quire.op.numeric.narrow`,
    exactly one argument referencing the node) consume takes the bound they narrow it to as its result
    bound: `x + y` over `Int[0, 9]` and `Int[10, 20]` narrowed to `Int[10, 29]` generates with checked
    bounds `[10, 29]`, `[0, 9]`, `[10, 20]` and derives the descriptor over `[10, 29]`, a descriptor over
    `[0, 9]` is `BoundMismatch` against `[10, 29]`, and `-z` over `Int[0, 9]` narrowed to `Int[-9, 0]`
    checks `[-9, 0]` then `[0, 9]`.'
- id: FR-014-AC-28
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'The referenced node''s body, not its form or type, makes an operand a literal: `x + n`, where
    `n` is a plain-Integer-typed `value` node of form `literal` whose body is not a literal, is refused
    as `RequiresBound` naming Integer, in generation and in derivation.'
- id: FR-014-AC-29
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: '`x + p` narrowed to `Int[0, 29]`, where `p` is a parameter typed by the plain Integer type,
    is refused as `RequiresBound` naming Integer, in generation and in derivation.'
- id: FR-014-AC-30
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: '`q + q` narrowed to `Int[0, 29]`, where no operand owns a bound and `q`''s node reaches `[0,
    29]`, is refused as `RequiresBound` naming Integer, in generation and in derivation: the narrowing
    bounds the result as a `bounded_domain` result type does.'
- id: FR-014-AC-31
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A literal operand of an integer operation that is not a canonical `integer` literal is refused
    as `OperandTypeMismatch` naming its position and literal kind, whatever the referenced node''s type:
    `x + "a"` with the text literal in a plain-Integer-typed `value` node is refused at position 1 with
    kind `text`, in generation and in derivation.'
- id: FR-014-AC-32
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A node that narrowing conversions narrow to two distinct bounds (`[10, 29]` and `[0, 40]`)
    is refused as `AmbiguousBound`, in generation and in derivation.
- id: FR-014-AC-33
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A node narrowed to a `text_bounds` bound over Integer, or to an `integer_range` over Rational,
    is refused as `MissingBound` naming Integer and `integer_range`, in generation and in derivation.
- id: FR-014-AC-34
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A node that a narrowing conversion and a node that is not a narrowing conversion both consume
    (`x + 2` narrowed to `Int[0, 11]` and an operand of `(x + 2) + x`) is refused as `AmbiguousBound`,
    in generation and in derivation, while `x + 1`, consumed only by its narrowing, generates.
- id: FR-014-AC-35
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Boolean `and`, `or`, `not`, `implies`, `eq` and `ne` nodes and integer `eq`, `ne`, `lt`, `le`,
    `gt` and `ge` nodes each derive a descriptor and generate an `ir_confirmed` oracle; each connective's
    oracle calls `evaluate_boolean` or `evaluate_boolean_short_circuit`, and each `eq`/`ne` oracle calls
    `check_equality` and `CheckedEquality::evaluate`, each returning `Outcome<bool>`.
- id: FR-014-AC-36
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Every oracle's source map declares the node's implication-consequent count, one evaluation-entry
    probe on the function-entry line disjoint from every consequent region, and one entry-token probe
    inside the exact region of each consequent; a dropped or duplicated region does not change the declared
    count.
- id: FR-014-AC-37
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A differential corpus covering every node FR-014-AC-35 names compiles against the runtime
    alone, and each oracle''s outcome, admitted charges and consumed counters equal direct runtime execution.
    This criterion claims no agreement with the QSL value authority: that boundary is owned by quire-integration
    IR-842 (generated-oracle against QSL agreement), and FR-014-AC-47 keeps CG''s artifacts from claiming
    it.'
- id: FR-014-AC-38
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: The Boolean condition in the third argument of a `state`/`state_clause` node's `quire.op.state.clause`
    body, addressed by the clause node id, derives a descriptor and generates an `ir_confirmed` oracle
    over its connectives, integer comparisons and integer add, subtract, multiply and negate, returning
    `Outcome<bool>`. When the condition is a reference to an admitted Boolean expression node, the oracle
    uses that node's expression, not the reference as a Boolean variable; an inline condition uses its
    own expression. In both cases the claim and generated oracle retain the clause node identity, not
- id: FR-014-AC-39
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'The generated `src/lib.rs` of every crate the TC-024 corpus generates, and the non-test code
    of `src/oracle/scalar/mod.rs` (comments and every `#[cfg(test)]` item removed wherever the item sits,
    string literals the generator emits counted), contain zero panic tokens: the identifiers `unwrap`,
    `expect`, `unwrap_unchecked`, `unwrap_err`, `expect_err`, `unwrap_err_unchecked`, `panic_any` and
    `resume_unwind` however written (called, with whitespace before the paren, named on a path such as
    `Option::unwrap`, or imported); the macros `panic`, `unreachable`, `todo`, `unimplemented`, `assert`,
    `assert'
- id: FR-014-AC-40
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A package whose lowering returns `failed` for the `bytes` limit (Contract IR FR-038-AC-95)
    is refused per item as `ExactScalarRefusal::LoweringByteLimitExceeded { limit, consumed }` with `limit`
    and `consumed` equal to the record''s, never as `LoweringWorkExhausted`, and contributes no generated
    function. Through a whole call of `generate_exact_scalar_oracles` whose input is read under a byte
    ceiling that admits the checked package and is one byte below the canonical length of the lowered
    contract package of that call (the package `lower` returns for the call''s requested nodes, not the
    checked '
- id: FR-014-AC-41
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A `failed` record whose `limit_kind` is `work` is still refused as `LoweringWorkExhausted`
    with the record''s `limit` and `consumed` (FR-014-AC-15 unchanged), and one whose `limit_kind` is
    neither `work` nor `bytes` (each of `nodes`, `edges`, `occurrences` and `diagnostics`) is refused
    as `ExactScalarRefusal::LoweringLimitUnrecognised` with `limit_kind` equal to that snake_case name
    and the record''s `limit` and `consumed`, without a panic and without a `LoweringWorkExhausted` or
    `LoweringByteLimitExceeded`. Each of the two is asserted on hand-built `Failed` records given to the
    scalar `Failed` '
- id: FR-014-AC-42
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'No `limit_kind` of a `failed` record is read in `src/oracle/scalar/mod.rs`, `src/oracle/equality/mod.rs`
    or `src/oracle/function/mod.rs` but through the one shared classifier, `classify_lowering_failure`
    of the shared `oracle` module: the non-test code of each of the three files, as FR-014-AC-39 defines
    it (comments and every `#[cfg(test)]` item removed wherever the item sits, taken with `non_test_code`
    and `comments_stripped` of `tests/common/panic_scan.rs`), names no `CheckedPackageLimit` and accesses
    no `.limit_kind`, has exactly one such direct-lowering `Failed` arm in `src/oracle/scalar/m'
- id: FR-014-AC-44
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: For each IR-admitted direct request naming a `temporal`/`formula`, `temporal`/`fairness` or
    `expression`/`case` node, the scalar claim map has exactly one entry with that requested node id and
    `ExactScalarRefusal::Unsupported`. For an IR `Unsupported` lowering record, the refusal's `unsupported_node_id`
    and `node_tag` equal that record's first unsupported reachable node and tag, even if that node is
    not the request root; for a lowered `case` root they name the root and `expression`. A lowered `case`
    is refused as `Unsupported`, not `FormMismatch`. Each refused item has no generated function or
- id: FR-014-AC-45
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'For a QSL-shaped outer integer operation narrowed to an `integer_range`, a direct reference
    to a plain-Integer inner application over independently bounded operands supplies a conservative inclusive
    operand interval: `(x + 1) + 1` for `x: Int[0, 9]` supplies `[1, 10]` for its inner operand, and `-(x
    + 1)` supplies `[1, 10]` to the outer negate; `x * -1` for `x: Int[1, 9]` supplies `[-1, -1]` from
    a negate application over the canonical literal `1`. Each outer node derives and generates an `ir_confirmed`
    oracle, records the inferred range by argument position without inventing a checked bound i'
- id: FR-014-AC-46
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: An inferred interval uses only the four named integer operations and canonical literal or `integer_range`-typed
    leaves. A direct reference to a raw application consumed by a separate narrowing conversion never
    takes that conversion's domain as its operand interval; where the raw application has another consumer,
    requesting that application itself still yields FR-014-AC-34's `AmbiguousBound`. A plain-Integer parameter
    at any depth remains `RequiresBound` naming Integer. A missing, cyclic or unsupported application,
    wrong-kind or noncanonical literal, and an inference depth, work or endpoint-bit
- id: FR-014-AC-47
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: No test name, trace tag, test output line or doc comment of every oracle-agreement corpus of
    FR-014, FR-018 and FR-021 (the TC-185, TC-186, TC-187, TC-192 and TC-193 vectors, the FR-014-AC-37
    corpus, the composite-equality corpus and the function-application corpus), no TC-024, TC-029 or TC-031
    step and no oracle-matrix row for those cases or corpora claims, or presents as evidence, agreement
    with the QSL value authority; text that states this boundary is permitted.
- id: FR-014-AC-48
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A `quire.op.quantity.convert` node whose `type_argument` names a quantity unit and whose `operation.mode`
    is `exact` derives and generates an `ir_confirmed` oracle, described by the public operation `quantity_conversion`
    (`ExactScalarOperation::QuantityConversion`, a unit-free variant: the target unit is a call-time argument
    `unit: &QuantityUnit`, and the generator does not check it against the `type_argument`, which the
    node carries only as a type node, Contract IR FR-038-AC-84 and FR-038-AC-213), with no checked bound,
    and the claim-map entry records the `type_argument` type node id (FR-014-'
- id: FR-014-AC-49
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'When the runtime refuses a `quire.op.quantity.convert` with `IllTyped` before any charge,
    the oracle stops as `ill_typed` with zero admitted charges and no change to any consumed counter,
    equal to direct `convert_quantity` execution (Contract Runtime FR-007-AC-21): `1 m` to `s` (U03-style
    `m` to `s`), U09 (a base dimension of the same declaration and shape as `L` under a different node
    key), `1 N_m` to `J` (U22) and compound `4 m^2` to `m`. An affine source or target converts and is
    not refused as `AffineUnitArithmetic`; only arithmetic on an affine unit is refused (FR-014-AC-6,
    U05).'
- id: FR-014-AC-50
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'No oracle generated for `quire.op.quantity.convert` returns a scalar: its result is always
    a `Quantity` in the supplied target unit, and a conversion node whose `operation.mode` is not `exact`
    is refused with `ClaimDerivationRefusal::ModeNotDerivable`, checked first, so a scalar `type_argument`
    with a non-`exact` mode is `ModeNotDerivable`; one whose mode is `exact` and whose `type_argument`
    is a scalar type (`Decimal` or `Integer`, QSpec TC-187 U07, U08, U16, U17, U18 and U29) or a compound
    unit (U25''s first step; IR-831) is refused with `OperandFormsNotDerivable`, each with no descriptor,
    no'
- id: FR-014-AC-51
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A selected `quire.op.quantity.convert` claim settles through capability settlement like any
    other quantity operation, as `OperationNotRendered` (Planned, IR-269), because the Kani scalar renderer
    builds no quantity harness; the conversion is credited by its oracle only and is uncredited for Kani.
    The settlement is not a refusal by name, and a quantity harness is IR-269's work, not this requirement's.
    The owning settlement row is TC-030's Quantity scalar row, which names `quire.op.quantity.convert`
    and cites this criterion.
- id: FR-015-AC-1
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Pre, post, invariant and frame obligations of one contract produce separate harnesses with
    distinct identities.
- id: FR-015-AC-2
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Every harness identity records its model-domain bounds, taken only from IR `bounded_domain`
    nodes, and its solver and options and unwind bound.
- id: FR-015-AC-3
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An unbounded or non-finite obligation is refused with a typed reason and no harness.
- id: FR-015-AC-4
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: No harness assumption excludes an undefined, refused or incomplete runtime outcome.
- id: FR-015-AC-5
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An obligation whose bounds are unsatisfiable is refused with a typed reason and no harness.
- id: FR-015-AC-6
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An obligation over an oracle whose operation identity is `caller_declared` is refused with
    a typed reason and no harness.
- id: FR-015-AC-7
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Every generated harness contains exactly one non-vacuity cover; a precondition harness covers
    that the precondition holds within the IR bounds, and a contract harness''s cover stands after the
    contract call, so a run that satisfies every check without satisfying the cover is not a proof. The
    cover is the last statement of the harness, after every assertion the harness carries: the state-clause
    and frame-effect harnesses and the scalar harness place it after their assertions, and a contract
    harness''s checks are inside the contract call its cover follows. A cover that preceded an assertion
    would '
- id: FR-015-AC-8
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A postcondition or invariant harness emits every package precondition sharing its anchor operation
    as a `requires` on its generated contract and records each in its identity's embedded oracles; it
    embeds no other obligation's oracle; and an obligation whose sibling precondition is not a supported
    item of the same request is refused with a typed reason naming that precondition and no harness.
- id: FR-015-AC-9
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Every harness identity records solver `cadical` and the complete ordered option vector — function
    contracts, concrete playback, the exact fully qualified harness, `--exact`, the explicit unwind, the
    explicit solver, `--output-format regular`, and `--concrete-playback print` — and no option enabling
    stubbing is emitted.
findings:
- fnd: FND-037
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-42
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '778'
  excerpt: A V2 clause body holding an operator the V2 arm does not support (division, modulo, absolute
    value, and every operator outside connectives, the six integer comparisons and add, subtract, multiply,
    negate; those operators are planned scope for a later criterion) is `unsupported`, with a typed reason
    naming the node and the operator and no harness; a node that is not a modelled clause role, or a node
    id absent from the package, is refused as FR-015-AC-14 states; no item is skipped silently, and a
    supported item of the same request is unaffected. PLANNED (IR-489).
  finding: 'Inherited FR-015-AC-42 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-038
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-43
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '779'
  excerpt: A V2 postcondition harness emits every `precondition` clause node sharing its anchor node as
    a `requires` and records each in its identity's embedded oracles, embeds no other obligation's oracle,
    and a postcondition whose sibling precondition is not a supported item of the same request is refused
    naming that precondition with no harness; a V2 invariant is anchored at an object type, binds `self`
    alone, and so has no operation precondition to assume. PLANNED (IR-489).
  finding: 'Inherited FR-015-AC-43 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-039
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-44
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '780'
  excerpt: A census declared on a V2 clause claim with an empty or duplicate dependency identity, an inconsistent
    kind, state and path combination, or a non-`Required` kind is refused as a typed invalid input with
    no harness, by the same rules as FR-015-AC-22; the census request and readiness types it uses are
    not `ProofDependencyGraph`. PLANNED (IR-489).
  finding: 'Inherited FR-015-AC-44 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-040
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-45
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '781'
  excerpt: A valid census declared on a V2 clause claim is folded into that harness's identity record,
    readiness is `ready` only when every dependency passed and `incomplete` while any is missing or failed,
    and proof execution is recorded `not_run`, by the same rules as FR-015-AC-25; in addition, the folded
    census is independent of the order the caller lists its dependencies. PLANNED (IR-489).
  finding: 'Inherited FR-015-AC-45 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-041
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-46
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '782'
  excerpt: 'Every V2 clause harness contains exactly one non-vacuity cover, placed after every assumption,
    after the subject call and after every assertion the harness carries (the last statement, as FR-015-AC-7
    states), that states the family''s reachability, as FR-015-AC-7 does: a precondition harness covers
    that the precondition holds within the bounds; a postcondition harness covers, after the subject call
    returns, that the requires and the bounds are jointly satisfiable; an invariant harness covers that
    the drawn state satisfies its domain assumptions. The subject call is the call of the customer subj'
  finding: 'Inherited FR-015-AC-46 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-042
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-47
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '783'
  excerpt: Regeneration of a V2 clause harness from equal inputs is byte-identical, and changing the unwind
    bound or the customer subject changes the harness identity record (as FR-015-AC-10 states). PLANNED
    (IR-489).
  finding: 'Inherited FR-015-AC-47 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-043
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-48
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '784'
  excerpt: The obligation identity of a V2 clause claim is formed by AD-003 E-1 from the clause node id,
    its occurrence key, the obligation kind, and the arguments each as parameter node id and declared
    domain for an operation parameter, as E-1 states (the domain FR-015-AC-39 defines). A state field
    read is named by its declaring node id and field name, never by a node of its own; that E-1 lists
    parameters only is an open question for the AD-003 owner and E-1 is not widened here. The source span
    is excluded; changing any included member changes it, and changing the span, the unwind bound or the
    subject d
  finding: 'Inherited FR-015-AC-48 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-044
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-49
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '785'
  excerpt: A V2 invariant claim yields a harness that draws its state and parameters within their declared
    domains (FR-015-AC-39) and asserts the invariant clause; its subject is the clause itself, with no
    subject call. Preservation of the invariant under an operation is not specified by this criterion.
    PLANNED (IR-489).
  finding: 'Inherited FR-015-AC-49 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-045
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-51
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '787'
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes the hand-built bounded-corpus
    path, a finite input validated under a profile selection other than the offered profile's is refused
    as a typed `InvalidInput` `kani_profile_input_mismatch` result naming the request's source id, with
    no artifact and no case identity recorded.
  finding: 'Inherited FR-015-AC-51 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-046
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-52
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '788'
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes the hand-built bounded-corpus
    path, the context of every outcome and refusal that path returns (generated-case `Proved` or `Counterexample`,
    `kani_corpus_dependency_invalid`, `kani_corpus_identity_collision`, a lowering refusal and `kani_profile_input_mismatch`)
    is the profile selection's revision. The generated-case values are not Kani run verdicts.
  finding: 'Inherited FR-015-AC-52 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-047
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-56
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '792'
  excerpt: With the installed backend, a V1 bundle harness whose requires clause some bounded argument
    satisfies and whose `ensures` holds for every such argument classifies `Verified` (FR-017-AC-4); the
    same bundle with a requires clause no bounded argument satisfies classifies `CoverUnsatisfied` with
    its satisfied and total cover counts, never `Verified` and never `Falsified`.
  finding: 'Inherited FR-015-AC-56 is tagged-by-ignored-test at the frozen candidate: every bound test
    is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or
    qualify that obligation.'
- fnd: FND-048
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-57
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '793'
  excerpt: While its arm remains (FR-015-AC-109 to FR-015-AC-111), with the installed backend, a bounded-corpus
    harness of each of the arithmetic, graph and collection families whose oracle is true classifies `Verified`,
    and a graph or collection harness whose oracle is false (an arithmetic case's oracle is always true)
    classifies `Falsified` carrying the assertion's playback, which until FR-015-AC-111 deletes the corpus
    replay is replayed through the corpus case's QSL source (`tc_023_real_corpus_playback_replays_through_its_qsl_source`),
    never `Inconclusive` for lack of a counterexample.
  finding: 'Inherited FR-015-AC-57 is tagged-by-ignored-test at the frozen candidate: every bound test
    is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or
    qualify that obligation.'
- fnd: FND-049
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-69
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '866'
  excerpt: Emit only the `Equal` harness for a `NotEqual` item, emit a `production` harness for the family,
    or read a bound from the descriptor.
  finding: 'Inherited FR-015-AC-69 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-050
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-70
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '867'
  excerpt: Draw a payload for an absent option, collapse `absent` and `null` into one draw, take a bound
    from the whole `i64`, or leave a literal operand's leaf unpinned.
  finding: 'Inherited FR-015-AC-70 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-051
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-71
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '868'
  excerpt: Compute the expectation by calling the shadow, so the assertion is `x == x`; stop counting
    after the first unequal field; or place the cover before the assertions.
  finding: 'Inherited FR-015-AC-71 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-052
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-72
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '869'
  excerpt: Emit a loop or a `Vec` in the shadow, or raise `SHADOW_PAIR_NODE_BUDGET` to fit a closure instead
    of refusing it.
  finding: 'Inherited FR-015-AC-72 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-053
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-73
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '870'
  excerpt: Refuse every shape as one reason, treat an unbounded integer as the whole `i64`, a recursive
    declaration as depth one, or a `convert<T>` operand as its source type, or let a refused item drop
    a supported sibling.
  finding: 'Inherited FR-015-AC-73 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-054
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-74
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '871'
  excerpt: Pass a corpus case whose cover is unsatisfiable, whose refinement ran over a narrower domain
    than the shadow drew, or whose refinement was `sampled` where its domain fits the cap.
  finding: 'Inherited FR-015-AC-74 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
bindings: []
review_head: 3d303ea6988496c4e1102faa58a56c1f260193c3
review_verdict: 'FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence
  gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls
  now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.'
part: 3
parts: 18
scope_kind: full-repository criterion presence; no blanket semantic acceptance
```

+++