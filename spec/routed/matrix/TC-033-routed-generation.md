---
id: TC-033
title: "Verify routed generation per backend kind without re-negotiation"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: verifies
---
# TC-033: Verify routed generation per backend kind without re-negotiation

## Description

Verify that `generate_routed` runs each routed item's backend-kind generation
arm, keyed by the driver's request index. It takes the routed backend and kind
as given, refuses an inconsistent routing, and returns for Kani exactly what
the FR-015 generator returns for the same items.

## Preconditions

- An admitted `CheckedPackageV2` fixture with at least three `expression`
  nodes that FR-014 generates as IR-confirmed integer `add`, `sub` or `negate`
  claims, plus one node that FR-015 accounts `Unsupported` without
  invalidating the request (for example an IR-confirmed family with no Kani
  renderer, refused `OperationNotRendered`). The existing TC-024 and TC-025
  fixtures serve.
- The package alone: no claim map is built or supplied.
- A package with a `quire.op.integer.rem` node, and the bounded-increment
  package (`x + 1` over a parameter `Int[0, 9]`).
- A `KaniGenerationContext` with a valid subject path and unwind `1`.
- The routed backend `Candidate { identity: "kani" }`.
- QSL RR-16's checked package and its occurrence-keyed requirement records:
  two occurrences of one `x + 1` application, with direct narrows to
  `Int[0, 10]` and `Int[0, 20]`. A second checked function has
  `if p then x + 1 else x + 1` and opposite guards on the two occurrences.

## Test Procedure

For FR-022-AC-23 and AC-24, use the same admitted ContractPackage and two
preflighted driver positions as TC-030: one `integer.add` node, two distinct
application occurrences and direct `numeric.narrow` selections with different
bounds. Route both and assert per-position claim identity, harness identity and
result assertion retain the selected occurrence and bound. In a fresh
settlement, change only one selection and check the other's output remains
unchanged. Then alter one lowering position's pair after preflight, remove
its conversion context root, and mispair its occurrence in separate cases.
Check the typed whole-call selection refusal occurs before any harness.
Inspect the generation path for any bound search across the package or
source-text/QSL context reconstruction.

For FR-022-AC-26 through AC-28, project both QSL `RequirementItem`s and
their keyed records through the driver's typed CG input. Route both RR-16
items together, then reverse their input order. Compare each output's
selected conversion, result assertion, application occurrence in the
obligation identity and harness identity by request index. Route the guarded
pair and inspect each proof condition: the then claim requires `p=true`, the
else claim requires `p=false`, and an unguarded item has no guard. Evaluate a
counterexample with `p=false` against the then item and with `p=true` against
the else item to verify neither counts as a proof of the opposite claim.
Remove one record, then separately replace its result bound, conversion
occurrence and guard sequence with the other occurrence's values. Mismatch
the item's occurrence-key node with the authenticated application selection
in a separate case; the record has no application-node field. Assert that the
typed refusal identifies the affected request index and occurrence and that
no oracle, harness or obligation identity is emitted for
that request. Keep the shared application node and the other record present
so a node-id fallback would fail this test.

For FR-022-AC-25, compile an external consumer against CG's public exports and
match `RoutedGenerationError::ContractScalar(ContractScalarError::Oracle(
OracleGenerationError::SourceTooLarge { bytes, limit, setting }))`. Assert the
typed size and limit fields remain available without a private module path or
diagnostic-string parsing.

1. **Differential with FR-015 (AC-2, AC-8).** Route the three generated nodes
   and the `Unsupported` node at the non-contiguous request indexes `7`, `2`, `11`
   and `4`, with kind `Kani`, and call `generate_routed`. Separately call
   `negotiate_kani_obligations` with the same four nodes as `ScalarClaim` items
   in ascending request-index order (`2`, `4`, `7`, `11`) and the same context.
   Compare record by record and harness by harness.
2. **No settlement (AC-3).** Assert, from the public signature, that
   `generate_routed` takes no `BackendDescriptor`, `Candidates`,
   `ExtentClassification` or `CapabilityKind`.
3. **Kind disagreement (AC-4).** Route one item whose backend identity is
   `"not-a-backend"`, with kind `Kani`, at request index `5`, beside a valid
   item.
4. **Duplicate index and missing context (AC-5).** Route two items at request
   index `3`. Separately, route a valid Kani item with `GenerationContexts {
   kani: None }`.
5. **Kani group refusal (AC-6).** Call once with unwind `0`, and once with the
   subject path `"not a path"`.
6. **Kani group rejection (AC-7).** Route the same node at request indexes `9`
   and `6`.
7. **Empty (AC-8).** Call with no routed items and `kani: None`.
8. **Determinism (AC-9).** Repeat step 1 twice, then with the routed slice
   reversed, and compare the results. Route one node alone at request index `0`,
   then at request index `40`, and compare its harness `rust` and `record`
   bytes.
