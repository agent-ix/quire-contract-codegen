---
id: SR-12930
title: IR846 native implementation code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; IR-846 joint IR-847 IR-857 PR442; FR017 AC41..50,
  TC027/status delta, native runner/generator private controls
review_set: subset
---

## Summary

Ticket: IR-846. Independent review of the frozen joint scalar-native implementation and its owning contract; no findings from historical spec PR419 are reused.

## Verdict

FAIL — duplicated test implementations require removal; three medium contract/test defects require fixes. Current classification and healthy receipt paths otherwise match the inspected contract.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | A non-UTF-8 staged Cargo.lock is misclassified before the new UTF-8 capture guard runs. generate_lockfile passes fs::read_to_string into verify_runtime_revision; lock.ok() drops InvalidData and returns DependencyLock/dependency_lock instead of the required Snapshot/snapshot_io/EnvironmentFault. The non-UTF-8 test invokes capture_entries directly, bypassing this earlier production read. Preserve the explicit non-UTF-8 Snapshot classification while retaining DependencyLock for the specified unreadable or revision-mismatched staged lock. A verbatim-function scratch probe reproduces DependencyLock for a file containing byte 0xff. | src/kani/run/scalar.rs:1220-1239 |
| FND-002 | medium | The frozen tables assert only the rows manually present; they do not detect missing vocabulary rows. In particular, deleting CoverUnsatisfied from the outcome array, or any nested-refusal row, leaves every assertion green. Adding a new variant plus its production code/class match arms can likewise leave the frozen test list unchanged. Production exhaustive matches enforce a classification decision but do not freeze the independent expected table. Make the expected-table lookup itself exhaustive over each closed vocabulary so an omitted row cannot compile, retaining independently literal expected codes/classes. | src/kani/run/scalar.rs:1754-2051 |
| FND-003 | high | The new routed_scalar_with_artifacts copies the existing admitted-package/routing fixture at tests/it/kani_obligations.rs:2590-2710, changing module paths and the wall-clock argument. Searching both files finds the same bound creation, occurrence selection, driver positions, descriptor, generate_routed call and output extraction in two definitions. This introduces a second implementation of one fixture that will drift. Use one shared test-support implementation, parameterized by the needed wall-clock value and consumed by both the integration binary and crate-root private tests; preserve the private generator/run layer boundary. | tests/it/kani_scalar_native_private.rs:31-135 |
| FND-004 | high | The new independent_scalar_digest duplicates src/kani/run/scalar.rs:1570-1589 independent_digest: both independently JSON-quote the same string-only entry shape, order contents/path keys, concatenate the identical schema wrapper and SHA-256 it. Searching both helpers shows two test implementations of the same independent oracle. Put this test oracle in one shared test-support home and use it from both targets, keeping it independent of production canonicalization and keeping the fixed literal golden as a separate expected value. | tests/it/kani_obligations.rs:3095-3113 |
| FND-005 | medium | A valid helper result followed by a second bare IR791_NATIVE_V1 line is accepted. The new unknown-record check recognizes the tag, but the result-record iterator only counts lines starting with IR791_NATIVE_V1 followed by a tab, so it silently discards the malformed duplicate. The verbatim decoder probe returns Completed(5) and an observation for this output. Count result records by their exact tag, as single_record already does for limits and meter, and reject malformed/repeated native records. This permissive result filter predates the PR; it remains a violation of the newly implemented malformed-helper contract and the new documentation claiming repeated/malformed records refuse. | src/kani/run/scalar.rs:954-972 |

## Coverage

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
- id: FR-017-AC-27
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  excerpt: A genuine routed `x + 1` scalar harness from `generate_routed`, its complete emitted oracle package and
    one in-range concrete operand are passed to the paired runner. One real Kani run reports `Verified`, and the
    runner invokes the public oracle from the same staged library source at that operand with the request's exact
    `ScalarLimits`; the receipt contains the Kani evidence and the actual typed runtime result of that invocation.
    A Kani verdict alone or a result reconstructed by the caller is insufficient.
- id: FR-017-AC-28
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  excerpt: Before either child starts, a package or harness whose path or bytes differ from the retained routed
    output, a selected oracle symbol or argument order that differs from the retained scalar identity, or an operand
    count/value outside the retained ordered ranges is refused with a typed cause and no receipt. For an accepted
    request, the exact supplied `ScalarLimits` are passed unchanged to `KaniScalarNativeRequest`, used to initialize
    the native meter, and returned in the receipt; no hidden or substituted limit value is permitted. CG attests
    these supplied, used and returned values are equal, but does not attest their provenance. The IR-773 driver
    must compare its retained original limits against both the request and receipt before accepting the receipt;
    on mismatch it detects locally, refuses receipt acceptance and skips QSL evaluation. If IR-773 has no defined
    typed outcome for this mismatch, its owner must specify and plan one before implementing the consumer check;
    CG defines no typed outcome for this mismatch. A caller-provided content identity cannot make an inconsistent
    request acceptable.
- id: FR-017-AC-29
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  excerpt: 'The runner measures one canonical proof-content identity over the exact paths and bytes of every staged
    execution input: the complete generated package, retained harness, native invocation adapter and resolved dependency
    lock. This identity binds the content actually proved and executed; it is not a tool-version or file-version
    tracking identity, and no separate identity is added for any staged item. The Kani and native invocations cannot
    write the staged source tree. A changed, replaced or additional staged file before or after either invocation
    yields a typed failure and no receipt. The receipt separately binds the scalar identity, oracle symbol, ordered
    concrete operands and exact supplied `ScalarLimits`; none of these request fields may be substituted after measurement
    or inferred from the proof-content identity.'
