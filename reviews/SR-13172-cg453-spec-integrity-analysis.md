---
id: "SR-13172"
title: "CG PR 453 integrity analysis: AC-45 atomicity, the conflict note and FR-331 framing"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@bf69bb4f23a976e8a858ac16460f1c740ab491ae; spec/routed/functional/FR-019-capability-settlement.md, spec/routed/matrix/TC-030-capability-settlement.md, spec/routed/matrix/tests.md (diff origin/main...HEAD); src/routed/capability.rs:940-950 read for consistency"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-030
    type: reviews
---

# SR-13172: CG PR 453 integrity analysis

## Summary

Ticket: IR-887. PR: agent-ix/quire-contract-codegen#453 at bf69bb4.

Clean:

- **Ids.** FR-019-AC-45 is the next free id: AC-44 was the highest on main, and no AC-45 exists
  anywhere in the repo.
- **Matrix.** AC-45 is added to the 🚧 Planned FR-019 row of `tests.md` and to TC-030's Traces
  To list. TC-030's procedure and expected-result text name AC-45. `tests.md` has no TC count
  lines that would need updating.
- **Coverage.** `quire matrix --format tsv` (quire 0.36.2) gives 854 criteria on origin/main and
  855 at the head. The only rows that differ are AC-10's statement text, untagged both before and
  after, and the new AC-45 row, untagged. No status flips.
- **Contradictions.** `quoin matrix --json` (quoin 0.28.3) has no contradicted criterion, but
  only vacuously: all 855 criteria read "no run evidence".
- **Validation.** `make spec` passed, with warnings only in FR-017 and FR-014, both pre-existing
  and unrelated.
- **Spec-reading tests.** None read `spec/routed/**`, so no cargo spot-check is needed.

## Verdict

CONDITIONAL. The matrix is consistent. The defects below are in FR-019's own consistency and
atomicity. Fix FND-002 together with SR-13170 FND-001 to FND-004.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | "QSpec FR-331's manifest has unique registered identities after this rule is applied" turns FR-331:57 around. FR-331 requires the manifest as produced, the FR-290 snapshot, to have unique identities. So a manifest with a repeat is already not FR-331-conformant, and CG's rule is a defence at its public API, not the step that makes FR-331 hold. FR-019 Inputs ("the registered backend descriptors") and the `BackendProviderEnvelope.manifest` doc ("One descriptor per registered backend in the registry snapshot") are also not updated to say the public boundary may receive repeats | spec/routed/functional/FR-019-capability-settlement.md:132-134, spec/routed/functional/FR-019-capability-settlement.md:74-76, src/routed/capability.rs:945 |
| FND-002 | medium | FR-019-AC-45 combines about six separately failing claims in one criterion: byte-equal repeats held once, differing repeats refused with a typed payload, no descriptor retained, other identities isolated, independence from order and adjacency, and several conflicting identities. One tag on AC-45 could back part of it, as SR-13131 FND-001 found for the old compound AC-10. Split it into two or three ACs, for example hold-once, refusal and payload, and isolation and order | spec/routed/functional/FR-019-capability-settlement.md:426 |
| FND-003 | low | The conflict note is a Behavior bullet with no "shall", and ends "not a change in the number of accepted repeats". That is inaccurate: byte-equal repeats change from routing nothing (old AC-10) to being held once. Move the note to rationale or change-note prose, and state the behaviour change plainly | spec/routed/functional/FR-019-capability-settlement.md:121-125 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | FR-019-AC-47 has a split verification, "Test (TC-030) for CG; Analysis for QSL routing precondition", in a single criterion. Its Test half adds nothing CG can observe beyond AC-45: an `Err` already means no settlements. Its "produces no generation input or artifact" is not CG behaviour either, because `RoutedGenerationItem::from_descriptor` and `generate_routed` take descriptors and routed items, not a settlement, so the caller decides. One tag on AC-47 would mark the Analysis half Covered. Fold "returns no item settlements" into AC-45, and make AC-47 an Analysis-only criterion stating the corrected QSL premise (SR-13170 FND-006) | spec/routed/functional/FR-019-capability-settlement.md:439, src/routed/generate.rs:79-101, src/routed/generate.rs:295-300 |

## Dispositions

Round 1, reviewed at def6c3f4529e4439f83cc8b5972d7600eb40e364.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | The Note says CG's check "defends that precondition rather than establishing it". Inputs states the FR-331 uniqueness precondition, and the `manifest` doc reads "One descriptor per backend identity in a valid FR-331 registry snapshot" with the repeat rule |
| FND-002 | fixed | AC-45 is split into AC-45 (refusal and payload), AC-46 (ordering, case and precedence) and AC-47. Each of AC-45 and AC-46 can fail on its own in TC-030. AC-47's split verification is the new FND-004 |
| FND-003 | fixed | The note is now a **Note:** paragraph outside the shall bullets. It states plainly that byte-equal repeats, which routed nothing under the old AC-10, now cause a typed precondition refusal |
