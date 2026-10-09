---
id: SR-3200
title: "IR-694 spec review (base checklist): queued InnerClaimed close-only disposal during claimed-startup negative receipt"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen PR #327, branch spec/ir694-queued-claim-cleanup (frozen head named in the Linear marker, one commit over main); spec/kani/functional/FR-034-caller-death-ownership.md (new section Queued claimed-phase cleanup during negative receipt, FR-034-AC-94), spec/kani/matrix/TC-049-caller-death-ownership.md (allocation row, Queued claimed-phase cleanup checks, Expected Results row), spec/kani/matrix/tests.md (FR-034 and TC-049 rows); context: FR-034 Claimed-startup negative terminal transaction (AC-77 receipt paragraph), Startup and termination observations stage table, FR-034-AC-24, AC-57..AC-77; published IR-639 guardian review-source backup ref (caller_bootstrap.rs advance_startup InnerClaimed admission)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: references
---

# SR-3200: IR-694 spec review (base checklist)

## Summary

Ticket: IR-694. PR: quire-contract-codegen#327. Spec-only: one new FR-034 Behavior section, one
new criterion FR-034-AC-94, one TC-049 procedure section, one allocation row, one Expected Results
row and the two tests.md trace rows. `make spec` (quire validate) exits 0; its only warnings are two
pre-existing EARS warnings on FR-017, which this PR does not touch.

The intended rule is present: a complete expected queued claim and its raw I right are held only
for close, with no identity/phase/gate/Dispatch/signal authority; the original C stop/cutoff, full
original O authentication and whole-chain settlement are retained; missing, partial or malformed
Failure retains CleanupUnconfirmed; no new frame, right, outcome, cause, window or fallback is
allocated. The new text does not contradict or weaken AC-77, the IR-687 negative-custody rules
(AC-57..AC-77) or AC-24.

## Method

Read the full diff against main, the merged AC-77 receipt paragraph (FR-034 lines 391-397) and
TC-049 claimed-startup step 3, the stage table's ClaimedGated admission (pidfd/start/parent/
namespace), AC-24 and AC-57..AC-77. Checked the C-side admission order in the published IR-639
review source to test whether the trigger is decidable by C. Ran `make spec` and computed
`quire matrix` on main and on the PR tree.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The trigger is stated in terms C cannot decide when it acts. The section applies "When a COMPLETE expected InnerClaimed ... is queued on C's original startup cursor before O's committed claimed-startup OperationalFailure" and retains receipt "through AC-77-caused I death". C reads the cursor in order: it evaluates the claim's live-child admission before it has read, or can know of, any following Failure, and it cannot tell AC-77-caused I death from any other I death. The text never states C's actual transition: on a complete, authenticated, expected claim whose live-I admission fails, enter close-only provisional receipt and keep reading the same cursor until the original cutoff. It also leaves open whether a claim whose I is still alive takes the unchanged positive path. One implementer will add read-ahead; another will enter provisional receipt on any admission failure. The second reading also turns a spontaneous, non-AC-77 I death with no following Failure from a prompt setup refusal into a wait until the cutoff and then CleanupUnconfirmed. | spec/kani/functional/FR-034-caller-death-ownership.md:432-436; spec/kani/functional/FR-034-caller-death-ownership.md:1836 |
| FND-002 | medium | AC-94, the criterion the matrix tracks, does not name the disposal state the ticket exists to add: the raw unvalidated I right must actually be CLOSED through owned cleanup. "without positive phase or I-right authority" is satisfied by an implementation that retains the raw right indefinitely or leaks it past settlement. The AC row also omits that wrong-phase, wrong-sender, malformed, partial or extra-right prior traffic is refused rather than entering provisional receipt. Both are in the prose and the TC procedure, but the TC-049 Expected Results "Regression caught" column has no leaked or never-closed raw-right mutant. | spec/kani/functional/FR-034-caller-death-ownership.md:1836; spec/kani/matrix/TC-049-caller-death-ownership.md:1226 |
| FND-003 | low | "InnerClaimed" and "startup cursor" appear nowhere in the spec except this new text. InnerClaimed is a variant name from the unmerged IR-639 source. A reader of the spec alone cannot resolve which frame or stage transition is meant. Anchor the term to the stage table's ClaimedGated admission (the outer phase reply carrying the claimed INIT pidfd) and define the cursor as C's single retained direct-O startup control stream. | spec/kani/functional/FR-034-caller-death-ownership.md:432-433; spec/kani/matrix/TC-049-caller-death-ownership.md:1136-1137 |

## Verdict

**Changes requested (two medium, one low). `make spec` passes; the PR is not merge-ready until
FND-001 and FND-002 are fixed.** Each needs a sentence or two:

- FND-001: state the C-observable trigger. For example: "When C's live-child admission of a
  complete, authenticated, expected InnerClaimed fails because I has exited or its I lease reached
  EOF, C shall ..., and shall continue reading the same cursor for O's committed Failure until the
  original cutoff; otherwise the existing refusal applies". Say whether a claim admitted while I
  still lives keeps the unchanged positive path.
- FND-002: add "and closes the raw I right through owned cleanup; damaged or extra-right prior
  traffic is refused" to AC-94, and add a never-closed raw-right mutant to the Expected Results row.

Recorded clean, examined:

- No new frame, right, outcome, cause, public field, ACK, window, cap or cancellation authority.
- No generic startup waiver, no retry with a new parser, no splice, no EOF-inferred terminal.
- The original C stop/minimum cutoff is neither reset nor extended.
- Settlement is required for the whole chain (I/M/O/L/capture/creator/control-EOF).
- Positive admission is unchanged for every actual positive transition.
- AC-77, AC-57..AC-77 and AC-24 are not weakened. AC-24's lease-EOF cancellation is untouched.
- Actors C, O, I and M are named.
- AC-94 is PLANNED/UNRUN and claims no code credit.
