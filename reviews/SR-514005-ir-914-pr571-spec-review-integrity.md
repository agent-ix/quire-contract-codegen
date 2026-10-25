---
id: SR-514005
title: "IR-914 PR #571 spec-review/integrity review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; PR #571 head d4b7e842fd28bb9c21e413cc3a807d1fa83782f2 from main 1944268f; spec/core/functional/interface-001-codegen-api.md, spec/evidence/functional/FR-004-vacuity-evidence.md, spec/evidence/matrix/TC-006-vacuity.md, spec/strategy/functional/FR-002-tristate-proptest.md, spec/strategy/functional/FR-011-numeric-harness-campaigns.md; IR-914"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-004
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-011
    type: reviews
---

## Summary

Non-author integrity review (completeness, consistency, atomicity, testability) of draft PR #571
at exact head d4b7e842. Ticket: IR-914. The port is byte-faithful to #476; the findings below
were present in #476 as well and were not raised by its reviews.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-011 gains normative V2 identity rules (occurrence key as `RequirementId`, full `package_id` as `RevisionId`, the `qocc2_`/`qpkg2_` grammar, comparison without normalization) but no FR-011 criterion, TC-020 step or matrix row verifies them for the numeric campaign runner; FR-011-AC-1..5 and TC-020 are unchanged, and FR-002-AC-12 covers only the Boolean harness | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:72-79; spec/strategy/functional/FR-011-numeric-harness-campaigns.md:117-140; spec/strategy/functional/FR-011-numeric-harness-campaigns.md:146-150 |
| FND-002 | low | FR-002-AC-12 now packs two obligations: byte-identical repeated generation, and identity rendering/distinctness across header, `RequirementId`, `RevisionId`, source map and symbols; split so a determinism failure and a rendering failure trace separately | spec/strategy/functional/FR-002-tristate-proptest.md:97 |

## Verdict

CONDITIONAL. Consistency across the five files is good: FR-002, FR-004, TC-006, interface-001 and
FR-011 name the same identity source, the same canonical renderings and the same separation from
same-run producer authentication, and each keeps the execution-point/anchor question open. The
grammar is self-delimiting (counted hex fields, fixed 64-digit node) and yields valid Rust
identifier characters for the symbol suffix. The open gaps are testability of FR-011's own
identity rules (FND-001) and AC atomicity (FND-002); the package-binding and acceptance-carrier
gaps are recorded in SR-514004.

## Coverage

```yaml
scope:
- id: FR-011
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "The runner shall construct each verdict through the runtime `construct_verdict` operation (quire-contract-runtime interface-001) with the selected accepted QSL item's occurrence key as requirement identity and the admitted `CheckedPackageV2::package_id()` as revision identity, together with the runtime execution point and clause observations."
- id: FR-011-AC-1
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "Over V2 selected claims, a QSL-produced `VersionUnchanged` (`Postcondition`) `Broad` campaign records `Holds` cases as `Passed` and `Violated` cases as `FailedPostcondition` and passes"
- id: FR-002-AC-12
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "Repeated V2 harness generation for the same admitted package, selected accepted requirement occurrences, selected clause identities and campaign policy is byte-identical. The generated header, `RequirementId`, `RevisionId`, source map and symbols carry the FR-011 canonical renderings of each selected accepted occurrence and the admitted package's full `package_id`; a distinct occurrence or package id yields distinct rendered identity text."
- id: FR-004-AC-10
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: "The selected accepted QSL requirement occurrence and admitted `CheckedPackageV2::package_id()` revision must match the campaign through their FR-011 canonical renderings or exact typed fields; noncanonical or mismatched text refuses before classification"
- id: TC-006
  path: spec/evidence/matrix/TC-006-vacuity.md
  role: examined
  excerpt: "Corrupt one occurrence part and one package-id field at a time in a controlled generated-map/campaign input, and refuse a noncanonical rendering without normalizing it."
- id: interface-001
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: "the V2 requirement identity is the selected QSL RequirementItem occurrence accepted by export_bindings and the revision identity is the admitted CheckedPackageV2 package_id (FR-011 OQ-2); neither identity authenticates the same-run association"
- id: TC-020
  path: spec/strategy/matrix/TC-020-numeric-harness-campaigns.md
  role: context_only
  excerpt: "Run a generated campaign with a report whose requirement identity is wrong and whose prior discarded count already exceeds the configured ceiling."
```

## New findings (disposition pass 1)

Re-checked at head 73e48d3cf37c12f63722fca1b8857b4f94399217.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | New refusals `OwnerIdentityMismatch`, `UnboundModelElement` and `InvalidIdentityRendering` are named bare, while the neighbouring codes are qualified (`StrategyErrorCode::`, `HarnessErrorCode::`) and main already has `ExactFunctionRefusal::OwnerIdentityMismatch` (FR-021-AC-37); name the owning enum for each generator | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:66-70; spec/strategy/functional/FR-002-tristate-proptest.md:39-43 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | FR-011-AC-8 to AC-12 (all PLANNED) cover numeric-runner identity rendering, owner mismatch, driver-boundary refusals, invalid rendering and repeat byte identity. TC-020 steps 9-11 and expected results trace them, and the strategy matrix row and TC-020 trace list now run to FR-011-AC-12. |
| FND-002 | fixed | FR-002-AC-12 keeps its byte-identity obligation, now over the full selection (package, clause ids, claim occurrences, policy) with no narrowing. Rendering and distinctness move to FR-002-AC-21 and identity refusals to FR-002-AC-22, both PLANNED and traced in TC-004 and the matrix. |
| FND-003 | fixed | Round 2, head dd07a018. FR-011 names `StrategyErrorCode::OwnerIdentityMismatch`, `::UnboundModelElement` and `::InvalidIdentityRendering`; FR-002 names the `HarnessErrorCode::` equivalents and maps them to FR-011's `StrategyErrorCode` variants. Each is a generator-owned code for a request/package owner mismatch, separate from FR-021's `ExactFunctionRefusal::OwnerIdentityMismatch` for FR-485 acquisition, so there is no duplicate owner. |
