---
id: SR-12760
title: IR-846 code-review review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md,
  spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/tests.md, spec/replay/functional/FR-032-routed-scalar-replay-binding.md
review_set: subset
---

## Summary

Frozen own diff `4d1a1ede2b1795663dff459d5929f04745b1e9d9..89cdf4f36bc60b298a73c46ca8f451f9ebc4fe4e`; ticket IR-846. Maintenance range-diff and spec payload are unchanged from reviewed #419. No Cargo/Kani or branch edits.

## Verdict

PASS — Markdown-only specification delta introduces no copied implementation, duplicate owner type, weakened gate, invented tracking manifest, or false implementation claim. Canonical proof-content identity is the repository’s explicit permitted digest. QSL owns DigestRecord; this spec requires re-export, not a CG twin. Planned tests remain pending, and no runtime coverage is claimed.

## Evidence

Language-independent duplication, integrity, specification/implementation alignment, and matrix checks ran. No changed Rust/Cargo files: Rust lane and runtime gates do not apply. No changed production code: separate gap-analysis is inapplicable. No applicable AssuranceProfile exists under spec/. Current code retains the old receipt shape; DigestRecord exposure, retained entries, cleanup diagnostics, meter-used limits, non-UTF-8 mapping and identity-encode variant remain explicit implementation work (IR-846/847/857), not completed functionality. `quire matrix --strict` exits 1 with 318 untagged and 14 ignored-only; AC45–50 are all untagged and the changed matrix honestly labels them Planned. This is not a whole-repository PASS.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined scope

```yaml
scope:
- id: FR-017/canonical generated-content identity of the observation
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The measured proof-content identity of FR-017-AC-29 is the value QSL FR-357
    carries as the **canonical generated-content identity of the observation** (QSL
    main `812939fb5`, FR-357 Description and AC-14). This section fixes its digest
    domain, its exact preimage, who mints it and who authenticates it, so the IR-773
    driver can recompute it. It extends FR-017-AC-29 and changes none of its text;
    FR-032 refers here for the representation.
- id: FR-017/Domain
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '- **Domain.** The scalar runner shall expose the identity as a `qsl_replay::DigestRecord`,
    which the crate re-exports at its root (`quire_contract_codegen::DigestRecord`)
    so a consumer names one type and CG defines no twin, whose domain is `sha256-jcs`
    and whose bytes are the 32-byte SHA-256 of the RFC 8785 (JCS) canonical encoding
    of the structural preimage below. `sha256-jcs` is chosen because the preimage
    is a structured JSON document of several artifacts, not one byte string; QSL''s
    `GeneratedArtifactBytesV1` is raw-byte-addressed and would be correct only if
    CG hashed one artifact''s raw by'
- id: FR-017/Preimage
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '- **Preimage.** The preimage is the JSON object `{"schema": "agent-ix/cg-kani-scalar-proof-content/v2",
    "artifacts": [ {"path": P, "contents": C}, ... ]}`. `artifacts` is exactly one
    entry per staged execution input, with these paths: `Cargo.toml`, `claim-map.json`
    and the emitted package''s library renamed to `src/oracle-artifacts-lib.rs` (the
    emitted package holds exactly its `Cargo.toml`, `src/lib.rs` and `claim-map.json`);
    the retained scalar harness source under the path `src/lib.rs`; the retained harness
    record under its own record path; the runner-generated `Cargo.lock`; and the runner-g'
- id: FR-017/Exclusions
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '- **Exclusions.** The preimage binds the generated artifact alone. Operands,
    `ScalarLimits`, operand and result ranges, the native outcome, the Kani evidence,
    the meter observation and the scalar identity are not in it (FR-017-AC-29, AC-31).
    They are separate claim fields that QSL FR-357 binds in the claim identity; the
    observation digest shall not bind them, because binding them twice breaks FR-357-AC-14.
    No tool version is in it beyond the bytes of `Cargo.lock` and the adapter.'
- id: FR-017/Owners
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '- **Owners.** CG mints the identity at proof time, from the staged bytes,
    in `KaniScalarNativeReceipt`; the receipt exposes it read-only as the `DigestRecord`
    above, and its bytes equal `ScalarProofContentIdentity::as_bytes()`. The IR-773
    driver authenticates it, by recomputing it (below) from the artifact it executes
    before it sends the claim. QSL carries it and binds it into the claim identity;
    QSL never mints, recomputes or authenticates it. A consumer shall not mint a wire
    value from operands, limits or outcome; the only legitimate recomputation is the
    driver''s, over the exposed staged ent'
- id: FR-017/Retention for recomputation
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '- **Retention for recomputation.** The digest covers two inputs the caller
    never supplies: the `Cargo.lock` that `cargo generate-lockfile --offline` and
    the runtime-revision pin produce in the private snapshot, and the native adapter
    the runner generates from the harness identity. The receipt shall therefore retain,
    and expose read-only, the complete ordered list of staged `(path, bytes)` entries
    the preimage was built from, including those two. Without that list the driver
    holds only the package and harness it passed in, cannot reproduce those two entries,
    and cannot recompute the digest; tha'
- id: FR-017/Driver follow-up
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '- **Driver follow-up.** quire-driver FR-009 (statement and AC-7) says
    the driver verifies "QSL''s observation digest against that outcome". That conflicts
    with this section and must be amended: the observation digest is artifact-only
    and authenticated by recomputation over the receipt''s exposed entries, not checked
    against the outcome. The amendment is quire-driver PR #78 (FR-009/TC-016), owned
    by the quire-driver team (routed from IR-846) and blocks quire-driver IR-773 AC-6/7.'
- id: FR-017/Meter-used limits (IR-857)
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '- **Meter-used limits (IR-857).** The driver must show that the request
    limits, the limits the meter actually used and the limits the receipt returns
    are the same value before it compares them with its retained original (quire-driver
    FR-009-AC-7; IR-773 AC-12), and the receipt today exposes only `scalar_limits()`.
    The meter observation shall therefore carry the `ScalarLimits` the native meter
    was constructed with, read back from the native invocation itself (the adapter
    reports the limits its meter holds), not copied from the request, and bound to
    the executed snapshot like the other receipt f'
- id: FR-017/Stability
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '- **Stability.** The schema string is the version of the preimage. Any
    change to the artifact set, a path name or rename, the ordering, the content encoding,
    the field names, the canonicalization or the hash shall change the schema string
    (`.../v2` to `.../v3`), and digests under different schema strings are never equal
    or comparable. The domain stays `sha256-jcs`. A change to the bytes of a staged
    input (a different `Cargo.lock` or adapter) changes the digest without a schema
    bump, because it names different proved content. The golden digest is computed
    over fixed literal entries (a literal `'
