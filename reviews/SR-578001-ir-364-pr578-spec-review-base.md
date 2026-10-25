---
id: SR-578001
title: "IR-364 PR #578 spec-review/base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR #578 head 33116e82f60b1f9e0affff3f6bad89530df8859a on base b60b18ef; spec/strategy/functional/FR-002-tristate-proptest.md, spec/strategy/matrix/TC-004-proptest-strategies.md; IR-364"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-004
    type: reviews
---

## Summary

Base checklist review of draft PR #578 (FR-002 OQ-2, non-completed V2 oracle outcome) at frozen
head 33116e82, diff against main b60b18ef. Ticket: IR-364. Spec-only; no code, Kani or tests run.
`quire validate --scope .` over both changed files passes (module warnings only).

Checked against merged FR-002, FR-011, FR-012 and FR-004 on CG main, the generated V2 clause oracle
in `src/oracle/clause.rs`, `HarnessErrorCode` in `src/strategy/harness.rs`, quire-exact `Outcome`
and quire-contract-runtime `VerdictKind`, `ClauseOutcome`, `FailureKind` and `CampaignCounts` on
their `main`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-16 and the Behavior statement close the non-completed set to the three `Outcome<bool>` stop variants, but the merged generated V2 clause oracle returns `Result<rt::Outcome<bool>, ClauseStop>`. Its `Err` channel carries `ClauseStop::Oracle(OracleStop)` and `ClauseStop::InvalidConstant`, which are passed through unchanged. FR-002 does not say what the harness reports for that channel. Main's wording ("an outcome that does not complete (stopped or undefined)") could be read to cover it; the new closed list cannot | spec/strategy/functional/FR-002-tristate-proptest.md:174, spec/strategy/functional/FR-002-tristate-proptest.md:88-96, src/oracle/clause.rs:263, src/oracle/clause.rs:497-505 |
| FND-002 | medium | "The supplied runtime report ... Its `attempted` count continues to describe recorded verdicts and explicit discards only" names a field the runtime report does not have. RT `CampaignCounts` exposes accepted, rejected, failed, discarded and `total()`. `attempted` is FR-011's generated-summary field, and FR-012 defines it as every oracle evaluation the runner performed, including shrink replays (FR-012-AC-4). A stopped case did evaluate an oracle, so the new sentence contradicts FR-012's meaning if the V2 campaign shares that summary | spec/strategy/functional/FR-002-tristate-proptest.md:94-96, spec/strategy/functional/FR-012-constraint-preserving-shrinking.md:46, spec/strategy/functional/FR-011-numeric-harness-campaigns.md:70 |
| FND-003 | low | "pass artifact" (Behavior, AC-16, TC-004) is defined nowhere in the spec tree, so TC-004's "require no ... pass artifact" has no observable to check | spec/strategy/functional/FR-002-tristate-proptest.md:92, spec/strategy/functional/FR-002-tristate-proptest.md:174, spec/strategy/matrix/TC-004-proptest-strategies.md:43 |
| FND-004 | low | TC-004 does not check two things AC-16 requires of `NonCompleted`: the precondition or postcondition selection it carries, and that the retained cause equals the seeded cause value, not only its variant. It also gives no seeding method for `Refused` or `Incomplete`; only an undefined integer operation is named | spec/strategy/matrix/TC-004-proptest-strategies.md:39-44, spec/strategy/functional/FR-002-tristate-proptest.md:174 |
| FND-005 | low | Outputs and AC-16 say `NonCompleted` carries the selection's "accepted execution point". The Behavior lines and AC-19 say "rendered point", and TC-004 compares a runtime string. So it is unclear whether the variant holds the typed `ExecutionPoint` or the `<variant>:<len>:<AnchorName>` rendering | spec/strategy/functional/FR-002-tristate-proptest.md:41-45, spec/strategy/functional/FR-002-tristate-proptest.md:150-153, spec/strategy/matrix/TC-004-proptest-strategies.md:44-46 |

## Verdict

Base: NOT MERGEABLE until FND-001 and FND-002 are fixed; the low findings are recommended in the
same round.

Confirmed as intended:

- AC-17 and AC-18 are byte-identical to main.
- AC-16 is a strict strengthening of main. It still says the case keeps its cause and is not
  counted as a pass, and it adds the typed variant, the selection, the point, no verdict, no pass
  artifact and unchanged counters. OQ-2's resolution justifies the change.
- `NonCompleted` cannot be read as a pass. The Outputs bullet says it is not a runtime verdict.
  The Behavior bullet forbids a verdict, a pass count, and conversion to rejection, failure,
  discard or exhaustion, and the campaign ends non-success.
