---
id: SR-2026102765
title: IR-1079 CG structural identity specification review
type: SpecReview
analysis: base
scope: 'CG #658, 37182d8777c7320e6309cc0e10564f16271d8c4b; IR-1079; 19 changed spec
  documents'
review_set: subset
---

## Summary

Independent scoped review of CG #658 against IR #720 structural types, QSL ADR-013 O-09 correction 878381034d5ddda662def7cffe0096d0e3c5f0cb, and the prior acceptance rows. No applicable AssuranceProfile is present under spec/. Quire validation and quoin validation pass. Four findings require correction before merge.

## Verdict

CHANGES REQUESTED. The retired-with-replacement structure preserves the principal obligations, but valid wrong identities are confused with malformed text and the frame introduction retains the obsolete identity shape.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | A different but canonically rendered nested node or lock value is an identity mismatch, not an invalid rendering. Separate malformed/noncanonical text from a valid wrong identity and specify the matching refusal. | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:290 |
| FND-002 | medium | A wrong nested path, package name or lock member can still form canonical structural text. InvalidIdentityRendering cannot classify every such mismatch; separate canonicality failure from valid wrong identity. | spec/strategy/functional/FR-002-tristate-proptest.md:213 |
| FND-003 | medium | The frame introduction still says AC-47 through AC-30 and places arguments in the identity, then describes a CG identity function. This contradicts the replacement three-component producer identity and leaves the section ambiguous. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:121 |
| FND-004 | low | The same structural CheckedNodeId embeds its package name, so the same-node/different-package-name case is impossible. Specify different locks for the same-node control; different names belong to the different-node control. | spec/core/functional/FR-039-contract-package-consumption.md:221 |

## Coverage

All 19 changed documents were examined; new and retired acceptance rows, linked TC descriptions, and interface wording were compared with the previous version and upstream typed APIs. Existing matrix gaps remain planned and are not claimed as implementation evidence. No Cargo or code gate ran.

## Dispositions

At 6f85d981c76ec893ae60674dc667ca4ea71db55f, all four findings are fixed; this scoped specification review is mergeable. The original review verdict above remains the historical first-pass result.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | FR-011-AC-21 and Behavior now separate malformed rendering from a valid but wrong occurrence or package; TC-020 checks each typed cause. |
| FND-002 | fixed | FR-002-AC-26 and Behavior make the same distinction; TC-004 checks each typed cause. |
| FND-003 | fixed | FR-024 introduction now cites AC-47 through AC-51 and AC-25 through AC-30, places arguments on the separate record, and names the QSL producer-issued tuple. |
| FND-004 | fixed | FR-039-AC-11 and TC-053 use same-node/same-name/different-lock and changed-name/changed-node controls. |

