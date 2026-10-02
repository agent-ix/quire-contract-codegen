---
id: "SR-820"
title: "CG PR 230 code review: routed record-count check made a real assertion"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@c748818b253b2f8bf5f5ddc6518684a70d215930; src/routed_generation.rs (generate_kani, lines 297-350)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: references
---

# SR-820: CG PR 230 code review (code-review with the rust-review lane)

## Summary

Ticket: IR-468. PR: agent-ix/quire-contract-codegen#230 at c748818 (one commit over origin/main,
`src/routed_generation.rs` +3/-1). The PR turns the `debug_assert_eq!(records.len(), group.len())`
ahead of the `records.into_iter().zip(group)` in `generate_kani` into `assert_eq!`, with a comment
saying why. Rust lane folded in, as `rust-review` says.

Measured by the reviewer:

- Diff. `git diff origin/main...HEAD` is exactly the macro rename plus a two-line comment. The
  assertion's operands and message are unchanged.
- Reachability of the panic. `negotiate_kani_obligations` (`src/kani_obligations.rs:462-519`,
  same crate) builds `states` by mapping `request.items` one to one and pushes exactly one record
  per state in an `enumerate` loop, and `items` in `generate_kani` maps `group` one to one. So
  `records.len() == group.len()` holds by construction; the assertion is an internal-invariant
  check, not an input check. A panic, rather than a new `RoutedGenerationError` variant, is the
  right tool for an invariant the caller cannot violate.
- Panic surface. The new `assert_eq!` is the only added panic site; it is not reachable from
  caller input. No `unsafe`, no integer conversion, no allocation change.
- Comment. Accurate: in release `zip` truncates to the shorter side.
- Tests. No test added. None can be: the mismatch is unreachable without a seam, and tests run
  with debug assertions on, where the old `debug_assert_eq!` already fired. Reverting the change
  would pass the suite; that is a property of the invariant, not a weak oracle in this PR.
- Gates (reviewer, own worktree, head c748818): see the gap-analysis artifact SR-821 and the
  gate log; every lane passed.

The adjacent `debug_assert!(first.is_some(), ...)` at `src/routed_generation.rs:329` is the
same class and is left debug-only. Whether it belongs to IR-468 is an acceptance question,
recorded once as SR-821 FND-001, not duplicated here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. The change is correct, minimal, idiomatic, and its comment is accurate. The acceptance
gap on the sibling assertion is SR-821 FND-001.