9. **Exhaustive dispatch (AC-1).** Inspect the generation dispatch and the
   `GenerationContexts` definition.
10. **Derivation without a claim map (FR-022-AC-10).** Route the
    bounded-increment node with no claim map anywhere in the context.
11. **Underivable sibling (FR-022-AC-11, FR-015-AC-15).** Route the `integer.rem`
    node beside the three generated nodes.
12. **Refusals keep their disposition (FR-022-AC-11).** Route alone a node absent
    from the graph, an unbounded node, nodes whose bound is missing, of the
    wrong form or unreadable, an unsatisfiable bound and a function node; route one
    node twice at request indexes `5` and `3`.
13. **Returned claim map (FR-022-AC-12).** Compare `RoutedGeneration.claim_map`
    with `generate_exact_scalar_oracles` over `derive_exact_scalar_items` for
    the same nodes, and with `None` when nothing is routed.
14. **Bounded parameters (FR-022-AC-13).** Route alone the integer add over
    parameters `Int[0, 9]` and `Int[10, 20]` whose result is typed `[0, 29]`, the
    negation of `Int[1, 9]` with result `[-9, -1]`, the product of `Int[1, 9]` and
    `Int[100, 200]` with result `[100, 1800]`, and the additions `a + p`, `p + p`
    with `p` a plain-Integer reference.
15. **Returned oracle crate (FR-022-AC-14).** Route the bounded-increment node.
    Compare `RoutedGeneration.oracle_artifacts` with the artifacts of
    `generate_exact_scalar_oracles` over `derive_exact_scalar_items` for that node,
    and look up each `Generated` claim's oracle symbol in the returned
    `src/lib.rs` and in the harness's Rust source. Route alone the `integer.rem`
    node, and route nothing.
16. **QSL-shaped nodes (FR-022-AC-15, FR-015-AC-16).** Route alone, from a package in QSL's
    emitted shape, `x + 1`, `x + y` and `-z` narrowed to `Int[0, 10]`,
    `Int[10, 29]` and `Int[-9, 0]`; `x + p` with `p` a plain-Integer parameter;
    `x + n` with `n` a non-literal-bodied `value` node; and a sum narrowed to
    two distinct bounds.
17. **QSL-shaped literal and consumer refusals (FR-015-AC-17, FR-015-AC-18,
    FR-014-AC-31, FR-014-AC-34).** Route alone, in the same shape, `x + 100` and
    `x + 10^23` narrowed to `Int[0, 10]`, `x + "a"` with the text literal in an
    Integer-typed `value` node, and `x + 2` narrowed to `Int[0, 11]` and also
    consumed by `(x + 2) + x`.
18. **Duplicate harness (FR-022-AC-16).** Hand the arm's harness-pairing step two
    scalar harnesses with module symbols `a_module` and `b_module` and one harness
    symbol `x_proof`, and then two with distinct symbols. Unique name assignment
    makes the duplicate unreachable from `generate_routed` itself, so this step
    runs on the pairing function (`index_harnesses`).
19. **Nested bounded operands (FR-014-AC-45, FR-015-AC-103).** Route the outer
    applications of QSL-emitted `(x + 1) + 1` over `x: Int[0, 9]` into
    `Int[0, 20]`, `x * -1` over `x: Int[1, 9]` into `Int[-9, -1]`, and
    `-(x + 1)` over `x: Int[0, 9]` into `Int[-10, -1]`, with no caller claim
    map. Read the per-position harness ranges, result assertion, claim
    provenance and inferred source identities. Then route outer applications
    with a plain-Integer parameter nested under add and negate; an inferred
    result outside `i64`; and nested operands provoking
    `OperandRangeLimitExceeded`, `OperandUnsupported`, `OperandTypeMismatch`
    and `InvalidBody` beside one of the supported items (FR-014-AC-46,
    FR-015-AC-104). Exercise malformed shapes at the inference seam when
    Contract IR admission rejects them first.

## Expected Results

1. `items` holds four entries in the order `2`, `4`, `7`, `11`. Each carries
   the `kani` candidate and `KindOutput::Kani`. Each record equals the FR-015
   record at the same ascending position, except that `request_index` is the
   driver's index. The three generated nodes carry the harness whose
   `identity.harness_symbol` equals their record's `harness_symbol`. The
   `Unsupported` node carries its FR-015 reason and no harness.
   `rejected` is empty.
2. The signature carries none of the four settlement inputs.
3. `Err(BackendKindDisagrees { request_index: 5, backend, routed: Kani,
   converted: None })`, with no output.
4. `Err(DuplicateRequestIndex { request_index: 3 })` for the first call, and
   `Err(MissingKindContext { kind: Kani })` for the second. Neither returns
   output.
5. `Err(Kani(InvalidUnwind { unwind: 0 }))` and
   `Err(Kani(InvalidSubjectPath))`, each with no output.
