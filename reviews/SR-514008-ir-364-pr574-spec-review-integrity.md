---
id: SR-514008
title: "IR-364 PR #574 spec-review/integrity review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; PR #574 head 505c39bbba422e9469665656bc664842aabcd35d from main 1944268f; spec/evidence/functional/FR-004-vacuity-evidence.md; IR-364"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-004
    type: reviews
---

## Summary

Integrity (completeness, consistency, atomicity, cross-reference) review of PR #574's FR-004
edit. Ticket: IR-364. AC ids FR-004-AC-1..24 are contiguous and unchanged; no duplicate ids. No
other file references an FR-004 open-question id (repo grep), so the relabel breaks no link.
The new text depends on FR-011 content that only #572 adds.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-004's sole open question is relabelled OQ-1 to OQ-2, leaving no OQ-1; the local id now reads as FR-011 OQ-2 and diverges from #571, which keeps OQ-1 for the same entry, adding friction to the #571 rebase. | spec/evidence/functional/FR-004-vacuity-evidence.md:242 |
| FND-002 | low | The bullet defers the rule "under FR-011" and the OQ says it "settles FR-011 OQ-1", but FR-004 has no FR-011 relationship or Dependencies entry, and on main FR-011 OQ-1 is still open; after #572 deletes it the sentence cites a removed OQ. Hard dependency on #572 is stated only in the PR body. | spec/evidence/functional/FR-004-vacuity-evidence.md:31, spec/evidence/functional/FR-004-vacuity-evidence.md:243, spec/evidence/functional/FR-004-vacuity-evidence.md:250-252 |

## Verdict

PASS with two low notes. Structure, ids and AC table are intact; fix by keeping the label OQ-1
(or agreeing one label with #571), adding FR-011 as an upstream dependency, and phrasing the
settled point as "FR-011's execution-point rule" rather than a deleted OQ id. Merge only after #572.

## Coverage

```yaml
scope:
- id: FR-004-OQ-2
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: "OQ-2 (requirement and revision identity): the V2 source of these identities remains the open FR-011 OQ-2 decision. The explicit per-clause execution point above settles FR-011 OQ-1 for this analyzer; it does not supply or infer requirement or revision identity."
- id: FR-004
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: "Each selected clause's execution point is the explicit typed point carried in the V2 generation request and authenticated against its accepted QSL binding, or against the caller's typed invocation context when no binding supplies a point, under FR-011."
- id: FR-004-Dependencies
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: "**Upstream**: FR-014 and FR-002."
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The open question is labelled OQ-1 again (head b4876d1f). |
| FND-002 | fixed | FR-011 `depends_on` relationship and Upstream entry added; the OQ cites "FR-011's execution-point rule", not a deleted OQ id. |
