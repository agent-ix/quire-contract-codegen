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
  - target: ix://agent-ix/quire-contract-runtime/FR-273
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-036
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-038
    type: references
---
# FR-021: Generate exact complete-V1 function-application oracles

## Description

When a caller supplies an admitted `quire.checked-package/v2` package, its declared functions'
expression-node bodies, and one checked `call` expression node applying one of those functions to
argument operands, the code generator shall emit a deterministic Rust oracle crate that: lowers
each declared function's body into a `quire_contract_runtime::exact::Body` closure using this
generator's own established node classifiers (scalar forms per
[FR-014](./FR-014-exact-scalar-oracles.md), composite-equality forms per
[FR-018](./FR-018-composite-equality-oracles.md), and nested `call` forms per this requirement,
recursively), assembles those into a `PackageDeclarations` and admits it through
`PackageDeclarations::check(CheckMode::Linked, limits)`, and emits one oracle function per
requested `call` item that applies the named function through `CheckedPackage::call` with a
caller-supplied `Meter` and `ObjectEnvironment`, returning the `Evaluation`'s `Outcome<Value>`
unchanged. It never interprets an expression tree itself outside of what it lowers into Rust, and
never charges a resource independently of the runtime's `function.call` accounting.

This is the function-application slice of complete-V1 oracle generation. Its scalar and
composite-equality siblings are [FR-014](./FR-014-exact-scalar-oracles.md) and
[FR-018](./FR-018-composite-equality-oracles.md).

FR-018-AC-7 refuses `call` expressions and function-family nodes as blocked on
`agent-ix/quire-contract-runtime#34`. This requirement generates the function family. FR-018's criterion is
neither restated nor weakened here: it governs FR-018's own composite-equality entry point, which
still admits no `call` node, while the `call` family is admitted only through this requirement's
separate entry point.

The runtime function-application call surface this requirement generates against is
`quire-contract-runtime` FR-273.
FR-273's own System Boundary (AD-002) holds here exactly as it does in the runtime: `Body` is a
host callable, so the runtime is not a second semantic authority, and the callable's own logic is
this generator's responsibility, produced by lowering — never by hand-writing a body's semantics
inside the emitter.

A `CheckedPackage` has no public constructor other than `PackageDeclarations::check`. As in
FR-018's `CheckedEquality`, the static admission stage therefore runs at generation time (refusing
the item if the assembled package does not admit) and is trusted, not re-derived, by the generated
function: an unreachable `check` failure inside a generated oracle has nowhere to go but
`Outcome::Refused(Refusal::CheckedInvariant)`, never an `unwrap` or `expect`.

### Location tagging: static half specified here, dynamic half blocked upstream

Generated function bodies carry `Location` tags because `quire-contract-runtime` has no expression
tree of its own to derive them from once a body is a host callable. In Contract Runtime
(`src/exact/expression.rs`):

- `pub type Body = Box<dyn for<'f> Fn(&Frame<'f>, &[Value]) -> Outcome<Value>>` (line 157). This is
  decisive on its own and is the root of the gap: a host body's **return type is `Outcome<Value>`**.
  A body is structurally incapable of returning a `Location` or a `LocatedLoss`, whatever it
  computes internally and however the emitter writes it.
- `Frame`'s only public methods are `call`, `meter`, `objects` and `depth` (lines 871–943); none
  accepts or returns a `Location` or a loss record. So there is no side channel either: the return
  type closes the direct path and `Frame`'s surface closes the indirect one.
- `Evaluation` (struct at lines 500–511, doc comment at 490–499) carries `outcome: Outcome<Value>`,
  `location: Option<Location>` and `losses: Vec<LocatedLoss>`, and its own doc comment states
  plainly: "no producer here ever populates `location` or `losses`: a host body reports neither
  through `Frame`'s public surface."
