---
id: SR-1143
title: "integrity review of quire-contract-codegen PR 248 spec changes (IR-553 function-path obligation identity)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@360d58dae87a4486b4bcad945b8b1f4461e6e5f6; spec/replay/functional/FR-016-witness-native-replay.md, spec/replay/matrix/TC-026-witness-native-replay.md, spec/replay/matrix/tests.md, spec/tests.md, spec/assurance/AD-002-cg-qsl-replay-seam.md, spec/assurance/AD-003-evidence-chain.md"
review_set: subset
---

## Summary

Ticket: IR-553. Integrity analysis of the PR's spec diff (status flips for FR-016-AC-21 to
AC-23 and AD current-state text), checked against merged quire-spec-language main c83be71
(ADR-013 O-07, O-09, section 2, QC-4, QC-14, TK-05, TK-07; FR-121-AC-15) and
quire-specification main AD-016 arrow 5 and its seed vector.

What is pinned upstream: the members (function node id, `declaration` occurrence key as
node/role/ordinal, obligation kind, arguments as parameter node id plus declared domain),
source span excluded, one RFC 8785 encoder (`quire-canonical`), arguments ascending by
identifier (O-09, AD-016 arrow 5), and the harness bound as the per-argument domain (AD-016
arrow 5 "Harness domain", ADR-014 B-4). What is not pinned anywhere: JSON member names, the
domain encoding, the node-id text form, and the digest domain (O-09: not in FR-201; QC-4 and
TK-07 open to add one). The AD-016 seed vector `obligationIdentitySha256` is `OPEN — decided
in QSL ADR-013 TK-05`, which is CG's conformance work, and no QSL or QSpec fixture carries an
obligation-identity vector. O-09 makes CG the obligation identity's owner and QSL only wraps
CG's digest, so CG choosing the spelling is in its lane and not incompatible with any QSL
text; the gap is that the choice is recorded only in Rust serde attributes.

Clean: matrix rows, `spec/tests.md`, FR-016 statement and AC rows, TC-026 status and AD-003
E-1 agree that AC-21 to AC-23 are implemented for the function path only, and the V1, scalar
and frame paths still compute none.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | ADR-013 section 2 defines normalized identity as RFC 8785 of a closed preimage schema with golden vectors, and O-09 makes CG the owner, yet the PR's closed schema (member names, `declaration` as `{node,role,ordinal}`, domain as `{type:boolean}` / `{type:integerRange,minimum,maximum}` with decimal-string bounds, lowercase-hex node ids, no digest label) is stated in no CG spec artifact: FR-016-AC-21 and AD-003 E-1 name only the members, and the code comment says "the member names below are CG's". A downstream verifier (or the TK-05 seed-vector regeneration) cannot recompute the identity from the spec | spec/assurance/AD-003-evidence-chain.md:264-272 |
| FND-002 | low | AD-002 Current state now reads "Until QSL's queued code PR adds those members, the code cannot land; FR-016-AC-21 to AC-23 are implemented." The first clause is stale and contradicts the second | spec/assurance/AD-002-cg-qsl-replay-seam.md:148-149 |

## Verdict

Status changes and trace counts are consistent. FND-001 should be closed in this PR by
stating the closed preimage schema (and its interim no-label status pending QSpec TK-07) in
AD-003 E-1, citing the AC-21 golden text as the vector. FND-002 is a one-line wording fix.

## New findings (disposition pass 1)

Reviewed at 5aee2cf161b3b1068c3d70d4de895de0cba8a377. `quire validate`: existing warnings only.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | interface-001 is not updated for the PR's public API: `ReplayPackage::request` now takes the harness `KaniObligationIdentity` and returns `Result<_, ObligationIdentityError>`, `replay_counterexample` gains `SpineReplayError::Identity`, and `replay_counterexample_through`, `ObligationIdentityError` and `DigestError` are new exports; the interface's `ReplayPackage::new` / `replay_counterexample` entries still describe the old surface and list no identity refusal | spec/core/functional/interface-001-codegen-api.md:153-164 |
| FND-004 | medium | The interim, CG-owned status of the preimage spelling is stated only in AD-003 E-1. AD-002 (the seam AD) and the PR body say nothing about it, and AD-003 omits that if QSL later recomputes or compares the identity, QSL pins the spelling and CG follows in a follow-up | spec/assurance/AD-002-cg-qsl-replay-seam.md:140-148 |

## Dispositions

Round 1, reviewed at 5aee2cf161b3b1068c3d70d4de895de0cba8a377.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 5aee2cf |
| FND-002 | fixed | 5aee2cf |

Round 2, reviewed at a485e43b7622df28214062afbbad450ce40c5588. `quire validate`: existing warnings only; `quire coverage --strict`: 66 unbacked, 0 contradicted.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-003 | fixed | a485e43 |
| FND-004 | fixed | a485e43 |
