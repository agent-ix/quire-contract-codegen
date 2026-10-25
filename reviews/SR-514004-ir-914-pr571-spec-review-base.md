---
id: SR-514004
title: "IR-914 PR #571 spec-review/base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR #571 head d4b7e842fd28bb9c21e413cc3a807d1fa83782f2 from main 1944268f; spec/core/functional/interface-001-codegen-api.md, spec/evidence/functional/FR-004-vacuity-evidence.md, spec/evidence/matrix/TC-006-vacuity.md, spec/strategy/functional/FR-002-tristate-proptest.md, spec/strategy/functional/FR-011-numeric-harness-campaigns.md; IR-914"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-004
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-011
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-006
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: reviews
---

## Summary

Non-author base spec review of draft PR #571, the forward-port of draft PR #476 (IR-914 V2
requirement and revision identity), at exact head d4b7e842 on CG main 1944268f. Ticket: IR-914.
Compared against #476 head 33ae65ce, its four prior SR artifacts (SR-14094..SR-14097) and the
IR-914 Linear review comments, current CG main (FR-021 FR-485 owner-path identity, AD-004 L-12),
QSL main 305f199e (`OccurrenceKey`, `WireNodeId`, `export_bindings`, `RequirementItem`, ADR-017
AR-4), IR main ea9af060 (`CheckedSemanticId`, `CheckedPackageV2::package_id`), and the open
draft execution-point PR #572.

Port fidelity: every added and removed line of the #571 diff is byte-identical to the #476 spec
diff (diff-of-diffs over `+`/`-` lines is empty); #571 carries the five spec files only and none
of #476's review artifacts. No identity rule was dropped, added or reworded. Prior finding
SR-14095 FND-001 (lossless canonical rendering) stays fixed at this head: FR-011 "Canonical V2
identity rendering" defines `qocc2_`/`qpkg2_` over every typed field. SR-14094, SR-14096 and
SR-14097 had no findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Identity pair is not bound to one package: the requirement occurrence comes from the QSL package `export_bindings` accepted it against, the revision from the IR `CheckedPackageV2`, and no rule requires them to be the same package or refuses a mismatch (contrast FR-021-AC-37 `OwnerIdentityMismatch`) | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:49-51; spec/strategy/functional/FR-011-numeric-harness-campaigns.md:159-164 |
| FND-002 | medium | "Accepted by `export_bindings`" has no stated verifier or carrier: per QSL ADR-017 AR-4 the driver calls the export and CG receives only bound items, yet FR-011 takes a bare `RequirementItem`; no rule says whether CG calls the export, receives a `BoundItem`, or trusts the caller, and no outcome is specified for an unaccepted or `Unbound` occurrence | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:49-51; spec/strategy/functional/FR-002-tristate-proptest.md:23-25 |
| FND-003 | low | Overlaps draft PR #572 (FR-011 execution-point decision): both rewrite FR-002 Inputs, FR-002-AC-12, FR-002 OQ-1, the FR-011 `construct_verdict` bullet and FR-011 OQ-1/OQ-2, and this PR restates "execution-point source remains open" in FR-002, FR-004 and FR-011; whichever lands second must reconcile, and FR-004 OQ-1's FR-011 OQ-1 reference will not conflict textually but dangles once #572 deletes it | spec/strategy/functional/FR-002-tristate-proptest.md:97; spec/evidence/functional/FR-004-vacuity-evidence.md:240-241 |

## Verdict

