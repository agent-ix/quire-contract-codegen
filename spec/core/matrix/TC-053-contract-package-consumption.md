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
4. Submit a protocol-tagged node with an FR-322 local keyed reference through
   the same package. Compare exact typed handling with an admitted protocol
   form outside the selected backend's encoder. Inspect the consumed fields to
   confirm no separate protocol-target slot or text-derived binding is read.
5. Hold the original checked graph constant while supplying two independently
   admitted FR-330 packages with different represented operation or accounting
   members. Compare CG's production decisions and artifact bytes. Conversely,
   vary a retained original graph value outside the admitted package and
   verify the same admitted package gives the same decision for that item.
6. Through Contract IR's FR-330 reader, submit wrong-version, duplicate-member,
   noncanonical, wrong-kind, stale-identity, incomplete-source-map and
   accounting/request-mismatch wires, and a package with a stale selected
   dependency. Also stop reading at a declared limit. Confirm no refused or
   incomplete value is handed to CG. Inspect the production call graph for a
   second FR-330 parser or an original-graph decision path.

## Expected Results

The accepted package determines each production item by its request position
and exact typed identity. Supported siblings alone generate artifacts;
refusals preserve their distinct typed payloads and exact available source
occurrences. Selected dependency targets cannot be redirected. Protocol-tagged
meaning uses the admitted graph without an invented target binding. A changed
admitted package changes the applicable decision or output, and an original
graph change outside that package does not. Reader refusals and limit stops
reach no CG consumer and emit no artifact. No detached record, copied schema,
second parser or original-graph fallback decides an item.
