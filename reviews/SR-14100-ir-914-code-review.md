---
id: SR-14100
title: "IR-914 PR #478 code and Rust review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; PR #478 head be53787719c890fccf296c6deeed078dbcc414f7; tests/it/kani_scalar_native_private.rs; FR-017-AC-42"
review_set: subset
---

## Summary

Reviewed the sole changed Rust test in PR #478 against FR-017-AC-42. The cancellation and timeout outcomes are asserted through the real scalar runner with isolated scratch paths. No production code, specification, CI workflow, or vendored material changed. No applicable AssuranceProfile was found under `spec/`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | If `execute_kani_scalar_native` panics before creating `started`, `completed.store` never runs and the scoped watcher loops forever. The test process hangs instead of reporting the panic. Give the watcher a bounded failure path or set completion through a drop guard that runs during unwinding. | tests/it/kani_scalar_native_private.rs:162-183 |

## Verdict

**FAIL** until FND-001 is resolved. The intended slow-start cancellation probe and its `FR-017-AC-42` trace are sound on normal return; the new unbounded wait makes a regression capable of hanging the test suite.

## Coverage

- `FR-017-AC-42` (examined): `class()` is `CompletionRefusal` for `invalid_request`, `artifact_package` and `oracle_stop`, and `EnvironmentFault` for `snapshot_io`, `native_launch`, `dependency_lock`, `native_result`, `snapshot_changed`, `identity_encode` and `codegen_lock`. For `proof_not_verified` it is `CompletionRefusal` for evidence outcome `Falsified`, `CoverUnsatisfied`, and `Inconclusive` `MissingCoverSummary`, `VacuousProof`, `UnwindBoundExhausted` or `FailedWithoutCounterexample` with no `failureCause` or `propert
- Rust lane: inspected watcher lifecycle, `AtomicBool` publication, proof completion, cancellation outcome assertions, test isolation, panic behavior, and the sole changed file.
- Gate evidence in PR metadata reports pre-PR CI and Kani 37/37; this review independently checked the diff and did not rerun full gates.
