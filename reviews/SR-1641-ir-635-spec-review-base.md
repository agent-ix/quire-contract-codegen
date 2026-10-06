---
id: SR-1641
title: "IR-635 spec review (base checklist): composite parity replay binding"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen#298; spec/kani/functional/FR-025-generated-subject-abi.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/replay/functional/FR-033-composite-parity-replay-binding.md, spec/replay/matrix/TC-048-composite-parity-replay-binding.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-033
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-048
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-025
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: reviews
---

# SR-1641: IR-635 spec review, base checklist

## Summary

Ticket: IR-635. PR: quire-contract-codegen#298 (spec only; the reviewed head is recorded in the private ticket marker). Reviewer: claude-opus-5-5, session 2f7447c0-1934-4031-8988-fab17e2f41b7. This review applied the base checklist to FR-033, TC-048, the FR-025, FR-028 and FR-029 amendments and the registry row in spec/spec.md. It re-measured upstream citations against QSL main, QSL's open PR #645 (the QSL-640 draft), QSpec main and QSL's ADR-013, ADR-014 and ADR-021. `quire validate --scope .` over the six changed files exits 0 (quire 0.36.1, engine 0.50.1). Two low findings.

## Method

I read every changed file in full and the context it amends: FR-015-AC-70/71, FR-028-AC-14 to AC-24, FR-029's tables and AC-1 to AC-18, AD-002 R-6/R-7, TC-040 and the kani and replay matrix indexes. Upstream, I read QSL FR-070 and FR-358 at the PR #645 head, which still specifies the selected-Boolean-function route with `Verified { refinement_exhausted: bool }`. I also read QSpec FR-181 (the typed canonical form), QSpec FR-322 (the checked-package artifact), QSL ADR-013 O-09, QSL ADR-014 B-4 and the QSL ADR-021 TX table. The transcript escaping in FR-033 (`%25`, `%3B`, `%3C`, `%3E`) matches QSL FR-070 at the PR #645 head. Findings about the matrix, criterion form, boundaries, failure domains and relationship edges are in SR-1642 to SR-1646.

## Verdict

**PASS with two low findings.** The following parts were clean:
- FR-033 consistently labels the work PLANNED and GATED on actual QSL-640 delivery and claims no executable coverage. TC-048 states that scenario prose is not coverage.
- The ruled shape is carried faithfully:
  - node-selected claim in the original recompiled package;
  - CG-minted `ObligationIdentity` (AD-002 R-6, QSL ADR-013 O-09);
  - a same-artifact native observation for falsified replay only;
  - divergence `Failed`/`CgDefect` and agreement parity-agreement `Inconclusive`, never `Refuted`;
  - no native-observation field on verified settlement, with `not_run` as explicit absence;
  - closed `Exhausted`/`NotExhausted`/`CeilingReached`/`Disagreed` projection that keeps every FR-028-AC-17 strength;
  - the priority order disagreement, ceiling, zero, exhausted and covered, then `Tested`;
  - QSL-derived bound keys, with an empty list covering only key-free domains;
  - an encoded-byte guard distinct from FR-017 capture;
  - iterative decode, clone, equality, Debug and drop with no depth cap.
- FR-025-AC-7's refusal of a composite machine argument is kept, and FR-033 states it does not expand the primitive ABI.
- No vendoring, compatibility layer, local QSL type, new pin or digest is introduced.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-033 says QSL ADR-021 "TX2 names the separate bounded-shadow route". ADR-021 TX-2 is "Symmetry transfer is a precondition" and says nothing about a bounded-shadow route. TX-3 is the rule on tightened harness bounds. Delete the TX2 clause or cite the clause that actually names the route. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:134 |
| FND-002 | low | FR-033 calls FR-358's selected-Boolean-function route "published". FR-358 exists only on QSL's open, unmerged PR #645 and is absent from QSL main. The brief requires the pending upstream shape (FR-322/FR-358 node parity, FR-070 canonical values, closed refinement evidence and parity-agreement cause) to carry an explicit UNVERIFIED SOURCE label. FR-033 and FR-029 call these items "proposed" or "planned upstream semantics" but never use that label. Say "proposed (unmerged)" and add the UNVERIFIED SOURCE label to the list of upstream items. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:47, spec/replay/functional/FR-033-composite-parity-replay-binding.md:49, spec/kani/functional/FR-029-run-outcome-terminal-record.md:160 |

## Dispositions

Round 1 re-check of the fix-round candidate of PR #298, covering every original finding and regressions in the fix. The fixing commit is recorded in the private ticket marker.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | FR-033 no longer mentions TX2. The verified-shadow bullet now ends "ADR-021 TX-3 owns tightened-bound coverage.", and FR-033-AC-8 keeps "under ADR-021 TX3". This matches QSL ADR-021, where TX-2 is symmetry transfer and TX-3 governs tightened harness bounds. |
| FND-002 | fixed | FR-033 now opens its gate paragraph with "UNVERIFIED SOURCE:" naming each pending upstream item: FR-070 canonical values, QSL-640's node-selected parity claim and FR-358 settlement, closed refinement evidence and the parity-agreement record/cause. It calls the FR-358 route "proposed (unmerged)". FR-029 carries the same label before its projection table, and the kani and replay matrix rows and spec/tests.md repeat it. QSL-640 remains the code gate, not a spec-merge dependency. |
