---
id: "SR-702"
title: "CG PR 210 spec review: FR-017 exported-report amendments, interface-001, AD-001"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@363148d4e18e30c278b84c106a384503ed0ed47c; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/tests.md, spec/core/functional/interface-001-codegen-api.md, spec/assurance/AD-001-codegen-architecture.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: references
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: references
  - target: ix://agent-ix/quire-contract-codegen/AD-001
    type: references
---

# SR-702: CG PR 210 spec review

## Summary

Ticket: IR-277 (also IR-288, IR-463). PR: agent-ix/quire-contract-codegen#210 at 363148d.
Methods: spec-review with integrity (ids, matrix consistency), EARS reading of the new FR-017
behaviour bullets, and consistency against the code and AD-003/AD-004. The matrix and trace
defects are recorded once, in SR-701 (gap analysis), and not repeated here.

Checked and clean:

- New ids FR-017-AC-18, AC-19 and AC-20 are fresh; no existing id changed meaning, none was
  removed. FR-017-AC-4, AC-12 and AC-13 were reworded from the prose transcript to the report
  without narrowing what they require.
- The new behaviour bullets (export flags and stale-report removal; report-only reading with
  typed refusal; per-check view with one wire shape; SUCCESS-check count; console playback as a
  payload only) are EARS ubiquitous statements with a named subject, and each matches the code.
- interface-001 `launch_evidence` / `classify_kani_run` entries and the slice's `evidence` and
  `outcome_source` fields match the new signatures and refusal causes. No terminal-map entries.
- AD-001 steps 3-4 and the CG-Kani boundary row now name the exported report. AD-001 Risks
  (:210, "Kani publishes no machine-readable verdict") is stale too, but AD-004 step 7
  explicitly defers that edit, so it is not a finding here.
- AD-003 (c)'s recommendation ("`KaniCheckResult` gets one shape, serialize-only") is what FR-017
  and the code now state; AD-003 needs no edit from this PR.
- No pins, SHAs, digests or Kani version records were added to spec text; the report schema
  version is named only as "the one schema version the module reads".
- `make spec` exits 0 with the 3 baseline warnings (FR-017 AC-17's `process` verb, now at line
  155 because lines were added above it, and FR-014:278 twice). The PR adds none.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-017-AC-14 still says the bounded stream keeps "the verdict lines included". After this PR the verdict is not in the stream at all (FR-017 now says it comes from the report, and the `CAPTURE_LIMIT` doc says so); the tail matters only for the playback. Reword to the playback | spec/kani/functional/FR-017-kani-execution-evidence.md:152 |
| FND-002 | low | interface-001's `kani_obligation_execution_slice` was updated in `evidence` and `outcome_source` but its `refusals` field still lists only the crate-mismatch and tool refusals, omitting the new `KaniExecutionRefusal::Report`, and `outcomes` still says "without a readable, fully satisfied cover summary" (and omits `vacuous_proof`). The slice now contradicts its own `outcome_source` line | spec/core/functional/interface-001-codegen-api.md:268-269 |

## Verdict

The FR-017 amendments are sound and complete for what the code does; two low wording defects
remain, best fixed with SR-701's matrix items in one spec commit.