- id: FR-017-AC-30
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  excerpt: The public receipt exposes the measured identity, scalar identity, ordered operands, exact supplied and
    meter-used `ScalarLimits`, verified Kani evidence and actual typed native outcome through read-only access.
    Callers cannot construct, deserialize or mutate a receipt, set its identity or outcome, or turn any non-verified,
    refused, inconclusive, faulted or malformed run into a receipt.
- id: FR-017-AC-31
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  excerpt: The paired runner executes an order-sensitive routed scalar operation with two symbolic operands in two
    concrete operand orders. Each real Kani run is verified; the native invocation returns the result for that order;
    and each receipt binds its exact ordered operand vector and typed native result. Swapping the operands changes
    the observed result and receipt context while retaining the same generated proof-content identity.
- id: AD-004-dependency-direction
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "### Dependency direction\n\n```text\npublication --> core\nrouted      --> replay, kani, strategy, oracle,\
    \ core\nreplay      --> kani, core\nkani        --> oracle, core\nstrategy    --> oracle, core\nevidence   \
    \ --> oracle, core\noracle      --> core\ncore        --> (nothing in this crate)\nlib         --> everything\
    \ (re-exports only)\n```\n\nEvery arrow has a basis in code or spec. `evidence --> oracle` is FR-004's `depends_on`\
    \ FR-014 and\nthe `bound_coverage` edge. `strategy --> oracle` is `bound_strategy` and `harness` calling oracle\n\
    generation. `kani --> oracle` is the scalar family embedding the FR-014 oracle. `replay --> kani`\nis the identity\
    \ and binding types the witness decode and replay read. `routed --> ...` is the\n`BackendKind` match calling\
    \ each generator.\n\nRules, each checkable:\n\n- A directory imports only the directories to its right. `strategy`,\
    \ `evidence` and `kani` are peers\n  and import none of each other. `kani` and `replay` import nothing from\
    \ `routed`, and the adapter\n  trait, when it exists, is defined in the reserved `routed/adapter.rs` with the\
    \ Kani\n  implementation beside it. The driver\n  (`quire-driver`, outside this crate) pairs a Kani outcome\
    \ with a replay result, as QSL's merged\n  ADR-011 T-13 says; no module of this crate does.\n- Imports use a\
    \ module path (`use crate::core::artifact::Artifact`), never an item re-exported\n  from the crate root. `lib.rs`\
    \ is the only file that names a root re-export.\n- Inside `kani/` the order is `abi`, `census`, `identity`,\
    \ then `generate` and `output`, then\n  `classify`, `run`, `terminal`. A file imports only earlier names in\
    \ that order. `output` imports\n  `identity` and `abi` and nothing from `generate`, `classify` or `run`. `run`\
    \ imports `identity`,\n  `output` and `classify` (and `abi`, for `KaniSolver`), not `generate`: the harness\
    \ and identity\n  record types it needs live in `kani/identity.rs` (step 2b), so a harness is run from its identity\n\
    \  and its source text.\n- Inside `kani/generate/` the order is `outcome`, `record`, `spec`, `render`, `census_validation`,\
    \ then the families\n  (`scalar` and `collection`, `clause`, then `precondition` and `contract`, `frame`, `lower`,\n\
    \  `corpus`, `v1_bundle`), then `negotiate`, which imports `outcome`, `record`, `scalar`,\n  `collection`, `clause`,\
    \ `precondition`, `contract` and `frame` (the `StateFrame` arm of step 4d\n  calls the frame family's role function;\
    \ none of `lower`, `corpus`, `v1_bundle` or\n  `census_validation`) and is imported by none. `collection` imports\
    \ `outcome` and `record` from\n  this directory, besides `core`, `kani::abi` and `kani::identity`, and imports\
    \ no other crate\n  path, so never `negotiate`, `lower` or `corpus` either (the layout test lists exactly these\n\
    \  five prefixes for `collection.rs`); `scalar` and `collection` share a rank, so neither imports the other.\
    \ `spec` imports `outcome` (for `MAX_OBLIGATION_UNWIND`), `core::identity` (for `HarnessPath`) and `kani::abi`;\
    \ `render` imports `spec`, `outcome`, `core::artifact` and `kani::abi`, and not `record`; a family that has\
    \ been ported (scalar at 4b) imports `spec` and `render`. A family file may import `outcome` and `record` and\
    \ never `negotiate`: the passes that read the whole request\n  (classification, name settlement, assumption\
    \ resolution) are `negotiate`'s, and what lowers or\n  renders one item is its family's. `corpus` imports `lower`\
    \ and `census_validation`; `v1_bundle`\n  imports `census_validation`; `frame` imports `outcome` only. `kani/test_support.rs`\
    \ is\n  `#[cfg(test)]`, imports `identity` and `abi` and no generator, and may be imported by any test\n  module\
    \ under `kani/`.\n- A `#[cfg(test)]` module obeys the same direction as the file it sits in. A test that needs\
    \ a\n  generator as a fixture lives in `tests/it/`, not in the runner's file. The back-edge in the pair\n  `kani_execution`\
    \ and `kani_transcript` (the `kani_transcript` tests that import\n  `classify_kani_run`, `KaniInconclusiveReason`\
    \ and `KaniRunOutcome`) is not moved to `tests/it/`:\n  it needs no generator, it is a classification test,\
    \ and classification is a later file than the\n  report parse, so those tests move to `kani/classify.rs`, which\
    \ may import `output`. The tests\n  that need only the report parse stay with `output/report.rs`. The step 2f\
    \ item map decides each\n  test by name.\n- One interim exception to the typed-values target, recorded as the\
    \ code has it. `classify_kani_run`\n  takes the console text beside the typed report, and `classify_report`\
    \ calls\n  `counterexample_playback` on it for a falsifying check, so `kani/classify.rs` imports\n  `kani/output/playback.rs`.\
    \ The Kani run table below records the target and today's reading side\n  by side. The edge points in the allowed\
    \ direction (`classify` is after `output`), so no order rule\n  is broken; the exception is to L-6 (the classifier\
    \ takes typed values) and is listed there. It\n  holds before this AD, and step 2f keeps it as it is. Step 5\
    \ removes it by changing the classifier's\n  input from the text to the playback `output/playback.rs` extracts;\
    \ that signature change is public\n  (`classify_kani_run` is `pub` so tests can reach the production classifier)\
    \ and is step 5's to\n  design.\n- `serde_json::Value` is named only in `core/ir`, `core/canonical` and the\
    \ report parser. No other\n  file reads a field of a body term.\n- `BoundPackage` and `BoundClause` are named\
    \ only in `strategy/`, `evidence/`, `oracle/boolean_v1.rs`,\n  `oracle/bound_v1.rs` and the V1 arm that step\
    \ 4f deletes. After step 4f none is named in `kani/`,\n  `routed/` or `replay/`."
