---
id: "SR-2922"
title: "IR-629 integrity analysis: process-provider settlement"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen branch spec/ir-629-input-contract (frozen head recorded in the IR-629 Linear review marker, not here); spec/decisions/ADR-002-backend-adapter-boundary.md, spec/routed/functional/FR-019-capability-settlement.md, spec/routed/functional/FR-022-routed-generation.md, spec/routed/functional/FR-026-backend-adapter-contract.md, spec/routed/matrix/TC-046-process-provider-settlement.md, spec/routed/matrix/tests.md, spec/tests.md; context spec/core/functional/interface-001-codegen-api.md"
---

# SR-2922: IR-629 integrity analysis

## Summary

Ticket: IR-629. I checked completeness, internal consistency and atomicity across the seven
changed files and the unchanged CG documents that restate the same types.

I built the process-arm truth table over: extent mode; advertised modes for the kind
({bounded}, {unbounded}, {bounded, unbounded}); bounded `bounds[].kind` coverage (empty, all
covered, one uncovered); unbounded `domains[].kind` coverage (all covered, one uncovered); and
`finite_bound_available`. Every cell has exactly one outcome under the FR-019 process bullets
(lines 124-146). Kinds repeated in `bounds[]` are harmless: the check is set membership, and
the wire makes manifest `domains` unique. Unbounded domain kinds `quantity`, `loop` and
`infinite-trace` can never be advertised, so they always decline on a bounded-only provider.
That outcome is consistent.

Units examined: FR-019 Behavior lines 97-150, FR-019-AC-3, FR-019-AC-11, FR-019-AC-12, FR-019
Inputs, FR-022 Inputs and Outputs, FR-022-AC-1, FR-022-AC-17, FR-026-AC-5, the ADR-002
Amendment, TC-046 steps 1 to 4, the routed matrix rows and `spec/tests.md` row Routed.
Relationship edges (TC-046 verifies FR-019, FR-022 and FR-026) resolve. The new matrix rows
are untagged and Planned, as stated. The `quire matrix` TSV against main changes only the
edited AC text and adds the two untagged rows, with no status regression.

## Verdict

Not merge-ready. The general advertised-mode rule and FR-019-AC-3 contradict the process rule
in one cell. FR-019-AC-12 asserts invariance over two inputs that CG's stated view does not
carry. Several restatements of the changed types were left stale.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The general rules contradict the process rule. FR-019 Behavior and FR-019-AC-3 settle every unbounded item on a bounded-only single candidate `requires-bound` when a finite bound is available. The process rule settles that item `unsupported`/`unbounded-extent` when a `domains[].kind` is unadvertised. Neither statement scopes itself to the Kani arm, so a process item with `finite_bound_available=true` and an uncovered domain has two required outcomes. | spec/routed/functional/FR-019-capability-settlement.md:99-101, 223, 133-139 |
| FND-002 | medium | FR-019-AC-12 and TC-046 step 3 require invariance under manifest run-limit defaults and under `temporal.subject.proof_bounds`. FR-019 Inputs give CG's descriptor identity, pairs, origin and `domains` (no `bounds`), and give the request item no temporal member. A CG test cannot vary inputs that CG never receives, so those clauses are either vacuous or imply members that the Inputs do not declare. | spec/routed/functional/FR-019-capability-settlement.md:38-53, 230; spec/routed/matrix/TC-046-process-provider-settlement.md:43-49 |
| FND-003 | low | `Process(BackendId)` is used throughout, but CG defines no `BackendId`, and FR-022 rules out `qsl-route`'s. The "canonical bytes" ordering key is undefined. `BackendKind::index`, which `generate_routed` sorts by, has no stated value for `Process`, and FR-019-AC-11 drops it silently. | spec/routed/functional/FR-022-routed-generation.md:88-91, 169-173; spec/routed/functional/FR-019-capability-settlement.md:229 |
| FND-004 | low | The ADR-002 Amendment says the process arm settles from the manifest and "the item's extent classification". FR-019 reads `bounds[].kind` and `domains[].kind`, which go beyond the classification. | spec/decisions/ADR-002-backend-adapter-boundary.md:91 |
| FND-005 | low | interface-001 still describes `GenerationContexts` as "one Option field per BackendKind". FR-022 removed that rule for `Process`. interface-001 is not updated. | spec/core/functional/interface-001-codegen-api.md:164 |
| FND-006 | low | The `spec/tests.md` Routed summary drops the earlier "FR-026 is planned". FR-026-AC-1 and AC-4 (TC-037, Planned) now appear in neither the planned list nor the open list. | spec/tests.md:19 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | interface-001 does not list CG's new public descriptor-to-kind conversion that the driver must call. | spec/core/functional/interface-001-codegen-api.md:159-166 |
| FND-008 | low | For a Process(id) identity mismatch, the value of BackendKindDisagrees.converted is unspecified. | spec/routed/functional/FR-022-routed-generation.md:269 |

## Dispositions

Round 1, against the fix-round head recorded in the IR-629 Linear dispositions marker.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | fix commit 'spec(CG): resolve IR-629 process review findings' |
| FND-002 | fixed | fix commit 'spec(CG): resolve IR-629 process review findings' |
| FND-003 | fixed | fix commit 'spec(CG): resolve IR-629 process review findings' |
| FND-004 | fixed | fix commit 'spec(CG): resolve IR-629 process review findings' |
| FND-005 | fixed | fix commit 'spec(CG): resolve IR-629 process review findings' |
| FND-006 | fixed | fix commit 'spec(CG): resolve IR-629 process review findings' |
