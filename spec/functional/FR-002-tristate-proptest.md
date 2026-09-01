---
id: FR-002
title: "Generate tri-state harnesses and shaped strategies"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-002: Generate tri-state harnesses and shaped strategies

## Description

When executable pre/post contracts are lowered, the generator shall emit entry-point harnesses and
proptest strategies that preserve pass, failed postcondition, and rejected precondition outcomes.

## Inputs

- Executable clauses, execution points, state bindings, ranges, memberships, and supported relations.

## Outputs

- Tri-state harness source and broad, boundary, state-pinned, or no-event strategies.

## Behavior

- A harness shall snapshot pre-state before invoking the subject.
- A harness shall check post-state after invoking the subject.
- Ranges, memberships, and supported correlated relations shall shape admissible strategies.
- Residual constraints shall use explicit rejection and retain discard rates.
- Shrinking shall preserve shaped constraints or report residual rejection.
- Generated harnesses, rather than caller glue, shall own the ordering of pre-state snapshot,
  precondition evaluation, subject invocation, and post-state evaluation.
- Unsupported constraint forms shall produce a structured diagnostic.
- Strategy generation shall not silently fall back to filtering.
- Every generated harness campaign shall expose framework-discard recording and a terminal
  conclusion that reads all counters and rejects an all-rejected/all-discarded run.
- Generated expected-domain tags shall be executable checks against runtime tri-state verdicts.
- Harness and strategy generation shall return deterministic source plus a PGM-01 derivation
  manifest, or a structured non-generated terminal state with no partial bundle.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-002-AC-1 | Rejected inputs remain distinct from passing cases in generated reports. | Test (TC-004) |
| FR-002-AC-2 | Boundary strategies exercise immediately inside and outside supported constraints. | Test (TC-004) |
| FR-002-AC-3 | Supported correlated constraints do not rely on improbable filtering. | Analysis |
| FR-002-AC-4 | Shrinking preserves generated constraints or records a residual rejection. | Test (TC-004) |

## Dependencies

- **Upstream**: [FR-001](./FR-001-deterministic-oracles.md).
