---
id: SR-12933
title: IR846 native implementation spec-review/evidence
type: SpecReview
analysis: evidence
scope: agent-ix/quire-contract-codegen; IR-846 joint IR-847 IR-857 PR442; FR017 AC41..50,
  TC027/status delta, native runner/generator private controls
review_set: subset
---

## Summary

Ticket: IR-846. Independent review of the frozen joint scalar-native implementation and its owning contract; no findings from historical spec PR419 are reused.

## Verdict

CONDITIONAL — the promised runner-produced oracle-stop refusal control is missing; the remaining documented evidence is attributed to its actual author head and lane.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The required runner-produced OracleStop refusal has no end-to-end runner control. native_decoder_retains_actual_limits_and_rejects_each_mismatch fabricates helper text and calls decode_native_output directly; the outer table also constructs OracleStop manually. Neither exercises execute_kani_scalar_native through native execution and error propagation. An orchestration regression that remaps an oracle stop would leave these tests green. Add a faithful bounded dependency/fixture control that reaches the paired runner and asserts OracleStop/oracle_stop/CompletionRefusal without constructing the error itself; retain the direct decoder cases as unit coverage. | src/kani/run/scalar.rs:1519-1532 |

## Coverage

quoin advise --json and quoin catalog methods --json ran and their outputs are retained. All ten new obligations AC41..50 are authored Test, mismatch=false, inconclusive=false, uncatalogued=false. Advisor recommendations include example/unit controls for AC42/44/45, property controls for AC41/47, negative/fuzz controls for AC43/50, and compile/invariant/integration controls for AC46/48. AC49 includes the explicit inspection recommendation (no executable oracle for golden-update review), beside property testing. Judgement: the mixed unit, compile, literal golden, independent digest, dependency-seam, bounded launcher and real Kani/native controls generally fit these obligations. Recommendations are advisory; no fuzz/model-check/sanitizer execution is claimed. The real oracle-stop runner requirement remains unserved (FND001), and code/test defects are retained in SR12930. Actual author results are tied to the author head and documented private-lib command; no future root gate is represented as already executed. Changed matrix/index prose calls these implemented controls, not downstream driver completion.
Frozen head: 5ee6f3f0be5863e4a5d22cd70325a454d3ef9a23; base cb96317e27184e379869ca63a06e70a5daba9ce2; PR442; joint IR-846/IR-847/IR-857, primary tracker IR-846. The repository is public; findings go only to private Linear IR-846. This is a new implementation review, not historical spec PR419. No source, spec, test, lock, CI, branch or staging edits were made. No applicable AssuranceProfile exists under spec/.

Author execution is evidence for 4f2cf0bb52e63a345fcd44dde7a0f1a22f716817 only: full canonical make kani-gate 36/36, 798 seconds, Kani 0.68.0, clean tree; private actual controls 2/2 in 153.63 seconds; unit controls 10; doctests 14; private default 10 passed/2 ignored; final layout/lane census 7 and scalar differential 1. These logs and their source assertions were inspected. The historical 139-pass selection predates the private fixture and is not current fixture evidence. Independently compared original/current complete binary own-delta patches: byte-identical. All eight files are unchanged across the rebase; PR441 vacuity tests/reviews are retained. No backend run is claimed on 5ee6f3f. Root owns final aggregate and full Kani gates after findings and artifact custody.

The reviewer ran one scratch-only rustc probe under the assigned exclusive slot, exact-name pgrep guards, systemd 16GiB/zero-swap, QSPEC_DIR, existing worktree dependency artifacts and own scratch output. The decoder and lock-verification function bodies were copied verbatim; only the small result/error containers are standalone structural stand-ins. This establishes local decode/classification behavior, not a full paired execution. The first compile refused an incompatible ambient compiler (1.99.1 versus cached 1.98.1); the explicit 1.98.1 retry passed, with no dependency rebuild. probe-output.log reproduces the malformed repeated native record acceptance and non-UTF-8 lock classification. The slot was explicitly released with an empty guard in heavy-release.txt. No Cargo, Kani, aggregate gate or full matrix test run was launched by the reviewer.

