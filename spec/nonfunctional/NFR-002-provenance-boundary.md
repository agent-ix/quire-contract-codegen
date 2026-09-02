---
id: NFR-002
title: "Provenance, licensing, and qualification boundary"
type: NFR
quality_attribute: compliance
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: constrains
---
# NFR-002: Provenance, licensing, and qualification boundary

## Statement

- Every emitted artifact shall identify its tool, input, schema, backend, configuration, and output,
  and shall be bound to the digest of its own bytes when its attestation is sealed.
- Generated Rust shall carry `MIT OR Apache-2.0` SPDX identity.
- No automated output shall claim project-specific validation, accreditation, certification, or human release approval.

## Scope

Generated source, proof attestations, diagnostics, source maps, proof graphs, and reports.

## Rationale

Opaque derivation or licensing prevents independent audit, while automated qualification claims
would exceed the tool's authority and hide consuming-project responsibilities.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Missing mandatory identity fields | 0 | 0 | contract-testing |
| Generated files missing dual-license SPDX header | 0 | 0 | inspection |
| Silent unsupported/inconclusive states | 0 | 0 | negative-abuse-testing |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-002-AC-1 | Every emitted artifact records the required tool, input, schema, backend, configuration, and output identity, and the digest identity of its own bytes is established when its attestation is sealed. | Test (TC-001) |
| NFR-002-AC-2 | Every generated Rust file carries the `MIT OR Apache-2.0` SPDX identity. | Test (TC-001) |
| NFR-002-AC-3 | Unsupported and inconclusive states remain explicit rather than silently succeeding. | Test (TC-003) |
| NFR-002-AC-4 | Generated output makes no project-specific validation, accreditation, certification, or release-approval claim. | Inspection |
| NFR-002-AC-5 | A green local gate run is claimed only for the tree as committed, and the limits of that claim are recorded rather than closed. | Inspection |

## Qualification Integrity

A green `make ci` is a statement about the tree as committed. It is not a statement about a tree whose
Makefile has been edited, and this requirement does not pretend otherwise.

Make can be told to ignore failure, and one line does it: `.IGNORE:` at the top of the file, a `-`
prefix on a recipe line, or an assignment to `SHELL`. Measured on this repository with three injected
defects — a rustfmt violation, a failing test assertion, and an upstream constant the crate does not
declare — the control tree exits **2** at `fmt-check`, the first of eleven `ci` prerequisites, having
run only that one. With `.IGNORE:` prepended the same tree exits **0**: all eleven prerequisites run,
**seven** of them fail — `fmt-check`, `spec`, `lint`, `msrv`, `upstream-identity`, `test` and
`assurance-chain` — every one prints its diagnostic, and none of them fails the build.

`assurance-chain` failing in that list is the part worth reading twice. The chain did notice: the
upstream producer's rows became `not-computed`, the attestation stated that because the producer's
bytes stated it, and the scenario that requires every producer to have reported success went red for
exit 1. What did not notice was Make.

So the boundary is drawn where it actually falls. Quoin binds retained inputs by digest and every
attested result is derived from a producer's own bytes, so a Makefile that lies about running a
producer yields an absent or unreadable input and the chain errors rather than passing. The gates that
feed nothing into the chain — `fmt-check`, `lint`, `deny`, `audit-unsafe`, `rustdoc` — are simply
neutered, and the structural replacement does not reach them. `tests/shared_assurance.rs` asserts the
committed Makefile declares no such directive, which protects a reviewer reading a diff and does not
make this file's exit code trustworthy on a tree where it has been edited: under `.IGNORE:` that test
also runs, also fails, and is also swallowed.

This is recorded, not closed. No policer target was re-added, because a Makefile that attests to its
own execution controls is the arrangement this migration removed. Tracked as
agent-ix/quire-contract-codegen#14.

## Verification

TC-001 and TC-003 validate identities, licensing, and explicit diagnostic states; TC-001 is where
NFR-002-AC-1 is measured, against the proof attestations this crate emits. The
seventh identity in the earlier statement of this requirement — the digest of the
artifact's own bytes — is deliberately no longer the generator's. The deprecated
envelope carried the generator's own SHA-256 of its own output, a self-check that
could only ever agree with itself. The shared shape puts that binding in
`retained_output.digest`, which `quoin change-assurance seal-attestation` computes
from the bytes on disk, so the statement and the criterion above say "when its
attestation is sealed" rather than continuing to claim the generator records it.
What is lost by that move is stated rather than glossed: a reader holding only an
emitted attestation and its artifact can no longer check the pairing with
`sha256sum` alone. TC-001 measures the replacement, including a control that one
appended byte moves the sealed digest and one that sealing is deterministic. TC-011 previously measured that
retained bytes were read without being written and that Git agreed they were the committed bytes;
that test is removed with the records it measured, and no weaker substitute is put in its place.
NFR-002-AC-5 is verified by the measurement recorded above and by
`tests/shared_assurance.rs`, which asserts the committed Makefile declares no failure-suppressing
directive. The Measurement Plan retains the exact draft or released upstream revisions used by each
candidate.

## Dependencies

- **Upstream**: PGM-01 and [FR-001](../functional/FR-001-deterministic-oracles.md).