- Every construction of an `Evaluation` in the crate fixes both fields unconditionally:
  `CheckedPackage::call` (line 753) and `CheckedPackage::evaluate` (lines 797 and 805 — its
  `ForeignExpression` early return and its normal return) write `location: None, losses:
  Vec::new()`, never read back from anything the invoked `Body` computed. `CheckedPackage::run_call`,
  `run_evaluate` and `Frame::run` construct no `Evaluation` at all: each returns
  `Result<Value, Stop>`, which is the same fact seen from the other side.

Two scoping notes, because the unqualified claim would be wrong in both directions:

- It is **`Evaluation.location`** that is never populated, not `Location` generally. The runtime does
  produce `Location` values — `PackageDeclarations::check` returns `CheckRefusal`s carrying real
  `Origin::Body { function, index }` values (lines 303–360), and that is a runtime-authored fact this
  requirement's AC-15 uses as an independent cross-check.
- Every `Location` the runtime constructs anywhere goes through `fn location_at(origin) -> Location`
  (line 221), which hardcodes `path: Vec::new()`. The runtime therefore never emits a non-empty
  `path` at any stage, static or dynamic. A non-empty `path` is **codegen-originated**: this
  requirement ports the `Origin`/`Location` type shape verbatim and is the first producer to
  populate the `path` field at all.

So: no generated body, however it is written, can make `Evaluation.location` or `.losses`
non-empty under Contract Runtime. This is a runtime capability gap, not something an emitter choice
can work around. Populating those two fields
**dynamically, per invocation** is Out of Scope (see below) until the runtime publishes a
reporting channel a `Body` can reach.

What this requirement *can* and does specify is the **static** half: a deterministic,
generation-time record of which `Location{origin, path}` each generated sub-expression in a
function body corresponds to, independent of any runtime cooperation, verifiable by structural
inspection of the generated crate alone — see AC-15/AC-16 below. This is new claim-map content,
not a reinterpretation of FR-014's or FR-018's existing `source_map` field (a `CheckedSourceMapEntry`
per requested item, carried unchanged from Contract IR): that field maps one whole requested node
to its IR source span, at IR-node granularity; the location map this requirement adds maps every
sub-expression *inside one lowered function body* to the runtime's own `Origin`/`path` addressing,
at expression-tree granularity, because only a `call` item's assembled package has functions with
bodies to address that way at all.

## Inputs

- An admitted `CheckedPackageV2` read through Contract IR's strict reader.
- A request of items, each naming: (a) the package's declared functions, each a checked node whose
  body is a `binary` scalar expression (FR-014's forms), a `binary` equality expression (FR-018's
  forms), or a nested `call` expression (this requirement's own form, recursively bounded exactly
  as `CheckingLimits::depth` bounds it at runtime), its declared parameter types in order and its
  declared result type; and (b) one checked `call` expression node naming one declared function and
  its argument operands, each either a literal or a reference to an admitted value, in the same
  literal/reference shape FR-014 §Inputs already defines.
- The declaration closure reachable from every function's parameter and result types, in the same
  `composite_type`/`scalar_type`/`bounded_domain` encoding FR-018 §Inputs already defines.
- Contract Runtime with the `exact` feature, as this repository's `Cargo.toml` names it.

As in FR-014 and FR-018, the V2 transport carries each function declaration's own operator and
capability requirements (the `IeeeItemRequirement` and `IntegerDivisionConsumer` lists FR-009's
negotiators consume), so a generated package's `ieee_requirements`/`integer_division_consumers`
come from the request, not from this generator's own inference.

## Outputs