- id: AD-004-scalar-port
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: context_only
  excerpt: "**Decision P-1. The scalar port.** `scalar.rs` keeps `lower_scalar_claim`, `ScalarOperation`,\n`derive_domain`,\
    \ `unsatisfiable` and `scalar_stem` unchanged. `render_scalar` becomes\n`scalar_harness` (same arguments and\
    \ result), and it does four things in order: build the\n`HarnessSpec` from the lowered claim (`path`, one `Input`\
    \ group per operand, the unlimited-meter\n`Let`, the oracle `Call`, the `exact`, `admitted` and `sound` `Let`s,\
    \ the one `Assert` on `sound`,\nand the `completed` binding with its cover condition), call `render`, build\
    \ the\n`ScalarObligationIdentity` from the same lowered claim and the spec's `unwind`, `solver` and\n`options`,\
    \ and call `record`. The helpers `native_expression` and `operand_names` stay as the\nsource of the `ExprText`s.\
    \ The rename's whole surface is: the import and call in `negotiate.rs`\n(`:85`, `:135`) and its module doc (`:23`),\
    \ the doc mentions in `src/oracle/scalar/mod.rs`\n(`:637`, `:3158`), `tests/exact_scalar_support/agreement_cases.rs`\
    \ (`:98`, `:102`) and\n`scalar.rs`'s own intra-doc links. The code PR may keep the old name instead; nothing\
    \ in this\ncontract needs the rename.\n\n**Decision P-2. The port is byte-identical.** For every claim `render_scalar`\
    \ accepts today, the\nrendered `Artifact` (path and source bytes), the `ScalarObligationIdentity`, the `options`,\
    \ the\n`ObligationRecord` and every digest derived from them equal what the pre-port renderer computes.\nThe\
    \ port introduces no explicitly-specified difference. Rationale: byte identity makes the\nevidence a comparison,\
    \ needs no new real-Kani run to establish that the old verdicts still hold,\nand keeps the identity digest of\
    \ AD-003 stable for callers. A difference of any kind, including\nwhitespace, is a spec change to this subsection\
    \ before it is a code change, and if one is\naccepted the real-Kani lane of P-3 item 2 becomes mandatory for\
    \ it.\n\n**Decision P-3. Evidence for the port (TC-025 steps 42 to 47).**\n\n1. A differential test. The code\
    \ PR keeps the body of today's `render_scalar` as a `#[cfg(test)]`\n   reference function, test-only, and compares\
    \ the ported path against it, byte for byte, over a\n   fixed claim set: each of the four operations (`add`,\
    \ `subtract`, `multiply`, `negate`), a claim\n   with a literal operand, a claim with an inferred operand, a\
    \ claim with each operand at an `i64`\n   extreme, and claims at two unwind values (including `MAX_OBLIGATION_UNWIND`)\
    \ and two ceilings\n   values. The compared values are the artifact path and bytes, the identity, `options`\
    \ and the\n   record. There is no bless or regenerate mode: the reference is code, not a stored file. The\n\
    \   reference copy is deleted when step 4g retires the scalar render path or sooner if the reviewer\n   agrees\
    \ that the L-4 and AC-58 evidence has replaced it.\n2. The `kani` lane scalar tests of `tests/it/kani_obligations.rs`\
    \ run once under the installed\n   backend after the port, as the smoke that the rendered source still verifies:\
    \ the existing\n   scalar mutation control (`tests/it/kani_obligations.rs:2950`, FR-015-AC-37: the oracle's\
    \ `Add`\n   swapped for `Subtract`) classifies `Falsified`, and the unmutated harness classifies `Verified`\n\
    \   with its cover satisfied. This lane is a smoke, not the proof of equivalence: item 1 is.\n   QSL's `left\
    \ + right + 1` control (QSL-342 QI #10, L-5) is a quire-integration test and not\n   part of this PR's evidence.\n\
    3. The L-4 control: a new hand-built `HarnessSpec` with an unsatisfiable `Assume`, rendered by\n   `render`\
    \ into a generated crate and run under the installed backend, does not classify\n   `Verified` (it classifies\
    \ `CoverUnsatisfied`). No scalar claim can carry this, so it is\n   built from the spec, which is why `Assume`\
    \ is in the 4b shape.\n4. The FR-015-AC-58 inspection still drives every emitting entry point; the ported scalar\n\
    \   harness is emitted by `render`, so that inspection now reaches `render.rs` for scalar. The\n   guard's list\
    \ of driven files loses `scalar.rs` and gains `render.rs`.\n5. Constructor and renderer tests over hand-built\
    \ `HarnessSpec`s that no family produces (see\n   TC-025 steps 42 and 43)."
