<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15291 method=gap-analysis lang=rust pr=quire-contract-codegen#501 model=unavailable run=d9d507d8-42eb-4ddc-a8d0-3eed0b37342f -->

SR-15291 gap-analysis, part 7/18. FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-109 | high | coverage | Inherited FR-019-AC-38 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-110 | high | coverage | Inherited FR-019-AC-39 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-111 | high | coverage | Inherited FR-019-AC-40 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-112 | high | coverage | Inherited FR-019-AC-41 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-113 | high | coverage | Inherited FR-021-AC-30 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-114 | high | coverage | Inherited FR-022-AC-26 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-115 | high | coverage | Inherited FR-022-AC-27 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-116 | high | coverage | Inherited FR-022-AC-28 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-117 | high | coverage | Inherited FR-024-AC-1 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-118 | high | coverage | Inherited FR-024-AC-7 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-119 | high | coverage | Inherited FR-024-AC-8 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-120 | high | coverage | Inherited FR-024-AC-18 is tagged-by-ignored-test at the frozen candidate: every bound test is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-121 | high | coverage | Inherited FR-024-AC-34 is tagged-by-ignored-test at the frozen candidate: every bound test is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-122 | high | coverage | Inherited FR-024-AC-35 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-123 | high | coverage | Inherited FR-024-AC-36 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-124 | high | coverage | Inherited FR-024-AC-37 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-125 | high | coverage | Inherited FR-024-AC-38 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-126 | high | coverage | Inherited FR-024-AC-39 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |

+++ [reviewer data]

