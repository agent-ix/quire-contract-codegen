---
id: TC-005
title: "Verify Kani proof dependency closure"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-003
    type: verifies
---
# TC-005: Verify Kani proof dependency closure

## Description

Verify generated Kani source, framing, bindings, and dependency graphs preserve assumptions and block
complete-proof claims when required dependencies are not successful.

## Test Procedure

Generate fixtures with complete, missing, failed, assumed, and stubbed dependency edges under the
pinned Kani adapter. Require a bijection between generated assume/stub sites and graph edges, mutate
each edge state, execute bounded proofs, and independently derive graph/evidence classifications.

## Expected Results

Only a successful harness with complete required dependency closure and no assumptions/stubs can
support `complete`; assumed/stubbed graphs are `conditional`, missing/failed graphs are `incomplete`,
and every assumption, version, option, source site, and non-success state remains visible.
