---
id: TC-024
title: "Verify exact complete-V1 scalar oracle generation and agreement"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-218
    type: references
---
# TC-024: Verify exact complete-V1 scalar oracle generation and agreement

## Description

Verify that scalar oracles generated from an admitted CheckedPackage V2 cover
every scalar family, refuse every non-generated item with a typed reason, are
byte-deterministic, and agree with direct runtime execution. Agreement with the QSL value authority is not evidenced and not claimed by CG (FR-014-AC-47); the integer, rational and Boolean steps below that name it are an open gap.

## Test Procedure

1. Build an admitted V2 package holding expression nodes for every scalar
   operator, division law, IEEE operator and comparison, text profile and
   comparison operator, each depending on the `bounded_domain` nodes its
   descriptor needs, plus unlowered, unbounded, mismatched, mis-bounded,
   duplicate and non-scalar nodes and unsupported and literal operands.
2. Generate twice and with a permuted request; compare bytes with each other.
3. Inspect each claim-map entry, its ordering, its checked bounds and its
   `ir_confirmed` or `caller_declared` provenance, and each refusal; request a
   node under a descriptor naming a different catalogued operation; request a
   node under a
   descriptor naming a different law; request a node under a descriptor whose
   rounding mode agrees with the node's IR bound but disagrees with its
   catalogued operation mode; exhaust lowering work; exceed the
   generated source ceiling; construct invalid and incomplete body records.
4. Generate the corpus crate at test time, compile it with the agreement cases
   as its integration test, and execute its oracles on vectors
   adapted from QSpec TC-185, TC-186, TC-192 and TC-193 and from the TC-187 vectors U02, U03, U05, U09, U12, U13, U14, U19, U20, U21, U23, U24, U27 and U28 (FR-014-AC-6 lists the TC-187 vectors that are not covered); run each at QSpec's own limit tuple and assert QSpec's disposition, with the whole incomplete record; assert U06 and U11 against the runtime's unit-graph admission, since they have no generated oracle; compare each
   outcome, admitted charges and consumed counters with direct runtime
   execution, including the outcome of denying each admitted charge in turn.
   No TC-187 vector is compared with the QSL value authority here (FR-014-AC-47).
   U06 and U11 are a Contract Runtime unit-graph admission check (Contract Runtime FR-012-AC-4, TC-033), not oracle agreement.
