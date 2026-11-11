---
id: SR-789003
title: "IR-1505 PR #789 spec-review of the implementation-status spec edits"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen PR #789 at 8f6517e1; IR-1505; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, spec/assurance/AD-002-cg-qsl-replay-seam.md, spec/core/functional/interface-001-codegen-api.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-035
    type: reviews
---

## Summary

Ticket: IR-1505. PR #789 (head 8f6517e1 on origin/main 3a4fa795, branch code/ir-1505-impl) removes the PLANNED markers from FR-024-AC-57 to AC-64 (spec merged as CG #785), rewrites FR-024's Current state paragraph, TC-035's Status line and the tests.md FR-024 and TC-035 rows to Covered/Partial, removes "planned" from the AD-002 seam row, and adds `StateClauseReplay::new_invariant` and `InvariantReplayInputs::decode_falsification` to interface-001. Checked against the implementation (`src/replay/state_clause/invariant.rs`) and against pinned qsl-replay 434869e8 (Cargo.lock; checkout `~/.cargo/git/checkouts/quire-spec-language-c986aa40394a5ad7/434869e`). Sub-analysis chosen by scope: status integrity of the flipped criteria. No EARS or object review was run, because no requirement statement or domain object is new in this PR. `make spec` gives 6 warnings, the same set as main, and none is in an edited file. `quire matrix` shows FR-024-AC-57 to AC-64 as `tagged`.

## Verdict

**CHANGES REQUIRED (spec text only, no code consequence).** The status flips are true. Every criterion has a tagged test, `StateClauseReplay::new` is unchanged, and the interface-001 entries match the public signatures and the error variants the code returns. Two statements in the spec being flipped to implemented are false against the pinned QSL or the code: the four-label document identity and the unconditional per-field `DeclaredDomain` bullet. Both need a small text edit in this PR so the implemented spec says what was built. Gap rulings: gap 2 (four labels) and gap 3 (`declared_domains`) are rule (i), spec edits in this PR. Gap 4 (empty drawn fields, collection invariants) is rule (ii), an acceptable implementer choice, with an optional one-line spec mention. Gap 1 (`DuplicateDocumentIdentity`) is ruled in SR-789002.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Gap 2: the invariant Behavior bullet ("the snapshot's four identity labels (authority, identity, revision namespace and revision)") and FR-024-AC-62 ("documents sharing four identity labels") name four labels. At the pinned qsl-replay 434869e8, `DocumentRef` (qsl-semantics/src/model/observation.rs:215-223) is "FR-001's two source labels": `authority` and `identity`. CG's `DocumentLabel`, `IdentityMember` and `ensure_unique_documents` carry exactly those two, and so does the implementation. Ruling (i), fix in this PR: in the IR-1505 bullet replace "four identity labels (authority, identity, revision namespace and revision)" with "two identity labels (authority and identity, QSL's `DocumentRef`)", and in AC-62 replace "sharing four identity labels" with "sharing both identity labels (authority and identity)". No code change. The same four-label wording on the postcondition path (FR-024-AC-52, the invocation-document Behavior bullet, the AD-002 state-document row) predates this PR and is a follow-up ticket (iii). | spec/replay/functional/FR-024-counterexample-envelope-intake.md:173-180; spec/replay/functional/FR-024-counterexample-envelope-intake.md:587 |
| FND-002 | medium | Gap 3: the Behavior bullet "The generator shall set the envelope's `declared_domains` to one `DeclaredDomain` per state field, from the field's declared integer range" is unconditional. The invariant envelope sets `declared_domains: Some(Vec::new())` (invariant.rs:317), so as written the implemented invariant path violates a `shall`. Soundness: no weakening at the pinned QSL. qsl-replay's state-clause execution (`execute/state_clause.rs`) never reads `declared_domains`: only `model_check/graph_replay.rs:98` and `execute/composite_parity.rs:83` do, and FR-024 itself records that QSL has no declared-domain check yet (Current state, "Declared-domain key divergence"). The only range check on this path is CG's own `OutOfDomain` from `field_range`, which mutation M4 shows the tests hold. QSL's snapshot admission also refuses an out-of-range field value (`invalid_runtime_input`/`invalid-value`, reviewer probe), so empty `declared_domains` neither weakens QSL's check nor lets a wrong value pass. Ruling (i), fix in this PR: scope that bullet to "a postcondition state-clause envelope", and add one bullet modelled on FR-024-AC-29: "The generator shall leave the invariant envelope's `declared_domains` empty until the state-clause path adopts QSL's `FieldSite.domain` key (QSL-345); the playback's range check is CG's `OutOfDomain` (FR-024-AC-61)." No code change. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:400-401; src/replay/state_clause/invariant.rs:317 |
| FND-003 | low | Gap 4: two refusals the code returns are not stated in the spec. A V2 invariant harness that draws no state field returns `HarnessMismatch { StateFields }`, although AC-62 defines `HarnessMismatch` as a harness that "differs from the replay inputs". An invariant harness with a collection precondition or collection operation returns `NotInvariantHarness`, while AC-62 lists only "postcondition, frame or precondition" harnesses. Both stay inside AC-62's closed set of variants, and interface-001 already says "accepts only a scalar V2 invariant harness". Ruling (ii), acceptable implementer choice. Optional one-line addition to the AC-62 Behavior bullet: "a collection V2 invariant harness is `NotInvariantHarness`, and an invariant harness drawing no state field is `HarnessMismatch` (`StateFields`)". Neither branch is tested (SR-789001 FND-003). | src/replay/state_clause/invariant.rs:119-152; spec/replay/functional/FR-024-counterexample-envelope-intake.md:587 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-024-AC-57 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:582 | CG builds a StateClauseCounterexample whose observation is ClauseSelectionInput::Current ... exactly one state document, the current snapshot, with the same anchor as the selection |
| FR-024-AC-58 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:583 | The current snapshot binds exactly the state fields the V2 invariant harness draws ... the replay ends Refused holding QSL's missing-member refusal for audit |
| FR-024-AC-59 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:584 | The invariant is selected by its declared QualifiedName |
| FR-024-AC-60 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:585 | The invariant replay returns exactly the StateClauseReplayResult that qsl_replay::replay_state_clause returns |
| FR-024-AC-61 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:586 | Each playback refusal is a distinct typed StateClauseReplayError |
| FR-024-AC-62 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:587 | The refusal set of the invariant path is closed and typed ... documents sharing four identity labels return DuplicateDocumentIdentity |
| FR-024-AC-63 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:588 | The invariant envelope rides the Witness arm ... operation name is the selected anchor's name ... witness record is none |
| FR-024-AC-64 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:589 | Over a Bank model with object Account ... two playbacks ... each settling reproduced-with-evaluated-witness, violation |
| FR-024 Current state | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:616 | FR-024-AC-57 to FR-024-AC-64 ... are implemented by StateClauseReplay::new_invariant |
| FR-024 Behavior (IR-1505 bullets) | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:173-180, 400-401, 418-448 | invariant documents, declared_domains, observation, selection, transcript and refusal bullets |
| TC-035 steps 39-45 and Status | examined | spec/replay/matrix/TC-035-counterexample-envelope-intake.md:352-388, 431 | steps 39 to 45 and their mutation controls |
| spec/replay/matrix/tests.md | examined | spec/replay/matrix/tests.md:28, 50 | FR-024-AC-57 through AC-64 Covered (IR-1505); TC-035 Partial |
| AD-002 seam row | examined | spec/assurance/AD-002-cg-qsl-replay-seam.md:54 | object-type invariant with Current observation, FR-024-AC-57, IR-1505 |
| interface-001 new_invariant / decode_falsification | examined | spec/core/functional/interface-001-codegen-api.md:249-256 | StateClauseReplay::new_invariant and InvariantReplayInputs::decode_falsification |

