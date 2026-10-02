---
id: "SR-821"
title: "CG PR 230 gap analysis: IR-468 acceptance against the routed Kani arm"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@c748818b253b2f8bf5f5ddc6518684a70d215930; src/routed_generation.rs (generate_kani), src/kani_obligations.rs (negotiate_kani_obligations, context), spec/routed/functional/FR-022-routed-generation.md (FR-022-AC-7, Outputs), spec/kani/functional/FR-015-bounded-kani-obligations.md (context)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
---

# SR-821: CG PR 230 gap analysis (IR-468 acceptance, scoped to the PR)

## Summary

Ticket: IR-468. PR: agent-ix/quire-contract-codegen#230 at c748818. Planless, scoped to the PR
diff. Plan completion: not assessed.

The ticket (untrusted text, re-measured) asks that the routed-generation record-count check,
"debug_assert only", become a real assertion. It cites `tests/it/routed_generation.rs:287-302`
at CG 5a924e1.

Re-measured by the reviewer:

- `tests/it/routed_generation.rs:287-302` at 5a924e1 is the body of a TC-033 refusal test; it has
  no `debug_assert`. The path in the ticket is wrong.
- `src/routed_generation.rs` at 5a924e1 has exactly two `debug_assert`s, at line 287
  (`debug_assert_eq!(records.len(), group.len(), "FR-015 reports one record per item")`) and line
  302 (`debug_assert!(first.is_some(), "FR-015 names an earlier position")`). The ticket's line
  range 287-302 is exactly these two lines, so the citation names the right lines in the wrong
  file and covers both assertions.
- The PR converts the first (now line 314) and leaves the second (now line 329) debug-only,
  saying in its body that it is "out of this ticket's scope". The body also says the citation is
  stale and "the test file has none", which is half the story: the line range matches the two
  production assertions.
- Release behaviour of the left-behind assertion: if `first_index` were ever out of range of
  `driver`, the `if let Some(first)` skips the remap and the record keeps a Kani-group position
  as its `first_index`, silently contradicting FR-022-AC-7 and FR-022 Outputs ("Every request
  index inside that record ... is the driver's request index, not a position in the Kani group").
  This is the same silent-wrong-output-in-release class the ticket is about. Like the count
  check, it is unreachable by construction today (`reject_duplicates_and_mixtures` names an
  earlier position in the same `items` vector).
- Spec: FR-015/FR-022 already state one record per item and the driver-index mapping. No spec,
  matrix or test change is needed for either assertion; the existing TC-033 tests (FR-022-AC-7)
  pass at head.
- PR hygiene: title "Make the routed record-count check a real assertion" carries no ticket id;
  body says "Closes IR-468" once, consistently; branch `fix/ir-468-routed-count-assert`.
- Gates at c748818 (reviewer's own worktree and target dir): `make ci` stops at the `spec` lane
  on this host because the Makefile's `override QUIRE := $(TRUSTED_HOME)/.npm-global/bin/quire`
  path does not exist here (environment, not the PR). The same `quire validate` command run with
  the PATH `quire` passed, and `make fmt-check lint msrv deny audit-unsafe rustdoc test` passed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The PR says "Closes IR-468" but leaves `debug_assert!(first.is_some(), "FR-015 names an earlier position")` debug-only. That is the second of the two lines the ticket's 287-302 range names (src/routed_generation.rs:287 and :302 at 5a924e1). In a release build a violated invariant would silently leave a group position in `first_index`, contrary to FR-022-AC-7. Either convert it in this PR (for example `*first_index = driver[*first_index];`, which panics out of range and drops the `if let`) or change the body to "Part of IR-468" and file the remainder. | src/routed_generation.rs:328-332 |

## Verdict

Changes requested (one medium). The record-count conversion is correct and complete. The
ticket's own cited range also covers the `first_index` assertion, so "Closes IR-468" overclaims
while it stays debug-only. The cheapest fix is the one-line conversion in this PR.

## Dispositions

Round 1, reviewed at 893c759e0a1c44cd8abc84d095824672562af48a (rebased onto origin/main 1629715;
fix commit 893c759 over 99d5cf3, the rebased c748818). `debug_assert!(first.is_some())` plus the
`if let` is replaced by `*first_index = driver[*first_index];` with a comment. For valid input
`reject_duplicates_and_mixtures` (`src/kani_obligations.rs`) sets `first_index` to a `position` in
`states[..index]`, which is always in range of `driver` (one entry per item), so output is
unchanged; an out-of-range index now panics in every build instead of staying unmapped. The
comment is accurate. The body now says "Closes IR-468" and describes both assertions. Gate log
for 893c759: `make ci` exit 0, including clippy `-D warnings` and both FR-022-AC-7 TC-033 tests.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 893c759 |