5. Compile the generated crate manifest.
6. Derive an item for every node of the exact-scalar corpus with `derive_exact_scalar_items`
   and compare it with the descriptor the fixture declares (FR-014-AC-19);
   generate from the derived items and read each claim's provenance. Derive
   every overloaded identity (`rational.div` over integers and over rationals,
   `numeric.convert_rounding`); derive every `quantity.convert` node of the corpus
   (each carries the `exact` mode its unit type pins) and check each derives an
   `ir_confirmed` descriptor and generates an oracle that calls `convert_quantity`
   with `QuantityTarget::Exact` and returns a `Quantity` in the unit passed as the
   call-time argument `unit: &QuantityUnit`, never a scalar and never checked
   against the node's `type_argument` (FR-014-AC-48, FR-014-AC-50). Run the conversion vectors
   (U01, U04 `degC` to `K`, U09b, U10, U15, U22, U25's second step and U26) and the `ill_typed` vectors (`1 m` to `s`, U09, `1 N_m` to `J`, compound `4 m^2` to `m`),
   and check each outcome, admitted charges and consumed counters equal direct
   `convert_quantity` execution, with zero charges and no counter change for
   `ill_typed` (FR-014-AC-49). Request the same nodes under caller-declared
   descriptors, through `generate_routed` and through Kani obligation
   negotiation, and check capability settlement reports `OperationNotRendered`
   (Planned, IR-269) with no harness emitted (FR-014-AC-51). Check that the
   function-application generator and the function schedules refuse a
   conversion body as `UnsupportedOperator` (generator) and
   `ScheduleRefusal::UnsupportedOperation` (schedules), and the schedules refuse
   a function with a unit-typed parameter or result as a per-function
   `ScheduleRefusal::UnsupportedOperation`, never as a request-level error,
   accepting an earlier
   `Unsupported`, `NotExpression`, `UnsupportedOperandType` or argument refusal
   where that check fires first (FR-021-AC-21), that a conversion node with a scalar or compound
   `type_argument` is refused as `OperandFormsNotDerivable` and one with a
   non-`exact` mode as `ModeNotDerivable`, each with no oracle, and the mode is
   checked first, so a scalar `type_argument` with a non-`exact` mode is
   `ModeNotDerivable` (FR-014-AC-50). The scalar-target refusal runs on an
   admitted-package fixture; the `ModeNotDerivable` and compound-unit cases run
   only on hand-built nodes in module tests, because Contract IR FR-038-AC-213
   admits only `exact` for a unit `type_argument` and defines no compound-unit
   form (IR-831), so no admitted package reaches them. Check also that the composite-equality generator refuses a conversion
   root as `FormMismatch` and a conversion operand as an unsupported quantity,
   each before any artifact.
   The TC-187 quantity-to-quantity conversion vectors are agreement rows
   (FR-014-AC-6); U07, U08, U16, U17, U18, U29 (scalar targets) and U25's
   first step (compound target, IR-831) are refusal rows (FR-014-AC-50); check that every agreement row of
   step 4 compares at least one vector. Derive nodes that are not applications, carry no identity, name an
   identity outside the derivable set, or carry a law, mode, operand forms or
   bound that select no parameter (FR-014-AC-18).
   Keep the six `TextAdmission` fixtures in the refused corpus and check that
   Contract IR rejects each attempted text-operand `numeric.convert` node.
   None contributes a generated function, a derived descriptor or runtime
   agreement evidence for caller admission.
7. Generate and derive an integer add over two parameters typed by distinct
   `integer_range` domains `[0, 9]` and `[10, 20]` with its result typed by
   `[0, 29]`, and read the claim's checked bounds; refuse the descriptor over
   `[0, 9]` (FR-014-AC-20).
8. Generate a node over the same two parameters with a scalar-typed result and
   one further bound attached (FR-014-AC-21), and with none, in generation and
   derivation (FR-014-AC-22).
9. Generate an operand and a result each typed by a `text_bounds` bound over
   Integer (FR-014-AC-23).
10. Generate and derive `a + wide` over `[0, 9]` and `[0, 50]` with result
    `[0, 29]`, `-e` over `[1, 9]` with result `[-9, -1]` and `e * f` over
    `[1, 9]` and `[100, 200]` with result `[100, 1800]` (FR-014-AC-24).
11. Generate and derive `a + p`, `p + p` and `a + p` over a scalar result, where
    `p` is a `reference` typed by the plain Integer type, and `a + 1` with a
    literal operand (FR-014-AC-25).
12. In QSL's emitted shape (a literal as a reference to its own
    plain-Integer-typed `value` node, a parameter as a `parameter`-form `value`
    node, the declared bound on a narrowing `conversion` consuming the
    plain-typed arithmetic node), generate and derive `x + 1` over
    `x: Int[0, 9]` narrowed to `Int[0, 10]` (FR-014-AC-26).
13. In the same shape, generate and derive `x + y` over `Int[0, 9]` and
    `Int[10, 20]` narrowed to `Int[10, 29]`, also against a descriptor over
    `[0, 9]`, and `-z` over `Int[0, 9]` narrowed to `Int[-9, 0]`
    (FR-014-AC-27); call the narrowing check directly on raw conversion nodes
    with the narrow identity and one reference, two arguments, another identity,
    and a reference to another node (FR-014-AC-27).
14. Generate and derive `x + n` narrowed to `Int[0, 29]`, where `n` is a
    `literal`-form plain-Integer `value` node whose body is not a literal
    (FR-014-AC-28).
