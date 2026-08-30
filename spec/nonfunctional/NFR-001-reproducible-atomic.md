---
id: NFR-001
title: "Reproducible and atomic generation"
type: NFR
quality_attribute: reliability
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: constrains
  - target: ix://agent-ix/quire-contract-codegen/FR-005
    type: constrains
---
# NFR-001: Reproducible and atomic generation

## Statement

For a pinned input package, configuration, and supported platform profile, the generator shall emit
byte-identical bundles and shall publish them atomically without modifying developer-owned regions.

## Scope

Library and CLI generation across every supported backend and declared platform profile.

## Rationale

Reproducibility makes derivation auditable; atomic publication prevents partial or stale generated
artifacts from being mistaken for a complete verification bundle.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Repeated bundle digest differences | 0 | 0 | repeated golden generation |
| Partial published bundles after injected failure | 0 | 0 | fault-injection integration test |
| Developer-owned files modified | 0 | 0 | before/after tree digest |

## Verification

TC-001 repeats generation across randomized input order and supported platforms; TC-002 injects write
failures and verifies atomic directory state and developer-tree digests.

## Dependencies

- **Upstream**: [FR-001](../functional/FR-001-deterministic-oracles.md) and [FR-005](../functional/FR-005-cli-conformance.md).