```yaml
scope:
- id: AD-003
  path: spec/assurance/AD-003-evidence-chain.md
  role: examined
  excerpt: Its structural O-09 replacement and separate same-artifact content tie are explicitly scoped below.
- id: FR-039
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: map and structural package name plus immutable lock identity.
- id: interface-001
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: 'revision: for V1 compare IR and source-map u64 revision to runtime RevisionId using the canonical decimal string
    without leading zeroes; for V2 compare FR-011 canonical structural qocc2_ and qpkg2_ renderings of the QSL driver-bound
    FR-331 claim occurrence and CheckedPackageV2 name plus complete lock matched to the process FR-331 request owner when
    a process frame exists, or from the linked driver''s one E4-emitted/E5-admitted package in generated map and runtime RequirementId/RevisionId,
    using exact canonical text or typed fields without u64 conversion or digest-only matching'
- id: TC-053
  path: spec/core/matrix/TC-053-contract-package-consumption.md
  role: examined
  excerpt: has the admitted source package name and complete lock as `contract_package_identity`, the requested
- id: spec/core/matrix/tests.md
  path: spec/core/matrix/tests.md
  role: examined
  excerpt: "| FR-039 | FR-039-AC-1 through FR-039-AC-3, FR-039-AC-6, FR-039-AC-8, FR-039-AC-11 | TC-053 | \U0001F6A7 Planned;\
    \ production FR-330 consumer and criterion-tagged integration tests are not implemented |"
- id: FR-016
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: '- The generator shall carry the producer''s QSL structural `ObligationIdentity` for the'
- id: FR-024
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: The frame path's obligation identity, witness and pre-state tie are specified by FR-024-AC-47 to
- id: TC-026
  path: spec/replay/matrix/TC-026-witness-native-replay.md
  role: examined
  excerpt: For FR-016-AC-26 to AC-28, build the replay request for a function with two parameters
- id: TC-035
  path: spec/replay/matrix/TC-035-counterexample-envelope-intake.md
  role: examined
  excerpt: 18. Capture the QSL producer's exact typed `ObligationIdentity(NodeKey, Option<ModelIdentity>,
- id: spec/replay/matrix/tests.md
  path: spec/replay/matrix/tests.md
  role: examined
  excerpt: "| FR-024 | FR-024-AC-7, FR-024-AC-8, FR-024-AC-47 through FR-024-AC-51 | TC-035 | \U0001F6A7 Planned; these criteria\
    \ still lack tests of their own |"
- id: FR-019
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: reader's `CheckedPackageRefusal` has public `code`, nonempty ordered typed `loci`,
- id: TC-030
  path: spec/routed/matrix/TC-030-capability-settlement.md
  role: examined
  excerpt: same bytes and limits, including `code`, nonempty ordered typed `loci` with exact paths and source regions, `cause`,
    `locus`, `contract_version`, `document_pointer`, `extension_detail`, `protocol_detail` and `terminal_duplicate`, with
    every present and absent optional field. Assert obsolete `path`, `expected` and `actual` are not projection members. Exercise
    distinct extension, protocol and terminal detail refusals.
- id: spec/routed/matrix/tests.md
  path: spec/routed/matrix/tests.md
  role: examined
  excerpt: "| FR-019 | FR-019-AC-49 through FR-019-AC-52 | TC-030 | \u2705 Covered; tagged TC-030 tests assert typed whole-request\
    \ reader projections, first-failing-item order, admitted support and item-level backend refusal |"
- id: FR-002
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: '| FR-002-AC-21 | RETIRED, replaced by FR-002-AC-25; the former digest-shaped identity rendering is obsolete and
    earns no implementation credit. | Inspection |'
- id: FR-011
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: '- For a process-provider request, CG shall compare the FR-331 request''s structural package name'
- id: TC-004
  path: spec/strategy/matrix/TC-004-proptest-strategies.md
  role: examined
  excerpt: identities, source map, and symbols against the FR-011 `qocc2_`/`qpkg2_` grammar. Change only the occurrence structural
    node package, nested path or ordinal, then role or occurrence ordinal, and separately the package name or one lock field
    using admitted matching FR-331 request/package pairs; require the corresponding rendered identity to change. A body-only
    change with the same structural identity still fails the admitted-body comparison. For a linked
- id: TC-020
  path: spec/strategy/matrix/TC-020-numeric-harness-campaigns.md
  role: examined
  excerpt: Independently change the authentic occurrence's structural node package, nested path or ordinal, its role or occurrence
    ordinal, and the admitted package name or one lock field using separate accepted matching request/package pairs; compare
    identity text. Compare a body-only edit with unchanged structural identity through the admitted-body check.
- id: spec/strategy/matrix/tests.md
  path: spec/strategy/matrix/tests.md
  role: examined
  excerpt: "| TC-004 | Preserve shaped proptest strategies | Property | P0 | FR-002-AC-1, FR-002-AC-2, FR-002-AC-3, FR-002-AC-4,\
    \ FR-002-AC-5, FR-002-AC-6, FR-002-AC-7, FR-002-AC-8, FR-002-AC-10, FR-002-AC-11, FR-002-AC-12, FR-002-AC-13, FR-002-AC-14,\
    \ FR-002-AC-15, FR-002-AC-16, FR-002-AC-17, FR-002-AC-18, FR-002-AC-19, FR-002-AC-20, FR-002-AC-22, FR-002-AC-24, FR-002-AC-25,\
    \ FR-002-AC-26 | \U0001F6A7 Planned; AC-7 to AC-20, AC-22 and replacement AC-25/26 are V2 planned (IR-815), AC-24 is V2\
    \ planned (IR-364) |"
- id: spec/tests.md
  path: spec/tests.md
  role: examined
  excerpt: "| Routed | FR-019, FR-022, FR-026 | [routed](./routed/matrix/tests.md) | \U0001F6A7 Process cases of FR-019-AC-1,\
    \ AC-11 to AC-13 and AC-16 to AC-23; FR-022-AC-17 to AC-22; and FR-026-AC-5 are planned for TC-046 or analysis. FR-019-AC-49\
    \ to AC-52 are covered; replacement AC-54 is planned in TC-030. FR-026-AC-1 and AC-4 remain planned for TC-037; FR-022-AC-1,\
    \ AC-6 and AC-16 remain open |"
findings:
- fnd: FND-001
  severity: medium
  confidence: high
  method: spec-review/base
  check: soundness
  artifact_id: FR-011-AC-21
  related_ids: []
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  lines: '290'
  excerpt: '| FR-011-AC-21 | PLANNED (IR-1079; replaces AC-11). Malformed, truncated, case-folded or otherwise noncanonical
    structural `qocc2_`/`qpkg2_` text, including a wrong byte count, nested node path or lock member, yields `StrategyErrorCode::InvalidIdentityRendering`
    before bundle publication or verdict/report mutation, without normalization; exact typed and canonical-text comparisons
    agree on the accepted pair. | Test (TC-020) |'
  finding: A different but canonically rendered nested node or lock value is an identity mismatch, not an invalid rendering.
    Separate malformed/noncanonical text from a valid wrong identity and specify the matching refusal.
- fnd: FND-002
  severity: medium
  confidence: high
  method: spec-review/base
  check: soundness
  artifact_id: FR-002-AC-26
  related_ids: []
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  lines: '213'
  excerpt: '| FR-002-AC-26 | PLANNED (IR-815; replaces AC-23). At a V2 harness identity text comparison boundary, malformed,
    truncated, case-folded or otherwise noncanonical structural `qocc2_`/`qpkg2_` text, including a wrong nested path, name,
    lock member or byte count, returns `HarnessErrorCode::InvalidIdentityRendering` before publication and without normalizing
    or mutating a report; exact typed and canonical-text comparisons agree on the accepted selection. | Test (TC-004) |'
  finding: A wrong nested path, package name or lock member can still form canonical structural text. InvalidIdentityRendering
    cannot classify every such mismatch; separate canonicality failure from valid wrong identity.
- fnd: FND-003
  severity: medium
  confidence: high
  method: spec-review/base
  check: ambiguous
  artifact_id: FR-024
  related_ids:
  - FR-024-AC-47
  - FR-024-AC-48
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  lines: '121'
  excerpt: The frame path's obligation identity, witness and pre-state tie are specified by FR-024-AC-47 to
  finding: The frame introduction still says AC-47 through AC-30 and places arguments in the identity, then describes a CG
    identity function. This contradicts the replacement three-component producer identity and leaves the section ambiguous.
- fnd: FND-004
  severity: low
  confidence: high
  method: spec-review/base
  check: untestable-ac
  artifact_id: FR-039-AC-11
  related_ids: []
  path: spec/core/functional/FR-039-contract-package-consumption.md
  lines: '221'
  excerpt: '| FR-039-AC-11 | Generated scalar, composite equality and exact function claims each serialize `contract_package_identity`
    equal to the admitted FR-330 `package.source` structural package name plus complete lock, replacing the removed `ContractPackageV1.identity()`
    accessor, and `node_id` equal to the requested structural `CheckedNodeId`, with no generated-claim `ir_id` key. Each retains
    exact `semantic_type`, admitted `source_map`, family-specific provenance, and `package_id` denoting the originating checked
    package; each serializes typed `claims`, `bounds` and `dependencies`. The same reques'
  finding: The same structural CheckedNodeId embeds its package name, so the same-node/different-package-name case is impossible.
    Specify different locks for the same-node control; different names belong to the different-node control.
bindings: []
```