Production uses one core canonical encoder, QSL-owned DigestRecord, twelve literal stable code constants, closed class enum and exhaustive production classification over all nested outcomes/reasons/causes/refusals. No new production panic, unsafe code, narrowing wire cast, async blocking edge, dependency, CI weakening, compatibility layer, public receipt constructor or deserializer was found. Fixed ten-field array indexing follows checked arity; output collectors remain bounded by the existing launcher capture ceiling. Snapshot fields are owned and immutable; capture checks byte equality and exact path set, both child invocations retain read-only binding, and explicit cleanup precedes success. Cleanup failure is observed through the real removal dependency seam and does not enter the preimage. Actual limits come from meter.limits() and all ten reported fields are compared with request limits; unequal field tests and retained receipt limits are inspected.

The Python standard-library independent canonical string-only encoding and SHA-256 matches the literal golden 571d6199414afadc065744e9c47df06fbeb18ee36cca995ba029309add15d356 (golden-python.json). The literal golden is separate from live lock contents; all seven entries have drop/rename/reorder/duplicate/byte-mutation checks. Healthy/incomplete native runs and both subtraction operand orders compare artifact-only digest identity. Receipt opacity, immutable entries/limits, shared QSL type identity and non-exhaustive error/closed-class positive and compile-fail controls were inspected. Production schema artifact set/order/encoding is unchanged from the old raw identity; only staged adapter content changes, so no schema bump is needed.

The seeded negative generator fixture retains the original routed package and embedded oracle and changes only the asserted bound before normal render/record/origin sealing. It returns the existing sealed harness, binds the real artifact package and adds no public API; the crate-root private test performs execution/classification. Generator imports have no run/classify edge, normal production rendering is unchanged and IR858 root-private bridging is preserved. Actual author evidence confirms Falsified/proof_not_verified/CompletionRefusal and no native target. Naturally narrowing the admitted claim to [0,5] is sound and is not called a failing proof. Sealed-source mutation continues to refuse provenance. Downstream driver authentication and original-request limit provenance are not claimed.

Methods: code-review with Rust lane, whole-repository planless gap-analysis, spec-review/base and spec-review/evidence. Other lenses do not match this status/evidence-only specification delta: no requirement grammar, structure, relationship, object or architecture change. Optional expanded whole-repository semantic review was not commissioned; focused contract/code/test intent review was explicitly authorized and is recorded here. No agents were spawned.

## Examined units

Full examined text is retained below. Tracker excerpts may be verbatim prefixes to stay within the tracker size limit.

```yaml
scope:
- id: FR-017-AC-41
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Every `KaniScalarNativeError` variant has a `code()` that is a total `&''static str` from exactly this
    set: `invalid_request`, `artifact_package`, `snapshot_io`, `kani_execution`, `proof_not_verified`, `oracle_stop`,
    `native_launch`, `dependency_lock`, `native_result`, `snapshot_changed`, `identity_encode`, `codegen_lock`.
    The set is closed and the codes are pairwise distinct. The type is `#[non_exhaustive]` (a rustdoc `compile_fail`
    exhaustive match without a wildcard arm, beside a compiling control with one), and `class()` returns a closed
    `KaniScalarNativeErrorClass` (a compiling exhaustive two-arm control with no wildcard, and no `#[non_exhaustive]`
    on it). For a `Kani` error the code is `kani_execution` for every nested refusal.'
- id: FR-017-AC-42
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '`class()` is `CompletionRefusal` for `invalid_request`, `artifact_package` and `oracle_stop`, and `EnvironmentFault`
    for `snapshot_io`, `native_launch`, `dependency_lock`, `native_result`, `snapshot_changed`, `identity_encode`
    and `codegen_lock`. For `proof_not_verified` it is `CompletionRefusal` for evidence outcome `Falsified`, `CoverUnsatisfied`,
    and `Inconclusive` `MissingCoverSummary`, `VacuousProof`, `UnwindBoundExhausted` or `FailedWithoutCounterexample`
    with no `failureCause` or `properties_failed`, and `EnvironmentFault` for `Cancelled`, `Inconclusive` `TimedOut`,
    `MemoryExhausted` or `NoVerdict`, and `FailedWithoutCounterexample` with a `failureCause` other than `properties_failed`,
    and for the internal inconsistency of outcome `Verified`. For `kani_execution` it is `CompletionRefusal` for
    nested `HarnessNotInCrate`, `HarnessProvenanceMismatch`, `OutputOverLimit` and `PlaybackForNonMember`, and `EnvironmentFault`
    for every other nested refusal. Real errors produced by the runner for the request-mismatch, oracle-stop and
    falsified-proof causes are refusals (`dependency_lock` is an `EnvironmentFault` unreachable from a canonical
    request and is tested only through an in-crate seam); a real single run cancelled by its source and one stopped
    by its wall-clock timeout are faults, as are the injected snapshot-change, native-launch, malformed-helper-output
    and missing-embedded-lock-revision causes.'
