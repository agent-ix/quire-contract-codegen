---
id: SR-13306
title: "IR-899 PR 461 gap analysis: first-party lock cohort refresh"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@d40e60dbca7cfcf2328462684e483f8ace134e75 vs origin/main merge-base 58f447801773268b0178ef45c44fb0fc68dffbba; Cargo.lock, tests/it/kani_obligations_symbolic_domains.rs, spec/kani/functional/FR-015-bounded-kani-obligations.md"
review_set: subset
---

## Summary

Ticket: IR-899. PR agent-ix/quire-contract-codegen#461. This is a planless gap analysis. Plan completion: not assessed. The PR changes no `src/` file and no spec file. Its only test change is the profile line of the shared fixture helper `source_unit_with_body`, which feeds the FR-015 collection tests.

`quoin matrix --json` (quoin 0.28.3) at the head still binds the same tests to FR-015-AC-96, -97, -99, -100, -101 and -102 as on main; the PR adds and removes no trace tag. The reviewer ran those 20 tests at the head and all pass (the real-Kani test stays ignored). FR-015-AC-98 has no binder, which is unchanged from main and matches its PLANNED status (IR-805), so it is outside this diff.

The FR-015 prose cites test line numbers (for example `:705`) that are now one line low, because the PR adds a one-line comment above the helper. Each citation also names the test function, which still resolves, so the numbers are informational and are not recorded as a finding.

## Examined criteria

- FR-015-AC-96 (examined): "An admitted QSL V2 clause/function body that contains a `quire.op.collection.forall` or `quire.op.collection.exists` operation over a selected model collection field is a collection claim. For a field with typed inclusive cardinality `[minimum, maximum]` and supported finite elements, the generator emits a harness with symbolic length and symbolic elements."
- FR-015-AC-97 (examined): "For an admitted collection claim whose selected field has multiplicity lower bound `0` and no finite upper bound, QSL admits an unbounded collection type; when its element type is supported, this generator settles `requires_bound` naming the selected field or unbounded collection type and emits no harness."
- FR-015-AC-99 (examined): "The collection harness asserts the admitted clause/function body over symbolic inputs drawn from the selected model field's typed domain ... The independent native oracle replays the same authored function through `qsl_replay::replay`."
- FR-015-AC-100 (examined): "For the collection family, changing the selected field's authored cardinality or element bound, admitted body claim, customer subject or unwind changes the harness identity; equal admitted inputs regenerate byte-identically."
- FR-015-AC-101 (examined): "A request whose symbolic collection domain exceeds the generator's representational or resource ceiling settles `unsupported` with a typed reason naming the failing domain or ceiling and no harness."
- FR-015-AC-102 (examined): "An admitted collection body claim is settled through the FR-015 V2 obligation path, one disposition per requested item; supported items alone emit harnesses."
- FR-015-AC-98 (context_only): graph claim, PLANNED (IR-805); no binder before or after the PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

**PASS.** The lock move changes no requirement-owning code, and the fixture adaptation keeps every FR-015 collection binding and assertion intact. Coverage is the same as on main.

## Dispositions

Disposition pass 1, against head 02bb0b01963bd640e0ab8fcb6fa4cd514809f783. No finding to dispose of (clean review). The fix round changes only `Cargo.lock` revisions and adds the review files; `quoin matrix` bindings for FR-015-AC-96..102 are unchanged, and the 20 symbolic collection tests pass at the new head.
