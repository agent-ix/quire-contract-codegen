---
id: FR-015
title: "Generate separate bounded Kani obligations for complete-V1 oracles"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-196
    type: references
  - target: ix://agent-ix/quire-specification/TC-218
    type: references
---
# FR-015: Generate separate bounded Kani obligations for complete-V1 oracles

## Description

When a caller selects Kani generation for complete-V1 contracts whose scalar
oracles FR-014 generated, the code generator shall emit one bounded Kani
obligation per precondition, postcondition, invariant and frame condition,
each pinned to its backend identity and model-domain bounds. This is issue
#49.

## Inputs

- The FR-014 oracle crate and claim map for the contract's expressions.
- Contract IR's lowered claims and bounds for each obligation. Bounds come
  only from the lowered IR `bounded_domain` nodes, never from a caller
  descriptor.
- The pins: the Kani launcher and driver executable digests, the Kani version,
  the CBMC version, solver and options, the unwind bound, the adapter profile,
  the FR-014 oracle crate digest, and the Contract Runtime revision.

## Outputs

- One Kani harness per obligation kind and claim, with its bounds and every
  pin recorded in its identity.
- A typed refusal for each obligation that has no finite encoding.

## Behavior

- When generating proofs, the generator shall emit precondition,
  postcondition, invariant and frame obligations as separate harnesses.
- When an obligation has a finite model domain, the generator shall bound
  every symbolic input by that domain without assuming away refused,
  undefined or incomplete outcomes.
- If an obligation's domain is unbounded or its family lacks a finite
  encoding, then the generator shall refuse it with a typed reason and emit no
  harness.
- If an obligation's bounds are unsatisfiable, so that the harness would hold
  vacuously, then the generator shall refuse it with a typed reason and emit no
  harness.
- If an obligation depends on an FR-014 oracle whose operation identity is
  `caller_declared`, then the generator shall refuse it with a typed reason
  naming that blocked item and emit no harness.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-015-AC-1 | Pre, post, invariant and frame obligations of one contract produce separate harnesses with distinct identities. | Test (TC-025) |
| FR-015-AC-2 | Every harness identity records its model-domain bounds, taken only from IR `bounded_domain` nodes, and the Kani launcher and driver executable digests, Kani version, CBMC version, solver and options, unwind bound, adapter profile, oracle crate digest and runtime revision. | Test (TC-025) |
| FR-015-AC-3 | An unbounded or non-finite obligation is refused with a typed reason and no harness. | Test (TC-025) |
| FR-015-AC-4 | No harness assumption excludes an undefined, refused or incomplete runtime outcome. | Test (TC-025) |
| FR-015-AC-5 | An obligation whose bounds are unsatisfiable is refused with a typed reason and no harness. | Test (TC-025) |
| FR-015-AC-6 | An obligation over an oracle whose operation identity is `caller_declared` is refused with a typed reason and no harness. | Test (TC-025) |

## Dependencies

- **Upstream**: [FR-014](./FR-014-exact-scalar-oracles.md), [FR-003](../FR-003-kani-lowering.md).
- **Downstream**: [TC-025](../../test/complete-v1/TC-025-bounded-kani-obligations.md),
  [FR-016](./FR-016-witness-native-replay.md).
