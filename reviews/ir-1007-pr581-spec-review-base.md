---
id: SR-581002
title: "Base spec review of IR-1007 CG PR 581 (FR-021 delivery status re-point)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@ee51b2590f4e2909fc121fbeda082348a64fd37d; spec/assurance/AD-004-cg-crate-layout.md, spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md"
---

## Summary

Ticket: IR-1007. PR: quire-contract-codegen#581, frozen head ee51b2590f4e2909fc121fbeda082348a64fd37d,
base f353f91224e09ee6aabc5929123139dfb3d6e671. Reviewer model: Claude Opus 5.5
(`claude-opus-5-5`). Method: spec-review/base. Every changed acceptance criterion was compared
word for word against base. Only the status suffix changed. No criterion body, closing
condition or verification method was narrowed. The QSL delivery facts are accurate. One medium
defect remains: every remaining PLANNED gate names IR-595, but Linear marks IR-595 Done
(completed 2026-10-25T17:33:48Z), and no open ticket owns the CG FR-485 consumer.

## Examined scope

- FR-021-AC-2: examined. The body is unchanged. The suffix goes from "PLANNED pending the
  IR-1007 facade and IR-595 CG consumer, with an independently assembled direct application"
  to "PLANNED pending the IR-595 CG consumer and independently assembled direct application;
  the QSL FR-485 facade is delivered." It is not narrowed.
- FR-021-AC-4, AC-5, AC-9, AC-17: examined. Each body is unchanged. Each suffix drops IR-1007
  and adds "the QSL FR-485 facade is delivered". AC-4 keeps "the existing dangling-reference
  reachability gap remains open". None is narrowed.
- FR-021-AC-32: examined. The body is unchanged, and "Close only after inspecting CG imports,
  emitted source, dependencies and the executed corpus" is kept. It is not narrowed.
- FR-021-AC-35, AC-36, AC-37: examined. Each body is unchanged and only the suffix moved.
  None is narrowed.
- FR-021 Description (lines 43-48), Behavior bullet (line 81), Dependencies "QSL owner path"
  (lines 426-431) and Status (lines 460-468): examined. The new text says CG "still must
  implement and verify the IR-595 consumer against its leaf root-site behavior", which
  keeps the verification obligation.
- AD-004 External evaluation seam (lines 361-375): examined. The precondition is now "the
  IR-595 CG consumer must land and pass its acceptance checks".
- TC-031 Objective, steps 6, 7, 12, 12b and Expected Results: examined. The corpus stays
  Planned.
- QSL FR-485 and FR-262 delivery: context only (verified in SR-581001).
- IR-595 tracker state: examined via `linear api`. The state is Done, completedAt
  2026-10-25T17:33:48.747Z. That is when PR #580, which was spec-only, merged. A Linear
  search of open IR issues for FR-485 or checked-closure work returns only IR-1007 itself.

## Verdict

CHANGES REQUESTED (one medium). The delivery re-point is accurate, and no AC is narrowed. The
concrete oracle and corpus correctly remain Planned. But the gate those Planned rows name is a
ticket already closed while the consumer code has not landed. This is the same defect class
FR-021's own Status paragraph calls out for IR-853 ("IR-853 is Done; its obsolete blocked-by
text gives no acceptance credit").

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | After this PR, every remaining PLANNED gate in FR-021 (AC-2, AC-4, AC-5, AC-9, AC-17, AC-32, AC-35 to AC-37, Description, Status), AD-004 and TC-031 names only IR-595 as the owner of the CG FR-485 consumer. Linear marks IR-595 Done (completed 2026-10-25T17:33:48Z, after the spec-only #580), but CG main has no `acquire_checked_closure`, `lookup_callable` or `begin_call` call, and `src/core/profile.rs:77` still uses `quire_contract_runtime::exact`. No open ticket owns the consumer. The Planned rows therefore point at a closed owner, and nothing in ix-board's ready queue will pick up the work. Recommended fix: reopen IR-595 (or file a new consumer-code ticket blocked by nothing) and make the gates name that open ticket. If a new ticket is used, replace "IR-595" in the gate text. | spec/oracle/functional/FR-021-function-application-oracles.md:47; spec/oracle/functional/FR-021-function-application-oracles.md:341-376; spec/oracle/functional/FR-021-function-application-oracles.md:464; spec/assurance/AD-004-cg-crate-layout.md:372; spec/oracle/matrix/TC-031-function-application-oracles.md:20 |

## Dispositions

Round 1, checked against fix-round head 7e5673628f3a547bd6d408d81d1a340ac1ac0fff by Claude Opus 5.5 (`claude-opus-5-5`).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Measured via `linear api` on 2026-10-25: IR-595 state is In Progress and completedAt is null. Comment 867eaebf-444b-4dae-94b9-b546c6f50dec (2026-10-25T22:08:19Z) reopens it as the owner of the remaining CG code/corpus cutover, and says #581 does not satisfy it. The spec gate text naming IR-595 (FR-021 AC-2/4/5/9/17/32/35-37, Status; AD-004:372; TC-031:20) is unchanged, and it now names an open owner. CG code still has no consumer: the fix delta touches no `src/`, so the gate stays truthful. |
