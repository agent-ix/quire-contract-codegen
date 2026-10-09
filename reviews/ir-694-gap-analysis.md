---
id: SR-3204
title: "IR-694 gap analysis: FR-034-AC-94 against the computed matrix and its Test allocation"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen PR #327, branch spec/ir694-queued-claim-cleanup (frozen head named in the Linear marker) compared with main; quire matrix --format tsv on both trees; spec/kani/functional/FR-034-caller-death-ownership.md AC-94 and its CODE-duty sentence; spec/kani/matrix/TC-049-caller-death-ownership.md AC-94 allocation row and procedure; context: TC-049 AC-75 allocation row (IR-655 SPEC-before-fixture-CODE route)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: references
---

# SR-3204: IR-694 gap analysis

## Summary

Ticket: IR-694. Plan completion: not assessed. This is a planless gap analysis.

- The computed Test Matrix grows from 615 to 616 records, and all prior records are unchanged.
- FR-034-AC-94 computes `untagged`, consistent with its PLANNED/UNRUN status.
- No test claims AC-94. No source or test file changed, so there is no stub or coverage inflation
  to find.
- The CODE-duty sentence accurately says that the claimed path and the bounded same-cursor
  integration do not exist yet: "Actual fixed pending storage/custody and bounded same-cursor
  integration remain PLANNED/UNRUN CODE duties ... this text supplies no capacity or runtime
  proof".

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The Test half of AC-94 needs a schedule that ordinary seams cannot force: C reads the queued claim only after I's cancellation, with the committed Failure queued behind it. The TC disclaims any synthetic actor, new fixture hook or sampling pause, and says missing construction "leaves this Test owed". Unlike the AC-75 allocation row, which routes unavailable predicates through IR-655 SPEC-before-fixture-CODE, neither the AC-94 allocation row nor the procedure names an owner or route for that construction. IR-639 code activation, which IR-694 blocks, could therefore land with only the Analysis half and no tracked owner for the Test. | spec/kani/matrix/TC-049-caller-death-ownership.md:145; spec/kani/matrix/TC-049-caller-death-ownership.md:1144-1145 |

## Verdict

**No coverage inflation and no code credit claimed. One low finding:** name the allocation route
for the owed Test, for example the existing IR-655 SPEC-before-fixture-CODE route, as the AC-75
row does.
