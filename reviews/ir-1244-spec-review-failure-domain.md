---
id: SR-17940486709564847047608
title: "IR-1244 PR #770 pre-decoded prepared function failure-domain analysis"
type: SpecReview
analysis: failure-domain
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

CHANGES REQUIRED. The in-proof failure paths are kept genuine: ready() still maps EnvironmentFailure::Refused to GeneratedEnvironmentRejected and other failures to a typed BodyFailure, and finish() keeps the admits/GeneratedUnexpectedOutcome path, because no environment verdict is pre-built. Defects: the named generation refusal cannot be reached, the const-data enumeration and rebuild have unstated failure modes, and the genuine path's own refusals have no outcome.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-46 cannot be reached through the current generator. generate_exact_function_oracles receives source: &CheckedPackageV2, a typed value that exists only after Admitted. CheckedPackageV2 keeps no bytes; CG re-encodes them via encode_checked_package_v2 (owner_path.rs:273). No CG generator path reads retained bytes: the production CheckedPackageV2::read calls are in src/replay/composite_builder.rs. So 'the strict reader does not return Admitted for the retained package bytes at generation time' cannot happen there. Name the generator entry that takes the retained bytes, read limits and evidence, so that AC-46's refusal is reachable and testable. | spec/oracle/functional/FR-021-function-application-oracles.md:539 |
| FND-002 | medium | The const-data list (key, qualified name, form, members; parameters; result; location) leaves out inputs the genuine construction uses: NominalDeclaration::node(), the final_key argument of seal_union_verified(handle, final_key, bindings); the union bindings as quire_exact::UnionMember (declaration NodeKey plus identifier); TypeEnvironmentLimits (caller-supplied CheckedFunctionResources.environment_limits, which decides EnvironmentFailure::Limit); and the Cancel. NodeKey (Arc<NodeData>), ValueType, Identifier, UnionMember and ExpressionLocation have heap/Arc storage and fallible constructors (NodeKey::new -> InvalidNodeKey, UnionMember::from_admitted -> EmptyVariantName, Identifier::new), so they cannot be Rust `const` items. qsl_replay NominalDeclaration views cannot be forged (compile_fail). The 'const data' must therefore be literal descriptors rebuilt inside the proof. The spec gives no outcome for a failed rebuild, and does not say which limits the harness and the differential use (they must match, or the environment comparison differs on Limit). Add node and bindings to the list, name the limits source, and state the rebuild-failure outcome. | spec/oracle/functional/FR-021-function-application-oracles.md:411 |
| FND-003 | medium | AC-45(b) defines only structural equality of the Ok values. The genuine PreparedFunction::new returns Result<Self, CheckedFunctionError>: checked_function_declarations can give OwnerPackage or OwnerIdentityMismatch, and the member-form mismatch or Identifier::new failure gives InvalidIdentity. Nothing says what the generator does when that genuine call errs at generation time (typed generation refusal, no harness?) or what the differential reports when it errs. Name both outcomes. | spec/oracle/functional/FR-021-function-application-oracles.md:538 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-021-behavior-pre-decode | examined | spec/oracle/functional/FR-021-function-application-oracles.md:411 | - The function generator shall emit the pre-decoded prepared data (IR-1244) as `const` data holding only the reader's outputs consumed by `PreparedFunction::new`: the declaration views reachable from the selected callable (key, qualified name, form and members) and the selected signature (parameters and result), plus the result location. The emitted harness shall build the type environment from that data through the genuine `TypeEnvironment::bounded_with_cancel` and `seal_union_verified` calls, and shall run the generated `PreparedFunction::ready` and `finish` result-membership guards (`adm... |
| FR-021-AC-44 | examined | spec/oracle/functional/FR-021-function-application-oracles.md:537 | The emitted `cfg(kani)` harness for each function claim builds its prepared function from `const` data that CG generated from the retained package after the strict reader returned `Admitted` (not hand-built, not caller-supplied), consisting of the selected callable's declaration views and signature parameters/result plus the result location. Under `cfg(kani)` the harness calls `TypeEnvironment::bounded_with_cancel` and, for each union declaration, `seal_union_verified`, and runs `PreparedFunction::ready` and `finish` with their `admits` result-membership guard; the emitted text holds no pre... |
| FR-021-AC-45 | examined | spec/oracle/functional/FR-021-function-application-oracles.md:538 | Per emitted harness, a host test tagged to this criterion (a) runs the unmodified `CheckedPackageV2::read` over the retained package bytes and requires `CheckedPackageV2ReadResult::Admitted`; (b) builds `PreparedFunction::new` through the genuine path (`checked_function_declarations` over the acquired closure and selection, and the environment construction of AC-44) and requires it structurally equal to the value rebuilt from the generated `const` data in its environment (or its `EnvironmentFailure` refusal), parameters, result and location. Each of these adverse controls makes the differen... |
| FR-021-AC-46 | examined | spec/oracle/functional/FR-021-function-application-oracles.md:539 | When the strict reader does not return `Admitted` for the retained package bytes at generation time, the generator returns a typed `OracleGenerationError` refusal naming the reader refusal, introduced by IR-1244 (no existing variant describes it), and emits no harness, no const data and no partial bundle; an unrelated admitted item in the same request is unchanged. This is a generation-time refusal, distinct from AC-45's run-time differential control that mutates the package bytes after generation. |
| FR-021-AC-46-mutation | examined | spec/oracle/functional/FR-021-function-application-oracles.md:590 | Generate a harness or const data from a package the strict reader refuses, drop the refusal into a partial bundle, or reuse an unrelated `OracleGenerationError` variant that hides the reader refusal. / |

