---
id: SR-12771
title: 'IR-847 ears-conformance review: stable scalar native error discriminant'
type: SpecReview
analysis: ears-conformance
scope: 'agent-ix/quire-contract-codegen; PR #418 IR-847; spec/kani/functional/FR-017-kani-execution-evidence.md;
  spec/kani/matrix/TC-027-kani-execution-evidence.md; spec/kani/matrix/tests.md'
review_set: subset
---

## Summary

Ticket: IR-847. Reviewed frozen head 4d1a1ede2b1795663dff459d5929f04745b1e9d9 against own parent 4b1f2ad41924ff3532fbc488adc9d9b25b7bd251. Supplemental spec-review/ears-conformance method; original base/failure-domain methods are preserved rather than repeated.

The own diff contains five Markdown files only. No Rust, Cargo, executable, CI, dependency or source-embedded-document change exists. All four new criteria remain explicitly Planned; implementation and runnable evidence are owed to the later IR-847 code slice, so this spec delivery makes no implementation-completion claim. Original base and failure-domain artifacts remain byte-identical to their latest reviewer scratch; all nine published comment bodies match scratch after normalization of a final newline only and parse as YAML, with all ten latest finding outcomes fixed. The final twelve-code vocabulary and AC-41..44 agree with TC-027. DependencyLock remains EnvironmentFault, unreachable from a canonical request through a request fault, with only an in-crate seam required. ProofNotVerified mapping distinguishes genuine proof verdicts from host/tool faults and handles Verified as a non-panicking internal inconsistency. No applicable AssuranceProfile exists under spec/.

## Verdict

PASS — edited obligations name the scalar runner/consumer and specify concrete total return types, stable discriminants, exhaustive classification, and measurable error separation. No new momentary/state trigger confusion or vague response exists. The shared predicate for the identity-encode/native-result separation states complementary observable branches; it does not obscure the actor or oracle. Engine validation of all spec Markdown exits 0: 107/109 documents grammar-clean, with three existing advisories (FR-017:259 missing-subject/unclassifiable and FR-014:252 non-canonical-trigger). Those lines are unchanged by this PR and outside its examined statement delta; no whole-repo EARS PASS is claimed.

## Reviewed scope