- id: FR-017-AC-43
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A canonical-identity encoding failure is `IdentityEncode` with code `identity_encode` and class `EnvironmentFault`;
    no path returns `NativeResult` for it. `NativeResult` is produced only for the helper's malformed, missing or
    unsupported structured result, and a forced identity-encode failure through an in-crate test seam returns `IdentityEncode`,
    not `NativeResult`.
- id: FR-017-AC-44
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The code table and the mapping to class are frozen: a test lists every variant with its expected code
    and its mapping to class (one class for a fixed variant; for `proof_not_verified` the class of each `KaniRunOutcome`,
    `KaniInconclusiveReason` and `KaniFailureCause` value; for `kani_execution` the class of each nested refusal),
    fails if a variant, outcome, reason, cause or nested refusal exists that the list lacks or if a code value or
    a mapped class differs, and fails if two variants share a code. Adding a variant, code, class or mapping entry
    is a spec change to the "Scalar native error discriminant" section made together with that table. The nested
    `KaniExecutionRefusal::code()` of `OutputOverLimit` and `OutputUnread` is unchanged.'
- id: FR-017-AC-45
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The receipt exposes its proof-content identity as a `DigestRecord`, the `qsl_replay::DigestRecord` re-exported
    at the crate root (no CG-defined twin type; a compile test names it by the CG path and by the `qsl_replay` path
    as the same type), with domain `sha256-jcs` and 32 bytes equal to `ScalarProofContentIdentity::as_bytes()`.
    An independent computation in the test, SHA-256 of the RFC 8785 encoding of `{"schema": "agent-ix/cg-kani-scalar-proof-content/v2",
    "artifacts": [...]}` over the entries of AC-46, gives the same 32 bytes. The domain is not `GeneratedArtifactBytesV1`.'
- id: FR-017-AC-46
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'For a genuine routed `x + 1` package the preimage artifacts are, in this order by UTF-8 byte order of
    `path`: `Cargo.lock`, `Cargo.toml`, `claim-map.json`, the harness record at `kani-obligations/<module>.json`
    (the only record path the runner admits), `src/bin/ir791_native.rs`, `src/lib.rs` holding the harness source,
    and `src/oracle-artifacts-lib.rs` holding the emitted library bytes. Removing, renaming, reordering or duplicating
    any entry, or changing one byte of any entry''s contents, changes the digest. A non-UTF-8 runner-read `Cargo.lock`
    or adapter is refused, never lossily converted, as `Snapshot` / `snapshot_io` / `EnvironmentFault` (not `InvalidRequest`),
    and no receipt is returned. A golden digest over fixed literal entries (not a real staged run) is pinned in
    one assertion with the schema string, and fails on any silent preimage change.'
- id: FR-017-AC-47
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Two receipts for the same package and harness with different operands, different `ScalarLimits` and different
    native outcomes have equal `DigestRecord`s; operand order, limits, ranges, outcome and Kani evidence appear
    nowhere in the preimage. This holds for the swapped-operand runs of FR-017-AC-31.
- id: FR-017-AC-48
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The receipt exposes read-only the ordered staged `(path, bytes)` entries the digest was built from,
    including the runner-generated `Cargo.lock` and `src/bin/ir791_native.rs`. Recomputing the digest from only
    those exposed entries equals the receipt''s `DigestRecord`. A recomputation that substitutes one byte in the
    package, harness or harness record (which a driver holding different bytes would do) differs from the receipt''s
    digest. The exposed entries are exactly the bytes measured for the identity, captured at measurement time and
    held in the receipt; no entry accessor reads the filesystem. The runner removes the private snapshot explicitly
    before it returns the receipt; when that removal succeeds the snapshot path no longer exists, and the exposed
    entries still read and the digest recomputes byte-identically from them (the receipt never reads the filesystem).
    `Drop` of the snapshot is a best-effort backstop only. A failure to remove the snapshot directory changes neither
    the receipt nor the verdict: the receipt exposes it read-only through `cleanup_diagnostic()`, which reports
    the removal failure (and is empty when removal succeeded). The cleanup diagnostic is excluded from the digest
    and from the staged entries, and never turns a verified run into a failure or alters an exposed entry or the
    digest. The receipt has no constructor, deserializer or setter for the entries or the digest, and nothing mints
    the digest from operands, limits or outcome.'
