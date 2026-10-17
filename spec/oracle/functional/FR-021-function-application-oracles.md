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
QSL exports no callable application API through qsl-replay (IR-845), and QSL
arch-lint T12-A bars CG from qsl-eval, so CG claims no QSL application API.
Concrete Rust oracle emission, compilation and execution against a QSL
application API are outside CG's boundary until a concrete need is filed and
QSL specifies that API (IR-845; FR-014-AC-47). The generator shall
not preserve the port through an alias, wrapper or compatibility layer.

This is the function-application slice of complete-V1 oracle generation.
[FR-014](./FR-014-exact-scalar-oracles.md) supplies scalar forms and
[FR-018](./FR-018-composite-equality-oracles.md) supplies equality forms.
FR-018's entry point still refuses `call` nodes; this entry point alone
classifies function application. Runtime outcome and charge amounts are not
decided here, and CG claims no agreement with the QSL value authority on them
(FR-014-AC-47; owned by quire-integration IR-842), while this requirement
decides the schedule's operand and call order.

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
`Evaluation.location`/`.losses` behavior is outside CG's boundary: CG claims
no QSL application API (IR-845).

## Inputs

- The direct `generate_function_schedules` API takes an admitted
  `CheckedPackageV2` read through Contract IR's strict reader. The package-backed
  `generate_exact_function_oracles` API takes an admitted `ContractPackageV1`
  from Contract IR's FR-330 lowerer and strict reader, the caller's complete
  root-position request, and exact positions for its declarations and call
  items. Both use the retained checked-V2 graph; neither adds an inline
  recursive V2 body form. QSpec
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
  Concrete callable Rust is outside this output: CG claims no QSL application
  API (IR-845), and no Contract Runtime `exact` dependency is required by this
  output.
- A typed claim map with one entry per requested item: its existing request
  identity, ordering, package id, source map, reconstructed declaration
  closure, function binding and typed refusal, or a `Scheduled` disposition
  naming the root instruction slot. A concrete oracle symbol is never
  fabricated by a schedule-only result, and CG emits none against a QSL
  application API (IR-845).
- A location map of constant-size parent-linked occurrence records as defined
  above, encoded as deterministic positional arrays with node ids in a shared
  dictionary so long field names and node ids are not repeated per record.
  The generator partitions ordinal ranges when one map artifact would exceed
  its publication byte limit. The record for a call point names its
  instruction slot; its full path is materialized only by a charged query.

## Behavior

- When body analysis begins, the generator shall apply the existing duplicate
  declaring-node, ambiguous-name, duplicate-request, unknown-function,
  unsupported-capability and admitted package-accounting precedence of AC-1,
  AC-6, AC-12, AC-22, AC-23 and AC-24. An admitted failed call-node record
  wins over that item's function-body record; if Contract IR emits no package
  because of its byte ceiling, FR-039 refuses the call before item generation.
  A refused item contributes no schedule
  root or concrete oracle symbol; a healthy sibling keeps its disposition.
- For a declared function whose admitted body root is an `expression`/
  `call` node, the generator shall classify that root once from the checked
  V2 graph, applying the source binding, operation and type rules below and
  AC-27. It shall not submit that root or its transitive callee closure as a
  Contract IR body-lowering request. Scalar and equality body roots and
  requested call-item nodes retain Contract IR lowering; actual `Failed`
  records retain AC-23's typed mapping, per-function isolation and
  call-node-first precedence. This classification shall not bypass the
  duplicate, capability or binding refusals above.
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
  Nested calls retain source operand order and the
  charge-before-work order of FR-021-AC-5. The schedule records charge *points*, not charge
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
- When a directly requested call-node id, or a declared function body's root,
  names an admitted `temporal`/`formula`, `temporal`/`fairness` or
  `expression`/`case` node, the generator shall refuse the affected item as
  `ExactFunctionRefusal::Unsupported`, retaining the requested call-node id
  in its claim and the first unsupported reachable node and tag in the
  refusal. A lowered `case` root shall name itself and `expression` as that
  cause. A directly requested temporal clause shall keep its existing
  quire-spec-language#121 blocker even when its closure reaches a formula or
  fairness node first. The refusal shall create no schedule root, oracle
  symbol or location record for the affected item.
