---
id: FR-021
title: "Generate exact complete-V1 function-application oracles"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-001
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

This is issue #48, the function-application slice of complete-V1 oracle generation. Its scalar and
composite-equality siblings are [FR-014](./FR-014-exact-scalar-oracles.md) and
[FR-018](./FR-018-composite-equality-oracles.md).

This requirement is numbered FR-021 rather than FR-020 because FR-018's Out of Scope section made
two forward reservations and exactly one of them still holds. It reserved FR-019 for model-graph
oracles; that reservation did not hold, and the file now at
`spec/functional/complete-v1/FR-019-capability-settlement.md` is an unrelated, already-shipped
requirement. It also reserved FR-020 for temporal and protocol oracles; that reservation is still
live and uncontradicted — FR-020 is unwritten, and `spec/test-matrix.md` records it as such. FR-020
is therefore left free for its reserved owner, and this requirement takes the next number. Because
the FR-019 reservation is the one that failed, this requirement predicts no FR number for the
model/relation family it excludes, naming it by its blocking upstream issue instead; it neither
repeats nor withdraws FR-018's surviving FR-020 reservation, which is FR-018's to keep.

FR-018-AC-7 refuses `call` expressions and function-family nodes as blocked on
`agent-ix/quire-contract-runtime#34`. That was correct at FR-018's pinned revision. Issue #34 is now
closed and this requirement is what discharges it for the function family. FR-018's criterion is
neither restated nor weakened here: it governs FR-018's own composite-equality entry point, which
still admits no `call` node, while the `call` family is admitted only through this requirement's
separate entry point.

