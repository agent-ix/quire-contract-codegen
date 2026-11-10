---
id: SR-17940486709564847047609
title: "IR-1244 PR #770 pre-decoded prepared function criterion-strength analysis (manual)"
type: SpecReview
analysis: criterion-strength
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

CHANGES REQUIRED. Jev (typesafe.ai System One) is not available here, so this analysis was done by hand against the code. Structural equality can be built field by field. TypeEnvironment has a manual semantic PartialEq (it ignores admission_work_units). EnvironmentFailure derives PartialEq. ValueType and ExpressionLocation implement PartialEq. The generated PreparedFunction does not (it derives Clone only and holds exact::Cancel, which is Clone only), so AC-45's list (environment-or-failure, parameters, result, location; Cancel left out) is the right comparison. Saying 'field-wise' would make that explicit. Controls 1-4 each change a different compared field, so together they catch 'compare only some fields' and 'compare the const data to itself'. Control 6 (bytes the reader refuses, failing at (a)) catches 'treat a non-Admitted read as a pass'. It is stated as a run-time control kept apart from AC-46's generation refusal, and TC-031 step 16 runs them separately, so the two are not mixed. The 'no hidden credit' clause can be checked through AC-44's inspection and TC-031 step 16. Defect: control 5 does not fail for the reason it states.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Control 5 ('mutating the retained package bytes so the reader still admits a package whose selected declaration or signature differs') does not fail the way it claims. The signature comes from QSL selection.signature(), not from the reader. With the acquisition source unchanged, select_checked_callable (owner_path.rs:305-309) and checked_function_declarations (416-419) compare the acquired package bytes with the re-encoded retained package and return OwnerIdentityMismatch before any structural comparison. So either the control fails through an identity refusal, which looks the same as a broken identity binding and is an outcome AC-45 does not define, or it needs a changed QSL source, which makes a different genuine package rather than 'mutated bytes'. Fix: compile a second genuine source whose declaration or signature changed, run acquisition over it, and require (b) to report structural inequality against the original const data. | spec/oracle/functional/FR-021-function-application-oracles.md:538 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-021-AC-44 | examined | spec/oracle/functional/FR-021-function-application-oracles.md:537 | The emitted `cfg(kani)` harness for each function claim builds its prepared function from `const` data that CG generated from the retained package after the strict reader returned `Admitted` (not hand-built, not caller-supplied), consisting of the selected callable's declaration views and signature parameters/result plus the result location. Under `cfg(kani)` the harness calls `TypeEnvironment::bounded_with_cancel` and, for each union declaration, `seal_union_verified`, and runs `PreparedFunction::ready` and `finish` with their `admits` result-membership guard; the emitted text holds no pre... |
| FR-021-AC-44-mutation | examined | spec/oracle/functional/FR-021-function-application-oracles.md:588 | Hand-build or accept caller-supplied prepared data, emit a pre-built `TypeEnvironment`, a membership verdict or an environment refusal as data, drop `ready`/`finish` or the `admits` guard from the harness, call `CheckedPackageV2::read` under `cfg(kani)`, or credit reader decode to a Kani proof. / |
| FR-021-AC-45 | examined | spec/oracle/functional/FR-021-function-application-oracles.md:538 | Per emitted harness, a host test tagged to this criterion (a) runs the unmodified `CheckedPackageV2::read` over the retained package bytes and requires `CheckedPackageV2ReadResult::Admitted`; (b) builds `PreparedFunction::new` through the genuine path (`checked_function_declarations` over the acquired closure and selection, and the environment construction of AC-44) and requires it structurally equal to the value rebuilt from the generated `const` data in its environment (or its `EnvironmentFailure` refusal), parameters, result and location. Each of these adverse controls makes the differen... |
| FR-021-AC-45-mutation | examined | spec/oracle/functional/FR-021-function-application-oracles.md:589 | Compare only some fields, compare the const data to itself instead of to the reader-built value, skip the `Admitted` requirement, or accept a differential that passes after one const member, the result type, a parameter type, the location or the retained bytes is mutated. / |
| FR-021-AC-46 | examined | spec/oracle/functional/FR-021-function-application-oracles.md:539 | When the strict reader does not return `Admitted` for the retained package bytes at generation time, the generator returns a typed `OracleGenerationError` refusal naming the reader refusal, introduced by IR-1244 (no existing variant describes it), and emits no harness, no const data and no partial bundle; an unrelated admitted item in the same request is unchanged. This is a generation-time refusal, distinct from AC-45's run-time differential control that mutates the package bytes after generation. |
| FR-021-AC-46-mutation | examined | spec/oracle/functional/FR-021-function-application-oracles.md:590 | Generate a harness or const data from a package the strict reader refuses, drop the refusal into a partial bundle, or reuse an unrelated `OracleGenerationError` variant that hides the reader refusal. / |
| TC-031-step-16 | examined | spec/oracle/matrix/TC-031-function-application-oracles.md:326 | 16. Pre-decoded prepared function (FR-021-AC-44 through AC-46; IR-1244). Inspect the emitted harness text: it holds `const` declaration views, parameters, result and location, calls `TypeEnvironment::bounded_with_cancel` and `seal_union_verified`, runs `PreparedFunction::ready` and `finish`, and contains no `CheckedPackageV2::read`, pre-built environment or membership verdict. Run the host differential: the unmodified reader over the retained bytes returns `Admitted`; `PreparedFunction::new` built through the genuine path equals the value rebuilt from the const data in environment (or refus... |

## New findings (disposition pass 1)

Re-review of fix head cf124b62.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | The mutated-bytes control ('the retained bytes of the first source mutated while its source is kept, so the genuine path returns Err(CheckedFunctionError::OwnerIdentityMismatch)') does not require the mutated bytes to stay reader-admitted. An arbitrary byte mutation is most likely reader-Refused. It then fails at (a) exactly like the truncated-bytes control and never reaches the identity check, so the two controls do not separate. Specify a reader-admitted replacement (for example the second source's genuine package bytes with the first source kept) and require (a) to pass before the OwnerIdentityMismatch. This also depends on base FND-003's binding between the read package and the selection inputs. | spec/oracle/functional/FR-021-function-application-oracles.md:562 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | fixed in cf124b62 (round 1) |
| FND-002 | fixed | fixed in a8848c2e (round 2) |

## New findings (disposition pass 2)

Re-review of fix head a8848c2e.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The mechanism is sound. The differential passes the reader's own Admitted value as both admitted unit and declaring package. select_checked_callable then compares handle.package_name() and acquired.package().bytes() with the re-encoded reader output (owner_path.rs:303-309) before any lookup, so any admitted bytes that differ from the unchanged acquisition yield OwnerIdentityMismatch first. But the control is pinned to 'a same-length edit to one identifier byte that the reader still returns Admitted for', and such an edit may not exist. The strict reader enforces identifier syntax, strictly ascending member order and identity joins (identity.rs, frame.rs, lock_tables.rs), and checked node ids embed names, so a single name byte usually appears in several joined places. Accept any reader-admitted retained bytes that differ from the acquisition's package, for example the second source's genuine package bytes with the first source's acquisition kept. Those always exist and keep the Admitted-then-OwnerIdentityMismatch order. | spec/oracle/functional/FR-021-function-application-oracles.md:567 |
