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
  order. The driver supplies that full order with each root typed as a driver
  item or a context-only root. Driver request indexes count only driver items;
  context-only roots have accounting but no independent settlement or artifact.
  Repeated requested IDs remain distinct positions, including when context
  roots are interleaved.
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
  record to every root in the full lowering order by both accounting position
  and exact node ID, then map each driver item to its own accounting position.
  It SHALL preserve duplicate IDs and refuse a missing, extra or mismatched
  record, including a context-only record, before generating an artifact. A
  context-only record SHALL NOT be counted as a driver item or substituted for
  one. For a `lowered` driver record, CG SHALL read
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
  graph (IR-766). For an admitted `lowered` item, that traversal is total over
  its required root, bounds and selected dependencies under Contract IR's
  FR-035 admission contract. CG SHALL retain its exact typed keys and each
  existing claim-family ordering and field semantics. An unresolved required
  reference SHALL NOT reach CG as an admitted `lowered` item: Contract IR's
  typed item accounting or strict reader owns its refusal. A reader refusal
  or incomplete result remains the whole-request outcome described above.
  CG SHALL NOT reconstruct a checked graph or perform a second lowering to
  fill missing keys.
- CG SHALL preserve independent sibling positions. CG SHALL NOT drop or change
  a representable sibling because another item's typed refusal. If Contract IR reports
  the package byte ceiling and `wire()` is absent, CG SHALL treat the whole
  call as refused and generate no sibling artifact, consistent with every
  record's `failed` disposition.
- For a scalar expression item, the driver SHALL supply the exact QSL-checked
  application occurrence and optional conversion occurrence described by
  Contract IR FR-049. The lowering request SHALL place each selected conversion
  as a context-only root in that driver's contiguous context span, before the
  next driver item. CG SHALL match the full root sequence and each root key to
  admitted accounting, check the selected occurrence keys and context-root
  association, and read the result bound only through the admitted FR-049
  `result_context` view. Two driver items sharing one expression key but
  selecting different occurrence/context pairs remain distinct; repeating the
  same pair retains the scalar duplicate refusal. A scalar item without an
  applicable result selection receives typed `IncompleteContext`. CG SHALL NOT
  infer a conversion by scanning consumers or compare source-region overlap:
  valid application and conversion regions may differ. The QSL driver owns the
  per-item pair; FR-330 alone cannot detect a swap between two represented
  direct narrows without an admitted binding witness.
- CG SHALL accept a `dependency_reference` only through the already admitted
  FR-330 package whose reader validated the source lock's selected dependency
  identity and `package_id`. CG SHALL retain the selected dependency key and
  exact target node identity; it SHALL NOT resolve the reference through an
  identically keyed local node, an unselected dependency or a display name.
  A dependency target outside a backend's representable family SHALL receive
  that backend's typed refusal without replacing its meaning.
- CG SHALL consume an admitted FR-322 `protocol` node with
  `semantic_form: "protocol_reference"` as the sole protocol-target carrier in
  the one FR-330 `semantic_graph.nodes` collection. CG SHALL select its target
  by the exact local typed key in `body.target`, whose sole dependency and
  `semantic_type` match the target under FR-322-AC-66. The selected dependency
  packages SHALL NOT supply a missing local protocol target, even when one
  contains the same key. CG SHALL NOT derive a target from a display name,
  generic reference in another node, or separate protocol collection. A
  backend unable to represent the admitted protocol form SHALL return its
  typed refusal with the requested identity and source occurrences and emit
  no artifact. FR-322's reader owns absent, wrong-tag and type-mismatched
  target refusals before CG consumes the package.
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
| FR-039-AC-2 | A mixed request with a representable clause or function and an independent `unsupported`, `requires_bound` or `invalid_input` sibling retains one typed decision per driver position, including duplicate requested IDs, and emits only the representable sibling's artifact. An interleaved context-only root retains its own exact accounting position and key without creating a driver decision. For two QSL scalar items sharing one expression key but selecting different checked application/conversion pairs, CG retains each pair's own admitted FR-049 result bound; a conversion outside its driver's contiguous context span yields typed `IncompleteContext`, while repeating one pair retains `DuplicateRequest`. An IR accounting refusal retains its exact record payload and available source occurrences; a backend refusal after a `lowered` record retains that requested identity, the admitted `lowered` accounting disposition and available source occurrences, with its own distinguishable typed cause. A CG scalar `RequiresBound` refusal after `lowered` is not relabelled as an IR `requires_bound` record. An absent ID has no fabricated region. | Test |
| FR-039-AC-3 | A selected dependency function reference resolves only through the reader-validated selected package identity and target key. A missing, wrong-identity or stale selected dependency refuses before CG; an identically keyed local node or extra unselected dependency cannot redirect generation. | Test |
| FR-039-AC-4 | For a genuinely admitted FR-330 package, CG binds a `protocol`/`protocol_reference` carrier's exact `body.target` to the local `protocol` node, retaining its typed key, requested identity and source occurrences; changing that local target and recomputing the admitted carrier and package identities changes the selected target and package identity. A matching key only in a selected dependency does not bind locally. An absent local target, a present non-`protocol` target and a target with unequal `semantic_type` are refused by the FR-322 reader before CG with the respective `missing_declaration`/`missing-name` or `ill_typed`/`operator-ineligible` cause at `/body/target`; no artifact is emitted. An ordinary local reference or display text does not supply protocol-target binding. A backend that cannot encode an admitted protocol form returns its typed refusal and no artifact. | Test |
| FR-039-AC-5 | A malformed or noncanonical FR-330 wire, wrong version, invalid accounting/source map, stale semantic identity or incomplete reader result reaches no CG consumer and emits no artifact. CG preserves the reader's typed refusal or limit result and performs no second parse or checked-graph fallback. | Analysis |
| FR-039-AC-6 | An item whose admitted package accounting is `failed`, `invalid_body` or `body_incomplete`, or whose `lowered` form is unrepresentable by the selected backend, emits no artifact and retains a distinct typed cause; the package byte-ceiling case emits no artifact for any sibling. | Test |
| FR-039-AC-7 | Generated scalar, composite equality and exact function claims each serialize `contract_package_identity` equal to the admitted FR-330 `identity` and `node_id` equal to the requested `CheckedNodeId`, with no generated-claim `ir_id` key. Each retains exact `semantic_type`, admitted `source_map`, family-specific provenance, and `package_id` denoting the originating checked package; each serializes typed `claims`, `bounds` and `dependencies`. For the same requested node in two genuinely admitted packages with different identities, the claim identity differs; for two different requested nodes in one admitted package, it also differs. A downstream consumer can join a claim to its admitted meaning by this pair and can use request index only to distinguish repeated request positions. | Test |
| FR-039-AC-8 | For each admitted `lowered` item, Contract IR's authoritative traversal is total over the required root, bounds and selected dependencies, and the generated claim's reached claim, bound and dependency keys and dependent declaration closure equal its output. An unresolved required reference cannot reach CG as an admitted `lowered` item: Contract IR records a typed item refusal or its strict reader refuses the package; a refused or incomplete reader result yields no item claim or artifact. Inspection finds no per-node ID from a second lowering or wire-fragment hash. | Test |

