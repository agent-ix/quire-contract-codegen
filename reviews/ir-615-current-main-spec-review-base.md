---
id: SR-15225
title: spec-review/base narrow rebase disposition of IR615 PR485
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen; current-main rebase, source equivalence, external
  review custody and three upstream planned criterion rows
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-015
  type: references
- target: ix://agent-ix/quire-contract-codegen/NFR-006
  type: references
---

## Summary

Ticket IR615, PR485; SAME reviewer narrow range/base/custody disposition at 3e6e1099513087a876d2471e0cf97a74d7dc655a, current base 942ece51039427eb7c58f54addc2c9b533a3a8ad, previously reviewed source 7d2c2b20dbdd677b31f2639e1e63f62e41784485. User specifically authorized an additional external non-author review and requires new final gates after current-main rebase. This is a narrow delta/source-equivalence pass, not a repeat of unchanged whole semantic review. No product edits or Cargo/rustc/fmt/metadata/native/backend/aggregate/heavy commands. No applicable AssuranceProfile. Prior final gate runs are historical only; both new final-head aggregate and full NFR006 make kani-gate remain PENDING.

## Verdict

CONDITIONAL — inherited table separation hides three new criteria; same coder must fix formatting.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The blank line before AC120 ends the acceptance table, and AC120/121/122 lack a new header/separator, so all three authored criteria disappear from the computed matrix. Both actual base/head return880rows. In an external diagnostic copy, removing only that blank line yields883rows and the three missing untagged criteria. Join these rows to the actual acceptance table; do not invent tags or alter their statements. | spec/kani/functional/FR-015-bounded-kani-obligations.md:842-845 |

## Coverage

Independently reran git range-diff: all eight authored patches are '='. Independently compared source/tests/Cargo manifest/lock byte-for-byte old-to-current: no difference. Own AC82/83/86/88 statements/methods, TC023 and all six prior review report bytes are unchanged. Therefore prior three concrete code findings remain FIXED and all309prior owned deferrals remain intact. The added external SR15220/15221/15222 reports were read in full and independently matched actual Git bytes to their recorded transfer bytes; no report text was edited. Their earlier-head PASS/native compile evidence is not a new final gate or a substitute for the user's required reruns. No .github/Makefile/scripts gate change. There is no new production behavior, API, unsafe, conversion, lifecycle, test seam, copied producer schema or invented trace tag in this delta. External reports are reviewer records, not new normative tool/version tracking rules.

Current source equivalence retains the genuine Supported/Refused-only profile behavior and actual run Inconclusive/vacuity classifier separation. Re-read changed current-main IR942 clauses as baseline context only; their implementation is explicitly PLANNED. The unexpected extraction defect is recorded in the base/gap artifacts, not hidden by the code PASS. Full computed matrix at actual current base/head was executed under the same installed engine; diagnostics and structured data are preserved outside the branch. Scope does not claim optional semantic fanout or new dynamic proof qualification.

Base applicability is limited to current-main three planned criterion rows, their trace extraction and unchanged IR615 status paragraphs. No normative statement/object/relationship change was authored by this rebase; no whole unrelated spec review is repeated. The actual omitted rows are a structural defect, independently demonstrated on an external diagnostic copy. EARS/domain/architecture specialist semantic reviews of IR942 are outside this narrow rebase disposition. Original external report bytes remain intact.

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
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-profile classifier, an empty, duplicate
    or absent construct is a typed `KaniOutcome` refusal rather than a partial disposition
    census.
- id: FR-015-AC-86
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the graph slice (FR-015-AC-111) deletes the finite-graph lowerer,
    a profile/dispatch-admitted request follows only the selected reference field,
    finds a positive-length path in sorted depth-first order, and checks the expansion
    bound before entering each new object; a target edge is found without expanding
    its target.
- id: FR-015-AC-88
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the collection slice (FR-015-AC-110) deletes the finite-collection
    lowerer, a profile/dispatch-admitted `ForAllNonNegative` or `ExistsEqual` query
    retains input order and duplicates and returns its truth value and examined-item
    count; a decisive item ends that count, otherwise every item counts.
- id: NFR-006-AC-13
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: For a Kani-touching pull request, the author runs the full gate on its
    clean, stable candidate before opening it, and the merger runs it again on the
    clean, stable current candidate before merge; both actual processes exit successfully
    with all expected tests run. A copied body line alone cannot satisfy either run.
- id: NFR-006-AC-15
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: A Kani-touching pull request whose required lane did not run or failed
    remains unmerged and does not claim real-Kani verification; a written explanation
    does not waive the gate.
- id: FR-015-AC-120
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'PLANNED (IR-942; depends on QSL ADR-013 O-07/O-09 and QSV FR-123). After
    the selected owner and occurrence validate under AC-122, CG forms the per-step
    record content identity from an RFC 8785 object whose only members are `subject_kind:
    "protocol-refinement-step"` and `record: StepSimulationRecord` encoded by QSV.
    Two separately valid selected refinements with the same obligation and hypotheses
    but different record `refinement` values, each with its own matching occurrence,
    yield different content identities; a record with a mismatched owner has no identity
    to compare. Within one admitted ref'
- id: FR-015-AC-121
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-942). A per-step `KaniObligationIdentity` uses the QSV-owned
    `StepSimulationRecord` as a typed subject, carries its content identity and the
    owning refinement's O-07 `generated` occurrence key separately, and keeps any
    CG proof or obligation kind in the harness record rather than the record content
    preimage. CG publishes a persisted harness-record digest over every `KaniObligationIdentity`
    member except `source_span`; changing any included member changes this digest,
    while changing only `source_span` does not. This digest is distinct from the per-step
    record content identity. The r
- id: FR-015-AC-122
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'PLANNED (IR-942). A selected proof-eligible refinement under QSpec FR-322-AC-88
    through AC-90 with an `Initial` record and `Step { row: i }` records for its admitted
    FR-375 `steps` order yields generated occurrences on the owner node at ordinals
    0 and `i + 1`, respectively. An ineligible refinement under FR-322-AC-91 (hidden
    field, history field, or `any` step target) carries only its `declaration` occurrence
    and yields no per-step record, harness, or proof credit; the IR reader refuses
    a generated occurrence on it as `invalid_package`/`invalid-value` before CG setup.
    Two step rows tied on nod'
bindings: []
```

## Dispositions

Round1 at a34058084047a0f92c3eb07cc394f67a97f76e01: FND001 FIXED. Current narrow base disposition PASS, conditional only on latest custody and the two still-PENDING required final gates. Original finding and initial verdict remain unchanged. No new finding in the one-line formatting fix.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Verified a3405808 removes only the blank separator; all three criterion row bytes equal actual base942ece51. Fresh actual current matrix now mints all3IDs:883rows,521tagged/294untagged/18ignored-only/50method-without-symbol. This fixes table extraction, not planned implementations. Source/test/manifest/lock and external report bytes remain unchanged. |

```yaml
dispositions:
- fnd: FND-001
  outcome: fixed
  after_excerpt: 'ield range path and makes no claim that the separate IR-624 state-field
    path is implemented. | Test (TC-025) |

    | FR-015-AC-120 | PLANNED (IR-942; depends on QSL ADR-013 O-07/O-09 and QSV FR-123).
    After the selected owner and occurrence validate under AC-122, CG forms the per-step
    record content identity from an RFC 8785 object whose only members are `subject_kind:
    "protocol-refinement-step"` and `record: StepSimulationRecord` encoded by QSV.
    Two separately valid selected refinements with the same obligation and hypotheses
    but different record `refinement` values, e'
```
