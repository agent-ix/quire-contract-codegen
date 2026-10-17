---
id: SR-12130
title: "IR-847 spec review: scalar native error discriminant (base)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@ae13663706dd319a2bb1e4e94ac183406125fdac (PR #418, IR-847, diff vs origin/peter/ir-827-cbmc-failure-cause); spec/kani/functional/FR-017-kani-execution-evidence.md (Scalar native error discriminant, FR-017-AC-41..44), spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: references
---

## Summary

Ticket: IR-847 (PR agent-ix/quire-contract-codegen#418, frozen head ae13663706dd319a2bb1e4e94ac183406125fdac, stacked on #410). base review of the new FR-017 "Scalar native error discriminant" subsection, FR-017-AC-41..44, the TC-027 steps and the tests.md rows, verified against src/kani/run/scalar.rs and src/kani/run/execute.rs at the frozen head and against consumer quire-driver FR-009-AC-7.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-017-AC-41 | examined | Every `KaniScalarNativeError` variant has a `code()` that is a total `&'static str` from exactly this set: `invalid_request`, `artifact_package`, `snapshot_io`, `kani_execution`, `proof_not_verified`, `oracle_stop`, `native_launch`, `dependency_lock`, `native_result`, `snapshot_changed`, `identity_e |
| FR-017-AC-42 | examined | `class()` is `CompletionRefusal` for `invalid_request`, `artifact_package`, `proof_not_verified`, `oracle_stop` and `dependency_lock`, and `EnvironmentFault` for `snapshot_io`, `native_launch`, `native_result`, `snapshot_changed` and `identity_encode`. For `kani_execution` it is `CompletionRefusal`  |
| FR-017-AC-43 | examined | A canonical-identity encoding failure is `IdentityEncode` with code `identity_encode` and class `EnvironmentFault`; no path returns `NativeResult` for it. `NativeResult` is produced only for the helper's malformed, missing or unsupported structured result, and a source scan of the identity-measureme |
| FR-017-AC-44 | examined | The code table is frozen: a test lists every variant with its expected code and class, fails if a variant exists that the list lacks or if a code value differs, and fails if two variants share a code. Adding a variant, code or class is a spec change to the "Scalar native error discriminant" section  |
| FR-017 discriminant: code/class statement | examined | - The scalar runner shall give `KaniScalarNativeError` a total method `code()` returning `&'static str`, never an `Option`, from the closed set below, and a total method `class()` returning `KaniScalarNativeErrorClass`, a closed two-value enum (`CompletionRefusal`, `EnvironmentFault`) that is not `n |
| FR-017 discriminant: non_exhaustive statement | examined | - The scalar runner shall mark `KaniScalarNativeError` `#[non_exhaustive]`. A downstream `match` on its variants must then carry a wildcard arm; a consumer shall select on `class()` and `code()`, not on variants. Adding a variant is an additive Rust change but is a spec change (FR-017-AC-44). |
| FR-017 discriminant: refusal/fault definition | examined | - A completion refusal is a failure fixed by the request and staged content, or a typed result of the proof itself, that the consumer shall preserve unchanged. An environment or decode fault is a failure of the host, toolchain, helper or runner that the consumer shall not present as a refusal of the |
| FR-017 discriminant: ProofNotVerified row | examined | / `ProofNotVerified` / `proof_not_verified` / `CompletionRefusal` / A typed proof result with its evidence (falsified, cover-unsatisfied, inconclusive, no verdict); the evidence, not the class, tells the consumer why. / |
| FR-017 discriminant: InvalidRequest row | examined | / `InvalidRequest` / `invalid_request` / `CompletionRefusal` / Fixed by the request: identity, package, operand or limit mismatch, found before either child starts. / |
| FR-017 discriminant: nested Kani classification | examined | For `Kani`, `class()` is `CompletionRefusal` when the nested refusal is `HarnessNotInCrate`, `HarnessProvenanceMismatch`, `OutputOverLimit` or `PlaybackForNonMember` (fixed by the harness, crate or captured output), and `EnvironmentFault` for `Tool`, `MemoryMechanismUnavailable`, `MemoryObservationF |
| FR-017 discriminant: stability contract | examined | Stability contract: the eleven codes above are the whole set. A code is never renamed, reused for another variant, or removed while its variant exists. A variant's `class()` never changes. Adding a variant, code or nested classification is a spec change to this section, made with its tests. |
| TC-027 discriminant steps | examined | Scalar native error discriminant (FR-017-AC-41 to AC-44): build one `KaniScalarNativeError` of each of the eleven variants, the nested `Kani` error once for each `KaniExecutionRefusal` variant. (1) Assert `code()` of each equals its table code, that the eleven codes are pairwise distinct, and that ` |
| TC-027 discriminant pass criteria | examined | For the scalar error discriminant, each variant has its table code and class, the codes are distinct and frozen, the five refusal variants and the nested request-fixed `Kani` refusals are `CompletionRefusal`, the five fault variants and every other nested refusal are `EnvironmentFault`, and an ident |
| tests.md FR-017-AC-41..44 rows | examined | / FR-017 / FR-017-AC-41 through FR-017-AC-44 / TC-027 / 🚧 Planned (IR-847); the stable `KaniScalarNativeError` code set, the completion-refusal/environment-fault split, the separate identity-encode variant and the frozen code table / |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-43 and TC-027 step (3) require 'a source scan of the identity-measurement function finds no NativeResult'. The checklist forbids any test case that scans source text (Hash/Digest/Pin antipattern: a high finding, the fix is removal). The scan is also unfalsifiable as worded: at ae13663 the offending NativeResult (scalar.rs:512) is in canonical_identity, not measure, so a scan of 'the identity-measurement function' read as measure passes today. Remove the scan; the behavioural half (forced encode failure returns IdentityEncode, identity_encode, EnvironmentFault, never NativeResult) is the evidence. | spec/kani/functional/FR-017-kani-execution-evidence.md:708 |
| FND-002 | medium | AC-41 asserts KaniScalarNativeError is #[non_exhaustive] and that KaniScalarNativeErrorClass is closed (not non_exhaustive), but no TC-027 step checks either. An in-crate unit test cannot observe non_exhaustive; it needs an external-crate check such as a rustdoc compile_fail example with an exhaustive match on KaniScalarNativeError beside a compiling control that matches exhaustively on KaniScalarNativeErrorClass (the same pattern FR-017-AC-38 uses). Add that step or drop the attribute claims from the AC. | spec/kani/functional/FR-017-kani-execution-evidence.md:706 |
| FND-003 | low | The section says each code constant is 'exported beside the type' but does not say KaniScalarNativeErrorClass and the code constants are re-exported from the crate root as KaniScalarNativeError is (src/lib.rs:154). The quire-driver consumer has to name the class enum to select on it; state the public path. | spec/kani/functional/FR-017-kani-execution-evidence.md:614-618 |

## Verdict

Code inventory at ae13663 matches the table: the ten existing KaniScalarNativeError variants plus the new IdentityEncode give the eleven codes, all snake_case and pairwise distinct; all twelve nested KaniExecutionRefusal variants (BatchCancelled, Tool, MemoryMechanismUnavailable, MemoryObservationFailed, BatchMemoryExhausted, HarnessNotInCrate, HarnessProvenanceMismatch, Report, OutputOverLimit, OutputUnread, BatchTimedOut, PlaybackForNonMember) are classified exactly once (4 refusal, 8 fault), and the existing nested code() (Some only for OutputOverLimit/OutputUnread) is stated unchanged. code() total, class() closed, the non_exhaustive change and its downstream wildcard consequence, and the stability contract are stated; in-repo integration tests (tests/it/kani_obligations.rs:3083-3189) only pattern-match InvalidRequest and are unaffected. The NativeResult misuse is real (scalar.rs:512) and correctly fixed by the new IdentityEncode variant. TC-027 steps cover AC-41 codes, AC-42 classes with real and injected causes, AC-43 behaviour and AC-44 frozen table, apart from FND-001 and FND-002. Author decisions accepted on merit: snake_case codes 'snapshot_io'/'kani_execution' differing from variant names (the string is the contract); no stable codes for nested refusals other than OutputOverLimit/OutputUnread (outer class() gives FR-009-AC-7 the refusal/fault split it needs). Not mergeable until FND-001 is fixed.

## New findings (disposition pass 1)

Rechecked at frozen head 75a0bb88176666b785d64bf7496bb5bf768a713c (fix commit 58f8bbe).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | The class-enum compile_fail half ('a rustdoc compile_fail match that names a third class', TC-027 step 5) fails to compile whether or not KaniScalarNativeErrorClass is closed or non_exhaustive: naming a variant that does not exist is always an error. Only the compiling control (an external-crate exhaustive two-arm match with no wildcard) tests closedness. Drop the vacuous compile_fail or replace it with one that fails only if a third class existed; keep the control. | spec/kani/functional/FR-017-kani-execution-evidence.md:724 |
| FND-005 | low | The stability contract says 'A variant's class() never changes', and AC-44 / TC-027 step 4 freeze 'every variant with its expected code and class', but ProofNotVerified (by evidence outcome) and Kani (by nested refusal) have no single class. State the frozen unit as the mapping (variant, and for these two the outcome or nested refusal, to class) so the contract and the frozen-table test have one reading. | spec/kani/functional/FR-017-kani-execution-evidence.md:674-676 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | verified in 58f8bbe at 75a0bb8 |
| FND-002 | fixed | verified in 58f8bbe at 75a0bb8 |
| FND-003 | fixed | verified in 58f8bbe at 75a0bb8 |