15. Generate and derive `x + p` narrowed to `Int[0, 29]`, where `p` is a
    plain-Integer parameter (FR-014-AC-29).
16. Generate and derive `q + q` narrowed to `Int[0, 29]`, where `q` is a node
    like `n` that also reaches `[0, 29]` (FR-014-AC-30).
17. Generate and derive `x + "a"` narrowed to `Int[0, 10]`, the text literal in
    a plain-Integer-typed `value` node (FR-014-AC-31).
18. Generate and derive a sum narrowed by two conversions to `[10, 29]` and
    `[0, 40]` (FR-014-AC-32).
19. Generate and derive sums narrowed to a `text_bounds` bound over Integer and
    to an `integer_range` over Rational (FR-014-AC-33).
20. Generate and derive `x + 2` narrowed to `Int[0, 11]` and also an operand of
    the unnarrowed `(x + 2) + x`, and `x + 1` consumed only by its narrowing
    (FR-014-AC-34).
20a. From admitted QSL-shaped nodes, derive and generate the outer operations
    `(x + 1) + 1` and `-(x + 1)` for `x: Int[0, 9]`, and `x * -1` for
    `x: Int[1, 9]`. Use the QSL emitter's reference to a plain-Integer inner
    application, its negate application over a literal `1`, and the outer
    narrowing conversions. Inspect each operation's `ir_confirmed` provenance,
    checked outer result bound, and per-position inferred operand interval
    (FR-014-AC-45). Also infer `-0` and negate of the canonical positive
    literal `9223372036854775808` as operands; inspect the exact singleton
    before Kani's `i64` check.
20b. Add a plain-Integer parameter under an inner add and under a negate;
    directly reference a raw inner add that is also consumed by a separate
    narrow whose bound is narrower than the inner add's independently inferred
    interval; request that inner add alone as well. Supply a wrong-kind and a
    noncanonical literal, a missing reference, a cycle, an unsupported
    application identity, and paths one beyond each of the 64-level,
    4,096-node and 4,096-bit limits beside a supported sibling. Exercise
    malformed shapes at the inference seam when Contract IR admission rejects
    them first. Check the exact refusal and sibling isolation (FR-014-AC-46).
21. Direct unsupported roots (FR-014-AC-44). Admit packages through Contract
    IR's public V2 reader with `temporal`/`formula`, `temporal`/`fairness` and
    `expression`/`case` roots, using QSpec FR-370 and FR-440 for formula and
    case shapes and Contract IR FR-038-AC-107 for fairness. Request each root
    with a supported scalar sibling. Compare each root's scalar-profile
    lowering record with its generated claim: the claim keeps the requested
    id, while `Unsupported` carries that record's first unsupported reachable
    id and tag. Also exercise a lowered `case` root at the shape-checking
    seam: its cause is the root and `expression`, never `FormMismatch`. Repeat
    without the refused root and compare the sibling's claim and source bytes.
    Derive the roots and route them through Kani; inspect their typed
    underivable or unsupported claim and absence of a harness, with no panic
    or missing claim.

## Expected Results

Every catalogued expression family in FR-014-AC-2 is generated; every refused
item is absent from the source and
carries its typed reason; a descriptor unequal to its IR bound is refused, and
one naming a different law, a different mode, or a different catalogued
operation, over equal bounds is generated only as `caller_declared`, while a
descriptor agreeing on identity, law and mode over a node that passes every
check is `ir_confirmed`;
bytes are identical across runs and orderings; all
three executions agree on every vector; the generated crate compiles with
`publish = false` and contains no charge literal; every exact-scalar corpus node
derives to its declared descriptor and generates `ir_confirmed`, and every
node with no derivable descriptor is refused with its typed
`ClaimDerivationRefusal`.

Each direct temporal formula, temporal fairness and case request has one
refused claim with its requested id, the causative unsupported id and tag
when IR lowering supplies them, and no oracle symbol. A lowered case root is
refused as unsupported at scalar shape checking. Derivation and routed Kani
retain typed outcomes without a harness; the supported sibling's claim and
source are unchanged.