## Dispositions

Round 1, head 4c1b0509 (fix commit 72867ce8, rebased on main 0f3bc428 after #788). Focused job at the new head: lib unit test and 8 integration tests pass. quire matrix shows FR-024-AC-57..64 tagged; make spec gives 6 warnings, the same as main, none in edited files. Reviewer mutations: D1, removing ensure_unique_documents from request_documents, is caught by the unit test (invariant.rs:378). D2, having new_invariant bypass the seam and build documents inline without the guard, is not caught; it is an equivalent mutant on public behaviour, because no public input yields a duplicate (AC-62 now says so). The two unreachable decode branches (non-integer, i64 overflow) were folded into one guard with a source comment; that is acceptable, because the decoder's I64 binding yields WitnessValue::Integer(i128::from(i64)) only (replay/witness.rs:375-380).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | after: and the snapshot's two identity labels (authority and identity, QSL's `DocumentRef`); ... if two documents share the identity labels, `DuplicateDocumentIdentity` (FR-024-AC-52's guard, unreachable by a public invariant input) |
| FND-002 | fixed | after: - The generator shall set a postcondition envelope's `declared_domains` to one `DeclaredDomain` per state field, from the field's declared integer range. - The generator shall leave an invariant envelope's `declared_domains` empty until that path adopts the key QSL's `FieldSite.domain` returns (QSL-345), as FR-024-AC-29 does for the frame path. The range check on an invariant playback is CG's `OutOfDomain` (FR-024-AC-61): QSL's state-clause replay never reads `declared_domains`, and its admission refuses an out-of-range value. |
| FND-003 | accepted-no-change | Ruled (ii), an acceptable implementer choice; the spec mention was optional and was not added. The behaviour is now tested (tc_035_the_remaining_invariant_refusals_are_typed), and interface-001 states 'only a scalar V2 invariant harness'. |
