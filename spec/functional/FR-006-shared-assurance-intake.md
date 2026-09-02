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
| FR-006-AC-5 | Pass, fail, unavailable, inconclusive, not-computed, malformed, partial, stale, suspect, vacuous, and tampered are each demonstrated by a case that produced it, `unsupported` is demonstrated by nothing, and every negative is paired with a positive control. | Test (TC-012) |
| FR-006-AC-6 | No repository-local generic runner, envelope builder, manifest, verifier, anchor writer, failure-propagation policer, or aggregate verdict remains, and the gates that replaced them are reachable from `ci`. | Test (TC-013) |

## Dependencies

- **Upstream**: the released Engineering Assurance, Quire/quire-cli, Quoin, and ix-flow versions
  recorded in `assurance/pins.json`; [FR-001](./FR-001-deterministic-oracles.md) and
  [NFR-002](../nonfunctional/NFR-002-provenance-boundary.md).

## Notes

The shared verification vocabulary is twelve states. FR-006-AC-5 requires eleven
of them. Measured on the tree before anything was deleted, per state and per
source: the assurance chain alone demonstrated ten, and the compatibility census
over the retained `evidence/` tree supplied `unsupported` and `malformed`. The
repository owner released the preservation constraint for the pre-stable phase on
2026-09-02 (`agent-ix/engineering-assurance#7`) and those records are deleted.

`malformed` did not go with them. It is a declared member of the assurance
chain's producer outcome vocabulary, so a producer reporting it is a state that
travels the intake path; the `attested-malformed` scenario demonstrates it by
deriving it from the real corpus run, exactly as the `fail` state is demonstrated.
Leaving it undemonstrated would have left TC-012 passing at ten and reported the
same green, which is a gate weakening rather than a claim being withdrawn.

`unsupported` is withdrawn. It is not in that producer vocabulary and never was:
it was a property of the compatibility mapping, raised by a retained record
carrying an unknown PGM-01 schema version, and nothing on the intake path
produces it. The generation corpus does reach an `unsupported` *Interface-001
terminal state*, and borrowing that would collapse two vocabularies this
requirement exists to keep apart. So the claim was removed with the evidence
rather than restated over a weaker substitute, and TC-012 asserts the state stays
absent so a later change cannot quietly re-acquire it from a refusal.

FR-003 and FR-004 are specified and not implemented at this revision. This requirement covers the
generation behaviour that exists; it does not create a proof obligation over absent code, because a
proof obligation whose subject does not exist is the most complete false green available.
