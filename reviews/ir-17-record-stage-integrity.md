---
id: SR-15252
title: "IR-17 record stage integrity review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; IR-17; spec/oracle/functional/FR-018-composite-equality-oracles.md; spec/oracle/matrix/TC-029-composite-equality-oracles.md"
review_set: subset
---

## Summary

Independent PR496 review of record admission/parser stage allocation. Exact reviewed head 7ab13b096fc004ed6c5eabab41440614791b8608; exact baseline bc4aea2bdc4635cd6fb76e5a7cf9adb9ac528301. This is specification review, not product qualification.

## Verdict

**PASS** — Disposition round 1 at 6a7d1eb643918110f48e51810b283a26844e70a8. The public-cause defect is fixed at the frozen disposition head. code() remains Code::AmbiguousDeclaration; cause() is explicitly None for Assembly. No new defect found; all independent source/context/private defense and no-charge constraints remain intact.

## Examined scope

scope:
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A declaration closure `TypeEnvironment::new` refuses is refused at generation
    time carrying that `DeclarationCause`, including both recursion passes. The authentic
    duplicate-field source control instead retains typed AdmittedRefusal::Package
    with the owning CompileRefusal code/cause (ambiguous_declaration/ambiguous-name),
    produces no successful AdmittedContext and is not passed to public CG generation.
    Private CG declaration-admission defenses retain the exact DuplicateMember name/cause
    and unchanged healthy sibling separately, without public-acceptance credit; and
    each generated ora
- id: FR-018-AC-26
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Reading the QSpec List `next` field as `binding(next, aggregate([binding(optional,
    reference Option<List>)]))` yields exactly one `FieldDeclaration` named `next`,
    with `Presence::Optional` and payload type `ValueType::Composite(List NodeKey)`
    on the genuine public admitted List path. At the separately labelled private parser/type-resolution
    seam, removing or renaming `optional`, adding a second nested member, or targeting
    a non-option node retains exact MalformedComposite, no symbol, no required-field
    reinterpretation and an unchanged healthy Tree sibling. These inconsistent internal
- id: FR-018
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A duplicate-field source control is rejected by the owning QSL declaration
    admission before a genuine context exists; it is not a public CG malformed-declaration
    input.
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: Use the root facade's public refusal accessors; do not add a dependency
    on QSL assembly internals.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-8 and TC-029 require ambiguous-name from the public CompileRefusal cause accessor, but the selected owning QSL accessor explicitly returns None for Assembly refusals; only code() exposes AmbiguousDeclaration. Retain the actual public cause absence and precise private DuplicateMember instead of requiring an inaccessible public cause. | spec/oracle/functional/FR-018-composite-equality-oracles.md:591 |

## Evidence and coverage

The selected QSL source was independently read: qsl-semantics/src/check/assemble.rs:499-500 and 589-590 map underlying DuplicateMember to AmbiguousDeclaration/ambiguous-name; qsl-replay/src/spine.rs:235-239 exposes its code, while :255-272 explicitly returns None for Assembly in cause(). qsl-replay/src/admitted.rs:404-427 creates context only after successful recompile; :380-390 classifies this non-resource refusal as Package. The underlying cause mapping is not automatically a public cause accessor value. qsl-semantics/src/check/lowering.rs:742-760 emits the one optional wrapper from admitted Presence. No private assembly dependency is justified.

The genuine public positive List/Tree controls, exact List key/Optional field, original package/context binding, generated check_type/no-charge behavior and private exact four-way MalformedComposite defenses remain required. DuplicateMember's exact name and healthy sibling remain separate private obligations. No recursion-wide admission classification is inferred from the duplicate example.

Both computed matrices contain 884 criteria: 521 tagged, 295 untagged, 50 method-without-symbol and 18 ignored-only; IDs, methods, binders and statuses are unchanged. Only AC8/AC26 statements change. Prior independently posted PR494 census's exact 311 IDs equal this inherited set minus IR621 AC123/124; no unmatched owner set. One additional planned AC8 authentic-source stage-control obligation is not established by existing synthetic CG admission test. Tags alone do not prove updated stage semantics.

Whole-source reverse inventory examined public library exports, CLI, publication, core profile/oracle, proptest, Kani generation/execution/corpus and equality seams against the corresponding core/oracle/proptest/kani requirement families. No newly unspecified surface or actual hollow source/test implementation was identified in this docs-only change; no source is changed. Existing explicit planned paths are not presented as implementation. No optional semantic repository-wide review was requested or performed. Plan completion: not assessed

Catalog advisor independently reports AC8 and AC26 authored Test with no mismatch or inconclusive result: property-based tests for AC8; fuzz/example/negative-abuse/unit recommendations for AC26. These methods are appropriate, but cannot make an inaccessible cause field observable. Both changed Verification cells now name only Test. Quire spec validation and whitespace checks pass. No Cargo, rustfmt, metadata, build, native execution or proof job ran; Rust gate execution is inapplicable to this specification-only diff and prohibited during the root's exclusive cargo gate.

No applicable AssuranceProfile with enforced review_selection found in the scoped spec. EARS analysis skipped because Description requirement statement is unchanged; dependency analysis skipped because no relationship edge changes; object review skipped because no domain object schema changes; risk-complexity skipped because no algorithm/resource design change; criterion-strength calibrated Jev unavailable and not selected.

## Disposition round 1 verification

Exact disposition head 6a7d1eb643918110f48e51810b283a26844e70a8; reviewed original 7ab13b096fc004ed6c5eabab41440614791b8608; unchanged base bc4aea2bdc4635cd6fb76e5a7cf9adb9ac528301. Only FR018 and TC029 changed, with public accessor assertions and mutation guard corrected. Source, dependency, lock, private precise DuplicateMember and four optional-wrapper controls, List/Tree genuine key/Presence routes, package/context authenticity, sibling isolation and generated check_type/no-charge clause are unchanged. Selected owner spine.rs:235-239 and :255-272 independently reconfirm the actual code and absent cause. No fabricated context, assembly dependency, weakened assertion or changed recursion classification introduced. No native gate was executed.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Verified exact owning public accessor contract and corrected AC8/procedure/mutation text. |
