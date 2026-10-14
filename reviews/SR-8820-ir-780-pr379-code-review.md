---
id: SR-8820
title: code-review review of IR-780 cancellation producer
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md,
  spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/kani/functional/FR-030-ir-outcome-terminal-map.md,
  spec/kani/functional/FR-040-cancellation-producer.md, spec/kani/matrix/TC-041-ir-outcome-terminal-map.md,
  spec/kani/matrix/TC-055-cancellation-producer.md, spec/kani/matrix/tests.md, spec/spec.md
review_set: subset
---

## Summary

The eight-file Markdown diff introduces no production code, copied artifact, or claimed implemented cancellation path. Its planned status is explicit.

## Examined units

- `FR-029-AC-15` (examined): For the ordinary source-predicate `ReplaySettlement` input, a falsified outcome given no replay settlement is refused with `TerminalPairError::MissingSettlement`, and each other outcome (`verified`, `cover-unsatisfied`, `cancelled` and every inconclusive reason) given a settlement is refused with `TerminalPairError::UnexpectedSettlement`; neither returns a terminal value. The distinct typed composite-parity settlement input is governed by AC-17 and AC-19 to AC-28, not this criterion.
- `FR-029-AC-29` (examined): A direct `KaniRunOutcome::Cancelled { source }` maps both IR source variants to QSL `IncompleteCause::Cancelled` with the matching exact cause, using the same exhaustive conversion as FR-030; supplying a replay settlement returns `UnexpectedSettlement`. The production cancellation path publishes only FR-030's result.
- `FR-030-AC-1` (examined): Every pair (`KaniOutcomeKind`, replay settlement) that the map's input can express maps to exactly one `TerminalValue`.
- `FR-030-AC-14` (examined): A `Counterexample` given no replay settlement is refused with `TerminalPairError::MissingSettlement`, and each other kind given a settlement is refused with `TerminalPairError::UnexpectedSettlement`; neither returns a terminal value.
- `FR-030-AC-3` (examined): `TimedOut` and `ResourceExhausted` map to their distinct `Incomplete` causes. `Cancelled { source: Requested }` and `Cancelled { source: Deadline }` map to `IncompleteCause::Cancelled` carrying the corresponding `quire_exact::CancelCause`, with no source inferred from `code`, `source_id`, `context` or elapsed time.
- `FR-030-AC-7` (examined): The map is one `match` over the pair (`KaniOutcomeKind`, replay settlement) with no wildcard arm.
- `FR-040-AC-1` (examined): A caller retains a live `Cancel`, sends its clone through a public single-run request, and the bounded launcher observes it before spawn and during an actual running backend. Pre-spawn cancellation starts no backend; running cancellation confirms owned teardown before evidence returns.
- `FR-040-AC-2` (examined): In two live single runs, issue `Requested` then `Deadline`, and reverse the calls. Each run's actual launcher stop, `KaniExecutionEvidence.outcome`, constructed IR `Cancelled { source }` and QSL `IncompleteCause::Cancelled { source }` retain the first cause; a later event cannot change them. A hand-built IR outcome alone does not satisfy this criterion.
- `FR-040-AC-3` (examined): A run that reaches its execution ceiling with no cancellation event remains `TimedOut`. A `Deadline` event recorded only after completion, memory/capture stop or elapsed-ceiling stop does not turn that result into `Cancelled`.
- `FR-040-AC-4` (examined): Completion before observed cancellation keeps the report-based result; measured memory exhaustion, memory observation failure, output over-limit, unread capture and unconfirmed cleanup keep their existing result/refusal and confirmed teardown rules when cancellation competes. A cancellation observed first and causing stop is not overwritten by later completion or elapsed time.
- `FR-040-AC-5` (examined): A multi-member group sharing one handle stops wholly on its observed cause, returns `BatchCancelled { source, members }`, and classifies no member; another group using a distinct handle retains its own result. Reversing the two cause calls changes the cancelled group's source and not the other group's evidence. Distinct handles never share one launcher.
- `FR-040-AC-6` (examined): The single-run producer constructs an IR `KaniOutcomeKind::Cancelled { source }` from settled execution evidence, and its real call path reaches the FR-030 map. Mapping both IR source variants exhaustively yields the identical `quire_exact::CancelCause` inside QSL's `IncompleteCause::Cancelled`; neither metadata nor a copied enum determines the source.
- `FR-040-AC-7` (examined): A direct `quire-exact` dependency resolves to the same package revision as `qsl-replay`'s exact cause, while all QSL imports remain through `qsl-replay`; no implicit cancellation control, source fallback, or compatibility overload is present.
- `spec/kani/functional/FR-017-kani-execution-evidence.md` (examined): [FR-040](./FR-040-cancellation-producer.md) adds the one launcher-observed cancellation outcome:
- `spec/kani/functional/FR-029-run-outcome-terminal-record.md` (examined): The Kani adapter shall map every `KaniRunOutcome`, with its inconclusive reason or typed
- `spec/kani/functional/FR-030-ir-outcome-terminal-map.md` (examined): Two Kani maps exist and their domains are selected by the producer. [FR-029](./FR-029-run-outcome-terminal-record.md)
- `spec/kani/functional/FR-040-cancellation-producer.md` (examined): id: FR-040
- `spec/kani/matrix/TC-041-ir-outcome-terminal-map.md` (examined): 3. Map `TimedOut`, `ResourceExhausted`, `Cancelled { source: Requested }` and
- `spec/kani/matrix/TC-055-cancellation-producer.md` (examined): id: TC-055
- `spec/kani/matrix/tests.md` (examined): | FR-029 | FR-029-AC-1, FR-029-AC-2, FR-029-AC-4 through FR-029-AC-6, FR-029-AC-8 through FR-029-AC-14 | TC-040 | ✅ Covered (IR-465); `run_terminal_value` in `kani/terminal.rs`, with the `From` conversions in `replay/` from each error the replay path returns, asserted over QSL's own refusal values; the `Inconclusive` terminal value is merged in QSL. FR-029-AC-10 is asserted by `tc_040_a_fault_in_any_replay_wrapper_is_failed`, which builds each fault wrapper from QSL's constructible `InternalFault` (QSL bcca433) |
- `spec/spec.md` (examined): | Kani obligations and execution | FR-015 generation, FR-017 execution, FR-025 subject ABI, FR-028 bounds and ceilings, [FR-034](kani/functional/FR-034-caller-death-ownership.md) planned original-caller lifecycle ownership, [FR-040](kani/functional/FR-040-cancellation-producer.md) planned caller cancellation producer | TC-025, TC-027, TC-036, TC-039, [TC-049](kani/matrix/TC-049-caller-death-ownership.md) planned production guardian verification, [TC-055](kani/matrix/TC-055-cancellation-producer.md) planned cancellation path |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

No defects in this method.
