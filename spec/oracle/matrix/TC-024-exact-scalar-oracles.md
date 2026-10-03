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
byte-deterministic, and agree with the runtime and the QSL value authority.

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
   adapted from QSpec TC-185, TC-186, TC-187, TC-192 and TC-193; compare each
   outcome, admitted charges and consumed counters with direct runtime
   execution and with `quire_spec_language::value`, including the outcome of
   denying each admitted charge in turn.
5. Compile the generated crate manifest.
6. Derive an item for every node of the exact-scalar corpus with `derive_exact_scalar_items`
   and compare it with the descriptor the fixture declares (FR-014-AC-19);
   generate from the derived items and read each claim's provenance. Derive
   every overloaded identity (`rational.div` over integers and over rationals,
   `numeric.convert_rounding`, `numeric.convert`, `quantity.convert` to each
   target); derive nodes that are not applications, carry no identity, name an
   identity outside the derivable set, or carry a law, mode, operand forms or
   bound that select no parameter (FR-014-AC-18).
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

## Expected Results

Every family is generated; every refused item is absent from the source and
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

Integer arithmetic, rational arithmetic and ordering (including decimal
ordering) have an operator in the QSL value authority
(`evaluate_integer_arithmetic`, `evaluate_rational_arithmetic`, and
`order_numbers`). Their oracles are checked against direct runtime execution
and are counted separately from the three-way vectors.

## Boolean connectives, comparisons and source maps

1. Build a `CheckedPackageV2` with one node for each of Boolean `and`, `or`, `not`, `implies`, `eq`
   and `ne` and integer `eq`, `ne`, `lt`, `le`, `gt` and `ge` over bounded operands. Derive and
   generate every node and read each oracle's runtime call. Every node derives, every oracle is
   `ir_confirmed`, each connective calls `evaluate_boolean` or `evaluate_boolean_short_circuit`, and
   each `eq`/`ne` calls `check_equality` and `CheckedEquality::evaluate`, each returning
   `Outcome<bool>` (FR-014-AC-35). Compile the crate against the runtime alone and run each oracle
   over the bounded domain beside direct runtime execution and the QSL value authority. Outcomes,
   charges and counters agree on every input (FR-014-AC-37).
2. Generate an oracle over an `implies` node and read its source map. It declares the consequent
   count, one evaluation-entry probe on the function-entry line outside every consequent region and
   one entry-token probe inside each consequent's region; dropping or duplicating a region leaves the
   declared count unchanged (FR-014-AC-36).

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
