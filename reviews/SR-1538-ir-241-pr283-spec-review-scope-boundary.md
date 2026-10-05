---
id: SR-1538
title: "Scope-boundary review of quire-contract-codegen PR #283"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen@1f3025ea99c3fe1fa804ed43fce116e339f9c96d; FR-028 (Planned paragraph, Open Questions, Checklist), FR-015 IR-264 unsupported-shape table, FR-028-AC-17, FR-028-AC-23, AD-001 Risks bullet; ADR-003 Q1/Q2 (context)"
review_set: subset
---
# Scope-boundary review of PR #283

## Summary

Ticket: IR-241 (also IR-264). This review checks three things. First, that the PR stays inside
IR-241 and IR-264 (record, tuple and option equality only, with no scalar families). Second, that
it is the decision ADR-003 accepted. Third, that it is consistent with the owner's "complete Kani"
bar, relayed by the planner and not confirmed first-hand: every family row gets a real harness, and
interim refusals are checklist items, not end states.

Scope holds. The only prose edit outside FR-015, FR-018 and FR-028 is the AD-001 Risks bullet,
which now matches FR-028's Rationale. FR-028-AC-21 (memory ceiling) and AC-23 (tool versions in
the evidence) are covered by IR-264's acceptance criteria ("complete within the Kani run's resource
limits"; "record the Kani, Rust and solver versions, the unwind bounds and every option"). AC-23
applies to every run and not only shadow runs, which follows from IR-264's wording. The design is
ADR-003 Q1 (shadow plus refinement obligation, never reported alone) and Q3 (no stubs). The
Checklist lists all five owner items as not end states: leaf families, collections, `convert<T>`
and other operands, a function-application shadow, and a recursive depth bound. Two findings.

## Scope examined

- FR-028 Planned paragraph, Open Questions and Checklist (examined).
- FR-015 unsupported-shape table rows (examined).
- FR-028-AC-17 (examined), FR-028-AC-23 (examined).
- AD-001 Risks bullet (examined).
- ADR-003 Q1 to Q3 (context_only). IR-241 and IR-264 descriptions (context_only, untrusted, re-measured where used).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The FR-015 unsupported-shape table says "Every row is an interim refusal ... none is the end state", but FR-028's Checklist lists only some of the rows. Four are missing. (1) `domain_not_representable_in_i64`: this is a limit of the shadow's representation, since production `Integer` is unbounded. (2) `ShadowShapeOverBudget`. (3) The reference, model and relation blocker (quire-spec-language#120). (4) An unbounded integer leaf `requires_bound`, which should either be a Checklist item or be named an ADR-003 Q2 end state ("an item with no finite bound gets no harness"). Under the owner's bar, every interim refusal must be a checklist item. | FR-015 IR-264 unsupported-shape table; FR-028 Checklist |
| FND-002 | medium | `shadow_proved_refinement_sampled` and `shadow_proved_refinement_not_run` are reportable results for a verified family, but nothing marks them as not end states, and nothing keeps a family row such as IR-264 open while its strength is one of them. ADR-003 Q1 requires a refinement obligation "showing that the production code agrees with the shadow on the bounded domain", and a sampled run or a run that never happened shows neither. FR-028 should state that only `production_proved`, and, if the owner accepts it (Q1), `shadow_proved_refinement_exhaustive`, close a family row. The other two strengths should be Checklist items. | FR-028-AC-17; ADR-003 Q1; FR-028 Open Question 1 |

## Verdict

Within scope, and faithful to ADR-003. Q1 is the right question to put to the owner, and it
should be put more sharply. Under ADR-003 Q1 as accepted, `shadow_proved_refinement_exhaustive` is
an honest end state for composite equality: it is a Kani proof of an independent model plus an
exhaustive agreement check over the same bounded domain. It is not a Kani proof of the production
code. The owner needs to answer three things:

- whether "complete Kani" for IR-264 means ADR-003 Q1's shadow plus refinement, or a Kani proof
  of the production oracle, which no probe has shown tractable;
- whether a family row may close on `exhaustive` when realistic models have ranges too wide to
  enumerate, so that their items will report `sampled`;
- whether `sampled` is ever acceptable.

These are in addition to whether `requires_bound` on an unbounded `Integer` leaf is an accepted end
state.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | FR-029 calls `NonProductionProof` interim (a verified shadow result gets no terminal value, and the item has no terminal record). The FR-028 Checklist lists only the falsified-shadow replay path, not the verified-shadow terminal value. Under the owner's bar every interim refusal is a Checklist item, so the Checklist should add "a terminal value for a verified shadow result, pending QSL (FR-029 Open Questions)". Until then a composite equality item never reaches QSL with a terminal record. | FR-028 Checklist; FR-029-AC-17; FR-029 Open Questions |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed 3e0bc7c | The Checklist now lists `domain_not_representable_in_i64`, `ShadowShapeOverBudget` and the QSL#120 blocker as not end states, and routes `requires_bound` on an unbounded integer to owner question (d). |
| FND-002 | fixed 3e0bc7c | The Checklist lists sampled, not_run and inconclusive as interim strengths. Open Question 1 is rewritten as owner questions (a) to (d). No AC says a strength closes a row: FR-015-AC-76 only withholds closure for an unbacked cover. |
| FND-003 | fixed 5833419 | The FR-028 Checklist adds 'A terminal value for a verified shadow result: FR-029-AC-17 interim-refuses it (`NonProductionProof`, no terminal value), which is not an end state and waits on QSL's answer.' |
