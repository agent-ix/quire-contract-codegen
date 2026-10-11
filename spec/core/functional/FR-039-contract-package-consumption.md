---
id: FR-039
title: "Consume one admitted ContractPackage for production decisions"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-330
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-035
    type: references
---
# FR-039: Consume one admitted ContractPackage for production decisions

## Description

When the driver submits a complete-V1 request for production negotiation or
generation, this generator SHALL base every decision about its selected IR
meaning on the one admitted QSpec FR-330 `ContractPackage` emitted for that
request. Contract IR owns its lowering and sole strict reader; CG consumes the
typed admitted value and does not define another package schema or reader.

This requirement governs the production oracle and Kani paths, including the
[FR-019](../../routed/functional/FR-019-capability-settlement.md) settlement
and [FR-022](../../routed/functional/FR-022-routed-generation.md) generation
arms. A caller-supplied subject path, unwind or backend descriptor remains a
CG input for the purpose its own requirement defines; it is never a substitute
for package meaning or a source of native bounds.

## Inputs

- One `ContractPackageV1` admitted by Contract IR's FR-330 reader or returned
  as the `wire()` value of a successful Contract IR lowering call. The driver
  supplies the exact admitted selected dependency packages required by the
  source lock to that reader. The package holds its explicit lowering profile,
  key-ordered semantic graph, ordered typed feature accounting, total source
  map and semantic identity.
- Requested items identified by their exact `CheckedNodeId` and request
  index, plus the existing FR-019 routing envelope and backend descriptors.
  The package's `feature_accounting` order is the lowering call's request
  order; repeated requested IDs remain distinct positions.
- CG-owned generation context and customer subject inputs required by the
  chosen backend arm. These inputs do not alter the admitted package.

## Outputs

- One production decision for each request position, retaining its exact
  requested node ID and source occurrences: a supported routing/generation
  result, a typed CG refusal derived from its package accounting record, or
  the existing backend-specific refusal after a `lowered` record.
- No generated artifact for a request whose accounting disposition is not
  `lowered`, or whose represented meaning the selected backend cannot encode.
- A whole-request refusal with no artifact if the FR-330 reader refuses or
  stops incomplete before CG receives an admitted package.
- For each generated scalar, composite equality or exact function item, a
  public claim identified by the admitted FR-330 `identity` together with the
  exact requested `CheckedNodeId`.

## Behavior

- CG SHALL inspect the admitted package's seven-member
  `feature_accounting` vocabulary as a closed typed enum. It SHALL match a
  record to the requested item by both request position and exact node ID,
  preserving duplicate positions, and refuse a missing, extra or mismatched
  record before generating an artifact. For a `lowered` record, CG SHALL read
  its root and reached closure from `semantic_graph` by typed key. It SHALL
  never infer a success from a missing record or substitute a graph node for a
  refused item.
- CG SHALL preserve `unsupported`, `requires_bound`, `invalid_input`,
  `failed`, `invalid_body` and `body_incomplete` as distinguishable typed
  causes. It SHALL retain each record's exact payload, requested identity and
  the `source_map.requested_items` and `source_map.refusals` occurrences the
  admitted package gives it. An absent requested key retains an empty
  occurrence list and its exact identity; CG SHALL NOT invent a source region.
  CG SHALL NOT convert a package refusal to backend support.
- CG SHALL settle and generate each `lowered` item from its represented typed
  node, type, operation, bounds, clause or function and reached local
  dependencies in that same package. It SHALL use the package's exact source
  lock, selected profile and identity when forming an artifact or proof claim.
  The original `CheckedPackageV2` graph, a separate
  `CompleteLoweringResultV2.records` slice, display text and reconstructed
  body shape SHALL NOT decide the same item's capability, bounds, source
  location or generated expression.
- In each public `GeneratedScalarClaim`, `GeneratedCompositeEqualityClaim` and
  `GeneratedExactFunctionClaim`, CG SHALL serialize `contract_package_identity`
  as the exact admitted FR-330 `identity` and `node_id` as the exact requested
  `CheckedNodeId`. The pair identifies the claim; request index distinguishes
  repeated positions in the surrounding decision, not the claim's semantic
  identity. CG SHALL retire the generated claim's `ir_id` member, including its
  serialized `ir_id` key: FR-330 supplies no per-node IR semantic identity.
  The containing claim-map entry SHALL retain its requested `node_id`, and
  existing generated-claim source correspondence, semantic type, operation or
  descriptor, schedule, origin and checked bounds SHALL retain their admitted
  or request-defined provenance. The existing `package_id` provenance on
  equality and function claims SHALL remain the originating checked-package
  identity, distinct from `contract_package_identity`; the scalar claim SHALL
  expose that same originating `package_id`. In all three generated claims,
  `claims`, `bounds` and `dependencies` SHALL serialize as typed
  `CheckedNodeId` lists. Generated scalar claims SHALL retain `checked_bounds`,
  while their containing `ExactScalarClaim` SHALL retain `operation`
  provenance. Equality claims SHALL retain `descriptor`,
  `declaration_keys`, `declaration_runtime_keys` and `schedule`; function
  claims SHALL retain `function`, `function_origin` and `declaration_keys`.
  `semantic_type` and `source_map` SHALL remain present in each family, with
  source correspondence taken from the admitted package. CG SHALL NOT derive
  a replacement per-node identity by lowering again or hashing a wire fragment.
