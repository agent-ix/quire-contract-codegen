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
- A `KaniGenerationContext` with `KaniToolPins::pinned()`, a valid subject
  path, unwind `1` and a valid attestation context.
- The routed backend `Candidate { identity: "kani", manifest_digest: <any> }`.

## Test Procedure

1. **Differential with FR-015 (AC-2, AC-8).** Route the three generated nodes
   and the `Unsupported` node at the non-contiguous request indexes `7`, `2`, `11`
   and `4`, with kind `Kani`, and call `generate_routed`. Separately call
   `negotiate_kani_obligations` with the same four nodes as `ScalarClaim` items
   in ascending request-index order (`2`, `4`, `7`, `11`) and the same context.
   Compare record by record and harness by harness.
2. **No settlement (AC-3).** Run the FR-019-AC-5 source scan (TC-030) over the
   crate including the new module. Also assert, from the public signature, that
   `generate_routed` takes no `BackendDescriptor`, `Candidates`,
   `ExtentClassification` or `CapabilityKind`.
3. **Kind disagreement (AC-4).** Route one item whose backend identity is
   `"not-a-backend"`, with kind `Kani`, at request index `5`, beside a valid
   item.
4. **Duplicate index and missing context (AC-5).** Route two items at request
   index `3`. Separately, route a valid Kani item with `GenerationContexts {
   kani: None }`.
5. **Kani group refusal (AC-6).** Call once with pins that differ from
   `KaniToolPins::pinned()` in one field, once with unwind `0`, and once with
   the subject path `"not a path"`.
6. **Kani group rejection (AC-7).** Route the same node at request indexes `9`
   and `6`.
7. **Empty (AC-8).** Call with no routed items and `kani: None`.
8. **Determinism (AC-9).** Repeat step 1 twice, then with the routed slice
   reversed, and compare the results. Route one node alone at request index `0`
   under manifest digest `A`, then at request index `40` under manifest digest
   `B`, and compare its harness `rust` and `record` bytes and its
   `identity_sha256`.
9. **Exhaustive dispatch (AC-1).** Inspect the generation dispatch and the
   `GenerationContexts` definition.
10. **Derivation without a claim map (FR-022-AC-10).** Route the
    bounded-increment node with no claim map anywhere in the context.
11. **Underivable sibling (FR-022-AC-11, FR-015-AC-15).** Route the `integer.rem`
    node beside the three generated nodes.
12. **Returned claim map (FR-022-AC-12).** Compare `RoutedGeneration.claim_map`
    with `generate_exact_scalar_oracles` over `derive_exact_scalar_items` for
    the same nodes, and with `None` when nothing is routed.

## Expected Results

1. `items` holds four entries in the order `2`, `4`, `7`, `11`. Each carries
   the `kani` candidate and `KindOutput::Kani`. Each record equals the FR-015
   record at the same ascending position, except that `request_index` is the
   driver's index. The three generated nodes carry the harness whose
   `identity.harness_symbol` equals their record's `harness_symbol`. The
   `Unsupported` node carries its FR-015 reason and no harness.
   `rejected` is empty.
2. The scan is green and finds no `Disposition` built in the new module. The
   signature carries none of the four settlement inputs.
3. `Err(BackendKindDisagrees { request_index: 5, backend, routed: Kani,
   converted: None })`, with no output.
4. `Err(DuplicateRequestIndex { request_index: 3 })` for the first call, and
   `Err(MissingKindContext { kind: Kani })` for the second. Neither returns
   output.
5. `Err(Kani(UnpinnedBackend { .. }))` naming the differing field,
   `Err(Kani(InvalidUnwind { unwind: 0 }))` and
   `Err(Kani(InvalidSubjectPath))`, each with no output.
6. `rejected == [Kani]`, and both items have no harness. The record at request
   index `9` is `InvalidRequest { DuplicateItem { first_index: 6 } }`, naming
   the driver's index and not the Kani-group position `0`.
7. `Ok` with empty `items` and empty `rejected`.
8. The repeated runs and the reversed slice produce equal results. The
   single-node harness bytes and `identity_sha256` are equal under both
   request indexes and both manifest digests. Only the output's
   `request_index` and `backend` differ.
9. The dispatch is an exhaustive `match` over `BackendKind` with no `_` arm.
   `GenerationContexts::has` is an exhaustive `match` too, and `GenerationContexts`
   has exactly one field per `BackendKind::ALL` member.
10. The record is `Supported`; its harness names `quire.op.integer.add` with
    arguments `[0, 9]` and `[0, 9]`.
11. The `integer.rem` node's record is `Unsupported` with `no_derivable_claim`
    naming the node and `operation_not_derivable`, and carries no harness; the
    group is not rejected and the three siblings keep their harnesses.
12. The claim map is `Some`, holds the FR-014 entries for the derivable nodes
    and a `NoDerivableClaim` entry for the others, ordered by node id; it is
    `None` for an empty routed set.