```yaml
clean: false
scope:
- id: FR-017-AC-16
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The launcher''s capture threads are stopped and joined on every outcome: they return what
    the launcher wrote before it ended, stop while a write end is still held open, and stop within their
    drain limit while a straggler keeps writing, so a process holding a pipe open does not delay the return.'
- id: FR-017-AC-17
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A run that times out has the whole group the launcher leads killed, a real grandchild included.
- id: FR-017-AC-18
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A report that is malformed, of an unknown check or harness status, of another schema version,
    that holds other than one harness result in a single run, or whose harness states success while it
    lists a failed, errored, undetermined or unknown check (of any class, including an unwinding assertion)
    is refused with its own typed cause and never classified, for every obligation kind; a run that exited
    successfully and exported no report is refused, and one that exited unsuccessfully and exported none
    is `NoVerdict`.
- id: FR-017-AC-19
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The generator appends `-Z unstable-options --export-json /proc/self/fd/N` after unchanged harness
    options, with actual child-only anonymous pipe writer N >= 5, and records the actual argv bytes. It
    uses per-run unnamed report authority, never a named target-directory report or fallback. FR-034 AC-32/33
    own collector bounds, resource-stop classification, kernel lifetime and immutable final handoff. Zero-byte
    EOF means no exported report; nonempty partial/malformed content is refused under AC-18 unless an
    independently established resource/deadline stop takes precedence with the existing single-
- id: FR-017-AC-20
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The evidence and the classified run list every check of a real run with its id, class, source
    file and line and status, a line stated as unknown is absent, and a non-numeric line or a check with
    no location is a refused report; the view serializes as `id`, `class`, `location { file, line }` and
    `status` and is not deserializable.
- id: FR-017-AC-21
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: N harnesses with equal option vectors (the harness selection removed) and equal identity `ProofCeilings`
    (`wall_clock` T and `memory_bytes` M) start exactly one launcher process whose argument vector holds
    one `--harness <module::harness> --exact` pair per member in request order, `--harness-timeout` T,
    then the shared options; N harnesses in G such groups start G processes; so N > 1 compatible harnesses
    start fewer than N processes; one harness starts one process with the single-run argument vector.
    Requests whose wall-clock or memory ceilings differ start separate processes. The group's proc
- id: FR-017-AC-22
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Over a real or captured batch report, members are matched by `module::harness` path (two members
    with the bare symbol `check` stay distinct), and each member''s outcome, checks and SUCCESS count
    come from its own entry: a falsified member beside a verified member leaves the verified member verified;
    a Success member in a non-zero exit with no Failure entry is inconclusive, and with one Failure entry
    is verified; a Failure entry with no checks and exit status `timeout` is inconclusive timed-out naming
    T while the others keep their results; a Failure entry with no checks and no timeout is inconcl'
- id: FR-017-AC-23
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A group's report that lacks a requested harness, holds one twice or holds an unrequested one
    refuses the group with a typed cause and classifies no member; a member whose source is not in the
    crate refuses the whole batch with `HarnessNotInCrate` and launches nothing; a falsified member's
    playback is the first counterexample block headed for its own path and never another member's block,
    a member that fails two property checks (two counterexample blocks under its path, as real Kani prints)
    takes the first, as the same harness alone does, and neither it nor its neighbours lose their evidence,
    a
- id: FR-017-AC-24
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A launcher that exits on its own after leaving a real grandchild in its process group has that
    grandchild killed by the time the run returns, the same as on a timeout.
- id: FR-017-AC-25
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A capture thread that panics, or whose poll or read of the stream errs, refuses the run with
    `kani_output_unread`; the run is never classified from empty text.
- id: FR-017-AC-26
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A generated FR-035 caller Text-admission harness enters the same production execution path
    in single and compatible batch runs; its evidence carries the exact typed caller identity (profile,
    bounds and finite payload class) supplied with that harness and `None` for contract obligation kind.
    A matching successful report with a satisfied cover and successful agreement assertion classifies
    `Verified`; an unsatisfied cover classifies `CoverUnsatisfied` with counts; a failed agreement assertion
    with its own concrete playback classifies `Falsified` with that playback. A source-mismatched harness,
    mi
- id: FR-017-AC-27
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A genuine routed `x + 1` scalar harness from `generate_routed`, its complete emitted oracle
    package and one in-range concrete operand are passed to the paired runner. One real Kani run reports
    `Verified`, and the runner invokes the public oracle from the same staged library source at that operand
    with the request's exact `ScalarLimits`; the receipt contains the Kani evidence and the actual typed
    runtime result of that invocation. A Kani verdict alone or a result reconstructed by the caller is
    insufficient.
- id: FR-017-AC-28
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Before either child starts, a package or harness whose path or bytes differ from the retained
    routed output, a selected oracle symbol or argument order that differs from the retained scalar identity,
    or an operand count/value outside the retained ordered ranges is refused with a typed cause and no
    receipt. For an accepted request, the exact supplied `ScalarLimits` are passed unchanged to `KaniScalarNativeRequest`,
    used to initialize the native meter, and returned in the receipt; no hidden or substituted limit value
    is permitted. CG attests these supplied, used and returned values are equal, bu
- id: FR-017-AC-29
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The runner measures one canonical proof-content identity over the exact paths and bytes of
    every staged execution input: the complete generated package, retained harness, canonical native invocation
    adapter, observer manifest and both independently generated dependency locks. This identity binds
    the content actually proved and executed; it is not a tool-version or file-version tracking identity,
    and no separate identity is added for any staged item. The Kani and native invocations cannot write
    the staged source tree. A changed, replaced or additional staged file before or after either invocati'
- id: FR-017-AC-30
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The public receipt exposes the measured identity, scalar identity, ordered operands, exact
    supplied and meter-used `ScalarLimits`, verified Kani evidence and actual typed native outcome through
    read-only access. Callers cannot construct, deserialize or mutate a receipt, set its identity or outcome,
    or turn any non-verified, refused, inconclusive, faulted or malformed run into a receipt.
- id: FR-017-AC-31
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The paired runner executes an order-sensitive routed scalar operation with two symbolic operands
    in two concrete operand orders. Each real Kani run is verified; the native invocation returns the
    result for that order; and each receipt binds its exact ordered operand vector and typed native result.
    Swapping the operands changes the observed result and receipt context while retaining the same generated
    proof-content identity.
- id: FR-017-AC-32
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'In the `make kani` lane, a crate whose library source holds the generated harness byte for
    byte and, beside it, an item that does not compile (a reference to an undefined name) is run through
    `execute_kani_obligation`: the run exits unsuccessfully, exports no report and is `Inconclusive` `NoVerdict`
    with its exit code, and its evidence carries a diagnostic record whose phase is `compile`, whose stdout
    excerpt holds the rustc error code and Kani''s `error: Failed to execute cargo` line and whose stderr
    excerpt holds Cargo''s `error: could not compile` line with the crate path as `<CRATE_DIR>`. A '
- id: FR-017-AC-33
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Over single-run launcher stand-ins that exit 1: a report of success beside the unsuccessful
    exit is `unknown` even when its stdout holds `Solving with CaDiCaL 3.0.0`; with no report, stdout
    holding the Kani banner, `Checking harness proofs::check...` and `Solving with CaDiCaL 3.0.0` is `solver`;
    the banner and the `Checking harness` line alone is `backend`; the banner and `error: Failed to execute
    cargo (exit status: 101). Found 1 compilation errors.` is `compile`, as is stderr `error: could not
    compile` alone with the banner; a usage error on stderr and nothing on stdout is `launcher`; the ba'
- id: FR-017-AC-34
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'A redacted stream of exactly 8192 bytes is kept whole with `omittedBytes` 0; one of 8193 bytes
    whose final line starts within the window keeps the longest line-aligned suffix of at most 8192 bytes
    with `omittedBytes` the byte count before it; a stream whose final line alone exceeds 8192 bytes keeps
    at most 8192 bytes ending at its last byte, starting on a character boundary; a stream that is one
    10000-byte line followed by `\n` keeps the last 8192 bytes of it, ending in `\n`, never an empty excerpt;
    a stream of a 10000-byte line, `\n`, `error: last\n` keeps exactly `error: last\n`; a 4-byte ch'
- id: FR-017-AC-35
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'With crate directory `/h/u/ws/crate`, target directory `/h/u/ws/crate/target` and home directory
    `/h/u` supplied in the request, a stand-in whose streams print `(/h/u/ws/crate)`, `/h/u/ws/crate/src/lib.rs:6:21`,
    `/h/u/ws/crate/target/debug/x.out`, `/h/u/.kani/bin`, `/h/u/ws/crate2/src/lib.rs`, `/opt/tool/bin`,
    `https://host/x/y`, `and/or`, `/`, `2026-10-16T22:27:04.275975036Z`, `Runtime Symex: 0.00331318s`,
    `Runtime Solver: 1.385e-05s`, `in 0.02s`, `check_ttl_30s`, `u32s`, `pid 4242` and `thread ''main''
    (4242) panicked` yields, in order, `(<CRATE_DIR>)`, `<CRATE_DIR>/src/lib.rs:6:21`, `<TARGET_'
- id: FR-017-AC-36
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The diagnostic record never feeds classification: two runs with the same report and exit and
    different stream text classify to the same outcome, checks, SUCCESS count and playback, and two `NoVerdict`
    runs with equal exit and no report whose streams differ have equal evidence in every field but `diagnostics`,
    `memory` (the observed peak resident bytes vary run to run) and the run''s report-path argument (the
    last argument, unique to each run); the FR-029 terminal value of a `NoVerdict` evidence carrying a
    record is `Failed`, the value FR-029-AC-4 gives the no-verdict reason, equal to the termin'
- id: FR-017-AC-37
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Over single-run launcher stand-ins that exit 1, print `Checking harness proofs::check...`
    and `CBMC failed with status 64` on stdout and `Unknown option: --no-such-option` on stderr, and export
    a report whose one result `proofs::check` is `Failure` with no checks and whose one `error_details`
    entry for `proofs::check` has the exit status shown: `exit_code_64` is `Inconclusive` `FailedWithoutCounterexample`
    with `failureCause` `{ "kind": "exit_code", "code": 64 }` and a `diagnostics` record of phase `unknown`
    whose stderr excerpt holds `Unknown option: --no-such-option`; `exit_code_15` has `cod'
