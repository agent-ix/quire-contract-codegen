---
id: SR-12761
title: IR-846 ears-analysis review
type: SpecReview
analysis: ears-conformance
scope: agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md
review_set: subset
---

## Summary

Frozen own diff `4d1a1ede2b1795663dff459d5929f04745b1e9d9..89cdf4f36bc60b298a73c46ca8f451f9ebc4fe4e`; ticket IR-846. Maintenance range-diff and spec payload are unchanged from reviewed #419. No Cargo/Kani or branch edits.

## Verdict

PASS — changed requirement prose identifies the scalar runner, receipt, consumer, observation digest, meter observation or preimage/schema as subject and specifies measurable responses. No added EARS defect.

## Evidence

Engine `quire validate --scope . "spec/**/*.md" --summary` passed structural validation: 107/109 docs grammar-clean; three advisory warnings are inherited FR-017 line259 (unclassifiable/missing-subject) and FR-014 line252 (non-canonical-trigger), outside the changed section. Semantically, exposure/retention/nonbinding are ubiquitous obligations; schema change is a discrete event; mismatch is an unwanted condition with typed failure and no receipt. Clauses are concrete and supported by AC45–50. No warning or new finding is manufactured from unchanged statements. AC and TC text are acceptance context, not independently subject to EARS grammar.

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
```