- id: FR-017-AC-45
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The receipt exposes its proof-content identity as a `DigestRecord`, the
    `qsl_replay::DigestRecord` re-exported at the crate root (no CG-defined twin type;
    a compile test names it by the CG path and by the `qsl_replay` path as the same
    type), with domain `sha256-jcs` and 32 bytes equal to `ScalarProofContentIdentity::as_bytes()`.
    An independent computation in the test, SHA-256 of the RFC 8785 encoding of `{"schema":
    "agent-ix/cg-kani-scalar-proof-content/v2", "artifacts": [...]}` over the entries
    of AC-46, gives the same 32 bytes. The domain is not `GeneratedArtifactBytesV1`.'
- id: FR-017-AC-46
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'For a genuine routed `x + 1` package the preimage artifacts are, in this
    order by UTF-8 byte order of `path`: `Cargo.lock`, `Cargo.toml`, `claim-map.json`,
    the harness record at `kani-obligations/<module>.json` (the only record path the
    runner admits), `src/bin/ir791_native.rs`, `src/lib.rs` holding the harness source,
    and `src/oracle-artifacts-lib.rs` holding the emitted library bytes. Removing,
    renaming, reordering or duplicating any entry, or changing one byte of any entry''s
    contents, changes the digest. A non-UTF-8 runner-read `Cargo.lock` or adapter
    is refused, never lossily converted, as'
- id: FR-017-AC-47
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Two receipts for the same package and harness with different operands,
    different `ScalarLimits` and different native outcomes have equal `DigestRecord`s;
    operand order, limits, ranges, outcome and Kani evidence appear nowhere in the
    preimage. This holds for the swapped-operand runs of FR-017-AC-31.
- id: FR-017-AC-48
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The receipt exposes read-only the ordered staged `(path, bytes)` entries
    the digest was built from, including the runner-generated `Cargo.lock` and `src/bin/ir791_native.rs`.
    Recomputing the digest from only those exposed entries equals the receipt's `DigestRecord`.
    A recomputation that substitutes one byte in the package, harness or harness record
    (which a driver holding different bytes would do) differs from the receipt's digest.
    The exposed entries are exactly the bytes measured for the identity, captured
    at measurement time and held in the receipt; no entry accessor reads the filesystem.
    T
- id: FR-017-AC-49
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A change to the preimage schema (artifact set, path names, ordering, encoding,
    canonicalization or hash) is made only by changing the schema string, with the
    golden digest of AC-46 updated in the same change; the golden is computed over
    fixed literal entries, so dependency bumps do not move it, and is asserted together
    with the schema string in one assertion; a golden change without a schema bump
    is rejected in review (a test cannot enforce it). A change to the bytes of `Cargo.lock`
    or the adapter changes the digest with the schema string unchanged. The domain
    remains `sha256-jcs` across a sch