- id: FR-017-AC-38
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'A single-run stand-in that exits 1 and exports a `Failure` report listing a failed assertion
    check with no playback on the console, whose entry for the harness has exit status `properties_failed`,
    is `FailedWithoutCounterexample` with `failureCause` `{ "kind": "properties_failed" }` and no `diagnostics`
    member. The same report with no `error_details` member, with an entry that names another harness only,
    with two entries naming the harness, with an entry whose `exit_status` is null, or whose one result
    names no `harness_id`, is `FailedWithoutCounterexample` with neither a `failureCause` nor a '
- id: FR-017-AC-39
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A batch stand-in of members `a::check`, `b::check`, `c::check`, `d::check` and `e::check` whose
    process exits 1 and whose report has `a::check` `Success` with a passing assertion, every cover satisfied
    and no exit status, `b::check` `Failure` with no checks and exit status `exit_code_64`, `c::check`
    `Failure` with no checks and exit status `timeout`, `d::check` `Failure` listing a failed assertion
    with no playback block headed for its path and exit status `timeout`, and `e::check` `Failure` listing
    a failed assertion with no playback block headed for its path and exit status `properties_failed
- id: FR-017-AC-40
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The cause never feeds classification: three single-run stand-ins with the same `Failure` report,
    no checks and the same exit, whose entries give `exit_code_64`, `out_of_memory` and `cbmc_exploded`
    and whose streams differ, classify to the same outcome, checks, SUCCESS count and playback, and their
    evidence is equal in every field but `failureCause`, `diagnostics`, `memory` and the report-path argument;
    the cause is the only input of the FR-029 terminal value that differs: `exit_code_64` and `cbmc_exploded`
    give `Failed`, equal to the terminal value of the same evidence with `failureCause` and '
- id: FR-017-AC-41
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Every `KaniScalarNativeError` variant has a `code()` that is a total `&''static str` from
    exactly this set: `invalid_request`, `artifact_package`, `snapshot_io`, `kani_execution`, `proof_not_verified`,
    `oracle_stop`, `native_launch`, `dependency_lock`, `native_result`, `snapshot_changed`, `identity_encode`,
    `codegen_lock`. The set is closed and the codes are pairwise distinct. The type is `#[non_exhaustive]`
    (a rustdoc `compile_fail` exhaustive match without a wildcard arm, beside a compiling control with
    one), and `class()` returns a closed `KaniScalarNativeErrorClass` (a compiling exhaustive '
- id: FR-017-AC-42
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '`class()` is `CompletionRefusal` for `invalid_request`, `artifact_package` and `oracle_stop`,
    and `EnvironmentFault` for `snapshot_io`, `native_launch`, `dependency_lock`, `native_result`, `snapshot_changed`,
    `identity_encode` and `codegen_lock`. For `proof_not_verified` it is `CompletionRefusal` for evidence
    outcome `Falsified`, `CoverUnsatisfied`, and `Inconclusive` `MissingCoverSummary`, `VacuousProof`,
    `UnwindBoundExhausted` or `FailedWithoutCounterexample` with no `failureCause` or `properties_failed`,
    and `EnvironmentFault` for `Cancelled`, `Inconclusive` `TimedOut`, `MemoryExhausted` or'
- id: FR-017-AC-43
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A canonical-identity encoding failure is `IdentityEncode` with code `identity_encode` and class
    `EnvironmentFault`; no path returns `NativeResult` for it. `NativeResult` is produced only for the
    helper's malformed, missing or unsupported structured result, and a forced identity-encode failure
    through an in-crate test seam returns `IdentityEncode`, not `NativeResult`.
- id: FR-017-AC-44
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The code table and the mapping to class are frozen: a test lists every variant with its expected
    code and its mapping to class (one class for a fixed variant; for `proof_not_verified` the class of
    each `KaniRunOutcome`, `KaniInconclusiveReason` and `KaniFailureCause` value; for `kani_execution`
    the class of each nested refusal), fails if a variant, outcome, reason, cause or nested refusal exists
    that the list lacks or if a code value or a mapped class differs, and fails if two variants share
    a code. Adding a variant, code, class or mapping entry is a spec change to the "Scalar native error
    dis'
- id: FR-017-AC-45
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The receipt exposes its proof-content identity as a `DigestRecord`, the `qsl_replay::DigestRecord`
    re-exported at the crate root (no CG-defined twin type; a compile test names it by the CG path and
    by the `qsl_replay` path as the same type), with domain `sha256-jcs` and 32 bytes equal to `ScalarProofContentIdentity::as_bytes()`.
    An independent computation in the test, SHA-256 of the RFC 8785 encoding of `{"schema": "agent-ix/cg-kani-scalar-proof-content/v3",
    "artifacts": [...]}` over the entries of AC-46, gives the same 32 bytes. The domain is not `GeneratedArtifactBytesV1`.'
- id: FR-017-AC-46
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'For a genuine routed `x + 1` package the v3 preimage has exactly ten artifacts, in UTF-8 byte
    order: `Cargo.lock`, `Cargo.toml`, `claim-map.json`, the only admitted record `kani-obligations/<module>.json`,
    `observer/Cargo.lock`, `observer/Cargo.toml`, `source-maps/lib.json`, `src/bin/ir791_native.rs`, `src/lib.rs`
    (harness), and `src/oracle-artifacts-lib.rs` (emitted library). All four original emitted artifacts,
    harness and record compare byte-identically under the rename map. Removing, renaming, reordering,
    duplicating or changing one byte of any entry changes the digest. Non-UTF-8 runner-re'
- id: FR-017-AC-47
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Two receipts for the same package and harness with different operands, different `ScalarLimits`
    and different native outcomes have equal `DigestRecord`s; operand order, limits, ranges, outcome and
    Kani evidence appear nowhere in the preimage. This holds for the swapped-operand runs of FR-017-AC-31.
- id: FR-017-AC-48
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The receipt exposes read-only the ordered staged `(path, bytes)` entries the digest was built
    from, including both manifests, both independently generated locks, the source map and the canonical
    `src/bin/ir791_native.rs`. All ten entries remain measured, sealed and compared across both execution
    boundaries. Recomputing the digest from only those exposed entries equals the receipt's `DigestRecord`.
    A recomputation that substitutes one byte in the package, harness or harness record (which a driver
    holding different bytes would do) differs from the receipt's digest. The exposed entries are exactl
- id: FR-017-AC-49
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'A change to the preimage schema (artifact set, path names, ordering, encoding, canonicalization
    or hash) is made only by changing the schema string, with the golden digest of AC-46 updated in the
    same change; the golden is computed over fixed literal entries, so dependency bumps do not move it,
    and is asserted together with the schema string in one assertion; a golden change without a schema
    bump is rejected in review (a test cannot enforce it). A change to either manifest, either lock or
    adapter bytes changes the digest with the schema string unchanged. The domain remains `sha256-jcs`
    across '
