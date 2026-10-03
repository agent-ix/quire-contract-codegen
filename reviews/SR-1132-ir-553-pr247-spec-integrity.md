---
id: SR-1132
title: "integrity review of quire-contract-codegen PR 247 (IR-553 function-path obligation identity)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@f04c7a88081180ddf8170e79a9ea7005f55ae953; spec/replay/functional/FR-016-witness-native-replay.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-026-witness-native-replay.md, spec/replay/matrix/tests.md, spec/tests.md, spec/assurance/AD-001-codegen-architecture.md, spec/assurance/AD-002-cg-qsl-replay-seam.md, spec/assurance/AD-003-evidence-chain.md"
review_set: subset
---

## Summary

Ticket: IR-553. Integrity analysis of the PR diff: cross-document consistency of the
obligation-identity preimage, and trace and matrix consistency for FR-016-AC-21 to AC-23.

Examined and clean: the replay matrix row adds AC-21 to AC-23 to the Planned row; the TC-026
summary row traces all 23 ACs; `spec/tests.md` names AC-21 to AC-23 as planned; `quire
validate --scope . "spec/**/*.md"` exits 0 with only the existing `semantic.`/Duplicate
warnings; `quire coverage --strict` reports 66 unbacked rows and 0 contradicted statuses, the
same as the stated baseline. No stale "transcript digest is the design" statement remains in
AD-002, AD-003 or interface-001.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-024-AC-1 (and FR-024's Behavior bullet and AD-001:160) still define the obligation identity as every `KaniObligationIdentity` member except `source_span`, which contradicts the O-09 preimage the new FR-016-AC-21 states; AD-003 already rules "O-09 wins" and routes FR-024 to "the follow-up spec PR", and this is the identity spec PR | spec/replay/functional/FR-024-counterexample-envelope-intake.md:103 |
| FND-002 | medium | TC-026's added procedure exercises neither AC-21's positive claim (the slot equals the O-09 digest recomputed from the `FunctionSite` members through the canonical encoder) nor AC-22's "one identity per kind, not one per conjunct" clause | spec/replay/matrix/TC-026-witness-native-replay.md:30 |

## Verdict

Trace tables and counts are consistent. The PR leaves two ACs in one repository defining the
obligation identity differently (FR-016-AC-21 versus FR-024-AC-1); the correction is a few
lines in FR-024 and AD-001 and belongs in this PR. TC-026 should state how AC-21's equality
and AC-22's conjunct clause are tested.

## New findings (disposition pass 1)

Reviewed at 146b5d6306cf1195b82885c3f2705f116978f01c.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | AD-002 now decides the envelope question (FR-024 covers function counterexamples, per merged O-25), but two statements in the same AD still say otherwise: the Current state bullet at lines 164-166 ("either outside FR-024 or short of it. This AD records the gap; it does not decide it.") and the R-Q7 row ("no separate envelope for the function path (QSL's review, to be confirmed by QSL)") | spec/assurance/AD-002-cg-qsl-replay-seam.md:195 |
| FND-004 | low | AD-003's resolved-decision bullet still says in the present tense that AD-001 "defines" the preimage as every `KaniObligationIdentity` member and that FR-024 "repeats it", then says both state the O-09 preimage; the first sentence is now false | spec/assurance/AD-003-evidence-chain.md:307 |

## Dispositions

Round 1, reviewed at 146b5d6306cf1195b82885c3f2705f116978f01c.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 146b5d6 |
| FND-002 | fixed | 146b5d6 |
| FND-003 | fixed | 6b16acb |
| FND-004 | fixed | 6b16acb |