- The schedule crate shall contain no `unwrap`, `expect`, panicking index,
  `unreachable!`, `panic!`, `todo!` or `unimplemented!`, no literal charge
  amount and no literal `Outcome`/`Value` standing in for an owner result.
  Concrete oracle emission and its owner result variants are outside CG's
  boundary (IR-845): no QSL application API exists for CG to match.
- Concrete package admission, `CheckMode::Linked`, input-refusal forwarding,
  capability negotiation against registered backends, execution outcomes,
  and meter charge sequences are outside CG's boundary (IR-845); comparison
  with the QSL value authority is outside CG's boundary (FR-014-AC-47; owned by
  quire-integration IR-842). AC-2 through AC-5, AC-8, AC-9, AC-14, AC-16, AC-17, AC-19 and AC-20
  and their TC-031 vectors remain retained as target-neutral expected
  behavior; their present Contract Runtime `exact` witnesses are historical.
  CG emits no concrete oracle against a QSL application API (IR-845). The
  generator shall not emit or retain `quire_contract_runtime::exact` paths,
  nor an alias, wrapper or compatibility layer for them.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-021-AC-1 | Every requested `call` item receives exactly one scheduled or typed-refused disposition; a refused item contributes no instruction root, concrete symbol or location entry, while an unrelated admitted item is unchanged. Concrete generated disposition is outside CG's boundary (IR-845). | Test (TC-031) |