- id: FR-017-AC-50
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The `NativeMeterObservation` of a receipt exposes read-only the `ScalarLimits` the native
    meter was constructed with. They are read back from the native invocation''s own output, not copied
    from the request: a test seam that makes the adapter report a well-formed but different limit value
    makes the helper''s structured result unsupported in FR-017-AC-43''s sense and yields `NativeResult`
    / `native_result` / `EnvironmentFault` and no receipt. For an accepted request the request limits,
    the meter-observed limits and `scalar_limits()` are equal, and the meter-observed limits are unchanged
    after the '
- id: FR-017-AC-51
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A consumer compile test names and constructs `quire_contract_codegen::ScalarLimits` using only
    codegen crate paths, a typed accessor to `KaniScalarNativeRequest::scalar_limits` compiles with that
    same return type, and assignment to the owning `quire_exact::ScalarLimits` compiles without conversion,
    wrapper or compatibility shim. The CG public name and field shape remain unchanged. Planned with IR-17
    kernel adoption.
- id: FR-017-AC-52
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Production emitted bytes and feature-free exact dependency remain unchanged. The isolated private
    observer uses resolver 2 and its sole example compiles the single canonical adapter in place through
    `../src/bin/ir791_native.rs`, with runtime `exact` and kernel `test-support` exposed only by dev-dependencies;
    no wrapper, copied source, symlink, normal/build logging edge or CLI feature override is allowed.
    Native invocation explicitly names `observer/Cargo.toml` and the example; the real production proof
    retains its harness, ordered options, claims, covers, bounds and backend gates. A control re