- id: FR-017-AC-49
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A change to the preimage schema (artifact set, path names, ordering, encoding, canonicalization or hash)
    is made only by changing the schema string, with the golden digest of AC-46 updated in the same change; the
    golden is computed over fixed literal entries, so dependency bumps do not move it, and is asserted together
    with the schema string in one assertion; a golden change without a schema bump is rejected in review (a test
    cannot enforce it). A change to the bytes of `Cargo.lock` or the adapter changes the digest with the schema
    string unchanged. The domain remains `sha256-jcs` across a schema bump.
- id: FR-017-AC-50
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The `NativeMeterObservation` of a receipt exposes read-only the `ScalarLimits` the native meter was
    constructed with. They are read back from the native invocation''s own output, not copied from the request:
    a test seam that makes the adapter report a well-formed but different limit value makes the helper''s structured
    result unsupported in FR-017-AC-43''s sense and yields `NativeResult` / `native_result` / `EnvironmentFault`
    and no receipt. For an accepted request the request limits, the meter-observed limits and `scalar_limits()`
    are equal, and the meter-observed limits are unchanged after the snapshot directory is gone. The receipt has
    no setter or constructor for them (IR-857).'
- id: TC-027-scalar-error-controls
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: examined
  excerpt: 'Scalar native error discriminant (FR-017-AC-41 to AC-44): build one `KaniScalarNativeError` of each

    of the twelve variants, the nested `Kani` error once for each `KaniExecutionRefusal` variant, and

    `ProofNotVerified` once for each evidence outcome.

    (1) Assert `code()` of each equals its table code, that the twelve codes are pairwise distinct, that

    `Kani` gives `kani_execution` for every nested refusal, and that the type, the class enum and the

    code constants are reachable at the crate root. (2) Assert `class()` of each variant, of each nested

    `Kani` refusal, and of `ProofNotVerified` for `Falsified`, `CoverUnsatisfied`, each inconclusive

    reason, `Cancelled`, `Verified` (an internal inconsistency, requiring `EnvironmentFault` and no panic), and
    `FailedWithoutCounterexample` with no `failureCause`, with `properties_failed` (a real single-run stand-in of
    AC-38''s failed assertion without playback, requiring `CompletionRefusal`) and with each of `timeout`, `out_of_memory`,
    `exit_code` and `unknown` (requiring `EnvironmentFault`), against

    the FR-017 table. Produce real errors by the runner for a request mismatch, an oracle stop and a falsified proof,
    and require `CompletionRefusal`;

    produce a real single run cancelled by its retained source and a real single run stopped by its

    wall-clock timeout (launcher stand-ins), each `ProofNotVerified`, and require `EnvironmentFault`

    with the evidence outcome unchanged; construct `DependencyLock` through an in-crate seam only (it is unreachable
    from a canonical request, whose manifest must equal the canonical one, and a lock update that writes no matching
    revision or an unreadable lock is its host fault) and require `EnvironmentFault`; inject a changed staged byte,
    a native launch failure,

    malformed helper output and an embedded lock with no runtime revision, and require

    `EnvironmentFault` (the last as `codegen_lock`). (3) Force the canonical identity encode to fail (a

    content limit of zero in a test seam) and require `IdentityEncode`, `identity_encode`,

    `EnvironmentFault`, and not `NativeResult`; feed malformed helper output and require

    `native_result`. (4) Keep a frozen list of all variants and codes and of the mapping to class: one class for
    a fixed

    variant, and for `ProofNotVerified` and `Kani` the class of every `KaniRunOutcome`,

    `KaniInconclusiveReason`, `KaniFailureCause` and nested-refusal value, with exhaustive `match`es (no

    wildcard) over those types inside the defining crate that stop compiling when one is added, and

    assert the nested `OutputOverLimit` and `OutputUnread` codes are unchanged. (5) Non-exhaustive and

    closed class: a rustdoc `compile_fail` example matches `KaniScalarNativeError` exhaustively without

    a wildcard arm and must fail to compile, with a compiling control that adds the wildcard arm; the

    class enum is closed by a compiling control that matches `KaniScalarNativeErrorClass` exhaustively

    with exactly `CompletionRefusal` and `EnvironmentFault` and no wildcard, together with the absence of

    `#[non_exhaustive]` on it; the doctests run by `cargo test --doc`, the pattern of the AC-38

    serialize-only check.

    These controls are implemented in the scalar runner''s unit tests and public rustdoc examples.

    The crate-root private scalar-native tests exercise cancellation and timeout with bounded

    single-run launcher stand-ins. Their real-Kani seeded false-bound fixture keeps the routed

    package and embedded oracle intact, changes the asserted bound before the normal renderer and

    origin seal, and requires the paired runner to return real `Falsified` evidence without starting

    native execution. It is a deliberately defective proof property, not a naturally failing admitted

    production claim. Fabricated evidence is confined to the exhaustive classification table.'
