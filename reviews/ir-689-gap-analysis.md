---
id: SR-3104
title: "IR-689 gap analysis: FR-034 AC-78..AC-82 against the computed matrix and the published guardian review source"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen branch spec/ir689-stage2-observation (frozen head named in the Linear marker) compared with main; quire matrix --format tsv on both trees; spec/kani/functional/FR-034-caller-death-ownership.md AC-78..AC-82 and the observation transport section; spec/kani/matrix/TC-049-caller-death-ownership.md; spec/kani/matrix/tests.md; newer published guardian review-source backup ref (head commit 'Retain producer clock failure with borrowed outer setup custody'): Cargo.toml, README, src/kani/run/fixture.rs, fixture_execution.rs, owned.rs, and a repository-wide search for guardian-test-support uses"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: references
---

## Summary

Ticket: IR-689. Plan completion: not assessed. This is a planless gap analysis of a Markdown-only change.

- **Matrix.** The computed Test Matrix grows from 615 to 620 criterion records, and every prior record is byte-identical. AC-78..AC-81 compute `untagged`. AC-82, which is Analysis-only, computes `method-without-symbol`, the same as the existing Analysis-only rows. No test is tagged to the new criteria, and none claims them.
- **Status.** tests.md marks the FR-034 row "AC-1 through AC-82" and the TC-049 index row as Planned, and lists IR-689's methods correctly: AC-78/79/81 Test/Analysis, AC-80 Test, AC-82 Analysis.

The change is spec-only, so there is no code to find stubs in. The gap check therefore tests the spec's statements about current source against the published guardian review source:

- **No existing O-to-C observation surface.** The public fixture result (fixture.rs GuardianFixtureObservation) holds only C-side lease, publication, worker, reporter and marker facts. The fixture runs through owned.rs start_sequence and RunOwner (fixture_execution.rs run), so no O operation facts reach it. This matches TC-049:189-190 and justifies a new surface.
- **No separate I Completed carrier.** The fixture result has no authenticated I Completed field. AC-80's carrier is new, as stated.
- **Feature wiring.** `guardian-test-support` is an empty, off-by-default feature. It is used only as cfg gates on C-side fixture items: the fixture module, C-side read accessors (namespace-owner INIT claim and gate, reporter transport, stage publication) and the caller-fixture binary. No L/O helper-side code is gated yet, so the O writer, the binding decoder and the record types are all still to be built, as the PLANNED/UNRUN status says.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

**Clean for gap analysis.** The matrix delta is exactly the five new criteria, with additions only and no status inflation. Every statement in the slice about current source matches the published guardian review source. The substantive defects are recorded in the companion spec-review artifacts SR-3100..SR-3103.
