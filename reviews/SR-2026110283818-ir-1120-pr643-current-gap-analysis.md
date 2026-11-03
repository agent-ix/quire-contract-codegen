---
id: SR-2026110283818
title: "IR-1120 PR 643 current-head gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #643 head 92262c6b; FR-045-AC-4 through AC-7, TC-059, src/routed/smt/refinement.rs and refinement_source_tests.rs"
review_set: subset
---

## Summary

Computed matrix and source inspection confirm AC-4, AC-6 and AC-7 have tagged controls; AC-5 is untagged and remains explicitly partial. This PR does not claim live QSL FR-314 readback or a production FR-315 driver.

## Verdict

**PASS for the stated partial slice.** AC-5 and the FR-315 driver/refusal work remain open; QSL public adapter IR-1287 is required for live readback. No new coverage inflation was found in the changed tests.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

`quire matrix --scope . --format tsv` at this head reports FR-045-AC-4 tagged by the source-authored seven-row and zero-step tests, AC-6 tagged by shape/association controls, AC-7 tagged by missing-field and typed non-proof controls, and AC-5 untagged. The latter is honest: `spec/routed/functional/FR-045-hyper-smt-induction-map.md:206-228` labels the handoff and live proof readback partial. The code does not invoke a private QSL certificate API. The serial form is checked through a manual certificate construction; live FR-314 reader acceptance is not asserted.

The seven-row source test checks all distinct query bytes and all seven named-row movements (`src/routed/smt/refinement_source_tests.rs:285-319`), then mutates row bindings and query bytes (`:329-366`). The Base-only zero-step source test is at `:370-382`. Shape, association, proof provenance and no-handoff assertions are in `src/routed/smt/refinement.rs:447-680`.

Plan completion: not assessed.