- id: TC-027-scalar-digest-controls
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: examined
  excerpt: 'Scalar proof-content digest (FR-017-AC-45 to AC-50): run the paired runner on a genuine routed

    `x + 1` package. (1) Assert the receipt''s `DigestRecord` has domain `sha256-jcs` and 32 bytes equal

    to `as_bytes()`, and recompute SHA-256 of the RFC 8785 encoding of the schema object and the sorted

    artifact list in the test, independently of the runner''s encoder. (2) Assert the exposed entry

    paths are exactly the AC-46 set in UTF-8 byte order, with the library under

    `src/oracle-artifacts-lib.rs` and the harness under `src/lib.rs`; then drop, rename, reorder,

    duplicate and one-byte-change each entry in turn and require a different digest; through a test

    seam that writes non-UTF-8 bytes to the snapshot''s `Cargo.lock` (and again to the adapter) after

    it is generated, require `Snapshot`, `snapshot_io`, `EnvironmentFault`, and no receipt. (3) Run the two operand
    orders of AC-31 and a different `ScalarLimits`

    value and require equal `DigestRecord`s. (4) After the receipt is returned, assert the snapshot path no longer
    exists (the private snapshot is dropped before the receipt is returned), then require the exposed entries to
    still read and the digest to recompute byte-identically; this is a mutation-style check, not a source scan.
    (5) Recompute from only the exposed entries, including

    `Cargo.lock` and `src/bin/ir791_native.rs`, and require equality; substitute one byte of the

    package, harness or record and require inequality. (6) Compute a golden digest with the measurement function
    over fixed literal entries, including a

    literal `Cargo.lock` and adapter and not a real staged run, and pin `(schema string, golden)` in one

    assertion. Change the `Cargo.lock` bytes in a test seam to see the digest change with the schema

    unchanged. A golden edited without a schema bump is a review rule, not a test. (7) Meter limits (FR-017-AC-50):
    require the meter observation''s `ScalarLimits` equal the request''s and `scalar_limits()`, equal and unchanged
    after the snapshot is gone; through a seam that makes the adapter report a different limit, require `native_result`,
    `EnvironmentFault` and no receipt. (8) Name `quire_contract_codegen::DigestRecord` and `qsl_replay::DigestRecord`
    as the same type, and make the snapshot directory removal fail through a seam: require the same verdict, exposed
    entries and digest as an un-failed run, a non-empty `cleanup_diagnostic()` naming the removal failure, and an
    empty `cleanup_diagnostic()` when removal succeeds; the diagnostic is in neither the digest nor the entries
    No

    driver-side authentication coverage is claimed here: the driver''s recomputation and refusal are

    driver-owned (FR-032 boundary), as is the quire-driver FR-009 amendment named in FR-017. These controls are
    implemented by the scalar runner unit tests, the real paired receipt and

    subtraction tests in `tests/it/kani_obligations.rs`, and the private cleanup control in

    `tests/it/kani_scalar_native_private.rs`. The private real controls run separately from the

    integration lane with `cargo test --locked --lib kani_scalar_native_private -- --ignored

    --test-threads=1`; the default private control covers cancellation and timeout stand-ins.

    The literal JCS/SHA-256 golden stays independent of real lock contents, and schema/golden

    changes remain subject to review. No driver authentication coverage is claimed.'
- id: Kani-matrix-native-errors
  path: spec/kani/matrix/tests.md
  role: examined
  excerpt: "| FR-017 | FR-017-AC-41 through FR-017-AC-44 | TC-027 | \u2705 Covered (IR-847); the stable `KaniScalarNativeError`\
    \ code set, exhaustive completion-refusal/environment-fault classification, typed identity-encode and codegen-lock\
    \ faults, frozen tables and downstream rustdoc controls; paired seeded falsification and bounded cancellation/timeout\
    \ stand-ins are crate-root private controls |"
