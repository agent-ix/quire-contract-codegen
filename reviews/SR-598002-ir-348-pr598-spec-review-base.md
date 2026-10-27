---
id: SR-598002
title: "IR-348 PR #598 spec-review/base (V1 retirement evidence disposition and operator hold)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR #598 head d9ce52c931dbcf0bad43a35d4ff0706705814842 against origin/main e45be9eb (#597); spec/replay/functional/FR-016-witness-native-replay.md, spec/replay/matrix/TC-026-witness-native-replay.md, spec/replay/matrix/tests.md, spec/kani/functional/FR-041-claimed-module-proof-gate.md, spec/kani/matrix/TC-056-claimed-module-proof-gate.md, spec/kani/matrix/tests.md, spec/kani/non-functional/NFR-006-real-kani-lane-gating.md, spec/spec.md; live code read: scripts/kani_claims.json, scripts/cg_gates.rs, tests/it/kani_generation_claims.rs, tests/it/kani_generation.rs, tests/it/kani_argument_order.rs, tests/it/kani_witness_join.rs, tests/it/kani_obligations_state_clause_replay.rs, src/replay/witness.rs, src/kani/generate/clause/v2.rs, src/kani/generate/frame.rs; IR-348"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-041
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/NFR-006
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-026
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-056
    type: reviews
---

## Summary

Ticket: IR-348. This review checks PR #598 against merged #597 and the planner and operator
ruling given in the brief:

- Legacy V1 is deleted now and Kani proof work is deferred.
- Only the V1-sourced `scripts/kani_claims.json` module claim may be held, with the exact reason
  and expiry.
- The held count is reported separately, and the gate still fails on any non-held defect.
- IR-1047 owns the V2 replacement. It exists in Linear (Backlog).

Facts measured on the live tree:

- `scripts/kani_claims.json` has exactly one module row, `kani_fr_003_4_proof_boolean_transition_module`.
- `tests/it/kani_generation_claims.rs:463` builds that row's source from
  `kani_generation::guard_sources()[0]`. `tests/it/kani_generation.rs:367` labels that source
  `"v1 bundle"`. The row is V1-sourced, as the PR says.
- The `Claim` struct uses `deny_unknown_fields` and has no hold field.
- `scripts/cg_gates.rs` prints only `result= ran= expected= elapsed= kani= tree=`. It requires the
  claim test to be listed once and to report `... ok` once.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The FR-016-AC-8 evidence inventory leaves out a surviving V2 control. FR-016 Status, TC-026 and the replay matrix say only the `witness.rs` unit tests survive and that a V2 producer must still supply persisted argument bindings. But `state_clause_kani_playback_adapter_refuses_wrong_evidence` is tagged FR-016-AC-8 and uses the V2 `generate_state_frame_obligations` producer, which survives V1 deletion. The record should say what partial V2 evidence this test supplies and what it does not (emission order and real-Kani decode). | spec/replay/functional/FR-016-witness-native-replay.md:158-166; spec/replay/matrix/TC-026-witness-native-replay.md:158-165; spec/replay/matrix/tests.md:14; tests/it/kani_obligations_state_clause_replay.rs:144-170 |
