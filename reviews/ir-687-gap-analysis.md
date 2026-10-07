---
id: SR-3064
title: "IR-687 gap analysis: FR-034 AC-57..AC-65 against the computed matrix and the published guardian review source"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen branch spec/ir687-prearm-negative-custody (frozen head named in the Linear marker) compared with main; quire matrix --format tsv on both trees; spec/kani/functional/FR-034-caller-death-ownership.md AC-57..AC-65 and CODE-gate sentences; published guardian review-source backup ref (the older one, whose head commit is 'Retain original outer input on pre-arm split failure'): outer_sampling.rs unclaimed_failure_step and preparation failure delivery, helper_entry.rs run_outer, launcher_owner.rs, caller_bootstrap.rs, outer_failure.rs"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: references
---

## Summary

Ticket: IR-687. Plan completion: not assessed. This is a planless gap analysis.

The computed Test Matrix grows from 588 to 597 criterion records, and every prior record is unchanged. AC-57..AC-63 and AC-65 compute `untagged`. AC-64, which is Inspection-only, computes `method-without-symbol`, the same as the existing PLANNED Analysis rows AC-41..AC-50.

No test claims the new criteria. Because this change is spec-only, there is no code to find stubs in. The gap analysis therefore checks the spec's statements about current source against the published guardian review source.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The CODE-gate status sentence for the claimed-startup transaction describes the source incompletely. In the guardian source, a claimed failure has no negative publication route at all. unclaimed_failure_step refuses whenever a stored I claim exists (outer_sampling.rs:943-969), and the helper then returns the original Sampling error (helper_entry.rs run_outer). The cited "successful later exhaustion has ledger, history and OwnerStop custody" exists only on the unclaimed and preparation paths. There, a fresh Exhausted tick SUPERSEDES the earlier failure by retiring the unsent frame (outer_sampling.rs:1028-1060). With a partial frame, it returns UnexpectedPhase and loses the exhaustion. The CODE gate omits the missing claimed-path negative route, and its wording suggests the current behaviour already matches first-wins. | spec/kani/functional/FR-034-caller-death-ownership.md:358-361 |

## Verdict

**Changes requested (medium).** Restate the CODE gate with three facts:

- No claimed-startup negative transaction exists yet.
- The existing unclaimed retire-on-exhaustion behaviour is the opposite of first-wins; see SR-3062 FND-001.
- Exhaustion arriving during a partial send is currently lost.

The pre-Armed section's CODE-gate sentence is accurate: no L/C negative publication route exists, and positive-arm capability publication does not implement it.

## Dispositions

Round 1, re-checked at the branch's round-1 fix head (the commit after an ordinary main merge, subject 'Clarify negative startup publication and settlement contracts'; head named in the Linear marker only), against the newer published guardian review-source backup ref (the one whose head commit is 'Retain producer clock failure with borrowed outer setup custody'). Static, read-only; no build, test, Kani or replay run.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | fix commit 'Clarify negative startup publication and settlement contracts': FR-034:374-384 states no claimed-startup negative route exists today, that retire-on-fresh-exhaustion is pre-byte precedence, and that partial exhaustion can be lost. The cited CODE boundary was verified in the newer guardian source: finish_startup_negative_after_roles requires Reaped Code(0) and otherwise returns OuterExitAbnormal, and caller_public's unsettled cancellation path selects CleanupUnconfirmed |