- id: FR-017-AC-53
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Both package locks are independently freshly resolved with the existing bounded offline generation/update
    workflow, without copying a parent/other lock or substituting a cached resolution. Each resolves both
    owning runtime and exact identities to the build's authoritative lock identities; the independent
    observer regression asserts both literal package names and repository identities independently of
    the renderer's list. Both manifests, locks and all other AC-46 entries are sealed and compared as
    a complete path/byte set before proof, after proof and after native execution; mutation/replacemen
- id: FR-017-AC-54
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'A successful claimed-init confirmation poll with no `IN` yields public `MemoryObservationFailed`
    with the existing outer and nested prefixes and labelled observations of that attempt: returned poll
    count, raw revents bits, monotonic poll elapsed duration with its unit, captured init PID/start ticks,
    captured wrapper PID or explicit absence, init-handle presence, gate presence and cleaned state. Test
    controls through the existing `PollOperation` seam delegate first to real poll on the real owned pidfd,
    record its positive count and `IN`, then clear revents and return either zero or that genuine'
- id: FR-017-AC-55
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Through the real public execution entry and real owned namespace, an unchanged real-poll control
    retains its original public result and confirmed cleanup; each AC-54 omission refuses even when cancellation
    is submitted before or after the cleanup barrier. The confirmation seam sees the same claimed pidfd
    and exactly one poll with timeout `(5, 0)` per cleanup invocation. Failure retains gate/cleaned state
    and the existing `Drop` retry, and actual backend processes/namespace are gone after return. The existing
    real-poll-delegating errno control retains its public refusal rather than cancellation