- id: AD-003-E-1
  path: spec/assurance/AD-003-evidence-chain.md
  role: context_only
  excerpt: "- E-1. CG computes QSL's `ObligationIdentity` by one function over the ADR-013 O-09 members\n  (the\
    \ clause or application node id, its occurrence key, the obligation kind, and the\n  arguments each as parameter\
    \ node id and declared domain), excluding the source span, encoded\n  as RFC 8785 JSON in CG's one canonical-encoding\
    \ place by `quire_canonical`, never by\n  `serde_json`;\n  the value changes when any included member changes\
    \ and does not when the span changes. The\n  V1 contract and frame paths (`StateFrameIdentity`) use this parameter-domain\
    \ shape; the scalar\n  operation-application path uses the positional allocation below through the same canonical\n\
    \  encoding authority. The function path (`call_site` over a `QualifiedName`, ADR-013 O-09 as amended by QSL-352)\
    \ uses it\n  too, with the checked function node id and its `declaration` occurrence key in place of the\n \
    \ clause's, both read from `FunctionSite` and never derived by CG, and the existing `ObligationKind` of the\n\
    \  harness replayed (O-09 adds no subject tag and no new kind: one identity per kind the function\n  requests;\
    \ FR-016-AC-21 to AC-23, implemented for the function path and, by FR-024-AC-20 to\n  FR-024-AC-24 (IR-459),\
    \ for the frame path: the V1 contract and scalar paths do not compute it\n  yet). The frame path's subject is\
    \ the operation's frame node\n  and its occurrence the frame's own occurrence, both read from `OperationSite`\
    \ as the envelope's\n  `clause_node` and `occurrence_key` are (FR-015-AC-34), mapped into the same four members\
    \ under\n  their existing names; QSL gives each operation its own frame occurrence, so no member is added\n\
    \  and E-1 is not widened. A frame has no parameters (its subject is `fn(&mut State)`), so its\n  `arguments`\
    \ are empty: the state fields and their ranges are not members, because the frame\n  node names the grants and\
    \ the ranges are the model's and are checked at replay (FR-024-AC-21, FR-024-AC-26). The harness is tied to\
    \ the identity\n  by the replay's checks (decode against `state_fields`, field set, domain, pre state and scope),\n\
    \  not by the digest. The harness's clause node, module and harness symbols, paths, unwind bound\n  and options\
    \ are not members either: they name the generated artifact and the harness identity\n  record (FR-015-AC-48\
    \ keeps the two distinct), and two postcondition clauses of one operation\n  share one frame identity. The function's\
    \ `arguments` are all of its parameters\n  (O-09): a harness that leaves a parameter without an argument, or\
    \ declares an integer with no\n  bound (AD-016 arrow 5: `requires-bound`, never narrowed implicitly) or a Boolean\
    \ with bounds,\n  has no identity and is refused with a typed error. The retained per-argument bound is the\n\
    \  per-argument domain AD-016 puts in the identity; it is not a subset of the parameters.\n  The closed preimage,\
    \ CG's own spelling (O-09 fixes the members, and no QSL or QSpec text pins\n  the member names, the domain encoding,\
    \ the node-id text form or a digest label as of this\n  revision; AD-016 TK-05, the seed vector `obligationIdentitySha256`,\
    \ is open): one RFC 8785\n  object with members `function` (the function node id, or for a state frame the frame\
    \ node id,\n  as 64 lowercase hex digits), `declaration` (`node` as 64 lowercase hex digits, `role` as the\n\
    \  role string, `ordinal` as a number: the function's `declaration` key, or the frame's own),\n  `kind` (the\
    \ `ObligationKind` in snake case), and `arguments`, an array ascending by the\n  parameter's declared identifier\
    \ (empty for a state frame), each `{domain, parameter}` with\n  `parameter` the parameter node id in lowercase\
    \ hex and `domain` either `{\"type\":\"boolean\"}` or\n  `{\"type\":\"integerRange\",\"minimum\":\"<decimal>\"\
    ,\"maximum\":\"<decimal>\"}` (bounds are decimal\n  strings because RFC 8785 integers above 2^53 are not exact).\
    \ The digest is the plain SHA-256 of\n  that text, with no domain label: interim, until QC-4 / TK-07 may add\
    \ an FR-201 domain, which\n  would change every identity. The golden text of the FR-016-AC-21 tests\n  (`tests/it/skeleton_spine.rs`,\
    \ `recomputed`) is the vector; it is written by hand, not by CG's\n  encoder. For a frame the names `function`\
    \ and `declaration` read awkwardly (they hold the frame\n  node and its occurrence key) and are kept, so that\
    \ no function-path identity changes\n  (FR-024-AC-20). If QSL later recomputes or compares the identity, QSL\
    \ pins the spelling and CG\n  follows in a follow-up; until then the spelling is CG's own and interim.\n- E-1\
    \ scalar allocation (PLANNED/CODE-GATED, IR-631). CG owns and mints the scalar\n  operation-application preimage\
    \ under measured merged QSL change #650. ADR-013 O-09,\n  FR-357 and `qsl-replay/src/execute/parity_identity.rs`\
    \ agree on tagged `domain` entries.\n  CG shall construct the owning `ParityPreimage` and call public `parity_obligation`,\
    \ the\n  shared scalar/composite mint function using ADR-013 section 2's `quire-canonical` encoder.\n  QSL owns\
    \ that sole parity encoder; CG calls it through its existing dependency and adds\n  no obligation-canonical-byte\
    \ encoder. The FR-032 vector is a cross-check only.\n  `operator_parity.rs` authenticates application/operator/function\
    \ membership, occurrence\n  and positional operands before verifying the digest. Mismatch refuses\n  `ScalarIdentity::Obligation`\
    \ with claimed/recomputed digests; encoder refusal returns\n  `ScalarIdentity::Encoding` without an identity\
    \ (`stale_dependency`/`revision-mismatch`).\n  The driver calls `replay_operator_parity(wire, claim, replay_limits)`,\
    \ `ReplayLimits` last\n  under merged change #645; this byte guard is distinct from claim `ScalarLimits` and\
    \ is\n  not a full-claim identity member. Both contexts must be retained.\n  This is source inspection, not\
    \ passing-test evidence; actual CG conformance remains gated.\n  Function/clause carry-only contracts and existing\
    \ function/frame encoding remain unchanged.\n  The subject is the claimed operator's FR-322 application node\
    \ id with its authentic O-07\n  `CheckedOccurrence` key (node, `expression` role, ordinal), the scalar kind\
    \ allocated once\n  by FR-032 (typed generation source CODE-gated), and one argument per operand position in\
    \ operand/harness draw order. Each entry is its position\n  ordinal, typed `OperandIdentity` and actual harness\
    \ operand range. Its tagged representation\n  is `GraphChild { node_id }` for an actual graph reference (parameter,\
    \ subterm or an existing\n  literal node), or `InlineLiteral { node_id, occurrence_key, position }` using the\
    \ authentic\n  application node id, selected occurrence key and position ordinal. The inline literal's value\n\
    \  is carried by its singleton range (FR-015 AC-16); QSL emits no extra literal node for it.\n  The typed identifier\
    \ member holds this representation without fabricating a checked node id.\n  Repeated independent draws of one\
    \ parameter remain distinct positional\n  entries. Scalar entries are neither identifier-sorted nor deduplicated\
    \ by node id; unrelated\n  enclosing-function parameters are absent. Function/frame preimages and identifier\
    \ ordering\n  remain unchanged. The exact scalar preimage, minted through the shared owning function, is\n \
    \ `{arguments, node, obligation_kind, occurrence_key}`; occurrence is `{ordinal, role}`, and\n  ordered arguments\
    \ are `{domain, operand, position}` with scalar domain\n  `{lower, tag: \"range\", upper}`, both inclusive bounds\
    \ as decimal strings.\n  Operand is `{node_id, tag: \"graph_child\"}` or\n  `{node_id, occurrence_key, position,\
    \ tag: \"inline_literal\"}` using the application id/key in\n  the inline arm. Node ids are lowercase hex; ordinals/positions\
    \ numeric; SHA-256 hashes the\n  RFC 8785 bytes with no added digest-label preimage member. Independent CG conformance\
    \ checks must match that QSL\n  encoding; no copied encoder or additional CG member enters it. Source spans,\
    \ native\n  outcomes, transcript, generated-content identity, operator label, renderer counters and\n  build/tool/version\
    \ fields remain excluded.\n  [FR-032](../replay/functional/FR-032-routed-scalar-replay-binding.md) owns acceptance\
    \ criteria\n  and the metadata/accessor gate. IR's `CheckedPackageV2::graph()` exposes authentic node ids\n\
    \  and occurrences, and `source_map()` the source map, but the public node body remains JSON;\n  no public typed\
    \ scalar ordered-operand identity/range accessor was measured. IR-648 in\n  Contract IR owns that `CheckedPackageV2`\
    \ accessor and blocks IR-631 CODE admission; its\n  specification and implementation remain to be measured.\
    \ The model-field\n  accessor and wrapped optional leaf walk do not provide one. CG must retain the sole authentic\n\
    \  expression occurrence and operand metadata. Zero/multiple\n  expression occurrences refuse without a chosen\
    \ ordinal; multi-occurrence replay awaits\n  generation-owned selection/retention. Current clause-role `ObligationKind`\
    \ supplies no\n  scalar wire kind: FR-032 owns its sole allocation; typed generation source/retention\n  remains\
    \ CODE-gated. IR-648 must expose that same tagged\n  identity; the inline arm names application/occurrence/position\
    \ and needs no literal node.\n  Missing or ambiguous metadata/kind refuses without minting an identity.\n  Merged\
    \ QSL authenticates the occurrence, operand count, each graph-child reference and\n  inline literal value/singleton\
    \ range before digest verification, and encoding errors refuse\n  without an identity. Graph-child range correspondence\
    \ to the declared proving domain remains\n  a CG validation obligation with positional provenance: own bounded-domain\
    \ parameters or\n  subterms use the checked derived interval, literals use the actual singleton, and untyped\n\
    \  nonliteral subterms use the enclosing scalar claim's first checked result bound as current\n  generation\
    \ does. Missing provenance or a widened range refuses; QSL's child-identity check\n  alone does not establish\
    \ it.\n  Literal and repeated-parameter semantics are resolved; authentic metadata access, retention\n  and\
    \ actual consumer conformance remain CODE gates. Decoder/context work is independent.\n  The same-artifact content\
    \ identity is a separate canonical proof-content tie. The driver\n  executes the same proved generated artifact\
    \ at decoded operands and retained renderer limits,\n  authenticates its typed observation against CG's retained\
    \ generation/proving record, then\n  completes CG's checked plan. This trusts the driver's actual execution/authentication;\
    \ an\n  unchecked digest/outcome echo or regenerated oracle is insufficient. QSL never sees the\n  artifact\
    \ and does not authenticate it: it carries the `DigestRecord` and evaluates exact\n  semantics. Before any scalar\
    \ settlement, CG compares every member of the report's full claim\n  with the retained sent claim at the immutable-report/retained-binding\
    \ converter boundary,\n  plus checked driver completion's actual wire/run context against the original proving\n\
    \  package/function/source/dependencies/limits. The report and O-09 do not bind that wire;\n  an equal echoed\
    \ claim is insufficient. This independent R-7 retention/check is CODE-gated\n  and applies on every outcome,\
    \ including refusal, exact `Incomplete`,\n  generated fault and divergence. Report `claim()` on every outcome\
    \ is measured in merged\n  change #650; actual CG conformance remains CODE-gated; agreement-only claim identity\
    \ and opaque obligation-digest equality are insufficient.\n- E-1 composite extension (PARTIAL: IR-635 delivered\
    \ the Eq graph-child positional O-09 and Ne parameter/parameter and parameter/literal controls of FR-033-AC-11;\
    \ Ne literal/literal, Ne self-comparison and the inline-literal refusals (FR-033-AC-14 to AC-17) are PLANNED\
    \ under IR-814; UNRUN CG CONSUMER, QSL-640). CG's\n  composite O-09 preimage contains only the exact claimed\
    \ equality/inequality application\n  node, its authentic occurrence key from the recompiled original package,\
    \ the obligation\n  kind and one argument per operand position in operand order. A self-comparison contributes\n\
    \  two graph-child argument entries for the same actual parameter, each with only its actual\n  Node-keyed harness\
    \ bounds naming that parameter. Population-keyed metadata remains only in the full\n  CG proving record, outside\
    \ the admitted QSL composite claim and O-09 arguments. A QSL request\n  carrying a Population-keyed harness\
    \ bound receives typed `HarnessUnknownKey` during `prepare`,\n  before identity tying; its binding-checked `Refused`\
    \ report retains ReplayRefused terminal mapping.\n  CG pre-invocation refusal still produces no report or terminal\
    \ value. Repeated positions are not a distinct-node set.\n  A composite literal uses its own actual graph-child\
    \ node with empty Bounds. The owning operand projection\n  refuses every top-level inline literal, so CG builds\
    \ no inline argument and returns that\n  refusal without a report. Within each Bounds argument, keys sort by\
    \ canonical encoded\n  key bytes, not DomainKey ordering, and duplicate keys refuse within that argument.\n\
    \  Repeated parameter positions retain separate argument entries with the same admitted\n  bounds; they are\
    \ not duplicate keys inside one Bounds argument.\n  CG shall construct the owning typed preimage and use the\
    \ public shared `parity_obligation`\n  mint, with no copied or second parity encoder. The full FR-015 AC-76\
    \ CG record keeps\n  abstractions, size budget, static closure pair-node count and unexercised behaviours outside\n\
    \  this preimage. The static record count is distinct from QSL's runtime occurrence-pair count.\n  Existing\
    \ function/frame member spelling, domain encoding and digest preimages stay\n  unchanged. QSL's owning composite\
    \ identity/parity interfaces are delivered; actual CG\n  metadata retention, typed consumer conformance and\
    \ generated-family evidence remain\n  CODE-gated and UNRUN. The request's O-09 identity is checked against actual\
    \ original\n  node/occurrence/operand membership before settlement, including Disagreed. Full retained\n  sent-claim\
    \ comparison on every report outcome and independent same-proved-artifact/context\n  authentication remain required\
    \ under E-2 and FR-033. O-09 or an echoed content identity\n  alone cannot establish that artifact tie; no operator/native/content/refinement/counter\n\
    \  top-level O-09 member or tracking digest is added."
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
- id: src/kani/run/scalar.rs
  path: src/kani/run/scalar.rs
  role: examined
  excerpt: "//! Paired proof and native execution of one retained routed scalar artifact (FR-017).\n\nuse std::{\n\
    \    fmt, fs,\n    path::{Path, PathBuf},\n    sync::atomic::{AtomicU64, Ordering},\n    time::Instant,\n};\n\
    \nuse qsl_replay::{DigestDomain, DigestRecord};\nuse quire_canonical::{FixedShape, Limits};\nuse quire_contract_runtime::exact::{\n\
    \    ChargePoint, Incomplete, Integer, LimitKind, Outcome, Refusal, ScalarLimits, Undefined,\n};\nuse quire_exact::Cancel;\n\
    use serde::Serialize;\n\nuse crate::{\n    core::{\n        artifact::{Artifact, ArtifactBundle, PublicationDiagnostic},\n\
    \        canonical::{content_digest"
