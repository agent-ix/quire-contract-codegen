---
id: FR-006
title: "Adopt the shared assurance intake contract"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/NFR-002
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-006: Adopt the shared assurance intake contract

## Description

When a candidate revision is offered for review, this repository shall publish its verification
results in declared structured formats produced by its own tools, and shall obtain retention,
integrity checking, audit, attestation, and verification receipts from the released Quoin
change-assurance surface, from static facts exported by Quire, and from the Engineering Assurance
surface — without either Quire or Quoin executing a producer.

The repository shall not implement a generic runner, evidence envelope, manifest, identity framework,
retention store, audit store, anchor file, or aggregate verdict of its own.

## Inputs

- The bounded generation corpus and this crate's public generation API.
- The declared upstream IR and runtime revisions, in the crate constants, the dependency
  declarations, and the resolved lockfile.
- The released Engineering Assurance distribution and its packaged compatibility matrix.

## Outputs

- `codegen.generation-conformance/v1`, one row per corpus case, carrying the outcome, the
  Interface-001 terminal state reached, the diagnostic code produced, and the number of declared
  checks discharged against a floor.
- `codegen.upstream-identity/v1`, one row per declared upstream.
- A Quire static export and a `cargo` MSRV message stream.
- A sealed Quoin change-assurance record, one proof attestation per obligation over the exact bytes a
  producer wrote, and a verification receipt.

## Behavior

- Exactly one Make target shall run producers.
- Every consumer of a producer result shall refuse to create that result.
- Every consumer of an absent producer result shall name the target that makes it.
- The assurance driver shall read each attested result out of the producer's own bytes.
- The assurance driver shall treat an unreadable producer result as an environment error.
- The declared command of a proof obligation shall be the command the producer target runs.
- The assurance driver shall record an unobservable tool version as unobserved rather than defaulting
  it.
- No install line, requirement, lockfile, or registry configuration shall name the internal mirror or
  a component version the accepted compatibility matrix names incompatible.
- Hosted CI shall remain manual-dispatch only.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-006-AC-1 | Adopted component versions are classified by the packaged compatibility matrix rather than a local restatement, and no install line names a version that matrix calls incompatible. | Test (TC-008) |
| FR-006-AC-2 | Generation-conformance and upstream-identity results are produced by this repository's own tools in declared structured formats and transcribed by Quoin without Quoin or Quire executing a producer. | Test (TC-009) |
| FR-006-AC-3 | Static specification, obligation, and coverage facts come from a Quire export that names every requirement in this repository, and the sealed record's impact snapshot is that export's digest. | Test (TC-010) |
| FR-006-AC-5 | Pass, fail, unavailable, inconclusive, not-computed, partial, stale, suspect, vacuous, and tampered are each demonstrated by a case that produced it, `unsupported` and `malformed` are demonstrated by nothing, and every negative is paired with a positive control. | Test (TC-012) |
| FR-006-AC-6 | No repository-local generic runner, envelope builder, manifest, verifier, anchor writer, failure-propagation policer, or aggregate verdict remains, and the gates that replaced them are reachable from `ci`. | Test (TC-013) |
| FR-006-AC-7 | No executable or configuration surface in this repository names the deprecated evidence format: neither its schema version string, in either spelling, nor any of the eleven serde types that stated that shape before its schema was deleted. Markdown and the change declaration are outside the census, and the change declaration therefore does not name the format either. | Test (TC-013) |

## Dependencies

- **Upstream**: the released Engineering Assurance, Quire/quire-cli, Quoin, and ix-flow versions
  recorded in `assurance/pins.json`; [FR-001](./FR-001-deterministic-oracles.md) and
  [NFR-002](../nonfunctional/NFR-002-provenance-boundary.md).

## Notes

The shared verification vocabulary is twelve states. FR-006-AC-5 requires ten
of them. Measured on the tree before anything was deleted, per state and per
source: the assurance chain alone demonstrated ten, and the compatibility census
over the retained `evidence/` tree supplied exactly `unsupported` and
`malformed`. The repository owner released the preservation constraint for the
pre-stable phase on 2026-09-02 (`agent-ix/engineering-assurance#7`), those
records are deleted, and both claims are withdrawn with them rather than restated
over a weaker substitute.

`unsupported` is not in the assurance chain's producer outcome vocabulary and
never was: it was raised only by the compatibility mapping, against a retained
record carrying an unknown PGM-01 schema version. The generation corpus does
reach an `unsupported` *Interface-001 terminal state*, and borrowing that would
collapse two vocabularies this requirement exists to keep apart.

`malformed` is in that vocabulary, and it was still withdrawn, because being a
declared key is not the same as being distinguishable. The adapter maps
`malformed` onto the same `fail` in both of its tables, so a scenario feeding it
a stream declaring that outcome yields receipt reasons byte-identical to the
`fail` case — which is why the chain's own anti-collapse scenario could not have
included it. Such a scenario was written, measured, found to be the `fail` case
under another name, and removed. Eleven states with one painted on is worse than
ten that are real.

TC-012 asserts both states stay absent, so a later change re-acquires one
deliberately rather than drifting into it.

FR-006-AC-7 is the absence half of the migration off that format. The presence
half — that what a generated artifact carries instead is Quoin's packaged
`ProofAttestationV1`, accepted by `quoin change-assurance seal-attestation` and
validating against the schema `quoin change-assurance schema` publishes — is
NFR-002-AC-1's, measured by TC-001 against artifacts this crate emits. Splitting
them is deliberate: an absence claim over the whole tree and an identity claim
about one emitted artifact are different populations and different measurements,
and one criterion covering both would be satisfied by whichever half was easier.

The criterion states its two carve-outs rather than claiming a reach the census
does not have. Markdown is excluded because prose cannot read anything and this
repository's planning documents and reviews are the record of what was removed.
`assurance/change-assurance.json` is excluded wholesale by TC-013 because its job
is to say what a change deleted — so the criterion also requires that file not to
name the format, which is the only way the claim holds over the one surface its
own test cannot see.

FR-003 and FR-004 are specified and not implemented at this revision. This requirement covers the
generation behaviour that exists; it does not create a proof obligation over absent code, because a
proof obligation whose subject does not exist is the most complete false green available.
