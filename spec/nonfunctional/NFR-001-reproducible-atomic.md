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
| Repeated bundle digest differences | 0 | 0 | golden-approval-testing |
| Partial published bundles after injected failure | 0 | 0 | fault-injection |
| Developer-owned files modified | 0 | 0 | inspection |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-001-AC-1 | Identical pinned inputs and configuration produce byte-identical bundles. | Test (TC-001) |
| NFR-001-AC-2 | An injected generation failure leaves no partially published bundle. | Test (TC-002) |
| NFR-001-AC-3 | Generation does not modify developer-owned regions or files. | Test (TC-002) |

## Verification

TC-001 repeats generation across randomized input order and supported platforms; TC-002 injects write
failures and verifies atomic directory state and developer-tree digests.

## Dependencies

- **Upstream**: [FR-001](../functional/FR-001-deterministic-oracles.md) and [FR-005](../functional/FR-005-cli-conformance.md).