- id: src/kani/generate/scalar.rs
  path: src/kani/generate/scalar.rs
  role: examined
  excerpt: "//! The scalar family: lowering an IR-confirmed V2 exact-scalar claim and rendering its Kani\n//! harness\
    \ (FR-015).\n\nuse crate::oracle::operator_catalog::{\n    Operator as CatalogOperator, ScalarKaniOperator as\
    \ ScalarOperation,\n};\nuse quire_contract_model::{CheckedNodeId, CheckedSemanticNodeV2};\n\n#[cfg(test)]\n\
    use crate::{\n    core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES},\n    kani::abi::adapter_options,\n\
    };\n\nuse crate::{\n    kani::abi::{i64_literal, readable_component, KaniSolver},\n    kani::generate::outcome::{DerivedDomain,\
    \ KaniObligationRequest, UnsupportedObligation},\n    kani::gene"
- id: src/kani/generate/mod.rs
  path: src/kani/generate/mod.rs
  role: examined
  excerpt: '//! The one Kani generator: the request and result vocabulary, the negotiation entry and the

    //! family lowerers.


    // Census validation; interim, see the file header.

    pub(crate) mod census_validation;

    // The V1 clause lowering the precondition and contract families share.

    pub(crate) mod clause;

    // Bounded collection claims over admitted V2 model fields.

    pub(crate) mod collection;

    // The contract family.

    pub(crate) mod contract;

    // The bounded-Kani corpus family.

    pub(crate) mod corpus;

    // The state-frame family.

    // Implements: FR-015 (IR-412: state-clause operation contract and frame effects).

    '
