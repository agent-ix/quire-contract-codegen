---
id: "SR-13223"
title: "CG PR 455 integrity: ids, trace rows and TC-030 consistency"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@cfa6dcf6bf52f886e780e8234bffa249b8bd44b9; spec/routed/functional/FR-019-capability-settlement.md, spec/routed/matrix/TC-030-capability-settlement.md, spec/routed/matrix/tests.md, spec/core/functional/FR-039-contract-package-consumption.md, spec/core/matrix/tests.md, spec/tests.md (diff origin/main...HEAD)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/TC-030
    type: reviews
---

# SR-13223: CG PR 455 integrity

## Summary

Ticket: IR-892.

These checks pass:

- **Ids.** AC-48 to AC-50 are next-free. FR-019's highest criterion on origin/main is AC-47,
  and no open CG PR (#448, #445, #413, #397, #381) uses FR-019-AC-48 or later.
- **Trace rows.** The routed `tests.md` gains one row, and TC-030's Traces To lists all three
  new ids. The core `tests.md` FR-039-AC-5 row and the `spec/tests.md` Routed row are updated
  consistently.
- **FR-039.** The FR-039-AC-5 statement (line 188) is unchanged. Its Analysis still says the
  deviation remains until the code PR lands.

## Verdict

PASS WITH ONE LOW FINDING. TC-030's Expected Results section was not extended for the new
criteria.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The TC-030 Test Procedure gains an AC-48 to AC-50 paragraph, but `## Expected Results` has no entry for those criteria. The expected outcomes exist only as inline "Assert ..." clauses in the procedure. Every other criterion group, for example AC-45 and AC-46, has an Expected Results entry | spec/routed/matrix/TC-030-capability-settlement.md:37-53; spec/routed/matrix/TC-030-capability-settlement.md:235-263 |
