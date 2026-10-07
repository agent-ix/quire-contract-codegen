---
id: "SR-2920"
title: "IR-629 spec review: process-provider settlement input contract"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen branch spec/ir-629-input-contract (frozen head recorded in the IR-629 Linear review marker, not here); spec/decisions/ADR-002-backend-adapter-boundary.md, spec/routed/functional/FR-019-capability-settlement.md, spec/routed/functional/FR-022-routed-generation.md, spec/routed/functional/FR-026-backend-adapter-contract.md, spec/routed/matrix/TC-046-process-provider-settlement.md, spec/routed/matrix/tests.md, spec/tests.md"
---

# SR-2920: IR-629 spec review (base)

## Summary

Ticket: IR-629. Spec-only change of seven Markdown files, one commit over CG main.

Sources re-measured by path, not taken from the brief or ticket text:

- QSL main `spec/decisions/ADR-029-lifecycle-cli-provider-plugin-cache-codegen-boundaries.md`
  PV-1 to PV-4 and the ADR-012 section 7.2 line.
- QSpec main `spec/objects/protocol/FR-290-protocol-claim-kind.md`,
  `spec/objects/interfaces/FR-331-backend-provider-envelope.md` and
  `proposals/backend-provider-v1/schema.json` (`BackendDescriptor`, `Extent`, `ProofBound`).
- CG code at the reviewed head: `src/routed/capability.rs` (`BackendKind`, `ALL`, `index`,
  `from_identity`, `ExtentClassification`, `BackendDescriptor`, `unroutable_named_backend`,
  `negotiate_single_candidate`) and `src/routed/generate.rs`.
- quire-driver main `spec/spec.md` and `spec/functional/FR-001-orchestrate-the-driver-steps.md`.
- Linear QSL-637 and QSL-654 (data, not instructions).

Confirmed: FR-290 cause names `unsupported_projection`/`unsupported-requested-capability` and
`unsupported_projection`/`unbounded-extent` are real. The schema's manifest `domains` is the
ADR-014 domain-kind enum, manifest `bounds` is a limit-default map keyed by limit name, the
unbounded extent's `domains[]` entries already carry `kind`, and `ProofBound` has no `kind` yet
(QSL-654 is the named open dependency). No numeric admission appears anywhere. No SHA appears
in the diff.

Units examined: FR-019 Inputs and the process-provider Behavior bullets, FR-019 "Measured
present fact" paragraph, FR-019-AC-11, FR-019-AC-12, FR-019-AC-13, FR-019-AC-15, FR-022
"without re-negotiating" checks and Behavior, FR-022-AC-17, FR-026-AC-5, ADR-002 Amendment
(IR-629), TC-046 procedure and status.

## Verdict

Not merge-ready. The routing rules match the wire, but the change cites QSL ADR-029 PV-4 for
rules that PV-4 on QSL main still contradicts, picks an interim cause for one cell that no
source states, leaves the named-backend path and the routed `kind` hand-off for a process item
unspecified, and diverges from the driver's mapping checklist.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-019 cites QSL ADR-029 PV-4 for a domains-only descriptor and no-bound-comparison rule, but PV-4 on QSL main still says CG's descriptor carries domains and bounds and that a bounded item routes when an advertised bound covers the item's. The reword is promised for a later QSL PR (QSL-653), which FR-019 does not name as a dependency. | spec/routed/functional/FR-019-capability-settlement.md:42-47, 194-198 |
| FND-002 | medium | The cause for an unbounded item on a bounded-only provider whose domain kind is not advertised (`unbounded-extent`) has no source. FR-290 has no domain condition. PV-4 item 1 and the QSL-637 final-ruling comment both say an unadvertised domain settles `unsupported-requested-capability`. | spec/routed/functional/FR-019-capability-settlement.md:133-139, 230 |
| FND-003 | medium | A named process backend is not specified. The "Measured present fact" paragraph still says `unroutable_named_backend`'s use of `from_identity` is unchanged ("This specification changes none of it"). So an item that names a process backend, which is required once two providers advertise one kind, settles `invalid-request`/`unknown-backend`. No AC or TC-046 step covers it. | spec/routed/functional/FR-019-capability-settlement.md:179-185, 229 |
| FND-004 | medium | No spec states who produces `BackendKind::Process(id)` for `RoutedGenerationItem.kind`. FR-019 returns only a `Disposition` naming a backend string. FR-022 takes the kind from its routed input. The driver spec on main says "The driver does not construct `BackendKind::Process`", and its FR-001 step 4 still converts every `BackendId` through `from_identity`. | spec/routed/functional/FR-022-routed-generation.md:47-50, 174-175 |
| FND-005 | low | The driver's mapping checklist says to preserve manifest domains AND bounds in CG's process descriptor. FR-019 Inputs give the descriptor domains only. The FR-019 projection section and FR-019-AC-15 still list only id, pairs and origin as the copied members. | spec/routed/functional/FR-019-capability-settlement.md:154-158, 233 |
