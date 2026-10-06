---
id: SR-2223
title: "spec-failure-domain-analysis of quire-contract-codegen PR #313 (IR-666)"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen@cf3d6e4f0712fe4853095bfd51dcbd108e49bf74; FR-030 composite input, F-1..F-7 table and paragraph, AC-15..AC-17; FR-033 Falsified Settlement, Setup Refusal Precedence, AC-9, AC-13; compared with public QSL qsl-replay/src/composite.rs, execute/composite_parity.rs and proof_result.rs at 30d7beb7 (#645)"
review_set: subset
---

## Summary

Ticket: IR-666. Failure modes and identity confusion on the planned CG consumer of
`CompositeParityReport`: refusal before join/report, same-claim binding through the full
`CompositeIdentity`, fault versus non-fault refusals, and separation from ordinary source-predicate
`ReplaySettlement`. Two gaps.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-030 says a QSL common-step refusal "settles Inconclusive(ReplayRefused(code))" unconditionally, but QSL #645 maps `CompositeParityResult::Refused` through `TerminalValue::from_replay_refusal`, which gives `Failed` for `ReplayRefusal::Fault`/`Admission(Fault)`, and the composite path does construct `ReplayRefusal::Fault`; the fault reading is unspecified for the composite consumer and AC-17/TC-041 step 16 test only the non-fault case | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:112; spec/kani/functional/FR-030-ir-outcome-terminal-map.md:175 |
| FND-002 | medium | FR-030 delegates construction of the full `CompositeIdentity` binding "including observation" to FR-033, but FR-033 (AC-9, Setup step 2) lists node/run/operation/operand/domain/limits/content only and never requires comparing `report.claim()` with `CompositeIdentity::new` over the sent shadow, native and refinement; the observation check is owned only by the IR-input map FR-030-AC-17 | spec/replay/functional/FR-033-composite-parity-replay-binding.md:269; spec/kani/functional/FR-030-ir-outcome-terminal-map.md:56; spec/kani/functional/FR-030-ir-outcome-terminal-map.md:175 |

## Verdict

Refusal-before-join, three distinct `Incomplete` stages, F-2-before-F-3, and the separation from
ordinary `ReplaySettlement` MissingSettlement/UnexpectedSettlement are stated and testable. Backend
Kani timeout/memory stays outside the facade. The two gaps are a missing fault branch and a missing
owner for the observation half of the identity check.

## Dispositions

Round 1, reviewed at agent-ix/quire-contract-codegen@c44edbab9de0245308a5ea414e8774ce8e5356cb (fix diff cf3d6e4..c44edba).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | c44edba: FR-030, FR-029, FR-033 and AC-17/AC-27/AC-9 now map non-fault `ReplayRefusal` to Inconclusive(ReplayRefused) and `Fault`/`Admission(AdmissionFailure::Fault)` to Failed, matching delivered `TerminalValue::from_replay_refusal` (qsl-replay/src/proof_result.rs:312-319 at 30d7beb7); the FR-358 common-steps prose saying every refusal is Inconclusive is an upstream QSL correction, not a CG defect |
| FND-002 | fixed | c44edba: FR-033 Behavior and FR-033-AC-9 now require `report.claim() == CompositeIdentity::new(obligation, &claim, evidence)` over obligation, node, occurrence, operator, obligation kind, harness bounds, limits, content identity and the full falsified (operands, shadow, native, refinement) or verified (SUCCESS count, refinement) observation; this matches every field of QSL `CompositeIdentity` |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | The fix calls `Fault`/`Admission(Fault)` QSL common-step `Refused` results that precede Disagreed and has TC-041 step 16 and TC-048 step 10 cause them in competition with Disagreed. In delivered QSL, `Refused` also comes from operand admission after F-1/F-2 (non-Input admission refusals and `fault("converted-operand"/"two-operands")`), where Disagreed wins; and these are invariant faults with `CompositeParityReport::new` crate-private, so no named public input lets a CG test produce them | spec/replay/functional/FR-033-composite-parity-replay-binding.md:231; spec/replay/matrix/TC-048-composite-parity-replay-binding.md:120; spec/kani/matrix/TC-041-ir-outcome-terminal-map.md:48 |

## Dispositions (round 2)

Round 2, reviewed at agent-ix/quire-contract-codegen@9003f83f52a7cb7476915c20a75e843b295c1a04 (fix diff c44edba..9003f83). Changed lines re-checked for regressions of FND-001 and FND-002: none.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-003 | fixed | 9003f83: FR-029 body/AC-28, FR-030 body/AC-17, FR-033 Behavior/Setup step 2/AC-9, TC-040, TC-041 and TC-048 now place QSL `prepare` refusals before F-1 and operand-admission/exact-comparison refusals after F-1/F-2 (Disagreed wins), matching composite_parity.rs at 30d7beb7; the reachable non-fault case uses a replay input-byte limit below the encoded request (`stage_limit_exceeded/input-bytes-exceeded`, bounds.rs:72-94, checked in `ReplayRequest::decode` inside `prepare`); invariant faults are covered by inspection of public `terminal_value` and CG pass-through without a fabricated report |
