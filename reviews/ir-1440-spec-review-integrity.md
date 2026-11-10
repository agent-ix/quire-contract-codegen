---
id: SR-17940486709564847047581
title: "IR-1440 PR #766 FR-051 integrity analysis"
type: SpecReview
analysis: integrity
date: 2026-11-10
scope: "agent-ix/quire-contract-codegen; PR #766 @ 6c9302b6; IR-1440; spec/kani/functional/FR-051-decimal-scalar-harnesses.md, spec/kani/matrix/TC-065-decimal-scalar-harnesses.md, spec/kani/functional/FR-025-generated-subject-abi.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/matrix/TC-036-generated-subject-abi.md, spec/routed/matrix/TC-030-capability-settlement.md, spec/kani/matrix/tests.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-051
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-065
    type: reviews
---

## Summary

Ticket: IR-1440. Spec-only PR #766 (head 6c9302b6) adding FR-051 and TC-065, the planned Kani row for decimal scalar operations, with edits to FR-025, FR-028, TC-036, TC-030, spec/kani/matrix/tests.md and spec/spec.md. Reviewed against origin/main 66e7eac3 (includes #764 FR-049 and #765 FR-050), quire-exact decimal.rs/numeric.rs/outcome.rs/accounting.rs, CG src/oracle/scalar/derive.rs and src/kani, qsl-replay WitnessValue, and Contract IR FR-038.

## Verdict

