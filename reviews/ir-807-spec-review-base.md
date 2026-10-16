---
id: SR-11730
title: IR-807 base spec review of the FR-029 terminal-pair wording cleanup
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen@f9ef0f5c246793ee5b0f48f8c52b9e083f577448 (PR 398, diff vs origin/main 2897d5e); spec/kani/functional/FR-029-run-outcome-terminal-record.md,
  spec/core/functional/interface-001-codegen-api.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/matrix/TC-040-run-outcome-terminal-record.md,
  spec/kani/matrix/tests.md, spec/replay/matrix/TC-048-composite-parity-replay-binding.md
review_set: subset
---

## Summary

Ticket: IR-807. Base checklist over the six-file spec diff that retires the claim that CG has or
will own `TerminalPairError::NonProductionProof` / `ShadowCounterexample` and a `ProofStrength`
input. Every new claim was checked against `origin/main` source, the QSL `main` FR-358 text and
Linear ticket state. Review-only; no edits.

## Verdict

**CONDITIONAL**: one high and two medium defects in the rewritten FR-029-AC-21 and its ownership,
one low in TC-040.

What holds:

- `git grep -E 'NonProductionProof|ShadowCounterexample|ProofStrength' origin/main -- src tests` is
  empty. `TerminalPairError` (src/kani/terminal.rs:53-62) has exactly `MissingSettlement`,
  `UnexpectedSettlement` and `InvalidCancellation(KaniOutcomeError)`, as the new FR-029 Outputs
  bullet says. `run_terminal_value` (terminal.rs:90-94) takes only the outcome, the SUCCESS-check
  count and an optional `ReplaySettlement`, as the new Inputs bullet says. The interface-001
  `output` lines list the two variants `run_terminal_value` and `ir_outcome_terminal_value` can
  return; `InvalidCancellation` comes only from `cancellation_terminal_value` (terminal.rs:245-260).
- QSL `main` FR-358 (spec/functional/FR-358-settle-a-composite-bounded-shadow-item.md:144-146)
  says it "replaces CG FR-029-AC-17's interim `NonProductionProof` row ... and its
  `ShadowCounterexample` row" for composite items, so retiring the CG interim rows agrees with QSL.
- AC ids are not renumbered: the FR-029 AC table has the same 29 rows and ids (AC-7 absent before
  and after). AC-21 is still PLANNED and still has a `Test` method; the tests.md row still maps it
  to TC-048. No other AC row changed.
- TC-048 step 8's new assertion (no proof-subject/strength input, no bounded-shadow variant) is a
  real, checkable assertion: a fn-pointer coercion of `run_terminal_value` and an exhaustive
  in-crate match over the three variants fail to compile if either changes.
- No IR-17 overlap: FR-014, FR-021, FR-031, TC-024, TC-031 and TC-044 are untouched.
- No hash, digest or pin is added.
- Gates: `quire validate` on the six touched files exits 0 (module-registry warnings only).
  `quire matrix` differs from origin/main only in the AC-21 text cell (still `untagged`).
  `quire coverage --scope . --strict` is 212 unbacked, 0 contradicted at both origin/main and
  f9ef0f5.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | New FR-029-AC-21 says "no CG function maps a verified, falsified, inconclusive or cover-unsatisfied bounded-shadow outcome ... to a terminal value". origin/main contradicts it: `verified_shadow_terminal_value` and `composite_parity_terminal_value` (src/replay/composite.rs:82-113, exported at src/lib.rs:204) return QSL's terminal value for a verified or falsified composite shadow, and IR-635's `settle_verified` calls `settle_verified_shadow` then `verified_shadow_terminal_value` (src/replay/composite_builder.rs:427-440). AC-20 (partial) and AC-28 (covered) already rely on these. A test written to AC-21 as worded fails against correct code. FR-028:376-378 repeats the claim ("no CG function maps a shadow result to a terminal value"). Fix: scope AC-21 to the ordinary map, e.g. "`run_terminal_value` and `TerminalPairError` carry no bounded-shadow input or variant; a bounded-shadow terminal value comes only from the composite parity converters (AC-20, AC-28) or the planned production route". | spec/kani/functional/FR-029-run-outcome-terminal-record.md:334, spec/kani/functional/FR-028-bounded-proof-ceilings.md:376-378 |