- id: src/lib.rs
  path: src/lib.rs
  role: examined
  excerpt: '//! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.


    #![forbid(unsafe_code)]

    #![deny(missing_docs)]


    // Integration fixture modules name this crate as an external dependency.

    #[cfg(test)]

    extern crate self as quire_contract_codegen;


    // Private integration control needs internal generation and execution in the library test binary.

    #[cfg(test)]

    #[path = "../tests/it/kani_render_private.rs"]

    mod kani_render_private;


    #[cfg(test)]

    #[path = "../tests/it/kani_scalar_native_private.rs"]

    mod kani_scalar_native_private;


    // The leaf directory: artifact, diagnost'
- id: tests/it/kani_obligations.rs
  path: tests/it/kani_obligations.rs
  role: examined
  excerpt: '//! FR-015 separate bounded Kani obligations, FR-017 Kani execution, and Contract IR FR-036

    //! backend negotiation.

    //!

    //! The default lane checks negotiation, refusal and harness shape without running Kani. The

    //! `kani` lane (`make kani`, `#[ignore]` here) measures the installed backend, runs real

    //! verified and seeded-failing harnesses one at a time, and writes execution evidence under

    //! `CARGO_TARGET_TMPDIR/kani-obligation-evidence`.


    // package.rs holds a process-global `application_registry()` static keyed by small integer

    // fixture codes that this file and `exact_scalar_generati'
