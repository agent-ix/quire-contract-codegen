---
id: SR-014
title: "Numeric strategy core closing code review"
type: SpecReview
analysis: code-review
scope: "Task-008 at fa1b682: FR-009, FR-010, FR-012-AC-1 through FR-012-AC-3, and NFR-004-AC-2"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/Task-008
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TM-001
    type: references
---
# SR-014: Numeric strategy core closing code review

## Summary

Reviewed the independently buildable relation, population, shrinking, and boundary-census core at
`fa1b682`, including every public item, generated-Rust path, refusal, extreme-domain branch, traced
test, and repository gate. Three review findings were repaired before this closing record; no
blocking finding remains in Task-008's scope.

## Verdict

**APPROVED** for Task-008's buildable core. This verdict does not approve the Task-009 admission,
runner, consumer bundle, or attestation work blocked on codegen #4.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-1401 | medium | Public `Relation` fields permitted a two-read relation whose primary was the authored right operand, contradicting FR-008's primary-read rule; constructors and private fields now make that state unrepresentable. | `src/bound_strategy/relation.rs`, FR-008 | correct-requirement-no-evidence |
| FND-1402 | low | The exhaustive census test asserted the correct maximum of 20 but its comment still described the superseded 22-case bound. | `tests/bound_census.rs`, NFR-004-AC-2 | wrong-requirement |
| FND-1403 | low | FR-012-AC-1 said every `complicate` path even though proptest 1.5.0 explicitly does not require `complicate` before the first successful `simplify`; the requirement, TC, and generated test support now state the protocol-valid path. | FR-012-AC-1, TC-021 | wrong-requirement |

## Dispositions

| Finding | Disposition | Evidence |
| --- | --- | --- |
| FND-1401 | **FIXED** | `be2df91`; callers use `Relation::with_literal` or `Relation::between_reads`, and all 14 focused tests pass on stable and Rust 1.75. |
| FND-1402 | **FIXED** | `be2df91`; the comment now matches the measured 20-case maximum and the accepted NFR. |
| FND-1403 | **FIXED** | `fa1b682`; FR-012, TC-021, and `walk_every_path` agree with the pinned `ValueTree` contract. |

## Gates at fa1b682

| Gate | Result |
| --- | --- |
| `make ci` | **exit 0** on the committed tree, with `CARGO_TARGET_DIR=/tmp/quire-codegen-target-e-numeric-strategies-20260912` |
| Rust tests | **79 passed, 0 failed, 0 ignored** on stable and again on Rust 1.75.0 |
| Focused tests | TC-018/TC-021 6/6 and TC-019 8/8 on stable and Rust 1.75.0 |
| Format and lint | rustfmt clean; all-target Clippy with denied warnings clean |
| Specification | Quire validation exits 0; duplicate-module and inverse-edge diagnostics are the documented shared-module warnings |
| Supply chain and docs | cargo-deny passes; unsafe audit passes; rustdoc passes with denied warnings |
| Shared assurance | Four pinned components compatible; all scenarios, controls, and probes pass through Quoin |

## Rust review checklist

- The generated code uses checked wide arithmetic and named `i64` extremes; request-controlled
  arithmetic has no panic or truncating-cast path.
- Public types and functions carry documentation, structured error codes, and deterministic
  ordering. The relation constructors encode the only cross-field invariant.
- Generated cases use constructive ranges, `Just`, mapping, and `prop_flat_map`; they contain no
  filtering, assumption, rejection, discard, or cross-side union shrink.
- Tests exercise real generated crates under denied warnings, independent relation evaluation,
  extreme domains, empty sides, deterministic regeneration, exhaustive small-domain sets, and the
  20-case census ceiling. No source/test stub, unsafe block, ignored test, or weakened gate exists in
  the change.
