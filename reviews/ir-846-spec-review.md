---
id: "SR-12135"
title: "IR-846 spec review: scalar proof-content identity as the FR-357 observation digest"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@fa8c661de64f5023e51c23e29bdf8c1da226ae8c; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/tests.md, spec/replay/functional/FR-032-routed-scalar-replay-binding.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-027
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
---

# SR-12135: IR-846 spec review

## Summary

Ticket: IR-846. PR: agent-ix/quire-contract-codegen#419, frozen head fa8c661, stacked on #418
(peter/ir-847-native-error-discriminant-spec); only `git diff origin/peter/ir-847-native-error-discriminant-spec...HEAD`
was reviewed. Method: spec-review base checklist, with failure-domain, integrity and consumer-fit
checks folded in (no separate sub-analysis artifacts).

## Method

I read the new FR-017 section "Scalar proof-content identity as the FR-357 observation digest",
FR-017-AC-45..49, the TC-027 step paragraph, the two tests.md rows and the FR-032 pointer. I
checked the preimage against the code at this head: `src/kani/run/scalar.rs` (`CanonicalArtifact`,
`ScalarProofPreimage`, `stage_package`, `measure`, `canonical_identity`, the request validation at
:275-281 and :378-408) and `src/core/canonical.rs` (`content_digest_with_limits`), plus
quire-canonical 5dc4e12 (`sha256`, `Limits`). Ground truth was QSL main 812939fb5 (FR-357, AC-14,
qsl-foundation digest.rs `Sha256Jcs` and `is_raw_byte_addressed`, qsl-replay composite.rs
`content_identity: DigestRecord`). Consumer fit was checked against quire-driver FR-009 (b3d7112).

The preimage matches the code exactly: schema `agent-ix/cg-kani-scalar-proof-content/v2`; fields
`schema`, `artifacts`, `path`, `contents`; the emitted `src/lib.rs` renamed to
`src/oracle-artifacts-lib.rs` and the harness at `src/lib.rs`; the record at
`kani-obligations/<module>.json`; the runner-read `Cargo.lock` and `src/bin/ir791_native.rs`; a
`String::cmp` sort, which is UTF-8 byte order; the reserved-path and duplicate refusals; plain
SHA-256 of the RFC 8785 bytes with no domain label; a 256 MiB ceiling on canonical output bytes.
At this head the encode failure maps to `NativeResult`; #418's FR-017-AC-43 makes it
`IdentityEncode`, which the spec correctly cites. The AC-46 order (Cargo.lock, Cargo.toml,
claim-map.json, kani-obligations/..., src/bin/ir791_native.rs, src/lib.rs,
src/oracle-artifacts-lib.rs) is correct in byte order. `sha256-jcs` over a structural preimage
agrees with QSL's `is_raw_byte_addressed`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-017-AC-49 asks for a test that fails when the golden digest changes but the schema string does not. No test in the same tree can enforce that: one edit updates the golden and leaves the schema. The golden's input is also left open. If it is a real staged run, the real `Cargo.lock` is in the digest, so every dependency bump changes the golden with no schema change. AC-49's own Cargo.lock clause says that is legitimate, and it collides with the golden rule. | spec/kani/functional/FR-017-kani-execution-evidence.md:774, spec/kani/functional/FR-017-kani-execution-evidence.md:771, spec/kani/matrix/TC-027-kani-execution-evidence.md:446-448 |
| FND-002 | medium | The non-UTF-8 refusal names no variant, code or class. The code returns `InvalidRequest("staged artifact is not UTF-8")`, which #418's AC-42 classes as `CompletionRefusal`. But request artifacts hold `String` contents, so only the runner-read `Cargo.lock` or adapter can be non-UTF-8. That is an environment or snapshot fault, not a caller refusal. The TC step names no seam that injects it. | spec/kani/functional/FR-017-kani-execution-evidence.md:682, spec/kani/functional/FR-017-kani-execution-evidence.md:771 |
| FND-003 | medium | Consumer misfit. quire-driver FR-009 (statement and AC-7) says the driver "verif[ies] QSL's observation digest against that outcome". Under this section the digest excludes the outcome and is checked by recomputing it over the receipt's staged entries. No driver contract in the spec matches the authentication this section assigns to the IR-773 driver, and no follow-up ticket is named. | spec/kani/functional/FR-017-kani-execution-evidence.md:693-709, spec/kani/functional/FR-017-kani-execution-evidence.md:773 |
| FND-004 | low | The Owners bullet says "A consumer shall not mint this value from receipt fields other than the exposed identity". The Retention bullet and AC-48 have the driver recompute it from the exposed entries, which are receipt fields. Read literally, the two rules contradict. | spec/kani/functional/FR-017-kani-execution-evidence.md:695-696 |
| FND-005 | low | AC-46 hedges the record position ("assuming the `kani-obligations/<module>.json` path ...; any other record path sorts where its bytes put it"). The runner admits only `kani-obligations/{module_symbol}.json` (scalar.rs:275-281), so the hedge describes an unreachable case and blurs an otherwise exact order assertion. | spec/kani/functional/FR-017-kani-execution-evidence.md:771 |
| FND-006 | low | The driver's comparison ("requires the package, harness and harness-record entries to equal the bytes of the artifact it runs") does not restate the rename map. Under that map the emitted `src/lib.rs` is entry `src/oracle-artifacts-lib.rs` and entry `src/lib.rs` is the harness. A path-by-path comparison of the emitted package would falsely mismatch. | spec/kani/functional/FR-017-kani-execution-evidence.md:704-706 |

