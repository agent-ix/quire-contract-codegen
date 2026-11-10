---
id: SR-17940486709564847047607
title: "IR-1244 PR #770 pre-decoded prepared function dependency analysis"
type: SpecReview
analysis: dependency
date: 2026-11-10
scope: "agent-ix/quire-contract-codegen; PR #770 @ 154396ea; IR-1244 (decision D5 on IR-1436); spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md, spec/kani/functional/FR-047-v2-function-proof-producer.md, spec/kani/matrix/TC-061-v2-function-proof-producer.md, spec/oracle/matrix/tests.md, spec/kani/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-047
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-031
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-061
    type: reviews
---

## Summary

Ticket: IR-1244. Spec-only PR #770 (head 154396ea, one commit on base 67739cf5) for IR-1436 decision D5: the oracle-function Kani harness runs over pre-decoded prepared data that a host differential binds to the reader. It edits FR-021 Behavior, adds FR-021-AC-44/45/46 and their mutation rows, and edits FR-047-AC-1, TC-031 steps 15/16, TC-061 step 2 and both tests.md files. Reviewed against CG origin/main 67739cf5 src/oracle/function/owner_render.rs (HEADER PreparedFunction::new/ready/finish, render_item, call_item), src/oracle/function/owner_path.rs (CheckedFunctionResources, apply_checked_function, select_checked_callable, checked_function_declarations, package_bytes), src/oracle/function/mod.rs generate_exact_function_oracles, src/oracle/claim.rs OracleGenerationError; quire-semantic-value 3c4616d declaration.rs (TypeEnvironment, PartialEq impl, EnvironmentFailure, bounded_with_cancel, seal_union_verified) and location.rs (ExpressionLocation); quire-exact 5db2d5e node.rs (NodeKey), value.rs (UnionMember), value_type.rs, cancel.rs; qsl-replay 434869e application.rs (NominalDeclaration, NominalDeclarationMember); Linear IR-1244 (AC-4 measurements) and IR-1436 D5 decision comment.

## Verdict