| FND-002 | medium | The spec describes the hold as current state, but nothing implements it and no ticket owns building it. TC-056 Status says the V1 declaration and fixture "are operator-held". The hold needs a `held`/reason/expiry field that `deny_unknown_fields` currently rejects, plus gate printing and counting that `cg_gates.rs` lacks. IR-1047 owns only the V2 replacement. No artifact names the ticket that implements the hold, or requires it to land in or before the V1 deletion PR. Without it, deleting `guard_sources()[0]` breaks the claim test, and with it the gate's `claim_run` requirement. | spec/kani/matrix/TC-056-claimed-module-proof-gate.md:77-82; spec/kani/matrix/tests.md:80; scripts/kani_claims.json:1-15; tests/it/kani_generation_claims.rs:41-49,463; scripts/cg_gates.rs:393-418 |
| FND-003 | low | The spec does not say that the hold makes the module gate vacuous for now. The held row is the only declared claim, so the non-held claim set and its computed shared-helper set are both empty. "Run every non-held claim and shared-helper control" then runs no proof or mutation. The text should say that while the hold is active the module lane discharges zero claims, and say what the claim test must still do to report `ok`. | spec/kani/functional/FR-041-claimed-module-proof-gate.md:61-72; spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:44-51; scripts/kani_claims.json:2-13 |
| FND-004 | low | `spec/spec.md` drops the pointer to planned ACs. The registry rows for `kani/generate/{negotiate,...}` and `kani/generate/clause/v2` no longer say "V2 contract-clause FR-015-AC-38 to AC-49 remain PLANNED (IR-489)". The kani matrix still marks AC-38 through AC-49 Planned. The live postcondition text is accurate, but without the pointer a reader could take "V2 ... postcondition ... live" to mean the V2 contract-clause criteria are delivered. | spec/spec.md:130,133; spec/kani/matrix/tests.md:38 |

## Verdict

Changes requested at low severity (FND-001 and FND-002 are medium). What the PR gets right:

- **AC text.** FR-016-AC-8, FR-041-AC-1 through AC-5 and NFR-006-AC-19 are byte-unchanged.
  None is marked fulfilled.
- **FR-016-AC-8.** It moves from the Covered row to its own row, marked Partially backed and
  Open. AC-9 through AC-23 keep their Covered row. The tagged V1 tests in
  `kani_argument_order.rs` and `kani_witness_join.rs` trace only AC-8, so no other criterion
  loses credit.
- **FR-041 gate.** It is kept, not disabled. FR-041, TC-056 and NFR-006 all require every
  non-held control to run and the gate to fail on any non-held defect.
- **Hold scope.** Only the one measured V1-sourced row may be held. A hold on another row, or one
  missing its reason or expiry, is invalid.
- **Reason and expiry.** Both match the operator ruling verbatim in FR-041, TC-056 and NFR-006.
- **Held count.** It is kept separate, and the hold earns no proof credit.
- **`spec.md` registry.** It correctly describes `{clause,v1_bundle}` as live before deletion and
  retiring under step 4f. The V2 precondition and postcondition and the open V2 invariant
  (IR-266) match `clause/v2.rs`.

## Coverage

