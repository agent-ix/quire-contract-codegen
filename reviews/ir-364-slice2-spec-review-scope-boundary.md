---
id: SR-11774
title: IR-364 slice 2 scope-boundary review of the V2 strategy chain criteria
type: SpecReview
analysis: scope-boundary
scope: agent-ix/quire-contract-codegen@f30f17b1a086211cd0ab87f2445eb5565fac74b5 (PR 400, diff vs origin/main 007417d); spec/assurance/AD-001, spec/assurance/AD-004, spec/core/functional/interface-001, spec/evidence/functional/FR-004, spec/core/functional/FR-005
review_set: subset
---

## Summary

Ticket: IR-364 (slice 2). This review covers the AD-004 and AD-001 edits that change which
requirements the IR-364 chain owns, and how this PR's boundary sits against CG #396 and #397. It
also checks that the open questions stay open.

## Verdict

**CONDITIONAL: one medium finding.**

These checks passed:

- **Cross-references.** FR-004 is vacuity evidence (`spec/evidence/functional/FR-004-vacuity-evidence.md`).
  FR-005 is CLI conformance (`spec/core/functional/FR-005-cli-conformance.md`). TC-034 exists nowhere
  in `spec/`.
- **AD-004 against the module map.** The AD-004 text agrees with the module map: `strategy` maps to
  `strategy/campaign.rs`, `bound_strategy/*` to `strategy/bound/` and `harness` to
  `strategy/harness.rs`. All three exist in `src/strategy/`.
- **AD-001 and AD-004 status.** "IR-703 implemented" matches IR FR-038-AC-202..209 on IR main 32e433a.
- **Overlap.** Nothing overlaps CG #396 or #397: no shared files, no edits to FR-014, 021, 031, 029
  or 033 or to TC-024, 031 or 044, and nothing under `spec/oracle/`.
- **Open questions.** These are left open and not decided silently:
  - interface-001 still lists `generate_bound_strategy` over `BoundPackage`/`ClauseRef`, and the PR
    does not touch it. The ticket says interface-001 changes with the code, because TC-028 checks it
    against the public functions.
  - The `generate_bound_*` versus FR-039 direction is not decided anywhere in the diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AD-004 narrows IR-364 from "FR-002, FR-004, FR-005, FR-008 to FR-013" to "FR-002 and FR-008 to FR-013" in three places (:427, :886-889, :1338). Moving FR-004 out of the chain is plausible, since FR-004 is evidence and its Inputs still read the V1 "bound executable clause population". But the new text only says FR-004's V2 criteria are "also required before step 6" and names no owning ticket. FR-005 is dropped from the list with no statement of whether it needs V2 criteria or who owns them. The IR-364 ticket title (tracker data) still lists FR-004 and FR-005. The result is a V1 reader (`evidence/bound_coverage`) whose step-6 deletion depends on V2 criteria that nothing owns. Name the ticket that owns the FR-004 (and, if needed, FR-005) V2 criteria, or keep them in IR-364. The planner should then align the ticket title. | spec/assurance/AD-004-cg-crate-layout.md:427,886-889,1338 |

## Examined units

```yaml
scope:
- id: AD-004-cg-crate-layout
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "IR-364 (the V2 strategy chain: FR-002 and FR-008 to FR-013 over `CheckedPackageV2`) is owned by the IR team and is ordered before this step; FR-004's V2 criteria for `evidence/bound_coverage` are not part of that chain and are also required before this step."
- id: AD-001-codegen-architecture
  path: spec/assurance/AD-001-codegen-architecture.md
  role: examined
  excerpt: "IR-703 is implemented; a CG-side real QSL producer fixture must exist before CG V2 implementation can claim the corresponding runtime tests."
- id: FR-004
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: context_only
  excerpt: "The IR-owned bound executable clause population, including clause identities, typed expressions, clause kinds, execution anchors, and dependency/declaration context."
- id: FR-005
  path: spec/core/functional/FR-005-cli-conformance.md
  role: context_only
  excerpt: "Serialized contract package, output directory, and backend selection."
- id: interface-001
  path: spec/core/functional/interface-001-codegen-api.md
  role: context_only
  excerpt: "generate_bound_strategy inputs: [public BoundPackage, full ClauseRef, population Satisfying|Violating|Broad|Boundary, campaign policy]"
bindings: []
```

## New findings (disposition pass 1)

Reviewed d8f2f8a6af1c791f5b0b48e5bb32eafae4474b6a (diff f30f17b..d8f2f8a).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | The new AD-004 step 6 text says FR-005 (CLI conformance) "names no `BoundPackage` input ... so it needs no V2 change of its own". FR-005's input is a "Serialized contract package". interface-001 defines `contract_package` as the "public IR BoundPackage from the public IR derived executable projection decoder" (:14), and `serialized_package_cli` says the decoder "now supplies BoundPackage" (:406). FR-005's input is therefore V1 through interface-001, even though FR-005 does not spell `BoundPackage`. The conclusion "needs no V2 change" is unsupported. The author's round note says FR-005 "follows FR-002 under IR-815", but the AD text says something different. Pick one: route FR-005's V2 input to IR-815, or tie it to the open interface-001 direction. Do not assert that no change is needed. | spec/assurance/AD-004-cg-crate-layout.md:888-890,1340, spec/core/functional/interface-001-codegen-api.md:14,406 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |

## Disposition round 1 verdict

**PASS on FND-001, with one new low finding (FND-002).**

- **FND-001.** AD-004 (:427, step 6 and the rulings row) now assigns the FR-002 and FR-004 V2
  criteria to IR-815, ordered before step 6. IR-815 exists in Linear ("CG spec: FR-002 and FR-004 V2
  criteria (needed before AD-004 step 6)", Backlog). IR-364 is stated as FR-008 to FR-013 and
  NFR-004.
- **FND-002.** The FR-005 part of the original finding was answered with a claim that does not hold.
  That is recorded as FND-002.
