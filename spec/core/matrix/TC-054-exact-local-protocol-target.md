---
id: TC-054
title: "Verify exact local protocol-target consumption"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-039
    type: verifies
---
# TC-054: Verify exact local protocol-target consumption

## Description

Verify [FR-039-AC-4](../functional/FR-039-contract-package-consumption.md)
through the production CG entry using genuine QSL-admitted, IR-lowered FR-330
packages. The QSpec FR-322-AC-66 carrier and FR-330-AC-4 single graph determine
the target; the FR-322 reader owns target admission and located refusals.

## Test Procedure

1. Admit the FR-322 worked `protocol`/`protocol_reference` carrier and its local
   `protocol` target through QSL and Contract IR. Supply the resulting admitted
   FR-330 package to CG. Check that CG selects exactly `body.target` in the same
   `semantic_graph.nodes`, retains the carrier's requested key and admitted
   source occurrences, and emits an exact artifact when the selected backend
   can represent it, or its typed unsupported refusal with no artifact when it
   cannot. Inspect the production path for one package graph and no text-based
   target lookup.
2. Bind the carrier to a second eligible local `protocol` node through a
   genuinely admitted QSL/IR package. Compare the selected target key and
   admitted package identity with the first package while keeping the backend
   and request position fixed. Check that the target and package identity
   change together, and that an ordinary local `reference` in another node or
   changed display text cannot redirect either binding.
3. At the FR-322 reader boundary, set the carrier body target and sole
   dependency to an absent local key, with all affected identities recomputed.
   Repeat with an admitted selected dependency package containing a
   `protocol` node with that key, but no such local node. Then use a local
   non-`protocol` node as target, and separately a local `protocol` target
   whose `semantic_type` differs from the carrier's. Submit each through the
   strict reader and verify its typed cause and `/body/target` pointer before
   any CG settlement or generation. Also check that an unequal sole dependency
   is refused at `/dependencies` before the target join, per FR-322.

## Expected Results

Only the exact local typed `body.target` of an admitted
`protocol`/`protocol_reference` carrier binds the CG target. A different
admitted local binding changes the selected target and package identity.
An absent local target, including one present in a selected dependency,
refuses `missing_declaration`/`missing-name` at `/body/target`; a present
wrong-tag or unequal-semantic-type target refuses
`ill_typed`/`operator-ineligible` there. A dependency mismatch refuses
`invalid_semantic_graph` at `/dependencies` before the target join. None of
these reader refusals reaches CG or emits an artifact. An admitted protocol
form beyond backend support keeps its requested identity and source
occurrences in a typed CG refusal with no artifact.
