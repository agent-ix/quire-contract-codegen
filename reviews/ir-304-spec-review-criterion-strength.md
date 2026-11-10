---
id: SR-17940486709564847047597
title: "IR-304 PR #768 FR-022 per-occurrence requirement records: spec-review criterion-strength"
type: SpecReview
analysis: criterion-strength
date: 2026-11-10
scope: "agent-ix/quire-contract-codegen; PR #768 @ de3d7ee1; IR-304; spec/routed/functional/FR-022-routed-generation.md, spec/routed/matrix/TC-033-routed-generation.md, spec/routed/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-033
    type: reviews
---

## Summary

Ticket: IR-304. Spec-only PR #768 (head de3d7ee1, base origin/main cf10ec78) rewrites FR-022's QSL requirement-record intake, adds the 'Per-occurrence requirement records' and 'Path conditions in the scalar harness' subsections, rewrites AC-26/27/28, adds AC-31..35 and an IR-304 Rationale, and updates the TC-033 procedure for slices (a)/(b) and the routed tests.md row. Verified against CG src/routed/generate.rs, src/kani/generate/{spec,scalar}.rs, src/kani/identity.rs, src/replay/scalar.rs, src/core/contract_package.rs, src/oracle/scalar/lower.rs; IR quire-contract-model c8dca8c (ContractOccurrenceKeyV1, result_context); QSL origin/main ffec6c72 (qsl-semantics check/claims.rs PathGuard/RequirementRecord/key_claims, qsl-route request.rs RequirementItem).

## Verdict