- When forming generated scalar, equality and function claims, CG SHALL obtain
  reached claim, bound and dependency keys, and any declaration closure based
  on them, from Contract IR's authoritative traversal of the admitted FR-330
  graph (IR-766). CG SHALL retain the resulting exact typed keys and each
  existing claim-family ordering and field semantics. If that traversal cannot
  resolve a required root, bound or selected dependency, CG SHALL emit no
  generated claim or artifact for the item and SHALL retain a typed item
  refusal with the exact requested ID and available admitted source
  occurrences. A reader refusal or incomplete result remains the
  whole-request outcome described above. CG SHALL NOT reconstruct a checked
  graph or perform a second lowering to fill missing keys.
- CG SHALL preserve independent sibling positions. CG SHALL NOT drop or change
  a representable sibling because another item's typed refusal. If Contract IR reports
  the package byte ceiling and `wire()` is absent, CG SHALL treat the whole
  call as refused and generate no sibling artifact, consistent with every
  record's `failed` disposition.
- CG SHALL accept a `dependency_reference` only through the already admitted
  FR-330 package whose reader validated the source lock's selected dependency
  identity and `package_id`. CG SHALL retain the selected dependency key and
  exact target node identity; it SHALL NOT resolve the reference through an
  identically keyed local node, an unselected dependency or a display name.
  A dependency target outside a backend's representable family SHALL receive
  that backend's typed refusal without replacing its meaning.
- CG SHALL consume protocol-tagged nodes and any FR-322 local keyed
  references in the single `semantic_graph` under their admitted typed forms.
  If the selected backend cannot represent an admitted protocol form, CG SHALL
  return a typed refusal and no artifact. FR-330 defines no separate
  `protocol_references` field or protocol-target reference carrier; CG SHALL
  NOT infer one from text or invent target binding.
- CG SHALL pass malformed, noncanonical, wrong-version, mismatched-identity,
  incomplete-source-map and stale-selected-dependency inputs through Contract
  IR's single FR-330 reader. A `Refused` or `Incomplete` reader outcome SHALL
  terminate the request before negotiation or generation; CG SHALL preserve
  its typed code/cause, pointer or locus and measured limit details rather
  than parsing the wire, repairing it or falling back to the original graph.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-039-AC-1 | Each of two genuine QSL-admitted source graphs with a different clause or pure-function operation lowers through IR to its own admitted FR-330 package. CG's production settlement and generation use each package's typed graph and accounting for its exact request index and node ID; supplying the two packages with each one's corresponding node ID at the same request position changes the generated bytes to reflect the represented operation while backend and generation context stay fixed. If the production API still accepts an original checked graph, holding that legacy input fixed across the two calls does not prevent the package-driven byte change; if it no longer accepts one, its signature and call path have no original-graph fallback. Independently changing an admitted accounting disposition changes the item's decision and cannot replace the lowered-operation byte control. | Test |
| FR-039-AC-2 | A mixed request with a representable clause or function and an independent `unsupported`, `requires_bound` or `invalid_input` sibling retains one typed decision per position, including duplicate requested IDs, and emits only the representable sibling's artifact. Every refusal retains its accounting payload and available source occurrences; an absent ID has no fabricated region. | Test |
| FR-039-AC-3 | A selected dependency function reference resolves only through the reader-validated selected package identity and target key. A missing, wrong-identity or stale selected dependency refuses before CG; an identically keyed local node or extra unselected dependency cannot redirect generation. | Test |
| FR-039-AC-4 | Protocol-tagged nodes and their admitted local keyed references remain in the one semantic graph and receive an exact generation or typed unsupported result. A backend does not infer a protocol-target binding that FR-330 does not encode. | Test |
| FR-039-AC-5 | A malformed or noncanonical FR-330 wire, wrong version, invalid accounting/source map, stale semantic identity or incomplete reader result reaches no CG consumer and emits no artifact. CG preserves the reader's typed refusal or limit result and performs no second parse or checked-graph fallback. | Test |
| FR-039-AC-6 | An item whose admitted package accounting is `failed`, `invalid_body` or `body_incomplete`, or whose `lowered` form is unrepresentable by the selected backend, emits no artifact and retains a distinct typed cause; the package byte-ceiling case emits no artifact for any sibling. | Test |
| FR-039-AC-7 | Generated scalar, composite equality and exact function claims each serialize `contract_package_identity` equal to the admitted FR-330 `identity` and `node_id` equal to the requested `CheckedNodeId`, with no generated-claim `ir_id` key. Each retains exact `semantic_type`, admitted `source_map`, family-specific provenance, and `package_id` denoting the originating checked package; each serializes typed `claims`, `bounds` and `dependencies`. For the same requested node in two genuinely admitted packages with different identities, the claim identity differs; for two different requested nodes in one admitted package, it also differs. A downstream consumer can join a claim to its admitted meaning by this pair and can use request index only to distinguish repeated request positions. | Test |
| FR-039-AC-8 | Generated claims' reached claim, bound and dependency keys and dependent declaration closure equal Contract IR's authoritative traversal over the admitted package. A missing required root, bound or selected dependency yields a typed item refusal retaining the exact requested ID and admitted source occurrences, with no generated claim or artifact; a refused or incomplete reader result yields no item claim or artifact. No per-node ID from a second lowering or wire-fragment hash appears. | Test |

## Dependencies

QSpec `ix://agent-ix/quire-specification/FR-330` owns the closed package and
reader contract. Contract IR `ix://agent-ix/quire-contract-ir/FR-035` emits it.
The CG requirement covers consumption only. Protocol-target binding needs a
QSpec/IR carrier before it can become a CG consumer criterion.
Contract IR's planned IR-766 admitted-package traversal is required for the
reached-key portion of generated claims.
