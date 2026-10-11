---
id: "SR-7686"
title: "IR-268 spec-review/evidence review of symbolic collection and graph obligations"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen@8f10b7440fd63cfd6b17719605bb99540c1eea78; spec/core/functional/interface-001-codegen-api.md, spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

# SR-7686: IR-268 spec-review/evidence review

## Summary

Ticket: IR-268. PR: agent-ix/quire-contract-codegen#353 at 8f10b7440fd63cfd6b17719605bb99540c1eea78.
Method: spec-review/evidence. Examined FR-015-AC-96 through AC-102, TC-025, the matrix and public API edit.

## Scope

- FR-015 (examined): The admitted V2 interfaces do not yet expose a typed finite model-object population and per-reference-field edge multiplicity sufficient to bound every symbolic graph input.
- FR-015-AC-96 (examined): A collection claim over a selected model field whose typed collection member has inclusive cardinality `[minimum, maximum]` and whose element domain is finite generates a harness with symbolic length and symbolic elements; its assumptions admit both cardinality endpoints, both element endpoints and every value between them, preserve the collection's declared order and uniqueness rules, and use no caller-supplied `max_items` as a semantic bound. PLANNED (IR-268).
- FR-015-AC-97 (examined): A collection field with no authored finite maximum, an element with no finite representable domain, or a V2 `RequiresBound` lowering record settles `requires_bound` naming the field or unbounded type and emits no harness; a collection form or query whose semantics this generator cannot encode settles `unsupported` naming that capability and emits no harness. Neither disposition substitutes a finite ceiling. PLANNED (IR-268).
- FR-015-AC-98 (examined): A graph claim generates a harness only when admitted typed Contract IR inputs identify finite model-object populations for its start and target types and a finite edge multiplicity for its selected reference field; every symbolic object and edge is bounded inclusively by those declarations, including zero and maximum where admitted, and reachability follows only that field. Missing or non-finite population or multiplicity settles `requires_bound` when the typed input identifies an unbounded domain, otherwise `unsupported` naming the unavailable graph-domain capability; no harness or substitute
- FR-015-AC-99 (examined): The generated collection and graph harnesses assert the claim over their symbolic model-admitted inputs, end with the FR-015-AC-7 cover after their assertions, and never replace symbolic inputs with the current finite-input corpus's concrete `Vec<i128>` or object IDs. A healthy QSL-emitted admitted V2 case of each supported family yields an installed-Kani `Verified` result with its cover satisfied; a seeded claim or subject mutant yields `Falsified` with a concrete in-domain playback. A generation-time Boolean or `KaniOutcome::Proved` is not proof evidence. PLANNED (IR-268).
- FR-015-AC-100 (examined): For either family, changing an authored cardinality, element bound, object population, edge multiplicity, claim, customer subject or unwind changes the harness identity; equal admitted inputs regenerate byte-identically. The identity records every symbolic domain and the solver and complete ordered option vector, and FR-017 run evidence records the actual Kani, Rust and solver versions, unwind and every option used by the installed backend. PLANNED (IR-268).
- FR-015-AC-101 (examined): A request whose symbolic collection or graph domain exceeds the generator's representational or resource ceiling settles `unsupported` with a typed reason naming the failing domain or ceiling and no harness; it never tightens the model domain to make a proof tractable. A missing or malformed selected declaration likewise refuses with its typed reason and no harness. PLANNED (IR-268).
- FR-015-AC-102 (examined): The symbolic collection and graph items are settled through the FR-015 V2 obligation path, one disposition per requested item; supported items alone emit harnesses. The finite-input lowerers and hand-built corpus of AC-86 to AC-95 neither supply these items' bounds nor classify their proof verdicts. PLANNED (IR-268).
- TC-025 (examined): For IR-268, emit a collection claim and a reference-graph claim through QSL, admit each as `CheckedPackageV2`, and resolve its selected model declaration through Contract IR's typed interfaces.
- interface-001-codegen-api (examined): planned IR-268 collection and graph items take domains only from admitted V2 model declarations and typed IR bounds
- tests-matrix (examined): FR-015-AC-96 through FR-015-AC-102 | TC-025 | 🚧 Planned (IR-268)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

No findings in this method on the four-file diff. The new criteria remain explicitly planned; this review claims no implementation or Kani run.
