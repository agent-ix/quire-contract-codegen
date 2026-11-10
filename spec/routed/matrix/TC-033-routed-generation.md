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
  `Int[0, 10]` and `Int[0, 20]`.
- The IR-304 guarded package named in FR-022 ("Path conditions in the scalar
  harness"): one QSL-checked package with `f`, `g`, `h`, `k`, `c` and `j` and their
  requirement records, so guarded and unguarded items route in one
  `generate_routed` call over one source package.

## Test Procedure

For FR-022-AC-30, pass a canonical plain reader-admitted V1 package containing
one view-independent integer operation and view-dependent division and
quantity-conversion operations to `generate_routed`. The integer-only request
retains its existing result. Each view-dependent request returns the whole-call
typed `UnauthenticatedOperationView { request_index, node_id }` before output;
the mixed request names the lowest view-dependent `request_index`, unchanged
by permuting the routed-item slice. Repeat against
the matching V2 source's authenticated V1 handoff and check that division uses
its admitted law profile and conversion produces its exact oracle while its
Kani capability settles as `OperationNotRendered`. TC-024 and TC-030 own the
oracle and capability details.

For FR-022-AC-29, use an admitted package with bound driver items at indexes
0 and 2 and an excluded accounting-only position at index 1. Construct a
routed item through the public descriptor constructor with index 1 and the
excluded position's exact node ID. Assert that `generate_routed` refuses the
whole call as `ExcludedDriverIndex { request_index: 1, node_id }` before any
generation arm, oracle or harness runs. Route the same descriptor at bound
indexes 0 and 2 as valid controls, checking that index 2 keeps its own
selection rather than the second FR-331 envelope position.

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

For FR-022-AC-26 and AC-28 (slice a), build each item's `RoutedRequirement` and the package's
`RequirementRecords` from the QSL `RequirementItem` and `RequirementRecord` values for the
checked RR-16 package, with node keys mapped to `CheckedNodeId`. Route both RR-16 items
together, then reverse their input order, and compare each request index's disposition and
result assertion. Exchange only the two items' `RoutedRequirement` values and expect
`ApplicationMismatch` at the lower index. Then, one change at a time: remove one item's
requirement; remove its record; change its occurrence ordinal to a recorded occurrence that is
not the authenticated application; replace the item's, then the record's, result bound with the
other occurrence's; replace the item's, then the record's, conversion context with the other
occurrence's; and add a guard to the record only. Keep the other occurrence's record of the same
node present in every case, so a node-id fallback would generate instead of refusing. Assert the
whole-call `RequirementRecord` variant, request index and occurrence, and that no oracle
artifact, claim map or harness is returned. Make two items fail and check the lower index is
reported. Assert as a precondition that each admitted `ContractResultBoundV1.node` differs from
the record's V2 `result_bound`, so a raw cross-frame comparison would refuse the valid input.

For FR-022-AC-36, route an existing `NotApplicable` item with no record for its node as the
unchanged control. Then route the FR-022-AC-27 `x + 1` node, whose records carry `p` guards,
with a `NotApplicable` selection, without and then with one occurrence's requirement; expect
`OccurrenceNotSelected` for both. Route the same node with an `OwnResult` selection and its
requirement and check it reaches FR-022-AC-38 (slice a) or FR-022-AC-27 (slice b).

For FR-022-AC-37, over RR-16 combine a `ScalarSelectionMismatch` at the higher request index
with a `RecordMissing` at the lower; in the FR-022-AC-30 plain-read package give the
view-independent integer item a requirement with no record beside the view-dependent item; and,
in one RR-16 item, set a wrong result bound with a wrong path condition, then the other recorded
occurrence's ordinal with a wrong conversion context, then an ordinal with no record. Read the
single reported refusal each time. In the IR-304 guarded package, route `k`'s `x + m` and `c`'s
`n >= 0` with valid records in both slices, and in slice (b) `j`'s `x + 100` item, whose record
carries `[(x > 20, holds=true), (x + 1 > 5, holds=true)]` with both guard nodes as context roots;
read each disposition.

For FR-022-AC-38 (slice a only), make one `generate_routed` call over the IR-304 guarded package
routing `f`'s then item, `h`'s `x + 100` and `g`'s `x + 1` with valid records, and read each
disposition, reason and harness.

For FR-022-AC-27 and AC-31 to AC-35 (slice b), route the QSL-emitted packages named in each
criterion, with each guard's node as a `ContextRoot` in its item's context span. Inspect each
harness's `Statement::Assume` list and order, symbolic input ranges, `guard_inputs`,
`path_condition` and `requirement_occurrence`, and the persisted record JSON. For AC-27, also
exchange the two records' path conditions together with the items' projections and check the
assumptions exchange. For AC-31, generate the same scalar node through direct
`negotiate_kani_obligations` and compare its record bytes with the committed golden
`tests/fixtures/kani-scalar/direct-increment-record.json` that slice (a) captures from the
unchanged generator. For AC-32,
route `x + 1 > 5`, `n >= 0` over a plain-Integer `n`, `x < 10^23` and a guard reading a model
member beside a lowerable sibling, then drop one guard's context root. For AC-33 and AC-34, read the argument ranges and dispositions, and route
the unguarded `x + 100` control, the literal-left `20 < x` guard, and the Boolean
`f(p: Boolean, x: Int[0, 9]): Int[0, 10] { if p then (if p then 0 else x + 1) else 0 }` case. For AC-35, run both harnesses through FR-017 under installed
Kani and read the evidence outcome.

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
20. Both RR-16 items are `Supported` with result assertions `[0, 10]` and `[0, 20]` at their own
    request indexes, unchanged by reversing the slice. Exchanging the two `RoutedRequirement`
    values yields `RequirementRecord` with `ApplicationMismatch` at the lower index
    (FR-022-AC-26).
21. The then harness assumes `p` and the else harness `!p`, each once and before the oracle call,
    with one Boolean guard input and the guard listed in `path_condition`; `p` precedes `q` in
    the nested case; exchanging the path conditions exchanges the assumptions; the unguarded item
    has no guard input or assumption (FR-022-AC-27).
22. Each single change yields `RequirementRecord` with `RequirementMissing` (`occurrence: None`),
    `RecordMissing`, `ApplicationMismatch`, `ResultBoundMismatch`, `ConversionContextMismatch`
    or `PathConditionMismatch` respectively, naming the request index and occurrence, with no
    oracle artifact, claim map or harness; the lower failing index is reported; the fixture's
    represented bound ids differ from its V2 ids (FR-022-AC-28, FR-022-AC-26).
23. A routed item naming excluded driver index `1` yields the typed
    `ExcludedDriverIndex` whole-call refusal before any oracle or harness;
    routed items at bound indexes `0` and `2` retain their own selections
    (FR-022-AC-29).
24. The two RR-16 identities and records differ in `requirement_occurrence`, serialized as node,
    role and ordinal; the direct FR-015 record omits the field and equals the committed golden
    byte for byte (FR-022-AC-31).
25. Lowerable guards produce their assumptions; `x + 1 > 5`, `n >= 0`, `x < 10^23` and the
    model-member guard yield `path_guard_not_lowered` with `UnsupportedGuardForm`,
    `UnboundedGuardOperand`, `GuardLiteralNotI64` and `GuardRootNotLowered`, each without a
    harness and with the sibling unchanged; the missing guard root yields the whole-call
    `GuardContextMissing` (FR-022-AC-32).
26. The guarded `x + 100` has arguments `[6, 9]` and `[100, 100]` (`[0, 5]` with `holds=false`);
    the `x < y` item keeps `x` at `[0, 9]` with guard input `y` over `[0, 9]` (FR-022-AC-33).
27. The dead-branch item is `path_condition_unsatisfiable` naming the guard, `x` and
    `Integer { minimum: 0, maximum: 9 }`, with no harness, also for `20 < x`; the Boolean case
    names the inner `p` guard, `p` and `Boolean`; the unguarded control remains
    `result_bound_unreachable` (FR-022-AC-34).
28. The infeasible-guard harness reports `CoverUnsatisfied` and the feasible one `Verified`
    (FR-022-AC-35).
29. The `NotApplicable` control keeps its result; the recorded node and the attached requirement
    each yield `OccurrenceNotSelected`; the `OwnResult` route is accepted (FR-022-AC-36).
30. The reported refusals are `ScalarSelectionMismatch` at the higher index, `RequirementRecord`,
    `ResultBoundMismatch`, `ApplicationMismatch` and `RecordMissing`; `x + m` is
    `requires_bound` and `n >= 0` is `operation_not_rendered` in both slices; `j`'s two-guard
    `x + 100` is `path_guard_not_lowered` in slice (b) (FR-022-AC-37).
31. In slice (a), in one call, `f`'s then item and `h`'s item are `path_guard_not_lowered` with
    `guard_lowering_not_implemented` and no harness, and `g`'s item is `Supported` with a
    harness (FR-022-AC-38).

## Scoped evidence status (IR-1316)

FR-022-AC-26/27/28 and AC-31 to AC-38 are Planned (IR-304): the production routed item carries
no typed QSL occurrence-record/path-condition intake. Slice (a) covers AC-26, AC-28 and AC-36 to
AC-38; slice (b) covers AC-27 and AC-31 to AC-35 and retires AC-38 (FR-022, "Scoped evidence status"). Same-node selected-bound
controls do not establish record-key association, ordered guards or typed missing/mismatched-record
refusal. FR-022-AC-1 and AC-20 are separate analysis obligations; the source/API analysis is
recorded in FR-022 and receives no backend-execution credit.