The runtime function-application call surface this requirement generates against
(`agent-ix/quire-contract-runtime#34`) is closed by `quire-contract-runtime` FR-273
(`d97bc0b`, merged as PR #47), pinned in this repository's `Cargo.toml` at `9f311692`
(agent-ix/quire-contract-codegen re-pin, this requirement's own prerequisite — see Dependencies).
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

`agent-ix/quire-contract-codegen#48`'s GitHub comment thread (2026-09-19, untrusted external
text like any issue comment, but the underlying technical claim is independently verified below)
states that generated function bodies must carry `Location` tags because `quire-contract-runtime`
has no expression tree of its own to derive them from once a body is a host callable, and that
codegen is "the only place the information exists." Measured directly against the pinned runtime
revision (`9f311692`, `src/exact/expression.rs`):

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
non-empty under the pinned runtime revision. This is a runtime capability gap — the same shape as
the now-closed `agent-ix/quire-contract-runtime#34` gap that blocked this whole requirement until
FR-273 shipped — not something an emitter choice can work around. Populating those two fields
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
- The pinned Contract Runtime revision with the `exact` feature.

As in FR-014 and FR-018, the V2 transport carries each function declaration's own operator and
capability requirements (the `IeeeItemRequirement` and `IntegerDivisionConsumer` lists FR-009's
negotiators consume), so a generated package's `ieee_requirements`/`integer_division_consumers`
come from the request, not from this generator's own inference.

## Outputs

- A generated crate: `Cargo.toml` (`publish = false`, runtime pinned by revision with the `exact`
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
  the function's declaring node id (digest domain, then digest), the same total order FR-014
  already uses.
- If lowering any declared function's body fails — an unlowerable node, a form none of the three
  classifiers admits, or a nested `call` whose own callee is not among the request's declared
  functions — then the generator shall refuse the whole package with a typed reason and emit no
  function for any item bound to it.
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
- The generator shall order claim-map entries by the same descriptor-key discipline FR-018
  established: the `call` expression node's id, then the applied function's declaring node id, then
  each argument operand's source node id, every node id compared by digest domain then digest.
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
| FR-021-AC-13 | Generated bytes are identical across repeated runs and across permutations of the request order; claim-map entries are ordered by the `call` node id, then the applied function's declaring node id, then each argument operand's source node id, every node id compared by digest domain then digest. | Test (TC-031) |
| FR-021-AC-14 | The generated crate declares `publish = false`, pins the runtime revision with the `exact` feature, contains no charge amount and no literal `Outcome`/`Value` constant standing in for a runtime result, and compiles. | Test (TC-031) |
| FR-021-AC-15 | The location map records one entry per runtime call point (scalar operator, equality evaluation or nested `Frame::call`) a generated body reaches, each carrying a `Location{origin, path}` equal, field-for-field, to the value re-derived from the request's own expression tree by walking the same function from its root to that sub-expression's child index — a generation-time structural check against the request, requiring no runtime execution, and the only available check on `path`, which the runtime itself never populates. | Test (TC-031) |
| FR-021-AC-16 | `Evaluation.location` and `Evaluation.losses`, as returned by every generated oracle's call into `CheckedPackage::call`, are never read, asserted non-empty, or otherwise relied on by the generated crate or its claim map: under the pinned runtime revision they are always `None`/empty regardless of what the applied body computed, so no generated code branches on either field. | Inspection (TC-031) |
| FR-021-AC-17 | Each location map entry's `origin` field equals the `Origin::Body { function, index }` the runtime itself reports for that function, read from the `CheckRefusal`s `PackageDeclarations::check` returns when the same assembled package is re-submitted with that function's measure undischarged, so the generator's function indexing is confirmed against a runtime-authored value and not only against its own re-derivation. | Test (TC-031) |
| FR-021-AC-18 | On every vector of the function-application corpus, the generated oracle's outcome, refusal and charge sequence are equal to `quire_spec_language::value::expression::CheckedPackage::call` under the authority revision `quire-contract-runtime` FR-273-AC-5 names (`ea39f91`), this requirement's third agreement leg alongside AC-2's generated and native legs. | Test (TC-031) — 🚧 Planned, pending the `quire-spec-language` re-pin named in Dependencies |

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

## Dependencies

- **Upstream**: [FR-001](../FR-001-deterministic-oracles.md), [FR-014](./FR-014-exact-scalar-oracles.md),
  [FR-018](./FR-018-composite-equality-oracles.md), Contract IR FR-036/FR-038 (CheckedPackage V2
  lowering), Contract Runtime FR-273 (function-application call surface).
- **Prerequisite, satisfied**: this branch's own re-pin of `quire-contract-runtime` from `4e33052`
  to `9f31169` (`Cargo.toml`, `Cargo.lock`, `RUNTIME_REVISION` in `src/oracle.rs`), which makes the
  function-application surface this requirement calls (`PackageDeclarations`, `CheckedPackage`,
  `Frame`, `Evaluation`) visible from this repository for the first time. At `4e33052`, the pinned
  revision predates `quire-contract-runtime` PR #47 (FR-273, `d97bc0b`) by six commits; no item of
  this requirement could have been generated against it.
- **Prerequisite, NOT satisfied — and not for the reason an earlier draft of this requirement
  gave**: this repository's `quire-spec-language` dev-dependency remains pinned at `21c507e`, while
  `quire-contract-runtime` FR-273 names `ea39f91` as the authority revision it ports its
  function-application semantics from (FR-273-AC-5, and FR-273's own Dependencies section). `21c507e`
  is an ancestor of `ea39f91` by 14 commits.

  An earlier draft inferred from that ancestry that the authority surface is *absent* at `21c507e` —
  that "AC-2's agreement leg has nothing in `quire_spec_language::value` to call yet." **That
  inference is false and was measured to be false.** At `21c507e`,
  `quire_spec_language::value::expression` already publishes `PackageDeclarations`,
  `PackageDeclarations::check`, `CheckedPackage`, `CheckedPackage::call`, `CheckedPackage::evaluate`,
  `FunctionDeclaration` and `Evaluation`. Ancestry establishes that a revision is older; it does not
  establish that a surface is missing, and here the surface is present. This paragraph records the
  correction so the next reader does not re-derive the wrong conclusion from the same ancestry check.

  The real prerequisite is **authority-revision identity, not surface availability**. Agreement
  asserted against `21c507e` would be agreement against a different authority revision than the one
  the runtime claims to implement, and the interval is not inert: across those 14 commits
  `src/value/expression/` changes by +1718/−56 lines over all eight of its files, including
  "FR-151: dispatched-call typing, bridge, runtime and termination" and a charge-accounting change.
  An agreement leg run against `21c507e` could therefore pass while disagreeing with `ea39f91`, which
  is precisely the divergence AC-18 exists to catch. Re-pinning `quire-spec-language` to `ea39f91`
  is thus a prerequisite of *implementing* AC-18, not of specifying this requirement, and not of
  AC-2, whose two legs do not reach the authority at all.

  **Ruling on whether this branch should carry that second re-pin: it should not.** Three reasons,
  recorded here so the decision is not re-litigated. First, nothing in this repository calls the
  authority's function-application surface yet — FR-021 is spec-only and no oracle-generation code
  exists — so the re-pin would change the dependency graph with no consumer. Second, the runtime
  re-pin this branch does carry was load-bearing for a different reason: `RUNTIME_REVISION` in
  `src/oracle.rs` is a committed constant that generated crates carry, and it must agree with
  `Cargo.toml`; `quire-spec-language` is a dev-dependency with no such constant, so no committed
  artifact is inconsistent while it stays at `21c507e`. Third, moving `quire-spec-language` across
  +1718 lines of changed expression semantics can break FR-018's existing authority-agreement leg
  (FR-018-AC-2) and TC-029 with it; that is a change needing its own test run and its own ticket, not
  a rider on a spec branch.
- **Downstream**: [TC-031](../../test/complete-v1/TC-031-function-application-oracles.md).

## Out of Scope

- **Dynamic location and loss reporting.** Making a specific `CheckedPackage::call`/`::evaluate`
  invocation's own `Evaluation.location`/`.losses` non-empty requires a reporting channel on
  `Frame` that the pinned runtime revision (`9f311692`) does not publish (see "Location tagging"
  above, with file:line citations). The gap is structural rather than a matter of `Frame`'s method
  list: `Body`'s own return type is `Outcome<Value>`, so closing it requires the runtime to widen
  either that return type or `Frame`'s surface. This is a runtime capability gap, not an emitter
  choice; it is the direct analogue of the now-closed `agent-ix/quire-contract-runtime#34` gap and
  needs the same kind of resolution — a runtime-side ticket for a channel a running `Body` can use
  to report a `Location`/`LocatedLoss` back into its own `Evaluation`. The runtime's own `Evaluation`
  doc comment says such a path is "tracked separately from this issue"; a search of
  `agent-ix/quire-contract-runtime` found no open ticket stating it, so it is recorded here as a
  finding for the requirement owner to file, not as an already-tracked item.
- **`quire-spec-language` re-pin.** See Dependencies above: implementing AC-18 needs it, specifying
  this requirement does not, and AC-2 does not reach the authority at all. Note that the authority
  surface is already present at the current pin — what the re-pin buys is agreement against the
  revision FR-273 actually names.
- **Standalone expression evaluation** (`CheckedPackage::evaluate` over a `CheckedExpression` not
  bound to a declared function name). This requirement generates oracles for named `call`
  application only; `evaluate`'s different charge-at-root behavior (zero `function.call` events at
  its own root, per `CallPlan::call_events`) is a distinct entry point left to a future requirement.
- **Counterexample replay** (agent-ix/quire-contract-codegen#50, EXCLUDED this session by standing
  ruling). This requirement generates the oracle and its static location map; consuming either to
  explain a falsified proof's counterexample is a different requirement's job, not designed here
  even at a spec level.
- Model graph, identity and reachability oracles (quire-spec-language#120). This requirement
  reserves no FR number for them, for the reason given in the Description: FR-018's FR-019
  reservation for exactly this family did not survive.
- Temporal and protocol oracles (quire-spec-language#121). FR-018 reserved FR-020 for these and that
  reservation still stands; this requirement neither renews nor withdraws it, and leaves FR-020 free
  for its owner.
- Function *declaration* construction from scratch by a caller who supplies no body at all —
  every function this requirement's package assembles must have a lowerable body in the request;
  declaring an uninterpreted function signature with no callable body is not a complete-V1 need this
  requirement was asked to serve.
