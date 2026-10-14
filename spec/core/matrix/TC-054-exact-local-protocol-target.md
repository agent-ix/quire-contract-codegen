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
in two stages. The FR-322-AC-66 worked carrier can exercise the public strict
reader now; that reader evidence does not establish QSL production or CG
consumption. The production CG controls require QSL-764 to emit the carrier
from source, followed by Contract IR lowering into genuine admitted FR-330
packages. The FR-322 reader owns target admission and located refusals.

## Test Procedure

1. Submit QSpec FR-322-AC-66's worked `protocol`/`protocol_reference` carrier
   and local `protocol` target to the public strict reader. Check that the
   admitted typed graph has one local `body.target`, a matching sole dependency
   and `semantic_type`, and the carrier's one expression occurrence. This is a
   reader control over the worked package, not a QSL-emitted or IR-lowered
   package and not evidence that CG generates from it.
2. After QSL-764 supplies source-to-checked-IR production, admit a source
   protocol reference through QSL, retain its checked package, and lower that
   package through Contract IR. Supply the genuine admitted FR-330 package to
   the production CG entry. Check that CG selects exactly `body.target` in the
   same `semantic_graph.nodes`, retains the requested key
   and admitted source occurrences, and emits an exact artifact when the
   selected backend can represent it, or its typed unsupported refusal with
   no artifact when it cannot. Inspect the production path for one package
   graph and no text-based target lookup.
3. Rebind the source reference to a second eligible local `protocol` node and
   independently produce a second checked and genuinely admitted QSL/IR
   package. Retain its checked package as well. With backend and request
   position fixed, check that selected target key and
   package identity change together. An ordinary local `reference` in another
   node or changed display text cannot redirect either binding.
4. Derive adverse checked-package variants from the FR-322 worked carrier in
   step 1, independently of QSL-764. At the public strict-reader boundary,
   change the carrier's body target and sole dependency together to an absent
   local key and recompute all affected identities. Repeat with an admitted
   selected dependency package containing
   a `protocol` node with that key but no such local node. Then target a local
   non-`protocol` node, and separately a local `protocol` target with unequal
   `semantic_type`, recomputing each carrier and package identity. Submit each
   through the strict reader and verify its typed cause and `/body/target`
   pointer before any CG settlement or generation. An unequal sole dependency
   refuses at `/dependencies` before the target join, per FR-322.

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

## Status

Planned (IR-765). The FR-322 worked carrier supplies a public strict-reader
positive control (step 1) and recomputed adverse reader controls (step 4) now.
QSL-764 is required for genuine QSL/IR source-to-CG positive and changed-binding
controls (steps 2 and 3): no current QSL/IR source producer supplies genuine
`protocol_reference` packages. Reader controls alone do not establish
FR-039-AC-4 implementation coverage.