The six attempted text-admission conversion nodes remain refused by Contract
IR. Their refusal is evidence of the checked-package boundary, not evidence
that a caller `TextPayload` was admitted or proved.

For the bounded-parameter and QSL-shaped steps:

- Step 7: `a + b` generates `ir_confirmed` with checked bounds `[0, 29]`,
  `[0, 9]`, `[10, 20]`, derives the descriptor over `[0, 29]`, and the
  descriptor over `[0, 9]` is `BoundMismatch` against `[0, 29]`.
- Step 8: the attached node's result bound is `[0, 29]`; with none attached
  the node is `AmbiguousBound`, in generation and derivation.
- Step 9: both are `MissingBound` naming Integer and `integer_range`.
- Step 10: each generates, derives the same descriptor, and records its
  operand bounds after the result bound.
- Step 11: `a + p`, `p + p` and `a + p` over a scalar result are
  `RequiresBound` naming Integer in generation and derivation; `a + 1`
  generates.
- Step 12: `x + 1` generates with checked bounds `[0, 10]` then `[0, 9]` and
  derives the descriptor over `[0, 10]`.
- Step 13: `x + y` checks `[10, 29]`, `[0, 9]`, `[10, 20]` and derives over
  `[10, 29]`; the descriptor over `[0, 9]` is `BoundMismatch` against
  `[10, 29]`; `-z` checks `[-9, 0]` then `[0, 9]`; only the one-reference
  narrow-identity conversion narrows.
- Steps 14 to 16: each is `RequiresBound` naming Integer, in generation and
  derivation.
- Step 17: `OperandTypeMismatch` at position 1, expected `integer`, found
  `text`, in generation and derivation.
- Step 18: `AmbiguousBound` naming Integer, in generation and derivation.
- Step 19: both `MissingBound` naming Integer and `integer_range`, in
  generation and derivation.
- Step 20: `x + 2` is `AmbiguousBound` naming Integer in generation and
  derivation; `x + 1` generates.
- Step 20a: the outer operations derive and generate `ir_confirmed`. Their
  inferred operand intervals by position are `[1, 10]`, `[1, 1]`;
  `[1, 10]`; and `[1, 9]`, `[-1, -1]`, respectively. No inferred interval is
  entered as a checked bound node, and each outer descriptor is checked
  against only its outer result bound. The edge singletons are `[0, 0]` and
  `[-9223372036854775808, -9223372036854775808]` without overflow or
  endpoint truncation.
- Step 20b: each plain-Integer parameter is `RequiresBound` naming Integer.
  The raw inner add's operand interval follows its independently bounded
  leaves, even though the separate narrow has a smaller bound; requesting
  that mixed-consumer inner node alone is `AmbiguousBound` as in step 20.
  A wrong-kind literal is `OperandTypeMismatch` naming outer argument position
  and literal kind; an unsupported nested operation is `OperandUnsupported`
  naming outer argument position and operation identity. Malformed literal or
  reference, missing node, or cycle refuses at IR admission or lowering, or
  yields `InvalidBody` naming the offending node at the inference seam. The
  three ceiling crossings are `OperandRangeLimitExceeded` naming node, kind,
  limit and consumed amount. None emits an oracle, and the supported sibling
  is byte-identical to its separate request.

Integer arithmetic, rational arithmetic and ordering (including decimal
ordering) have an operator in the QSL value authority
(`evaluate_integer_arithmetic`, `evaluate_rational_arithmetic`, and
`order_numbers`). Their oracles are checked against direct runtime execution
only.

## Boolean connectives, comparisons and source maps

