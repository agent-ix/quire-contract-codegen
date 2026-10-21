---
id: SR-14301
title: spec-review/base review of IR-615 profile disposition specification
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen; spec/kani/functional/FR-015-bounded-kani-obligations.md;
  spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md
review_set: subset
---

## Summary

Ticket: IR-615. Independent review of PR #482; only two Markdown files change. The changed AC keeps its unique criterion id, replaces the legacy profile disposition with exact Supported/Refused census preservation, and uses the method-only Test verification cell. TC-023 remains linked to FR-015. The existing FR-015 stakeholder/dependency relationships are unchanged. External FR-029 and FR-030 references resolve to real owner requirements. No new file-tracking or digest requirement is introduced. Canonical proof-identity prose elsewhere in these large documents is unchanged.

## Examined Scope

```yaml
scope:
- id: FR-015-AC-82
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-profile classifier, a request for
    distinct named constructs returns one `CapabilityEntry` per requested construct
    in request order, preserving each admitted `Supported` module or `Refused` cause
    code exactly. The Contract IR profile reader rejects the removed `inconclusive`
    disposition rather than admitting it; no profile disposition or profile-supplied
    code becomes an `Inconclusive` run outcome or asserts proof vacuity.
- id: FR-015-AC-83
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: Until FR-015-AC-111 deletes the bounded-profile classifier, an empty, duplicate
    or absent construct is a typed `KaniOutcome` refusal rather than a partial disposition
    census.
- id: FR-015-interim-ownership
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Planned (IR-615): remove the legacy

    profile `Inconclusive` disposition: profile selection cannot establish the run-only

    vacuity classification governed by

    [Contract IR FR-030](ix://agent-ix/quire-contract-ir/FR-030). This cleanup does

    not complete the `unsupported` matrix entry, capability-kind warning and

    no-artifact negotiation behavior required by

    [Contract IR FR-029](ix://agent-ix/quire-contract-ir/FR-029), AC-2;

    that integration remains planned.'
- id: TC-023-Expected-Results
  path: spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md
  role: examined
  excerpt: 'The census preserves each admitted `Supported` module and

    `Refused` cause code exactly. The removed `inconclusive` profile disposition is

    rejected by the owning reader, with no admitted profile or manufactured run

    outcome. Full `unsupported` matrix entries, capability-kind warnings and

    no-artifact negotiation semantics remain planned under Contract IR FR-029-AC-2;

    this cleanup does not establish that coverage. The IR-615 cleanup remains

    planned until its producer and consumer changes are implemented.'
- id: FR-029-AC-2
  path: quire-contract-ir/spec/kani/functional/FR-029-versioned-bounded-kani-profile.md
  role: context_only
  excerpt: Every encountered construct has exactly one supported, refused, or unsupported
    matrix entry; an unsupported entry settles its item `unsupported` with a warning
    naming the item's capability kind from the closed `quire.capability-kind/v1` vocabulary
    (QSpec FR-290) and emits no artifact, `KaniOutcome` or `TerminalValue`; no matrix
    entry records resource exhaustion, cancellation or an absent executable; unknown,
    conflicting, and incomplete matrices refuse with the originating source identity.
- id: TC-023-Test-Procedure-1
  path: spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md
  role: examined
  excerpt: 'Request supported and refused constructs from one selected profile in
    a

    reordered subset; compare the returned modules and cause codes with the selected

    entries.'
- id: TC-023-Test-Procedure-2
  path: spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md
  role: examined
  excerpt: Submit empty, duplicate and unknown names.
- id: TC-023-Test-Procedure-3
  path: spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md
  role: examined
  excerpt: 'For the planned IR-615

    cleanup, at Contract IR''s owning

    profile reader, submit the removed `inconclusive` disposition with a cause code;

    require rejection before a profile can be classified or lowered.'
- id: TC-023-Test-Procedure-4
  path: spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md
  role: examined
  excerpt: 'Inspect the

    retained profile classifier and lowerers to confirm that no profile disposition

    or profile-supplied code constructs an `Inconclusive` run outcome.'
- id: TC-023-Test-Procedure-5
  path: spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md
  role: examined
  excerpt: 'This check

    does not replace installed-backend proof-vacuity classification.'
```

## Method and Evidence

No applicable AssuranceProfile or required review-selection profile exists in this repository spec tree. The brief selects base plus the applicable integrity, evidence, failure-domain and scope-boundary lenses. EARS is inapplicable: no Description/Behavior/Constraints requirement statement changes; only interim status, AC-82 and test procedure/results change. Dependency analysis is inapplicable: no relationships edge changes. Object, risk-complexity and criterion-strength analyses are not selected for this small existing-boundary amendment.

Rust-review is inapplicable: neither Rust nor Cargo inputs change. Standalone gap-analysis is inapplicable to this Markdown-only diff with no production changes or embedded spec tests; no repository-wide assurance PASS is claimed. As a base coverage check, the computed matrix reports FR-015-AC-82 untagged with zero binders. Existing source tests use TC-023 tags, which are not criterion bindings; legacy behavior still exists and producer/consumer implementation is explicitly planned. This is not a claim that AC-82 has been implemented or tested. The forthcoming implementation review must require actual criterion-tagged rejection/census controls. Plan completion: not assessed.

No Cargo, rustc, fmt, metadata, native test or Kani/backend execution was run. Quire validated both changed Markdown inputs; grammar summary 2/2 clean and zero grammar findings. Source examination confirmed the actual owner enum, private-field constructor/serde admission and current offending lowering arm; they remain unfixed by this spec-only PR. Genuine run-level Inconclusive is not removed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for the reviewed specification amendment. This is not acceptance of the planned producer/consumer implementation or full Unsupported negotiation feature.
