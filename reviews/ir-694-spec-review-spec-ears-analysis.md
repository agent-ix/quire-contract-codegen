---
id: SR-3202
title: "IR-694 spec review (EARS conformance): queued claimed-phase cleanup prose and FR-034-AC-94"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-contract-codegen PR #327, branch spec/ir694-queued-claim-cleanup (frozen head named in the Linear marker); spec/kani/functional/FR-034-caller-death-ownership.md lines 430-459 (new Behavior section) and the AC-94 row; compared with the surrounding FR-034 prose and the declarative AC-57..AC-77 rows; make spec EARS diagnostics"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: references
---

# SR-3202: IR-694 spec review (EARS conformance)

## Summary

Ticket: IR-694. `make spec` raises no EARS diagnostic on FR-034. Its only two EARS warnings are
pre-existing, on FR-017 line 174, which this PR does not touch.

- The section's opening sentence is a well-formed event-driven EARS statement: "When ..., C shall
  retain ...".
- Most of the remaining statements name C as the subject.
- The AC-94 row uses the same declarative, non-`shall` form as the neighbouring AC-57..AC-77 rows,
  with the same PLANNED/UNRUN prefix. It raises no non-canonical warning.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Five new normative sentences have a non-actor subject before `shall`: "This exception shall apply only ...", "Provisional receipt shall retain ...", "It shall neither reset ...", "I death, I-lease EOF, the prior claim or close-only right shall supply neither ...", and "The original positive admission checks shall remain required ...". EARS expects the responsible system (C) as subject. The neighbouring FR-034 prose uses the same pattern and quire's grammar check does not flag it, so this is style only. Recasting with C as subject (for example "C shall apply this exception only to ...") would make the obligations attributable. | spec/kani/functional/FR-034-caller-death-ownership.md:440; spec/kani/functional/FR-034-caller-death-ownership.md:442; spec/kani/functional/FR-034-caller-death-ownership.md:447-449; spec/kani/functional/FR-034-caller-death-ownership.md:453-454 |

## Verdict

**EARS-clean for the tooling. One low style finding.** Actors C, O, I and M are named. No
`shall`-style non-canonical warning is introduced, and the AC row follows the neighbouring
declarative form.

## Dispositions

Replacement Codex disposition, round 1, on the frozen published PR head. Reviewer run 3911c05e-0b3e-4a4f-bac7-379bbc166b22. The original findings above are unchanged.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | The rewritten normative sentences assign C as the responsible actor. |
