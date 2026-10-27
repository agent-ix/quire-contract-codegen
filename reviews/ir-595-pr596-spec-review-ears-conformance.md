---
id: SR-590010
title: 'IR-595 #596 spec-review/ears-conformance review'
type: SpecReview
analysis: ears-conformance
scope: agent-ix/quire-contract-codegen@614556c4c00e88a73a0804273793aa1be33ba63e; spec/oracle/functional/FR-021-function-application-oracles.md,
  FR-021 AC-3/13/17/22/33
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-021
  type: reviews
---
# SR-590010: spec-review/ears-conformance

## Summary

Ticket: IR-595. PR: agent-ix/quire-contract-codegen#596. Frozen head: 614556c4c00e88a73a0804273793aa1be33ba63e. Reviewed only the FR-021 canonical index clarification against AC-13, AC-17, AC-22, AC-33 and existing TC-031 evidence. No production code or test changed.

## Verdict

**PASS** — no additional findings in this method’s scoped checks.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

- `quire validate --scope . spec/**/*.md --summary`: 118/120 docs grammar-clean; three EARS warnings lie in unchanged FR-017/FR-014. The changed FR-021 document is grammar-clean.
- `quoin matrix --repo . --json`: AC-3, AC-13, AC-17, AC-22 and AC-33 are tagged. The matrix establishes binding, not semantic adequacy.
- No applicable AssuranceProfile with a `review_policy` was found in this scoped spec tree.
- Code-review checks: no source/schema copy, generated duplicate, code edit or CI gate change in this Markdown-only diff.
- EARS: one new `shall` behavior, zero engine warnings and no semantic trigger/pattern defect.
- Integrity: AC-13 permutation invariance and AC-22/AC-33 refused-sibling isolation are preserved by sorting survivors; AC-17 source-body order remains a separate location index.

### Examined scope

- FR-021: For a scheduled item's function index, the generator shall use the canonical order of the request's surviving function declarations: ascending declaring V2 node id (domain, then digest), with zero-based indices.
- FR-021-AC-3: Each scheduled item records the requested function name and declaring V2 function node id, plus its zero-based index among surviving scheduled declarations in canonical request order (ascending declaring node-id domain, then digest). Permuting caller declaration order or adding a refused declaration leaves a healthy item's index and claim bytes unchanged. Any independent application a test builds for comparison is constructed from the request, never read back from generated code. Mapping the origin to the FR-485 callable selection remains Planned through IR-595.
- FR-021-AC-13: Schedule source and claim-map bytes are identical across runs and request permutations; entries order by call-node id, declaring function-node id, then source argument-node ids, with the name tie-break AC-24 specifies.
- FR-021-AC-17: Each location record uses the request function binding and function-body ordering for its origin; the runtime cross-check uses the FR-485 session's location derived from the declaring checked graph's root call site, for own and imported callables. The runtime check remains PLANNED pending IR-595; the QSL FR-485 facade is delivered.
- FR-021-AC-22: When two or more declarations share one declaring node id, none of them appears in the instruction table or in `location-map.json`, and every item naming one of them is refused with `ExactFunctionRefusal::DuplicateDeclaringNode { node_id }` (the shared node id; the smallest in node-id order when the item's name is held by duplicate groups on several node ids) before Stage 1 classification: never `UnknownFunction`, never `AmbiguousFunctionName` (the node-id refusal takes precedence when the declarations also share a name), never `DuplicateRequest` (two items on one `call` node naming different
- FR-021-AC-33: With an IR-admitted `temporal`/`formula`, `temporal`/`fairness` or `expression`/`case` node used directly as an item's call-node id, the claim-map entry retains that id and is `Refused { refusal: ExactFunctionRefusal::Unsupported }`. For the separate declared-body-root case, an item names that function through a distinct, IR-admitted `expression`/`call` node that lowers successfully and has no duplicate, capability, or earlier item refusal; its claim retains the call-node id and carries the body's `ExactFunctionRefusal::Unsupported`. For an IR `Unsupported` lowering record the refusal's `unsup
