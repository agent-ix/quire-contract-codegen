---
id: FR-021
title: "Generate exact complete-V1 function-application oracles"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-018
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: references
  - target: ix://agent-ix/quire-specification/FR-460
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-030
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-036
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-038
    type: references
---
# FR-021: Generate exact complete-V1 function-application oracles

## Description

When a caller supplies an admitted `quire.checked-package/v2` package, function
bindings and requested `call` nodes, the generator shall classify the admitted
expression-node graph, build a deterministic, target-neutral instruction
schedule for each admitted body, and record one generated or typed-refused
disposition per item. The schedule preserves V2 operation identity, operand
order, value/result types, function binding, and source occurrence identity;
it states no charge amount and executes no operation itself. Its body analysis
and schedule emission use charged iterative walks and no fixed depth limit.

The current Contract Runtime `exact` implementation and its emitted host
closures are migration inputs, not this requirement's target ABI. Contract
Runtime [FR-275](ix://agent-ix/quire-contract-runtime/FR-275) deletes that port;
QSL-358 and IR-349 determine the QSL-owned callable, admission and result
surfaces. Concrete Rust oracle emission, compilation and execution against
those owner paths are planned until those surfaces exist. The generator shall
not preserve the port through an alias, wrapper or compatibility layer.

This is the function-application slice of complete-V1 oracle generation.
[FR-014](./FR-014-exact-scalar-oracles.md) supplies scalar forms and
[FR-018](./FR-018-composite-equality-oracles.md) supplies equality forms.
FR-018's entry point still refuses `call` nodes; this entry point alone
classifies function application. The QSL-owned application authority decides
runtime outcome and charge amounts, while this requirement decides the
schedule's operand and call order.

### Static location records

The generator shall record each operation occurrence with a stable occurrence
ordinal, its function origin, a parent occurrence ordinal and the child ordinal
at which its parent reached it. The root has no parent. A location path is
obtained only on request by following parent links to the root and reversing
the child ordinals; it is never copied into every entry during generation.
Each record is constant-size apart from its existing source-node reference.
The map is independent of runtime location or loss reporting. A path request
is charged one work unit and its output bytes per traversed parent link, and
refuses at the first denied charge without a partial path. Dynamic
`Evaluation.location`/`.losses` behavior remains owned by the eventual
QSL application API.

## Inputs

- An admitted `CheckedPackageV2` read through Contract IR's strict reader.
  The generator does not add an inline recursive V2 body form: QSpec
  [FR-322](ix://agent-ix/quire-specification/FR-322) fixes each body at
  schema depth and represents every nested composite subterm as a separate
  node reached by a `reference` leaf.
- Function bindings supplied with the request. Each binding names one admitted
  V2 `function` declaration node, whose own V2 body is the root, its function name,
  ordered parameter bindings to admitted `value` nodes of semantic form
  `parameter`, their declared type-node ids, and its result type-node id.
  The request does not insert these nodes into the package. A nested call's
  callee is argument 0 of a V2 `quire.op.function.call` application and shall
  be a `reference` to exactly the bound function declaration node; remaining
  arguments are V2 Member terms in source order. An item naming an unknown
  function retains `UnknownFunction`, and a body call to an unbound function
  retains `UnknownCallee`.
- A body graph rooted at each binding's `function` node body. An interior node is an
  admitted `expression` node whose Body is an `application` of an FR-014 scalar
  form, an FR-018 equality form or `quire.op.function.call`. Its value
  arguments are `literal` leaves or `reference` leaves to another expression,
  an admitted value node, or one of that function's bound `parameter` nodes.
  An expression reference may be reached from only one parent in this CG
  subset; a shared expression node is refused as `BodyMismatch` rather than
  given invented memoization or repeated-charge semantics. Parameter and
  immutable value leaves may be referenced more than once. Every referenced
  node, operand type and application result type shall match the admitted V2
  graph and the declared parameter/result types; a missing, cyclic or
  ill-typed edge is refused before schedule emission with its typed cause.
- Requested `call` items, each with an admitted V2 call node, a bound function
  name and its ordered source argument node ids. Their existing duplicate,
  ambiguous-name and missing-name rules continue to apply.
- A caller-selected `u64` per-function generation work limit covering graph
  analysis, schedule emission and compact location records; its default is
  65,536 units and its setting name is `cg.function.generation_work_units`.
  A caller-selected generated-source byte limit defaults to 1,048,576 bytes
  and has setting name `cg.function.source_bytes`. The caller may raise
  either to fit an admitted body.
- For a later full-path query, caller-selected query work and output-byte
  limits, defaulting to 65,536 parent links and 1,048,576 output bytes.
  Their setting names are `cg.function.location_query_work_units` and
  `cg.function.location_query_bytes`. They do not charge schedule generation
  and can be raised independently.

## Outputs

- A deterministic, unpublished Rust crate containing a flat SSA instruction
  table and operand tables in `src/lib.rs` or deterministic ordinal-range
  modules. Each instruction has one output slot, an operation tag, indices of
  previously defined operand slots, its V2 node id and one occurrence ordinal.
  The table contains no nested source expression or host-call closure. Its
  source bytes are bounded by the caller's generated-source byte limit; each
  module and the bundle remain within the publication artifact limits.
  Concrete callable Rust using the QSL-owned application API is a planned
  downstream emission from this table after QSL-358/IR-349 establishes that
  API; no Contract Runtime `exact` dependency is required by this output.
- A typed claim map with one entry per requested item: its existing request
  identity, ordering, package id, source map, reconstructed declaration
  closure, function binding and typed refusal, or a `Scheduled` disposition
  naming the root instruction slot. A concrete oracle symbol is added only
  when the owner-path emission is available; it is never fabricated by a
  schedule-only result.
- A location map of constant-size parent-linked occurrence records as defined
  above, encoded as deterministic positional arrays with node ids in a shared
  dictionary so long field names and node ids are not repeated per record.
  The generator partitions ordinal ranges when one map artifact would exceed
  its publication byte limit. The record for a call point names its
  instruction slot; its full path is materialized only by a charged query.

## Behavior

- When body analysis begins, the generator shall apply the existing duplicate
  declaring-node, ambiguous-name, duplicate-request, unknown-function,
  unsupported-capability and failed-lowering precedence of AC-1, AC-6,
  AC-12, AC-22, AC-23 and AC-24. A failed call-node lowering record wins over
  that item's function-body record. A refused item contributes no schedule
  root or concrete oracle symbol; a healthy sibling keeps its disposition.
- The generator shall bind each call's callee reference to its exact admitted
  V2 function node and each parameter reference to its binding's parameter
  slot. It shall inspect each application in the graph for its V2 operation
  identity, ordered arguments, result type and classifier-specific operand
  types. It shall refuse a graph edge outside the admitted subset, a shared
  expression node, an unbound parameter, a cycle, a mismatched type or an
  unsupported operator before emitting that function's schedule. The source
  V2 graph, never a recursively nested invented wire term, is the sole input.
- The generator shall traverse each admitted body on an explicit heap
  worklist in left-to-right, postorder dependency order. Each node-entry and
  child-edge occurrence receives one stable occurrence ordinal. It shall emit
  one SSA output slot per evaluated occurrence, with operand slots defined
  earlier, and explicit branch/stop instructions at charge boundaries so a
  denied left operand cannot evaluate a later sibling. A callee reference
  selects its function binding; it is not an evaluated value operand.
  Nested calls retain source operand order and the QSL authority's
  charge-before-work order. The schedule records charge *points*, not charge
  amounts or literal outcomes.
- The generator shall charge one work unit before each node entry, child edge,
  emitted instruction and location record, with one counter per function
  spanning analysis and emission. It shall use at most constant heap storage
  per charged unit and no native call-stack frames proportional to graph size
  or depth, including rendering and release. The first denied action shall
  refuse the function's items as `GenerationWorkExhausted { limit,
  consumed }` before the action or partial publication; `consumed` is the
  first denied count and the refusal names its stable setting. The `u64`
  setting `u64::MAX` is invalid and shall
  return `InvalidGenerationWorkLimit` before lowering or charging; admitted
  settings end at `u64::MAX - 1`, so their first denied count fits `u64`.
- The generator shall charge every generated Rust source byte against the
  effective caller-selected byte limit before append. At the first denied
  byte it shall return the existing whole-request `SourceTooLarge` error,
  extended to name the effective limit, needed byte count and setting, and publish no
  artifact. The default remains 1,048,576 bytes. Flat instructions and
  constant-size indices shall keep output linear in instruction and operand
  counts; when one file would exceed the publication artifact byte limit,
  the generator shall use deterministic ordinal-range modules. The bundle byte limit
  remains separately enforced, so a caller cannot bypass publication bounds
  by raising the source limit.
- For each call point, the generator shall record only function origin,
  occurrence ordinal, parent ordinal, child ordinal and instruction slot.
  Each record shall use constant storage and the positional encoding above.
  The generator shall reconstruct a requested
  root-to-point path by following parent links iteratively, charging every
  link and output byte against caller-selected query work and byte limits;
  a denied query shall return its limit refusal and no partial path. The
  compact map shall preserve the same origin and child-index path that the
  legacy full-path map represented for shallow cases (AC-15/AC-17).
- The generator shall order claims by the call node id, then applied
  function's declaring node id, then argument source node ids, with the
  function name as the final byte-wise tie-break for equal earlier keys.
  Distinct unknown names and duplicate-node pair members on one call node
  retain the separate dispositions and exact precedence AC-22 and AC-24
  specify. Reordering the request shall not change source or claim bytes.
- The schedule crate shall contain no `unwrap`, `expect`, panicking index,
  `unreachable!`, `panic!`, `todo!` or `unimplemented!`, no literal charge
  amount and no literal `Outcome`/`Value` standing in for an owner result.
  The concrete owner-path emitter shall preserve this property when it lands.
  Unknown future owner outcomes shall refuse through the owner API's checked
  invariant cause rather than panic; this is planned pending that API.
- Concrete package admission, `CheckMode::Linked`, input-refusal forwarding,
  capability negotiation against registered backends, execution outcomes,
  meter charge sequences and comparison with QSL are planned owner-path
  obligations. AC-2 through AC-5, AC-8, AC-9, AC-14, AC-16 through AC-20
  and their TC-031 vectors remain retained as target-neutral expected
  behavior; their present Contract Runtime `exact` witnesses are historical.
  When QSL-358/IR-349 establishes its callable API, the concrete emitter shall
  target that QSL-owned API. The
  generator shall not emit or retain `quire_contract_runtime::exact` paths,
  nor an alias, wrapper or compatibility layer for them.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-021-AC-1 | Every requested `call` item receives exactly one scheduled or typed-refused disposition; a refused item contributes no instruction root, concrete symbol or location entry, while an unrelated admitted item is unchanged. Concrete generated disposition is planned after QSL-358/IR-349. | Test (TC-031) |
| FR-021-AC-2 | For each function-application corpus vector, the eventual QSL-owner-path oracle outcome, admitted charge sequence and consumed counters equal a direct application through the independently assembled QSL-owned package and fresh meter. This migrates the current native-runtime comparison without retaining its `exact` path. PLANNED (QSL-358/IR-349). | Test (TC-031) |
| FR-021-AC-3 | Each scheduled item records the requested function name and declaring V2 function node id, plus its request-order function index; the independent application in AC-2 is constructed from the request, never read back from generated code. The owner API origin mapping is verified when that API is available. PLANNED (QSL-358/IR-349 for origin mapping). | Test (TC-031) |
| FR-021-AC-4 | The eventual oracle forwards the QSL-owned arity, wrong-value-kind, dangling-reference and unknown-function input refusals unchanged before any execution charge; the schedule records ordered arguments without inventing a refusal result. PLANNED (QSL-358/IR-349). | Test (TC-031) |
| FR-021-AC-5 | The schedule orders arity validation before per-argument validation, validates arguments in declared parameter order, then places `function.call` before body work; the eventual QSL-owner-path execution admits exactly that order. PLANNED (QSL-358/IR-349 for execution). | Test (TC-031) |
| FR-021-AC-6 | A function whose declared operator requirements name a capability no registered backend can discharge is marked `unsupported` at generation time, naming the capability, before any item naming it is applied; no oracle is emitted for such an item, and the disposition is never an `Outcome` variant, never an `InputRefusal`, and takes no `Meter`. | Test (TC-031) |
| FR-021-AC-7 | A tree-shaped 100,000-node V2 `expression`/`call` graph whose nested children are separate admitted nodes reached by `reference` produces a flat SSA instruction table on a 512 KiB stack when caller work and source-byte limits fit it; no CG analysis or emission depth cap participates. The schedule crate compiles. Running the concrete oracle against the QSL-owned iterative evaluator is a separate planned leg after QSL-358/IR-349; no Contract Runtime `MAX_CALL_DEPTH` behavior is required. | Test (TC-031) |
| FR-021-AC-8 | The schedule marks only linked-package admission as applicable and never marks a kernel-mode package applicable. Concrete mode admission is checked through the QSL-owned API when available. PLANNED (QSL-358/IR-349 for concrete admission). | Inspection (TC-031) |
| FR-021-AC-9 | The schedule preserves a `function.call` charge point and an incomplete stop before body work; the eventual oracle returns the QSL-owned incomplete outcome at a denied charge without applying it. PLANNED (QSL-358/IR-349 for execution). | Test (TC-031) |
| FR-021-AC-10 | A declared parameter or result type reaching a `reference` composite form is refused as blocked on quire-spec-language#120, with no schedule root or concrete code for that function. The eventual oracle admits no meter charge for it. PLANNED (QSL-358/IR-349 for runtime charge assertion). | Test (TC-031) |
| FR-021-AC-11 | Model and relation nodes are refused as blocked on quire-spec-language#120, and state, temporal and protocol nodes as blocked on quire-spec-language#121, each blocker distinct and neither reported as scheduled. | Test (TC-031) |
| FR-021-AC-12 | A declared function whose own body fails to lower — an unlowerable node, a form none of this generator's classifiers admits, or a nested `call` naming a function absent from the request — refuses every item bound to that function with a typed reason, without changing an unrelated package's items. | Test (TC-031) |
| FR-021-AC-13 | Schedule source and claim-map bytes are identical across runs and request permutations; entries order by call-node id, declaring function-node id, then source argument-node ids, with the name tie-break AC-24 specifies. | Test (TC-031) |
| FR-021-AC-14 | The schedule crate declares `publish = false`, compiles without a Contract Runtime `exact` dependency, contains no charge amount or literal outcome/value standing in for owner execution, and stays within caller-selected source bytes and publication artifact limits. Concrete QSL-owner-path emission is PLANNED (QSL-358/IR-349). | Test (TC-031) |
| FR-021-AC-15 | Each scheduled call point has one constant-size location record with function origin, stable occurrence ordinal, parent ordinal, child ordinal and instruction slot. A charged query reconstructs the same path as an independent walk of the request V2 reference graph; no record embeds the entire root-to-point path. A 100,000-link chain uses O(100,000) map storage and generation work. | Test (TC-031) |
| FR-021-AC-16 | The schedule and its claim/location maps neither read nor depend on dynamic invocation `location` or `losses` fields. The eventual QSL-owner-path oracle makes no claim that those fields are populated; concrete behavior is PLANNED (QSL-358/IR-349). | Inspection (TC-031) |
| FR-021-AC-17 | Each location record uses the request function binding and function-body ordering for its origin; when the QSL-owned checker publishes an origin for that function, a check refusal from an independently assembled package matches that origin. PLANNED (QSL-358/IR-349 for owner-authored cross-check). | Test (TC-031) |
| FR-021-AC-18 | For every function-application corpus vector, the eventual oracle agrees in outcome, refusal and charge order with the QSL-owned application authority, independently assembled from the request. PLANNED (QSL-358/IR-349). | Test (TC-031) |
| FR-021-AC-19 | The emitted schedule crate for scalar, equality and nested-call corpus cases contains zero `.unwrap(`, `.expect(`, `unreachable!`, `panic!`, `todo!` and `unimplemented!` tokens; the eventual owner-path oracle has the same no-panic property. PLANNED (QSL-358/IR-349 for concrete oracle). | Test (TC-031) |
| FR-021-AC-20 | For each scalar and equality instruction, the target-neutral schedule preserves completed, undefined, refused, incomplete and checked-invariant result distinctions without folding one into another. The eventual QSL-owner-path emitter exhaustively forwards its owner result variants and maps an unknown future variant to the owner checked-invariant refusal rather than panicking. PLANNED (QSL-358/IR-349 for concrete match). | Test (TC-031) |
| FR-021-AC-21 | A scalar `Negate` body is refused as `UnsupportedOperator` with no schedule root; the generator contains zero `unreachable!`, `panic!`, `todo!` and `unimplemented!` invocations in `src/oracle/function/mod.rs`, excluding comments. | Test (TC-031) |
| FR-021-AC-22 | When two or more declarations share one declaring node id, none of them appears in the instruction table or in `location-map.json`, and every item naming one of them is refused with `ExactFunctionRefusal::DuplicateDeclaringNode { node_id }` (the shared node id; the smallest in node-id order when the item's name is held by duplicate groups on several node ids) before Stage 1 classification: never `UnknownFunction`, never `AmbiguousFunctionName` (the node-id refusal takes precedence when the declarations also share a name), never `DuplicateRequest` (two items on one `call` node naming different members, or one such item requested twice, each carry the node-id refusal), and no claim-map entry records another function's name, schedule root or function index. A declaration with its own distinct node id that shares a name with one of them is absent from the instruction table and `location-map.json`. A declared function whose nested `call` names such a function is refused as `UnknownCallee`. The refusal and these outputs are identical under every permutation of the request order. Every item naming a function with a distinct declaring node id, and not a duplicate name, has a claim-map entry equal to the one the same request produces with the duplicate declarations removed. | Test (TC-031) |
| FR-021-AC-23 | A `failed` lowering record is refused as `ExactFunctionRefusal::LoweringWorkExhausted` for the `work` limit, as `LoweringByteLimitExceeded { limit, consumed }` with the record's `limit` and `consumed` for the `bytes` limit (Contract IR FR-038-AC-95), and as `LoweringLimitUnrecognised` with `limit_kind` the snake_case name FR-014 states for each of `nodes`, `edges`, `occurrences` and `diagnostics`, never as another arm's refusal and without a panic. The mapping is asserted on hand-built `Failed` records given to `lowered_binary_body`, the one function that maps a record to a refusal, in a `#[cfg(test)]` module the code change adds to the function module (it has none today), so a mapping of an unrecognised kind to `LoweringWorkExhausted` in this module fails it. That a body failure refuses only that function's items, and leaves an unrelated function's items unchanged, is FR-021-AC-12's and is asserted by its existing `tc_031_ac12_*` tests, not re-asserted on hand-built records, because the isolation lives in the classification loop that takes its records from `lower`. Through a whole call of the function generator whose input is read under a byte ceiling that admits the checked package and is one byte below the shorter of the canonical lengths of the two lowered contract packages of that call (the one lowered for the function bodies and the one lowered for the requested `call` nodes; the fixture makes both longer than the checked package), every body record and every call-node record fails for bytes, so every function is absent and every item is refused as `LoweringByteLimitExceeded` with `limit` equal to that ceiling and `consumed` equal to the call-node lowering's, which the fixture makes differ from the body lowering's `consumed` (read by lowering each request alone under the same ceiling), so the call-node-first order of the Behavior section is asserted through the call. | Test (TC-031) |
| FR-021-AC-24 | Items on one `call` node, with equal argument operands, that name functions absent from the request's declarations each get their own claim-map entry. Requesting `zz_unknown` and `aa_unknown` on one node yields two entries, in that order: `UnknownFunction { name: "aa_unknown" }`, then `UnknownFunction { name: "zz_unknown" }` (function name compared byte-wise over its UTF-8 bytes, case-sensitively), and no `DuplicateRequest`; the entries are identical under both request orders. Requesting `Zz_unknown` and `aa_unknown` yields `Zz_unknown` first, because `Z` (0x5A) sorts before `a` (0x61). Requesting `zz_unknown` twice on one node still yields one entry, `DuplicateRequest`. Requesting a declared `add_fn` and `zz_unknown` on one node yields two entries: `UnknownFunction { name: "zz_unknown" }` ordered before the entry generated for `add_fn` (an item naming no declared function ranks before one that does), identical to the entries each item gets when requested alone. Requesting `zz_unknown` once yields `UnknownFunction { name: "zz_unknown" }` as before. Items on one `call` node naming the two members of a duplicate-node pair are two entries ordered by function name in the same byte order: with `m_multi` and `z_pair` declared on node id N2 and `m_multi` and `q_extra` declared on the smaller node id N1, requesting `m_multi` and `z_pair` yields `DuplicateDeclaringNode { node_id: N1 }` (for `m_multi`, whose name the smaller group also holds), then `DuplicateDeclaringNode { node_id: N2 }` (for `z_pair`), under both request orders; with `z_multi` and `a_pair` in place of `m_multi` and `z_pair` the order is `{ N2 }` (for `a_pair`) then `{ N1 }` (for `z_multi`). The name orders only items equal on the call node, declaring node id and arguments, so items whose declaring node ids differ are ordered by declaring node id first: with `a_pair` and `z_pair` declared on N2, `z_pair` and `q_extra` on N1, and `a_pair` alone on N3 (N1 < N2 < N3), requesting `a_pair` and `z_pair` yields `DuplicateDeclaringNode { node_id: N1 }` (for `z_pair`), then `DuplicateDeclaringNode { node_id: N2 }` (for `a_pair`), under both request orders, because `a_pair` resolves to the larger N3 and `z_pair` to N2. No new refusal variant is added. | Test (TC-031) |
| FR-021-AC-25 | Across one body, the counter charges each node entry, child edge, emitted instruction and location record once in source traversal order. A setting equal to the required units emits a complete schedule; one less refuses that function as `GenerationWorkExhausted { limit, consumed: limit + 1 }` before the denied action, with no partial root or location record and no change to an unrelated function. A failed Contract IR lowering record keeps its lowering refusal and incurs no generation charge. PLANNED (IR-511). | Test (TC-031) |
| FR-021-AC-26 | A denied left-operand charge stops the eventual flat execution before the right operand charge; after the left completes, the right charge precedes the parent operation. The target-neutral schedule contains explicit branch/stop edges in that order, and eventual QSL-owner-path execution matches an equivalent shallow body. PLANNED (QSL-358/IR-349 for execution). | Test (TC-031) |
| FR-021-AC-27 | An admitted nested body consists of separate V2 expression nodes connected only by `reference` leaves under QSpec FR-322, with each expression node reached from one parent in this CG subset; callee argument 0 binds an exact V2 function node, parameter references bind the declared parameter slots, and remaining arguments preserve source order. Missing, cyclic, shared-expression, unbound-parameter, ill-typed or wrong-operation edges refuse with a typed cause before schedule emission. PLANNED (IR-511). | Test (TC-031) |
| FR-021-AC-28 | Under the default 1,048,576-byte source limit, a 100,000-instruction body that needs more bytes returns `SourceTooLarge` naming the default; raising the caller limit to its measured flat source bytes emits a complete compilable schedule crate, with deterministic ordinal-range modules if one artifact would exceed its publication limit. One byte below the measured count refuses before append with no partial output. PLANNED (IR-511). | Test (TC-031) |
| FR-021-AC-29 | A 100,000-call linear body produces 100,000 constant-size parent-linked location records; requesting its deepest path under sufficient query limits returns exactly 100,000 child ordinals, and a one-unit-below query limit refuses without a partial path. Generation does not materialize all paths. PLANNED (IR-511). | Test (TC-031) |
| FR-021-AC-30 | A requested generation work limit of `u64::MAX` returns `InvalidGenerationWorkLimit` before lowering or charging; at `u64::MAX - 1` the first denied count is representable in the `u64` refusal and never wraps or panics. PLANNED (IR-511). | Test (TC-031) |
| FR-021-AC-31 | Once QSL-358/IR-349 establishes the callable owner API, the 100,000-node generated oracle compiles and runs on a 512 KiB stack under raised caller work and source-byte limits, agreeing with an independently constructed shallow equivalent in outcome and admitted charges; no fixed depth refusal occurs. This is a distinct PLANNED execution gate, not evidence from the schedule-only crate. | Test (TC-031) |
| FR-021-AC-32 | Neither the schedule crate nor eventual concrete oracle requires `quire_contract_runtime::exact`, an alias, wrapper or compatibility layer; owner-path emission waits for QSL-358/IR-349 and preserves AC-2 through AC-5, AC-8, AC-9, AC-14 and AC-16 through AC-20 against the QSL-owned API. PLANNED (QSL-358/IR-349 for concrete oracle). | Inspection (TC-031) |

### Mutations these criteria detect

Each criterion is recorded with the mutation it exists to catch; a criterion without one is not
written.

| ID | Mutation that breaks it |
|----|-------------------------|
| FR-021-AC-1 | Abort the whole request on one refusal or assign a schedule root to a refused item. |
| FR-021-AC-2 | Compare only with the former runtime port or build the direct QSL application from emitted source, so two matching errors pass. |
| FR-021-AC-3 | Read function identity from emitted source, so a mutated schedule and claim map agree falsely. |
| FR-021-AC-4 | Collapse distinct owner input refusals into one generic cause after the API migration. |
| FR-021-AC-5 | Validate arguments in a different order or schedule `function.call` after body work. |
| FR-021-AC-6 | Schedule an unsupported-capability function as applicable or defer its refusal to a meter charge. |
| FR-021-AC-7 | Keep a fixed depth ceiling, recursively traverse the V2 graph, or emit nested Rust expressions; the deep compile case refuses or overflows. |
| FR-021-AC-8 | Mark a kernel-mode package applicable or omit the owner admission check. |
| FR-021-AC-9 | Turn a denied `function.call` into a completed value or apply the denied charge. |
| FR-021-AC-10 | Schedule a function with a reference-typed signature instead of its typed blocker. |
| FR-021-AC-11 | Merge distinct upstream blockers or schedule a model, state, temporal or protocol node. |
| FR-021-AC-12 | Insert a stub instruction for a body whose lowering or classifier failed. |
| FR-021-AC-13 | Order claims by function name alone or by request position, changing output under permutation. |
| FR-021-AC-14 | Emit a hardcoded charge or result value, require the deleting Runtime `exact` dependency, or produce a crate that does not compile. |
| FR-021-AC-15 | Store every full root-to-node path or an empty path at every call point; the former is quadratic and the latter loses source position. |
| FR-021-AC-16 | Read dynamic location or losses as if the eventual QSL owner populated them before its API states that contract. |
| FR-021-AC-17 | Take function origin from request ordinal or emitted source instead of the binding, allowing claim and source to drift together. |
| FR-021-AC-18 | Compare the emitted oracle only with its own schedule and miss a divergence from the QSL-owned application authority. |
| FR-021-AC-19 | Leave a panic token in schedule source or a future owner-path result match. |
| FR-021-AC-20 | Fold an unknown owner result into a completed value or an unrelated refusal. |
| FR-021-AC-21 | Allow `Negate` to reach an unreachable arm instead of refusing it before schedule emission. |
| FR-021-AC-22 | Let duplicate declaring-node ids survive into the instruction table, collapse pair members, or choose a request-order-dependent refusal. |
| FR-021-AC-23 | Keep the single `Failed` arm that reports every failure as `LoweringWorkExhausted`, so a byte-ceiling failure reads as work exhaustion; or leave a `_ => LoweringWorkExhausted` or `_ => unreachable!(..)` arm for an unrecognised kind; or report the function's body-lowering `consumed` for an item whose call-node lowering failed first. |
| FR-021-AC-24 | Key an item by the call node, applied function's declaring node id and arguments only, so every unknown name on one call node shares one key and the second name is reported as `DuplicateRequest` and lost (the `zz_unknown` and `aa_unknown` example fails); or key an item by its request position, so the same unknown name requested twice yields two entries (the `zz_unknown` twice example fails); or put the name ahead of the declaring node id in the key, so `add_fn` sorts before `zz_unknown` (the `add_fn` and `zz_unknown` example fails); or compare names case-insensitively, so `aa_unknown` sorts before `Zz_unknown` (the `Zz_unknown` example fails); or break ties by request position, so the order changes with the request order (the both-orders check fails, and so does the duplicate-node pair example, whose order is `{ N1 }`, `{ N2 }` for `m_multi` and `z_pair` and reversed for `z_multi` and `a_pair`). |
| FR-021-AC-25 | Skip instruction or location charges, check after emission, or expose a partial instruction root on refusal. |
| FR-021-AC-26 | Compute a right operand before testing the left stop condition; a denied left charge then admits a right charge. |
| FR-021-AC-27 | Treat a nested application as an inline recursive V2 term, bind callee by name alone, or accept an unbound/shared/cyclic expression edge. |
| FR-021-AC-28 | Keep the fixed 1 MiB source ceiling or check it only after publishing; the raised-limit deep source refuses or leaks partial output. |
| FR-021-AC-29 | Copy a full path into every record or fail to charge path expansion; the deep map grows quadratically or bypasses query limits. |
| FR-021-AC-30 | Accept `u64::MAX` and increment a `u64` at the first denied charge; the refusal wraps or panics. |
| FR-021-AC-31 | Count schedule-source compilation as execution evidence; an owner-path codegen or stack failure remains undetected. |
| FR-021-AC-32 | Keep the deleted Contract Runtime `exact` port reachable through a re-export, wrapper or compatibility layer. |

## Dependencies

- **Upstream**: [FR-014](./FR-014-exact-scalar-oracles.md),
  [FR-018](./FR-018-composite-equality-oracles.md), Contract IR FR-036/FR-038
  (admitted CheckedPackage V2 lowering), QSpec FR-322 (flat V2 body grammar)
  and FR-460 (no depth caps).
- **Owner API pending**: QSL-358 and Contract Runtime FR-275/IR-349 settle the
  QSL-owned callable, admission, location and result surfaces for concrete
  Rust oracle emission. This requirement makes no path choice ahead of them.
- **Downstream**: [TC-031](../matrix/TC-031-function-application-oracles.md).

## Status

The target-neutral graph analysis, flat schedule, compact location map and
caller limits are planned in IR-511. The current implementation and existing
TC-031 corpus still witness the former Contract Runtime `exact` port. Its
behavioral obligations are retained by AC-2 through AC-5, AC-8, AC-9,
AC-14 and AC-16 through AC-20, but their concrete execution and source
assertions are planned for QSL-358/IR-349 owner paths. The historical port
is not the target architecture and no compatibility surface is specified.
TC-031 records separate schedule-compilation and owner-execution gates.

## Out of Scope

- Dynamic invocation location and loss reporting, whose producer and return
  types belong to the QSL-owned application API.
- Standalone expression evaluation outside a named function application.
- Counterexample replay, model graph, identity, reachability, temporal and
  protocol oracles.
- Constructing a function declaration without a lowerable admitted body.
