---
id: "SR-2401"
title: "spec-review (base checklist) of IR-670 FR-034 native ABI admission"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@24b4135daad56b1673e2107e953cb823a9cac5ad; diff e5b303f5e5010906567a81bbcdbd6d42c58b5407...24b4135daad56b1673e2107e953cb823a9cac5ad; spec/kani/functional/FR-034-caller-death-ownership.md; spec/kani/matrix/TC-049-caller-death-ownership.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-2401: spec-review (base checklist) of IR-670 FR-034 native ABI admission

## Summary

Ticket: IR-670. Base checklist (ID format, criterion verifiability, coverage rules, hash/pin antipattern) over the FR-034 and TC-049 changes in e5b303f..24b4135. IDs are well formed and sequential (FR-034-AC-39 follows AC-38; step 26 follows step 25), quire validate exits 0 on both files, quire matrix computes FR-034-AC-39 as untagged as a PLANNED criterion should, and no hash, digest, pin or tracking record is introduced. One evidence-path gap makes part of AC-39 unverifiable as written.

Reviewer: session 931a2951-8e8f-4dc1-b745-5c804f4eb6a6, model claude-opus-5-5, run 0fde4e67-c6f4-484d-8e20-14fb1de71f48.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Step 26 requires a Test that exercises 'unavailable native policy support' through to a pre-Dispatch BackendIpcExclusion refusal, but on every host where the matching policy builds (the audited native ABIs) that branch cannot be reached at runtime, the step forbids an invented admission seam, and, unlike the compat/x32 workload, it gives this branch no unavailable/UNRUN rule or Analysis-only evidence route. | spec/kani/matrix/TC-049-caller-death-ownership.md:606; spec/kani/matrix/TC-049-caller-death-ownership.md:612; spec/kani/functional/FR-034-caller-death-ownership.md:1073 |

## Finding detail

### FND-001

Confidence: medium. Check: untestable-ac. Unit: TC-049 (spec/kani/matrix/TC-049-caller-death-ownership.md:604-609).

Failure scenario: On an x86_64 CI host the unsupported-ABI refusal is compiled out, so no real Test can drive it; AC-39 then either stays incomplete forever or a coder adds a test-only switch to force the branch, which the step's no-seam rule forbids.

## Scope

- `FR-034-AC-39` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034-AC-39 (middle)` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034-AC-39 (tail)` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `TC-049 L105` (spec/kani/matrix/TC-049-caller-death-ownership.md) — examined
- `TC-049 step 26 L604-609` (spec/kani/matrix/TC-049-caller-death-ownership.md) — examined
- `TC-049 step 26 L611-615` (spec/kani/matrix/TC-049-caller-death-ownership.md) — examined
- `TC-049 step 26 L616-622` (spec/kani/matrix/TC-049-caller-death-ownership.md) — examined
- `TC-049 step 26 L623-627` (spec/kani/matrix/TC-049-caller-death-ownership.md) — examined
- `TC-049 step 26 L628-632` (spec/kani/matrix/TC-049-caller-death-ownership.md) — examined
- `FR-034 L616-620` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L635-639` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L766-768` (spec/kani/functional/FR-034-caller-death-ownership.md) — context_only
- `FR-034 L770-772` (spec/kani/functional/FR-034-caller-death-ownership.md) — context_only

## Verdict

Structurally valid and free of the hash/pin antipattern; the narrowing is stated rather than hidden. The defect is that one AC-39 branch has no feasible evidence path. Review set: subset (base plus ears-conformance, integrity, failure-domain, scope-boundary, dependency). Criterion-strength was not run because no Jev client is installed; no calibrated strength judgement is claimed.

## Dispositions

Round 1, reviewed at agent-ix/quire-contract-codegen@aef3d5715543db202d37d28c5fa4f422b99dc369 (fix diff 24b4135..aef3d57; 79a1a9a only adds the seven original SR files under reviews/, byte-identical to this file's original prefix). Reviewer session 931a2951-8e8f-4dc1-b745-5c804f4eb6a6, model claude-opus-5-5, run fea14e45-7035-49c6-9f42-ef4ddc5a1514. Every outcome was re-measured against the fix head; the author's fix map was read as data only. No build, test, Kani or full gate was run.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | aef3d57: TC-049 step 26 (L599-605) now routes the unreachable unsupported-native branch to source/cfg Analysis labelled not an executed Test, UNRUN until source exists, and makes absent installation-failure controls unavailable/UNRUN with no Test credit; AC-39 (FR-034:1106), the slice row (TC-049:105) and the Expected Results row (TC-049:749) state the same. |
