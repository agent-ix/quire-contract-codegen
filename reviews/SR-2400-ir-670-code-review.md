---
id: "SR-2400"
title: "code-review of IR-670 FR-034 native ABI admission spec diff"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@24b4135daad56b1673e2107e953cb823a9cac5ad; diff e5b303f5e5010906567a81bbcdbd6d42c58b5407...24b4135daad56b1673e2107e953cb823a9cac5ad; spec/kani/functional/FR-034-caller-death-ownership.md; spec/kani/matrix/TC-049-caller-death-ownership.md; src/kani/classify.rs; src/kani/run/namespace.rs; src/kani/run/tool.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-2400: code-review of IR-670 FR-034 native ABI admission spec diff

## Summary

Ticket: IR-670. Code-review of the spec-only diff e5b303f..24b4135 (FR-034 replaced sentence, new section and AC-39; TC-049 row and step 26), checking each claim about CG behaviour against the source at the reviewed sha. The diff removes exactly one line (FR-034:618) and adds only lines otherwise, so reversing it reproduces both baseline files and the 38 prior AC rows are byte-identical. The NoVerdict, Missing, launcher-precheck and pathname-exec claims match the source; one wording claim does not.

Reviewer: session 931a2951-8e8f-4dc1-b745-5c804f4eb6a6, model claude-opus-5-5, run 0fde4e67-c6f4-484d-8e20-14fb1de71f48.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new admission paragraph, AC-39 and TC-049 step 26 call the CapabilityUnavailable { BackendIpcExclusion } admission context 'existing', but at 24b4135 src/ contains no KaniStartupAdmissionCause, CapabilityUnavailable or BackendIpcExclusion, and FR-034:772 states that context is a planned API amendment, not current code. | spec/kani/functional/FR-034-caller-death-ownership.md:636; spec/kani/functional/FR-034-caller-death-ownership.md:1073; spec/kani/matrix/TC-049-caller-death-ownership.md:608; spec/kani/functional/FR-034-caller-death-ownership.md:772 |

## Finding detail

### FND-001

Confidence: high. Check: soundness. Unit: FR-034 (spec/kani/functional/FR-034-caller-death-ownership.md:635-638).

Failure scenario: A coder reads 'through the existing CapabilityUnavailable context naming BackendIpcExclusion', searches src/ for it, finds nothing, and either assumes it is already wired and skips adding it, or treats the requirement as inconsistent with the code.

## Scope

- `FR-034 L616-620` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L635-639` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L641-645` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L645-648` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L650-654` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L654-658` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L660-664` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L664-667` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034-AC-39` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034-AC-39 (middle)` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `TC-049 L105` (spec/kani/matrix/TC-049-caller-death-ownership.md) — examined
- `TC-049 step 26 L604-609` (spec/kani/matrix/TC-049-caller-death-ownership.md) — examined
- `TC-049 step 26 L623-627` (spec/kani/matrix/TC-049-caller-death-ownership.md) — examined
- `TC-049 step 26 L628-632` (spec/kani/matrix/TC-049-caller-death-ownership.md) — examined
- `src/kani/classify.rs L128-137` (src/kani/classify.rs) — context_only
- `src/kani/run/tool.rs L63-65` (src/kani/run/tool.rs) — context_only
- `src/kani/run/namespace.rs L146-148` (src/kani/run/namespace.rs) — context_only
- `FR-034 L766-768` (spec/kani/functional/FR-034-caller-death-ownership.md) — context_only
- `FR-034 L770-772` (spec/kani/functional/FR-034-caller-death-ownership.md) — context_only
- `FR-034 L592` (spec/kani/functional/FR-034-caller-death-ownership.md) — context_only

## Verdict

The behavioural claims are source-grounded: src/kani/classify.rs:128-137 maps unsuccessful exit with no report to Inconclusive NoVerdict and successful exit with no report to KaniReportRefusal::Missing; src/kani/run/tool.rs:63-65 disclaims file pinning and replacement races; src/kani/run/namespace.rs:146-148 hands bwrap the original program pathname, which bwrap resolves at exec, matching the stated PATH/execvp residual. The narrowing is explicit and adds no hash, pin, unsafe, fallback, compatibility layer, catalog, budget or new public cause. One wording defect: the admission context is called 'existing' though it is not in the source. rust-review was not run as a method (no .rs or Cargo file in the diff); its checklist was applied only while reading source for context. No build, test, Kani or gate was run, by assignment.
