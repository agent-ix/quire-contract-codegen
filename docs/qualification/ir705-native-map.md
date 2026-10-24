# IR-705 native CG integration map

The original CG profiles use the existing `Int[0,9]` increment fixture, not a
new bounded model. `tests/exact_scalar_support/package.rs` owns that QSL
package and checked claim. `tests/it/kani_obligations.rs` obtains the original
public routed scalar harness through `routed_scalar_increment_with_artifacts()`.

| Required result | Current source and invariant | Native integration |
| --- | --- | --- |
| TC-027 healthy | `routed_scalar_increment()`, `tc_027_a_routed_scalar_harness_verifies()` and `run_scalar_under_real_kani()`; the original i64 draws are `[0,9]` and `[1,1]`, semantic result `[0,9]`. | `src/kani/generate/scalar.rs` must validate the whole Add operand interval with `NativeIntegerPlan`, render `NativeInteger` inputs and kernel operation, and keep the original independent expected expression, cover, Cadical solver, unwind and 16 GiB ceiling. |
| TC-027 bound mutant | The same public harness; only its independent result bound changes. | Retain both original draw intervals and the original native oracle. The proof must falsify the changed bound, and the existing scalar decoder must return original-domain values. |
| TC-025 arithmetic mutant | `tests/it/kani_scalar_native_private.rs` obtains the same public harness, then privately seals one Add-to-Subtract source fault for the existing bounded executor. | Keep the original source identity, semantic domain, independent Add expectation and Meter limits. The compiled mutant must falsify and replay a decoded original-domain witness. |

The QSL V2 clause qualification uses `tests/state_frame_support/native_twin.rs`
and `tests/it/kani_obligations_state_frame.rs`. Its current path is
`src/oracle/clause.rs` into `src/kani/generate/clause/v2.rs`. Native clause
selection must preserve its admitted `[0,1000]` parameter domain and all
intermediate predicates, with the genuine harness's existing solver and unwind.
That route is a separate integration slice from the three scalar proofs.

The published `quire-exact` API is `NativeInteger`, `NativeIntegerInterval`,
`NativeIntegerOperation::{Add,Subtract,Multiply,Negate}`,
`NativeIntegerPlan::new`, `plan.admit`, and
`evaluate_native_integer_arithmetic`. `NativeIntegerPlan` derives the complete
capacity from both operand intervals. Multiply refuses operands outside
signed32 even if the actual draw fits. `admit` checks payload membership before
the real Meter runs. The evaluator charges operands, arithmetic, semantic
domain, and result retain in that order, preserving refusal and cancellation.
`Integer::as_big` remains the arbitrary-precision refinement observation for
the general path. CG must call this owner API directly and retain typed
generation-time refusal separately from evaluator refusal.

Minimal CG implementation files for the scalar route: `src/kani/generate/scalar.rs`,
`src/kani/identity.rs`, `src/kani/generate/outcome.rs`, and
`src/kani/run/scalar.rs`, plus `src/core/native_integer.rs` for selected policy
metadata, `tests/it/kani_obligations.rs` for the healthy and bound profiles,
and the private library test fixture for the arithmetic fault injection.
The V2 clause route additionally touches `src/oracle/clause.rs`, its native
selection module, `src/kani/generate/clause/v2.rs`, and
`tests/it/kani_obligations_state_frame.rs`. The latter test file overlaps an
active unrelated PR, so only IR-705-specific hunks belong here. The positive
helper-mutant decoder test traces typed decoding AC-1 rather than byte/value
limit AC-6, correcting old review finding FND-005. Its real playback is still
pending the coordinated Kani run.