CHANGES REQUIRED. The FR-025 ABI table row, tests.md FR-051 row, spec.md row and TC-030 Decimal row are well-placed and auto-merge with main. FR-025-AC-15 is not in the acceptance-criteria table, and the branch no longer merges cleanly with main after #765.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Half-applied edit: the FR-025-AC-15 row was appended at end of file, after the '## Dependencies' bullet list, not inside the '## Acceptance Criteria' table that ends at FR-025-AC-14 (line 186). quire parse files it as Dependencies section content and `quire matrix` mints no FR-025-AC-15 criterion (it lists FR-025-AC-14 and FR-051-AC-1..10 only), so TC-036 step 15, TC-036 Expected Result 15 and the tests.md TC-036 row all cite a criterion that does not exist. make spec still passes because nothing resolves the citation. Move the row into the table after AC-14. | spec/kani/functional/FR-025-generated-subject-abi.md:194 |
| FND-002 | high | The branch (based on d1c81837, before #765) does not merge with origin/main 66e7eac3: `git merge-tree` and GitHub (mergeable=CONFLICTING) report content conflicts in FR-025 (the AC-7 line, which #765 changed to add 'quantity outside the planned FR-050 row') and FR-028 (the ShadowFamilyNotBuilt bullet, which #765 changed to end '(quantity scalar operations: planned FR-050)'). The FR-025 ABI table rows, tests.md, spec.md and TC-030 auto-merge and coexist one row per family. The rebase must keep both carve-outs on each conflicting line: AC-7 naming 'decimal outside the planned FR-051 row' and 'quantity outside the planned FR-050 row', and the FR-028 bullet naming both FR-051 and FR-050. | spec/kani/functional/FR-025-generated-subject-abi.md:179 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-025-AC-15 | examined | spec/kani/functional/FR-025-generated-subject-abi.md:194 | PLANNED (IR-1440). Every supported FR-051 decimal operand has exactly two ordered `i64` physical bindings (coefficient, scale) grouped to its original parameter, `decimal_range` domain and retained representation. Playback decodes with the existing I64 width/comment checks and entry and byte caps, validates the scale and `DecimalType::contains`, then submits one unnormalized `WitnessValue::Decimal` per parameter. Missing, duplicate, foreign, role-swapped or domain-conflicting schema, malformed entries and out-of-domain pairs refuse with a typed reason and no replay. Existing Boolean/integer... |
| FR-025-AC-7 | examined | spec/kani/functional/FR-025-generated-subject-abi.md:179 | An argument with no applicable row (decimal outside the planned FR-051 row, IEEE, text, enum outside the planned FR-049 row, composite, collection, function, or Rational outside the planned FR-046 row) is accounted `unsupported` with a typed reason naming its family/capability, and no harness is emitted. The planned Rational row does not authorize a harness without its lossless decoding and public replay prerequisites. |
| FR-025-decimal-row | examined | spec/kani/functional/FR-025-generated-subject-abi.md:74 | / Bounded Decimal (planned [FR-051](./FR-051-decimal-scalar-harnesses.md) scalar operations only) / two `i64` calls: coefficient then scale, inside the original `decimal_range` intervals and constrained by owner `DecimalType::contains`; the pair is the retained representation / one owner `Decimal::new(Integer::from(c), s)` value for the original parameter / |
| FR-028-shadow-bullet | examined | spec/kani/functional/FR-028-bounded-proof-ceilings.md:357 | - A shadow or a production harness for each leaf and member family `ShadowFamilyNotBuilt` names:   text, enum (a standalone comparison node is planned under FR-049; an enum leaf inside a composite stays here), rational, decimal (leaves only; scalar operations are planned under FR-051), quantity, float, sequence, set, bag and ordered set; the scalar   families are their own rows and PRs (decimal scalar operations: planned FR-051). |
| TC-036-step-15 | examined | spec/kani/matrix/TC-036-generated-subject-abi.md:73 | 15. For the planned [FR-051](../functional/FR-051-decimal-scalar-harnesses.md) decimal row, generate a     decimal operation and read its persisted identity: assert exactly two `i64` bindings,     coefficient then scale, grouped to the original parameter with its `decimal_range` domain.     Decode retained playback with the existing I64 checks and caps; assert a valid pair submits one     unnormalized `WitnessValue::Decimal`, and a missing, duplicate, foreign, role-swapped, negative     scale, out-of-domain or over-limit entry refuses with a typed reason and no replay. |
| TC-036-expected-15 | examined | spec/kani/matrix/TC-036-generated-subject-abi.md:121 | 15. One decimal operand has two grouped `i64` bindings (coefficient, scale) with its domain; a valid     pair replays as one unnormalized Decimal, and each malformed or out-of-domain case refuses with     no replay (FR-025-AC-15). |
| tests.md-TC-036-row | examined | spec/kani/matrix/tests.md:130 | / TC-036 / Verify the generated harness subject ABI / Integration / P0 / FR-025-AC-1, FR-025-AC-2, FR-025-AC-3, FR-025-AC-4, FR-025-AC-5, FR-025-AC-6, FR-025-AC-7, FR-025-AC-8, FR-025-AC-9, FR-025-AC-14, FR-025-AC-15 / 🚧 Planned / |
| tests.md-FR-051-row | examined | spec/kani/matrix/tests.md:13 | / FR-051 / FR-051-AC-1 through FR-051-AC-10 / TC-065 / 🚧 PLANNED (IR-1440): no decimal scalar Kani harness; needs the quire-exact decimal carrier (IR-1444); TC-065 controls have not run. / |
| spec.md-FR-051-row | examined | spec/spec.md:92 | / Decimal scalar operation harnesses / [FR-051](kani/functional/FR-051-decimal-scalar-harnesses.md) planned decimal arithmetic, rounding and ordering proof row / [TC-065](kani/matrix/TC-065-decimal-scalar-harnesses.md) planned proof and mutant controls / |
| TC-030-decimal-row | examined | spec/routed/matrix/TC-030-capability-settlement.md:397 | / Decimal scalar `expression` claim, IR-confirmed generated claim / Planned (IR-269) / Decimal scalar operation forms; the arithmetic, rounding and ordering operations are planned under FR-051 (conditional on the quire-exact decimal carrier, IR-1444), which supersedes this row for them only when delivered. / `OperationNotRendered` (`classify_claim` maps `ScalarLoweringRefusal::NoRenderer`). / |
