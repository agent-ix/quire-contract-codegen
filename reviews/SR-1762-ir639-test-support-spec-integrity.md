---
id: SR-1762
title: "IR-639 guardian-test-support spec review (integrity)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@7742a43f62fc3feddde61a529a2bab37b4d0a67b; spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md, spec/kani/matrix/tests.md, spec/tests.md (PR #305 diff against main 28553daeb1e9cfd88bb6620125bb1df27273c68e)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-1762: IR-639 guardian-test-support spec review, integrity

## Summary

Ticket: IR-639. PR: quire-contract-codegen#305. Reviewer: claude-opus-5-5, session
9072f908-e626-4176-ae33-69e582990445. I checked the amendment for internal consistency, factual
accuracy against the merged code it now cites, and matrix and index agreement. Three findings: two
medium and one low.

## Method

I compared the PR's status changes for PR #295 ("merged") with git history (7889182, #295 on
main) and with the bubblewrap arguments in `src/kani/run/namespace.rs:128-147`. I checked that the
26 original criteria are retained, that AC-27 to AC-30 appear in TC-049, the Kani matrix row,
the TC-049 index row and Expected Results, and that `spec/tests.md` agrees. I also checked the
build configurations TC-049 needs against Cargo's feature model: one compilation of a crate has
one feature set, and the single `tests/it` binary links one library build. Scope units examined:
the FR-034 Description and Dependencies, FR-034-AC-14, AC-23, AC-24 and AC-27 to AC-30, TC-049
Description and steps 1, 12, 13 and 14, the two Kani matrix rows and the `spec/tests.md` Kani
row.

## Verdict

**FAIL: two medium findings and one low finding.** The following were clean:
- All 26 original criteria are retained, and AC-26 is unchanged.
- AC-27 to AC-30 are listed consistently in the FR-034 matrix row, the TC-049 index row and the
  Expected Results.
- PR #295 is merged on main, so the updated Description and Dependencies status is accurate.
- The matrix rows stay Planned and claim no coverage.
- No vendored file, hash or pin is introduced.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-034-AC-14 now says the bubblewrap "PID-1/new-session" features are "required by the merged containment code slice (PR #295)". The merged launcher passes `--unshare-user`, `--unshare-pid`, `--die-with-parent`, `--info-fd` and `--block-fd`, and neither `--as-pid-1` nor `--new-session`. Those two are guardian (IR-639) prerequisites. Documentation written to this AC would wrongly attribute them to the current release. Separate the prerequisites the merged slice requires from those the planned guardian adds. | spec/kani/functional/FR-034-caller-death-ownership.md:257, src/kani/run/namespace.rs:128-147 |
| FND-002 | medium | FR-034-AC-23 and TC-049 steps 1 and 13 need the public caller-death fixtures, the compile-fail consumer and the controls built without guardian-test-support, while step 12 needs it on. A single cargo invocation compiles the library with one unified feature set. The suite convention is one `tests/it` binary, and a self dev-dependency enabling the feature would unify it into every test build. "Without guardian-test-support" is therefore ambiguous: it could mean "does not call the operation" (vacuous when the feature is compiled in) or "a library built with the feature off". The TC does not say how the two configurations are built and run. State that AC-23/26 fixtures run against a feature-off library build, and allocate the feature-on and feature-off builds to separate named invocations. | spec/kani/functional/FR-034-caller-death-ownership.md:266, spec/kani/matrix/TC-049-caller-death-ownership.md:28-34, spec/kani/matrix/TC-049-caller-death-ownership.md:140-148 |
| FND-003 | low | TC-049 step 12 invokes "the single guardian-test-support fixture operation", but the only build defined before it (step 1) is explicitly without the feature. The feature-enabled build is defined only later, in step 13. A reader executing the procedure in order has no build that provides the operation. Move the feature-enabled build before step 12, or have step 12 reference step 13's build. | spec/kani/matrix/TC-049-caller-death-ownership.md:28, spec/kani/matrix/TC-049-caller-death-ownership.md:109-111, spec/kani/matrix/TC-049-caller-death-ownership.md:140 |

## Dispositions

Round 1, reviewed at ef9ee8ec51b81c4f9a967c6ea738e16cc9e4528d.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | ef9ee8ec51b81c4f9a967c6ea738e16cc9e4528d: AC-14 now attributes PID-1/new-session to the planned guardian and lists the merged #295 flags as user/PID namespace, die-with-parent, info-fd and block-fd. That matches src/kani/run/namespace.rs. |
| FND-002 | fixed | ef9ee8ec51b81c4f9a967c6ea738e16cc9e4528d: AC-23 now requires a normal library built with the feature off. FR-034 and TC-049 steps 1/12 define separate named guardian-feature-off/on Cargo invocations with no self dev-dependency unification. |
| FND-003 | fixed | ef9ee8ec51b81c4f9a967c6ea738e16cc9e4528d: Step 12 now begins by building the feature-on library, caller and helper before it uses the operation. |