CONDITIONAL; mergeable on review once FND-001 and FND-002 are resolved (FND-003 is a sequencing
note for whichever of #571/#572 lands second). The merge decision is Peter's (identity decision).

Verified clean: no AC narrowed (FR-002-AC-12 and FR-004-AC-10 gain the identity inputs and the
rendering/match rule; the V1 decimal-revision rule in interface-001 is unchanged); no status
overstated (every touched criterion stays PLANNED, no matrix row promoted, FR-011 OQ-1 kept open);
no duplicate identity owner (QSL keeps `OccurrenceKey`/`WireNodeId` and IR keeps
`CheckedSemanticId`; CG defines only an injective text rendering at the borrowed-string runtime
boundary and forbids computing a new digest or a separate requirement-revision artifact); no
compatibility, fallback or digest-only identity path (V2 forbids u64 conversion, digest-only
matching, truncation, case folding and normalization); the identity is explicitly kept separate
from same-run producer authentication (FR-004-AC-10, AC-14). `package_id` is the canonical
content identity binding a verdict to the package revision (QSL ADR-017 AR-4 relation revision),
the one digest use the checklist permits. Nothing names `BoundPackage` (AD-004 L-12). The
`qpkg2_` projection keeps `algorithm` while FR-021-AC-36's `DigestRecord` mapping uses domain and
digest only; these are different projections of the same id and do not conflict. Accessors named
by the grammar exist on QSL and IR main. `quire validate --summary` on the five files: 5/5
grammar-clean; `git diff --check` clean.

## Coverage

```yaml
scope:
- id: FR-011
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "The selected QSL `RequirementItem`, whose `occurrence()` has an `Ok` result from `qsl_package::export_bindings`, and the admitted `CheckedPackageV2` whose `package_id` identifies its revision."
- id: FR-011-OQ-2
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "OQ-2 (resolved, requirement and revision identity): the selected QSL `RequirementItem` whose occurrence was accepted by `export_bindings` supplies its requirement occurrence key, including node, role and ordinal; the admitted `CheckedPackageV2::package_id()` supplies the revision identity."
- id: FR-002-AC-12
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "Repeated V2 harness generation for the same admitted package, selected accepted requirement occurrences, selected clause identities and campaign policy is byte-identical. The generated header, `RequirementId`, `RevisionId`, source map and symbols carry the FR-011 canonical renderings of each selected accepted occurrence and the admitted package's full `package_id`; a distinct occurrence or package id yields distinct rendered identity text."
- id: FR-004-AC-10
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: "The selected accepted QSL requirement occurrence and admitted `CheckedPackageV2::package_id()` revision must match the campaign through their FR-011 canonical renderings or exact typed fields; noncanonical or mismatched text refuses before classification"
- id: FR-004-OQ-1
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: "OQ-1 (anchor open; identity resolved): the V2 source of a clause's execution anchor remains FR-011 OQ-1."
- id: TC-006
  path: spec/evidence/matrix/TC-006-vacuity.md
  role: examined
  excerpt: "Corrupt one occurrence part and one package-id field at a time in a controlled generated-map/campaign input, and refuse a noncanonical rendering without normalizing it."
- id: interface-001
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: "for V2, use FR-011 canonical qocc2_ and qpkg2_ renderings of the selected accepted QSL occurrence and full admitted CheckedPackageV2 package_id in generated map and runtime RequirementId/RevisionId"
- id: FR-021-AC-37
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: context_only
  excerpt: "If the acquired QSL package/selection owner or callable id differs from the requested admitted V2 package/function identity, the item is refused as `ExactFunctionRefusal::OwnerIdentityMismatch`"
- id: AD-004
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: context_only
  excerpt: "L-12. `BoundPackage` and `BoundClause` are named nowhere in `kani/`, `routed/` or `replay/`"
```

## New findings (disposition pass 1)

Re-checked at head 73e48d3cf37c12f63722fca1b8857b4f94399217, rebased on main 4cb41df5 (#572-#575 merged).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | The owner check compares FR-331 `request.package_id`, but QSpec FR-331 gives that member only to the process-provider frame; the in-process form CG also consumes (FR-019 "in-process CG view", `ProviderOrigin::Linked`) carries no `package`/`package_id`, so the same-package rule has no stated source for a linked provider | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:92-97; spec/strategy/functional/FR-011-numeric-harness-campaigns.md:198 |
| FND-005 | low | Port drops two #476 grammar sentences without a review decision behind it: "`qocc2_` and `qpkg2_` separate the two identity domains" and the symbol-suffix clause "where both components follow the grammar above"; the literal prefixes still separate the domains, so restore the sentences or record the drop as deliberate | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:170-178 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | FR-011 Behavior compares FR-331 `request.package_id` with `CheckedPackageV2::package_id()` over domain, algorithm and digest and returns `OwnerIdentityMismatch` with no artifact or verdict; FR-011-AC-9, FR-002-AC-22, TC-020 step 10 and TC-004 verify it. Sibling-package substitution is forbidden. The in-process gap is new FND-004. |
| FND-002 | fixed | FR-011 states the ADR-017 AR-4 / FR-307 driver boundary: the driver forwards only `export_bindings`-accepted items, `UnknownItem`/`Unbound`/`Fault` stay driver-side, and CG does not call the export but defensively returns `MissingClaimOccurrence` or `UnboundModelElement`; FR-011-AC-10 and FR-002-AC-22 verify it. No new QSL input or facade is added. |
| FND-003 | fixed | The branch is rebased on main 4cb41df5 with #572 merged. FR-011 OQ-1 no longer exists and no spec cites it; FR-002-AC-19/20, FR-011-AC-6/7 and the merged point and occurrence text are unchanged; FR-004 OQ-1 is now identity-only and points to FR-011's execution-point rule. |
| FND-004 | fixed | Round 2, head dd07a018. FR-011 Inputs/Behavior and FR-011-AC-9 split the routes. The process frame compares reader-validated `request.package_id` with the admitted id and returns `StrategyErrorCode::OwnerIdentityMismatch`. On the linked route the driver exports, E4-emits and E5-admits one S4 package (E5's driver id check exists in QSL ADR-011), and CG checks claim membership in the held package, refusing a foreign claim as `MissingClaimOccurrence`. FR-002, FR-004, interface-001, TC-004 and TC-020 match. No new carrier, facade or wire member. |
| FND-005 | fixed | Round 2, head dd07a018. FR-011 again reads "`qocc2_` and `qpkg2_` separate the two identity domains" and "`cg_<qocc2-string>_<qpkg2-string>`, where both components follow the grammar above", matching #476. |
