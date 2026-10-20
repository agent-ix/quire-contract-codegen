---
id: SR-14095
title: "IR-914 PR #476 spec-review/base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR #476 head d58eac1a02af61671fd8298c3f248f6e28106094; five changed spec files; IR-914"
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

Non-author review of PR #476 at exact head d58eac1a02af61671fd8298c3f248f6e28106094. Ticket: IR-914. Compared the five changed spec files with QSL origin/main d656b0b6 and IR CheckedPackageV2.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Specify a lossless canonical text encoding for V2 occurrence and full package id before placing them in runtime RequirementId and RevisionId | spec/strategy/functional/FR-002-tristate-proptest.md:97; spec/strategy/functional/FR-011-numeric-harness-campaigns.md:72 |

## Verdict

CONDITIONAL. `RequirementId` and `RevisionId` are borrowed strings in quire-contract-runtime. QSL `OccurrenceKey` is a typed node/role/ordinal key with a Display form, while IR `CheckedSemanticId` is a domain/algorithm/digest record with no Display implementation. The new AC-12 says generated headers, symbols and runtime ids carry these identities, and FR-004-AC-10 compares them to campaign/map identities, but it specifies no injective text form for either record. Implementations could retain only the digest or render different strings from the same package, defeating the specified match or byte identity. Define one canonical lossless rendering (or a single typed comparison before deterministic emission) for these seams. Preserve all package-id fields and occurrence parts.

## Coverage

```yaml
scope:
- id: FR-002-AC-12
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "Repeated V2 harness generation for the same admitted package, selected accepted requirement occurrences, selected clause identities and campaign policy is byte-identical."
- id: FR-004-AC-10
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: "The selected accepted QSL requirement occurrence and admitted `CheckedPackageV2::package_id()` revision must match the campaign"
- id: FR-011
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "the selected accepted QSL item's occurrence key as requirement identity and the admitted `CheckedPackageV2::package_id()` as revision identity"
- id: TC-006
  path: spec/evidence/matrix/TC-006-vacuity.md
  role: examined
  excerpt: "For V2 identity, compare the selected QSL `RequirementItem` occurrence accepted by `export_bindings` (node, role and ordinal) and the admitted `CheckedPackageV2::package_id()` with the campaign, generated map and analyzed population."
```

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | At exact head `a86d380f2a929703b0807d368b837c7f04c5977f`, FR-011 defines `qocc2_` over the 64-digit node, UTF-8 role byte count/hex and decimal ordinal, and `qpkg2_` over counted UTF-8 domain, algorithm and digest. The grammar preserves every typed field and gives `RequirementId`, `RevisionId`, header, source map and symbol suffixes one injective canonical rendering; comparison rejects noncanonical text. FR-002-AC-12 and FR-004-AC-10 reference that rendering, and TC-006 requires one-field mutation controls. QSL `OccurrenceKey`/`WireNodeId` accessors and IR `CheckedSemanticId` fields support the specified values. FR-011 OQ-1 stays open and these strings remain separate from same-run producer authentication. No new finding in the narrow fix diff. |