| FND-002 | medium | The rewrite drops AC-21's only behavioural obligation. The old row required that a bounded-shadow outcome yield a typed refusal and no terminal value. The new row and the Behavior text (FR-029:159-162) state an absence ("`run_terminal_value` receives no bounded-shadow evidence") with no "shall". `KaniRunOutcome` cannot tell a shadow run from a production run, so once IR-241's planned shadow harnesses (FR-028-AC-13 to AC-24) run through the ordinary path, a verified shadow outcome would map to `Proved`/`Certified` (terminal.rs:98-101), the mutation FR-028-AC-17 forbids. Nothing in the new text forbids that routing. Fix: keep a shall-statement such as "the production path shall not pass a `bounded_shadow` harness outcome to `run_terminal_value`; it reaches a terminal value only through the composite parity route". | spec/kani/functional/FR-029-run-outcome-terminal-record.md:159-162,334 |
| FND-003 | medium | The PR moves AC-21, and the proof-subject/strength inputs, from IR-241 (open, In Progress) to IR-635. IR-635 is Done in Linear (completed 2026-10-16), and no open IR ticket mentions the parity, shadow or FR-029 production invocation. AC-21 and the new Inputs bullet are now planned work with no live owner. AC-17, AC-19, AC-20 (IR-635 part) and AC-22 to AC-27 already cited IR-635 on main and were ownerless before this PR; that is an observation. Fix: name an open ticket for the production parity invocation (file one if none exists) and cite it in AC-21, the Inputs bullet, interface-001:158-159 and the tests.md:57 row. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:78-82,334,382-385, spec/core/functional/interface-001-codegen-api.md:158-159, spec/kani/matrix/tests.md:57 |
| FND-004 | low | TC-040 step 16 now says "no bounded-shadow outcome reaches the map (FR-029-AC-21)". That is a whole-system claim about callers. No TC-040 step checks it, and TC-048 step 8 checks only the function signature and the variant set, not routing. Restate it as a pointer to what TC-048 actually asserts. | spec/kani/matrix/TC-040-run-outcome-terminal-record.md:95-96 |

## Coverage

Examined: FR-029 Inputs (75-87), Outputs (88-102), Behavior parity paragraph (154-165), AC-15,
AC-17, AC-20, AC-21, AC-28, the gate/status paragraphs (350-356, 375-388); interface-001
run_terminal_value block (148-160); FR-028 312-314 and 362-380; TC-040 step 16; TC-048 step 8;
tests.md row 57. Six coverage rules: AC-21 still has TC-048 and a tests.md row with a planned
status. No hash or pin is used.

## Examined units

```yaml
scope:
- id: FR-029-AC-21
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: "PLANNED (IR-635): `run_terminal_value` has no proof-subject or proof-strength input and `TerminalPairError` has no variant for a bounded-shadow outcome, so until the production CG path invokes the delivered QSL parity facade no CG function maps a verified, falsified, inconclusive or cover-unsatisfied bounded-shadow outcome (the last two with retained refinement_failed) to a terminal value or a typed refusal, and none fabricates a predicate replay or terminal cause."
- id: FR-029
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: "`TerminalPairError` has exactly the variants `MissingSettlement`, `UnexpectedSettlement` and `InvalidCancellation`. It has no variant for a bounded-shadow outcome, and `run_terminal_value` has no input that carries one."
- id: FR-029-AC-15
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: context_only
  excerpt: "For the ordinary source-predicate `ReplaySettlement` input, a falsified outcome given no replay settlement is refused with `TerminalPairError::MissingSettlement`, and each other outcome ... given a settlement is refused with `TerminalPairError::UnexpectedSettlement`"
- id: FR-029-AC-17
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: "PLANNED (IR-635). The Eq-only verified constructor invokes QSL with supplied evidence; the closed FR-028 strength producer/projection remains gated."
- id: FR-029-AC-28
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: context_only
  excerpt: "Covered (IR-666 converter); a real QSL public prepare BoundExceeded report binds and preserves its exact ReplayRefused code/terminal"
- id: FR-028
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: "A terminal value for a verified shadow result: FR-029-AC-17 specifies the planned QSL-640-gated parity map; no CG function maps a shadow result to a terminal value until the production path invokes QSL's delivered parity API."
- id: interface-001
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: "output: qsl-replay TerminalValue | TerminalPairError (MissingSettlement | UnexpectedSettlement) ... The function takes no proof-subject or proof-strength input and TerminalPairError has no bounded-shadow variant; IR-635 owns those inputs and the parity route."
- id: TC-048
  path: spec/replay/matrix/TC-048-composite-parity-replay-binding.md
  role: examined
  excerpt: "Until production CG invokes the delivered QSL parity facade, assert that `run_terminal_value` has no proof-subject or proof-strength input and `TerminalPairError` has no bounded-shadow variant (FR-029-AC-21)."
- id: TC-040
  path: spec/kani/matrix/TC-040-run-outcome-terminal-record.md
  role: examined
  excerpt: "Until CG consumes the delivered QSL facade, no bounded-shadow outcome reaches the map (FR-029-AC-21)."
bindings: []
```