CHANGES REQUIRED. Edges hold: Linear IR-1121 is blocked-by IR-1244 (and IR-1232, IR-1231, IR-595), matching the oracle matrix note 'IR-1121 stays blocked until that code lands' and the kani matrix row. No IR-1022-lane file (FR-015/017/028) is touched. One ownership split is left undefined.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Harness and differential ownership is undefined. AC-44/46 give the cfg(kani) harness to FR-021's function generator ('the emitted cfg(kani) harness', 'the generator ... emits no harness'). FR-047-AC-1 says the FR-047 producer (IR-1121) 'produces one executable Kani harness'. Today FR-021 emits body modules, not harnesses. AC-45's 'Per emitted harness, a host test tagged to this criterion' mixes two things: a tagged CG repo test over fixtures, which is where the six adverse controls can live, and a mandatory check for every harness the producer emits for arbitrary claims (TC-061 step 2: 'passed for this exact emitted harness'). Neither text says who emits or runs the per-harness differential, or where it lives. PreparedFunction and its fields are private to the emitted crate, so a field-wise compare must be emitted into that crate under cfg(not(kani)). Neither says how a pass is recorded in the FR-047 identity. If the differential is emitted into the crate, the AC-44 / TC-031 step 16 text check 'contains no CheckedPackageV2::read' must be limited to cfg(kani) items. Fix: split AC-45 into the fixture-level controls (TC-031) and a criterion for the per-emitted-harness differential that names its owner, location and recorded result. | spec/oracle/functional/FR-021-function-application-oracles.md:538 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-021-AC-44 | examined | spec/oracle/functional/FR-021-function-application-oracles.md:537 | The emitted `cfg(kani)` harness for each function claim builds its prepared function from `const` data that CG generated from the retained package after the strict reader returned `Admitted` (not hand-built, not caller-supplied), consisting of the selected callable's declaration views and signature parameters/result plus the result location. Under `cfg(kani)` the harness calls `TypeEnvironment::bounded_with_cancel` and, for each union declaration, `seal_union_verified`, and runs `PreparedFunction::ready` and `finish` with their `admits` result-membership guard; the emitted text holds no pre... |
| FR-021-AC-45 | examined | spec/oracle/functional/FR-021-function-application-oracles.md:538 | Per emitted harness, a host test tagged to this criterion (a) runs the unmodified `CheckedPackageV2::read` over the retained package bytes and requires `CheckedPackageV2ReadResult::Admitted`; (b) builds `PreparedFunction::new` through the genuine path (`checked_function_declarations` over the acquired closure and selection, and the environment construction of AC-44) and requires it structurally equal to the value rebuilt from the generated `const` data in its environment (or its `EnvironmentFailure` refusal), parameters, result and location. Each of these adverse controls makes the differen... |
| FR-021-AC-46 | examined | spec/oracle/functional/FR-021-function-application-oracles.md:539 | When the strict reader does not return `Admitted` for the retained package bytes at generation time, the generator returns a typed `OracleGenerationError` refusal naming the reader refusal, introduced by IR-1244 (no existing variant describes it), and emits no harness, no const data and no partial bundle; an unrelated admitted item in the same request is unchanged. This is a generation-time refusal, distinct from AC-45's run-time differential control that mutates the package bytes after generation. |
| FR-047-AC-1 | examined | spec/kani/functional/FR-047-v2-function-proof-producer.md:147 | An admitted current-V2 function claim selected at one exact occurrence produces one executable Kani harness and persisted V2 identity. The harness drives the generated FR-021 body module directly (`Factory::begin` then `resume(Resume::Start(..))`) over generation-time-decoded prepared data that CG derives from the strict-reader-admitted retained `CheckedPackageV2` (FR-039, unmodified) and that a host differential binds to that reader's output, per FR-021-AC-42 and FR-021-AC-44 through AC-46, with the `ready`/`finish` result-membership guards and the genuine type-environment construction ins... |
| TC-061-step-2 | examined | spec/kani/matrix/TC-061-v2-function-proof-producer.md:31 | 2. Run the unmodified harness through installed Kani within its recorded ceilings. The harness runs over generation-time-decoded prepared data, not an in-proof `CheckedPackageV2::read`; require FR-021-AC-45's host differential (tagged in TC-031 step 16) to have passed for this exact emitted harness before its verdict is accepted, and credit no reader decode or admission to Kani (FR-047-AC-1, FR-021-AC-44 through AC-46). Require `Verified` and a satisfied cover. Mutate only the selected function body in a separately admitted same-claim source, regenerate through the same producer, and requir... |
| TC-031-step-16 | examined | spec/oracle/matrix/TC-031-function-application-oracles.md:326 | 16. Pre-decoded prepared function (FR-021-AC-44 through AC-46; IR-1244). Inspect the emitted harness text: it holds `const` declaration views, parameters, result and location, calls `TypeEnvironment::bounded_with_cancel` and `seal_union_verified`, runs `PreparedFunction::ready` and `finish`, and contains no `CheckedPackageV2::read`, pre-built environment or membership verdict. Run the host differential: the unmodified reader over the retained bytes returns `Admitted`; `PreparedFunction::new` built through the genuine path equals the value rebuilt from the const data in environment (or refus... |
| oracle-tests.md-FR-021-AC-44-46-row | examined | spec/oracle/matrix/tests.md:62 | / FR-021 / FR-021-AC-44 through FR-021-AC-46 / TC-031 / 🚧 Planned (IR-1244; spec first, code follows; IR-1121 stays blocked until that code lands): the pre-decoded prepared function for the oracle-function Kani harness, its mandatory per-harness host differential with adverse controls, and the generation-time refusal of a reader-refused package. Inspection plus host tests; reader decode and admission earn no Kani credit. / |
| kani-tests.md-FR-047-row | examined | spec/kani/matrix/tests.md:18 | / FR-047 / FR-047-AC-1 through FR-047-AC-7 / TC-061 / 🚧 PLANNED (IR-1121): no current-V2 function Kani proof producer or authentic retained proving-run identity; TC-061 additionally requires FR-021-AC-45's host differential per emitted harness (pre-decoded prepared data, IR-1244; reader decode earns no Kani credit); TC-061's installed healthy/mutant, same-node occurrence, mismatch and limit controls have not run. / |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | fixed in cf124b62 (round 1) |
