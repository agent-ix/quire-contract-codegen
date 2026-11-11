---
id: SR-17940486709564847047689
title: "IR-1516 PR #790 two-label identity and obligation-identity cross-check: integrity"
type: SpecReview
analysis: integrity
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #790 @ b0763d44; IR-1516; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/assurance/AD-002-cg-qsl-replay-seam.md; whole-repo label sweep over spec/"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-044
    type: reviews
---

## Summary

Ticket: IR-1516. This review covers whole-spec consistency of the label correction, matrix and Status integrity, and AC numbering.

**Label sweep.** I ran `grep -rniE 'four[^.]{0,40}label|label[^.]{0,30}four|revision namespace|three[^.]{0,40}label'` over `spec/`. Every FR-024, AD-002 and TC-035 document-identity occurrence is corrected. Two hits remain:

- interface-001:246, "three document identity labels". This is unrelated and correct. It counts the three `DocumentLabel` values of `StateClauseReplayInputs` (`invocation_label`, `pre_label`, `post_label`, src/replay/state_clause.rs:103-107), not labels per document.
- FR-044:31/56 and TC-058:25, "all four `SourceIdentity` string labels (`authority`, `identity`, `revision_namespace`, `revision`)". This is a different type from `DocumentRef`. It is not unrelated in substance, though: pinned qsl-foundation/src/source.rs:19-25 `SourceIdentity` also has only `authority` and `identity` (`SourceLabel`: "One of the two labels"). See FND-001.

**Code doc comments.** These still say "four labels": src/replay/state_clause.rs:275 (`DuplicateDocumentIdentity`), :874 (`SnapshotLink`), and src/replay/frame.rs:234 (`PreNotProvided.reference`) and :583 (`provided`). src/replay/state_clause/invariant.rs has none. They belong in IR-1519, and the lead's IR-1519 comment already adds them; that comment lists invariant.rs, which has no such occurrence.

