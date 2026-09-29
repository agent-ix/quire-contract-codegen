---
id: FR-006
title: "Run the bounded generation conformance corpus"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-006: Run the bounded generation conformance corpus

## Description

When `make conformance` runs, the generation-conformance program shall run the bounded generation
corpus through the public generation API, print one row per corpus case, and exit with a status
that says whether any case failed or was vacuous.

## Inputs

- The bounded generation corpus and this crate's public generation API.

## Outputs

- One JSON row per corpus case, carrying the outcome, the Interface-001 terminal state reached, the
  diagnostic code produced, and the number of declared checks that held against a floor.
- A process exit status.

## Behavior

- The program shall derive its exit status from the rows it printed.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-006-AC-1 | The exit status is 0 when every printed row's outcome is `pass`, 1 when any row's outcome is `fail`, and 2 when none is `fail` and any is `vacuous`; a run with both a failing and a vacuous row exits 1, and a run that printed no rows exits 0. | Test (TC-032) |
| FR-006-AC-2 | A printed line that does not carry a string `outcome` aborts the run with a message naming that field, rather than classifying as 0. | Test (TC-032) |
| FR-006-AC-3 | The compiled conformance binary exits 0 when run against the real bounded corpus. | Test (TC-032) |

## Dependencies

- **Upstream**: [FR-001](./FR-001-deterministic-oracles.md).
