---
id: SR-3060
title: "IR-687 spec review (base checklist): pre-Armed negative custody and claimed-startup terminal transaction"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen branch spec/ir687-prearm-negative-custody (frozen head named in the Linear marker); spec/kani/functional/FR-034-caller-death-ownership.md (FR-034 AC-57..AC-65 and their two Behavior sections), spec/kani/matrix/TC-049-caller-death-ownership.md (Pre-Armed negative capability checks, Claimed-startup negative transaction checks, nine Expected Results rows); context: published guardian review-source backup ref (the older one, whose head commit is 'Retain original outer input on pre-arm split failure') (launcher_owner.rs confirm_arm descriptor identity check, outer_sampling.rs negative delivery)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: references
---

## Summary

Ticket: IR-687. This base checklist covers id format, AC testability, vacuity (the cheapest wrong implementation that would still pass), and AC/TC coverage for FR-034-AC-57..AC-65. All nine ids are well formed and contiguous, and each one is unique repo-wide. Each AC has a TC-049 procedure step and an Expected Results row. Status is PLANNED/UNRUN throughout.

The cheapest wrong implementation for each AC, and what catches it:

- **AC-57.** L sends a fresh `pidfd_open` of O's numeric PID instead of a clone. Not caught by the stated oracle; see FND-002.
- **AC-58.** C trusts the L sender PID only. Caught by the wrong-sender, credential and build/run cases.
- **AC-59.** A zero-right fallback. Caught by the missing-right and outside-AwaitArm cases.
- **AC-60.** C uses the pidfd for more than cleanup. Partly caught; see SR-3063 FND-004.
- **AC-61.** A fabricated Armed. Caught by the no-M/backend marker.
- **AC-62.** C adopts the stamp only after L's wait. Caught by the delayed-wait case with an absent deadline.
- **AC-63.** A ready pidfd taken as settlement. Caught by the case where the pidfd is ready but the wait is unconfirmed.
- **AC-64.** Inspection only.
- **AC-65.** Later facts are recorded and then dropped. Not caught; see FND-001.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Later-fact retention in AC-65 cannot be observed. Once the first byte is sent, no OwnerStop path exists in O; the guardian source moves to Committed, and an Exhausted tick with a partial frame returns UnexpectedPhase. Facts kept in "original O/L accounting or diagnostic custody" are dropped when O exits. No text says what consequence a retained later error or exhaustion has: O's exit status, C's Code0 O-wait acceptance, a diagnostic, or nothing. The cheapest wrong implementation records the fact into a field that is dropped at exit, and TC-049 step 3 "Inspect the retained ... custody" cannot fail it. Two readers can also disagree on whether a retained later error makes O exit non-zero, which would turn C's original-cause result into CleanupUnconfirmed. | spec/kani/functional/FR-034-caller-death-ownership.md:346-350; spec/kani/matrix/TC-049-caller-death-ownership.md:1010-1015 |
| FND-002 | medium | The AC-57 and AC-60 oracle compares the forwarded descriptor with L's retained one "using actual descriptor identity". The guardian's existing pattern is a fstat st_dev/st_ino comparison (launcher_owner.rs:587). With pidfs, every pidfd for one process shares an inode, so a fresh `pidfd_open` of the numeric PID is indistinguishable from a clone. Before pidfs, all pidfds share one anonymous inode, so a pidfd for ANY process matches. Under that oracle, the "no reopened numeric-PID substitute" and "foreign capability must refuse" predicates are vacuous. Open-file-description identity, for example KCMP_FILE, would distinguish them. | spec/kani/matrix/TC-049-caller-death-ownership.md:972-976; spec/kani/functional/FR-034-caller-death-ownership.md:295-296 |

## Verdict

**Not merge-ready on its own merits.** Two medium testability gaps remain. The ids, status, Test/Inspection/Analysis method labels and Expected Results rows are otherwise sound. The examined criteria with no finding are AC-58, AC-59, AC-61, AC-62, AC-63 and AC-64.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | AC-72's runtime Test half cannot tell retention of late exhaustion from dropping it: holding settlement incomplete already forbids normal O Code0 for every implementation, and once settlement completes the planner's semantics give the dropped fact no observable consequence. Only the white-box source-state assertion with a mutant clearing the state at COMMIT can fail. TC-049 claimed-startup step 3 does not name the seam that assertion reads, and it says the runtime witness may stay owed. | spec/kani/functional/FR-034-caller-death-ownership.md:356-362; spec/kani/matrix/TC-049-caller-death-ownership.md:1089-1094 |

## Dispositions

Round 1, re-checked at the branch's round-1 fix head (the commit after an ordinary main merge, subject 'Clarify negative startup publication and settlement contracts'; head named in the Linear marker only), against the newer published guardian review-source backup ref (the one whose head commit is 'Retain producer clock failure with borrowed outer setup custody'). Static, read-only; no build, test, Kani or replay run.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | fix commit 'Clarify negative startup publication and settlement contracts': FR-034:356-362 names WHERE (actual accounting/history/owned-stop custody until settlement, ending with O exit, no after-exit diagnostic) and AC-72 plus TC step 3 add a mutant clearing that state at COMMIT |
| FND-002 | fixed | fix commit 'Clarify negative startup publication and settlement contracts': FR-034:315-321 and AC-66 require an admitted KCMP_FILE or equivalent open-file-description oracle, reject st_dev/st_ino equality, and require same-O fresh-open and foreign-pidfd negatives; unavailable oracle is UNRUN |
