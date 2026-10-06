---
id: SR-1690
title: "IR-635 follow-up code review: composite parity identity and settlement binding"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@04608d2942dc27f47d0b820a8a58a1b82b1669c3; spec/assurance/AD-003-evidence-chain.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/kani/matrix/tests.md, spec/replay/functional/FR-033-composite-parity-replay-binding.md, spec/replay/matrix/TC-048-composite-parity-replay-binding.md, spec/replay/matrix/tests.md, spec/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-033
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/AD-003
    type: reviews
---

# SR-1690: IR-635 follow-up code review

## Summary

Ticket: IR-635. PR: quire-contract-codegen#303, spec only, base `deb612e5f763fdb99499d478670d3956f2deb0f4`. Reviewer: claude-opus-5-5, session 94d28b25-7330-445c-a85c-7e177ba3c080. The diff touches eight Markdown files under `spec/` and no production or test code. I therefore ran no separate Rust review or gap analysis. This pass checks that the changed text matches the measured upstream source. Two low findings.

## Method

I read the full diff `deb612e..04608d2`. I measured the following fresh, read-only:

- **QSL PR #645 head `7e508c499e3f14184227d0256558afb812e4e901` (OPEN):** `spec/functional/FR-358-…`, `spec/functional/FR-070-…`, `qsl-replay/src/witness/value.rs`, and a `git grep` over `qsl-replay/src` for the parity symbols.
- **QSL main `03d3f899ae5c3c2b1fde00ea572c56e4ded82b79`:** `qsl-replay/src/witness.rs`.
- **CG:** `src/kani/terminal.rs` (`run_terminal_value`) and `src/kani/classify.rs` (`KaniRunOutcome`).

I compared each changed claim against these sources:

- O-09 preimage members and DomainKey order
- CG extras excluded from O-09
- the separate `content_identity`
- common refusals before `Disagreed`
- the shared four-state `Refinement`
- F-1 to F-6 and V-1 to V-5
- the V-2 category
- `NativeCause` and the three resource stages
- `ScalarAgrees`/`CompositeEquality`
- ExactInteger leaves versus top-level `Integer(i64)`
- full enum `Variants` coverage
- other leaf families uncovered
- the FR-028-AC-2/3 backend mapping

I checked that no test tags FR-033-AC-* or the planned FR-029 criteria. I did not run cargo, Kani or `make ci`, as the brief requires.

## Verdict

**PASS with two low findings.** The following match the measured source:

- **O-09 preimage:** the claimed node, occurrence key, `obligation_kind`, and parameter ids with harness bounds ascending by `DomainKey`. Abstractions, size budget, pair count and unexercised behaviours stay in CG's record.
- **Content binding:** `content_identity` is separate from O-09 and QSL never recomputes it.
- **Common refusals:** these come before F-1.
- **F-1 to F-6:** the order, `GeneratedFault` with `NativeCause`, the `ExactEvaluation` and `RefinementCeiling` stages, and `Inconclusive(ScalarAgrees)` with `CompositeEquality` all match. A Completed/Refused native observation does not change the settlement.
- **V-1 to V-5:** the verified order matches. FR-029 priority 2's category is `incomplete`, as V-2 states.
- **Witness values:** ExactInteger exists only on QSL #645 (`WitnessValue::ExactInteger(Integer)`). Main still has only `Boolean(bool)` and `Integer(i64)`.
- **Parity API:** `replay_composite_parity`, `settle_verified_shadow` and `CompositeParity*` do not exist in either source, so the QSL-640 code gate is correctly kept.
- **FR-028-AC-2/3:** `KaniRunOutcome::Inconclusive{TimedOut}` maps to `Incomplete(TimedOut)` and `MemoryExhausted` maps to `Incomplete(ResourceExhausted)` in `run_terminal_value`, as the narrowed wording states.
- **Planned criteria:** FR-029-AC-23/24 and FR-033-AC-1..13 remain untagged.
- **Hashes and pins:** no new tracking hash or pin is introduced.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The upstream status paragraph says QSL #645 now contains composite witness value and decode code, but it omits the measured limit stated in QSL #645 FR-070 Status: "A `union` value text decodes, but the checker has no union type form (FR-321 is not yet implemented), so no replay converts one." FR-033-AC-2/3 and TC-048 step 3 still list union among the families to replay. A reader of the status paragraph can conclude that union operands only wait on the parity API. Record the union replay-conversion gap as measured upstream capability data next to the ExactInteger status. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:51, spec/replay/functional/FR-033-composite-parity-replay-binding.md:222 |
| FND-002 | low | The new enum coverage rule (FR-033 Behavior, AC-8, TC-048 step 6) covers only an all-variants harness versus a partial one. It omits two refusals that measured FR-358 step 5 and AC-7 require: a `Variants` bound naming a variant the declared enum does not admit, and a request `DeclaredDomain` over an enum position. Both refuse with the key named. AC-8's list "unknown/duplicate/kind-mismatched keys refuse" covers keys, not an undeclared variant value. A converter test written from AC-8 therefore never checks that a superset bound such as `{Blue, Green, Purple, Red}` is refused rather than counted as covering. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:227, spec/replay/matrix/TC-048-composite-parity-replay-binding.md:70 |

## Dispositions

Round 1 re-check of fix commit `48f3555` on quire-contract-codegen#303. Each finding was verified against the spec text at that commit, not against the author's receipt.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 48f3555: FR-033 Description now records the measured union gap (union text decodes; no replay conversion without checker union admission) beside the ExactInteger status; TC-048 step 3 forbids counting a union decode as replay. |
| FND-002 | fixed | 48f3555: FR-033 Behavior, AC-8, TC-048 step 6 and the AC-8 Expected Results row now require QSL's key-naming refusal for an undeclared Variants member and a DeclaredDomain over an enum position. |
