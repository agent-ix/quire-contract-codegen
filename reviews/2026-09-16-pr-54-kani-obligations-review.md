---
id: SR-031
title: "PR 54 separate Kani obligations review"
type: SpecReview
analysis: gap-analysis
scope: "PR #54 at 36486e90be4761e3e96e06c3d70d0cafb2a25a0c; FR-015 and TC-025 (slice A); Contract IR FR-036 ACs claimed"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TM-001
    type: references
---
# SR-031: PR 54 separate Kani obligations review

## Summary

One independent review of PR #54: code review, Rust review and gap analysis, limited to the PR
diff. The PR negotiates each requested item and records one outcome per item. It lowers V1
precondition, postcondition and invariant clauses to separate harnesses. It refuses every V2
scalar claim as caller-declared, and it runs harnesses only after checking the installed backend
against the pins in the harness identity. Negotiation, refusals, argument-only assumptions and
the seeded falsification all hold up.

The problem is vacuity. A contract harness turns every precondition that shares its anchor, plus
the invariant's pre-state, into `kani::requires`. Kani assumes those requires. The only
non-vacuity check is each precondition's own `kani::cover!`, run on its own. Nothing checks the
conjunction the contract harness actually assumes. This was reproduced under the pinned Kani
0.67.0: two preconditions that are each satisfiable but contradict each other, plus a
postcondition that is false for every input, gave three `VERIFICATION:- SUCCESSFUL` results with
both covers satisfied. `classify_run` reports all three as `Verified`.

## Verdict

**FAIL**: FND-001 is high.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Contract harnesses can pass vacuously. `render_contract` (src/kani_obligations.rs:1403-1494) emits one `#[kani::requires]` for each assumed precondition, plus `inv(pre)` for invariants, and adds no cover. `classify_run` (src/kani_execution.rs:467-471) returns `Verified` for non-precondition kinds without reading a cover summary. The precondition harness (1384-1401) covers each precondition alone, so it cannot catch a conjunction that nothing satisfies. Reproduced with Kani 0.67.0: `pre_a: x<=5` and `pre_b: x>=10` both anchored to `withdraw`, with `ensures post==-12345`. Both covers were satisfied and `proof_for_contract` passed (0 of 49 failed). Fix: in every contract harness, emit a `kani::cover!` after the contract call (reachable only if the requires and bounds are jointly satisfiable). Require `total>0 && satisfied==total` for every kind in `classify_run`, otherwise return `CoverUnsatisfied`. Add a TC-025 test with jointly unsatisfiable preconditions, and a kani-lane case, that must not report `Verified`. | FR-015 Behavior (vacuity), FR-015-AC-5, FR-036-AC-2 |
| FND-002 | medium | No committed value anchors the backend pins. The kani lane generates harnesses from `installation.observe()` (tests/kani_obligations.rs:1049-1055) and asserts only `kani_version == 0.67.0`. `validate_request` (src/kani_obligations.rs:514-521) accepts any CBMC version, toolchain, triple or executable digest. So `PinDrift` only compares a harness with the machine it was generated on, and no committed constant pins CBMC `6.8.0` or toolchain `nightly-2025-11-21`. The Makefile comment says the lane asserts the pins, but nothing does. Fix: add committed expected pins (at minimum `rust_toolchain` and `cbmc_version` for 0.67.0), refuse other values at negotiation, and assert them in the kani lane. | FR-015-AC-2, Makefile:96-104 |
| FND-003 | low | A harness is reported `Falsified` whenever Kani prints `FAILED` plus any playback, whichever check failed (src/kani_execution.rs:480-486). Unwinding assertions are on (`--unwind` with no `--no-unwinding-checks`), so if the bound runs out and Kani prints a playback, the run is reported as falsified when the correct result is inconclusive. The loop-free fixture never triggers this. Fix: parse the failed check descriptions and classify `unwind` failures as `Inconclusive`. Add a unit case. | FR-015-AC-2, #50 |
| FND-004 | low | Harnesses for one contract can expect different subject signatures. The subject ABI is the union of the parameters of the clauses one harness embeds (`abi`, src/kani_obligations.rs:1169-1224). If a postcondition reads an input that the invariant and preconditions do not, the two harnesses call `subject_path` with different argument lists, and one of them cannot compile. The result is `Inconclusive::NoVerdict`, not a wrong verdict. Fix: build the ABI from every supported item that shares the operation anchor, or refuse the item with a typed reason. | FR-015-AC-1 |
| FND-005 | low | Frames are blamed on the wrong upstream issue. The test matrix and TC-025 `## Blocked` say frames are blocked on quire-specification#76, but #76 adds the operation identity of V2 application nodes (the arithmetic law), not frames. Frames are actually blocked because the IR has no typed frame item: V1 `ClauseKind` has no frame, and FR-014 refuses V2 `state` nodes as `NoFiniteEncoding`. The by-value ABI also cannot express `kani::modifies`. Frames cannot be built from the merged IR, so this is not an implementation gap. Fix: correct the blocker text. | spec/test-matrix.md:76-85, TC-025 |

