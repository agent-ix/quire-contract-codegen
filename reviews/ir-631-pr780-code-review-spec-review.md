---
id: SR-17940486709564847047646
title: "IR-631 PR #780 scalar settlement spec-review of edited text"
type: SpecReview
analysis: base
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #780 @ 630e1e39; IR-631; spec/replay/functional/FR-032-routed-scalar-replay-binding.md (statuses/prose), spec/replay/matrix/TC-047-routed-scalar-replay-binding.md (Description), spec/replay/matrix/tests.md (FR-032 rows), spec/kani/functional/FR-029-run-outcome-terminal-record.md (scalar paragraph)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-047
    type: reviews
---

## Summary

Ticket: IR-631. PR agent-ix/quire-contract-codegen#780, branch code/ir-631-scalar-settlement, head 630e1e3941b517477bfd4daa0a588d1d375e248e (one commit), reviewed in a detached worktree /home/peter/dev/worktrees/IR-631-review, scope git diff origin/main...HEAD (12 files). Authoritative spec on main: FR-032-AC-7/8/20..25, TC-047 steps 10-14, FR-029 scalar paragraph, reviews/ir-631-pr777-spec-review-*. Reviewer runs (thermal budget: one focused run plus one mutation control): `cargo test --locked --test it scalar_settlement` with a temporary reviewer probe test: 10/10 PR tests pass and the probe confirmed QSL refuses the bounded-addition source; mutation control (Inconclusive run / TimedOut IR kind moved into the settled arm) with filters scalar_settlement and tc_047_bounded_addition: 12/12 still pass, so the mutant survives. Both edits reverted; worktree clean. No Kani, no make ci. quire matrix tags every flipped AC; quire validate passes on the four edited spec files.

## Verdict