- RT `VerdictKind` stays at its three values, and no RT change or compat layer is proposed. Because
  AC-16 forbids building any verdict, RT's existing `ClauseOutcome::Undefined` and
  `FailureKind::Undefined` cannot be used for these stops. Those RT categories would also lose
  `Refused` and `Incomplete` and the typed cause, so `NonCompleted` duplicates nothing.
- `V2HarnessCaseResult` does not exist in code today, which is consistent with Planned.
  `HarnessErrorCode` is a generation-time refusal enum and does not overlap.
- Point assignment matches merged FR-002-AC-19 and the FR-011 single-source, occurrence-keyed
  `ClauseBinding.execution_point` rules: the point is the one of the oracle that did not complete.
- The PLANNED status is truthful: QSL-1082 is in Backlog and IR-1013 is In Progress. The new lines
  reuse the file's "Planned (IR-815)" tag, although IR-815 is a completed spec ticket.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | The new Outputs rule for a runtime report already at its count limit (the limit signal is kept, the V2 summary marks the exact attempted count unavailable, and one non-completed case is still recorded) has no Behavior statement, no AC and no TC-004 step. TC-004 seeds only nonzero counters below the limit | spec/strategy/functional/FR-002-tristate-proptest.md:55-59, spec/strategy/matrix/TC-004-proptest-strategies.md:55-60 |

## Dispositions

Round 1, checked against the repository at head 6c4f1f1623d94d49744d14bdc2129cbcd5435bd4 (merge-base main b60b18ef). FR-002-AC-17, AC-18 and AC-19 rows are byte-identical to main; only AC-16 changed, and it is not narrowed.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AC-16 and two Behavior If-then statements now cover `Ok(Outcome::Undefined/Refused/Incomplete)` and `Err(ClauseStop::Oracle(OracleStop))` / `Err(ClauseStop::InvalidConstant)`, each returning `NonCompleted` with the exact typed stop. |
| FND-002 | fixed | `attempted` and `non_completed` now live on the CG-owned `V2HarnessCampaignSummary`. `attempted` = RT `counts().total()` + 1, so the stopped evaluation counts as an attempt, consistent with FR-012-AC-4. RT accepted, rejected, failed and discarded counters stay unchanged, and RT exposes `counts()` and `total()` on main. |
| FND-003 | fixed | "pass artifact" is removed from FR-002 and TC-004 and replaced by "no successful case or campaign conclusion/result". |
| FND-004 | fixed | TC-004 now compares the checked clause id and authentic `claim` occurrence with the selected tuple. It injects exact, distinguishable values of all five stops at each oracle position through the generated oracle boundary and compares the variant and cause with the injected value. |
| FND-005 | fixed | `NonCompleted` carries the typed `ExecutionPoint`; its runtime string is derived through FR-011 rendering, and TC-004 compares both. |

## Coverage

```yaml
scope:
- id: FR-002-AC-16
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "A seeded V2 case on which the precondition or postcondition oracle returns an `Outcome<bool>` that does not complete (`Undefined`, `Refused`, or `Incomplete`) terminates as CG-owned `V2HarnessCaseResult::NonCompleted` retaining that oracle's original typed variant and cause, its precondition or postcondition selection, and that selection's accepted execution point."
- id: FR-002-AC-17
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "A seeded V2 case on which the precondition oracle returns an outcome that does not complete does not invoke the subject."
- id: FR-002-AC-18
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "A seeded V2 case on which the precondition oracle returns an outcome that does not complete does not evaluate the postcondition oracle."
- id: FR-002-AC-19
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "a non-completed case carries the point of the oracle that did not complete and retains its cause without counting as a pass."
- id: FR-002
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "if an embedded V2 clause oracle returns an outcome that does not complete, then the V2 harness shall return `V2HarnessCaseResult::NonCompleted` ... The supplied runtime report shall retain its prior verdict counters. Its `attempted` count continues to describe recorded verdicts and explicit discards only"
- id: TC-004
  path: spec/strategy/matrix/TC-004-proptest-strategies.md
  role: examined
  excerpt: "Seed each stop variant at each oracle position. Instrument runtime verdict construction and recording, and start with nonzero report counters; require no verdict call or pass artifact for a stopped case, unchanged prior verdict counters, and a non-success campaign result distinct from rejection, failure, discard, and exhaustion."
- id: FR-011
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: context_only
  excerpt: "A generated summary type with the existing `attempted`, `accepted`, `rejected`, `failed`, and `discarded` fields, plus `discard_rate()` and `rejection_rate()`."
- id: FR-012-AC-4
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: context_only
  excerpt: "After a shrinking campaign, `attempted` equals the number of oracle evaluations the runner performed, including shrink replays."
- id: FR-004
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: context_only
  excerpt: "The accepted QSL `ClauseBinding.execution_point` for that exact package, clause and occurrence is exposed through the planned Contract IR FR-058 typed accessor under FR-011."
```
