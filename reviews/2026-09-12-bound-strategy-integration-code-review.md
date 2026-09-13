---
id: SR-016
title: "Bound strategy integration closing code review"
type: SpecReview
analysis: code-review
scope: "Task-009 at 4228611: FR-008 through FR-013 and NFR-004"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/Task-009
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TM-001
    type: references
---
# SR-016: Bound strategy integration closing code review

## Summary

Reviewed Task-009's admission, generated campaigns, consumer surface, and attestation integration at
`4228611`, including every authored refusal class, clause-kind path, population, campaign size,
counter edge, shrinking replay, identity input, and generated-consumer boundary. Five findings were
repaired before this closing record; no blocking finding remains in the numeric-strategy slice.

## Verdict

**APPROVED** for Task-009 and the complete FR-008 through FR-013 / NFR-004 slice.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-1601 | high | TC-020 covered only part of the required clause-kind, population, campaign-size, rate, census, and negated-oracle matrix. It now runs all three clause kinds across all populations at 256 and 10,000 cases, verifies exact counters and zero rejects, instruments the census, checks zero/saturated rates, and exercises shrinking mismatches. | TC-020, FR-011, NFR-004-AC-1 | correct-requirement-no-evidence |
| FND-1602 | medium | TC-022 did not mutate the package digest and clause identity independently. It now proves each input changes the artifact, header, attestation, and argv identity. | TC-022, FR-013-AC-4 | correct-requirement-no-evidence |
| FND-1603 | medium | TC-017 omitted authored refusal and diagnostic-order fixtures for numeric obligations, comparison shapes, same-declaration reads, Assertion/Case kinds, and combined invalid input. | TC-017, FR-008 | correct-requirement-no-evidence |
| FND-1604 | low | Generated admission/runner construction used internal panic assertions for states that can instead retain a structured diagnostic. Those paths now return the existing refusal envelope. | `src/bound_strategy/generation.rs`, FR-008-CON-1 | correct-requirement-no-evidence |
| FND-1605 | low | TC-022 required a concrete generated-name collision although names contain a full SHA-256 identity; constructing that fixture would require a hash collision. The test procedure now records the deterministic source inspection that verifies the collision-resistant construction. | TC-022, FR-013 | wrong-requirement |

## Dispositions

| Finding | Disposition | Evidence |
| --- | --- | --- |
| FND-1601 | **FIXED** | `4228611`; the generated consumer executes every required 256/10,000-case matrix cell with `max_global_rejects: 0`, exact rate/counter assertions, ordered census instrumentation, and structured mismatch shrinking. |
| FND-1602 | **FIXED** | `4228611`; independent digest and ClauseRef mutations produce distinct source and attestation identities. |
| FND-1603 | **FIXED** | `4228611`; TC-017 now covers all authored syntax, obligation, relation, kind, span, and diagnostic-order refusals. |
| FND-1604 | **FIXED** | `4228611`; the generated path contains no `expect` or `unreachable` invariant escape and returns `UnsupportedClauseKind` if its admitted kind cannot be rendered. |
| FND-1605 | **FIXED** | `4228611`; TC-022 and FR-013 now agree on inspection of the full-digest naming rule. |

## Review evidence

- Focused TC-017 through TC-022 integration tests pass 6/6 on stable and Rust 1.75.0.
- All-target Clippy with denied warnings and rustfmt pass after the finding repairs.
- Generated fixtures compile with denied warnings and depend only on the generated artifact,
  `proptest`, and `quire-contract-runtime`.
- Campaigns use constructive strategies with zero proptest global rejects; no filter, assumption,
  silent discard, panic assertion, unsafe block, ignored test, source stub, or test stub remains in
  the reviewed change.
- Quire reports all 34 selected criteria and constraints backed, with no selected status
  contradiction. The two matrix rows whose declared method is `Inspection` intentionally have no
  standalone executable symbol; their underlying IDs are also traced by the exercising tests.
