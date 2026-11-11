---
id: SR-17940486709564847047703
title: "IR-1520 PR #799 spec-review (status and evidence edits)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #799 at 0945ae7a; IR-1520; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/core/functional/interface-001-codegen-api.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/kani/matrix/tests.md, spec/replay/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
---

## Summary

Ticket: IR-1520. Reviews the PR's spec edits: the eleven rows flipped from PLANNED to Implemented,
the evidence sentences appended to them, TC-025 step 54 and TC-035 step 47, both matrix rows, the
interface-001 entries (the `planned (IR-1520, ...)` qualifiers dropped), the FR-015-AC-48 row and
its closure paragraph.

Rows flipped only where tests exist: every flipped criterion has a tagged test (`quire matrix`: all
eleven `tagged`). `quire validate` via `make spec`: exit 0 with 6 warnings, the same set as main
(none on a line this PR touches).

AC-48 closure claim checked against merged state: IR-1517 (#798, 354e7c6f), IR-1518 (#794, 69ab6109)
and IR-1519 (#793, e75918f4) are merged on main; FR-015-AC-133 to AC-136 and FR-024-AC-65/66 read
Implemented/Delivered. With this PR, the postcondition path (FR-024-AC-71/72) and the frame path
(FR-024-AC-73 to AC-75) cross-check the identity. The only other V2 decode entry,
`decode_v2_clause_falsification` (`src/replay/witness.rs`), returns decoded values and builds no
replay request carrying an obligation identity, so it is not a replay path condition (4) misses; the
precondition harness has no replay path, as the paragraph says. Conditions (1) to (4) hold once this
PR merges, and "the IR-1507 umbrella can close" is true at that point.

The coder's added sentence in the FR-015-AC-138 row (the writer refuses a key nested more than 32
levels as `StateFrameRefusal::RecordSerialization`) is a normative change made in a code PR. It
contradicts the unedited text of FR-015-AC-137 and AC-141 and the interface-001 entries (FND-002).

## Verdict

**Status edits accurate except as below; not mergeable until FND-001 and FND-002 are resolved in
this PR (text only).**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-015-AC-141, now Implemented, says classification is "`classify_state_frame_role`, called by negotiate's disposition and by the routed preflight", and that "its result is the same for any identity". In the code only the routed preflight calls `classify_state_frame_role`; negotiate's disposition calls `generate_state_frame_role` with the item's identity, renders the harness, and its outcome depends on the identity (an identity nested past 32 levels settles `Unsupported(RenderFailed)`). The "M9 is not expressible" note in TC-025 step 54 is true only for the routed preflight | spec/kani/functional/FR-015-bounded-kani-obligations.md:956, spec/kani/matrix/TC-025-bounded-kani-obligations.md:1013-1015, src/kani/generate/negotiate.rs:1148-1156 |
| FND-002 | medium | The writer depth bound is specified only as a sentence added to the FR-015-AC-138 row. FR-015-AC-137 ("returns a harness whose `StateFrameIdentity.obligation` equals ... the identity passed"), AC-141 (the negotiated harness carries the item's identity) and interface-001 (`generate_state_frame_obligations`/`generate_state_frame_role` outputs, negotiate's `StateFrameRefusal` mapping) still promise a harness for every identity. The bound itself is defensible, since without it AC-138's round trip fails for a deep key, but the exception has to be stated where those promises are made, and a deep identity's negotiate result (`RenderFailed`) has to be stated against the routed preflight's "supported" | spec/kani/functional/FR-015-bounded-kani-obligations.md:952-956, spec/core/functional/interface-001-codegen-api.md:117-125 |
| FND-003 | low | FR-024-AC-65 (not edited by this PR) still ends "the frame path's is FR-024-AC-73 to FR-024-AC-75 (IR-1520, planned)", stale once this PR flips those rows to Implemented | spec/replay/functional/FR-024-counterexample-envelope-intake.md:606 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-015-AC-48 | examined | Implemented (specified by IR-489; code IR-1507, IR-1517, IR-1518, IR-1519 and IR-1520; closed by IR-1520) ... Closure condition (4) below is met: the invariant path (IR-1519), the postcondition path and the frame path (IR-1520) each cross-check the identity on replay, so the PARTIAL note is retired and the IR-1507 umbrella can close. |
| FR-015-AC-48 closure paragraph | examined | AC-48 is fully met now that they do (IR-1517, IR-1518, IR-1519 and IR-1520 are all implemented) ... so the IR-1507 umbrella can close. |
| FR-015-AC-137 | examined | Implemented (IR-1520). generate_state_frame_role returns a harness whose StateFrameIdentity.obligation equals, member for member, the QSL ObligationIdentity passed for that role. |
| FR-015-AC-138 | examined | Implemented (IR-1520). ... The writer refuses a key nested more than 32 levels (the reader's serde_json depth limit would not read it back) as StateFrameRefusal::RecordSerialization, not writing an unreadable record. |
| FR-015-AC-139 | examined | Implemented (IR-1520). from_record refuses a record that lacks obligation, or whose obligation lacks node or ordinal, with StateFrameRecordError. |
| FR-015-AC-140 | examined | Implemented (IR-1520). The identity is not an input to harness source text, the harness symbol or the module symbol. |
| FR-015-AC-141 | examined | Implemented (IR-1520). Classification (classify_state_frame_role, called by negotiate's disposition and by the routed preflight KaniForm::V2StateFrame) never renders a harness and does not read the identity: its result is the same for any identity. |
| FR-015-AC-142 | examined | Implemented (IR-1520). StateFrameRoleIdentities::new refuses an equal pair with StateFrameRoleIdentitiesError::SharedIdentity. |
| FR-024-AC-71 | examined | Implemented (IR-1520). The check runs after the harness-kind check (NotPostconditionHarness) and before the HarnessMismatch checks. |
| FR-024-AC-72 | examined | Implemented (IR-1520). An equal identity is accepted and the packet and request carry it member for member. |
| FR-024-AC-73 | examined | Implemented (IR-1520). FrameReplay::new mints the frame obligation identity and compares it with the harness obligation; a difference returns FrameReplayError::ObligationIdentityMismatch before the request is built. |
| FR-024-AC-74 | examined | Implemented (IR-1520). An equal minted identity is accepted and the request and packet carry it. |
| FR-024-AC-75 | examined | Implemented (IR-1520). FrameReplayError::ObligationIdentityMismatch settles as ReplaySettlement::CgDefect. |
| TC-025 step 54 | examined | (Implemented, IR-1520.) ... M9 is not expressible, since classify_state_frame_role takes no identity parameter, so classification cannot read one. |
| TC-035 step 47 | examined | (Implemented, IR-1520.) ... Every mutant named above was applied at its site and fails the named assertion. |
| kani tests.md FR-015-AC-137..142 row | examined | Covered (IR-1520): ... asserted by the tagged tests of tests/it/state_frame_obligation_identity.rs. |
| replay tests.md FR-024-AC-71..75 row | examined | Covered (IR-1520); step 47 ... asserted by the tagged tests of tests/it/state_frame_obligation_identity.rs. |
| interface-001 generate_state_frame_obligations, generate_state_frame_role, FrameReplay::new, decode_falsification | examined | IR-1520 qualifiers changed from planned to current. |
| FR-024-AC-65 | examined | ... the frame path's is FR-024-AC-73 to FR-024-AC-75 (IR-1520, planned). |