## Verification

### Reader-refusal analysis

Analysis checks the CG type boundary in `src/routed/capability.rs`
(`negotiate_contract_package`), `src/routed/generate.rs` (`generate_routed`),
`src/oracle/equality/mod.rs` (`generate_composite_equality_oracles`) and
`src/oracle/function/mod.rs` (`generate_exact_function_oracles`): each public
package parameter is `&ContractPackageV1`. The crate-internal scalar consumers
`generate_contract_scalar_oracles` and `preflight_contract_scalar_claim` in
`src/oracle/scalar/mod.rs`, and `classify_scalar_form` and `classify_node` in
`src/kani/generate/negotiate.rs`, also take `&ContractPackageV1`.
`src/core/contract_package.rs` constructs `PackageItems` from
`&ContractPackageV1`. A source search of CG production APIs checks that none
constructs `ContractPackageV1` from raw wire bytes or admits `Refused` or
`Incomplete` as a package. This establishes the admitted-package type boundary;
no CG path admits a package without Contract IR's reader or lowerer.

The production call in `src/oracle/function/mod.rs` is a known deviation from
AC-5's result-preservation clause: `checked_integer_function_application_is_supported`
drops `Refused` and `Incomplete` with `_ => return false`. Its caller in
`src/routed/capability.rs` then returns `KaniFormRefused` with
`OperationNotRendered(quire.op.function.call)`, replacing the reader's typed
refusal or limit result with a CG backend refusal. IR-892 tracks a tests-first
fix under FR-019-AC-48 through FR-019-AC-52 and TC-030: the routed negotiation
result must carry a CG-owned serializable projection of the reader's public
`CheckedPackageRefusal` or `CheckedPackageIncomplete` fields as a whole-request
`EnvelopeRefusal`, while an
admitted read keeps its existing supported or genuine backend-refusal decision.
These criteria are planned; this deviation remains until the code PR lands.
The current oracle-function helper re-lowers `CheckedPackageV2` and reads the
resulting wire. AC-48 and AC-49 describe that current helper's observable
outcome; on the FR-330 production path, only an `Admitted` package reaches CG
after the caller's authoritative read. The code PR must use that admitted
result without re-reading or falling back to the checked graph. AC-5's
preservation clause remains unmet; this Analysis is
planned and does not claim full conformity.

Contract IR owner evidence on origin/main at 4213b98 separates criterion text
from test tags. [FR-035-AC-10](ix://agent-ix/quire-contract-ir/FR-035-AC-10)
states the wrong-version refusal, [FR-035-AC-11](ix://agent-ix/quire-contract-ir/FR-035-AC-11)
states the incomplete reader result, and [FR-035-AC-5](ix://agent-ix/quire-contract-ir/FR-035-AC-5)
owns the source-map class. Each has tagged TC-047 tests in Contract IR's
`tests/it/complete_v1_contract_package.rs`. Stale own identity and stale
selected-dependency identity are stated by
[FR-035-AC-2](ix://agent-ix/quire-contract-ir/FR-035-AC-2), but its tagged
TC-044 tests are in `tests/it/complete_v1_checked_package.rs`; IR's matrix
marks TC-044 planned/unrun for FR-330. TC-047 tests of stale identities are
tagged AC-5, not AC-2. Malformed and noncanonical wires and invalid accounting
have AC-5-tagged TC-047 tests, but no FR-035 criterion states those FR-330
reader obligations. IR-893 tracks that owner-evidence gap. These IR tests are
owner evidence only; CG implementation coverage remains planned.

## Dependencies

QSpec `ix://agent-ix/quire-specification/FR-330` owns the closed package and
reader contract. Contract IR `ix://agent-ix/quire-contract-ir/FR-035` emits it.
The CG requirement covers consumption of FR-322-AC-66's local carrier through
FR-330-AC-4's one graph; the FR-322 reader owns target admission and refusal.
TC-054's worked FR-322 carrier can exercise positive and adverse strict-reader
controls now. QSL-764 must produce the carrier from source before genuine
QSL/IR positive and changed-binding controls can verify CG consumption.
Contract IR's planned IR-766 admitted-package traversal is required for the
reached-key portion of generated claims.
