---
id: SR-018
title: "PR 30 numeric strategy Rust review"
type: SpecReview
analysis: code-review
scope: "PR #30 at 1f49184: Task-008 and Task-009 numeric strategy slice"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/Task-008
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/Task-009
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TM-001
    type: references
---
# SR-018: PR 30 numeric strategy Rust review

## Summary

An independent Rust and semantic review rechecked admission, population/census construction,
generated runners, downstream consumption, failure precedence, resource bounds, public invariant
surfaces, and requirement-to-test meaning across FR-008 through FR-013 and NFR-004. The review
found two high-severity correctness defects, seven medium evidence or API defects, and four low
hardening/documentation defects. Commit `1f49184` repairs every finding.

## Verdict

**APPROVED** for the numeric-strategy slice after the repairs in `1f49184`.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-1801 | high | Generated census metadata dropped each read's required `Input`/`State` declaration kind. | FR-008-AC-1, TC-017, `generation.rs` | implementation-bug-despite-evidence |
| FND-1802 | high | A pre-existing over-ceiling discard count masked `IdentityMismatch`, although only `ConformanceMismatch` has discard-ceiling precedence. | FR-011, TC-020, `generation.rs` | implementation-bug-despite-evidence |
| FND-1803 | medium | TC-017 did not compare oracle refusal code, terminal state, and exact span against `generate_bound_oracles`, and did not cover the complete refusal order. | FR-008-AC-3, FR-008-AC-5, TC-017 | correct-requirement-no-evidence |
| FND-1804 | medium | Admission wording simultaneously required oracle-first admission and no source rendering, and did not state the Boolean/Text comparison exception to oracle-refusal mapping. | FR-008 behavior, FR-008-CON-2 | wrong-requirement |
| FND-1805 | medium | TC-020 omitted one fresh zero-ceiling cell and derived seeded-rate expectations from the result under test. | FR-011-AC-4, FR-011-AC-5, NFR-004-AC-1, TC-020 | correct-requirement-no-evidence |
| FND-1806 | medium | Shrink replay accounting proved only `attempted > 1`, not one report count per oracle evaluation. | FR-012-AC-4, TC-021 | correct-requirement-no-evidence |
| FND-1807 | medium | Clause identity was changed only by changing the package, so the package digest confounded the identity assertion. | FR-013-AC-4, TC-022 | correct-requirement-no-evidence |
| FND-1808 | medium | Public `ValueSet::DomainExceptPoint` and `SideValues::Correlated` variants allowed downstream construction that violated method invariants. | FR-009, `population.rs` | implementation-bug-despite-evidence |
| FND-1809 | medium | Public census rendering accepted unbounded names and could return source above the repository's generated-source budget. | FR-010, NFR-004, `census.rs` | implementation-bug-despite-evidence |
| FND-1810 | medium | Empty-side tests stopped at the internal population renderer and did not prove public generation returns no bundle. | FR-009-AC-4, TC-018 | correct-requirement-no-evidence |
| FND-1811 | low | Boundary admission retained an internal `expect_err`, and the admitted primary-read invariant was represented by vector indexing. | FR-008, `generation.rs` | correct-requirement-no-evidence |
| FND-1812 | low | Three Clippy exception sites lacked rationale. | `generation.rs` | correct-requirement-no-evidence |
| FND-1813 | low | The census sweep's direct bound asserted 22 although NFR-004 requires 20, and arithmetic wording overstated the already bounds-proven implementation. | FR-009, FR-010, NFR-004-AC-2, TC-019 | wrong-requirement |

## Dispositions

All FND-1801 through FND-1813 are **FIXED** in `1f49184`. The repairs retain declaration kinds,
preserve identity failures, compare exact diagnostics, complete the refusal order and campaign
matrix, independently count oracle calls, isolate ClauseRef identity, seal public invariants, bound
census rendering, exercise public empty-side refusal, remove panic-style invariant handling,
document lint exceptions, and align the arithmetic and census-bound requirements with the checked
implementation.

## Review evidence

- Focused admission, population, census, runner, shrink, and consumer tests pass on the stable
  toolchain after all repairs.
- Generated downstream crates compile with denied warnings and exercise exact metadata, census
  fields, counters, rates, identity failures, and attestations.
- `cargo fmt --all -- --check`, all-target/all-feature Clippy with denied warnings, and
  `git diff --check` pass.
- No unsafe block, ignored test, source/test stub, direct strategy filter, global reject, silent
  discard, valid-input overflow, or generated dependency escape remains in the reviewed slice.
