---
id: SR-202610310133
title: IR-595 takeover gap-analysis review
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-contract-codegen; IR-595 PR #595 head 88fe1dd63ddbcde87e79032810accdf8f2be651d
  versus f3d328e37d28a381e68bb4b7bbe248218e712f95; function owner path, generated
  sources, structural identities and changed FR-021/AD-004'
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-021
  type: references
---

## Summary

Ticket: IR-595. Independent frozen-head takeover review. No source edits or Cargo runs were performed. Historical PR reviews remain historical evidence.

## Verdict

CONDITIONAL — original trace gap fixed; new FND-002 remains open.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Computed Test Matrix has no FR-021-AC-37 evidence despite the new negative tests. | src/oracle/function/owner_path.rs:1059-1218 |

## Coverage

Plan completion: not assessed. Review scope is the PR delta, not repository-wide delivery. No applicable AssuranceProfile exists in the repository. No enforced review workflow applies.

The code-review includes Rust style/API/boundary/safety checks. Gap analysis used the computed `quire matrix --scope .` (the installed CLI does not accept --json); findings are limited to changed FR-021 criteria. Spec base checks changed requirement/architecture text, and scope-boundary checks CG ownership of classification/rendering versus QSL acquisition/selection and leaf execution. EARS was not separately run because normative trigger statements were not changed; no new relationship edges or domain objects require dependency/object/integrity lenses. No optional repository-wide semantic audit or plan completion claim was made.

Evidence inspected: all-target clippy PASS; 14 native agreement tests PASS; 8 origin/isolation regressions PASS; 9 owner-path tests PASS; 100000-call/512KiB/split compilation PASS; focused Quire validation exit 0 with advisory warnings. The final aggregate gate/full integration target remains pending.

### Examined units

