---
id: "SR-1481"
title: "CG PR 269 gap analysis: lock move, depth-limit removal, flat wire, decimal-string integers, versionless dependency entries"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@17f779983e55e025fde18969ad08ee51f59fbf22; FR-014-AC-10, FR-014-AC-41, FR-018-AC-20, FR-021-AC-23, FR-016-AC-15, FR-016-AC-17, interface-001 ReplayPackage::new, and the tests and code the PR diff touches (merge base f3ece43)"
---

# SR-1481: CG PR 269 gap analysis

## Summary

Ticket: IR-565 (primary), with IR-570 and IR-579. This is a planless audit scoped to the PR
diff. Plan completion: not assessed.

Examined, criterion by criterion:

- FR-014-AC-41, FR-018-AC-20, FR-021-AC-23 (unrecognised limit kinds). Each AC now lists
  `nodes`, `edges`, `occurrences` and `diagnostics`. Each is backed by
  `tc_024_/tc_029_/tc_031_a_failed_record_is_refused_by_its_limit_kind`, traced to the right AC,
  which iterates `UNRECOGNISED_KINDS` and has exactly those four entries. The binding is
  correct. TC-024 step 3, TC-029 and TC-031 step 11 list the same four kinds.
- FR-014's Behavior section ("the snake_case of the `CheckedPackageLimit` variant's name") lists
  the same four names, which matches `classify_lowering_failure`.
- FR-014-AC-10 (an operand neither a literal nor a reference is refused).
  `tc_024_refused_items_are_typed_emit_no_code_and_leave_siblings_unchanged` still asserts the
  typed refusal, now with `term: "aggregate"`. The criterion is still backed, through a term the
  V2 reader admits (see SR-1480 for reachability).
- FR-016-AC-15 (one dependency entry per lock selection, in identity order, with the recorded
  package_id). `tc_026_the_request_package_reference_carries_the_lock_dependencies` still
  asserts the order and the package_id. No AC names `version`.
- FR-016-AC-17 (a stale package_id is refused by QSL).
  `tc_026_a_lock_recording_another_dependency_identity_is_refused` is unchanged and runs
  against the identity-only import.
- interface-001 `ReplayPackage::new`. The merge kept main's planned note on line 159
  ("planned (IR-465, FR-016-AC-24): Duplicate is removed ..."). The PR's change to line 160
  drops only `version` from the entry tuple, so the conflict resolution is correct.
- Remaining-mention sweep across `spec/` and `src/`: no remaining `depth` refers to the lowering
  limit. The `depth` hits that remain are FR-021's call-depth budget, FR-018's "at any depth"
  and a local in `playback.rs`. No remaining `version` refers to a dependency entry, a lock
  selection or an import. The `project(deref(self), field)` notation in FR-014-AC-38 and
  FR-015 describes a chain of nodes joined by references, which `src/kani/generate/frame.rs`
  already follows. It does not describe a nested application, so the flat wire leaves it
  consistent.
- Strict coverage (`quire coverage --strict`, quire 0.36.1 / engine 0.50.1): 91 unbacked at main
  f3ece43, 91 at current main 67bc7b3 and 91 at head 17f7799. There are 0 contradicted statuses.
  The PR does not regress coverage. The pre-merge commit aa71a53 measures 89, which is the
  number the PR body reports (FND-001).
- Test-oracle strength: no test was deleted, ignored or renamed (540 `#[test]` and 27
  `#[ignore]` on each side). The relabelled `Depth`-to-`Nodes` assertions are each independent
  of the kind. The one weak replacement assertion is SR-1480 FND-001.

## Verdict

No implementation gap. Every touched AC is still backed by a correctly traced test, and the
spec edits match the code. One low finding about the PR body's evidence.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The PR body's claim "quire coverage --strict is 89 unbacked before and after" does not hold at the reviewed head. Re-measured: 91 at main f3ece43, 91 at head 17f7799, and 89 only at the pre-merge commit aa71a53. The equality that matters still holds (91 = 91), but the number reported is stale evidence from before the merge of main | PR #269 body |

## Dispositions

Round 1, reviewed at 9b87323bfc33987e770bafa8177d159851c53b50. I re-measured
`quire coverage --strict` (quire 0.36.1): 91 unbacked and 0 contradicted at head 9b87323, and
91 at current main 67bc7b3. The PR body now reads "quire coverage --strict reports 91 unbacked
rows on this head, equal to main's 91", which matches. The three failed-record tests and
`tc_026_the_request_package_reference_carries_the_lock_dependencies` still carry their traces
and pass at head.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | PR #269 body edited (no commit; re-measured 91 = 91 at 9b87323 / 67bc7b3) |