- id: tests/it/kani_scalar_native_private.rs
  path: tests/it/kani_scalar_native_private.rs
  role: examined
  excerpt: "//! Private orchestration controls for scalar-native cleanup after real execution.\n#[path = \"../exact_scalar_support/package.rs\"\
    ]\nmod package;\nuse crate::{\n    core::artifact::Artifact,\n    core::contract_package::{\n        routed_contract_lowering_profile,\
    \ ContractDriverResultSelection, ContractRequestPosition,\n    },\n    kani::{\n        identity::KaniScalarObligationHarness,\n\
    \        run::{\n            scalar::{execute_with_cleanup, KaniScalarNativeRequest},\n            tool::KaniInstallation,\n\
    \        },\n    },\n    routed::{\n        capability::{BackendDescriptor, Candidate, ProviderOrigin},\n"
- id: tests/it/kani_scalar_native_private.rs::routed_scalar_with_artifacts
  path: tests/it/kani_scalar_native_private.rs
  role: examined
  excerpt: "fn routed_scalar_with_artifacts(\n    mut builder: package::PackageBuilder,\n    operation_code: u32,\n\
    \    narrow_code: u32,\n    result_bound: Bound,\n    wall_clock: Duration,\n) -> (KaniScalarObligationHarness,\
    \ Vec<Artifact>) {\n    "
- id: tests/it/kani_obligations.rs::independent_scalar_digest
  path: tests/it/kani_obligations.rs
  role: examined
  excerpt: "fn independent_scalar_digest(entries: &[(String, Vec<u8>)]) -> [u8; 32] {\n    use sha2::{Digest, Sha256};\n\
    \    let encoded_entries = entries\n        .iter()\n        .map(|(path, bytes)| {\n            format!(\n\
    \                \"{{\\\"contents\\\":{},\\\"path\\\":{}}}\",\n                serde_json::to_string(std::str::from_utf8(bytes).unwrap()).unwrap(),\n\
    \                serde_json::to_string(path).unwrap()\n            )\n        })\n        .collect::<Vec<_>>()\n\
    \        .join(\",\");\n    let jcs = format!(\"{{\\\"artifacts\\\":[{encoded_entries}],\\\"schema\\\":\\\"\
    agent-ix/cg-kani-scalar-proof-content/v2\\\"}}\");\n    Sha256::digest(jcs.as_bytes()).into()\n}\n"
bindings:
- test_id: src/kani/run/scalar.rs::tests::frozen_outer_code_and_class_table_has_exactly_twelve_distinct_codes
  ac_id: FR-017-AC-41
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::lock_failures_distinguish_codegen_configuration_from_staged_lock
  ac_id: FR-017-AC-42
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::frozen_outer_code_and_class_table_has_exactly_twelve_distinct_codes
  ac_id: FR-017-AC-42
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::frozen_evidence_classification_covers_all_outcomes_reasons_and_causes_unchanged
  ac_id: FR-017-AC-42
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::frozen_nested_refusal_table_preserves_nested_output_codes
  ac_id: FR-017-AC-42
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::native_launch_failure_is_an_environment_fault_without_a_receipt
  ac_id: FR-017-AC-42
  trace: correct
- test_id: tests/it/kani_scalar_native_private.rs::bounded_single_run_standins_preserve_cancelled_and_timed_out_evidence
  ac_id: FR-017-AC-42
  trace: correct
- test_id: tests/it/kani_scalar_native_private.rs::seeded_false_bound_harness_produces_a_paired_completion_refusal
  ac_id: FR-017-AC-42
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::native_decoder_retains_actual_limits_and_rejects_each_mismatch
  ac_id: FR-017-AC-43
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::literal_seven_entry_jcs_golden_and_mutation_controls
  ac_id: FR-017-AC-43
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::lock_failures_distinguish_codegen_configuration_from_staged_lock
  ac_id: FR-017-AC-44
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::frozen_outer_code_and_class_table_has_exactly_twelve_distinct_codes
  ac_id: FR-017-AC-44
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::frozen_evidence_classification_covers_all_outcomes_reasons_and_causes_unchanged
  ac_id: FR-017-AC-44
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::frozen_nested_refusal_table_preserves_nested_output_codes
  ac_id: FR-017-AC-44
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::literal_seven_entry_jcs_golden_and_mutation_controls
  ac_id: FR-017-AC-45
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  ac_id: FR-017-AC-45
  trace: correct
