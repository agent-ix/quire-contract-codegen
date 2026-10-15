---
id: SR-11252
title: "Gap analysis — IR-791 paired routed-scalar runner"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; FR-017-AC-27..31, TC-027, src/kani/run/scalar.rs, src/kani/identity.rs, src/routed/generate.rs, tests/it/kani_obligations.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-017", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/TC-027", type: references }
---

## Summary

Audited the changed routed-scalar proof/native path against FR-017-AC-27 through AC-31 and TC-027 using `quoin matrix` plus source and test inspection. The request-to-receipt chain has a reverse binding gap for `ScalarLimits`: the new request accepts a value, but the retained routed context has no original value to authenticate it against.

## Verdict

**FAIL** — FR-017-AC-28's limit mismatch refusal is not implemented; AC-27 and AC-30 are tagged only by ignored Kani-lane tests, and no run evidence was available in this review.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The API has no authenticated original ScalarLimits value, so altered limits can flow into native execution and the resulting receipt | src/kani/run/scalar.rs:55-67, 255-309 |

## Coverage

- Reconciliation: quoin matrix 0.28.3 and quire matrix 0.36.2 (`quoin matrix --repo . --json`); changed criteria reviewed from computed bindings.
- Plan completion: not assessed
- Changed criteria: FR-017-AC-27 tagged-by-ignored-test; AC-28 tagged; AC-29 tagged; AC-30 tagged-by-ignored-test; AC-31 tagged.
- Evidence: no run evidence available for these criteria.
- Reverse gap: one implementation gap found; no source/test stub found in the changed path.
- Semantic review: skipped (not opted in).

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

Rechecked the public PR delta at `0fcbd23e74427d341b185b228cb0c595b29636cf` against main `2928df58a46928c2a84a225d2891914fc8f08caa`. No new requirement-to-implementation gap. The canonical helper keeps `quire_canonical::sha256` behind `core::canonical` while allowing the proof-content preimage its explicit 256 MiB limit. Package identity remains validated separately from per-harness equality before either child runs. Existing FND-001 remains deferred to IR-773; FND-002's separate TC-027 preflight controls remain fixed by source inspection. No Cargo/Kani or other build/gate evidence was produced in this pass.
