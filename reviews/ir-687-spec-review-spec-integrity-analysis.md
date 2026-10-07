---
id: SR-3062
title: "IR-687 spec review (integrity): pre-Armed negative custody and claimed-startup terminal transaction"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen branch spec/ir687-prearm-negative-custody (frozen head named in the Linear marker, two commits over main); spec/kani/functional/FR-034-caller-death-ownership.md (new sections Pre-Armed negative cleanup capability custody and Claimed-startup negative terminal transaction, AC-57..AC-65), spec/kani/matrix/TC-049-caller-death-ownership.md (two new procedure sections and nine Expected Results rows); context: FR-034 resource/deadline precedence over malformed partial content, FR-034 confirmed-settlement precedence, FR-034-AC-8, FR-034-AC-10, spec/kani/matrix/tests.md, published guardian review-source backup ref (the older one, whose head commit is 'Retain original outer input on pre-arm split failure') (src/kani/run/outer_sampling.rs, helper_entry.rs, launcher_owner.rs, caller_bootstrap.rs, outer_failure.rs, control.rs)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: references
---

## Summary

Ticket: IR-687. This is a SPEC-only change with no PR yet. It adds two FR-034 Behavior sections and FR-034-AC-57..AC-65, plus matching TC-049 procedures and Expected Results rows. This integrity pass checks four things: internal consistency, consistency with the existing FR-034 precedence text, consistency with the published guardian review source, and traceability completeness.

The id range AC-57..AC-65 is unique. It is disjoint from the sibling IR-655 range AC-51..AC-56. The PR deletes no existing line. The computed matrix grows from 588 to 597 criteria, and every prior row is byte-identical. `quire validate` passes on both documents with module warnings only.

I quote the two precedence passages the brief asked about. The planner's "first established wins" decision does not contradict either one textually.

- FR-034 Run artifact section: "Independently established resource/deadline stops take precedence over malformed partial content with existing single-run or whole-batch behavior". The new text keeps this explicitly at FR-034:329-330.
- FR-034 Settlement section: "other candidates retain existing resource, report and refusal precedence". No spec text defines precedence between a genuine failure and resource exhaustion, so the new pair rule fills a gap rather than overriding anything.

The contradiction is internal to the new text, and it comes from an undefined source-only rule that the new text imports.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-034:321-322 requires O to "preserve the FIRST independently established terminal stop". FR-034:338 then permits "an unsent frame may be retired under the existing zero-progress rule". No spec text defines that rule. In the guardian source, the only zero-progress retirement exists so that a LATER fresh Exhausted tick can replace an earlier, already established genuine failure: the unsent frame is retired and the state moves to OwnerStopped (outer_sampling.rs:1028-1060 and :526-552). Applied to the claimed path, a failure followed by an exhausting pre-send tick, which FR-034:335 makes mandatory, would publish exhaustion and break first-wins. Two conforming implementations therefore disagree. TC-049 step 1 checks only "failure then exhaustion" and "exhaustion first"; it never checks "failure first, exhaustion found at the required pre-send tick before the first byte". | spec/kani/functional/FR-034-caller-death-ownership.md:321-322; spec/kani/functional/FR-034-caller-death-ownership.md:335-339; spec/kani/matrix/TC-049-caller-death-ownership.md:998-1001 |
| FND-002 | medium | FR-034:285 says "the O→L context remains zero", and TC-049:992 says "Existing O→L zero-right negative handling remains intact". The guardian source has no O→L negative route. helper_entry.rs:93 says "No authenticated L/C negative publication route exists yet", L's confirm_arm decodes the Armed-only grammar, and the existing zero-right Committed/OperationalFailure travels O→C on O's own channel. The O→L receive site and L's schema selection between Armed and Committed during confirm_arm are new CODE. The text presents them as existing, and AC-64's Inspection ("reuses the existing bounded controls") could accept that wording at face value. | spec/kani/functional/FR-034-caller-death-ownership.md:274-275; spec/kani/functional/FR-034-caller-death-ownership.md:285; spec/kani/matrix/TC-049-caller-death-ownership.md:992 |
| FND-003 | medium | The TC-049 Evidence delivery allocation table has a row for every FR-034 criterion from AC-35 through AC-50, and the sibling IR-655 adds AC-51..AC-56 there. This PR adds AC-57..AC-65 only to Expected Results. The stage-1 ordinary-seam versus owed-evidence allocation for the new criteria is therefore unrecorded. | spec/kani/matrix/TC-049-caller-death-ownership.md:33-117 |
| FND-004 | low | spec/kani/matrix/tests.md still lists FR-034 as "FR-034-AC-1 through FR-034-AC-40", and the TC-049 criterion list ends at AC-40. The new AC-57..AC-65 (like the earlier AC-41..AC-50) are absent from the trace tables. | spec/kani/matrix/tests.md:56; spec/kani/matrix/tests.md:87 |