- test_id: tests/it/kani_scalar_native_private.rs::cleanup_success_and_failure_preserve_real_receipt_after_snapshot_removal
  ac_id: FR-017-AC-45
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::literal_seven_entry_jcs_golden_and_mutation_controls
  ac_id: FR-017-AC-46
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::owned_measurement_survives_explicit_cleanup_and_reports_invalid_utf8
  ac_id: FR-017-AC-46
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  ac_id: FR-017-AC-46
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  ac_id: FR-017-AC-47
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders
  ac_id: FR-017-AC-47
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::owned_measurement_survives_explicit_cleanup_and_reports_invalid_utf8
  ac_id: FR-017-AC-48
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  ac_id: FR-017-AC-48
  trace: correct
- test_id: tests/it/kani_scalar_native_private.rs::cleanup_success_and_failure_preserve_real_receipt_after_snapshot_removal
  ac_id: FR-017-AC-48
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::literal_seven_entry_jcs_golden_and_mutation_controls
  ac_id: FR-017-AC-49
  trace: correct
- test_id: src/kani/run/scalar.rs::tests::native_decoder_retains_actual_limits_and_rejects_each_mismatch
  ac_id: FR-017-AC-50
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  ac_id: FR-017-AC-50
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders
  ac_id: FR-017-AC-50
  trace: correct
- test_id: tests/it/kani_scalar_native_private.rs::cleanup_success_and_failure_preserve_real_receipt_after_snapshot_removal
  ac_id: FR-017-AC-50
  trace: correct
```

## Dispositions

### Pass 1

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The production early read now preserves InvalidData as Snapshot before revision parsing. The real-file UTF-8 regression is green; matching, mismatched and unreadable lock semantics remain explicitly covered. |
| FND-002 | fixed | Independent literal expected_error/outcome/reason/cause/nested lookups now exhaustively match their vocabularies. Five removed-arm compiler controls each produce E0004, while the restored frozen-head units pass. |
| FND-003 | fixed | Only one admitted routing fixture implementation remains, parameterized by wall-clock budget. Both original consumers include it while retaining their isolated package registry and caller budget helper. |
| FND-004 | fixed | Only one independent_digest implementation remains. Runner units, integration controls and private cleanup recomputation consume it; it does not call production canonicalization, and the separate literal schema/golden is unchanged. |
| FND-005 | fixed | The native-result parser counts exact-tag records like limits and meter. Prepended/appended bare duplicates and a sole bare tag now exercise NativeResult refusals; the new regressions were red before the fix and green afterward. |

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
  after_excerpt: "let lock = match lock {\n        Err(error) if error.kind() == std::io::ErrorKind::InvalidData\
    \ => {\n            return Err(KaniScalarNativeError::Snapshot(error));\n        }\n        other => other.ok(),\n\
    \    };"
  reason: The production early read now preserves InvalidData as Snapshot before revision parsing. The real-file
    UTF-8 regression is green; matching, mismatched and unreadable lock semantics remain explicitly covered.
- fnd: FND-002
  outcome: fixed
  after_excerpt: "fn expected_outcome(\n        outcome: &KaniRunOutcome,\n        cause: Option<&KaniFailureCause>,\n\
    \    ) -> KaniScalarNativeErrorClass {\n        use KaniScalarNativeErrorClass::{CompletionRefusal as R, EnvironmentFault\
    \ as F};\n        match outcome {\n            KaniRunOutcome::Verified => F,\n            KaniRunOutcome::Falsified\
    \ { .. } => R,\n            KaniRunOutcome::CoverUnsatisfied { .. } => R,\n            KaniRunOutcome::Cancelled\
    \ { .. } => F,\n            KaniRunOutcome::Inconclusive { reason } => expected_reason(*reason, cause),\n  \
    \      }\n    }"
  reason: Independent literal expected_error/outcome/reason/cause/nested lookups now exhaustively match their vocabularies.
    Five removed-arm compiler controls each produce E0004, while the restored frozen-head units pass.
- fnd: FND-003
  outcome: fixed
  after_excerpt: "pub(super) fn routed_scalar_with_artifacts(\n    mut builder: package::PackageBuilder,\n    operation_code:\
    \ u32,\n    narrow_code: u32,\n    result_bound: Bound,\n    wall_clock: Duration,\n) -> (KaniScalarObligationHarness,\
    \ Vec<Artifact>) {"
  reason: Only one admitted routing fixture implementation remains, parameterized by wall-clock budget. Both original
    consumers include it while retaining their isolated package registry and caller budget helper.
- fnd: FND-004
  outcome: fixed
  after_excerpt: "//! Independent JCS/SHA-256 oracle for the scalar proof-content string-only shape.\npub(crate)\
    \ fn independent_digest(entries: &[(String, Vec<u8>)]) -> [u8; 32] {\n    use sha2::{Digest, Sha256};\n    let\
    \ artifacts = entries\n        .iter()\n        .map(|(path, bytes)| {\n            format!(\n             \
    \   \"{{\\\"contents\\\":{},\\\"path\\\":{}}}\",\n                serde_json::to_string(std::str::from_utf8(bytes).unwrap()).unwrap(),\n\
    \                serde_json::to_string(path).unwrap()\n            )\n        })\n        .collect::<Vec<_>>()\n\
    \        .join(\",\");\n    let bytes = format!(\n        \"{{\\\"artifacts\\"
  reason: Only one independent_digest implementation remains. Runner units, integration controls and private cleanup
    recomputation consume it; it does not call production canonicalization, and the separate literal schema/golden
    is unchanged.
- fnd: FND-005
  outcome: fixed
  after_excerpt: "    let result = single_record(text, \"IR791_NATIVE_V1\")?;\n    let fields = result.split('\\\
    t').collect::<Vec<_>>();"
  reason: The native-result parser counts exact-tag records like limits and meter. Prepended/appended bare duplicates
    and a sole bare tag now exercise NativeResult refusals; the new regressions were red before the fix and green
    afterward.
```