## New findings (disposition pass 1)

Re-review of fix head cf124b62.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | The fourth AC-46 refusal, 'a failed descriptor rebuild on the host', has no reachable trigger. The descriptors come from values the genuine path has just built, so rebuilding them with the genuine constructors (NodeKey::new, UnionMember::from_admitted, Identifier::new) succeeds except on an allocation or policy failure. Yet TC-031 step 16 requires feeding the entry 'a failing descriptor rebuild'. Name the seam that makes it fail (for example the NodeKey::try_new_with before_reserve policy hook or an injected reservation denial), or drop the variant. | spec/oracle/functional/FR-021-function-application-oracles.md:561 |
| FND-005 | medium | AC-45 embeds only the retained package bytes under cfg(not(kani)). Its step (b) runs acquisition and selection, which need the generation-time CheckedFunctionSource (source bytes, path, supplied packages, DependencyInput, stage/replay limits, body resources), the read limits and evidence, the admitted unit/declaring package and the function node id. None of these is said to be embedded or to equal what generation used. So the emitted differential either cannot run the genuine path, or runs it over different inputs. State that every generation input is embedded byte-identically and reused. | spec/oracle/functional/FR-021-function-application-oracles.md:560 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | fixed in cf124b62 (round 1) |
| FND-002 | fixed | fixed in cf124b62 (round 1) |
| FND-003 | fixed | fixed in cf124b62 (round 1) |
| FND-004 | fixed | fixed in a8848c2e (round 2) |
| FND-005 | fixed | fixed in a8848c2e (round 2) |
| FND-006 | fixed | fixed in 8bd12940 (round 3) |
| FND-007 | fixed | fixed in cbc9d7fa (round 4) |

## New findings (disposition pass 2)

Re-review of fix head a8848c2e.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | AC-44 now limits the cfg(kani) harness to non-nested function-body claims. Nothing says what the Kani-harness entry does when it is given a nested-call function (call_item, Transition::Call): emit no harness silently, refuse with a typed cause, or emit the host differential only. AC-46's four refusal causes do not cover it. Name the outcome so that FR-047 does not receive a claim with neither a harness nor a refusal. | spec/oracle/functional/FR-021-function-application-oracles.md:564 |

## New findings (disposition pass 3)

Re-review of fix head 8bd12940.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | The fifth cause is keyed on 'a requested claim whose selected body is a nested call (ExactFunctionBody::Call)'. ExactFunctionBody::Call { callee } exists (src/oracle/function/mod.rs:1164) and is the nested-call classification. However, it is a field of the caller-supplied ExactFunctionDeclaration that the existing generator takes, and the Kani-harness entry's inputs (Behavior: retained bytes, read limits and evidence, CheckedFunctionSource, function node id) include no declaration. Say how the entry learns the body form: classified from the reader's admitted graph as the existing generator does, or with ExactFunctionDeclaration added to its inputs. Otherwise two implementers will source it differently, and a caller-supplied classification could bypass the refusal. | spec/oracle/functional/FR-021-function-application-oracles.md:566 |