6. `rejected == [Kani]`, and both items have no harness. The record at request
   index `9` is `InvalidRequest { DuplicateItem { first_index: 6 } }`, naming
   the driver's index and not the Kani-group position `0`.
7. `Ok` with empty `items` and empty `rejected`.
8. The repeated runs and the reversed slice produce equal results. The
   single-node harness bytes are equal under both request indexes. Only the
   output's
   `request_index` and `backend` differ.
9. The dispatch is an exhaustive `match` over `BackendKind` with no `_` arm.
   `GenerationContexts::has` is an exhaustive `match` too, and `GenerationContexts`
   has exactly one field per `BackendKind::ALL` member.
10. The record is `Supported`; its harness names `quire.op.integer.add` with
    arguments `[0, 9]` and `[1, 1]`.
11. The `integer.rem` node's record is `Unsupported` with `no_derivable_claim`
    naming the node and `operation_not_derivable`, and carries no harness; the
    group is not rejected and the three siblings keep their harnesses.
12. The absent node is `invalid_request` `unknown_node` and rejects the group;
    the unbounded and missing or wrong-form bound nodes are `requires_bound`;
    the function node is `blocked_on_upstream`; the unreadable bound is
    `oracle_refused` and the unsatisfiable one `unsatisfiable_bound`, none of them
    rejecting the group. The twice-routed node yields the first copy's supported
    record at `3`, `duplicate_item{first_index: 3}` at `5`, a rejected group and
    one claim. Underivable claims have provenance `underived`.
13. The claim map is `Some`, holds the FR-014 entries for the derivable nodes
    and a `NoDerivableClaim` entry for the others, ordered by node id; it is
    `None` for an empty routed set.
14. The first three records are `Supported`; each harness has one range per
    operand (`[0, 9]` and `[10, 20]`; `[1, 9]`; `[1, 9]` and `[100, 200]`) and its
    generated source asserts the result against the result bound. The `p`
    additions settle `requires_bound` and carry no harness.
15. `oracle_artifacts` is `Some` and equals FR-014's artifacts byte for byte. Each
    `Generated` claim's symbol is defined in `src/lib.rs` and appears in the harness
    source. The `integer.rem` node returns the artifacts FR-014 gives an empty item
    set. Nothing routed gives `None`.
16. `x + 1`, `x + y` and `-z` are `Supported` with arguments `[0, 9]` and
    `[1, 1]` (the literal at its own value); `[0, 9]` and `[10, 20]`; and
    `[0, 9]`, and each generated source asserts the result against `[0, 10]`,
    `[10, 29]` and `[-9, 0]`. `x + p` and `x + n` settle `requires_bound`, and
    the twice-narrowed sum is `oracle_refused` with `AmbiguousBound`; none
    carries a harness.
17. `x + 100` is `unsupported` as `result_bound_unreachable` with range
    `[0, 10]` and reachable `[100, 109]`; `x + 10^23` is `unsupported` as
    `domain_not_representable_in_i64` naming `100000000000000000000000` as both
    endpoints; `x + "a"` is `oracle_refused` with `OperandTypeMismatch` and
    `x + 2` with `AmbiguousBound`; none carries a harness.
18. `index_harnesses` returns `Err(DuplicateHarness)` naming `b_module::x_proof`
    for the second of two harnesses sharing the symbol `x_proof`, and indexes two
    harnesses with distinct symbols.
19. The three outer items are `Supported` with `ir_confirmed` claims and
    harness ranges `[1, 10]`, `[1, 1]`; `[1, 9]`, `[-1, -1]`; and
    `[1, 10]`, respectively. Each inferred range records the inner application
    id in its harness identity, and each harness asserts its declared outer
    result bound. A nested plain-Integer parameter is `requires_bound` naming
    Integer with no harness. An inferred range with either endpoint outside
    `i64` is `unsupported` as `domain_not_representable_in_i64`, with both
    exact endpoints. Each `OperandRangeLimitExceeded`, `OperandUnsupported`,
    `OperandTypeMismatch` and `InvalidBody` refusal settles `unsupported` with
    `oracle_refused` carrying the exact FR-014 variant and its node, position
    or limit fields, and no harness. The supported item's claim and harness
    are unchanged.
20. The RR-16 items retain separate `[0, 10]` and `[0, 20]` result
    assertions, separate conversion occurrences, and distinct obligation and
    harness identities despite sharing the add node. Reordering the routed
    input changes none of those per-index values (FR-022-AC-26).
21. The guarded pair's proof conditions include their own ordered QSL guard
    sequences with opposite required outcomes; wrong-branch assignments do
    not establish either claim. The unguarded item has no added guard
    (FR-022-AC-27).
22. Each missing or mismatched keyed record yields a typed refusal naming
    the request index and occurrence, with no generated artifact or identity
    for that request. No other record of the shared node is substituted
    (FR-022-AC-28).
