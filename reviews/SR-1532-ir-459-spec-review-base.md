---
id: "SR-1532"
title: "CG PR 284 spec review: FR-024-AC-20 to AC-30 status flips, the FR-029 frame error rows, AD-002, AD-003, interface-001 and TC-035"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@528f46bad4abf8f2dbcd46672ce54a75f9cd00b2; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, spec/tests.md, spec/assurance/AD-002-cg-qsl-replay-seam.md, spec/assurance/AD-003-evidence-chain.md, spec/core/functional/interface-001-codegen-api.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md (diff 735e704...528f46b)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: references
---

# SR-1532: CG PR 284 spec review

## Summary

Ticket: IR-459. The spec diff does four things:

- It removes `PLANNED (IR-459)` from FR-024-AC-20 to AC-30, leaving their text otherwise
  unchanged.
- It rewrites FR-024's Current state and TC-035's Status.
- It flips the replay matrix row for AC-20 to AC-30 to Covered and narrows spec/tests.md to
  "FR-024-AC-1 to AC-10 are planned".
- It updates AD-002 R-6 and the frame bullet, AD-003 E-1, the interface-001 `FrameReplay::new`
  row, and FR-029's `Failed` list and AC-11.

`quire validate` passes in `make ci`, apart from pre-existing EARS warnings on FR-024 lines 242
and 244, which this PR does not touch.

Every changed status matches the code and the tests (SR-1531). Interface-001 lists exactly the
`FrameReplayError` variants the code defines. FR-029-AC-11 lists exactly the seven variants that
`terminal_map.rs` adds.

The author's ambiguities, judged:

1. AC-20, "an order other than the encoder's". The author wrote the RFC 8785 text (arguments,
   declaration, function, kind) against the preimage struct's declaration order (function,
   declaration, kind, arguments). That test is meaningful: an encoder that did not sort members
   would fail it. It is a reasonable reading, but the AC's wording does not say it (FND-001).
2. The rebased node ids for AC-28 and AC-30 are SR-1531 FND-001.
3. The FR-029 mappings are consistent with FR-029's own rule and with the state-clause AC-16
   reading, except for the document-missing PreState faults (FND-002).
4. AC-21's edge was measured.
5. The state-clause identity is untouched (Q-1), and AD-003 says so.

## Verdict

APPROVE (spec), with two low findings. The statuses are truthful and the planned criteria stay
planned. The FR-029 edit is within the rule FR-029 already states.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-024-AC-20's "a hand-written text of that preimage, written in an order other than the encoder's" reads two ways: the encoder's input (struct) order, or its output order. A digest of non-canonical text could never equal the identity, so only the first reading is satisfiable. The AC should say "the RFC 8785 member order, which differs from the preimage's declaration order". | spec/replay/functional/FR-024-counterexample-envelope-intake.md:293 |
| FND-002 | low | FR-024-AC-27 and FR-029-AC-11 make an invocation or pre snapshot that is not provided, or not readable, a `PreState` refusal read as `Failed`. QSL's own admission of those documents would refuse with a catalog code, read as `Inconclusive(ReplayRefused)`. CG's check runs first and shadows that code. Whether a driver that omits a document is a CG defect or a setup refusal on data is not argued in either FR. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:300; spec/kani/functional/FR-029-run-outcome-terminal-record.md:181 |

## Dispositions

Round 1, reviewed at 8f7db6cbc47dc8738d76d8035849ffbf126043ef.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 1440e9f: FR-024-AC-20 now reads "written in the RFC 8785 member order, which differs from the preimage's declaration order". |
| FND-002 | fixed | 1440e9f: a digest-mismatched document is now QSL's `Refused(Request(..))` with its catalog code. FR-024 argues in one paragraph why a missing or wrong-shape document stays `PreState`, read as `Failed`: the playback and documents are this side of the seam, and no QSL code exists for the case. FR-024-AC-27 and FR-029-AC-11 agree. The reasoning is sound now that the decode has verified every digest first. |