- id: FR-017-AC-56
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Inspection of the cleanup change shows that init-pidfd KILL still precedes wrapper-group KILL,
    the single confirmation poll retains its `(5, 0)` timeout, signal-error/poll-error/missing-readiness
    precedence is unchanged, syscall errors retain their original kind and errno, and only successful
    cleanup closes the gate and marks cleaned. Existing `Drop` behavior is unchanged. Diagnostic observations
    come only from that poll and existing owned fields; no added wait, retry, skip, PID reopen, procfs/cgroup
    rescan or invented host-state field is introduced.
- id: FR-017-CON-1
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The generator SHALL NOT convert a non-verified outcome into a proof claim. | Integrity
- id: FR-017-CON-2
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The generator SHALL NOT report a generation-time classification as an execution outcome. |
    Integrity
- id: FR-018-AC-1
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Abort the whole request on the first refusal, or emit a stub function for a refused item.
- id: FR-018-AC-2
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Emit `EqualityOperator::Equal` where the descriptor says `NotEqual`, or apply the right operand's
    conversion before the left's.
- id: FR-018-AC-3
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Record the descriptor by reading it back out of the generated source, so a mutated emitter
    and a mutated claim map agree with each other and AC-2's native leg follows the mutation.
- id: FR-018-AC-4
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Record a fixed `EqualitySchedule::Plan` on every entry, so a top-level text, enum or quantity
    item reports a schedule the runtime never selected.
- id: FR-018-AC-5
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Generate the item and let the runtime refuse at evaluation time instead of refusing at generation
    time.
- id: FR-018-AC-6
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Check `contains_ieee` only at the top level, so a nested `float64` field generates.
- id: FR-018-AC-7
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Relabel an IR `Unsupported` root as a CG blocker, substitute a sibling's source occurrence,
    allow a strict-reader-invalid reference composite into CG, or generate an oracle for a `call` expression.