One medium spec/ticket mismatch for the IR lead to route (FR-032-AC-8 bounded-addition half unsatisfiable against QSL's checker); not caused by this PR's code. The edited status text is otherwise consistent.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The bounded-addition half of AC-8 cannot be delivered as written. Reviewer probe at the locked qsl-replay 434869e: compile_package of `function subject using v(x: Int[-1000, 1000], y: Int[-1000, 1000]): Int[-1000, 1000] pure { x + y }` returns Recompile(Check{ cause: Unproved(Range{ required: [-1000,1000], proved: [-2000,2000] }) }); with result Int[-2000, 2000] it compiles, but then no admitted pair has an exact result outside the result range. So no QSL-checked package can carry an admitted operand pair whose exact sum is outside its declared result range, and FR-032 (line ~447) requires the QSL source counterpart and forbids a hand-edited emitted package. The coder's claim is true. The PR's AC-8 status says the native/QSL half stays 'PLANNED', which implies a reachable future; it is unsatisfiable by construction (QSL's checker behaves correctly). This is a spec/ticket mismatch for the IR lead to route as an FR-032/TC-047 spec amendment: restate the out-of-result-range case as the descriptor-level harness/decoder check it can be (the existing kani_obligations.rs::scalar_package descriptor), or retire it; meanwhile mark it blocked on that amendment rather than PLANNED. Not a QSL defect. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:402, 441-450 |

## Coverage

Spec-review restricted to the spec text the PR edits: FR-032 status paragraph (line 37), AC-7/AC-8/AC-20..AC-25 status cells, the implementation-status paragraph (line 504), the Intent table row, TC-047 Description, tests.md rows for AC-7/8, AC-12/14 and AC-20..25, and the FR-029 scalar paragraph (line 311). The normative AC texts are unchanged from main (only status prefixes changed), verified by diff. Statuses agree with each other across FR-032, TC-047, tests.md and FR-029 (AC-20..25 implemented, AC-7/AC-8 partial, AC-12/14/19 planned). Accuracy issues of the AC-8 wording and the AC-22/AC-24 flips are recorded in the gap-analysis SR of this PR, not repeated here. The coder's QSL claim was verified by probe (FND-001). quire validate passes on the four edited files.

## Examined units and bindings

```yaml
scope:
- id: FR-032 status paragraph (line 37)
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: 'The full proof-driven replay pipeline remains Planned under IR-631. CG''s

    original scalar body/limits retention, checked plan completion, native

    observation comparison and the full-claim report check

    (`CompletedScalarReplay::settlement`, which replaced `settle`, FR-032-AC-20),

    the typed settlement and its pairing with the Kani outcome (FR-032-AC-20 to

    AC-25) are implemented. Native tests exercise the actual QSL

    operator route; they do not establish execution of the same proved artifact'
- id: FR-032 implementation status (line 504)
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: 'Status is stated per criterion. AC-1 to AC-6 and AC-15 to AC-18 are **Partial**: the merged

    native tests (#702, #741, #757, #481; IR-1184) assert the CG-side checks each row names, and the

    rest of each row stays Planned. AC-20 to AC-25 are **Implemented** (IR-631); AC-7 and AC-8 are **Partial**; AC-12, AC-14 and AC-19 are

    **Planned**; AC-9 to AC-11 and AC-13 are retired. The IR-648 and IR-940 gates are closed (both'
- id: FR-032 bounded-addition fixture prose (line 441)
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: 'The emitted increment fixture''s result range is `Int[0,10]`, as

    [FR-022](../../routed/functional/FR-022-routed-generation.md) AC-15 and

    `tests/it/routed_generation.rs` record. It supplies no result-range-refusal witness. The separate

    addition descriptor already accepted by `tests/it/kani_obligations.rs::scalar_package` has

    operand/result bounds `[-1000,1000]`; `(600,600)` giving `1200` illustrates a reachable

    out-of-result-range exact result, without prescribing Kani''s chosen witness. The check derives

    its expected exact result from the actual retained operand pair. Its QSL source counterpa...'
- id: FR-032 Settlement Contract terminal table
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: context_only
  excerpt: '| Settlement | Source (QSL-owned name) | `TerminalValue` |

    |---|---|---|

    | `Parity` | `OperatorParityResult::Diverged` | `Failed` |

    | `Parity` | `OperatorParityResult::GeneratedFault` | `Failed` |

    | `Parity` | `OperatorParityResult::Agrees` | `Inconclusive(ScalarAgrees)` |

    | `Parity` | `OperatorParityResult::RefusedInput` | `Inconclusive(ReplayRefused)` with `invalid_runtime_input` |

    | `Parity` | `OperatorParityResult::Incomplete` | `Incomplete(ResourceExhausted)` |

    | `Parity` | `OperatorParityResult::Refused`, non-fault | `Inconclusive(ReplayRefused)` with the refusal''s catalog code |

    | `P...'
- id: TC-047 Description
  path: spec/replay/matrix/TC-047-routed-scalar-replay-binding.md
  role: examined
  excerpt: 'Planned public-consumer coverage of

    [FR-032](../functional/FR-032-routed-scalar-replay-binding.md), distinguishing lowering parity

    failure from source-property refutation. Partial native evidence exists (IR-1184: #702, #741,

    #757, #481): `scalar_playback.rs`, `scalar_literal_operands.rs`, `scalar_replay_binding.rs` and

    `scalar_replay_residuals.rs` drive the real routed generation, the real QSL operator facade and

    a compiled generated artifact, without a Kani run. `scalar_settlement.rs` implements the

    settlement steps 10 to 14 (IR-631). Steps 1, 2 and 7 to 9 remain Planned, as does any real

    ...'
- id: TC-047 step 13
  path: spec/replay/matrix/TC-047-routed-scalar-replay-binding.md
  role: context_only
  excerpt: "13. Setup defects (FR-032-AC-24). Produce a real `ScalarBuildError` of each reachable kind: a\n    four-byte operand (`Playback`), a changed body (`BodyMismatch`), changed artifacts\n    (`Artifacts`), a mutated observation (`Observation`), a changed `harness.rust` so that\n    `scalar_replay_body` returns `GeneratedArtifactChanged` (`Authority`, tamper), a routed\n    Boolean `Implies` scalar harness (the `boolean_implies_scalar` fixture of\n    `kani_obligations.rs`), whose `scalar_replay_body` returns `ScalarAuthorityError::Unavailable`\n    (`Authority`, unavailable capability), an `OriginalS..."
- id: TC-047 Expected Results FR-032-AC-8
  path: spec/replay/matrix/TC-047-routed-scalar-replay-binding.md
  role: context_only
  excerpt: '| FR-032-AC-8 | Increment emission/result range is observed; the separate bounded-addition check derives the exact result from actual retained in-domain operands and asserts it is outside `[-1000,1000]`, with correct refusal/harness-defect agreement rather than source violation | Use a hand-built function harness, assume `(600,600)` instead of reading actual playback, substitute increment as the unreachable refusal case, relabel expected refusal as source falsification or accept wrong emitted fixture ranges |'
- id: tests.md FR-032 AC-7/AC-8 row
  path: spec/replay/matrix/tests.md
  role: examined
  excerpt: '| FR-032 | FR-032-AC-7, FR-032-AC-8 | TC-047 | 🚧 Partial (IR-631): the conversion over genuine QSL reports (AC-7) and the `x + 1` and bounded-addition fixtures (AC-8) are tested without Kani. A real falsified Kani run (the driver''s E9 run is IR-610) and native observation of the bounded-addition fixture, whose QSL source is refused by QSL''s checker, remain Planned. |'
- id: tests.md FR-032 AC-20..25 row
  path: spec/replay/matrix/tests.md
  role: examined
  excerpt: '| FR-032 | FR-032-AC-20 through FR-032-AC-25 | TC-047 | ✅ Covered (IR-631): `ScalarSettlement`, `CompletedScalarReplay::settlement` (which replaced `settle`), `ScalarBuildError::setup_settlement`, the scalar pair functions, the missing-settlement checks and the typed-seam scan are asserted in `tests/it/scalar_settlement.rs`; the settlement/terminal pairing is a trusted-driver boundary (FR-032 trust boundary). |'
- id: tests.md FR-032 AC-12/AC-14 row
  path: spec/replay/matrix/tests.md
  role: examined
  excerpt: '| FR-032 | FR-032-AC-12, FR-032-AC-14 | TC-047 | 🚧 Planned (IR-631): no test is tagged to these criteria. |'
- id: FR-029 scalar paragraph (line 311)
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`ReplaySettlement` input and its MissingSettlement/UnexpectedSettlement rules remain scoped to

    source-predicate replay. The composite route requires a distinct typed same-claim settlement input;

    AC-15 cannot reject that input as an ordinary unexpected replay settlement.

    The routed scalar route (FR-032-AC-20 to FR-032-AC-24, implemented under IR-631) likewise uses a

    distinct typed `ScalarSettlement` and its own pair entry points, which return the same

    `TerminalPairError`; `ReplaySettlement` and its six readings are unchanged.'
- id: FR-032-AC-8
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: 'PARTIAL (IR-631): `scalar_settlement.rs` runs the `x + 1` fixture (input `Int[0,9]`, result `Int[0,10]`) through routed generation, plan, native observation and QSL, and asserts that its operand ranges reach only `[1,10]`, so it has no out-of-result-range value; `kani_obligations.rs` asserts the `[-1000,1000]` bounded-addition descriptor and derives the exact sum `1200` of the decoded `(600,600)` playback outside the result range. Native observation and QSL parity of the bounded-addition fixture stay PLANNED, because QSL''s checker refuses its source (`Unproved(Range)`), so no QSL-emitted pa...'
bindings: []
```

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | The quoted source `function subject(x: Int[-1000, 1000], y: Int[-1000, 1000]): Int[-1000, 1000] { x + y }` is not the source QSL checked and refused: the compiled form (as in tests/it/scalar_playback.rs::replay_fixture and the reviewer probe) is `function subject using v(...): Int[-1000, 1000] pure { x + y }` under the language/profile header. As quoted it may fail to parse rather than be refused with Unproved(Range). Quote the real form or say 'the equivalent QSL function'. Pure wording. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:445-447 |
| FND-003 | low | The Description says steps 1, 2 and 7 to 9 remain Planned, but step 2's restated descriptor-level bounded-addition check is now delivered by kani_obligations::tc_047_bounded_addition_playback_derives_an_out_of_result_range_exact_value. Say that step 2's real-Kani run stays Planned while its descriptor-level check is implemented. Pure wording. | spec/replay/matrix/TC-047-routed-scalar-replay-binding.md:22-24 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 (35e5f198): FR-032-AC-8, its prose paragraph, TC-047 step 2 and the AC-8 expected row are restated: the out-of-result-range case is the descriptor-level decoder/harness check on kani_obligations::scalar_package; QSL's correct Unproved(Range) refusal is cited (no version/hash) and native observation, QSL parity and harness-assertion mutation of that case are explicitly neither required nor claimed. AC-8 stays PARTIAL. Consistent across FR-032, TC-047, tests.md; FR-029 untouched by the change; make spec passes with no grammar warning on these files. |

## Disposition round 1 coverage

Round 1 against fix commit 35e5f198 (rebased onto origin/main dbb88103; original commit now 08c6a332). Checked git diff 08c6a332..35e5f198 over src, tests and spec. One cargo job: the combined mutation control described in the Dispositions rows, filters scalar_settlement and tc_047_bounded_addition: 9 passed, 2 failed exactly at the mutated assertions; reverted, worktree clean, target removed. No regression: no stale `settle(` call in src/tests/spec/docs; quire matrix tags FR-032-AC-7, AC-8 and AC-20..AC-25; make spec passes with no grammar warning on FR-032, TC-047, tests.md or FR-029. The committed reviews/ir-631-pr780-code-review-*.md at 35e5f198 were byte-identical to the review-pass SR files before this section was appended.
