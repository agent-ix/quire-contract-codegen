---
id: "SR-2404"
title: "spec-failure-domain-analysis of IR-670 FR-034 native ABI admission"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen@24b4135daad56b1673e2107e953cb823a9cac5ad; diff e5b303f5e5010906567a81bbcdbd6d42c58b5407...24b4135daad56b1673e2107e953cb823a9cac5ad; spec/kani/functional/FR-034-caller-death-ownership.md; spec/kani/matrix/TC-049-caller-death-ownership.md; src/kani/classify.rs; src/kani/run/namespace.rs; src/kani/run/tool.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-2404: spec-failure-domain-analysis of IR-670 FR-034 native ABI admission

## Summary

Ticket: IR-670. Unstated failure modes and identity confusion in the narrowed ABI admission: the unsupported-native-support refusal versus installation I/O failure, the post-Dispatch kill/exec-failure path, and the check-to-exec residual inventory. Read with the merged classify.rs/tool.rs/namespace.rs and, read-only, the unmerged WIP backend_policy.rs at 6f5ad71 as a measured feasibility reference only (not reviewed or compiled code).

Reviewer: session 931a2951-8e8f-4dc1-b745-5c804f4eb6a6, model claude-opus-5-5, run 0fde4e67-c6f4-484d-8e20-14fb1de71f48.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Unsupported native policy support and installation failure share one typed context (CapabilityUnavailable { BackendIpcExclusion }); only installation failure is told to 'retain its actual cause', while the refusal carries a mandatory original cause: std::io::Error. The spec does not say what cause the unsupported-support refusal carries (it has no OS error), yet TC-049 step 26 requires both to be exercised 'separately' with 'the retained actual cause', and FR-034 forbids message parsing as a discriminator. | spec/kani/functional/FR-034-caller-death-ownership.md:635; spec/kani/functional/FR-034-caller-death-ownership.md:637; spec/kani/functional/FR-034-caller-death-ownership.md:761; spec/kani/matrix/TC-049-caller-death-ownership.md:606 |
| FND-002 | low | The check-to-exec residual inventory names path replacement, content mutation, PATH/execvp, script interpreter, ELF PT_INTERP and loader selection, but not kernel binfmt_misc handler resolution, under which a foreign-architecture image is run by a host-registered native emulator (possibly opened from the host root at registration) whose translated syscalls arrive as native syscalls. | spec/kani/functional/FR-034-caller-death-ownership.md:653; spec/kani/matrix/TC-049-caller-death-ownership.md:622 |

## Finding detail

### FND-001

Confidence: medium. Check: ambiguous. Unit: FR-034 (spec/kani/functional/FR-034-caller-death-ownership.md:635-638).

Failure scenario: One implementation reports unsupported support as io::ErrorKind::Unsupported, another as Other with a message; a test of step 26 cannot tell an unsupported-ABI refusal from an installation failure whose OS error also maps to ErrorKind::Unsupported (for example EOPNOTSUPP) without reading the message.

### FND-002

Confidence: medium. Check: other. Unit: FR-034 (spec/kani/functional/FR-034-caller-death-ownership.md:652-654).

Failure scenario: On a host with a registered qemu-user binfmt handler, a foreign-architecture backend image runs; its guest syscalls reach the filter as native syscalls and are enforced natively, but a tester reading step 26 may count this as an 'incompatible image' that the kernel rejected or as compat-syscall evidence, and the residual list gives no warning.

## Scope

- `FR-034 L635-639` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L641-645` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L645-648` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L650-654` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L654-658` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L660-664` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `FR-034 L664-667` (spec/kani/functional/FR-034-caller-death-ownership.md) — examined
- `TC-049 step 26 L604-609` (spec/kani/matrix/TC-049-caller-death-ownership.md) — examined
- `TC-049 step 26 L611-615` (spec/kani/matrix/TC-049-caller-death-ownership.md) — examined
- `TC-049 step 26 L616-622` (spec/kani/matrix/TC-049-caller-death-ownership.md) — examined
- `TC-049 step 26 L623-627` (spec/kani/matrix/TC-049-caller-death-ownership.md) — examined
- `FR-034 L766-768` (spec/kani/functional/FR-034-caller-death-ownership.md) — context_only
- `FR-034 L770-772` (spec/kani/functional/FR-034-caller-death-ownership.md) — context_only
- `src/kani/classify.rs L128-137` (src/kani/classify.rs) — context_only
- `src/kani/run/tool.rs L63-65` (src/kani/run/tool.rs) — context_only
- `src/kani/run/namespace.rs L146-148` (src/kani/run/namespace.rs) — context_only

## Verdict

Post-Dispatch handling is sound: exec failure and ABI kill go to the existing bounded result path, never retroactive admission, and NoVerdict is limited to an actual unsuccessful exit with no report after settlement, matching classify.rs:128-137. Kernel rejection before the syscall is correctly excluded as filter evidence. Two gaps: the unsupported-support refusal has no defined cause or discriminator, and the residual inventory omits one exec-time resolution path.
