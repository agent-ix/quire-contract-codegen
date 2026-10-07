---
id: "SR-2406"
title: "spec-dependency-analysis of IR-670 FR-034 AC-39 enablement"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-contract-codegen@24b4135daad56b1673e2107e953cb823a9cac5ad; diff e5b303f5e5010906567a81bbcdbd6d42c58b5407...24b4135daad56b1673e2107e953cb823a9cac5ad; spec/kani/functional/FR-034-caller-death-ownership.md; spec/kani/matrix/TC-049-caller-death-ownership.md; src/lib.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-2406: spec-dependency-analysis of IR-670 FR-034 AC-39 enablement

## Summary

Ticket: IR-670. Dependencies and enablement separated from feature work for AC-39 / TC-049 step 26. The diff adds no relationships edge and no Dependencies entry; FR-017 (reports/NoVerdict) and the existing FR-034 capability allocation are reused, not changed. One enablement dependency of the new test is unallocated.

Reviewer: session 931a2951-8e8f-4dc1-b745-5c804f4eb6a6, model claude-opus-5-5, run 0fde4e67-c6f4-484d-8e20-14fb1de71f48.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Step 26 needs 'a repository-owned real compat/x32 syscall workload built by ordinary test tooling', including native entry followed by a compat syscall, but neither TC-049 nor FR-034 Dependencies says how it is built. From Rust it needs inline assembly or a raw syscall call, both unsafe (the crate is forbid(unsafe_code) and FR-034:580 excludes unsafe at the boundary); otherwise it needs a non-Rust compiler with -m32/-mx32 multilib or extra rustup targets. | spec/kani/matrix/TC-049-caller-death-ownership.md:611; spec/kani/functional/FR-034-caller-death-ownership.md:580; src/lib.rs:3 |

## Finding detail

### FND-001

Confidence: medium. Check: other. Unit: TC-049 (spec/kani/matrix/TC-049-caller-death-ownership.md:611-612).

Failure scenario: The coder reaches step 26, finds that no safe Rust API issues an int 0x80 or x32-bit syscall, and must either add unsafe test code with an audit-baseline entry or add a C/multilib toolchain to CI; neither is allocated, so the step stalls or the workload is recorded UNRUN on every host.

## Scope

- `TC-049 step 26 L611-615` (spec/kani/matrix/TC-049-caller-death-ownership.md) — examined
- `TC-049 step 26 L616-622` (spec/kani/matrix/TC-049-caller-death-ownership.md) — examined
- `FR-034-AC-39` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034-AC-39 (middle)` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L660-664` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L664-667` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L1075-1078` (spec/kani/functional/FR-034-caller-death-ownership.md) — context_only
- `FR-034 L578-581` (spec/kani/functional/FR-034-caller-death-ownership.md) — context_only
- `src/lib.rs L3` (src/lib.rs) — context_only

## Verdict

No new spec-artifact dependency or cycle; reuse of FR-017 classification and the planned KaniStartupAdmissionCause allocation is consistent with FR-034's existing depends_on FR-017 edge. The compat/x32 workload's build dependency is the gap.
