---
id: SR-14862
title: spec-review/base current-cohort review of IR615 CG
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen; IR615 current main-to-candidate eight-file
  diff; whole repository matrix
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-015
  type: references
- target: ix://agent-ix/quire-contract-codegen/NFR-006
  type: references
---

## Summary

Ticket: IR-615; CG PR485. Fresh complete review of current main 8ac09bc85f2a34b7b91d36bd59ce8e92e6ced59d through current candidate 2f24b78989f550f3e68e31323cf33782a96caf5d, all eight changed files, including historical review custody. Old SR14500/14501/14502 are historical and supply no fresh acceptance. Reviewer GPT-6 (deployment SKU unavailable; marker gpt-6). No applicable AssuranceProfile exists. No product edit or Cargo/fmt/metadata/rustc/native/Kani/CBMC/goto/aggregate command was run. Scope excerpts longer than600characters are verbatim prefixes.

## Verdict

PASS — status-only base review; mandatory real-Kani and aggregate gates remain pending.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Status-only FR015 and TC023 edits retain exact normative criteria IDs/statements/methods, requirement statements, relationships and domain objects. Current full producer/consumer implementation exists and native reader/family controls passed, so the implemented cleanup description is accurate. No status change purports to implement full Unsupported negotiation or retire OutcomeConstruction; both remain separate work. TC023 correctly identifies a recorded report and explicitly says no backend invocation occurred as part of cleanup. Current mandatory gate is separately pending; this status paragraph is not a gate waiver.

Base checklist was applied to both changed status paragraphs; no new requirement statement, object, relationship, option, boundary or criterion was added, so EARS/integrity/domain/dependency specialist analyses are not applicable. Current AC86/88 evidence insufficiency is captured by code-review, not hidden by a structural matrix tagged result. Historical SRs are reports only. No new normative hash/pin/catalog/identity inventory is introduced. IDs and full changed documents are structurally validated.

Independently read current resume/focused.log: actual asserted memory.max17179869184/swap0, fmt-check and locked all-target Clippy PASS,9profile/family+4syntheticclassification+1recordedzero-SUCCESS+2terminal=16actual native tests PASS,0failed/ignored, peak4,935,614,464bytes/events0. Those passing partial controls do not resolve the criterion gaps. Recorded-real-Kani capture classification is not a live backend proof. NFR006 explicitly applies to src/kani and Cargo.lock; required full make kani-gate is PENDING on the final clean/stable candidate and cannot be waived by these reviews or native logs. Root aggregate is also PENDING. No merge-ready or backend-verified conclusion is given.

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
bindings: []
```