```yaml
scope:
- {id: FR-016-AC-8, path: spec/replay/functional/FR-016-witness-native-replay.md, role: examined, excerpt: "A witness schema binds the obligation's persisted argument bindings, in the order the obligation persists them, position for position to the harness's symbolic arguments, and each decoded value is named by the binding at its position; a binding that is not an argument is refused with a typed schema refusal that reports no failure and is none of the replay results."}
- {id: FR-016:Status, path: spec/replay/functional/FR-016-witness-native-replay.md, role: examined, excerpt: "FR-016-AC-8 remains Open and partially backed after the 2026-10-27 IR-348 retirement ruling."}
- {id: TC-026:AC-8-paragraph, path: spec/replay/matrix/TC-026-witness-native-replay.md, role: examined, excerpt: "AC-8 is therefore partially backed and Open until a genuine V2 producer persists argument bindings and a matching emitted harness is checked for emission order and real-Kani decode."}
- {id: replay-tests.md:FR-016-rows, path: spec/replay/matrix/tests.md, role: examined, excerpt: "FR-016-AC-1 through FR-016-AC-5, FR-016-AC-9 through FR-016-AC-23 | TC-026 | Covered; FR-016-AC-8 | TC-026 | Partially backed, Open under IR-348"}
- {id: FR-041-AC-1, path: spec/kani/functional/FR-041-claimed-module-proof-gate.md, role: examined, excerpt: "A nonempty reviewed declaration is checked against the current compiled-module census and the merge-base declaration. An absent compiled module, a removed claim, and an empty list each name the claim or condition and fail; adding a compiled claim does not erase an older one."}
- {id: FR-041-AC-2, path: spec/kani/functional/FR-041-claimed-module-proof-gate.md, role: examined, excerpt: "With two claimed modules sharing one src/lib.rs, a SUCCESS location inside only the first and an UNREACHABLE check inside the second reports the second unreached and fails."}
- {id: FR-041-AC-3, path: spec/kani/functional/FR-041-claimed-module-proof-gate.md, role: examined, excerpt: "The current build's transitive expectation/subject reachability yields the shared-helper intersection; the gate fails when the checked-in generated evidence omits or adds a helper"}
- {id: FR-041-AC-4, path: spec/kani/functional/FR-041-claimed-module-proof-gate.md, role: examined, excerpt: "Each claimed module and computed shared helper has a positive unmodified real-Kani proof and a source mutation that compiles, executes, and reports a failed proposition check."}
- {id: FR-041-AC-5, path: spec/kani/functional/FR-041-claimed-module-proof-gate.md, role: examined, excerpt: "A stubbed executor, predetermined outcome or build-only run produces no qualifying module evidence and cannot pass the gate."}
- {id: FR-041:hold-bullet, path: spec/kani/functional/FR-041-claimed-module-proof-gate.md, role: examined, excerpt: "only the V1-sourced module claim row named by scripts/kani_claims.json may be marked held in that reviewed declaration."}
- {id: FR-041:Status, path: spec/kani/functional/FR-041-claimed-module-proof-gate.md, role: examined, excerpt: "Open, with only the V1-sourced module claim operator-held under IR-348."}
- {id: NFR-006-AC-19, path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md, role: examined, excerpt: "On a clean current candidate, make kani-gate runs FR-041's module census and real mutation controls in its one required lane, and a failure of any claim or shared-helper control makes the gate exit non-zero."}
- {id: NFR-006:hold-bullet, path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md, role: examined, excerpt: "the one required lane shall print that row's identity, reason and expiry and count it as held separately from ran and expected."}
- {id: TC-056, path: spec/kani/matrix/TC-056-claimed-module-proof-gate.md, role: examined, excerpt: "The held V1 row remains Open, and all non-held controls stay required."}
- {id: kani-tests.md:FR-041/NFR-006/TC-056-rows, path: spec/kani/matrix/tests.md, role: examined, excerpt: "Open (IR-19), with only the V1 claim row operator-held under IR-348"}
- {id: spec.md:kani-registry, path: spec/spec.md, role: examined, excerpt: "Live on the measured pre-deletion source base, but IR-348 retires the V1 bundle, clause arm, V1-only readers and public operation immediately at AD-004 step 4f"}
- {id: TC-026, path: spec/replay/matrix/TC-026-witness-native-replay.md, role: context_only, excerpt: "FR-016-AC-1 through AC-5 and AC-13 are implemented and tested in the default suite through replay_counterexample"}
```

## New findings (disposition pass 1)