- A generated crate: `Cargo.toml` (`publish = false`, the Contract Runtime dependency this repository's `Cargo.toml` names, with the `exact`
  feature) and `src/lib.rs` holding, per admitted package, its assembled `PackageDeclarations`
  lowering and, per requested `call` item, one oracle function
  `(&CheckedPackage, &str, Vec<Value>, &ObjectEnvironment, &mut Meter) -> Result<Outcome<Value>,
  InputRefusal>` (the item's own function name and argument shape fixed at generation time; the
  runtime's own `InputRefusal` type surfaces unchanged).
- A typed claim map with one entry per requested `call` item: node id, Contract IR semantic id,
  package id, semantic type, source map, claims, the reconstructed declaration closure and its
  keys, the assembled function table's `Origin::Body { function, index }` for the applied function,
  and either the generated symbol or the typed refusal; plus the map-level blocked items.
- A location map: one entry per generated function body, recording every point where that body
  invokes a runtime scalar operator (FR-014), an equality evaluation (FR-018), or a nested
  `Frame::call` (this requirement), each tagged with the `Location{origin, path}` value — `origin`
  the function's own `Origin::Body { function, index }`, `path` the child-index path from that
  function's root to the sub-expression — whose type shape is ported verbatim: field and variant
  names and order equal to `quire_contract_runtime::exact::{Origin, Location}`. The shape is ported;
  the non-empty `path` is not. The runtime builds every `Location` through `location_at`, which
  always sets `path: Vec::new()`, so this generator is the first producer to populate that field,
  and a reader must not take the verbatim shape as evidence that the runtime populates paths too.

## Behavior

- When generation begins, the generator shall lower every requested function's body through its
  own established node classifiers — scalar per FR-014, composite equality per FR-018, and nested
  `call` per this requirement — assigning each declared function the `Origin::Body { function,
  index }` its position in the assembled `functions: Vec<FunctionDeclaration>` gives it, ordered by
  the function's declaring node id, in node-id order, the same total order FR-014
  already uses.
- If lowering any declared function's body fails — an unlowerable node, a form none of the three
  classifiers admits, or a nested `call` whose own callee is not among the request's declared
  functions — then the generator shall refuse every item bound to that function with a typed
  reason and emit no function for it, leaving the items of an unrelated function unchanged
  (FR-021-AC-12). A function whose own body lowers but whose nested `call` names a refused function
  is itself refused as `UnknownCallee`, and so on to its callers.
- If lowering a declared function's body returns a `failed` record, then the generator shall
  classify its `limit_kind` through `classify_lowering_failure`, which FR-014 specifies, and refuse
  that function's items, as the bullet above requires, with `LoweringWorkExhausted` for `work`,
  `LoweringByteLimitExceeded { limit, consumed }` for `bytes` (the `limit` and `consumed` Contract IR
  FR-038-AC-95 defines, carried unchanged) and `LoweringLimitUnrecognised` for any other kind, with
  `limit_kind` spelled as FR-014 states, so a byte-ceiling failure is never reported as work
  exhaustion and no `limit_kind` panics the generator (FR-021-AC-21's no-panic rule). Contract IR
  fails every requested body record when the package is over the byte ceiling, so every function is
  then refused and every item with it.
- The generator lowers twice: the declared functions' bodies and then the requested `call` nodes,
  each through the same `failed` classification. An item whose `call` node's record failed is
  refused with that record's refusal, which is checked first, so its `limit` and `consumed` are the
  call-node lowering's; only when that record lowered does the item carry its function's own
  refusal, whose `limit` and `consumed` are the body lowering's. When both lowerings fail for bytes
  the two refusals are equal in `limit`, and `consumed` is the call-node lowering's.
- When every declared function's body lowers, the generator shall assemble a `PackageDeclarations`
  and admit it through `PackageDeclarations::check(CheckMode::Linked, limits)` with
  `limits.depth() = MAX_CALL_DEPTH`. If `check` refuses, the generator shall refuse every item bound
  to that package with the reported `CheckRefusal`s and emit no function for any of them.
- When a package admits, the generator shall emit, for each requested `call` item naming one of its
  functions, an oracle function that validates no input itself and instead calls
  `CheckedPackage::call` with the item's function name and the caller's arguments, `ObjectEnvironment`
  and `Meter`, returning its `Result<Evaluation, InputRefusal>` outcome field unchanged as
  `Outcome<Value>` and its refusal unchanged as `InputRefusal`. It shall contain no `unwrap`,
  `expect`, index or arithmetic that can panic.
- The generator shall emit function-oracle source, every item function, every lowered function body
  and `checked_package()`, that contains no `unwrap`, `expect`, `unreachable!`, `panic!`,
  `todo!` or `unimplemented!`, the macros in any delimiter form.
- When the generator emits `checked_package()`, it shall build the empty `TypeEnvironment` with
  `TypeEnvironment::default()` and not with `TypeEnvironment::new(..).expect(..)`, so the
  function's return type stays `Result<CheckedPackage, Vec<CheckRefusal>>` and an admission
  failure is its `Err`, never a panic.
- When a lowered scalar or equality body forwards a runtime operator's result, the generated
  `match` over `Result<Outcome<_>, Refusal>` shall return `Completed`, `Undefined`, `Refused` and
  `Incomplete` through their own variants and `Err(refusal)` as `Outcome::Refused(refusal)`.
- When that `match` meets an `rt::Outcome` variant this generator does not know (`rt::Outcome` is
  `#[non_exhaustive]`), the generated body shall return `Outcome::Refused(Refusal::CheckedInvariant)`:
  the runtime's existing refusal for a checked-program invariant that failed during evaluation,
  and the one this requirement already names for an unreachable `check` failure. No new outcome
  or refusal type is added.
- If a function body's integer operator is not one of the binary `Add`, `Subtract` or
  `Multiply` (the unary `Negate`), then the generator shall refuse the body with
  `ExactFunctionRefusal::UnsupportedOperator` and omit that function from `checked_package()`.
- The generator's own `src/oracle/function/mod.rs` shall contain no invocation of `unreachable!`,
  `panic!`, `todo!` or `unimplemented!`, anywhere in the file; comment lines are not counted. Its
  non-test code shall also contain no `unwrap`, `expect` or other panic token FR-014-AC-39 lists:
  NFR-005 holds that scan for this file and names how its four remaining sites are expressed.
- If a function's declared parameter type or result type reaches a `composite_type` of form
  `reference` at any depth, then the generator shall refuse every item naming that function as
  blocked on quire-spec-language#120, for the same reason FR-018-AC-7 refuses a `reference` operand:
  the identity binding a reference needs is not yet available.
- If the node belongs to the model or relation family, then the generator shall refuse it as blocked
  on quire-spec-language#120; if it belongs to the state, temporal or protocol family, as blocked on
  quire-spec-language#121. This is FR-018's own split, restated here with each family bound to one
  named ticket rather than to a choice between two.
- If a function's declared operator requirements name a capability no registered backend can
  discharge, then the generator shall mark that function `unsupported` at generation time, naming
  the required capability, and shall emit no oracle for any item naming it; this mirrors
  FR-273-AC-4's own pre-application negotiation and is decided identically at generation time
  instead of at call time, since the negotiation itself takes no `Meter` and precedes any
  application.
- If one node id appears more than once in the request under one function binding, then the
  generator shall refuse every copy, matching FR-014/FR-018's duplicate handling.
- If two or more declared functions share one declaring node id, then the generator shall refuse
  each of them with `ExactFunctionRefusal::DuplicateDeclaringNode` before Stage 1 classification,
  so none enters the assembled package, and shall refuse every item naming one of them with that
  same reason. This check precedes the name-ambiguity check: a function that shares a declaring
  node id and also a name with another declaration is refused as `DuplicateDeclaringNode`, and an
  item whose function name is held by any such declaration is refused as
  `DuplicateDeclaringNode` rather than `AmbiguousFunctionName` (which remains the reason for a
  name shared by declarations whose node ids are all distinct). When the item's function name is
  held by duplicate groups on more than one node id, the refusal carries the smallest of those
  node ids in node-id order. This refusal also takes precedence over the duplicate-request
  refusal: two items on one `call` node naming different members of a duplicate-node pair, or one
  such item requested twice, each get `DuplicateDeclaringNode`. A third declaration with its own
  distinct node id that shares a name with one of the duplicate-node pair is not classified: it
  does not enter `checked_package()` or the location map. A declared function whose own
  nested `call` body names a function refused this way is refused as `UnknownCallee`, the reason
  `ExactFunctionRefusal::UnknownCallee` documents for a callee absent from the request or one that
  itself failed to classify.
- The generator shall order claim-map entries by the same descriptor-key discipline FR-018
  established: the `call` expression node's id, then the applied function's declaring node id, then
  each argument operand's source node id, every node id compared in node-id order.
- The generator shall record, in the location map, one entry per runtime call point a lowered body
  reaches (a scalar operator, an equality evaluation, or a nested `Frame::call`), each carrying the
  `Location` that call point's `Origin::Body { function, index }` and child-index path — computed
  entirely from the request's own expression trees, with no runtime execution required to produce
  it.
- If the generated source exceeds its size ceiling, then the generator shall return a typed error
  and no partial output.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-021-AC-1 | Every requested `call` item receives exactly one generated or typed-refused disposition; a refused item contributes no function, no symbol and no claim-map entry beyond its typed refusal, while its siblings bound to an admitted package are generated unchanged. | Test (TC-031) |
| FR-021-AC-2 | On every vector of the function-application corpus, the generated oracle's `Outcome<Value>`, its admitted charge sequence and its consumed counters are equal to those of `quire_contract_runtime::exact::CheckedPackage::call` invoked directly on an independently assembled package, arguments and fresh `Meter`. | Test (TC-031) |
| FR-021-AC-3 | Each generated item's claim-map entry records the applied function's name and its `Origin::Body { function, index }` equal to the request's own declared-function ordering, so the independent native run of AC-2 is driven from the request and never read back out of the generated crate. | Test (TC-031) |
| FR-021-AC-4 | An `InputRefusal::Arity`, `::WrongValueKind`, `::DanglingReference` or `::UnknownFunction` supplied at call time is returned by the generated oracle unchanged, before any charge, and no `Meter` observes a refused call. | Test (TC-031) |
| FR-021-AC-5 | The declared arity is decided before any per-argument check, each argument's value kind and carried references are validated in parameter order, and all of that precedes the `function.call` charge, which itself precedes the function's body, on every generated oracle call — matching FR-273-AC-2. | Test (TC-031) |
| FR-021-AC-6 | A function whose declared operator requirements name a capability no registered backend can discharge is marked `unsupported` at generation time, naming the capability, before any item naming it is applied; no oracle is emitted for such an item, and the disposition is never an `Outcome` variant, never an `InputRefusal`, and takes no `Meter`. | Test (TC-031) |
| FR-021-AC-7 | Re-entry reached from a generated oracle is bounded by one `CheckingLimits::depth` (`MAX_CALL_DEPTH`) budget shared across all three of `CheckedPackage::call`, `CheckedPackage::evaluate` and `Frame::call`, refusing `Refusal::CheckedInvariant` before any charge once exceeded, so a generated body that re-enters by any of the three paths is bounded by the same per-`CheckedPackage` budget as its entry call — matching FR-273-AC-7. | Test (TC-031) |
| FR-021-AC-8 | Only a package assembled and admitted under `CheckMode::Linked` is ever applicable; no generated oracle in the corpus applies a package this generator checked under `CheckMode::Kernel`, and `check`'s own refusal of `CheckMode::Kernel` is what makes that true. | Inspection (TC-031) |
| FR-021-AC-9 | Every generated oracle function returns `Outcome<Value>`, never `Value` or `bool`: with a denial injected at the `function.call` charge point, the oracle yields `Outcome::Incomplete` naming that point, the denied charge is not applied, and never a completed value. | Test (TC-031) |
| FR-021-AC-10 | A function whose declared parameter or result type reaches a `reference` composite form at any depth is refused as blocked on quire-spec-language#120 for every item naming it, emits no code, and admits no charge on any `Meter`. | Test (TC-031) |
| FR-021-AC-11 | Model and relation nodes are refused as blocked on quire-spec-language#120, and state, temporal and protocol nodes as blocked on quire-spec-language#121, each blocker distinct from the other and neither reported as generated. | Test (TC-031) |
| FR-021-AC-12 | A declared function whose own body fails to lower — an unlowerable node, a form none of this generator's classifiers admits, or a nested `call` naming a function absent from the request — refuses every item bound to that function with a typed reason, without changing an unrelated package's items. | Test (TC-031) |
| FR-021-AC-13 | Generated bytes are identical across repeated runs and across permutations of the request order; claim-map entries are ordered by the `call` node id, then the applied function's declaring node id, then each argument operand's source node id, every node id compared in node-id order. | Test (TC-031) |
| FR-021-AC-14 | The generated crate declares `publish = false`, names the Contract Runtime dependency this repository's `Cargo.toml` names with the `exact` feature, contains no charge amount and no literal `Outcome`/`Value` constant standing in for a runtime result, and compiles. | Test (TC-031) |
| FR-021-AC-15 | The location map records one entry per runtime call point (scalar operator, equality evaluation or nested `Frame::call`) a generated body reaches, each carrying a `Location{origin, path}` equal, field-for-field, to the value re-derived from the request's own expression tree by walking the same function from its root to that sub-expression's child index — a generation-time structural check against the request, requiring no runtime execution, and the only available check on `path`, which the runtime itself never populates. | Test (TC-031) |
| FR-021-AC-16 | `Evaluation.location` and `Evaluation.losses`, as returned by every generated oracle's call into `CheckedPackage::call`, are never read, asserted non-empty, or otherwise relied on by the generated crate or its claim map: under Contract Runtime they are always `None`/empty regardless of what the applied body computed, so no generated code branches on either field. | Inspection (TC-031) |
| FR-021-AC-17 | Each location map entry's `origin` field equals the `Origin::Body { function, index }` the runtime itself reports for that function, read from the `CheckRefusal`s `PackageDeclarations::check` returns when the same assembled package is re-submitted with that function's measure undischarged, so the generator's function indexing is confirmed against a runtime-authored value and not only against its own re-derivation. | Test (TC-031) |
| FR-021-AC-18 | On every vector of the function-application corpus, the generated oracle's outcome, refusal and charge sequence are equal to `quire_spec_language::value::expression::CheckedPackage::call` this requirement's third agreement leg alongside AC-2's generated and native legs. | Test (TC-031) |
| FR-021-AC-19 | The emitted `src/lib.rs` of the main corpus of `tests/it/exact_function_generation.rs` (`main_oracles()`, whose generated functions include the scalar `add_fn`, the equality `eq_fn` and the nested-call `call_fn`) and of its chain corpus (`chain_oracles()`) contains zero occurrences of `.unwrap(`, `.expect(`, `unreachable!`, `panic!`, `todo!` and `unimplemented!`, the macros in any delimiter form. | Test (TC-031) |
| FR-021-AC-20 | In the emitted body of `add_fn` and of `eq_fn`, the `match` over the runtime operator's `Result<Outcome<_>, Refusal>` has arms for `Ok(Completed)` (rewrapped as `Value::Integer` or `Value::Boolean`), `Ok(Undefined)`, `Ok(Refused)`, `Ok(Incomplete)` and `Err(refusal)` (returned as `Outcome::Refused(refusal)`), and a final `Ok(_)` arm whose value is `Outcome::Refused(Refusal::CheckedInvariant)`. | Test (TC-031) |
| FR-021-AC-21 | A request whose scalar body has operator `Negate` is refused with `ExactFunctionRefusal::UnsupportedOperator`, and that function does not appear in the emitted `checked_package()`; `src/oracle/function/mod.rs` contains zero invocations of `unreachable!`, `panic!`, `todo!` and `unimplemented!`, comment lines not counted. | Test (TC-031) |
| FR-021-AC-22 | When two or more declarations share one declaring node id, none of them appears in the emitted `checked_package()` or in `location-map.json`, and every item naming one of them is refused with `ExactFunctionRefusal::DuplicateDeclaringNode { node_id }` (the shared node id; the smallest in node-id order when the item's name is held by duplicate groups on several node ids) before Stage 1 classification: never `UnknownFunction`, never `AmbiguousFunctionName` (the node-id refusal takes precedence when the declarations also share a name), never `DuplicateRequest` (two items on one `call` node naming different members, or one such item requested twice, each carry the node-id refusal), and no claim-map entry records another function's name, oracle symbol or `Origin::Body` index. A declaration with its own distinct node id that shares a name with one of them is absent from `checked_package()` and from `location-map.json`. A declared function whose nested `call` names such a function is refused as `UnknownCallee`. The refusal and these outputs are identical under every permutation of the request order. Every item naming a function with a distinct declaring node id, and not a duplicate name, has a claim-map entry equal to the one the same request produces with the duplicate declarations removed. | Test (TC-031) |
| FR-021-AC-23 | A `failed` lowering record is refused as `ExactFunctionRefusal::LoweringWorkExhausted` for the `work` limit, as `LoweringByteLimitExceeded { limit, consumed }` with the record's `limit` and `consumed` for the `bytes` limit (Contract IR FR-038-AC-95), and as `LoweringLimitUnrecognised` with `limit_kind` the snake_case name FR-014 states for each of `depth`, `nodes`, `edges`, `occurrences` and `diagnostics`, never as another arm's refusal and without a panic. The mapping is asserted on hand-built `Failed` records given to `lowered_binary_body`, the one function that maps a record to a refusal, in a `#[cfg(test)]` module the code change adds to the function module (it has none today), so a mapping of an unrecognised kind to `LoweringWorkExhausted` in this module fails it. That a body failure refuses only that function's items, and leaves an unrelated function's items unchanged, is FR-021-AC-12's and is asserted by its existing `tc_031_ac12_*` tests, not re-asserted on hand-built records, because the isolation lives in the classification loop that takes its records from `lower`. Through a whole call of the function generator whose input is read under a byte ceiling that admits the checked package and is one byte below the shorter of the canonical lengths of the two lowered contract packages of that call (the one lowered for the function bodies and the one lowered for the requested `call` nodes; the fixture makes both longer than the checked package), every body record and every call-node record fails for bytes, so every function is absent and every item is refused as `LoweringByteLimitExceeded` with `limit` equal to that ceiling and `consumed` equal to the call-node lowering's, which the fixture makes differ from the body lowering's `consumed` (read by lowering each request alone under the same ceiling), so the call-node-first order of the Behavior section is asserted through the call. | Test (TC-031) |

### Mutations these criteria detect

Each criterion is recorded with the mutation it exists to catch; a criterion without one is not
written.

| ID | Mutation that breaks it |
|----|-------------------------|
| FR-021-AC-1 | Abort the whole request on the first refusal, or emit a stub function for a refused item. |
| FR-021-AC-2 | Swap which argument is validated first, or apply the runtime's charge sequence in a different order than the corpus's native leg. |
| FR-021-AC-3 | Record the applied function's identity by reading it back out of the generated source, so a mutated emitter and a mutated claim map agree with each other and AC-2's native leg follows the mutation. |
| FR-021-AC-4 | Emit a generic refusal that discards which `InputRefusal` variant the runtime actually returned. |
| FR-021-AC-5 | Validate arguments out of parameter order, or charge `function.call` after invoking the body instead of before. |
| FR-021-AC-6 | Generate the oracle anyway and let an unsupported capability surface as a runtime panic or an ordinary refusal instead of a pre-generation disposition. |
| FR-021-AC-7 | Omit the depth bound from a nested `Frame::call` site, or hardcode a depth limit different from `MAX_CALL_DEPTH`. |
| FR-021-AC-8 | Admit a `CheckMode::Kernel` package and generate an oracle from it. |
| FR-021-AC-9 | Emit a function returning plain `Value`, as `src/oracle.rs` does for Boolean connectives before FR-018's fix; a denial then has no representable result. |
| FR-021-AC-10 | Generate the item and let the runtime refuse a reference-typed argument at call time instead of refusing it at generation time. |
| FR-021-AC-11 | Collapse the two blockers into one "unsupported" reason, or generate an oracle over a model or temporal node. |
| FR-021-AC-12 | Generate the package anyway with a stub body standing in for the node that failed to lower. |
| FR-021-AC-13 | Order claim-map entries by the applied function's name alone, which collides two items over functions sharing a name across packages. |
| FR-021-AC-14 | Emit a charge amount, or a literal `Outcome::Completed(Value::Integer(..))`, standing in for what only a live call can produce. |
| FR-021-AC-15 | Record every location's `path` as empty, or record it against the wrong function's `index`. |
| FR-021-AC-16 | Add a check that treats a non-`None` `Evaluation.location` as a defensive branch, silently depending on a runtime capability that does not exist yet. |
| FR-021-AC-17 | Index the assembled functions by request ordinal rather than by the position `PackageDeclarations` gives them, so the generator's own re-derivation (AC-15) agrees with the mutated emitter and only the runtime's own `CheckRefusal` disagrees. |
| FR-021-AC-18 | Agree with the runtime port alone and inherit any divergence the port carries from the authority, which is the divergence FR-273-AC-5 exists to catch and which AC-2's two legs cannot see. |
| FR-021-AC-19 | Leave an `Ok(_) => unreachable!(..)` arm in a body template, or an `.expect(..)` in `checked_package()`, so a runtime release that adds an `Outcome` variant, or an unforeseen admission failure, panics a generated oracle instead of refusing. |
| FR-021-AC-20 | Replace the catch-all with a `Completed` fallthrough or a different refusal, so an unknown variant is reported as a value or as a refusal that names no checked-program invariant. |
| FR-021-AC-21 | Keep a defensive `Negate => unreachable!(..)` arm after the earlier refusal, so a change to the refusal turns a bad request into a generator panic. |
| FR-021-AC-22 | Key classification by position but resolve an item's function by name or node id, so two declarations with one node id both survive and the second item takes the first function's oracle symbol and claim-map index; refuse only the first (or only the later-sorted) declaration so its same-node sibling survives and the item reports `UnknownFunction` or depends on request order; check the name before the node id so a declaration sharing both reports `AmbiguousFunctionName`; exempt a distinct-node-id declaration that shares a name with the pair so it enters the package and answers to the pair's name; or refuse the whole request instead of only the duplicate declarations. |
| FR-021-AC-23 | Keep the single `Failed` arm that reports every failure as `LoweringWorkExhausted`, so a byte-ceiling failure reads as work exhaustion; or leave a `_ => LoweringWorkExhausted` or `_ => unreachable!(..)` arm for an unrecognised kind; or report the function's body-lowering `consumed` for an item whose call-node lowering failed first. |

## Dependencies

- **Upstream**: [FR-014](./FR-014-exact-scalar-oracles.md),
  [FR-018](./FR-018-composite-equality-oracles.md), Contract IR FR-036/FR-038 (CheckedPackage V2
  lowering), Contract Runtime FR-273 (function-application call surface).
- **Downstream**: [TC-031](../matrix/TC-031-function-application-oracles.md).

## Out of Scope

- **Dynamic location and loss reporting.** Making a specific `CheckedPackage::call`/`::evaluate`
  invocation's own `Evaluation.location`/`.losses` non-empty requires a reporting channel on
  `Frame` that Contract Runtime does not publish (see "Location tagging"
  above, with file:line citations). The gap is structural rather than a matter of `Frame`'s method
  list: `Body`'s own return type is `Outcome<Value>`, so closing it requires the runtime to widen
  either that return type or `Frame`'s surface. This is a runtime capability gap, not an emitter
  choice.
- **Standalone expression evaluation** (`CheckedPackage::evaluate` over a `CheckedExpression` not
  bound to a declared function name). This requirement generates oracles for named `call`
  application only; `evaluate`'s different charge-at-root behavior (zero `function.call` events at
  its own root, per `CallPlan::call_events`) is a distinct entry point left to a future requirement.
- **Counterexample replay.** This requirement generates the oracle and its static location map; consuming either to
  explain a falsified proof's counterexample is a different requirement's job, not designed here
  even at a spec level.
- Model graph, identity and reachability oracles.
- Temporal and protocol oracles.
- Function *declaration* construction from scratch by a caller who supplies no body at all —
  every function this requirement's package assembles must have a lowerable body in the request;
  declaring an uninterpreted function signature with no callable body is not a complete-V1 need this
  requirement was asked to serve.
