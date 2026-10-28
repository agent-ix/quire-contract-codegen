---
id: SR-619001
title: "IR-1090 CG #619 code review of collection launcher evidence control"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen#619; IR-1090; tests/it/kani_obligations_symbolic_domains.rs; src/kani/run/execute.rs; src/kani/run/launch.rs; FR-015-AC-100; TC-025"
review_set: subset
---

## Summary

Independent code and Rust review of CG #619 at 12dd264824f5c9482dcb5b3dbb17ff33d2fa9401. The new control compares a stand-in launcher's received argv with the persisted collection identity's options and the returned run evidence. No applicable AssuranceProfile was found under spec/.

## Examined units

- FR-015-AC-100 (examined): The identity records every symbolic collection domain and the solver and complete ordered option vector, and FR-017 run evidence records the solver kind, unwind and every option used by the installed backend in invocation order.
- TC-025 (examined): Assert that the evidence argument vector contains exactly the `kani` subcommand, the harness identity's options in order and the report-export flags in order, and that its solver kind and unwind equal the identity's.
- Collection launcher test (examined): `collection_run_evidence_preserves_every_ordered_identity_option` at `tests/it/kani_obligations_symbolic_domains.rs:1455-1530`.
- Production launch/evidence path (examined): `launch_command`, `run_single`, and `KaniExecutionEvidence` at `src/kani/run/execute.rs`; `run_bounded_launcher` at `src/kani/run/launch.rs`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new test is enabled on every Unix platform although bounded execution requires Linux procfs and pidfd support, so it fails before the probe launches on macOS or BSD. | tests/it/kani_obligations_symbolic_domains.rs:1457 |

## Verdict

CONDITIONAL. The option-vector comparison can fail on omission, reordering, or substitution: the shell stand-in receives the real invocation, and the test compares its argv with both the persisted identity and returned evidence. Solver and unwind are checked. Gate configuration is unchanged. Restrict the test to Linux before the final gate. The focused Cargo test was started but stopped before compilation finished because its `--exact` selector omitted the module path; no test result is claimed.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At final review head 404d3a0711a9c3a9cf5d9cc1a822e4caba53c866, the test is gated by `#[cfg(target_os = "linux")]`, matching the bounded runner's documented Linux prerequisites. |

## Current verdict

PASS at 404d3a0711a9c3a9cf5d9cc1a822e4caba53c866. The fix is confined to the test's platform gate; no new finding was introduced in this disposition pass. The lead retains the final aggregate gate.
