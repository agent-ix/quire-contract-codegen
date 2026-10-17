---
id: SR-12131
title: "IR-847 failure-domain review: scalar native error refusal/fault split"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen@ae13663706dd319a2bb1e4e94ac183406125fdac (PR #418, IR-847, diff vs origin/peter/ir-827-cbmc-failure-cause); spec/kani/functional/FR-017-kani-execution-evidence.md (Scalar native error discriminant, FR-017-AC-41..44), spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: references
---

## Summary

Ticket: IR-847 (PR agent-ix/quire-contract-codegen#418, frozen head ae13663706dd319a2bb1e4e94ac183406125fdac, stacked on #410). failure-domain review of the new FR-017 "Scalar native error discriminant" subsection, FR-017-AC-41..44, the TC-027 steps and the tests.md rows, verified against src/kani/run/scalar.rs and src/kani/run/execute.rs at the frozen head and against consumer quire-driver FR-009-AC-7.

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
| FND-001 | high | ProofNotVerified is CompletionRefusal for every non-Verified evidence outcome, but the scalar path (execute_kani_obligation_readonly -> run_single, execute.rs:378-384, 1090) returns caller cancellation as Ok(evidence) with KaniRunOutcome::Cancelled, and wall-clock timeout, memory exhaustion and NoVerdict ('a build, launcher or solver failure', classify.rs:26-27) as Inconclusive evidence; all become ProofNotVerified -> CompletionRefusal. The same causes in a batch (BatchCancelled, BatchTimedOut, BatchMemoryExhausted) are EnvironmentFault, and the section's own definition makes host/toolchain failures faults and uses 'retry can succeed' to class SnapshotChanged as a fault. quire-driver FR-009-AC-7 would then pass a cancelled or timed-out run through unchanged as a refusal of the request. Because the stability contract freezes class() per variant, this must be decided now: split ProofNotVerified's class by evidence outcome (Cancelled, TimedOut, MemoryExhausted, NoVerdict -> EnvironmentFault), or state and justify the rule explicitly in the table and AC-42, with TC-027 cases for a cancelled and a timed-out proof. | spec/kani/functional/FR-017-kani-execution-evidence.md:638 |
| FND-002 | low | InvalidRequest is CompletionRefusal as 'fixed by the request', but scalar.rs:835 also returns InvalidRequest("Contract Codegen lock does not name one runtime git revision"), a defect of codegen's own embedded Cargo.lock, not the request. Either state the spec's handling of that path (an internal fault variant, e.g. reuse the IdentityEncode-style runner fault) or note it as a known build-invariant that cannot occur in a released build. | spec/kani/functional/FR-017-kani-execution-evidence.md:634 |

## Verdict

Author decisions judged: SnapshotChanged = EnvironmentFault is accepted (staged bytes changing under a sealed snapshot is not request-determined; consistent with the fault definition); nested Report = EnvironmentFault accepted (report decode fault, parallel to NativeResult); OutputOverLimit as a refusal accepted (output size is determined by the harness). The failure-domain defect is FND-001: the class of ProofNotVerified ignores that the single-run Kani path reports cancellation, timeout, memory exhaustion and launcher/build failure as evidence rather than refusals, so these host faults are classed as completion refusals, opposite to their batch counterparts and to the section's own definition. Not mergeable until FND-001 is fixed or explicitly justified.

## New findings (disposition pass 1)

Rechecked at frozen head 75a0bb88176666b785d64bf7496bb5bf768a713c (fix commit 58f8bbe).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | high | The new rule makes FailedWithoutCounterexample 'with any failureCause' an EnvironmentFault, listing the causes as 'a CBMC timeout, out-of-memory, exit code or unknown failure'. KaniFailureCause also has PropertiesFailed (src/kani/output/report.rs:385), which real Kani writes for a failed assertion with no playback (FR-017-AC-38; AC-39's e::check), and the code already separates it with is_cbmc_failure(). As written, the common real failed-property case is classed a fault, the reverse of the rule's own 'failed property' refusal. Make the fault branch 'a failureCause other than properties_failed' (refusal for none or properties_failed), and add a TC-027 case for FailedWithoutCounterexample with properties_failed requiring CompletionRefusal. | spec/kani/functional/FR-017-kani-execution-evidence.md:654-662 |
| FND-004 | low | For Kani the section says the nested match is exhaustive so a new nested refusal forces this spec's decision; the ProofNotVerified rule says nothing equivalent for KaniRunOutcome, KaniInconclusiveReason and KaniFailureCause. A new reason or cause would get a class from whatever default the implementation picks. State that class() matches those enums exhaustively, with no wildcard arm, so a new value forces a spec decision. | spec/kani/functional/FR-017-kani-execution-evidence.md:654-662 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | verified in 58f8bbe at 75a0bb8 |
| FND-002 | fixed | verified in 58f8bbe at 75a0bb8 |
| FND-003 | fixed | round 2: verified in a569513 |
| FND-004 | fixed | round 2: verified in a569513 |

## New findings (disposition pass 4)

Planner-ruled gap round at frozen head 24a4caf84f0ad5f21e7160c683505a7b06ed7e1c (fix commit 24a4caf). The Verified-as-fault rule and the AC-43 in-crate seam wording were checked and are consistent with AC-42, AC-44 and the frozen mapping; no finding there.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | The round-4 text makes DependencyLock unreachable from a canonical request (validate_request requires Cargo.toml to equal oracle_crate_manifest, which names the runtime by branch = main with no rev; scalar.rs:402-406, profile.rs:22-27) and keeps it a CompletionRefusal 'because the package would fix it'. Both claims are verified except the conclusion: with the manifest forced canonical, no package can fix it, and the paths that still reach it (scalar.rs:879-889) are host or toolchain faults: `cargo update --precise <rev>` exits 0 but the lock does not name that revision, or reading Cargo.lock fails, which `.ok()` turns into DependencyLock{actual: None}. So the variant is reachable only through environment faults and is classed a refusal, the same mismatch SR-12131 FND-001 fixed for ProofNotVerified; quire-driver FR-009-AC-7 would pass it through unchanged as a refusal of the request. Class it EnvironmentFault (and update AC-42, TC-027 step 2 and the frozen mapping), or state that a refusal is chosen despite every reachable cause being environmental. | spec/kani/functional/FR-017-kani-execution-evidence.md:633 |