## Verdict

**Not merge-ready.** FND-001 is a core contradiction in the planner-decided transaction rule. Fix it by stating explicitly whether retiring an unsent frame may ever replace a first-established genuine failure. For the claimed path, the planner decision implies that it may not. Then add the "failure first, exhaustion at the required pre-send tick" Test case.

Recorded clean, examined:

- PLANNED/UNRUN status on every new row.
- No SHAs, local paths or conflict markers.
- AC-8 and AC-10 are unchanged and not weakened.
- CleanupUnconfirmed precedence is retained.
- The absent peak field is worded "no peak transport", never zero.

Merge with the sibling IR-655 head conflicts only at the tail of the FR-034 AC table, where both branches append rows. Resolve it by keeping AC-51..AC-56 followed by AC-57..AC-65. TC-049 auto-merges.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | AC-58 says "mapped sender UID/GID0", but the prose at FR-034:280 says "kernel sender UID/GID0", and L's actual check is on kernel sender credentials. The negative envelope has no mapped_uid field, but "mapped" can be read as a packet claim, which the same section forbids as authority. | spec/kani/functional/FR-034-caller-death-ownership.md:280; spec/kani/functional/FR-034-caller-death-ownership.md:1736 |
| FND-006 | low | In both new TC-049 procedure sections, steps 2 to 5 run inline inside the paragraph of item 1 (for example "must refuse. 2. Prove ..."). They render as one list item, so the steps cannot be referenced or traced individually. | spec/kani/matrix/TC-049-caller-death-ownership.md:1056-1075; spec/kani/matrix/TC-049-caller-death-ownership.md:1081-1102 |
| FND-007 | low | The pre-COMMIT rule says "O shall apply existing resource/deadline/refusal precedence", but it states concretely only the exhaustion-over-failure retirement. It does not say whether a work-deadline expiry or an owner refusal established before the first byte also retires an unsent failure. "Unpoisoned" is a source term with no spec definition. | spec/kani/functional/FR-034-caller-death-ownership.md:331-337 |

## Dispositions

Round 1, re-checked at the branch's round-1 fix head (the commit after an ordinary main merge, subject 'Clarify negative startup publication and settlement contracts'; head named in the Linear marker only), against the newer published guardian review-source backup ref (the one whose head commit is 'Retain producer clock failure with borrowed outer setup custody'). Static, read-only; no build, test, Kani or replay run.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | fix commit 'Clarify negative startup publication and settlement contracts': COMMIT is defined as the first emitted byte; before it existing precedence applies and a fresh complete exhausting pre-send tick retires a zero-progress failure into OwnerStop while the earliest stop/cutoff never resets; after it the candidate is immutable (FR-034:331-337, AC-65, AC-70); TC step 1 adds failure-first/exhaustion-at-pre-send-tick. No FR-034 text contradicts this: the malformed-content precedence and the confirmed-settlement precedence passages are kept, and the FIRST-stop clock allowance is preserved separately |
| FND-002 | fixed | fix commit 'Clarify negative startup publication and settlement contracts': FR-034:274-278 states the O→L negative is a NEW route and that the current zero-right OperationalFailure is O→C after arm; the TC 'existing O→L handling' sentence is gone |
| FND-003 | fixed | fix commit 'Clarify negative startup publication and settlement contracts': the Evidence delivery allocation table now has AC-57..AC-75 rows, plus a separate 'Negative startup transaction expectations' table |
| FND-004 | fixed | fix commit 'Clarify negative startup publication and settlement contracts': tests.md FR-034 row reads AC-1 through AC-75 and the TC-049 criterion list includes AC-57..AC-75 |
