---
id: "SR-2405"
title: "spec-scope-boundary-analysis of IR-670 FR-034 native ABI admission narrowing"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen@24b4135daad56b1673e2107e953cb823a9cac5ad; diff e5b303f5e5010906567a81bbcdbd6d42c58b5407...24b4135daad56b1673e2107e953cb823a9cac5ad; spec/kani/functional/FR-034-caller-death-ownership.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-2405: spec-scope-boundary-analysis of IR-670 FR-034 native ABI admission narrowing

## Summary

Ticket: IR-670. Boundary of the narrowed obligation: what pre-Dispatch admission now covers (trusted installer native ABI policy support and actual installation), what moved to syscall-boundary enforcement after Dispatch, and what is explicitly out of scope (backend image identity, exec-time rejection of incompatible binaries, executable stability). The narrowing is explicit and bounded; one boundary term is undefined.

Reviewer: session 931a2951-8e8f-4dc1-b745-5c804f4eb6a6, model claude-opus-5-5, run 0fde4e67-c6f4-484d-8e20-14fb1de71f48.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | 'where applicable, the x32 syscall-number/alias exclusion' does not say what decides applicability: the policy's target architecture (every x86_64 build), or the running kernel's x32 support. Under the second reading the x32 rule may be left out on hosts whose kernel lacks x32, yet x32 syscalls carry the native x86_64 audit architecture, so on an x32-enabled kernel they pass the architecture rule and need the x32 rule to be refused. | spec/kani/functional/FR-034-caller-death-ownership.md:642; spec/kani/matrix/TC-049-caller-death-ownership.md:611 |

## Finding detail

### FND-001

Confidence: medium. Check: ambiguous. Unit: FR-034 (spec/kani/functional/FR-034-caller-death-ownership.md:641-642).

Failure scenario: A coder reads 'where applicable' as 'when the build host's kernel supports x32', ships a policy without the x32 rule because CI kernels lack x32, and step 26 records the x32 workload as unavailable/UNRUN; on a deployment kernel with CONFIG_X86_X32_ABI, socket(AF_UNIX) issued as the x32 number is not matched by the native-number checks.

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
- `FR-034-AC-39 (tail)` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined

## Verdict

The narrowing replaces 'incompatible or unsupported execution ABIs shall refuse admission' with a truthful split and explicitly keeps IPC, privilege, descriptor, ownership, capture, deadline, resource and settlement obligations; it introduces no stability precondition, recipe rewrite, inherited execution descriptor, fallback or new cause. The undefined 'where applicable' for the x32 rule is the one boundary defect.