- id: FR-017-AC-50
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The `NativeMeterObservation` of a receipt exposes read-only the `ScalarLimits`
    the native meter was constructed with. They are read back from the native invocation''s
    own output, not copied from the request: a test seam that makes the adapter report
    a well-formed but different limit value makes the helper''s structured result
    unsupported in FR-017-AC-43''s sense and yields `NativeResult` / `native_result`
    / `EnvironmentFault` and no receipt. For an accepted request the request limits,
    the meter-observed limits and `scalar_limits()` are equal, and the meter-observed
    limits are unchanged after the '
- id: TC-027/scalar-proof-content
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: examined
  excerpt: 'Scalar proof-content digest (FR-017-AC-45 to AC-50): run the paired runner
    on a genuine routed `x + 1` package. (1) Assert the receipt''s `DigestRecord`
    has domain `sha256-jcs` and 32 bytes equal to `as_bytes()`, and recompute SHA-256
    of the RFC 8785 encoding of the schema object and the sorted artifact list in
    the test, independently of the runner''s encoder. (2) Assert the exposed entry
    paths are exactly the AC-46 set in UTF-8 byte order, with the library under `src/oracle-artifacts-lib.rs`
    and the harness under `src/lib.rs`; then drop, rename, reorder, duplicate and
    one-byte-change each entry'
- id: FR-032/digest-boundary
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: 'The digest''s domain (`sha256-jcs`), exact preimage, artifact-only content,
    CG minting and driver recomputation are fixed by FR-017-AC-45 to AC-50 (the "Scalar
    proof-content identity as the FR-357 observation digest" section); the receipt
    carries it as a `DigestRecord`. The remaining producer/consumer wiring stays CODE-gated;
    no signing scheme, tracking hash or new observation API is invented here. ## Outputs
    - A typed admitted scalar binding and QSL-owned replay plan for the operator-level
    lowering-parity arm, or a distinct typed setup refusal. The driver completes the
    plan with its actual nat'
- id: TestMatrix/IR-846
  path: spec/kani/matrix/tests.md
  role: examined
  excerpt: '| FR-017 | FR-017-AC-45 through FR-017-AC-50 | TC-027 | 🚧 Planned (IR-846);
    the scalar proof-content identity as a `sha256-jcs` `DigestRecord`, its exact
    preimage, the artifact-only exclusions, receipt retention of the staged entries
    for driver recomputation, the schema-bump rule and the meter-used limits (IR-857)
    | | FR-017 | FR-017-AC-39 | TC-043 | ✅ Covered (IR-827); a batch member''s CBMC
    failure cause beside the batch timeout rule | | FR-025 | FR-025-AC-1 | TC-036
    | 🚧 Planned; emission order is asserted only for the V1 `BoundClause` harness
    kinds, and the ascending order and the scalar-cla'
- id: FR-017-AC-29
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  excerpt: 'The runner measures one canonical proof-content identity over the exact
    paths and bytes of every staged execution input: the complete generated package,
    retained harness, native invocation adapter and resolved dependency lock. This
    identity binds the content actually proved and executed; it is not a tool-version
    or file-version tracking identity, and no separate identity is added for any staged
    item. The Kani and native invocations cannot write the staged source tree. A changed,
    replaced or additional staged file before or after either invocation yields a
    typed failure and no receipt. The rece'
- id: FR-017-AC-31
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  excerpt: The paired runner executes an order-sensitive routed scalar operation with
    two symbolic operands in two concrete operand orders. Each real Kani run is verified;
    the native invocation returns the result for that order; and each receipt binds
    its exact ordered operand vector and typed native result. Swapping the operands
    changes the observed result and receipt context while retaining the same generated
    proof-content identity.
- id: FR-017-AC-41
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
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
  role: context_only
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
  role: context_only
  excerpt: A canonical-identity encoding failure is `IdentityEncode` with code `identity_encode`
    and class `EnvironmentFault`; no path returns `NativeResult` for it. `NativeResult`
    is produced only for the helper's malformed, missing or unsupported structured
    result, and a forced identity-encode failure through an in-crate test seam returns
    `IdentityEncode`, not `NativeResult`.
- id: FR-017-AC-44
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  excerpt: 'The code table and the mapping to class are frozen: a test lists every
    variant with its expected code and its mapping to class (one class for a fixed
    variant; for `proof_not_verified` the class of each `KaniRunOutcome`, `KaniInconclusiveReason`
    and `KaniFailureCause` value; for `kani_execution` the class of each nested refusal),
    fails if a variant, outcome, reason, cause or nested refusal exists that the list
    lacks or if a code value or a mapped class differs, and fails if two variants
    share a code. Adding a variant, code, class or mapping entry is a spec change
    to the "Scalar native error dis'
```
