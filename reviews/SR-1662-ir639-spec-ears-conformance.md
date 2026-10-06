---
id: SR-1662
title: "IR-639 spec review (EARS conformance): FR-034 requirement statements"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen#299; spec/kani/functional/FR-034-caller-death-ownership.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
---

# SR-1662: IR-639 spec review, EARS conformance

## Summary

Ticket: IR-639. PR: quire-contract-codegen#299 (spec only; the reviewed head is recorded in the private ticket marker). Reviewer: claude-opus-5-5, session 44b8eed1-c733-42e1-b8f3-7818119a17d5. FR-034 has 12 requirement statements: the Description's opening `shall` statement and 11 Behavior bullets. The engine check is grammar-clean. Semantic review found one high finding, three medium findings and one low finding. The dominant defects are a Behavior statement that contradicts its own acceptance criterion, and acceptance criteria with no parent statement.

## Method

I ran `quire validate --scope . spec/kani/functional/FR-034-caller-death-ownership.md --summary`, scoped to the one requirement-bearing file in the diff rather than a full spec sweep. It reported "1/1 docs grammar-clean (100%); 0 grammar finding(s): none" and no `[ears:*]` warnings. I then judged each statement for:
- the right pattern for the obligation (unwanted conditions as `If … then`);
- a subject that can actually perform the response in the stated condition;
- singular response;
- agreement with the acceptance criteria that bind it.

TC-049 is a test case and FR-034's acceptance criteria are criteria, not requirement statements. I read them only as context, to check that every criterion has a parent statement.

## Verdict

**FAIL: one high finding.** The following statements are clean:
- Behavior bullets 1 to 3 (guardian launch, lease before monitor, monitor and gate ownership).
- Bullet 5 (observer readiness before Dispatch).
- Bullets 6 and 7, which use the correct `If … then` unwanted-behavior pattern for lease EOF.
- Bullets 10 and 11 (recipe preservation, deadline charging).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Behavior bullet 9 contradicts FR-034-AC-11. The bullet reads "If the guardian or ownership observation fails while the caller is alive, then the executor shall refuse a proof conclusion unless owned teardown is confirmed", which permits a proof conclusion after a guardian failure whenever teardown is confirmed. AC-11 says that with a live caller, "guardian failure, missing ownership or unconfirmed cleanup yields a typed refusal and never verified/falsified evidence, including beside an otherwise valid success report". The criterion has no confirmed-teardown exception. The case "guardian fails after a valid run, teardown is confirmed" is accepted under one and refused under the other, so the two would be built differently. Make the statement and criterion agree, with one explicit rule for guardian failure followed by confirmed teardown. | FR-034, FR-034-AC-11 |
| FND-002 | medium | The Description's opening statement has a subject that cannot respond and uses the wrong pattern. It reads "When the original codegen caller terminates during startup or execution, the bounded executor shall cancel its owned backend namespace…". The bounded executor runs inside the caller (Behavior bullets 1, 4, 9 and 11 make the executor and the caller one actor), so it cannot act after the caller terminates. Behavior assigns that response to the guardian. Caller death is also an unwanted condition, not a normal event. Restate it as "If the original caller terminates …, then the guardian shall …". | FR-034 |
| FND-003 | medium | Several acceptance criteria have no parent requirement statement. No Behavior statement requires the behavior asserted by:<br>- FR-034-AC-9: isolation of concurrent runs.<br>- FR-034-AC-13: typed refusal for a missing or unusable helper.<br>- FR-034-AC-15: bounds on typed controls and rejection of unknown fields.<br>- FR-034-AC-16: lease exclusivity and child-only descriptor mapping.<br>- FR-034-AC-18: separating control and diagnostics from captures.<br>- FR-034-AC-21: a finite setup cap distinct from identity timeout.<br>- FR-034-AC-22: finite shutdown and observation waits.<br><br>Each criterion is therefore the only place its obligation is stated, and the requirement does not trace to it. Add one atomic Behavior statement per obligation, for example "If the guardian helper is missing or not executable, then the executor shall refuse setup with a typed reason before Dispatch". | FR-034, FR-034-AC-9, FR-034-AC-13, FR-034-AC-15, FR-034-AC-16, FR-034-AC-18, FR-034-AC-21, FR-034-AC-22 |
| FND-004 | medium | Behavior bullet 8 packs two conditional responses into one statement. It reads "When a run is cancelled or completed, the guardian shall confirm owned INIT teardown when INIT has been claimed and settle pinned startup-group cancellation before gate closure and monitor reaping when it has not". The embedded `when INIT has been claimed` and `when it has not` are states, which EARS writes with `While`. "Settle" is an unmeasured response. Split it into two statements: `While INIT is claimed, when a run is cancelled or completed, the guardian shall …`, and `While INIT is unclaimed, when …, the guardian shall kill the pinned startup group and confirm … before closing the gate writer or reaping the monitor`. | FR-034 |
| FND-005 | low | The same in-process actor is named three ways: "the bounded executor" (bullet 1), "the caller" (bullet 4) and "the executor" (bullets 5, 9, 10 and 11). "The caller" also means the original caller process whose death is the triggering condition. Use one subject name for the in-process executor and reserve "original caller" for the process whose death is the triggering condition. | FR-034 |

## Dispositions

Round 1 re-check of the fix-round candidate of PR #299, covering every original finding and regressions in the fix. The fixing commit is recorded in the private ticket marker.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | Behavior now reads "If guardian failure occurs, then the bounded executor shall return a typed refusal" and, separately, "If owned teardown is unconfirmed, then the bounded executor shall refuse any proof conclusion". FR-034-AC-11 reads "guardian failure always yields a typed refusal and never verified/falsified evidence, even after confirmed teardown and beside a valid success report". The statement and criterion now agree. |
| FND-002 | fixed | The Description opens "If the original caller dies, then the namespace guardian shall cancel its backend descendants": the unwanted-behavior pattern, with a subject that is alive to respond. |
| FND-003 | fixed | Every criterion I named now has an atomic parent statement:<br>- AC-9: "shall restrict cancellation to its own namespace descendants".<br>- AC-13: an explicit helper path and "If the helper is missing or unusable, then … refuse setup".<br>- AC-15: control bounds and rejection of unknown fields.<br>- AC-16: an exclusive lease endpoint, CLOEXEC child-only mapping and safe descriptor APIs.<br>- AC-18: separation of control and diagnostics.<br>- AC-21: a finite setup cap and its typed refusal.<br>- AC-22: bounded observation waits.<br><br>The new AC-25 and AC-26 also have parents. |
| FND-004 | fixed | The compound bullet is replaced by separate statements: "While Bootstrap is unclaimed, … retain its gate", "When Bootstrap is cancelled, … signal its pinned startup group" and "While INIT is claimed, … confirm its pidfd termination". The stage table gives each stage its own confirming observation. |
| FND-005 | fixed | Behavior now defines the bounded executor, guardian and monitor in a lead paragraph. Every Behavior statement uses "the bounded executor" for the in-process actor, and "original caller" names only the process whose death is the trigger. |
