---
id: SR-17940486709564847047728
title: "IR-631 PR #803 spec-review (status cells and matrix)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen PR #803 at 37e3e1f6; IR-631; spec/replay/functional/FR-032-routed-scalar-replay-binding.md (AC rows 1-18, status paragraph), spec/replay/matrix/TC-047-routed-scalar-replay-binding.md (Description), spec/replay/matrix/tests.md (FR-032 rows)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-047
    type: reviews
---

## Summary

Ticket: IR-631. The spec edits are status-only. Nine FR-032 criterion cells move to IMPLEMENTED,
the status paragraph and the tests.md rows are regrouped, and the TC-047 Description gains a
sentence naming the steps that `scalar_replay_acceptance.rs` implements. No requirement statement,
domain object or relationship changed, so the EARS, object, dependency and scope sub-analyses do
not apply. This review checks consistency: the status paragraph, the tests.md rows and the AC cells
agree with one another, and each claim is checked against the code. `quire validate --scope .`
over the three files exits 0.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | The TC-047 Description says `scalar_replay_acceptance.rs` implements step 8 and leaves only steps 1, 7 and 9 Planned. Step 8 also requires independent verification of the canonical preimage's closed members, tags and encodings, the unencodable-preimage `IdentityEncodeError` refusal, and a membership-valid changed range, kind or occurrence under a retained digest. The new file covers only invented membership and a non-recomputed identity. Step 8 itself still ends "These are planned, unrun CG conformance checks.", which contradicts the Description. | spec/replay/matrix/TC-047-routed-scalar-replay-binding.md:24 |
| FND-002 | low | The AC-4 and AC-5 rows keep prose that this PR made false: AC-4 "whose tags do not name AC-4 (untagged in `quire matrix` until the IR-631 code slice tags it)" and AC-5 "(tag TC-047 only, so `quire matrix` reports AC-5 untagged ...)". The PR tags both. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:398-399 |

## Verdict

The status paragraph, the tests.md rows and the AC cells agree with one another. The partition is
AC-3 to AC-5 Partial; AC-1, 2, 6, 12, 14 to 18 and 20 to 25 Implemented; AC-7 and 8 Partial;
AC-19 Planned. Whether each IMPLEMENTED cell is backed by code is judged in gap-analysis
SR-17940486709564847047727. AC-1 and AC-2 are not backed (FND-001 there), and AC-15 and AC-18
overclaim. Leaving AC-4 and AC-5 Partial is correct. Leaving AC-19 Planned is correct and stated in
both FR-032 and tests.md.