## Verdict

Request changes for FND-001 through FND-003, then approve. The QSL-ruled contract is correct as
written: `sha256-jcs` domain, artifact-only preimage, CG mints, the driver authenticates, QSL
carries. The preimage agrees byte for byte with the code at this head, and AC-45, AC-47 and AC-48
can be tested as written. AC-48's retention of the staged `(path, bytes)` list, including
`Cargo.lock` and the adapter, is sound and is enough for the driver to recompute the digest. The
section also states honestly that for those two entries the driver relies on CG's attestation.

Suggested fixes:

- FND-001: compute the golden with the measurement function over fixed literal entries,
  including a literal `Cargo.lock` and adapter, not over a real staged run. Pin the
  `(schema string, golden)` pair in one assertion. Restate "a test fails if the golden changes
  while the schema is unchanged" as a review rule: a change to the golden without a schema bump
  is rejected in review.
- FND-002: name the variant, for example `IdentityEncode` / `identity_encode` /
  `EnvironmentFault`, and name the test seam that writes a non-UTF-8 `Cargo.lock`.
- FND-003: name the driver-side follow-up, or add a sentence that quire-driver FR-009 AC-7's
  "against that outcome" must be re-specified as recomputation over the exposed entries.

I considered the base checklist's hash/digest antipattern rule and did not apply it. This digest
is the cross-repo identity that QSL FR-357 requires, not a tracking hash.

## New findings (disposition pass 1)