```yaml
scope:
- id: FR-017-discriminant-statement-1
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The scalar runner shall give `KaniScalarNativeError` a total method `code()`
    returning   `&'static str`, never an `Option`, from the closed set below, and
    a total method `class()`   returning `KaniScalarNativeErrorClass`, a closed two-value
    enum (`CompletionRefusal`,   `EnvironmentFault`) that is not `non_exhaustive`.
    Each code is a snake_case string constant   exported beside the type. `KaniScalarNativeErrorClass`,
    `KaniScalarNativeError`'s `code()` constants   and `class()` are public at the
    crate root, re-exported beside `KaniScalarNativeError` in   `src/lib.rs` (the
    constants as `SCALAR_NA
- id: FR-017-discriminant-statement-2
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The scalar runner shall mark `KaniScalarNativeError` `#[non_exhaustive]`.
    A downstream `match`   on its variants must then carry a wildcard arm; a consumer
    shall select on `class()` and   `code()`, not on variants. Adding a variant is
    an additive Rust change but is a spec change   (FR-017-AC-44).
- id: FR-017-discriminant-statement-3
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The scalar runner shall return the identity-encode failure (canonical proof-content
    identity   could not be encoded within its byte limit, reported by `canonical_identity`)
    as the variant   `IdentityEncode`, code `identity_encode`, and shall return `NativeResult`
    only for a malformed,   missing or unsupported structured result from the native
    helper.
- id: FR-017-discriminant-statement-4
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The scalar runner shall return the failure to find a runtime revision
    in codegen''s own embedded   lock (today `InvalidRequest` in the lock-revision
    lookup) as the variant `CodegenLock`, code   `codegen_lock`: it is fixed by codegen,
    not by the request, so it is an environment fault.   `DependencyLock` remains
    only for a staged lock that does not name codegen''s runtime revision after the
    update step. It is unreachable from a canonical request by a request fault: request
    validation requires the manifest to equal the canonical one (runtime on `branch
    = main`, no `rev`), and a package that differs'
- id: FR-017-discriminant-statement-5
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A completion refusal is a failure fixed by the request and staged content,
    or a typed result of   the proof itself, that the consumer shall preserve unchanged.
    An environment or decode fault is   a failure of the host, toolchain, helper or
    runner that the consumer shall not present as a   refusal of the request.
- id: FR-017-code-invalid_request
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| `InvalidRequest` | `invalid_request` | `CompletionRefusal` | Fixed by
    the request: identity, package, operand or limit mismatch, found before either
    child starts. |'
- id: FR-017-code-artifact_package
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| `ArtifactPackage` | `artifact_package` | `CompletionRefusal` | Fixed
    by the supplied package bytes and paths against the standard bounds. |'
- id: FR-017-code-codegen_lock
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| `CodegenLock` (new) | `codegen_lock` | `EnvironmentFault` | Codegen''s
    own embedded lock names no runtime revision: a build or packaging fault of codegen,
    never of the request. |'
- id: FR-017-code-snapshot_io
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| `Snapshot` | `snapshot_io` | `EnvironmentFault` | Host I/O failure creating,
    writing or measuring the private snapshot. |'
- id: FR-017-code-kani_execution
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| `Kani` | `kani_execution` | by the nested refusal, below | The nested
    `KaniExecutionRefusal` mixes request-fixed refusals and host faults, so the outer
    variant is neither always. |'
- id: FR-017-code-proof_not_verified
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| `ProofNotVerified` | `proof_not_verified` | by the carried evidence
    outcome, below | On a single run, cancellation, timeout, memory exhaustion and
    no-verdict all arrive as `ProofNotVerified`, while their batch twins are `Kani`
    faults; the class follows the outcome so the two paths agree. |'
- id: FR-017-code-oracle_stop
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| `OracleStop` | `oracle_stop` | `CompletionRefusal` | The runtime oracle
    rejected a generated operand, constant or width: a typed, deterministic stop.
    |'
- id: FR-017-code-native_launch
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| `NativeLaunch` | `native_launch` | `EnvironmentFault` | The bounded
    native build or invocation did not complete: toolchain or host. |'
- id: FR-017-code-dependency_lock
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| `DependencyLock` | `dependency_lock` | `EnvironmentFault` | The staged
    lock does not name codegen''s runtime revision after the update step: `cargo update
    --precise` wrote no matching revision or the lock was unreadable. A host or toolchain
    fault, not fixed by the request (see above). |'
- id: FR-017-code-native_result
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| `NativeResult` | `native_result` | `EnvironmentFault` | The helper returned
    malformed, missing or unsupported structured output: a decode fault. |'
- id: FR-017-code-snapshot_changed
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| `SnapshotChanged` | `snapshot_changed` | `EnvironmentFault` | Staged
    bytes changed during a child run. The change is not determined by the request,
    so a retry can succeed; it is an integrity fault, not a refusal of the request.
    (IR-847''s description listed it as a refusal; this spec decides otherwise.) |'
- id: FR-017-code-identity_encode
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| `IdentityEncode` | `identity_encode` | `EnvironmentFault` | Encoding
    the canonical identity of already-bounded content failed: an internal runner fault.
    |'
- id: FR-017-discriminant-mapping-3
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'For `ProofNotVerified`, `class()` is `CompletionRefusal` when the evidence
    outcome is a genuine property or proof verdict: `Falsified`, `CoverUnsatisfied`,
    or `Inconclusive` with reason `MissingCoverSummary`, `VacuousProof`, `UnwindBoundExhausted`,
    or `FailedWithoutCounterexample` with no `failureCause` or with `failureCause`
    `properties_failed` (a failed property, which real Kani writes for a failed assertion
    without playback). It is `EnvironmentFault` when the run did not reach a verdict:
    `Cancelled`, `Inconclusive` with reason `TimedOut`, `MemoryExhausted` or `NoVerdict`,
    and `FailedWithout'
- id: FR-017-discriminant-mapping-4
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: For `Kani`, `class()` is `CompletionRefusal` when the nested refusal is
    `HarnessNotInCrate`, `HarnessProvenanceMismatch`, `OutputOverLimit` or `PlaybackForNonMember`
    (fixed by the harness, crate or captured output), and `EnvironmentFault` for `Tool`,
    `MemoryMechanismUnavailable`, `MemoryObservationFailed`, `BatchMemoryExhausted`,
    `BatchTimedOut`, `BatchCancelled`, `OutputUnread` and `Report` (host, backend,
    cancellation or report-decode faults). The nested match is exhaustive over `KaniExecutionRefusal`,
    so a new nested variant forces this spec's decision. The outer `code()` of `Kani`
    is alway
- id: FR-017-discriminant-mapping-5
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Stability contract: the twelve codes above are the whole set. A code is
    never renamed, reused for another variant, or removed while its variant exists.
    A variant''s class, or for `ProofNotVerified` and `Kani` its mapping to class,
    never changes. Adding a variant, code or nested classification is a spec change
    to this section, made with its tests.'
- id: FR-017-AC-41
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Every `KaniScalarNativeError` variant has a `code()` that is a total `&''static
    str` from exactly this set: `invalid_request`, `artifact_package`, `snapshot_io`,
    `kani_execution`, `proof_not_verified`, `oracle_stop`, `native_launch`, `dependency_lock`,
    `native_result`, `snapshot_changed`, `identity_encode`, `codegen_lock`. The set
    is closed and the codes are pairwise distinct. The type is `#[non_exhaustive]`
    (a rustdoc `compile_fail` exhaustive match without a wildcard arm, beside a compiling
    control with one), and `class()` returns a closed `KaniScalarNativeErrorClass`
    (a compiling exhaustive '
- id: FR-017-AC-42
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '`class()` is `CompletionRefusal` for `invalid_request`, `artifact_package`
    and `oracle_stop`, and `EnvironmentFault` for `snapshot_io`, `native_launch`,
    `dependency_lock`, `native_result`, `snapshot_changed`, `identity_encode` and
    `codegen_lock`. For `proof_not_verified` it is `CompletionRefusal` for evidence
    outcome `Falsified`, `CoverUnsatisfied`, and `Inconclusive` `MissingCoverSummary`,
    `VacuousProof`, `UnwindBoundExhausted` or `FailedWithoutCounterexample` with no
    `failureCause` or `properties_failed`, and `EnvironmentFault` for `Cancelled`,
    `Inconclusive` `TimedOut`, `MemoryExhausted` or'
- id: FR-017-AC-43
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A canonical-identity encoding failure is `IdentityEncode` with code `identity_encode`
    and class `EnvironmentFault`; no path returns `NativeResult` for it. `NativeResult`
    is produced only for the helper's malformed, missing or unsupported structured
    result, and a forced identity-encode failure through an in-crate test seam returns
    `IdentityEncode`, not `NativeResult`.
- id: FR-017-AC-44
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The code table and the mapping to class are frozen: a test lists every
    variant with its expected code and its mapping to class (one class for a fixed
    variant; for `proof_not_verified` the class of each `KaniRunOutcome`, `KaniInconclusiveReason`
    and `KaniFailureCause` value; for `kani_execution` the class of each nested refusal),
    fails if a variant, outcome, reason, cause or nested refusal exists that the list
    lacks or if a code value or a mapped class differs, and fails if two variants
    share a code. Adding a variant, code, class or mapping entry is a spec change
    to the "Scalar native error dis'
- id: TC-027-discriminant-steps
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: examined
  excerpt: 'Scalar native error discriminant (FR-017-AC-41 to AC-44): build one `KaniScalarNativeError`
    of each of the twelve variants, the nested `Kani` error once for each `KaniExecutionRefusal`
    variant, and `ProofNotVerified` once for each evidence outcome. (1) Assert `code()`
    of each equals its table code, that the twelve codes are pairwise distinct, that
    `Kani` gives `kani_execution` for every nested refusal, and that the type, the
    class enum and the code constants are reachable at the crate root. (2) Assert
    `class()` of each variant, of each nested `Kani` refusal, and of `ProofNotVerified`
    for `Fals'
- id: TC-027-discriminant-pass
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: examined
  excerpt: For the scalar error discriminant, each variant has its table code and
    class, the codes are distinct and frozen, the three refusal variants, the nested
    request-fixed `Kani` refusals and a `ProofNotVerified` carrying a property or
    proof verdict are `CompletionRefusal`; the seven fault variants, every other nested
    refusal and a `ProofNotVerified` of a cancelled, timed-out, memory-exhausted or
    no-verdict run are `EnvironmentFault`, and an identity-encode failure is `IdentityEncode`,
    never `NativeResult`.
- id: tests.md-IR-847-48
  path: spec/kani/matrix/tests.md
  role: examined
  excerpt: '| FR-017 | FR-017-AC-41 through FR-017-AC-44 | TC-027 | 🚧 Planned (IR-847);
    the stable `KaniScalarNativeError` code set, the completion-refusal/environment-fault
    split, the separate identity-encode variant and the frozen code table |'
- id: tests.md-IR-847-105
  path: spec/kani/matrix/tests.md
  role: examined
  excerpt: '| TC-027 | Verify Kani obligation execution and its evidence | Analysis
    | P0 | FR-017-AC-41, FR-017-AC-42, FR-017-AC-43, FR-017-AC-44 | 🚧 Planned (IR-847)
    |'
- id: src/kani/run/scalar.rs
  path: src/kani/run/scalar.rs
  role: context_only
  excerpt: pub enum KaniScalarNativeError {
- id: reviews/ir-847-spec-review.md
  path: reviews/ir-847-spec-review.md
  role: context_only
  excerpt: 'SR-12130: original base review and five fixed findings, including compile
    controls and mapping freeze.'
- id: reviews/ir-847-spec-review-failure-domain.md
  path: reviews/ir-847-spec-review-failure-domain.md
  role: context_only
  excerpt: 'SR-12131: original failure-domain review and five fixed findings, including
    DependencyLock EnvironmentFault ruling.'
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Method selection

Original base and failure-domain methods already cover public API consistency, AC/TC structural and evidence sufficiency, and failure classification. Integrity adds no uncovered structural change; object-review adds no separate persisted domain-object edit; dependency adds no relationship edge; evidence adds no changed verification method or missing oracle beyond the checked original base review. Code-review and EARS are the missing applicable methods supplied here. No supplemental method reruns the finished original reviews or reopens their resolved product ruling. Exact runtime model identifier is unavailable to this harness; the comment records unavailable rather than guessing.