- FR-021-AC-1 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): For a request whose required source views are admitted, every requested `call` item receives exactly one scheduled or typed-refused disposition; a refused item contributes no instruction root, concrete symbol or location entry, while an unrelated admitted item is unchanged. A plain-read V1 request with an eligible temporal-tag `Unsupported` record and `NotAdmitted` requested-root kind returns the whole-call AC-41 refusal before dispositions. Concrete generated disposition remains Planned through the FR-485 consumer (IR-595).
- FR-021-AC-2 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): For each function-application corpus vector, the oracle's outcome, admitted charge sequence and consumed counters equal a direct application independently assembled from the request with a fresh meter through QSL FR-485 acquisition, callable selection and session application of the QSL interpreted callable, with no generated body registered in that direct registry. The historical Contract Runtime `exact` port cannot serve as the target or independent comparison. This criterion claims no agreement with the QSL value authority: that boundary is owned by quire-integration IR-842 (FR-014-AC-47, FR
- FR-021-AC-3 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): Each scheduled item records the requested function name, actual declaring V2 `function` node id, and zero-based index among surviving schedule declarations sorted by `FunctionScheduleDeclaration.function_node_id` (domain, then digest). Each concrete exact-oracle claim also records the actual V2 function node id separately from its `ExactFunctionDeclaration.node_id` body-root expression key, and its zero-based original source function index, matching the FR-485 checked callable origin even when earlier functions are omitted or refused. Permuting either API's caller declaration order or adding a
- FR-021-AC-4 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): For an admitted callable, the oracle forwards arity, wrong-value-kind and dangling-reference Begin input refusals unchanged from `ApplicationBeginRefusal::Call(CallFailure::Input)` before any execution charge. An absent requested name retains CG `ExactFunctionRefusal::UnknownFunction`; an invalid owner or callable id is refused by FR-485 `lookup_callable` as `Unknown`, `Foreign(package_id)` or `WrongKind` before selection or Begin, never as a Begin input refusal. The schedule records ordered arguments without inventing a refusal result. Agreement with the QSL value authority remains owned by I
- FR-021-AC-5 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): The schedule orders arity validation before per-argument validation, validates arguments in declared parameter order, then places `function.call` before body work; the FR-485 session is the target for the matching owner-path corpus run. The corpus half remains
- FR-021-AC-8 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): The schedule marks only linked-package admission as applicable and never marks a kernel-mode package applicable. Concrete mode admission through FR-485 remains Planned through IR-595.
- FR-021-AC-9 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): The schedule preserves a `function.call` charge point and an incomplete stop before body work, and the denied charge is not applied; FR-485 session application is the target for the matching denied-charge vectors. The agreement vectors remain
- FR-021-AC-10 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): A declared parameter or result type reaching a `reference` composite form is refused as blocked on quire-spec-language#120, with no schedule root or concrete code for that function. No meter charge is admitted for it; a runtime charge assertion through FR-485 remains Planned through IR-595.
- FR-021-AC-12 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): A declared function whose own body fails to lower — an unlowerable node, a form none of this generator's classifiers admits, or a nested `call` naming a function absent from the request — refuses every item bound to that function with a typed reason, without changing an unrelated package's items.
- FR-021-AC-13 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): Schedule source and claim-map bytes are identical across runs and request permutations in either API; entries order by call-node id, that API's declaration key (the actual function node id for `generate_function_schedules`, the admitted body-root expression id for `generate_exact_function_oracles`), then source argument-node ids, with the name tie-break AC-24 specifies. The exact-oracle claim still records the actual function node id separately as AC-3 requires.
- FR-021-AC-14 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): The schedule crate declares `publish = false`, compiles without a Contract Runtime `exact` dependency, contains no charge amount or literal outcome/value standing in for owner execution, and stays within caller-selected source bytes and publication artifact limits. Concrete owner-path emission through FR-485 remains Planned through IR-595.
- FR-021-AC-16 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): The schedule and its claim/location maps neither read nor depend on dynamic invocation `location` or `losses` fields. No oracle makes a claim that those fields are populated. A future FR-485 session may return dynamic fields, but that alone does not make the CG schedule or an oracle claim their population.
- FR-021-AC-17 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): Each location record uses the request function binding and function-body ordering for its origin; the runtime cross-check uses the FR-485 session's location derived from the declaring checked graph's root call site, for own and imported callables. The runtime check remains
- FR-021-AC-18 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): No test name, trace tag, test output line or doc comment of the function-application corpus, no TC-031 step and no oracle-matrix row claims, or presents as evidence, agreement in outcome, refusal, charge order or consumed counters with the QSL-owned application authority; text that states this boundary is permitted. That agreement is not claimed by CG and is owned by quire-integration IR-842 (FR-014-AC-47).
- FR-021-AC-19 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): The emitted schedule crate for scalar, equality and nested-call corpus cases contains zero `.unwrap(`, `.expect(`, `unreachable!`, `panic!`, `todo!` and `unimplemented!` tokens; no concrete FR-485 owner-path oracle exists yet to check.
- FR-021-AC-20 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): For each scalar and equality instruction, the target-neutral schedule preserves completed, undefined, refused, incomplete and checked-invariant result distinctions without folding one into another. Forwarding FR-485's owner result variants remains Planned through IR-595.
- FR-021-AC-21 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): A scalar `Negate` body is refused as `UnsupportedOperator` with no schedule root, whether or not it reaches a `quire.op.quantity.convert` node (a conversion is not refused by name, because FR-021 has no quantity body, operand or argument support: the generator refuses a body that is one as `UnsupportedOperator` and the schedules as `ScheduleRefusal::UnsupportedOperation`; an earlier check that fires first is the accepted result: `Unsupported` for a direct unsupported form of the root, `NotExpression` and the other root checks, `UnsupportedOperandType` for a quantity-typed parameter or result, 
- FR-021-AC-22 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): When two or more declarations share one declaring node id, none of them appears in the instruction table or in `location-map.json`, and every item naming one of them is refused with `ExactFunctionRefusal::DuplicateDeclaringNode { node_id }` (the shared node id; the smallest in node-id order when the item's name is held by duplicate groups on several node ids) before Stage 1 classification: never `UnknownFunction`, never `AmbiguousFunctionName` (the node-id refusal takes precedence when the declarations also share a name), never `DuplicateRequest` (two items on one `call` node naming different 
- FR-021-AC-24 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): Items on one `call` node, with equal argument operands, that name functions absent from the request's declarations each get their own claim-map entry. Requesting `zz_unknown` and `aa_unknown` on one node yields two entries, in that order: `UnknownFunction { name: "aa_unknown" }`, then `UnknownFunction { name: "zz_unknown" }` (function name compared byte-wise over its UTF-8 bytes, case-sensitively), and no `DuplicateRequest`; the entries are identical under both request orders. Requesting `Zz_unknown` and `aa_unknown` yields `Zz_unknown` first, because `Z` (0x5A) sorts before `a` (0x61). Reques
- FR-021-AC-32 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): Neither the schedule crate nor any concrete oracle requires `quire_contract_runtime::exact`, an alias, wrapper or compatibility layer; a concrete oracle uses `qsl_replay::acquire_checked_closure`, `CheckedClosureHandle::lookup_callable` and `registry()`, `ApplicationRegistry::register` and `begin_call`, and `CheckedApplicationSession::step` or `run` through the permitted FR-485 facade, without direct QSL evaluator or checker internals. CG and any emitted oracle crate hosting a generated body depend directly on `quire-eval`, `quire-exact` and `quire-semantic-value` beside `qsl-replay`, not on Q
- FR-021-AC-33 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): With an IR-admitted `temporal`/`formula`, `temporal`/`fairness` or `expression`/`case` node used directly as an item's call-node id, and an authenticated requested-root kind for an IR `Unsupported` lowering record, the claim-map entry retains that id and is `Refused { refusal: ExactFunctionRefusal::Unsupported }`. For the separate declared-body-root case, an item names that function through a distinct, IR-admitted `expression`/`call` node that lowers successfully and has no duplicate, capability, or earlier item refusal; its claim retains the call-node id and carries the body's `ExactFunctionR
- FR-021-AC-35 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): For a concrete FR-485 corpus request, the caller supplies all seven acquisition inputs and the separately admitted Contract IR package. CG calls `qsl_replay::acquire_checked_closure` once for the generated run and receives `CheckedClosureCompilation` with emitted package, admitted context and retained `CheckedClosureHandle`; it does not reconstruct a QSL checked closure from the V2 graph or emitted source. Acquisition failure remains `AdmittedRefusal` and yields no handle.
- FR-021-AC-36 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): CG selects the own or imported declaring package explicitly by its admitted structural library name and compares its complete canonical V2 bytes, including the typed term sidecar, with the bytes retained by the same QSL acquisition. Imported bytes come from `CheckedClosureCompilation::owner_package_bytes`; own-unit bytes come from `package()`. CG resolves the function identity in its owning V2 frame through `ContractSourceKeyV1::from_v2_request(source, request)` and mechanically represents that structural key as the Exact `NodeKey`; no rehash, cross-frame index comparison, fabricated identity 
- FR-021-AC-37 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): A mismatch between the acquired unit's structural name or complete canonical V2 bytes and the CG request's admitted unit refuses the item as `ExactFunctionRefusal::OwnerIdentityMismatch` before registration, Begin, body invocation or application charge. The same refusal follows a mismatch between `CallableSelection::owner_package_name()` and the selected function's declaring admitted library name, or between the selection's read-only checker-issued structural key and that function's source-resolved structural identity. The selection comparisons run in a private post-lookup guard; TC-031 tests 
- FR-039 (context_only, spec/core/functional/FR-039-contract-package-consumption.md): CG SHALL bind the owning reader-admitted V2 source to the admitted V1 package.source before comparing keys.
- AD-004 (examined, spec/assurance/AD-004-cg-crate-layout.md): The concrete function oracle uses the public QSL checked-closure application facade and direct leaf crates under FR-021-AC-32.
- FR-021 (examined, spec/oracle/functional/FR-021-function-application-oracles.md): When a caller supplies an admitted `quire.checked-package/v2` package, function
bindings and requested `call` nodes, the generator shall classify the admitted
expression-node graph, build a deterministic, target-neutral instruction
schedule for each admitted body, and record one generated or typed-refused
disposition per item. The schedule preserves V2 operation identity, operand
order, value/result types, function binding, and source occurrence identity;
it states no charge amount and executes no operation itself. Its body analysis
and schedule emission use charged iterative walks and no fixe

### FND-001 detail

quire matrix --scope . reports FR-021-AC-37 as (none)/untagged. The owner_path negative tests use fully qualified #[ix_trace_rs::trace(...)] attributes, which this installed scanner does not bind; the agreement support include file is likewise not evidence in the matrix. Use the repository-recognized trace import/attribute and verify the computed matrix binds the real mismatch and post-lookup tests.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | New frame-refusal regression claims AC-10 without exercising a reference-type blocker. | tests/it/exact_function_agreement.rs:1473-1516 |

operand_type_refusal_retains_the_original_v2_frame supplies a bounded_domain parameter and asserts UnsupportedOperandType, but is tagged FR-021-AC-10, whose obligation is a reference composite blocker. This creates incorrect AC-10 matrix evidence. Bind it to a criterion it actually verifies (for example FR-039-AC-2 with explicit original requested-identity/backend-refusal assertions), or trace it only to its owning requirement until a precise criterion exists; retain the regression.

## Dispositions

Round 1: static verification of 610c342485023504fd513efb9d239a168b327e3e. No Cargo commands were run by the reviewer. Runtime/clippy fix evidence remains queued and is not claimed verified.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Owner-path uses imported bare trace attributes. The refreshed computed matrix lists real mismatch/post-lookup tests and compiled support evidence for AC-37. |
| FND-002 | still-open | operand_type_refusal_retains_the_original_v2_frame supplies a bounded_domain parameter and asserts UnsupportedOperandType, but is tagged FR-021-AC-10, whose obligation is a reference composite blocker. This creates incorrect AC-10 matrix evidence. Bind it to a criterion it actually verifies (for example FR-039-AC-2 with explicit original requested-identity/backend-refusal assertions), or trace it only to its owning requirement until a precise criterion exists; retain the regression. |
