---
id: SR-578002
title: "IR-364 PR #578 spec-review/spec-integrity-analysis review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; PR #578 head 33116e82f60b1f9e0affff3f6bad89530df8859a on base b60b18ef; spec/strategy/functional/FR-002-tristate-proptest.md, spec/strategy/matrix/TC-004-proptest-strategies.md; IR-364"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-004
    type: reviews
---

## Summary

Integrity sub-analysis of PR #578 at frozen head 33116e82, covering completeness, consistency and
atomicity of the FR-002 Outputs, Behavior, AC-16 and OQ-2 edits and the TC-004 procedure and
expected results. Ticket: IR-364.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-16 says the case "terminates as" `V2HarnessCaseResult::NonCompleted`, a per-case type, while Behavior says the campaign runner "shall terminate with this non-success result". The V2 campaign's typed terminal result (the FR-002-AC-5 analogue) is not named, so the spec does not say whether the runner returns the case result itself or wraps it in a campaign terminal variant | spec/strategy/functional/FR-002-tristate-proptest.md:91, spec/strategy/functional/FR-002-tristate-proptest.md:174, spec/strategy/matrix/TC-004-proptest-strategies.md:43-44 |

## Verdict

Integrity: mergeable after a low fix. Outputs, Behavior, AC-16, OQ-2 and TC-004 use one name
(`V2HarnessCaseResult::NonCompleted`) and one variant set. The resolved OQ-2 matches AC-16, and no
open question remains. AC-17 and AC-18 are unchanged and consistent with the new Behavior text. The
TC-004 Expected Results cover AC-16 in the same terms as the procedure. The undefined field and
missing stop channel are recorded once, in SR-578001.

## Dispositions

Round 1, checked against the repository at head 6c4f1f1623d94d49744d14bdc2129cbcd5435bd4 (merge-base main b60b18ef). FR-002-AC-17, AC-18 and AC-19 rows are byte-identical to main; only AC-16 changed, and it is not narrowed.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The campaign terminal is named: `V2HarnessCampaignResult::NonCompleted` carries the case result and `V2HarnessCampaignSummary`, in Outputs, Behavior, AC-16, OQ-2 and TC-004. |

## Coverage

```yaml
scope:
- id: FR-002-AC-16
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "terminates as CG-owned `V2HarnessCaseResult::NonCompleted` retaining that oracle's original typed variant and cause, its precondition or postcondition selection, and that selection's accepted execution point."
- id: FR-002
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "OQ-2 (resolved, non-completed oracle outcome): `V2HarnessCaseResult::NonCompleted` is the CG-owned result for an oracle that returns `Undefined`, `Refused`, or `Incomplete`. It retains the original typed stop and the selected accepted point, and creates no runtime verdict."
- id: TC-004
  path: spec/strategy/matrix/TC-004-proptest-strategies.md
  role: examined
  excerpt: "a non-completed oracle outcome to return `V2HarnessCaseResult::NonCompleted` with its original typed stop, selected accepted point, no runtime verdict or pass artifact, and unchanged prior verdict counters (AC-16)"
- id: FR-002-AC-5
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: context_only
  excerpt: "The generated campaign runner owns execution, explicit-discard accounting, and conclusion, returns a typed terminal result"
```
