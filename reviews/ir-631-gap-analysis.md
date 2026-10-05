---
id: SR-1608
title: "IR-631 gap analysis: FR-032 obligations and their claimed existing coverage"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen#290; spec/replay/functional/FR-032-routed-scalar-replay-binding.md, spec/replay/matrix/TC-047-routed-scalar-replay-binding.md, spec/spec.md; spec/replay/matrix/tests.md, spec/tests.md, tests/it/"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-047
    type: reviews
---

# SR-1608: IR-631 gap analysis (scoped)

## Summary

Ticket: IR-631. Reviewer: claude-opus-5-5, run d75eb678-590e-4bb0-b33c-ca7f9c5bde50. This was a planless, repository-driven audit scoped (per the planner request) to FR-032's new obligations and the existing coverage FR-032's Intent and Existing Coverage table claims. Plan completion was not assessed. Every claimed existing binder is real. The new criteria are untagged, and the documents say so accurately. The subsystem test matrix and the index do not record them.

## Verdict

**CONDITIONAL**: one medium finding. FR-032-AC-1 to AC-8 are untagged. The skill's literal rule would make that FAIL, but FR-032 (lines 183-186) and TC-047 declare those criteria Planned or Gated with no coverage claimed. The planner brief directs that a planned test is not to be demanded in a spec-only PR, so their untagged state is recorded under Coverage, not raised as a finding. Claimed coverage checked against the computed matrix:
- FR-022-AC-2, AC-10, AC-13 and AC-15 are tagged in tests/it/routed_generation.rs.
- FR-016-AC-1, AC-2, AC-5, AC-8, AC-9, AC-10 and AC-11 are tagged in skeleton_spine.rs, kani_witness_join.rs and src/replay/witness.rs.
- FR-016-AC-12 is untagged and, as FR-032 says, planned in the replay index.
- FR-029-AC-1, AC-10, AC-13 and AC-15 are tagged in tests/it/terminal_map.rs.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The replay matrix (TM-006, which calls its coverage tables "the authority") and the matrix index row (TM-008) do not list FR-032 or TC-047 or their Planned and Gated status, although the spec.md registry row now names both. The repository records planned criteria in those tables: IR-624 #289 and IR-629 #287 edited them. A reader of the matrix sees neither the requirement nor its gate. | spec/replay/matrix/tests.md:11, spec/tests.md:20, spec/spec.md:71 |

## Coverage

- Reconciliation: quire matrix (quire 0.36.1, engine 0.50.1); no run evidence read
- Plan completion: not assessed
- Criteria in scope: FR-032 has 8 criteria, all untagged and declared Planned/Gated; the 19 claimed existing criteria are 18 tagged and 1 untagged (FR-016-AC-12, declared planned)
- Untraced behaviors / stubs: 0 (the diff changes no source or test code)
- Semantic review: ran over FR-032's claimed-coverage rows, scoped

## Dispositions

Round 1 re-check of the fix-round candidate of PR #290. The fixing commit is recorded in the private ticket marker.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | TM-006 now has an FR-032 row (FR-032-AC-1 to AC-8, TC-047, planned under IR-631 and gated on QSL-641, no coverage claimed) and a TC-047 summary row. The TM-008 index row for Replay now lists FR-032 and its planned/gated status, consistent with the spec.md registry row. |
