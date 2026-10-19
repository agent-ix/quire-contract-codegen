---
id: SR-13304
title: "IR-891 PR 462 gap analysis: FR-016-AC-6 required witness decode bounds against TC-026"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@a18d5a991b34b9e2a74c8ea7cc6dbf3d436ad907 vs merge base 58f44780; spec/replay/functional/FR-016-witness-native-replay.md (Inputs, AC-6), spec/replay/matrix/{tests.md,TC-026-witness-native-replay.md}, spec/tests.md, src/replay/** and tests/it/** changed by the PR"
---

# SR-13304: IR-891 PR 462 gap analysis

## Summary

Ticket: IR-891 (FR-016-AC-6). This is a planless gap analysis of PR #462 against FR-016-AC-6 and
the FR-016 Inputs paragraph that AC-6 restates. Plan completion was not assessed.

`quire matrix --scope .` at the head lists FR-016-AC-6 as `tagged` with five binders:

- `src/replay/witness.rs:434`: unit test at both bounds, with a poisoned byte suffix and a
  poisoned extra value;
- `tests/it/skeleton_spine.rs:1063`: `replay_counterexample` and `replay_counterexample_through`.
  It also checks that QSL `input_bytes` does not limit CG decoding;
- `tests/it/kani_obligations_state_frame.rs:793`: `FrameReplay::new`;
- `tests/it/kani_obligations_state_clause_replay.rs:44`: `StateClauseReplayInputs::decode_falsification`,
  with the bound in the parameter and in the field;
- `tests/it/bounded_kani_corpus.rs:494`: `BoundedCorpusCase::replay_counterexample` on real Kani
  output. This test is in the ignored Kani lane, and the reviewer ran it and it passed.

Each test decodes at the exact bound and is refused one past it. The refusal comes before the
poisoned suffix is read, because the same suffix read under a wider bound produces a different
error. Eleven reviewer mutants on the production bound paths were all killed (SR-13303). The
"required" and "at its entry point" parts of AC-6 are met in code. The "no default" part has no
test.

Method note: the diff touches `spec/**`, but only matrix status rows. No requirement or acceptance
text changed. This analysis checked whether the matrix rows are true, and a separate spec-review
was not run.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Two matrix documents still call AC-6 planned, which contradicts the new ✅ Covered row. The TC-026 row in `spec/replay/matrix/tests.md` (line 40) still reads "🚧 Planned; AC-6 bounded-decode controls remain unimplemented". `TC-026-witness-native-replay.md` still says "FR-016-AC-6 and AC-7 are planned" (line 122) and "The caller-supplied byte and value-count decode limits (AC-6) and the charge, counter and limit comparison (AC-7) are planned" (lines 140-142). Fix: update the TC-026 row status and both TC-026 paragraphs to name the five AC-6 tests, and leave AC-7 planned. | spec/replay/matrix/tests.md:40, spec/replay/matrix/TC-026-witness-native-replay.md:122, spec/replay/matrix/TC-026-witness-native-replay.md:140-142 |
| FND-002 | medium | No test backs the "no default" part of AC-6. AC-6 says replay inputs carry both bounds "as non-optional fields with no default", and FR-016 Inputs says "the decode-limit type has no `Default`". Reviewer mutant: adding `Default` to the `WitnessDecodeLimits` derive passes every test, and adding `Default` to `ReplayInputs` would pass the same way. The matrix nonetheless marks AC-6 Covered. Fix: add a `compile_fail` doctest on `WitnessDecodeLimits` (`WitnessDecodeLimits::default()`) and on `ReplayInputs`, or a source-scan test like the FR-024 one, tagged FR-016-AC-6. | src/replay/witness.rs:45-52, src/replay/function.rs:343-347, spec/replay/functional/FR-016-witness-native-replay.md:31-33 |
| FND-003 | low | FR-016 Inputs makes the bounds "required parameters of every witness-decoding entry point". The public `replay_counterexample`, `replay_counterexample_through` and `FrameReplay::new` decode witnesses, but they take the bound from the required field `ReplayInputs.decode_limits`, not from a parameter. `StateClauseReplayInputs::decode_falsification` takes both and uses their minimum. The code is safe, because the field is required with no default. But a reader of the spec would expect a parameter on those three entry points. Fix: reword Inputs to "a required parameter, or a required field of the inputs the entry point receives", or make the paths agree. | spec/replay/functional/FR-016-witness-native-replay.md:31-33, src/replay/function.rs:1151-1156, src/replay/frame.rs:571, src/replay/state_clause.rs:457-462 |

## Verdict

**FAIL: not mergeable as is. A small fix round is needed.** The implementation is correct and
fully mutation-tested across all five decode paths. The downstream consumer list in the PR body is
complete, and no compatibility layer or hidden constant is added. FND-001 means the matrix
contradicts itself about AC-6. FND-002 means one clause of an AC marked Covered has no test.
Both are small fixes: two spec edits and one doctest. FND-003 is wording and can be deferred.