Jev (System One) client is not available on this host (`which jev` finds nothing), so this was done manually. PASS except one low finding. AC-26's positive half already holds before IR-304 (AC-23), but its swap half fails any node-id or slice-position pairing. AC-28 keeps the sibling record present, so a node-id fallback generates instead of refusing, and each single change reaches its named variant in the stated per-item order. AC-27's path-condition exchange and AC-33/AC-34/AC-35 each have a discriminating control (holds=false [0, 5]; the unguarded result_bound_unreachable control; `x > y` Verified). AC-31's byte-preservation half has no named golden.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-31's byte-preservation clause ('its record omits the field and keeps its existing bytes') is tested by TC-033 against 'the pre-IR-304 record', and no committed golden fixture is named. A test that builds both records with the new code compares the implementation with itself and cannot fail. Serializing `requirement_occurrence: null` or `path_condition: []` would also change bytes without failing that test. Fix: name the committed golden record (or an existing fixture) that the direct-path record must equal, and assert the absent keys. | spec/routed/functional/FR-022-routed-generation.md:587; spec/routed/matrix/TC-033-routed-generation.md:93-94 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-022-AC-26 | examined | spec/routed/functional/FR-022-routed-generation.md | Given the checked QSL RR-16 package with two `x + 1` occurrences of one application node, directly narrowed to `Int[0, 10]` and `Int[0, 20]`, each routed item carries the `RoutedRequirement` projected from its QSL `RequirementItem`, and `KaniGenerationContext.requirement_records` holds both records. Both items are `Supported` and each harness asserts its own result bound (`[0, 10]` at the first occurrence's request index, `[0, 20]` at the second); reversing the routed-item slice leaves every per-request-index result unchanged. Exchanging only the two items' `RoutedRequirement` values refuses t |
| FR-022-AC-27 | examined | spec/routed/functional/FR-022-routed-generation.md | For QSL-emitted `if p then x + 1 else x + 1` over `p: Boolean` and `x: Int[0, 9]`, the two `x + 1` occurrences of one application node have path conditions `[(p, holds=true)]` and `[(p, holds=false)]`. Each harness draws one Boolean guard input for `p` and contains exactly one `Statement::Assume` of it before the oracle call: `p` for the then item and `!p` for the else item. Each identity's `path_condition` and `guard_inputs` list that guard and input. A nested path `[(p, holds=true), (q, holds=true)]` assumes `p` before `q`. Exchanging the two records' path conditions together with the items' |
| FR-022-AC-28 | examined | spec/routed/functional/FR-022-routed-generation.md | Starting from the valid FR-022-AC-26 input, each single change below refuses the whole call as `RequirementRecord` naming the changed item's request index and occurrence, with no `RoutedGeneration`, oracle artifact, claim map or harness: removing the item's `RoutedRequirement` (`RequirementMissing`, `occurrence: None`); removing its record from `requirement_records` (`RecordMissing`); changing the item's occurrence ordinal to one with a record but not the authenticated application (`ApplicationMismatch`); replacing the item's or the record's result bound with the other occurrence's (`ResultBou |
| FR-022-AC-31 | examined | spec/routed/functional/FR-022-routed-generation.md | Each routed scalar harness of FR-022-AC-26 has `ScalarObligationIdentity.requirement_occurrence` equal to its item's `RequirementOccurrence` node, role and ordinal, and the two identities and persisted records differ in that field. The record serializes the occurrence as those three fields, not a hash, digest or text rendering. A harness generated through direct `negotiate_kani_obligations` has `requirement_occurrence: None`, and its record omits the field and keeps its existing bytes. |
| FR-022-AC-32 | examined | spec/routed/functional/FR-022-routed-generation.md | A routed item whose guard is a lowerable form (a Boolean parameter reference, or an integer `lt`/`le`/`gt`/`ge`/`eq`/`ne` comparison between bounded-integer parameter references or such a reference and an `i64` literal) generates a harness with that guard's `Statement::Assume`. A guard of any other form, such as `x + 1 > 5`, settles the item `unsupported` as `PathGuardNotLowered` naming the guard occurrence with reason `UnsupportedGuardForm`; a comparison reading a plain-Integer parameter settles `PathGuardNotLowered` with `UnboundedGuardOperand` naming that parameter; neither has a harness, a |
| FR-022-AC-33 | examined | spec/routed/functional/FR-022-routed-generation.md | For QSL-emitted `if x > 5 then x + 100 else 0` over `x: Int[0, 9]` into `Int[0, 200]`, the guarded `x + 100` item is `Supported` with arguments `[6, 9]` and `[100, 100]`, a `Statement::Assume` of `x > 5`, and a result assertion against `[0, 200]`. The same item with the guard's `holds=false` has arguments `[0, 5]` and `[100, 100]`. For `if x < y then x + 1 else 0` over `x: Int[0, 9]` and `y: Int[0, 9]` into `Int[0, 10]`, a comparison between two parameters, the argument `x` keeps `[0, 9]`, `y` is a guard input over `[0, 9]`, and the harness assumes `x < y`. |
| FR-022-AC-34 | examined | spec/routed/functional/FR-022-routed-generation.md | For QSL-emitted `f(x: Int[0, 9]): Int[0, 10] { if x > 20 then x + 100 else 0 }`, the guarded `x + 100` item settles `unsupported` as `PathConditionUnsatisfiable` naming the `x > 20` guard occurrence, the parameter `x` and its declared range `[0, 9]`, with no harness; it is not `result_bound_unreachable`. The same `x + 100` into `Int[0, 10]` with an empty path condition still settles `result_bound_unreachable` with reachable `[100, 109]` (FR-015-AC-18). |
| FR-022-AC-35 | examined | spec/routed/functional/FR-022-routed-generation.md | For a guarded item whose guards are jointly infeasible but not emptied by narrowing (`if x < y then x + 1 else 0` with `x: Int[5, 9]` and `y: Int[0, 4]` into `Int[0, 10]`), CG emits a harness whose `completed` cover follows the guard assumptions, and FR-017 execution under installed Kani reports `CoverUnsatisfied`, not `Verified`. The same harness with the guard `x > y` reports `Verified`. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | AC-38 cannot be built as written. Its discriminating clause ('while the unguarded sibling of FR-022-AC-26 stays Supported', TC-033 'beside the unguarded RR-16 item') puts items from three different checked packages in one generate_routed call: the RR-16 package, the AC-27 `if p` package and the AC-33 `if x > 5` package. generate_routed takes one CheckedPackageV2 and one admitted ContractPackageV1 per call, and the AC-27 package has no unguarded scalar item. As written, the case that separates a per-item interim refusal from a whole-call refusal cannot be constructed. Fix: name one QSL package that holds a guarded item and an unguarded sibling (for example an AC-33 package with an added unguarded `x + 1` function), or drop the sibling claim. | spec/routed/functional/FR-022-routed-generation.md (AC-38 row); spec/routed/matrix/TC-033-routed-generation.md:100-101 |


## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | AC-37's slice (b) two-guard case cannot be built from the package it names. It routes, 'in the IR-304 guarded package', an `x + 100` item whose record carries `[(x > 20, holds=true), (x + 1 > 5, holds=true)]`. No function of that package (f, g, h, k, c) contains `x > 20` or `x + 1 > 5`; h has only `x > 5`. In slice (b), GuardContextMissing is a whole-call check that runs before per-item settlement, and it needs each guard node as a ContextRoot in the item's span. The guard nodes must therefore exist in the checked package, with expression occurrences QSL emitted. As written, the case that tests precedence step 3 before step 5 refuses the whole call or cannot be assembled. Fix: add a function such as `j(x: Int[0, 9]): Int[0, 200] { if x > 20 then (if x + 1 > 5 then x + 100 else 0) else 0 }` to the guarded package and cite its item. | spec/routed/functional/FR-022-routed-generation.md:310-316 (guarded package), AC-37 row; spec/routed/matrix/TC-033-routed-generation.md:98-106 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 at d7760170. Verified: AC-31 names the committed golden tests/fixtures/kani-scalar/direct-increment-record.json, which slice (a) captures from the unchanged generator. Byte equality also fails a `null` or `[]` serialization. The fixture does not exist yet and is a slice (a) deliverable, as stated. |
| FND-002 | fixed | Round 2 at fc1988f8. Verified: AC-38 and TC-033 step 31 use one IR-304 guarded package (f, g, h, k, c) in one generate_routed call, and g is an unguarded sibling. The functions type-check as written: x + 1 into Int[0, 10], x + 100 into Int[0, 200], x + m into Integer, and n >= 0 into Boolean. |
| FND-003 | fixed | Round 3 at c0827cbb. Verified: function j `if x > 20 then (if x + 1 > 5 then x + 100 else 0) else 0` puts both guard nodes, the gt applications that are QSL's outermost guard occurrences, in the checked package as Expression nodes the routed profile lowers. They can be ContextRoots in the item's span, so GuardContextMissing passes. Guard x > 20 is a lowerable form. Guard x + 1 > 5 has an application operand, so it is UnsupportedGuardForm, and precedence step 3 (path_guard_not_lowered) is reported ahead of step 5 (path_condition_unsatisfiable), which narrowing x > 20 alone would give. A narrow-first implementation would fail the case. AC-38 is unchanged (j is not routed there). TC-033 precondition, procedure and expected result 30 cite j. |