findings:
- fnd: FND-109
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-019-AC-38
  related_ids: []
  path: spec/routed/functional/FR-019-capability-settlement.md
  lines: '562'
  excerpt: PLANNED (IR-516). An advertised SG-1, SG-2 or SG-3 item reaches EN-1 with its recorded form;
    only advertised SG-1 reaches an SMT arm. SG-2 or SG-3 against an SMT-only candidate settles unsupported
    and reaches no SMT engine.
  finding: 'Inherited FR-019-AC-38 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-110
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-019-AC-39
  related_ids: []
  path: spec/routed/functional/FR-019-capability-settlement.md
  lines: '563'
  excerpt: PLANNED (IR-516). A selected reduction reaches only a candidate advertising that exact reduction,
    with the selection preserved in the routed request. Symmetry declarations and POR enter the obligation
    identity, so their reduced and unreduced requests for one claim have distinct obligation identities;
    a state constraint changes request identity but leaves obligation identity unchanged. An unadvertised
    selection or POR on a subject with no footprints settles `unsupported` without an artifact, and no
    arm drops or adds a reduction.
  finding: 'Inherited FR-019-AC-39 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-111
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-019-AC-40
  related_ids: []
  path: spec/routed/functional/FR-019-capability-settlement.md
  lines: '564'
  excerpt: PLANNED (IR-516). For an item over a weak `parallel`, a candidate advertising the subject's
    resolved `tso` or `ra` model receives that exact model; a candidate lacking it settles `unsupported`
    with no artifact, and no arm weakens the resolved model. An `sc` subject is a separate strong-model
    control and does not take the weak-memory branch.
  finding: 'Inherited FR-019-AC-40 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-112
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-019-AC-41
  related_ids: []
  path: spec/routed/functional/FR-019-capability-settlement.md
  lines: '565'
  excerpt: PLANNED (IR-516). With otherwise equal form and kind advertisement, model and protocol subject
    classes are read separately; an arm that discharges only one class settles the other `unsupported`,
    while a supported protocol item retains its protocol transition-system subject identity through routing.
  finding: 'Inherited FR-019-AC-41 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-113
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-021-AC-30
  related_ids: []
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  lines: '319'
  excerpt: Accept `u64::MAX` and increment a `u64` at the first denied charge; the refusal wraps or panics.
  finding: 'Inherited FR-021-AC-30 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-114
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-022-AC-26
  related_ids: []
  path: spec/routed/functional/FR-022-routed-generation.md
  lines: '361'
  excerpt: Given the checked QSL RR-16 package with two `x + 1` occurrences of one application node, directly
    narrowed to `Int[0, 10]` and `Int[0, 20]`, the driver supplies two distinct request indexes and occurrence
    keys. CG matches each to its own requirement record and selected narrow; both supported scalar outputs
    use the shared application operation and their respective result bounds, and their obligation and
    harness identities differ by application occurrence. Reversing routed-item order leaves these pairings
    unchanged.
  finding: 'Inherited FR-022-AC-26 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-115
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-022-AC-27
  related_ids: []
  path: spec/routed/functional/FR-022-routed-generation.md
  lines: '362'
  excerpt: 'For `if p then x + 1 else x + 1`, each occurrence''s generated scalar claim is conditional
    on its own QSL path guard: the same guard occurrence with `holds=true` for the then item and `holds=false`
    for the else item. CG preserves the full outermost-first guard sequence and uses it when forming the
    proof obligation; a result under `p=false` cannot prove the then claim and one under `p=true` cannot
    prove the else claim. An empty path condition remains unguarded.'
  finding: 'Inherited FR-022-AC-27 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-116
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-022-AC-28
  related_ids: []
  path: spec/routed/functional/FR-022-routed-generation.md
  lines: '363'
  excerpt: If a routed scalar item's occurrence has no entry in the supplied requirement records, its
    key's node disagrees with the authenticated application selection, or the keyed record disagrees with
    the item's result bound, direct conversion occurrence or ordered path condition, CG returns a typed
    refusal identifying the request index and occurrence before emitting an oracle, harness or obligation
    identity for that request; it does not use another record with the same node or reconstruct missing
    values from the IR graph or source text.
  finding: 'Inherited FR-022-AC-28 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-117
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-024-AC-1
  related_ids: []
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  lines: '347'
  excerpt: 'The envelope''s obligation identity is QSL''s `ObligationIdentity` built as the ADR-013 O-09
    digest by the one scheme of FR-024-AC-20: the subject node id (function or state frame; a clause or
    application subject is not yet built), its occurrence key, the obligation kind and the arguments,
    each a parameter node id with its declared domain (empty for a frame, which has no parameters), and
    no `source_span`. Changing `source_span` leaves it unchanged, and changing the obligation kind or
    any argument binding changes it. The scheme''s spelling is CG''s own and carries no digest label (AD-003
    E-1, open '
  finding: 'Inherited FR-024-AC-1 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-118
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-024-AC-7
  related_ids: []
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  lines: '352'
  excerpt: A reduced `Witness`-arm candidate is retained only when a backend re-run and native replay
    both preserve domain validity and the failure; a candidate that changes domain validity or the verdict
    is discarded.
  finding: 'Inherited FR-024-AC-7 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-119
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-024-AC-8
  related_ids: []
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  lines: '353'
  excerpt: A retained reduced `Witness`-arm envelope carries its own backend re-run's transcript, never
    the original counterexample's.
  finding: 'Inherited FR-024-AC-8 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-120
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-024-AC-18
  related_ids: []
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  lines: '362'
  excerpt: With the installed backend, the falsified operation-contract harness of a postcondition state
    clause over a subject mutated to debit is replayed from its real Kani playback through `replay_state_clause`
    and settles `reproduced-with-evaluated-witness`, `violation`. The test is `tc_035_real_kani_state_clause_counterexample_replays_through_qsl`
    in the module `kani_obligations_state_clause_replay`, so the `kani_obligations` filter of `make kani`
    selects it.
  finding: 'Inherited FR-024-AC-18 is tagged-by-ignored-test at the frozen candidate: every bound test
    is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or
    qualify that obligation.'
- fnd: FND-121
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-024-AC-34
  related_ids: []
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  lines: '393'
  excerpt: Run the real-Kani cases over a hand-built fixture harness.
  finding: 'Inherited FR-024-AC-34 is tagged-by-ignored-test at the frozen candidate: every bound test
    is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or
    qualify that obligation.'
- fnd: FND-122
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-024-AC-35
  related_ids: []
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  lines: '394'
  excerpt: Settle refusal as a violation or `Verified`, omit `TypeNotRange`, or call an unread bounded
    field unranged.
  finding: 'Inherited FR-024-AC-35 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-123
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-024-AC-36
  related_ids: []
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  lines: '380'
  excerpt: 'Two distinct test backend adapters each produce a typed outcome: success carries equivalent
    assignments, each adapter''s backend identity, the same opaque obligation identity, declared domains,
    source/provenance and replay packet members; decode refusal carries only backend identity, obligation
    identity and typed cause. Native text is observed only within its own adapter; the common intake neither
    parses native text nor remints the obligation identity, and QSL receives each successful adapter''s
    original backend and source identity in a complete packet.'
  finding: 'Inherited FR-024-AC-36 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-124
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-024-AC-37
  related_ids: []
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  lines: '381'
  excerpt: The common intake maps an adapter's decode-refusal arm to a typed decode evidence failure retaining
    its cause, with no envelope or replay. Before native replay, an out-of-domain assignment yields a
    distinct typed domain evidence failure; an incomplete or QSL-refused witness/packet yields a distinct
    typed QSL admission evidence failure retaining its cause. A replay verdict disagreement remains a
    distinct typed evidence failure. None settles as contract success or contract failure, and none of
    the pre-replay failures invokes replay. Kani's exact decode and refusal partition remain governed
    by FR
  finding: 'Inherited FR-024-AC-37 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-125
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-024-AC-38
  related_ids: []
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  lines: '382'
  excerpt: 'Starting from one admitted original failure, one valid reduced candidate that preserves the
    failure becomes a new CG lineage revision referring to its exact admitted QSL envelope, its exact
    parent revision and the unchanged opaque obligation identity. After retention, the original QSL envelope
    compares equal to its pre-reduction clone by `WitnessEnvelope<P>: Eq`, and the original CG record''s
    revision identity, parent, obligation and typed evidence fields read back equal to their captured
    values. No lineage member is added to QSL''s envelope or packet.'
  finding: 'Inherited FR-024-AC-38 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-126
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-024-AC-39
  related_ids: []
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  lines: '383'
  excerpt: From a retained failure revision, an out-of-domain reduction and a domain-valid reduction that
    changes the native verdict each create no revision. After each rejection, every prior QSL envelope
    compares equal to its pre-proposal clone and every prior CG record's revision identity, parent, obligation
    and typed evidence fields equal their captured values; a later valid reduction names the actual retained
    parent, with no skipped or substituted parent.
  finding: 'Inherited FR-024-AC-39 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
bindings: []
review_head: 3d303ea6988496c4e1102faa58a56c1f260193c3
review_verdict: 'FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence
  gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls
  now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.'
part: 7
parts: 18
scope_kind: full-repository criterion presence; no blanket semantic acceptance
```

+++