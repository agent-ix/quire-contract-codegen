---
id: TC-052
title: "Verify checked-package fixture identity ownership"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-037
    type: verifies
---
# TC-052: Verify checked-package fixture identity ownership

## Description

Verify that both checked-package fixture builders preserve producer-assigned
structural application and enum-member identities, keep grouped references
consistent, and retain reader admission and typed negative controls.

## Test Procedure

1. Assemble one ungrouped application with each builder. Compare its
   producer-assigned `CheckedNodeId`, every dependent reference and the
   reader-admitted node ID (FR-037-AC-6). Inspect the builders for removed
   candidate derivation, local application preimages and identity digests.
2. Assemble a complete ordered recursion group with a member-to-member
   reference. Compare each producer-assigned member ID, ordinal and reference
   before and after reader admission. Submit missing and mismatched structural
   context and assert the owner's typed refusal. Set a reader byte ceiling
   below the complete package length and assert its typed limit result, with
   no admitted fixture (FR-037-AC-7).
3. Assemble a canonical enum member from the producer's structural ID,
   declaration and case. Compare it with the reader-admitted member and repeat
   with a changed case and its corresponding producer ID. Inspect the separate
   agreement fixtures for keys used only for in-process uniqueness
   (FR-037-AC-8).
4. Admit representative exact-scalar and composite-equality packages. Submit
   a stale structural key without repair and assert reader refusal. Change a
   body while retaining its structural key and compare it with the expected
   admitted source body; assert the mismatch is detected without assuming a
   new key (FR-037-AC-9). Inspect local keys and canonical-value operations to
   confirm fixture-only keys remain separate and no package or application
   digest derivation remains (FR-037-AC-10).

## Expected Results

Structural IDs and grouped references match the producer and reader-admitted
graph. Invalid context and the byte ceiling retain their typed causes without
an admitted fixture. Canonical enum-member identity follows the producer's
case. Positive packages admit; stale-key and changed-body controls fail at
their stated authority. No local application or package preimage rule remains.
