---
id: SR-17940486709564847047578
title: "spec-review/integrity of IR-1438 PR 764"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-049-enum-scalar-harnesses.md, spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/functional/FR-025-generated-subject-abi.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/matrix/tests.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-049
    type: reviews
---

## Summary

Ticket: IR-1438. PR #764 at dec7b1331330face200854102bcb755ce11f9aad. Integrity review of the new
FR-049/TC-063 pair against the one-line shared amendments in FR-015, FR-025, FR-028, `spec/spec.md`
and `spec/kani/matrix/tests.md`, compared with how FR-046 (IR-258) planned its Rational row.
FR-049 and TC-063 ids are unique on origin/main and in open PRs; `quire validate` passes both
files with two EARS warnings (see the EARS review). The `tests.md` and `spec.md` rows are
consistent with FR-049's PLANNED status and claim no credit.

## Verdict

**FAIL.** The amendments do not match the FR-046 pattern: FR-046 added a real row to FR-025's
symbolic argument table (plus planned playback AC-10); FR-049 claims its enum row is "planned there"
but adds no row, and its FR-015/FR-028 parentheticals point the composite-shadow enum leaf at FR-049
while FR-049-AC-9 keeps that leaf at its composite disposition. Three medium findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-049 Dependencies says FR-025 plans the enum index row "as a single unsigned binding", but the diff adds no row to FR-025's Family/Symbolic argument type table, whose rule "If an argument's family has no row in that table, then the generator shall account the obligation unsupported" still governs enum; unlike FR-046 (table row + FR-025-AC-10 lossless playback decode) there is no index width or playback decode/width check for the unsigned index. Only FR-025-AC-7 is amended. | spec/kani/functional/FR-049-enum-scalar-harnesses.md:110-111; spec/kani/functional/FR-025-generated-subject-abi.md:66-74 |
| FND-002 | medium | FR-049 Description says the row replaces the `unsupported`/`ShadowFamilyNotBuilt` disposition for the enum leaf family; that reason is the composite-equality shadow refusal of FR-015, while a top-level enum comparison claim today settles `Unsupported(OperationNotRendered)` (`src/kani/generate/negotiate.rs:1393-1397`), and FR-049-AC-9 keeps an enum inside a composite at its composite disposition. The disposition FR-049 actually replaces is never named, and no OperationNotRendered statement is amended. | spec/kani/functional/FR-049-enum-scalar-harnesses.md:31-32 |
| FND-003 | medium | The FR-015 composite-equality shadow table row now says "(an enum comparison leaf is instead planned under FR-049)", and FR-028's checklist says "enum (comparison leaves planned under FR-049)"; both lists are about shadow leaves/members, which FR-049-AC-9 explicitly leaves at their composite disposition, so the amendment contradicts FR-049. | spec/kani/functional/FR-015-bounded-kani-obligations.md:764; spec/kani/functional/FR-028-bounded-proof-ceilings.md:358 |

## Examined scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-049 | examined | This row replaces the `unsupported`/`ShadowFamilyNotBuilt` disposition for the enum leaf family only; every other family keeps that disposition. |
| FR-049-AC-9 | examined | A leaf or member of any other family remains `unsupported`/`ShadowFamilyNotBuilt` and an enum node inside an unsupported composite keeps its composite disposition. |
| FR-015 shadow table row | examined | a leaf or member of family text, enum, rational, ... (an enum comparison leaf is instead planned under FR-049) |
| FR-025-AC-7 | examined | An argument with no applicable row (decimal, IEEE, text, enum outside the planned FR-049 row, composite, collection, function, or Rational outside the planned FR-046 row) is accounted `unsupported`. |
| FR-028 checklist | examined | text, enum (comparison leaves planned under FR-049), rational, decimal, quantity, float, sequence, set, bag and ordered set |
| tests.md FR-049 row | examined | FR-049-AC-1 through FR-049-AC-10, TC-063, PLANNED (IR-1438) |
| spec.md index row | examined | Enum scalar comparison harnesses, FR-049 planned enum comparison proof row, TC-063 planned |
| TC-063 | examined | Verifies FR-049; status PLANNED, all ten criteria unimplemented and untagged. |
| FR-025 argument table | context_only | Boolean, Bounded integer, Bounded Rational (planned FR-046 predicate slice only) rows |
| FR-046 / FR-025-AC-10 | context_only | Every supported FR-046 Rational argument has exactly two ordered I64 physical bindings ... Actual playback decodes losslessly |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | The fix adds FR-025-AC-14 with Verification `Test (TC-036)` and adds it to TC-036's matrix row, but TC-036's procedure (spec/kani/matrix/TC-036) has no step for the enum `i64` binding or its playback decode; FR-025-AC-10 to AC-13 each have a numbered step there. AC-14 is mapped to a test case that does not exercise it. | spec/kani/functional/FR-025-generated-subject-abi.md:185; spec/kani/matrix/tests.md:129 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Fix 1479c253 adds the FR-025 table row "Enum operand (planned FR-049 comparison row only)" with one `i64` call, plus FR-025-AC-14 and an FR-049 Symbolic ABI section and AC-10 for playback decode. TC-036 procedure gap recorded separately as FND-004. |
| FND-002 | fixed | Description now names `unsupported`/`OperationNotRendered` as the replaced disposition and leaves the FR-015 `ShadowFamilyNotBuilt` row unchanged for composite enum leaves. |
| FND-003 | fixed | FR-015 and FR-028 parentheticals now say only a standalone enum comparison node moves to FR-049 and an enum leaf inside a composite keeps the shadow row, matching FR-049-AC-9. |
| FND-004 | fixed | Fix 436629e7 adds TC-036 procedure step 14 and expected result 14 for the FR-049 enum `i64` binding and playback decode, citing FR-025-AC-14. |