- id: Kani-matrix-native-digest
  path: spec/kani/matrix/tests.md
  role: examined
  excerpt: "| FR-017 | FR-017-AC-45 through FR-017-AC-50 | TC-027 | \u2705 Covered (IR-846, IR-857); the QSL `sha256-jcs`\
    \ `DigestRecord`, exact retained seven-entry preimage, independent JCS/SHA-256 golden and mutation controls,\
    \ artifact-only identity across real native outcomes and operand orders, explicit cleanup and retained observed\
    \ meter limits; schema/golden changes require review and driver authentication remains consumer-owned |"
bindings: []
```

## Dispositions

### Pass 1

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The new private control passes the public paired runner through real Kani verification and an actually launched bounded native protocol stand-in, observes OracleStop(InvalidConstant), code/class and exact native arguments, and never constructs returned errors or proof evidence. The private ignored-lib actual run passed. The dependency seam defaults to cargo in production and is selectable only by cfg(test) construction; this tests runner propagation without claiming a runtime-generated stop from a supported canonical integer operation. |

## Disposition pass 1 assessment

PASS for the scoped implementation review: all recorded findings are fixed, no new findings.

Disposition pass 1 reviewed frozen fix head 7e5392c74a8d58ddbf822f8564cb6a2e549bd441 against original reviewed head 5ee6f3f0be5863e4a5d22cd70325a454d3ef9a23, on unchanged cb96317 base. Original artifacts/findings/verdict sections remain byte-for-byte prefixes; this section records later facts only. No new defects were found in the complete fix delta.

Independent source inspection confirms all six findings fixed. The fixture and digest implementations each have one source home with both target consumers; caller package registries remain isolated. The public production API is not widened: the native executable is an immutable installation dependency defaulting to cargo, with a cfg(test)-only setter. Both proof and native dependency execute under the existing bounded launcher; lock generation continues to use cargo. The native stop stand-in records actual arguments and emits protocol output at the external process boundary; no harness/origin/receipt/evidence is forged and production decoder/orchestration remains the unit under test. No generator/run/classify back-edge, production test bypass, scanner weakening, CI/dependency/lock change, canonical preimage/schema change or held AD004 change was added. Root-private bridge and unchanged production generator inputs remain intact.

Inspected author fix1 logs: early-read/duplicate-native regressions first fail red (2), fixed scalar units pass (13), omission of five independent expected arms yields exactly five E0004 errors, docs pass (14), layout/lane census/preflight/fixture pass (9), default private pass (10, 3 ignored), explicit private actual controls pass (3, 202.89 seconds), affected actual receipt/subtraction integration controls pass (2, 188.65 seconds), and tool/differential pass (2). Lint, compile, format and spec checks pass. The checked tree 6066c5a8bf5ac281458aaf87832d43a81ef7d5af independently equals HEAD^{tree}; logs are author evidence for that exact content, not independently rerun tests. Full aggregate and full Kani gates were not repeated and remain lead-owned after custody. The old full Kani result still belongs only to 4f2cf0bb and is not a current fix-head gate claim.

Static inspection plus existing focused author evidence was sufficient; this reviewer launched no compiler/Cargo/Kani. The exclusive heavy slot was explicitly released after an empty exact-name pgrep guard; heavy-release.txt records it. Source/worktree remained read-only. The original whole-repository gap FAIL is unchanged inherited assurance debt; no repository PASS or full matrix rerun is claimed. The spec base review remains clean. Terminal review permits this PR to proceed to the lead's final gates and artifact custody; there are no open review findings.

```yaml
dispositions:
- fnd: FND-001
  outcome: fixed
  after_excerpt: "let error = execute_kani_scalar_native(&KaniScalarNativeRequest {\n        cancel: &cancel,\n\
    \        installation: &installation,\n        harness: &harness,\n        oracle_artifacts: &artifacts,\n \
    \       operands: &[4, 1],\n        scalar_limits: limits(),\n        target_directory: &target,\n    })\n \
    \   .unwrap_err();\n    assert!(matches!(\n        error,\n        KaniScalarNativeError::OracleStop(ScalarOracleStopKind::InvalidConstant)\n\
    \    ));"
  reason: The new private control passes the public paired runner through real Kani verification and an actually
    launched bounded native protocol stand-in, observes OracleStop(InvalidConstant), code/class and exact native
    arguments, and never constructs returned errors or proof evidence. The private ignored-lib actual run passed.
    The dependency seam defaults to cargo in production and is selectable only by cfg(test) construction; this tests
    runner propagation without claiming a runtime-generated stop from a supported canonical integer operation.
```
