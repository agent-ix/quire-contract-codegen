---
id: SR-17940486709564847047606
title: "IR-1244 PR #770 pre-decoded prepared function EARS conformance analysis"
type: SpecReview
analysis: ears-conformance
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

CHANGES SUGGESTED. The new Behavior bullet is a well-formed ubiquitous 'The function generator shall emit ...' sentence, and AC-46 is a well-formed 'When ... the generator returns ...' event response. Two terms are left open.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-46 requires 'a typed OracleGenerationError refusal naming the reader refusal, introduced by IR-1244' but gives no variant name or payload. I checked the real variants (ContextPackage*, MissingOriginalCheckedPackage, CheckedPackageEncoding, EnvironmentAdmission, PackageRequest, UnauthenticatedRequestedRootKind, ...). None describes a read result, so a new variant is right, but the spec must own its name and payload. CheckedPackageV2ReadResult has three non-admitted arms (Refused, CatalogUnavailable, Incomplete), and the payload must keep them distinct. Behavior says 'a retained package the strict reader refuses', while AC-46 says 'does not return Admitted'. Incomplete and CatalogUnavailable are not refusals, so align the two. Without a name, the AC-46 mutation 'reuse an unrelated variant that hides the reader refusal' cannot be checked. | spec/oracle/functional/FR-021-function-application-oracles.md:539 |
| FND-002 | low | 'The emitted cfg(kani) harness for each function claim' includes nested-call claims (call_item, Transition::Call). Behavior puts those outside the Kani obligation until a proof-safe scheduler entry exists. Limit AC-44 to the body forms inside the obligation (scalar and equality), or state what a nested-call claim gets instead. | spec/oracle/functional/FR-021-function-application-oracles.md:537 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-021-behavior-kani-obligation | examined | spec/oracle/functional/FR-021-function-application-oracles.md:377 | - Under `cfg(kani)` the emitted crate shall expose each generated body module `body_{symbol}` (with its `Factory`) as `pub(crate)` and shall not compile the host entry `oracle_{symbol}`: the owner-path entry runs `acquire_checked_closure`, which executes the QSL compiler, and Kani 0.68.0 cannot code-generate that compiler (model-checking/kani#4813, unreleased fix #4819; #5024, fix #5066 unmerged). QSL's checked closure is unforgeable and has no constructor from bytes; CG shall not add or request one. The Kani obligation is therefore exactly: the generated body, driven directly through `Body... |
| FR-021-behavior-pre-decode | examined | spec/oracle/functional/FR-021-function-application-oracles.md:411 | - The function generator shall emit the pre-decoded prepared data (IR-1244) as `const` data holding only the reader's outputs consumed by `PreparedFunction::new`: the declaration views reachable from the selected callable (key, qualified name, form and members) and the selected signature (parameters and result), plus the result location. The emitted harness shall build the type environment from that data through the genuine `TypeEnvironment::bounded_with_cancel` and `seal_union_verified` calls, and shall run the generated `PreparedFunction::ready` and `finish` result-membership guards (`adm... |
| FR-021-AC-44 | examined | spec/oracle/functional/FR-021-function-application-oracles.md:537 | The emitted `cfg(kani)` harness for each function claim builds its prepared function from `const` data that CG generated from the retained package after the strict reader returned `Admitted` (not hand-built, not caller-supplied), consisting of the selected callable's declaration views and signature parameters/result plus the result location. Under `cfg(kani)` the harness calls `TypeEnvironment::bounded_with_cancel` and, for each union declaration, `seal_union_verified`, and runs `PreparedFunction::ready` and `finish` with their `admits` result-membership guard; the emitted text holds no pre... |
| FR-021-AC-45 | examined | spec/oracle/functional/FR-021-function-application-oracles.md:538 | Per emitted harness, a host test tagged to this criterion (a) runs the unmodified `CheckedPackageV2::read` over the retained package bytes and requires `CheckedPackageV2ReadResult::Admitted`; (b) builds `PreparedFunction::new` through the genuine path (`checked_function_declarations` over the acquired closure and selection, and the environment construction of AC-44) and requires it structurally equal to the value rebuilt from the generated `const` data in its environment (or its `EnvironmentFailure` refusal), parameters, result and location. Each of these adverse controls makes the differen... |
| FR-021-AC-46 | examined | spec/oracle/functional/FR-021-function-application-oracles.md:539 | When the strict reader does not return `Admitted` for the retained package bytes at generation time, the generator returns a typed `OracleGenerationError` refusal naming the reader refusal, introduced by IR-1244 (no existing variant describes it), and emits no harness, no const data and no partial bundle; an unrelated admitted item in the same request is unchanged. This is a generation-time refusal, distinct from AC-45's run-time differential control that mutates the package bytes after generation. |
| FR-047-AC-1 | examined | spec/kani/functional/FR-047-v2-function-proof-producer.md:147 | An admitted current-V2 function claim selected at one exact occurrence produces one executable Kani harness and persisted V2 identity. The harness drives the generated FR-021 body module directly (`Factory::begin` then `resume(Resume::Start(..))`) over generation-time-decoded prepared data that CG derives from the strict-reader-admitted retained `CheckedPackageV2` (FR-039, unmodified) and that a host differential binds to that reader's output, per FR-021-AC-42 and FR-021-AC-44 through AC-46, with the `ready`/`finish` result-membership guards and the genuine type-environment construction ins... |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | still-open | AC-46 now fixes four distinct causes and leaves the names to the coder, which is acceptable because the causes are fixed. But it lists only the reader's Refused and Incomplete arms. The pinned reader (quire-contract-model c8dca8c, CG Cargo.lock) also returns CheckedPackageV2ReadResult::CatalogUnavailable(CatalogReadError), and CG already matches it (src/replay/composite_builder.rs:775). The author's claim that the pinned reader has no CatalogUnavailable is false. A CatalogUnavailable read has no named outcome; add it as its own refusal. |
| FND-002 | still-open | The non-nested scoping is not in the branch. The AC-44 criterion row was overwritten with mutation-row text (integrity FND-004), and no Behavior, AC or TC-031 text limits the harness to scalar/equality bodies or says what a nested-call claim gets. |
| FND-001 | fixed | fixed in a8848c2e (round 2) |
| FND-002 | fixed | fixed in a8848c2e (round 2) |
