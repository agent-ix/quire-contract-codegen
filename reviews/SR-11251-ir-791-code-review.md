---
id: SR-11251
title: "Code review — IR-791 paired routed-scalar runner"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; src/kani/run/scalar.rs, src/kani/identity.rs, src/kani/run/execute.rs, src/kani/run/namespace.rs, src/routed/generate.rs, tests/exact_scalar_support/package.rs, tests/it/kani_obligations.rs, FR-017-AC-27..31, TC-027"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-017", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/TC-027", type: references }
---

## Summary

Reviewed the full IR-791 code diff at `dca7ef47c16674d9f177987b007d4daa102f0860` against merged FR-017/TC-027 and the IR-791 ticket. The runner stages and measures the proof/native inputs, but it does not bind the caller-supplied `ScalarLimits` to an original retained limit context before returning a receipt.

## Verdict

**FAIL** — a caller can receive a receipt for native execution under limits that were never authenticated as the original request limits.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Scalar limits are caller supplied and never checked against an original retained context, so a mismatched native meter can be attested in the receipt | src/kani/run/scalar.rs:55-67, 255-309 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | IR-773 owns comparing CG request and receipt limits to its private original; revised FR-017-AC-28 assigns CG only supplied-to-meter-used-to-receipt equality and defines no CG provenance mismatch outcome. At dca7ef4, execute_native passes the request limits through limits_array to the native adapter, which uses those values to initialize Meter, and receipt construction stores request.scalar_limits. TC-027's result_units=0 control observes Incomplete.limit=0 and zero result-unit consumption, confirming the meter used the supplied zero limit. |
| FND-002 | fixed | The test now mutates artifact path and bytes, scalar identity, oracle symbol/source, operand count, and bounds; after each typed refusal it asserts the fake launcher marker is absent. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | TC-027 did not independently exercise all required preflight mismatch classes and prove refusal before launcher invocation | tests/it/kani_obligations.rs:2668-2829 |

## Delta disposition pass 3

Rechecked the public PR delta at `0fcbd23e74427d341b185b228cb0c595b29636cf` against main `2928df58a46928c2a84a225d2891914fc8f08caa`. No new code-review or Rust finding. `KaniScalarObligationHarness` equality now compares per-harness public values and generator-origin seals while excluding the whole-routed-package seal; `validate_request` separately calls `matches_artifact_package` before execution, so this exclusion does not bypass package provenance. The limit-provenance finding remains deferred to IR-773, and the TC-027 preflight-controls finding remains fixed. The manifest `let-else` refusal is preceded by a required-path guard that already returns typed `InvalidRequest` when `Cargo.toml` is absent; the new branch is unreachable but does not weaken refusal. No build or gate was run in this review-only pass.