Reviewed at 238ca5304ed1a81717b8b02be279aab2161657a7 (rebased onto #418 75a0bb8), diff `origin/peter/ir-847-native-error-discriminant-spec...HEAD`.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | The driver's accepted condition "The receipt shall bind the exposed entries to the immutable snapshot CG actually executed, never to a later filesystem read" is a new `shall` with no acceptance criterion. AC-48 checks only that recomputing over the exposed entries equals the digest, and a later re-read of an unchanged snapshot would also pass that. So no test can catch the entries being sourced from a post-run read. One more clause in AC-48 would make it testable, for example: the exposed entries are the bytes measured for the identity, and no entry accessor reads the filesystem (by source scan, or by mutating a snapshot file after the receipt is built). | spec/kani/functional/FR-017-kani-execution-evidence.md:723-724, spec/kani/functional/FR-017-kani-execution-evidence.md:806 |

## New findings (disposition pass 2)

Reviewed at d8c7be6d3850bba322b3117987ca3774098d12c2 (rebased onto #418 a569513), diff `origin/peter/ir-847-native-error-discriminant-spec...HEAD`. FND-001..006 text survives the rebase unchanged; the only other spec changes are #418's own AC-41/42/44 and TC-027 class-mapping edits.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | low | TC-027 step (4a) says "after the receipt is built, overwrite each snapshot file with different bytes and then delete the snapshot directory". But `PrivateSnapshot` is a local of the runner that unseals and removes the directory on `Drop` (scalar.rs:216, :935) before the caller receives the receipt. At the public API there is nothing left to overwrite: writing to those paths would create new files and pass vacuously. Either name an in-crate seam that keeps the snapshot alive past receipt construction, or restate the step as: assert the snapshot path no longer exists, then require the exposed entries to still read and recompute (which already proves no filesystem read). Also, (4a) is placed before (4). | spec/kani/matrix/TC-027-kani-execution-evidence.md:461, src/kani/run/scalar.rs:935 |

## New findings (disposition pass 4)

Reviewed at 987dd6f54897b1b6f78b063d7d8473d68b51c6d9 (rebased onto #418 24a4caf). The round-4 changes are planner-ruled gap fixes: the snapshot-removal diagnostic in AC-48, the `qsl_replay::DigestRecord` root re-export, and FR-017-AC-50 for IR-857. FND-001..008 still hold.

The limits are observable. quire-contract-runtime 3cc88c4, the revision CG's lock pins, has a public `Meter::limits() -> &ScalarLimits` (src/exact/accounting.rs:514). The adapter prints only `consumed` and `admitted_charges` today (scalar.rs:594-595), so emitting the limits is code-phase work. AC-50 fits IR-857: it puts the meter-used limits on the observation, guarantees they equal the request limits and `scalar_limits()`, binds them like the other fields, and refuses a mismatch with no receipt. The re-export is sound: `qsl-replay` is CG's one QSL dependency, and `src/replay/composite_builder.rs` already uses `qsl_replay::DigestRecord`.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-009 | medium | AC-50 makes a reported meter limit that differs from the request `NativeResult` / `EnvironmentFault`. #418's frozen text limits that variant: AC-43 says "`NativeResult` is produced only for the helper's malformed, missing or unsupported structured result", and the Behavior line and table row say the same ("a decode fault"). A well-formed line carrying a different limit is none of those three. As written, AC-43 and AC-50 contradict each other, and an AC-43 exclusivity test would flag the new cause. The class itself is consistent with AC-42. Fix: have AC-50 (and the Meter-used limits bullet) say that a reported limit differing from the request is an unsupported structured result in the sense of AC-43, or widen the AC-43 sentence. | spec/kani/functional/FR-017-kani-execution-evidence.md:822, spec/kani/functional/FR-017-kani-execution-evidence.md:752, spec/kani/functional/FR-017-kani-execution-evidence.md:815, spec/kani/functional/FR-017-kani-execution-evidence.md:650 |
| FND-010 | medium | The removal-failure clause in AC-48 ("recorded as a diagnostic") names no sink. TC-027 step (8) requires "a recorded diagnostic" and AC-48 requires that the failure change "neither the receipt nor the verdict". If the diagnostic lives in the receipt, the receipt changes; if it lives elsewhere (a log, a returned side value), nothing says where, so (8) has nothing to assert. The code removes the snapshot in `PrivateSnapshot::drop` (scalar.rs:935), which cannot report anything, so removal also has to become explicit before return. The same AC still says flatly that "The private snapshot is removed before the receipt is returned", which is no longer always true. Fix: name the sink (for example a read-only `cleanup_diagnostic()` on the receipt that is excluded from the digest and from receipt equality, or a named log event), and make the removal sentence conditional on success. | spec/kani/functional/FR-017-kani-execution-evidence.md:820, spec/kani/matrix/TC-027-kani-execution-evidence.md:465, src/kani/run/scalar.rs:935 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 238ca53: the golden is computed over fixed literal entries (a literal `Cargo.lock` and adapter included), never a real staged run, and is pinned with the schema string in one assertion. "A golden change without a schema bump is rejected in review (a test cannot enforce it)" (FR-017-AC-49; TC-027 step 5). |
| FND-002 | fixed | 238ca53: a non-UTF-8 runner-read `Cargo.lock` or adapter is refused as `Snapshot` / `snapshot_io` / `EnvironmentFault` (not `InvalidRequest`) with no receipt, and TC-027 names the seam that writes non-UTF-8 bytes. This fits #418's frozen row ("Host I/O failure creating, writing or measuring the private snapshot"): the check is part of measuring, and std's own `read_to_string` reports invalid UTF-8 as `io::ErrorKind::InvalidData`. |
| FND-003 | fixed | 238ca53: a "Driver follow-up" bullet records that quire-driver FR-009 statement and AC-7 must be amended to artifact-only recomputation, owned by the quire-driver team, blocking IR-773 AC-6/7 (routed to draco). |
| FND-004 | fixed | 238ca53: "A consumer shall not mint a wire value from operands, limits or outcome; the only legitimate recomputation is the driver's, over the exposed staged entries." |
| FND-005 | fixed | 238ca53: AC-46 reads "the harness record at `kani-obligations/<module>.json` (the only record path the runner admits)". |
| FND-006 | fixed | 238ca53: the Retention bullet spells out the rename map (emitted `src/lib.rs` against entry `src/oracle-artifacts-lib.rs`, harness against entry `src/lib.rs`, record against its `kani-obligations/<module>.json` entry) and adds that a path-by-path comparison would falsely mismatch. |
| FND-007 | fixed | d8c7be6: FR-017-AC-48 "The exposed entries are exactly the bytes measured for the identity, captured at measurement time and held in the receipt; no entry accessor reads the filesystem. After the receipt is built, overwriting a snapshot file or deleting the snapshot directory leaves the exposed entries and the digest unchanged, and the digest still recomputes from them." TC-027 step (4a) tests it by mutation. |
| FND-008 | fixed | a1815aa: FR-017-AC-48 "The private snapshot is removed before the receipt is returned; after that the exposed entries still read and the digest recomputes byte-identically from them." TC-027 step (4): "After the receipt is returned, assert the snapshot path no longer exists ..., then require the exposed entries to still read and the digest to recompute byte-identically". Re-verified at a1815aa (rebased onto #418 d6b9778): FND-001..007 text unchanged. |
