---
id: SR-1501
title: "Scope boundary review of quire-contract-codegen PR 276 (IR-460 state-clause replay spec)"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen@d22cc5d849db3edd7bba68c3638bb9c4995ac723; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, spec/assurance/AD-002-cg-qsl-replay-seam.md (PR #276 diff vs origin/main 7345463; QSL qsl-replay locked at c8f0c28)"
review_set: subset
---

## Summary

Ticket: IR-460. Checked scope and ownership. Only postconditions (Invocation) are specified, and the PreCall/Current exclusion is stated plainly. No behaviour beyond IR-460 is invented, and no compatibility layer is added. No QSL content is copied: qsl-replay at c8f0c28 exports no invocation or snapshot document builder, so building one in CG is CG's own job. Moving CG's own test-support builder (tests/state_frame_support/native_twin.rs) into CG's src is not vendoring. No public-repo problem was found: the PR cites only public tickets and QSL ids. Overlap: #275 (IR-461) edits interface-001, FR-015, TC-025 and AD-004, with no shared file and no shared numbering (FR-015-AC-59..65 there, FR-024-AC-11..17 here). Fixing SR-1497 FND-001 would touch interface-001 and conflict with #275. IR-459's PR #209 and its parked branch edit native_twin.rs heavily on the pre-IR-321 layout, a merge risk for the IR-460 code PR that moves that builder. Three boundary defects.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The spec does not say how the src builder encodes and digests documents. native_twin.rs, the builder the code PR is to move, has a hand-rolled RFC 8785 subset (`canonical()`, documented as handling only strings, integers and booleans) and hashes with sha2, which is only a dev-dependency. AD-004 makes src/core/canonical.rs (quire_canonical to_vec and sha256) the one canonical encoder. A verbatim move would add a second RFC 8785 encoder to src and promote sha2 to [dependencies]. FR-024 should require the sha256-jcs digests to come from core::canonical. | FR-024-AC-13, AD-004 (core/canonical.rs), Cargo.toml [dev-dependencies] sha2 |
| FND-002 | medium | AD-002 is updated only in its open-question row. Its title and System Boundary name call_site, replay and replay_frame. The "What crosses the seam" table lists no replay_state_clause, StateClauseCounterexample envelope, StateClauseReplayResult or CG-built invocation and snapshot documents, yet still says "Nothing else crosses". The known-gaps note at line 180 still says CG has no clause-replay entry. The resolved question also stays under Open questions. | AD-002:43-57, AD-002:180-181, AD-002:190 |
| FND-003 | medium | "Its post snapshot from a native run of the operation over that pre state" assigns no owner or input for the native run. The operation is customer subject code that CG does not link, and FR-024 Inputs list no post-state values. Whether CG runs the subject, a generated concrete-playback test reports the post state, or the caller supplies it is left to the code PR. | FR-024-AC-13, FR-024-AC-15, FR-024 Behavior line 102 |

## Dispositions

Round 1, reviewed at 02a5f67616fed74c0c014d546e2938e555791968.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 02a5f67616fed74c0c014d546e2938e555791968: the Behavior bullet routes every document's encoding and sha256-jcs digest through core::canonical, which gains a bytes function beside content_digest, with no second encoder and no hashing dependency in [dependencies]. The AC that should back this is weak; see SR-1500 FND-003. |
| FND-002 | still-open | AD-002 adds the title, boundary and seam-table rows and the known-gaps text, and removes the open question. Still stale: Dependency direction says "CG calls qsl_replay::replay and replay_frame only. No path takes a caller-supplied executor.", which is false with replay_state_clause and StateClauseReplay::replay_through. The failure-outcomes table has no row for StateClauseReplayError (MissingField, OutOfDomain, Document, or the state path's Name/Transcript/Envelope/CallSite/Refused), and its domain row says "(function path)". |
| FND-003 | fixed | 02a5f67616fed74c0c014d546e2938e555791968: the caller supplies the post-state values, obtained by running the customer subject natively. CG links no customer code, as on the frame path. Inputs list these values with the object address and labels. |

Round 2, reviewed at 6df9b318b612af8b7544f34e8c2d9e87ce65590c.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-002 | fixed | 6df9b318b612af8b7544f34e8c2d9e87ce65590c: AD-002 Dependency direction now names replay_state_clause and both caller-supplied executors (replay_counterexample_through and StateClauseReplay::replay_through), with "the frame path takes none". The failure table adds two state-clause rows: CG-raised MissingField, OutOfDomain, UnsupportedOperationShape, Document, Name, Transcript and Envelope map to Failed; Dependencies, CallSite and Refused read as their QSL refusals. |
| FND-004 | fixed | 6df9b318b612af8b7544f34e8c2d9e87ce65590c: the IR-459 ownership is removed. The state-clause code keeps its packet assembly local, and whichever code lands second extracts the shared piece in its own change. |

Round 3, reviewed at 19196250716f86adf9af536a1cfce0399cd10424.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-005 | fixed | 19196250716f86adf9af536a1cfce0399cd10424: FR-024 now names the source (the shape is read from the caller's admitted package) and states that it is the package whose documents the byte provision carries. The added claim about what QSL does on a mismatch is not accurate for the frame-template design; see FND-006. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | Risk flag, not decided here. The new Scope paragraph assigns the shared packet assembly (src/replay/envelope.rs) to IR-459's frame-envelope work and says the state-clause code "builds on it once it has merged", with a fallback if IR-460 lands first. IR-459's open PR #209 and its parked branch are on the pre-IR-321 layout (src/frame_replay.rs, "parked, not green"), and nothing in this repo or the PR shows that IR-459 has accepted that module. The spec commits another ticket's scope and an ordering it does not control. | FR-024 Scope |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | The state field ranges and the operation's declared parameters and result are "read from the admitted package and the clause's node, which the caller supplies". QSL's admission checks the domain package document in the byte provision, a second source the spec does not tie to the first. A disagreement settles honestly as a QSL Refused, read Inconclusive(ReplayRefused), so nothing is lost. Name which package the shape check reads, or state that the two must agree. | FR-024 Inputs, FR-024-AC-19 |

## New findings (disposition pass 3)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | FR-024-AC-19 says that when the caller's admitted package and the provided domain document differ, "QSL's recompile refuses the stale package_id (QSL FR-098)". That holds only if the request's package_id comes from the admitted package. On the frame template (src/replay/frame.rs:169-180) the package_id is call_site's, which QSL computes by compiling the unit with the same provided documents that replay_state_clause recompiles with (execute.rs Rule 5). The two ids therefore agree, and a stale admitted package passes the shape check and surfaces as an FR-106 admission refusal (missing-member or unknown-member), read Inconclusive(ReplayRefused). Fix: require StateClauseReplay::new to compare the admitted package's package_id with call_site's and refuse on mismatch, or drop the sentence. This is limited to a caller supplying inconsistent inputs, which is why it is low. | FR-024-AC-19, src/replay/frame.rs:169-180 |
