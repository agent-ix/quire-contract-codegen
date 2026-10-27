---
id: SR-590006
title: IR-595 code-review of CG checked-closure cutover
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen@fc6c55aab9ac9abffd89d11fb96c731a05039cbf; src/oracle/function/,
  src/core/profile.rs, tests/exact_function_support/, tests/it/exact_function_*.rs
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-021
  type: references
---
# SR-590006: IR-595 code-review

## Summary

Ticket: IR-595. PR: agent-ix/quire-contract-codegen#595. Frozen head: fc6c55aab9ac9abffd89d11fb96c731a05039cbf. Scope: FR-021 AC-2/3/17/18/32/33/35/36/37; Rust production and test diff. No spec or plan files changed.

## Verdict

**FAIL** — two reachable wrong-result cases and one claim-index discrepancy.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The classifier accepts a checked body with different operand or callee bindings from the request, while emitted bodies use positional arguments and the request-named callee. A source x+x or add(y,x) can be emitted as x+y or add(x,y), so the concrete oracle can disagree with the independent FR-485 application. | src/oracle/function/mod.rs:2564-2577,2594-2598 |
| FND-002 | high | The equality classifier checks only binary form and arity, never the admitted operation identity. A NotEqual request over a genuine QSL x = y source emits NotEqual and returns the opposite result from the direct application. | src/oracle/function/mod.rs:2580-2590 |
| FND-003 | medium | The claim origin index is assigned after sorting surviving declarations by node digest, while AC-3 calls for the request-order function index. The edited tagged test computes the same digest sort instead of the independent request order. Reconcile AC-3 with AC-13 permutation determinism without narrowing either AC. | src/oracle/function/mod.rs:2894-2901,3273-3276 |

## Coverage

Plan completion: not assessed.
Reconciliation: `quoin matrix --repo . --json` (quoin 0.29.1); FR-021 has 40 criteria: 31 tagged, 5 method-without-symbol, 4 untagged. Repository baseline has 951 criteria: 563 tagged, 319 untagged, 53 method-without-symbol, 16 tagged only by ignored tests. These baseline gaps are outside the PR diff.
Semantic review: scoped intent ↔ test ↔ code check ran for the nine ACs listed below.

### Examined criteria

- FR-021-AC-2: For each function-application corpus vector, the oracle's outcome, admitted charge sequence and consumed counters equal a direct application independently assembled from the request with a fresh meter through QSL FR-485 acquisition, callable selection and session application of the QSL interpreted callable, with no generated body registered in that direct registry. The historical Contract Runtime `exact` port cannot serve as the target or independent comparison. This criterion claims no agreement with the QSL value authority: that boundary is owned by quire-integration IR-842 (FR-014-AC-47, FR
- FR-021-AC-3: Each scheduled item records the requested function name and declaring V2 function node id, plus its request-order function index; any independent application a test builds for comparison is constructed from the request, never read back from generated code. Mapping the origin to the FR-485 callable selection remains Planned through IR-595.
- FR-021-AC-17: Each location record uses the request function binding and function-body ordering for its origin; the runtime cross-check uses the FR-485 session's location derived from the declaring checked graph's root call site, for own and imported callables. The runtime check remains PLANNED pending IR-595; the QSL FR-485 facade is delivered.
- FR-021-AC-18: No test name, trace tag, test output line or doc comment of the function-application corpus, no TC-031 step and no oracle-matrix row claims, or presents as evidence, agreement in outcome, refusal, charge order or consumed counters with the QSL-owned application authority; text that states this boundary is permitted. That agreement is not claimed by CG and is owned by quire-integration IR-842 (FR-014-AC-47).
- FR-021-AC-32: Neither the schedule crate nor any concrete oracle requires `quire_contract_runtime::exact`, an alias, wrapper or compatibility layer; a concrete oracle uses `qsl_replay::acquire_checked_closure`, `CheckedClosureHandle::lookup_callable` and `registry()`, `ApplicationRegistry::register` and `begin_call`, and `CheckedApplicationSession::step` or `run` through the permitted FR-485 facade, without direct QSL evaluator or checker internals. CG and any emitted oracle crate hosting a generated body depend directly on `quire-eval`, `quire-exact` and `quire-semantic-value` beside `qsl-replay`, not on Q
- FR-021-AC-33: With an IR-admitted `temporal`/`formula`, `temporal`/`fairness` or `expression`/`case` node used directly as an item's call-node id, the claim-map entry retains that id and is `Refused { refusal: ExactFunctionRefusal::Unsupported }`. For the separate declared-body-root case, an item names that function through a distinct, IR-admitted `expression`/`call` node that lowers successfully and has no duplicate, capability, or earlier item refusal; its claim retains the call-node id and carries the body's `ExactFunctionRefusal::Unsupported`. For an IR `Unsupported` lowering record the refusal's `unsup
- FR-021-AC-35: For a concrete FR-485 corpus request, the caller supplies all seven acquisition inputs and the separately admitted Contract IR package. CG calls `qsl_replay::acquire_checked_closure` once for the generated run and receives `CheckedClosureCompilation` with emitted package, admitted context and retained `CheckedClosureHandle`; it does not reconstruct a QSL checked closure from the V2 graph or emitted source. Acquisition failure remains `AdmittedRefusal` and yields no handle. PLANNED pending IR-595; the QSL FR-485 facade is delivered.
- FR-021-AC-36: CG maps the declaring admitted V2 `package_id` domain and digest exactly to FR-485's `DigestRecord` owner id, and the function's `CheckedNodeId` digest exactly to `WireNodeId` only when its domain is `quire.checked-semantic-node/v1`. The own or imported declaring package is selected explicitly; no cross-domain coercion, rehash, fabricated `NodeKey` or name-only owner substitution occurs. CG calls `CheckedClosureHandle::lookup_callable(owner_package_id, wire_node_id)` and receives a checker-issued opaque `CallableSelection` before registration. It registers the generated `BodyFactory` with that
- FR-021-AC-37: A mismatch between `CheckedClosureHandle::package_id()` and the CG request's admitted unit package id refuses the item as `ExactFunctionRefusal::OwnerIdentityMismatch` before registration, Begin, body invocation or application charge. The same refusal follows a mismatch between `CallableSelection::owner_package_id()` and the selected function's declaring admitted V2 package id, or between the selection's read-only check-minted key bytes and that function's node digest. The selection comparisons run in a private post-lookup guard; TC-031 tests the key comparison with a genuine second selection 

### Changed-code ownership

FR-021 owns the function generator, generated body renderer, checked-closure adapter, and genuine source corpus. Cargo dependency and replay call-site deltas were read; no copied external source/schema artifacts or weakened CI lane appeared in the diff.

### FND-001

The classifier accepts a checked body with different operand or callee bindings from the request, while emitted bodies use positional arguments and the request-named callee. A source x+x or add(y,x) can be emitted as x+y or add(x,y), so the concrete oracle can disagree with the independent FR-485 application. The relevant requirement is FR-021-AC-2.

### FND-002

The equality classifier checks only binary form and arity, never the admitted operation identity. A NotEqual request over a genuine QSL x = y source emits NotEqual and returns the opposite result from the direct application. The relevant requirement is FR-021-AC-2.

### FND-003

The claim origin index is assigned after sorting surviving declarations by node digest, while AC-3 calls for the request-order function index. The edited tagged test computes the same digest sort instead of the independent request order. Reconcile AC-3 with AC-13 permutation determinism without narrowing either AC. The relevant requirement is FR-021-AC-3.

