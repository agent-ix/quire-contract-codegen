---
id: SR-17940486709564847047687
title: "IR-1516 PR #790 two-label identity and obligation-identity cross-check: spec review (base)"
type: SpecReview
analysis: base
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #790 @ b0763d44; IR-1516; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/assurance/AD-002-cg-qsl-replay-seam.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-035
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/AD-002
    type: reviews
---

## Summary

Ticket: IR-1516. Spec-only PR #790 (head b0763d44, branch spec/ir-1516-labels-and-identity, diff `origin/main...HEAD` at a52a000e). There are three files and no non-markdown change, so the code-review lane did not run. The PR changes "four identity labels" to two (authority, identity) in the FR-024 Behavior, FR-024-AC-52, AD-002 and TC-035 steps/results 11. It also adds FR-024-AC-62's closing clause, PLANNED FR-024-AC-65/66 (IR-1519) and TC-035 step 46.

What I checked, from source:

- Pinned qsl-replay 434869e8 (Cargo.lock): `qsl-semantics/src/model/observation.rs:215-223` `DocumentRef { authority, identity }`, so the document identity has two labels.
- CG `src/replay/state_clause.rs`: `IdentityMember` at :813, `SnapshotLink::reference` at :882 and `DocumentLabel` at :74, each with two members.
- `src/replay/state_clause/invariant.rs:110-203` (`decode_falsification`), `StateClauseHarnessMember` at state_clause.rs:210-225, and `From<&StateClauseReplayError> for ReplaySettlement` at state_clause.rs:379-400.
- `KaniV2ClauseObligationHarness.obligation` at src/kani/identity.rs:489, with `has_valid_provenance` at :521.
- `qsl-replay/src/identity.rs:61-71`: `ObligationIdentity` is `{node, model, ordinal}`.

## Verdict

CHANGES REQUIRED. The two-label correction matches the pinned QSL type and the code. AD-002, the Behavior bullets, AC-52 and TC-035 11 now agree.

AC-65 is implementable as written. The comparison can sit before `decode_playback` (invariant.rs:171), and `harness.obligation` is public.

Two defects remain:

- The closed refusal set now says both "obligation differs gives `HarnessMismatch`" and "ObligationIdentity differs gives `ObligationIdentityMismatch`". AC-65 also cites a `HarnessMismatch` obligation member that does not exist.
- The new variant's settlement (FR-029-AC-16) is unstated.

