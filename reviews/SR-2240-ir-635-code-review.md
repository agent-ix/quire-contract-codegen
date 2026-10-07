---
id: "SR-2240"
title: "IR-635 composite claim-shape spec diff: code review"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@cc3a78b0df85c6d3fde86af0b5a13a6eccc02d08; spec/replay/functional/FR-033-composite-parity-replay-binding.md, spec/replay/matrix/TC-048-composite-parity-replay-binding.md, spec/assurance/AD-003-evidence-chain.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-033
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-048
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/AD-003
    type: reviews
---

# SR-2240: IR-635 composite claim-shape spec diff: code review

## Summary

Ticket: IR-635. PR: not yet opened. Diff reviewed: ca5d61adbb62959097bfdd06a3374f2f890f2e3b...cc3a78b0df85c6d3fde86af0b5a13a6eccc02d08.
Method: code-review. Reviewer model claude-opus-5-5, session cd5dfa92-5d1b-442d-b3fe-30c6c25b005e, run 09c0daf0-442e-4959-9cd9-2346807cdb0d.

## Method

Read the full three-file diff and checked each changed statement against public QSL source at public quire-spec-language@30d7beb7483b721bcb1ec98926e8b717e9a76fb7 (QSL #645; 6f8518414a88b48a380c54e4c3171bbf9968d0a1 (#650) verified ancestor), against IR-666's merged FR-033 text (ca5d61a) and against the merged IR-631 scalar text (FR-032/TC-047). Checked delivered API names and shapes in qsl-replay: parity_obligation, ParityPreimage, ParityArgument, Domain, BoundEntries, IdentityEncodeError, CompositeIdentity::new, CompositeParityClaim, CompositeEvidence, replay_composite_parity and settle_verified_shadow with ReplayLimits last, report constructor visibility, positional composite_parity::parity_preimage, and the encoded-key ordering test. Also listed the IR-666-added lines that this diff removes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-1 reverts IR-666's merged refusal/terminal split to the pre-IR-666 'refuses with no settlement' wording, contradicting Outputs, Setup Refusal step 2 and AC-9 | spec/replay/functional/FR-033-composite-parity-replay-binding.md:309 |
| FND-002 | medium | AC-9/TC-048 require corrupting a 'decoded' report.claim() member, which cannot be done without fabricating a QSL report | spec/replay/functional/FR-033-composite-parity-replay-binding.md:317 |
| FND-003 | low | Full sent-claim procedure is an unnumbered continuation of step 8 with a broken hard wrap | spec/replay/matrix/TC-048-composite-parity-replay-binding.md:109-119 |

### Finding detail

- FND-001 (severity high, confidence high, check soundness, unit FR-033-AC-1): Merged IR-666 (#313, ca5d61a) changed AC-1 to: 'A mismatch caught by CG before invocation returns no report or terminal value; a QSL common-step mismatch returns a binding-checked `Refused` report with its terminal value.' This diff deletes that sentence and restores the pre-IR-666 text 'another package/source/node, operation or domain refuses with no settlement', which is a rebase regression of peer terminal semantics. It now contradicts FR-033 Outputs, Setup Refusal Precedence step 2 and AC-9. In QSL 30d7beb7, `prepare` refuses a node or operator mismatch through `locate` and returns `CompositeParityResult::Refused` inside a report. Once bound, that report maps to Inconclusive(ReplayRefused). Scenario: a request that passes CG prechecks reaches QSL and fails node selection. AC-1 makes a test assert no settlement, while AC-9 makes a test assert a ReplayRefused terminal for the same input.
- FND-002 (severity medium, confidence high, check untestable-ac, unit FR-033-AC-9): QSL 30d7beb7 returns `CompositeParityReport` and `VerifiedShadowReport` in-process. Their `new` constructors are pub(crate), they derive no Deserialize, and every report's `claim()` is `CompositeIdentity::new` over exactly the arguments passed. FR-033 defines no CG decode boundary where a report member could be 'decoded' and then corrupted. So the new AC-9 clause 'an independently corrupted decoded member with sent identity held fixed refuses', and TC-048 step 8's twelve 'Independently corrupt decoded ...' cases, can be executed only by fabricating or mutating a QSL report. AC-9 itself forbids that ('No ... fabricated report'), and merged TC-047 forbids it too ('Do not mutate or fabricate QSL's private report representation'). The test can never be run as written. Fix: name the CG-owned decode seam, or restate the case as mutating CG's retained sent identity and cross-binding another run's report.
- FND-003 (severity low, confidence high, check other, unit TC-048 step 8 continuation): The new full sent-claim comparison procedure is attached as an unnumbered continuation paragraph of step 8. Step 8's body describes the interim unavailable-capability state. The first line breaks after 'Retain the actual'. So the procedure cannot be cited as its own step, and a reader can take it as part of the interim-gate step rather than the post-delivery consumer check.

## Verdict

Request changes. FND-001 regresses merged IR-666 terminal semantics and must be restored. FND-002 needs a feasible formulation. The composite claim shape itself matches QSL: positional operands, self-comparison twice, composite-literal graph child with empty Bounds, inline integer singleton Range, other inline literals refused, encoded-key byte order, duplicate-key refusal within an argument, and the four-member preimage. The API signatures, the Result-returning mint with no sentinel, the separate final ReplayLimits, and PLANNED/UNRUN gate status are also accurate. F-1..F-7, AC-7, AC-12, AC-13 and step 10 are unchanged.
