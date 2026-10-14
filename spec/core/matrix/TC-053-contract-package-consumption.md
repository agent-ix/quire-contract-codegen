---
id: TC-053
title: "Verify production decisions from one admitted ContractPackage"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-039
    type: verifies
---
# TC-053: Verify production decisions from one admitted ContractPackage

## Description

Verify [FR-039](../functional/FR-039-contract-package-consumption.md) through
the production negotiation and generation entries using genuine QSL-admitted,
IR-lowered FR-330 packages. The controls distinguish package decisions from
decisions the original checked graph or detached lowering records would make.

## Test Procedure

1. Admit a native QSL package with two independent clauses and a pure
   function, lower the requested items together through Contract IR, and pass
   its one admitted FR-330 value to the production CG entry. Record each
   request index, exact node key, source occurrence, typed disposition and
   generated artifact. Repeat with request order changed, including a repeated
   node ID at two positions.
2. Make one clause unrepresentable under the selected lowering profile while
   its sibling and function remain representable. Separately request an
   unbounded type and an absent node ID. Observe each accounting payload and
   source-map occurrence, including the absent ID's empty occurrence list.
   Exercise defensive `failed`, `invalid_body` and `body_incomplete` accounting
   through admitted typed package fixtures, and the package byte ceiling
   through IR lowering.
3. Admit a selected dependency package exporting a pure function and a source
   package calling it through `dependency_reference`. Compare the generated
   decision with a local node of the same target key and an extra unselected
   dependency present. Then omit the selected package, supply the wrong
   library identity, and supply a selected identity with a stale package ID.
4. Admit two genuine QSL source graphs differing in a clause or pure-function
   operation that the selected backend renders. Lower each source independently
   through IR, including its own source identity, exact request node ID and
   `lowered` accounting record; pass each resulting admitted FR-330 package to
   CG at the corresponding request position with the same backend and generation
   context. If the production API still takes a legacy original
   `CheckedPackageV2` argument, hold that argument at the first source graph
   for both calls. Assert that the generated oracle or harness bytes differ and
   encode the operation represented by each supplied package, despite that
   fixed legacy argument. If the production API takes only the admitted package,
   assert that its signature and call path contain no original-graph lookup or
   fallback, and make the same generated-byte comparison. Separately, lower a
   request whose valid admitted package has a different per-item accounting
   disposition and assert that CG changes its decision without emitting a
   substitute artifact. Do not construct a package by changing its graph or
   accounting outside the IR lowerer and strict reader, and do not claim the
   two changed packages originate from one checked source graph.
5. Through Contract IR's FR-330 reader, submit wrong-version, duplicate-member,
   noncanonical, wrong-kind, stale-identity, incomplete-source-map and
   accounting/request-mismatch wires, and a package with a stale selected
   dependency. Also stop reading at a declared limit. Confirm no refused or
   incomplete value is handed to CG. Inspect the production call graph for a
   second FR-330 parser or an original-graph decision path.
6. For one genuinely admitted package, generate scalar, composite equality and
   exact function claims at two different requested node IDs. For each family,
   inspect both the typed claim and serialized claim map: the generated result
   has the admitted `identity` as `contract_package_identity`, the requested
   `CheckedNodeId` as `node_id`, the originating checked-package `package_id`,
   `semantic_type`, admitted `source_map`, family-specific provenance, typed
   `claims`, `bounds` and `dependencies`, and no `ir_id` member. Compare
   reached claim, bound and dependency keys, and any declaration
   closure, with Contract IR's authoritative traversal of that admitted graph.
   Repeat one requested node in two independently lowered, admitted packages
   whose FR-330 identities differ. Assert that the two claim-identity pairs
   differ despite the equal node IDs; assert that the two nodes in the first
   package also have different pairs despite the equal package identity.
   Preserve repeated request positions without treating index as identity.
7. At the Contract IR lowering and strict-reader boundary, exercise inputs
   with an unresolved required root, bound or selected dependency. Assert that
   no such input becomes an admitted `lowered` item handed to CG; preserve
   Contract IR's typed item accounting, reader refusal or incomplete outcome,
   as applicable, and emit no generated claim or artifact. For a valid
   package with a `lowered` item, confirm the authoritative traversal returns
   a complete reached closure. Inspect the call path for a
   second lowering, reconstructed checked graph or wire-fragment-derived
   per-node identifier.

## Expected Results

The accepted package determines each production item by its request position
and exact typed identity. Supported siblings alone generate artifacts;
refusals preserve their distinct typed payloads and exact available source
occurrences. Selected dependency targets cannot be redirected. A changed
admitted package changes the applicable decision or output. In particular,
two independently lowered and admitted operation variants yield different
generated bytes even when a retained legacy checked-graph argument is fixed;
an API with no such argument has no graph fallback. Reader refusals and limit
stops reach no CG consumer and emit no artifact. No detached record, copied schema,
second parser or original-graph fallback decides an item.
Generated scalar, equality and function claim maps serialize the admitted
package identity plus requested node ID as the public claim identity, retain
the originating checked-package ID and their existing source/provenance fields,
and omit `ir_id`. The same-node/different-package and
different-node/same-package controls both change the identity pair. Reached
keys agree with Contract IR's total traversal of admitted lowered items;
unresolved required references never reach CG as lowered items, and reader
refusals stop the request.
