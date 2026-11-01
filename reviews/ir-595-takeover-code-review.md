---
id: SR-202610310132
title: IR-595 takeover code-review review
type: SpecReview
analysis: code-review
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

PASS — all original, fix-round and aggregate-discovered findings are resolved through 464d46da. Clippy and focused regressions pass; qualifying full integration on the changed production head remains pending.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Operand-type refusals expose represented V1 IDs instead of original V2 identities. | src/oracle/function/mod.rs:2947-2959 |
| FND-002 | low | Public origin rustdoc still promises request/survivor ordering after the source-order fix. | src/oracle/function/mod.rs:1434-1435,1508-1509 |

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

The new caller resolves parameter/result IDs with PackageItems::represented_node, then passes them directly to resolve_operand_type. Every UnsupportedOperandType/BlockedOnUpstream branch in that helper clones that represented ID into the public refusal. For types with graph-local Index children the original and represented frames differ, so the serialized refusal can name a different/nonexistent V2 node. The argument-type path at 3589 has the same issue. Preserve original identity in diagnostic payloads and add a regression with a source/represented frame difference.

### FND-002 detail

RecordedOrigin says claim records use request order, and GeneratedExactFunctionClaim::function_origin says canonical body-root order among surviving declarations. Both now use the original checked source function index (including omitted/refused predecessors), as the new regression proves. Update those comments and the stale Stage 2 package-admission comment at 3072.

## Dispositions

Round 1: static verification of 610c342485023504fd513efb9d239a168b327e3e. No Cargo commands were run by the reviewer. Runtime/clippy fix evidence remains queued and is not claimed verified.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Lookup and diagnostic identities are separated for parameter, result and argument paths; static inspection at 610c3424. Runtime regression is queued, not claimed passed. |
| FND-002 | fixed | Both public comments now match source-order behavior; stale Stage 2 admission comment corrected. |


## Qualification evidence

Inspected at e010c62b480477764b53d5b93f7d7b2583ae3a2e. Changes since the reviewed fixes are review-artifact custody and test doc-comment blank-line removal; no new production behavior is introduced.

- `/tmp/ir595-review-fix-clippy.log`: all-target clippy with denied warnings finished successfully.
- `/tmp/ir595-review-fix-it.log`: 16 passed, 0 failed; this includes the original-versus-represented type-refusal regression and compiled own/imported-owner native agreement cases.
- `/tmp/ir595-review-artifacts-validate.log`: Quire artifact validation completed with advisory warnings and no validation error.
- CG baseline remains f3d328e37d28a381e68bb4b7bbe248218e712f95; QSpec fixture baseline remains 3405bdb8c5355adef03202d4573a95c4219178d9.

The lead has started the single aggregate gate. `/tmp/ir595-aggregate-summary.txt` had no completed stage entries when inspected; no aggregate or full-IT success is claimed here. Historical pending notes in the earlier disposition rounds describe their evidence at that time. No reviewer Cargo commands or source edits were performed.


## New findings (disposition pass 3)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | Routed function-claim regression compared original metadata with the lowered occurrence frame. | src/routed/contract_package_tests.rs:3045-3048 (d0d4f039) |
| FND-004 | medium | Signature typing displaced the required body-form refusal for quantity-conversion bodies. | src/oracle/function/mod.rs:2944-2960 (d0d4f039) |
| FND-005 | medium | The owner-path re-export in function/mod.rs violated the root-only export boundary. | src/oracle/function/mod.rs:113-118 (d0d4f039) |
| FND-006 | high | The new preflight duplicates body-form validation still present in classify_body_shape. | src/oracle/function/mod.rs:2720-2724,2742-2746,2773-2777,2946-2960 |

### FND-003 aggregate detail

The aggregate library target reproduced one failure: valid original V2 source-map metadata was compared against V1 requested_items occurrences with different graph-local indexes. The fixed test must verify the original checked.source_map and prove the frames actually differ.

### FND-004 aggregate detail

The aggregate integration target reproduced UnsupportedOperandType instead of the existing FormMismatch(conversion) contract. The preflight at 874f9863 restores supported-body form refusal before signature typing and retains unsupported-scalar-operator precedence; the unchanged conversion fixture now passes.

### FND-005 aggregate detail

The aggregate layout target rejected pub use owner_path in an intermediate directory. At 874f9863 owner_path is pub(crate), the helpers are re-exported directly by lib.rs, and the layout and rustdoc checks pass without changing the root API.

### FND-006 aggregate detail

Searching classify_body_shape finds one production call. The new required_form preflight maps supported scalar/equality to binary and nested calls to call, but classify_body_shape independently checks the same three forms and constructs the same FormMismatch result. Two owners now encode one rule. Retain the required pre-signature ordering while removing the redundant inner guards, or centralize the policy once; preserve unsupported-operator precedence. The code-review duplication rule requires one home for a rule.


### Round 3

Inspected the complete aggregate logs at d0d4f039 and the fixes at 874f9863315b77f8bdef1479c355e1f569d530bd. The aggregate completed fmt/spec/clippy/check/deny/docs/gate-build/QSpec-selftest successfully. Library result was 487 pass/1 fail/8 ignored; integration was 744 pass/3 fail/34 ignored. Those were discovery results, not final qualification. The source-map, conversion-form and export-boundary fixes are verified statically and by the supplied focused logs (33/33 library, 23/23 integration, clippy and rustdoc PASS). The remaining tc_025_model_member_types_without_i64_ranges_remain_unranged error is the explicitly accepted existing QSL model.value-type/v1 baseline, not new delivery credit. Full integration qualification on the changed production head is pending. No reviewer Cargo commands or source edits occurred.

| FND | outcome | reason |
| --- | --- | --- |
| FND-003 | fixed | The aggregate library target reproduced one failure: valid original V2 source-map metadata was compared against V1 requested_items occurrences with different graph-local indexes. The fixed test must verify the original checked.source_map and prove the frames actually differ. |
| FND-004 | fixed | The aggregate integration target reproduced UnsupportedOperandType instead of the existing FormMismatch(conversion) contract. The preflight at 874f9863 restores supported-body form refusal before signature typing and retains unsupported-scalar-operator precedence; the unchanged conversion fixture now passes. |
| FND-005 | fixed | The aggregate layout target rejected pub use owner_path in an intermediate directory. At 874f9863 owner_path is pub(crate), the helpers are re-exported directly by lib.rs, and the layout and rustdoc checks pass without changing the root API. |
| FND-006 | still-open | The form-policy duplication remains at frozen 874f9863; consolidate it before the qualifying full IT. |


### Round 4

Inspected 464d46da44ed66513359aee0af6c16b72bf9e94c: the two binary and one call semantic-form guards were removed from classify_body_shape, along with its now-unused node/generic argument. The single supported-body required_form preflight remains before signature typing. The unsupported scalar operator path still defers to its existing signature/operator refusal order. The independent call-item form guard serves a different obligation and remains appropriate. Supplied all-target clippy and 23/23 focused integration checks pass, including conversion, Negate, isolation, signature, compiled own/imported agreement and root-only exports. Full changed-head integration qualification remains pending.

| FND | outcome | reason |
| --- | --- | --- |
| FND-006 | fixed | Required body-form policy has one home; classify_body_shape consumes the preflight result without repeating its form checks. |
