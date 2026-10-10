---
id: SR-4823
title: spec-review/integrity review of IR-335 counterexample intake
type: SpecReview
analysis: integrity
scope: agent-ix/quire-contract-codegen@6fd1ac20d815718d49d955eae64aeeda95f8cace; spec/replay/functional/FR-024-counterexample-envelope-intake.md,
  spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/assurance/AD-001-codegen-architecture.md,
  spec/replay/matrix/tests.md; ticket IR-335
review_set: subset
---

## Summary

Independent review of IR-335 draft PR #336 at the frozen head. Examined FR-024-AC-36 through AC-40, TC-035 steps 7 and 28, AD-001 replay boundary, and the replay matrix.

## Verdict

**CONDITIONAL** — 1 actionable specification gap.

## Examined Units

- `FR-024-AC-36` (examined, `spec/replay/functional/FR-024-counterexample-envelope-intake.md`): Two distinct test backend adapters submit equivalent typed assignments through one common intake contract carrying each adapter's backend identity, the same opaque obligation identity, declared domains, source/provenance and replay packet members. Native text is observed only within its own adapter; the common intake neither parses native text nor remints the obligation identity, and QSL receives each adapter's original backend and source identity in a complete packet.
- `FR-024-AC-37` (examined, `spec/replay/functional/FR-024-counterexample-envelope-intake.md`): Before any native replay, an out-of-domain assignment yields a typed out-of-domain evidence failure; an incomplete or QSL-refused witness/packet yields a typed QSL admission evidence failure retaining its cause. A decode failure reported by an adapter and a replay verdict disagreement remain distinct typed evidence failures. None settles as contract success or contract failure, and none of the pre-replay failures invokes replay. Kani's exact decode and refusal partition remain governed by FR-016.
- `FR-024-AC-38` (examined, `spec/replay/functional/FR-024-counterexample-envelope-intake.md`): Starting from one admitted original failure, one valid reduced candidate that preserves the failure becomes a new CG lineage revision referring to its exact admitted QSL envelope, its exact parent revision and the unchanged opaque obligation identity; the original envelope, evidence and lineage record remain byte-for-byte unchanged. No lineage member is added to QSL's envelope or packet.
- `FR-024-AC-39` (examined, `spec/replay/functional/FR-024-counterexample-envelope-intake.md`): From a retained failure revision, an out-of-domain reduction and a domain-valid reduction that changes the native verdict each create no revision and leave the retained chain and prior evidence unchanged; a later valid reduction names the actual retained parent, with no skipped or substituted parent.
- `FR-024-AC-40` (examined, `spec/replay/functional/FR-024-counterexample-envelope-intake.md`): A retained reduced `Witness` envelope holds a transcript from its own backend re-run, bound to its own reduced assignments; it cannot reuse any ancestor's transcript. A retained reduced `Input` envelope remains `Input`, contains its own canonical assignments and gains no backend witness or backend-evidence settlement.
- `TC-035-step-7` (examined, `spec/replay/matrix/TC-035-counterexample-envelope-intake.md`): Reduce Witness and Input failures through retained valid and rejected invalid candidates; inspect lineage and evidence before and after each proposal.
- `TC-035-step-28` (examined, `spec/replay/matrix/TC-035-counterexample-envelope-intake.md`): Submit two test backend adapters and decode, domain, QSL admission and verdict refusal cases through the common intake.
- `AD-001-replay-boundary` (examined, `spec/assurance/AD-001-codegen-architecture.md`): Backend adapters submit typed assignments, identity, domains, provenance and packet; CG validates, constructs QSL envelopes and owns immutable lineage records.
- `replay-matrix-IR-335` (examined, `spec/replay/matrix/tests.md`): FR-024-AC-36 through FR-024-AC-40 are Planned (IR-335); no production minimizer, lineage record or cross-backend intake test is claimed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The byte-for-byte immutability oracle has no defined bytes: QSL currently exposes an in-process WitnessEnvelope with Eq and to_packet, but no canonical envelope wire writer. Define an observable comparison of the original envelope and record, or name an existing byte representation. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:375 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | fa23816cdb6d1917da7966606b5120045dd40744 |

Disposition pass 1 at fa23816cdb6d1917da7966606b5120045dd40744: the revised FR-024-AC-38 states the observable behavior; TC-035 and AD-001 agree. After excerpt: Starting from one admitted original failure, one valid reduced candidate that preserves the failure becomes a new CG lineage revision referring to its exact admitted QSL envelope, its exact parent revision and the unchanged opaque obligation identity. After retention, the original QSL envelope compares equal to its pre-reduction clone by `WitnessEnvelope<P>: Eq`, and the original CG record's revision identity, parent, obligation and typed evidence fields read back equal to their captured values. No lineage member is added to QSL's envelope or packet.
