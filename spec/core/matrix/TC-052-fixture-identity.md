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

Verify that the two checked-package fixture builders use Contract IR for
application identities, use its nominal API for any canonical enum-member
identity, and preserve admitted packages and stale-key controls.

## Test Procedure

1. Build one ungrouped application with each package builder. Compare the
   candidate returned by `derive_application_node_id` with the corresponding
   node ID in the reader-admitted package. Inspect both builders for local
   application preimage construction, version constants, and direct SHA calls
   on application preimages.
2. Build an application in a recursion group with a reference to another
   member. Derive its candidate from a context containing the complete graph
   in graph order; compare it with the reader's admitted identity. Exercise a
   missing context, a context whose target fields differ, and a byte ceiling
   below the canonical preimage length. Assert the corresponding typed
   `ApplicationIdentityError` and absence of a candidate.
3. Derive a canonical enum-member node ID through
   `NominalIdentityPreimage::EnumMember(...).digest`; compare it with the
   reader-admitted member and repeat with a different case. Inspect agreement
   fixtures separately and identify any keys used only for in-process
   uniqueness.
4. Read representative exact-scalar and composite-equality packages. Then
   submit copies with one stale application key and one changed application
   body under its original key. Assert both are refused without fixture rekey
   repair. Inspect remaining local hashes and classify them as fixture-only
   keys, ordinary canonical values, or package identities; confirm the latter
   two use their owning APIs.

## Expected Results

Both builders' ordinary and grouped candidate IDs match the admitted reader
IDs. Context and byte-limit errors retain their typed causes and yield no
candidate. The nominal member identity matches the reader and changes with
the case. The positive packages admit; stale-key and body-mutation controls
refuse. No local application preimage implementation or version tag remains.