Clean units: the AD-002 row, AC-52 and the Behavior label bullets.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-024-AC-62 and the Behavior bullet (:449-451) still say a harness whose "obligation or field order" differs returns `HarnessMismatch`, and now also say a harness whose `ObligationIdentity` differs returns `ObligationIdentityMismatch`. A reader cannot tell which one an ordinal-only difference gets. AC-65 calls the existing check "the harness identity record's `HarnessMismatch` obligation member", but `StateClauseHarnessMember` has no obligation member (Clause, Operation, Object, Anchor, Frame, StateFields, Property). `decode_falsification` checks Clause, Object and StateFields. Fix: say "clause node, declaring object or field order" in AC-62 and the Behavior, and in AC-65 name the `HarnessMismatch{Clause, Object, StateFields}` checks. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:449-451, 592, 595; src/replay/state_clause.rs:210-225; src/replay/state_clause/invariant.rs:136-155 |
| FND-002 | medium | AC-65 adds `StateClauseReplayError::ObligationIdentityMismatch` but does not say how it settles. `From<&StateClauseReplayError> for ReplaySettlement` (FR-029-AC-16) is an exhaustive match: every harness-vs-input refusal (`HarnessMismatch`, `ClauseSelectionMismatch`, ...) maps to `CgDefect`, while caller-input faults map elsewhere. One implementer would choose `CgDefect` and another a caller-input category. State the settlement in AC-65 (or FR-029-AC-16). | spec/replay/functional/FR-024-counterexample-envelope-intake.md:595; src/replay/state_clause.rs:379-400 |
| FND-003 | low | interface-001's `InvariantReplayInputs::decode_falsification` entry is not updated. Its output enum (PlaybackAlreadySupplied, NotInvariantHarness, HarnessMismatch, Decode, AssertionMismatch) and its semantics omit the planned `ObligationIdentityMismatch` and the identity comparison. Add the variant there, marked planned (IR-1519). | spec/core/functional/interface-001-codegen-api.md:253-256 |
| FND-004 | low | FR-024-AC-66 says "CG forms no replacement identity from the harness", but no test can fail that clause. It only applies on the accept path, where the harness's and the caller's identity are equal, so a request carrying the harness's copy cannot be told from one carrying the caller's. Drop the clause or reword it so that it is falsifiable. The remaining text (accept on equality; packet and request carry the identity) is sound and kills an always-refuse mutant. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:596 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-024-AC-62 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:592 | ... a harness whose obligation or field order differs from the replay inputs returns `HarnessMismatch`; ... a harness whose QSL `ObligationIdentity` differs from the caller's returns `ObligationIdentityMismatch` (FR-024-AC-65). |
| FR-024-AC-65 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:595 | PLANNED (IR-1519). ... When `InvariantReplayInputs::decode_falsification` receives a V2 invariant harness whose `obligation` (QSL's `ObligationIdentity`) differs in any member from the `obligation_identity` the caller put in the inputs, it returns the distinct typed `StateClauseReplayError::ObligationIdentityMismatch` before it decodes the playback, builds any document or calls the executor ... |
| FR-024-AC-66 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:596 | PLANNED (IR-1519). Prerequisite: FR-024-AC-65. When the harness's `obligation` equals the caller's `obligation_identity` member for member, `decode_falsification` accepts it, and the packet and the request that `StateClauseReplay::new_invariant` builds carry that identity unchanged. CG forms no replacement identity from the harness. |
| FR-024-AC-52 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:582 | ... `pre` and `post` each naming the respective snapshot by its two identity labels (authority and identity) alone ... Two supplied documents with the same two labels ... return `DuplicateDocumentIdentity` ... |
| FR-024-behavior-invocation | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:411-414 | `pre` and `post` each the snapshot's two identity labels (authority and identity), `parameters` empty, `result` null |
| AD-002-state-documents-row | examined | spec/assurance/AD-002-cg-qsl-replay-seam.md:55 | each document declares the two labels (authority and identity) QSL reads as its identity. |
| interface-001-decode_falsification | context_only | spec/core/functional/interface-001-codegen-api.md:253-256 | output: Result<(), StateClauseReplayError (PlaybackAlreadySupplied \| NotInvariantHarness \| HarnessMismatch{member} \| Decode(DecodeFailure) \| AssertionMismatch)> |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 at 984a34b2. Verified in the fix commit: FR-024-AC-62 / Behavior :449: a harness whose clause node, declaring object or field order differs from the replay inputs returns `HarnessMismatch`; ... AC-65: `HarnessMismatch` carries no obligation member and is not reused: over one invariant harness, a caller identity differing from the harness's only in its ordinal returns `ObligationIdentityMismatch` and not `HarnessMismatch`. |
| FND-002 | fixed | Round 1 at 984a34b2. Verified in the fix commit: The variant settles as a CG defect (FR-029-AC-16), as `NotInvariantHarness` and `HarnessMismatch` do. IR-1516 delivers the invariant path only; the postcondition path's `StateClauseReplay::new` has no harness identity to compare (IR-1520). |
| FND-003 | still-open | interface-001:255 output list now has ObligationIdentityMismatch, but the semantics line (:256) still reads 'accepts only a scalar V2 invariant harness whose clause node, declaring object and drawn field order are the inputs'' with no mention of the obligation-identity comparison, so after IR-1519 the contract's acceptance rule is incomplete. Add 'and whose obligation equals the inputs' obligation_identity (FR-024-AC-65, planned IR-1519)'. |
| FND-004 | fixed | Round 1 at 984a34b2. Verified in the fix commit: FR-024-AC-66: ... the packet and the request that `StateClauseReplay::new_invariant` builds carry the caller's identity member for member, with an `ordinal` and a `node` that differ from every other identity the test constructs. |
| FND-003 | fixed | Round 2 at 788fb796. Verified in the fix commit: interface-001 decode_falsification semantics: accepts only a scalar V2 invariant harness whose clause node, declaring object and drawn field order are the inputs' and whose obligation equals the inputs' `obligation_identity` (FR-024-AC-65, planned IR-1519); ... |