| FR-021-AC-2 | For each function-application corpus vector, the oracle's outcome, admitted charge sequence and consumed counters equal a direct application independently assembled from the request with a fresh meter; today that application is the Contract Runtime `exact` port (historical), and CG names no QSL application API as its replacement (IR-845). This criterion claims no agreement with the QSL value authority: that boundary is owned by quire-integration IR-842 (FR-014-AC-47, FR-021-AC-18). | Test (TC-031) |
| FR-021-AC-3 | Each scheduled item records the requested function name and declaring V2 function node id, plus its request-order function index; any independent application a test builds for comparison is constructed from the request, never read back from generated code. The owner API origin mapping is verified when that API is available. PLANNED (QSL-358/IR-349 for origin mapping). | Test (TC-031) |
| FR-021-AC-4 | The oracle returns the arity, wrong-value-kind, dangling-reference and unknown-function input refusals unchanged to the caller before any execution charge; the schedule records ordered arguments without inventing a refusal result. That the refusals equal QSL-owned ones is outside CG's boundary (IR-842, FR-014-AC-47). | Test (TC-031) |
| FR-021-AC-5 | The schedule orders arity validation before per-argument validation, validates arguments in declared parameter order, then places `function.call` before body work; owner-path execution of that order is outside CG's boundary (IR-845). | Test (TC-031) |
| FR-021-AC-6 | A function whose declared operator requirements name a capability no registered backend can discharge is marked `unsupported` at generation time, naming the capability, before any item naming it is applied; no oracle is emitted for such an item, and the disposition is never an `Outcome` variant, never an `InputRefusal`, and takes no `Meter`. | Test (TC-031) |
| FR-021-AC-7 | A tree-shaped 100,000-node V2 `expression`/`call` graph whose nested children are separate admitted nodes reached by `reference` produces a flat SSA instruction table on a 512 KiB stack when caller work and source-byte limits fit it; no CG analysis or emission depth cap participates. The schedule crate compiles. Running a concrete oracle against a QSL evaluator is outside CG's boundary (IR-845); no Contract Runtime `MAX_CALL_DEPTH` behavior is required. | Test (TC-031) |
| FR-021-AC-8 | The schedule marks only linked-package admission as applicable and never marks a kernel-mode package applicable. Concrete mode admission through a QSL application API is outside CG's boundary (IR-845). | Inspection (TC-031) |
| FR-021-AC-9 | The schedule preserves a `function.call` charge point and an incomplete stop before body work; the incomplete outcome at a denied charge is not applied; owner-path execution of it is outside CG's boundary (IR-845). | Test (TC-031) |
| FR-021-AC-10 | A declared parameter or result type reaching a `reference` composite form is refused as blocked on quire-spec-language#120, with no schedule root or concrete code for that function. No meter charge is admitted for it; a runtime charge assertion against a QSL API is outside CG's boundary (IR-845). | Test (TC-031) |
| FR-021-AC-11 | Model and relation lowering records map to the quire-spec-language#120 blocker; state, temporal nodes other than directly requested `formula` and `fairness` roots, and protocol lowering records map to quire-spec-language#121; the blockers remain distinct. A requested temporal clause keeps that blocker even if its first unsupported reachable node is a formula or fairness child. TC-031's defensive Model/State classifier case pairs those request roots with call items bound to a different checked source function, so it does not establish applied-call behavior. The source-bound applied-call leg is PLANNED pending a reader-admissible upstream representation. | Test (TC-031, defensive classifier case only); PLANNED (source-bound Model/State applied calls pending reader representation) |
| FR-021-AC-12 | A declared function whose own body fails to lower — an unlowerable node, a form none of this generator's classifiers admits, or a nested `call` naming a function absent from the request — refuses every item bound to that function with a typed reason, without changing an unrelated package's items. | Test (TC-031) |
| FR-021-AC-13 | Schedule source and claim-map bytes are identical across runs and request permutations; entries order by call-node id, declaring function-node id, then source argument-node ids, with the name tie-break AC-24 specifies. | Test (TC-031) |
| FR-021-AC-14 | The schedule crate declares `publish = false`, compiles without a Contract Runtime `exact` dependency, contains no charge amount or literal outcome/value standing in for owner execution, and stays within caller-selected source bytes and publication artifact limits. Concrete owner-path emission is outside CG's boundary (IR-845). | Test (TC-031) |
| FR-021-AC-15 | Each scheduled call point has one constant-size location record with function origin, stable occurrence ordinal, parent ordinal, child ordinal and instruction slot. A charged query reconstructs the same path as an independent walk of the request V2 reference graph; no record embeds the entire root-to-point path. A 100,000-link chain uses O(100,000) map storage and generation work. | Test (TC-031) |
| FR-021-AC-16 | The schedule and its claim/location maps neither read nor depend on dynamic invocation `location` or `losses` fields. No oracle makes a claim that those fields are populated: CG claims no QSL application API (IR-845). | Inspection (TC-031) |
| FR-021-AC-17 | Each location record uses the request function binding and function-body ordering for its origin; the cross-check against an origin published by a QSL-owned checker is outside CG's boundary (IR-845). | Test (TC-031) |
| FR-021-AC-18 | No test name, trace tag, test output line or doc comment of the function-application corpus, no TC-031 step and no oracle-matrix row claims, or presents as evidence, agreement in outcome, refusal, charge order or consumed counters with the QSL-owned application authority; text that states this boundary is permitted. That agreement is not claimed by CG and is owned by quire-integration IR-842 (FR-014-AC-47). | Inspection (TC-031) |
| FR-021-AC-19 | The emitted schedule crate for scalar, equality and nested-call corpus cases contains zero `.unwrap(`, `.expect(`, `unreachable!`, `panic!`, `todo!` and `unimplemented!` tokens; no concrete owner-path oracle exists to check (IR-845). | Test (TC-031) |
| FR-021-AC-20 | For each scalar and equality instruction, the target-neutral schedule preserves completed, undefined, refused, incomplete and checked-invariant result distinctions without folding one into another. Forwarding a QSL application API's owner result variants is outside CG's boundary (IR-845). | Test (TC-031) |
| FR-021-AC-21 | A scalar `Negate` body is refused as `UnsupportedOperator` with no schedule root, whether or not it reaches a `quire.op.quantity.convert` node (a conversion is not refused by name, because FR-021 has no quantity body, operand or argument support: the generator refuses a body that is one as `UnsupportedOperator` and the schedules as `ScheduleRefusal::UnsupportedOperation`; an earlier check that fires first is the accepted result: `Unsupported` for a direct unsupported form of the root, `NotExpression` and the other root checks, `UnsupportedOperandType` for a quantity-typed parameter or result, and the argument's own refusal for a call argument, FR-014-AC-2); the generator contains zero `unreachable!`, `panic!`, `todo!` and `unimplemented!` invocations in `src/oracle/function/mod.rs`, excluding comments. | Test (TC-031) |
| FR-021-AC-22 | When two or more declarations share one declaring node id, none of them appears in the instruction table or in `location-map.json`, and every item naming one of them is refused with `ExactFunctionRefusal::DuplicateDeclaringNode { node_id }` (the shared node id; the smallest in node-id order when the item's name is held by duplicate groups on several node ids) before Stage 1 classification: never `UnknownFunction`, never `AmbiguousFunctionName` (the node-id refusal takes precedence when the declarations also share a name), never `DuplicateRequest` (two items on one `call` node naming different members, or one such item requested twice, each carry the node-id refusal), and no claim-map entry records another function's name, schedule root or function index. A declaration with its own distinct node id that shares a name with one of them is absent from the instruction table and `location-map.json`. A declared function whose nested `call` names such a function is refused as `UnknownCallee`. The refusal and these outputs are identical under every permutation of the request order. Every item naming a function with a distinct declaring node id, and not a duplicate name, has a claim-map entry equal to the one the same request produces with the duplicate declarations removed. | Test (TC-031) |
| FR-021-AC-23 | For an admitted FR-330 package carrying a `Failed` feature-accounting record, a function item retains that exact typed record, its `limit_kind`, `limit`, `consumed` and source occurrences in `PackageAccounting`, without relabelling bytes as work exhaustion. Under a ceiling one byte below the shorter of the separately measured body-root and call-root package lengths, Contract IR FR-038-AC-95 returns `Failed(Bytes)` for every root in each request, one shared `consumed` per request above the ceiling, and neither request exposes canonical package bytes or `wire()`. The two measured `consumed` values differ. With no admitted package, FR-039 refuses generation before CG emits any function claim or artifact. At a sufficient ceiling, the combined body-and-call lowering has a canonical package with every root `Lowered`; its larger size is not substituted for either isolated measured length. | Test (TC-031) |
| FR-021-AC-24 | Items on one `call` node, with equal argument operands, that name functions absent from the request's declarations each get their own claim-map entry. Requesting `zz_unknown` and `aa_unknown` on one node yields two entries, in that order: `UnknownFunction { name: "aa_unknown" }`, then `UnknownFunction { name: "zz_unknown" }` (function name compared byte-wise over its UTF-8 bytes, case-sensitively), and no `DuplicateRequest`; the entries are identical under both request orders. Requesting `Zz_unknown` and `aa_unknown` yields `Zz_unknown` first, because `Z` (0x5A) sorts before `a` (0x61). Requesting `zz_unknown` twice on one node still yields one entry, `DuplicateRequest`. Requesting a declared `add_fn` and `zz_unknown` on one node yields two entries: `UnknownFunction { name: "zz_unknown" }` ordered before the entry generated for `add_fn` (an item naming no declared function ranks before one that does), identical to the entries each item gets when requested alone. Requesting `zz_unknown` once yields `UnknownFunction { name: "zz_unknown" }` as before. Items on one `call` node naming the two members of a duplicate-node pair are two entries ordered by function name in the same byte order: with `m_multi` and `z_pair` declared on node id N2 and `m_multi` and `q_extra` declared on the smaller node id N1, requesting `m_multi` and `z_pair` yields `DuplicateDeclaringNode { node_id: N1 }` (for `m_multi`, whose name the smaller group also holds), then `DuplicateDeclaringNode { node_id: N2 }` (for `z_pair`), under both request orders; with `z_multi` and `a_pair` in place of `m_multi` and `z_pair` the order is `{ N2 }` (for `a_pair`) then `{ N1 }` (for `z_multi`). The name orders only items equal on the call node, declaring node id and arguments, so items whose declaring node ids differ are ordered by declaring node id first: with `a_pair` and `z_pair` declared on N2, `z_pair` and `q_extra` on N1, and `a_pair` alone on N3 (N1 < N2 < N3), requesting `a_pair` and `z_pair` yields `DuplicateDeclaringNode { node_id: N1 }` (for `z_pair`), then `DuplicateDeclaringNode { node_id: N2 }` (for `a_pair`), under both request orders, because `a_pair` resolves to the larger N3 and `z_pair` to N2. No new refusal variant is added. | Test (TC-031) |
| FR-021-AC-25 | Across one body, the counter charges each node entry, child edge, emitted instruction and location record once in source traversal order. A setting equal to the required units emits a complete schedule; one less refuses that function as `GenerationWorkExhausted { limit, consumed: limit + 1 }` before the denied action, with no partial root or location record and no change to an unrelated function. A failed Contract IR lowering record keeps its lowering refusal and incurs no generation charge. PLANNED (IR-511). | Test (TC-031) |
| FR-021-AC-26 | A denied left-operand charge stops the flat execution before the right operand charge; after the left completes, the right charge precedes the parent operation. The target-neutral schedule contains explicit branch/stop edges in that order, and owner-path execution matching an equivalent shallow body is outside CG's boundary (IR-845). | Test (TC-031) |
| FR-021-AC-27 | An admitted nested body consists of separate V2 expression nodes connected only by `reference` leaves under QSpec FR-322, with each expression node reached from one parent in this CG subset; callee argument 0 binds an exact V2 function node, parameter references bind the declared parameter slots, and remaining arguments preserve source order. Missing, cyclic, shared-expression, unbound-parameter, ill-typed or wrong-operation edges refuse with a typed cause before schedule emission. PLANNED (IR-511). | Test (TC-031) |
| FR-021-AC-28 | Under the default 1,048,576-byte source limit, a 100,000-instruction body that needs more bytes returns `SourceTooLarge` naming the default; raising the caller limit to its measured flat source bytes emits a complete compilable schedule crate, with deterministic ordinal-range modules if one artifact would exceed its publication limit. One byte below the measured count refuses before append with no partial output. PLANNED (IR-511). | Test (TC-031) |
| FR-021-AC-29 | A 100,000-call linear body produces 100,000 constant-size parent-linked location records; requesting its deepest path under sufficient query limits returns exactly 100,000 child ordinals, and a one-unit-below query limit refuses without a partial path. Generation does not materialize all paths. PLANNED (IR-511). | Test (TC-031) |
| FR-021-AC-30 | A requested generation work limit of `u64::MAX` returns `InvalidGenerationWorkLimit` before lowering or charging; at `u64::MAX - 1` the first denied count is representable in the `u64` refusal and never wraps or panics. PLANNED (IR-511). | Test (TC-031) |
| FR-021-AC-31 | No test name, trace tag, test output line or doc comment of the 100,000-node case, no TC-031 step and no oracle-matrix row claims, or presents as evidence, that a generated oracle compiled, ran on a 512 KiB stack or agreed with a shallow equivalent against a QSL application API; schedule-source compilation (AC-7) is not execution evidence. CG claims no QSL application API (IR-845, FR-014-AC-47). | Inspection (TC-031) |
| FR-021-AC-32 | Neither the schedule crate nor any concrete oracle requires `quire_contract_runtime::exact`, an alias, wrapper or compatibility layer; CG emits no owner-path oracle (IR-845). | Inspection (TC-031) |
| FR-021-AC-33 | With an IR-admitted `temporal`/`formula`, `temporal`/`fairness` or `expression`/`case` node used directly as an item's call-node id, the claim-map entry retains that id and is `Refused { refusal: ExactFunctionRefusal::Unsupported }`. For the separate declared-body-root case, an item names that function through a distinct, IR-admitted `expression`/`call` node that lowers successfully and has no duplicate, capability, or earlier item refusal; its claim retains the call-node id and carries the body's `ExactFunctionRefusal::Unsupported`. For an IR `Unsupported` lowering record the refusal's `unsupported_node_id` and `node_tag` match its first unsupported reachable node and tag, which can differ from either request root; for a lowered `case` root they name that root and `"expression"`. No affected item is scheduled, given an oracle symbol or location entry, omitted, or panicked, and an unrelated healthy function/item's claim, schedule source and location map are byte-identical to a run without it. AC-11's temporal-clause blocker and existing duplicate/capability/lowering precedence remain unchanged. | Test (TC-031) |
| FR-021-AC-34 | Given one authentic strict-reader-admitted `CheckedPackageV2` of less than 1,048,576 bytes containing 129 distinct functions with two bound Integer parameters each, 128 acyclic `quire.op.function.call` body-root edges forming one chain and a terminating admitted Scalar body, the public function generator schedules all 129 functions and all 128 call edges from that same checked graph. Use the default 1,048,576-byte Contract IR reader/lowering ceilings and default 1,048,576-byte generated-source ceiling; only the caller-selected generation work limit may be raised to the measured required units. The requested chain item has exactly one `Scheduled` claim naming its root instruction slot, the generated flat SSA schedule contains 128 call instructions with ordered two-argument operands and compiles within the default source limit, no item is refused with `LoweringByteLimitExceeded`, and no `SourceTooLarge` error occurs. Source, claims and location map are byte-identical under request permutation. AC-23's real Scalar/Equality and requested call-item `Failed` mappings, AC-12's isolation, call-node-first precedence and AC-27's source-binding refusals still apply. PLANNED (IR-768). | Test (TC-031) |

### Mutations these criteria detect

Each criterion is recorded with the mutation it exists to catch; a criterion without one is not
written.

| ID | Mutation that breaks it |
|----|-------------------------|
| FR-021-AC-1 | Abort the whole request on one refusal or assign a schedule root to a refused item. |
| FR-021-AC-2 | Compare only with the former runtime port or build the direct application from emitted source, so two matching errors pass. |
| FR-021-AC-3 | Read function identity from emitted source, so a mutated schedule and claim map agree falsely. |
| FR-021-AC-4 | Collapse distinct input refusals into one generic cause. |
| FR-021-AC-5 | Validate arguments in a different order or schedule `function.call` after body work. |
| FR-021-AC-6 | Schedule an unsupported-capability function as applicable or defer its refusal to a meter charge. |
| FR-021-AC-7 | Keep a fixed depth ceiling, recursively traverse the V2 graph, or emit nested Rust expressions; the deep compile case refuses or overflows. |
| FR-021-AC-8 | Mark a kernel-mode package applicable. |
| FR-021-AC-9 | Turn a denied `function.call` into a completed value or apply the denied charge. |
| FR-021-AC-10 | Schedule a function with a reference-typed signature instead of its typed blocker. |
| FR-021-AC-11 | Merge distinct upstream blockers or schedule a model, state, temporal or protocol node. |
| FR-021-AC-12 | Insert a stub instruction for a body whose lowering or classifier failed. |
| FR-021-AC-13 | Order claims by function name alone or by request position, changing output under permutation. |
| FR-021-AC-14 | Emit a hardcoded charge or result value, require the deleting Runtime `exact` dependency, or produce a crate that does not compile. |
| FR-021-AC-15 | Store every full root-to-node path or an empty path at every call point; the former is quadratic and the latter loses source position. |
| FR-021-AC-16 | Read dynamic location or losses as if a QSL application API populated them. |
| FR-021-AC-17 | Take function origin from request ordinal or emitted source instead of the binding, allowing claim and source to drift together. |
| FR-021-AC-18 | Claim or present as evidence agreement with the QSL-owned application authority in a test name, trace tag, output line, doc comment, TC-031 step or matrix row. |
| FR-021-AC-19 | Leave a panic token in schedule source. |
| FR-021-AC-20 | Fold one result distinction (completed, undefined, refused, incomplete, checked-invariant) into another in the schedule. |
| FR-021-AC-21 | Allow `Negate` to reach an unreachable arm instead of refusing it before schedule emission. |
| FR-021-AC-22 | Let duplicate declaring-node ids survive into the instruction table, collapse pair members, or choose a request-order-dependent refusal. |
| FR-021-AC-23 | Mislabel an admitted `Failed(Bytes)` record as work exhaustion, or synthesize a ContractPackage when Contract IR's byte ceiling returns no package so CG fabricates claims or artifacts. |
| FR-021-AC-24 | Key an item by the call node, applied function's declaring node id and arguments only, so every unknown name on one call node shares one key and the second name is reported as `DuplicateRequest` and lost (the `zz_unknown` and `aa_unknown` example fails); or key an item by its request position, so the same unknown name requested twice yields two entries (the `zz_unknown` twice example fails); or put the name ahead of the declaring node id in the key, so `add_fn` sorts before `zz_unknown` (the `add_fn` and `zz_unknown` example fails); or compare names case-insensitively, so `aa_unknown` sorts before `Zz_unknown` (the `Zz_unknown` example fails); or break ties by request position, so the order changes with the request order (the both-orders check fails, and so does the duplicate-node pair example, whose order is `{ N1 }`, `{ N2 }` for `m_multi` and `z_pair` and reversed for `z_multi` and `a_pair`). |
| FR-021-AC-25 | Skip instruction or location charges, check after emission, or expose a partial instruction root on refusal. |
| FR-021-AC-26 | Compute a right operand before testing the left stop condition; a denied left charge then admits a right charge. |
| FR-021-AC-27 | Treat a nested application as an inline recursive V2 term, bind callee by name alone, or accept an unbound/shared/cyclic expression edge. |
| FR-021-AC-28 | Keep the fixed 1 MiB source ceiling or check it only after publishing; the raised-limit deep source refuses or leaks partial output. |
| FR-021-AC-29 | Copy a full path into every record or fail to charge path expansion; the deep map grows quadratically or bypasses query limits. |
| FR-021-AC-30 | Accept `u64::MAX` and increment a `u64` at the first denied charge; the refusal wraps or panics. |
| FR-021-AC-31 | Count schedule-source compilation as execution evidence, or present an owner-path run as evidence without a QSL application API. |
| FR-021-AC-32 | Keep the deleted Contract Runtime `exact` port reachable through a re-export, wrapper or compatibility layer. |
| FR-021-AC-33 | Classify a directly requested formula or fairness root as the clause's QSL-121 blocker, classify a lowered `case` body or call item as `FormMismatch`, substitute the unsupported body root for the distinct valid call item and thereby mask the body's refusal, or omit the affected call item's claim while its healthy sibling is scheduled. |
| FR-021-AC-34 | Submit all 129 Call body roots to one Contract IR lowering request, duplicating their transitive suffixes until the unchanged 1,048,576-byte ceiling refuses an admitted public-generation chain. |

## Dependencies

- **Upstream**: [FR-014](./FR-014-exact-scalar-oracles.md),
  [FR-018](./FR-018-composite-equality-oracles.md), Contract IR FR-036/FR-038
  (admitted CheckedPackage V2 lowering), QSpec FR-322 (flat V2 body grammar)
  and FR-460 (no depth caps).
- **No QSL application API**: QSL exports no callable application API through
  qsl-replay (QSL decision, IR-845), so this requirement targets none. If a
  concrete oracle later needs per-charge results, linked-mode admission, origin
  or owner result variants, that need is filed and QSL specifies the API then.
- **Downstream**: [TC-031](../matrix/TC-031-function-application-oracles.md).

## Status

AC-7 remains unverified. The current source-bound V2 fixture's wire sizes were
measured before reader admission; no admission, lowering or generation was
attempted for these depth samples. The default reader byte limit is 1,048,576.

| Fixture | Serialized bytes | Within reader byte limit |
|---------|-----------------:|:------------------------:|
| Minimal checked-package base plus integer type | 11,505 | Yes |
| Base plus `FN_ADD` | 21,313 | Yes |
| One chain function | 39,423 | Yes |
| 128 chain functions (`MAX_CALL_DEPTH`) | 1,355,522 | No |
| 129 chain functions (`MAX_CALL_DEPTH + 1`) | 1,365,888 | No |

The required 100,000-node graph remains the AC-7 acceptance target. The
measured 128- and 129-function chain fixtures exceed the V2 reader's byte
budget. This result is limited to those serialized fixtures; whether a smaller
valid encoding fits remains unresolved under IR-768. These measurements do not
establish AC-7 admission or generation.

The target-neutral graph analysis, flat schedule, compact location map and
caller limits are planned in IR-511. The current implementation and existing
TC-031 corpus still witness the former Contract Runtime `exact` port. Its
behavioral obligations are retained by AC-2 through AC-5, AC-8, AC-9,
AC-14, AC-16, AC-17, AC-19 and AC-20, but their concrete execution and source
assertions are outside CG's boundary (IR-845). The historical port
is not the target architecture and no compatibility surface is specified.
TC-031 records the schedule-compilation gate; owner execution is outside CG's boundary (IR-845).

## Out of Scope

- Dynamic invocation location and loss reporting, whose producer and return
  types would belong to a QSL application API, which CG claims none of (IR-845).
- Standalone expression evaluation outside a named function application.
- Counterexample replay, model graph, identity, reachability, temporal and
  protocol oracles.
- Constructing a function declaration without a lowerable admitted body.
