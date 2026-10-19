---
id: SR-13308
title: "IR-915 PR 463 EARS analysis: excluded accounting-only CG positions"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen@fc288843; requirement statements and acceptance criteria edited in spec/core/functional/FR-039-contract-package-consumption.md, spec/routed/functional/FR-019-capability-settlement.md, spec/routed/functional/FR-022-routed-generation.md"
review_set: subset
---

## Summary

Ticket: IR-915. PR agent-ix/quire-contract-codegen#463. This analysis checked EARS
conformance of the edited statement bullets and the two new criteria. `make spec` (quire 0.36.2)
reports exactly one new warning, `ears:non-canonical-trigger`, at FR-019 line 163. The FR-039
bullet is ubiquitous `SHALL` but contains non-normative sentences. The FR-022 change is an
Inputs description and is not a statement.

## Examined criteria

- FR-019 statement, new bullet (examined): "On the admitted package path, CG shall check the envelope item count against bound driver items, not excluded positions or context roots. It shall settle only those bound items and retain their driver indexes across excluded positions. It shall not classify, negotiate or emit a disposition for an excluded position, even if its accounting record is `lowered`."
- FR-039 statement, new bullet (examined): "CG SHALL map an excluded accounting-only position ... Later bound driver items retain their original driver indexes; context-only roots advance neither driver indexes nor envelope item indexes. An excluded position ends the preceding bound item's contiguous context span and owns no context span. A missing, extra or mismatched excluded accounting record remains a whole-request correspondence refusal before any artifact is generated."
- FR-039-AC-9 (examined): event and state form, conformant.
- FR-019-AC-53 (examined): "Given ..." form, conformant.
- FR-022 Inputs `ContractRequestPosition` bullet (context_only): descriptive input, not a statement.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new FR-019 bullet opens with "On the admitted package path, CG shall ...". Quire flags this: `[ears:non-canonical-trigger] non-EARS trigger 'on'`. It is a state or feature condition, not an event. Fix: "While settling an admitted FR-330 package, CG shall ..." or "Where the admitted package path is used, ...". | spec/routed/functional/FR-019-capability-settlement.md:163 |
| FND-002 | low | Three sentences in the new FR-039 bullet are present-tense assertions with no `SHALL`, so they are not normative: "Later bound driver items retain their original driver indexes", "context-only roots advance neither ...", and "An excluded position ends the preceding bound item's contiguous context span and owns no context span". The span rule in particular is a new obligation (see SR-13307 FND-003). Fix: write each as "CG SHALL ..." or fold it into the leading `SHALL` clause. | spec/core/functional/FR-039-contract-package-consumption.md:91-96 |

## Verdict

**PASS with findings.** Both new criteria are well formed. Two low wording issues remain: the
FR-019 trigger keyword, and the FR-039 non-normative sentences.
