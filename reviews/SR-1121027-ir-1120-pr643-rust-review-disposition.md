---
id: SR-1121027
title: "IR-1120 CG #643 rust-review disposition"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; PR #643 head f8b6b0573c71cb0476a67253c00c1cb3a34005e6; rust-review disposition of SR-1121023 and the Rust fixes since 3f40b53ab057c52d72fdf49ca29faf2bcef89572"
review_set: subset
---

## Summary

Disposition of SR-1121023 at head f8b6b0573c71cb0476a67253c00c1cb3a34005e6. The original style note is fixed and the changed Rust reads cleanly. One low evidence gap is new: the supplied clippy log did not lint the test code.

## Verdict

PASS on source, with the lint of the test target left to the lead's final gate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `cg643-ac4-review-fix-clippy.log` does not cover `refinement_source_tests.rs`. It has no `proptest` or `rand` entry, although `proptest` is a dev-dependency that `--all-targets` would have to check, so the run omitted test targets and this `#[cfg(test)]` module. The log is clean lint evidence for library code only. | Makefile:51; Cargo.toml:58; src/routed/smt/refinement.rs:314-316 |

## Coverage

Original finding outcomes at this head:

| Original | Outcome | Evidence |
| --- | --- | --- |
| SR-1121023 FND-001 | fixed | `OsString` is now imported at src/routed/smt/refinement.rs:7 and used unqualified at src/routed/smt/refinement.rs:267. |

Fix code checked with no finding:

- `&name[2..]` (src/routed/smt/refinement_source_tests.rs:62) is test-only and every input is `format!("op{index}")`.
- `for row in 0..7` indexes two slices (src/routed/smt/refinement_source_tests.rs:304-306); a direct range is the plain form for that.
- Bounds in the pairwise loop are in range for the eight-entry `first_expected`, which is asserted at line 307.
- `valid_solver_tail`'s slice and `get` calls are bounded by the loop condition (src/routed/smt/mod.rs:326-330).

The focused test compiled and passed in `cg643-ac4-review-fix.log`, so the test module does type-check. This reviewer ran no compiler, clippy or test. The `/rust-review` and `rust-style` skills were not loadable here, so the review used repo conventions and general Rust idiom. Plan completion: not assessed.

## Examined scope and bindings

```yaml
scope:
- id: FR-045-AC-4
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: For a refinement with seven step rows, CG retains one `Base` and exactly
    seven typed `Step { row }` records numbered 0 through 6 from the recompiled, canonically
    ordered `steps` array, even when records arrive out of order or the source rows
    were written in another order. Each record binds the refinement item, subjects,
    distinct invocation attempt, role and row, and exact FR-315 `Initial` or `Step
    { row }` query bytes sent on stdin; changing a query byte or exchanging the base
    and a step role refuses the handoff. With zero step rows, the complete shape has
    only `Base`.
bindings: []
```

## Dispositions

| Finding | Outcome | Evidence |
| --- | --- | --- |
| FND-001 | deferred: the lead's final gate, which must include `make lint` | `make lint` runs clippy with `--all-targets -- -D warnings` (Makefile:51). A warning there in the test module reopens this finding. |