**AC numbers and status.** FR-024-AC-65/66 and TC-035 step 46 are free. No open PR (#791, 762, 583, 553, 535, 523, 501, 474, 413, 381) adds FR-024-AC-6x or a step 46. Their PLANNED status names IR-1519, which exists (Backlog) and describes this work. TC-035 Status records step 46 as planned for IR-1519. `quire matrix` lists AC-65/66 as untagged, as expected for planned criteria.

## Verdict

PASS WITH FINDINGS. The in-scope correction is complete and consistent. The PR does not fix the FR-044 staleness, but that is out of this diff and needs a follow-up ticket. The hand-written tests.md matrix and the AC-65 sequencing clause are small gaps.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-044 (and TC-058 step 1) still require "all four `SourceIdentity` string labels (`authority`, `identity`, `revision_namespace`, `revision`)". Pinned QSL `SourceIdentity` has two (qsl-foundation/src/source.rs:19-25), so the requirement asks for members the type no longer has. The author's ruling that it is unrelated is right about the type (a source identity, not a `DocumentRef`) and wrong about correctness: it is the same stale claim against the same pin. No evaluator-corpus code exists yet, so this is spec-only. Fix in this PR or file a follow-up ticket (FR-044/TC-058 two-label `SourceIdentity`). | spec/oracle/functional/FR-044-export-generated-closure-evaluation-corpus.md:31, 56; spec/oracle/matrix/TC-058-generated-closure-evaluation-corpus.md:25 |
| FND-002 | low | spec/replay/matrix/tests.md has no row for FR-024-AC-65/66. Every other planned FR-024 criterion has a 🚧 Planned row, and the TC-035 row's AC list (tests.md:52) omits them. Add `FR-024-AC-65, FR-024-AC-66 \| TC-035 \| 🚧 Planned (IR-1519)` and extend the TC-035 row. | spec/replay/matrix/tests.md:20-28, 52 |
| FND-003 | low | FR-024-AC-65 says "`StateClauseReplay::new_invariant` is not reached", which no test can observe. `decode_falsification` never calls `new_invariant`; the caller sequences them. Because `InvariantReplayInputs.playback` and `.obligation_identity` are public, a caller can fill the playback and call `new_invariant` directly, which bypasses the cross-check, as it already bypasses the `HarnessMismatch`/`AssertionMismatch` checks. AC-65 also leaves its order against the NotInvariantHarness, provenance and `HarnessMismatch` checks unstated; only "before decode" is fixed, which is enough for the step 46 ordinal-only fixture. Reword to "leaves the inputs' playback empty" (observable), or state the bypass as a known limit. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:595; src/replay/state_clause/invariant.rs:55-81, 110-178 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-044-source-labels | examined | spec/oracle/functional/FR-044-export-generated-closure-evaluation-corpus.md:31 | display path and all four `SourceIdentity` string labels (`authority`, `identity`, `revision_namespace`, `revision`) |
| interface-001-state-clause-inputs | examined | spec/core/functional/interface-001-codegen-api.md:246 | ... the state object's address, three document identity labels, ... |
| AD-002-state-documents-row | examined | spec/assurance/AD-002-cg-qsl-replay-seam.md:55 | each document declares the two labels (authority and identity) QSL reads as its identity. |
| FR-024-AC-65 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:595 | ... it returns the distinct typed `StateClauseReplayError::ObligationIdentityMismatch` before it decodes the playback, builds any document or calls the executor, and `StateClauseReplay::new_invariant` is not reached. |
| FR-024-AC-66 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:596 | PLANNED (IR-1519). Prerequisite: FR-024-AC-65. When the harness's `obligation` equals the caller's `obligation_identity` member for member, `decode_falsification` accepts it ... |
| TC-035-status | examined | spec/replay/matrix/TC-035-counterexample-envelope-intake.md:447 | Step 46 (FR-024-AC-65 and FR-024-AC-66) is planned for IR-1519. |
| tests-md-FR-024 | examined | spec/replay/matrix/tests.md:28, 52 | FR-024-AC-57 through FR-024-AC-64 \| TC-035 \| ✅ Covered (IR-1505) ... |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | still-open | Label count fixed (FR-044:31 'both SourceIdentity string labels (authority and identity)', :56 'two labels', AC-1 'two source-identity labels', TC-058:25 'two source labels'), but the revision-label clauses remain and refer to labels the pinned SourceIdentity (qsl-foundation/src/source.rs:19-25) does not have: FR-044:37 'Source authority, identity and byte digest bind that package id; revision labels do not.'; FR-044-AC-1 '... while changed revision labels alone do not.'; TC-058 step 1 :31 'change only revision labels and assert admission with the same package_id (FR-044-AC-1)', a test step that cannot be executed. Remove the three revision-label clauses. |
| FND-002 | fixed | Round 1 at 984a34b2. Verified in the fix commit: tests.md: / FR-024 / FR-024-AC-65, FR-024-AC-66 / TC-035 / Planned (IR-1519); step 46, the invariant path's obligation-identity cross-check; the postcondition path is IR-1520 / ; TC-035 row Traces To now ends '..., FR-024-AC-64, FR-024-AC-65, FR-024-AC-66'. |
| FND-003 | fixed | Round 1 at 984a34b2. Verified in the fix commit: When `InvariantReplayInputs::decode_falsification` receives a V2 invariant harness whose `obligation` (QSL's `ObligationIdentity`) differs in any member from the `obligation_identity` the caller put in the inputs, it returns the distinct typed `StateClauseReplayError::ObligationIdentityMismatch`, with the playback left unbound, no document built and the executor uncalled. The check runs after the harness-kind check (`NotInvariantHarness`) and before the `HarnessMismatch` checks of the clause node, declaring object and field order, so a harness differing in both identity and field order returns `ObligationIdentityMismatch`. `HarnessMismatch` carries no obligation member and is not reused |
| FND-001 | fixed | Round 2 at 788fb796. Verified in the fix commit: FR-044:37 'Source authority, identity and byte digest bind that package id.'; FR-044:79 '... refuses as `package_mismatch`.'; FR-044-AC-1 '... a changed authority, identity or valid source text with the old `package_id` refuses `package_mismatch`.'; TC-058 step 1 'Change authority, identity and valid source text one at a time without recompiling `package_id` and assert `package_mismatch` (FR-044-AC-1).' No revision-label reference remains in FR-044 or TC-058. |