1. Build a `CheckedPackageV2` with one node for each of Boolean `and`, `or`, `not`, `implies`, `eq`
   and `ne` and integer `eq`, `ne`, `lt`, `le`, `gt` and `ge` over bounded operands. Derive and
   generate every node and read each oracle's runtime call. Every node derives, every oracle is
   `ir_confirmed`, each connective calls `evaluate_boolean` or `evaluate_boolean_short_circuit`, and
   each `eq`/`ne` calls `check_equality` and `CheckedEquality::evaluate`, each returning
   `Outcome<bool>` (FR-014-AC-35). Compile the crate against the runtime alone and run each oracle
   over the bounded domain beside direct runtime execution and the QSL value authority. Outcomes,
   charges and counters agree on every input (FR-014-AC-37). Open gap: only the direct-runtime
   comparison runs; the QSL value-authority comparison is not evidenced and not claimed by CG, for
   the reason in FR-014's Dependencies note on the qsl-replay-only constraint (CG may call the
   `qsl-replay` facade only, QSL arch-lint T12-A); FR-014-AC-47 keeps CG's artifacts from claiming it.
2. Generate an oracle over an `implies` node and read its source map. It declares the consequent
   count, one evaluation-entry probe on the function-entry line outside every consequent region and
   one entry-token probe inside each consequent's region; dropping or duplicating a region leaves the
   declared count unchanged (FR-014-AC-36).

## Lowering byte-ceiling failure (IR-547)

1. Build a package whose lowered contract package for the call's requested nodes is longer than its
   checked package, read it under a byte ceiling that admits the checked package and is one byte
   below the canonical length of that lowered contract package, and request every node. Every item
   is refused `LoweringByteLimitExceeded` with `limit` equal to that ceiling and one shared
   `consumed`, none is `LoweringWorkExhausted`, and no function is generated (FR-014-AC-40).
2. In the scalar module's `#[cfg(test)]` seam, hand the module's `Failed` arm a
   `Failed { limit_kind: Bytes }` record and read `LoweringByteLimitExceeded` with the record's
   `limit` and `consumed`. The per-node case of Contract IR FR-038-AC-95 is not reachable
   through the public API (FR-014-AC-40).
3. Lower a package past the 65,536-unit work ceiling and read `LoweringWorkExhausted` with its
   ceiling and counter (FR-014-AC-41, with FR-014-AC-15). In the same seam, hand the scalar
   `Failed` arm a hand-built record for each of `nodes`, `edges`, `occurrences` and
   `diagnostics`; each is `LoweringLimitUnrecognised` with `limit_kind` the snake_case name and the
   record's `limit` and `consumed`, with no panic and never `LoweringWorkExhausted`
   (FR-014-AC-41).
4. Take the non-test code of `src/oracle/scalar/mod.rs`, `src/oracle/equality/mod.rs` and
   `src/oracle/function/mod.rs` with `non_test_code` and `comments_stripped`, find no
   `CheckedPackageLimit` and no `.limit_kind` access, `limit_kind` named only in the
   unrecognised-kind refusal and its conversion, and in each exactly one `Failed` arm that binds
   nothing and calls `classify_lowering_failure` (FR-014-AC-42).

## Panic-free source scan

1. Source-level scan (FR-014-AC-39). Read the `src/lib.rs` of every crate any step of this
   test case generates (all sections above) and the non-test text of `src/oracle/scalar/mod.rs`
   (comments and every `#[cfg(test)]` item removed wherever the item sits, string literals kept),
   and assert zero panic tokens as FR-014-AC-39 defines them: `unwrap`, `expect`,
   `unwrap_unchecked`, `unwrap_err`, `expect_err`, `unwrap_err_unchecked`, `panic_any` and
   `resume_unwind` however written (including `Option::unwrap` and `.unwrap ()`), the macros
   `panic`, `unreachable`, `todo`, `unimplemented`, `assert`, `assert_eq`, `assert_ne`,
   `debug_assert`, `debug_assert_eq` and `debug_assert_ne` in any delimiter form, and `abort`
   anywhere but as a method call. Run the scan over hand-written snippets of each spelling to show it
   names each, and over a source with an earlier mention of the attribute to show it cuts
   nothing early.