## Checked and not findings

- **Precondition as `cover!`.** No caller appears in the IR, so a precondition obligation cannot
  be checked at a call site. The harness checks that the precondition is total (an overflow panic
  is reported as `FAILED`) and satisfiable within the IR bounds, and it guards the contracts that
  assume it. That is acceptable once FND-001 closes the gap for conjunctions.
- **Assumptions.** Only `kani::assume` on argument ranges from the IR value types. Result ranges
  sit on the `ensures` side, so a subject whose result is out of range fails. No assumption touches
  results or state after the call (tested in tc_025_assumptions_constrain_only_arguments_to_their_ir_bounds).
- **Runtime pin a04bd47 vs 2d2dd41.** Not a finding for this PR. The a04bd47..2d2dd41 compare
  touches only `src/exact/*` in the runtime (the metered-charges code that FR-014 V2 oracles use).
  The V1 oracles that the verified harnesses embed call only `ContractIdentity`, `ClauseId` and
  `operators::*`, and no V2 harness is emitted. The bump belongs to FR-014 and `RUNTIME_REVISION`
  on main (src/oracle.rs:17 is not changed by this diff).
- **Operation identity of the verified items.** The three verified items are fixture clauses
  FR-200@1 `amount-within-balance` (precondition, `compare less_equal`), `balance-never-grows`
  (postcondition, `compare less_equal`) and `balance-nonnegative` (invariant,
  `compare greater_equal`). All three are bound through `BoundPackage::from_json_bytes`, which
  stores their operators as IR enum variants rather than caller descriptors. Every V2 claim is
  refused as `CallerDeclaredOperation`.
- **Whole-request rejection.** FR-036 Outputs says "only a fully negotiated request may emit".
  Rejecting the whole request when one item is `invalid_request`, while still recording every
  item's outcome, is consistent with that.
- **Counterexample capture.** The falsified evidence keeps the whole playback test, including the
  failed check expression and the concrete values. That is enough for #50.

## Coverage

- FR-015-AC-3..6: backed by tagged tests in tests/kani_obligations.rs. The test matrix marks them
  Covered.
- FR-015-AC-1, AC-2: partly backed (V1 half) and correctly marked Planned. `quire coverage`
  reports 6 of 6 AC rows backed by trace tags for FR-015.
- Gap in in-scope tests: no test runs a contract harness whose requires cannot all hold at once
  (FND-001), and no test checks that the pins match committed values (FND-002).
- Runs: `cargo +1.98.1 test --locked --test kani_obligations -j 4` gave 8 passed, 1 ignored.
  `make kani` gave 1 passed in 55.3s (verified precondition, postcondition and invariant,
  falsified the seeded postcondition, driver drift refused).
- Semantic review done inline for FR-015 at the caller's request.