Re-checked at head ad57b91cb39b7ad610ad191af92c15537eecdf9a against the previous head d9ce52c9.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | The new FR-016-AC-25 disposition misdescribes the evidence it retires. FR-016 Status says the file's "stdout pass-through assertion alone did not compare a retained counterexample against the complete independent stdout fence", and TC-026 says it "never independently compares" them. But the ignored `tc_026_installed_kani_falsification_is_bound_to_process_stdout` compares the retained counterexample byte-for-byte with the observed stdout assertion block (cover blocks excluded) and rejects a valid post-classification substitute. That is V1-producer AC-25-shaped evidence being lost, not pass-through only. Three of the four tagged tests and the `KaniStdoutCapture`/`assertion_playback` helper have no V1 dependency. Only the falsification test uses V1 `supported_contract_harnesses` (`ObligationItem::BoundClause`), and the helper is what IR-604 needs for a surviving harness. The no-credit outcome (Planned/Open) is correct. | spec/replay/functional/FR-016-witness-native-replay.md:171-177; spec/replay/matrix/TC-026-witness-native-replay.md:171-176; spec/replay/matrix/tests.md:17; tests/it/kani_stdout_capture.rs:51-66,145-284; tests/it/kani_obligations.rs:421-433 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-016 Status, TC-026 and the replay matrix AC-8 row now name the surviving V2 `state_clause_kani_playback_adapter_refuses_wrong_evidence`, say what it checks (persisted draw order against playback, mismatched assertion evidence) and what it does not (emitted symbolic argument order, real-Kani decode). AC-8 stays Partially backed/Open, and the TC-026 summary row is lowered to Partial. |
| FND-002 | still-open | FR-041 Status, TC-056 Status and its summary row, and the NFR-006 AC-19 note now say the hold is PLANNED and owned by IR-348's immediate V1 deletion code slice, in or before its deletion PR. But two `spec/kani/matrix/tests.md` rows still describe it as current state. Line 80 (FR-041): "with only the V1 claim row operator-held under IR-348". Line 91 (NFR-006 AC-19): "only the V1 claim row is operator-held, printed with reason/expiry and counted separately". `scripts/kani_claims.json`, the `Claim` struct and `cg_gates.rs` still have no hold. |
| FND-003 | fixed | FR-041 hold bullet and Status, TC-056 step 6 and Expected Results, the TC-056 summary row and the NFR-006 AC-19 note now say that while the sole current claim is held, zero claimed modules, proofs or mutations are discharged, and that a passing claim test validates declaration/reporting only, with no FR-041 proof credit. |
| FND-004 | fixed | `spec/spec.md` adds the registry row "V2 clause claim and proof census / FR-015-AC-38 to FR-015-AC-49 / PLANNED (IR-489, AD-004 steps 4c and 4d); the V1 deletion under IR-348 does not discharge these criteria." This matches `spec/kani/matrix/tests.md:38`. |
| FND-002 | fixed | Round 2, head bc82713c. The two remaining `spec/kani/matrix/tests.md` rows now describe the hold as PLANNED. Line 80 (FR-041): "the hold for only the V1 claim row is PLANNED under IR-348's immediate V1 deletion code slice, in or before its deletion PR ... A future valid hold earns no proof credit". Line 91 (NFR-006 AC-19): "the V1 claim-row hold is PLANNED in IR-348's immediate V1 deletion code slice; once implemented, the gate prints its reason/expiry and separate count". FR-041 Status, TC-056 Status and summary row, and the NFR-006 AC-19 note already said PLANNED and name the implementing slice. No matrix row or Status still presents the hold as implemented. This agrees with the live tree, where `scripts/kani_claims.json` has no hold field. |
| FND-005 | fixed | Round 2, head bc82713c. FR-016 Status, TC-026 and the replay matrix AC-25 row and TC-026 summary row now say that the ignored `tc_026_installed_kani_falsification_is_bound_to_process_stdout` uses a V1 `BoundClause` harness. They say it compared the retained counterexample byte-for-byte with the first eligible assertion playback in independently captured stdout, excluded cover blocks, and rejected a valid post-classification substitute, and that this V1 evidence retires without credit. They also say the observing-launcher helper and the other three AC-25-tagged tests survive. All of this checks against `tests/it/kani_stdout_capture.rs` at bc82713c. The four `#[trace("FR-016-AC-25", ...)]` tests are at lines 147, 166, 190 and 285. Only the test at line 193 calls `supported_contract_harnesses` (`ObligationItem::BoundClause`, `tests/it/kani_obligations.rs:421-433`). `assertion_playback` takes the first block with `concrete_playback_run` that is not a cover block, and the test asserts `observed.as_bytes() == counterexample` and `!bound_to_stdout(&replacement)` after showing that the replacement parses, decodes and replays `Reproduced`. AC-25 stays Planned/Open under IR-604. |
