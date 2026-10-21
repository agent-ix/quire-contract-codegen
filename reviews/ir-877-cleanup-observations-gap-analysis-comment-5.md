<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15291 method=gap-analysis lang=rust pr=quire-contract-codegen#501 model=unavailable run=d9d507d8-42eb-4ddc-a8d0-3eed0b37342f -->

SR-15291 gap-analysis, part 5/18. FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-073 | high | coverage | Inherited FR-015-AC-111 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-074 | high | coverage | Inherited FR-015-AC-117 is tagged-by-ignored-test at the frozen candidate: every bound test is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-075 | high | coverage | Inherited FR-015-AC-119 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-076 | high | coverage | Inherited FR-015-AC-120 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-077 | high | coverage | Inherited FR-015-AC-121 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-078 | high | coverage | Inherited FR-015-AC-122 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-079 | high | coverage | Inherited FR-017-AC-7 is tagged-by-ignored-test at the frozen candidate: every bound test is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-080 | high | coverage | Inherited FR-017-AC-19 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-081 | high | coverage | Inherited FR-017-AC-26 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-082 | high | coverage | Inherited FR-017-AC-27 is tagged-by-ignored-test at the frozen candidate: every bound test is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-083 | high | coverage | Inherited FR-017-AC-30 is tagged-by-ignored-test at the frozen candidate: every bound test is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-084 | high | coverage | Inherited FR-017-AC-47 is tagged-by-ignored-test at the frozen candidate: every bound test is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-085 | high | coverage | Inherited FR-017-AC-52 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-086 | high | coverage | Inherited FR-017-AC-53 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-087 | high | coverage | Inherited FR-017-CON-1 is tagged-by-ignored-test at the frozen candidate: every bound test is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-088 | high | coverage | Inherited FR-018-AC-21 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-089 | high | coverage | Inherited FR-018-AC-22 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-090 | high | coverage | Inherited FR-018-AC-23 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |

+++ [reviewer data]

```yaml
clean: false
scope:
- id: FR-015-AC-60
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A `StateFrame` item that yields is recorded `supported` carrying its harness symbol, and its
    harness, returned in the `state_frame_harnesses` field of `KaniObligationOutcome::Emitted` in request
    order, is byte-identical to the harness `generate_state_frame_role` returns for the same clause and
    role, and for a clause both of whose roles succeed to the corresponding harness `generate_state_frame_obligations`
    returns. COVERED (IR-461): a crate-internal test uses an admitted V2 package to compare direct role
    generation with each negotiated supported item, its symbol and exact harness bytes, and th'
- id: FR-015-AC-61
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A `StateFrame` item whose lowering is a requires-bound record is recorded `requires_bound`
    with that record's `unbounded_type` and no harness. A selected model declaration's clause read of
    a native `Integer` that makes lowering return `RequiresBound` keeps that lowering refusal; it is not
    remapped to value-based `ModelMemberNotI64Range`. The latter applies to a present listed `Integer`
    field only if range resolution is reached, such as an unread field. A non-model or admitted unselected
    model/object_type body is refused as `ModelFieldsUnavailable`, without inventing an `unbounded_type`
    node or
- id: FR-015-AC-62
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A `StateFrame` item whose lowering is an unsupported-family record is recorded `unsupported`
    with reason `NoFiniteEncoding` naming the record's node and family, and an item whose node is not
    a `state_clause` is recorded `unsupported` with `UnknownNodeKind`, each with no harness.
- id: FR-015-AC-63
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A `StateFrame` item whose condition is a negation, compares to a literal, uses an operator
    other than the six integer comparisons, reads through a parameter other than `self`, compares two
    fields or two reads of one side, or whose frame creates or deletes an object or grants a relationship
    or foreign field (naming the effect), is recorded `unsupported` with `StateFrameRefused` and no harness.
    The same result holds for a clause that is not a postcondition, is malformed, whose frame grants every
    field, whose lowering is an invalid-body, incomplete or over-budget record, or whose object is non-mo
- id: FR-015-AC-64
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A `StateFrame` item with an unparsable state path or an unparsable item subject path, an invalid
    or lacking state field, an absent node, a repeat of an earlier item or a package other than the first
    item's is recorded `invalid_request` (`InvalidStatePath`, `InvalidStateField`, `UnknownNode`, `DuplicateItem`,
    `MixedStatePackages`), and the outcome is `Rejected` with every item's record and no harness bytes.
- id: FR-015-AC-65
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A request holding `StateFrame` items and no items, more than `MAX_OBLIGATION_ITEMS`, an unparsable
    request subject path or an unwind bound outside `1..=MAX_OBLIGATION_UNWIND` is refused whole with
    the existing `KaniObligationError` variant and no record.
- id: FR-015-AC-66
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A test builds every retained `StateFrameRefusal` variant (including `ModelFieldsUnavailable`,
    `MemberAbsent`, `ModelMemberNotI64Range` with each cause, and each of the six refusal arms of `NotLowered`)
    and calls the mapping on it; each yields the disposition and reason of the table. It does not construct
    retired body-member `BoundNotResolvedCause` values. The variants a request reaches only with a defective
    generator or a source over 1 MiB (`UnwindOutOfRange`, `InvalidGeneratedSyntax`, `RecordSerialization`)
    are built directly, and `ResourceLimitExceeded` is built directly or reached with a su
- id: FR-015-AC-67
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '`generate_state_frame_obligations` refuses with `MalformedClause` at the node, not `InvalidField`
    or `ConditionNotSupported`, for a clause whose graph field name is not a Rust identifier; a field
    name the caller supplies that is not an identifier is still `InvalidField`.'
- id: FR-015-AC-68
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A `StateFrame` request whose frame grants every state field records its `frame` item `unsupported`
    (`StateFrameRefused`) and its `contract` item `supported` with the harness `generate_state_frame_role`
    returns for the `contract` role; a request whose condition is a negation records its `contract` item
    `unsupported` and its `frame` item `supported` with the harness `generate_state_frame_role` returns
    for the `frame` role; in both, `generate_state_frame_obligations` returns the refusal of the failing
    role; a request whose lowering is refused records both items alike. COVERED (IR-461): a crate-in'
- id: FR-015-AC-69
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Emit only the `Equal` harness for a `NotEqual` item, emit a `production` harness for the family,
    or read a bound from the descriptor.
- id: FR-015-AC-70
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Draw a payload for an absent option, collapse `absent` and `null` into one draw, take a bound
    from the whole `i64`, or leave a literal operand's leaf unpinned.
- id: FR-015-AC-71
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Compute the expectation by calling the shadow, so the assertion is `x == x`; stop counting
    after the first unequal field; or place the cover before the assertions.
- id: FR-015-AC-72
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Emit a loop or a `Vec` in the shadow, or raise `SHADOW_PAIR_NODE_BUDGET` to fit a closure instead
    of refusing it.
- id: FR-015-AC-73
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Refuse every shape as one reason, treat an unbounded integer as the whole `i64`, a recursive
    declaration as depth one, or a `convert<T>` operand as its source type, or let a refused item drop
    a supported sibling.
- id: FR-015-AC-74
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Pass a corpus case whose cover is unsatisfiable, whose refinement ran over a narrower domain
    than the shadow drew, or whose refinement was `sampled` where its domain fits the cap.
- id: FR-015-AC-75
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Mutate only the production oracle or only the shadow; or accept an equivalent mutant as the
    test of a check (a permutation of member order, or swapping two descriptors of one type).
- id: FR-015-AC-76
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Record `backed` for a covering criterion the matrix lists planned, omit `declaration_reconstruction`,
    or leave the identity unchanged when a bound changes.
- id: FR-015-AC-77
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Read a model field from an empty object body or a read's `result_type`, omit unread `audit`,
    narrow an `i128` endpoint, or let generation and replay use different domains.
- id: FR-015-AC-78
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Fall back after an accessor error; treat an absent accessor field as an unranged present field;
    use an `Integer`, `Option` or out-of-`i64` range as a range; or trust a grouped derived-shape body
    bound.
- id: FR-015-AC-79
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Reorder request fields to the accessor's name order, insert an unrequested inherited field,
    or emit the hand-built fixture's scope ids.
- id: FR-015-AC-80
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Generate the harness from the hand-built fixture rather than the emitted package.
- id: FR-015-AC-81
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Mark an unread `IntRange` as `NoRead`, give an untyped member a range, omit its reason, or
    accept a duplicate unranged name.
- id: FR-015-AC-82
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-profile classifier, a request for distinct named constructs
    returns one `CapabilityEntry` per requested construct in request order, preserving each admitted `Supported`
    module or `Refused` cause code exactly. The Contract IR profile reader rejects the removed `inconclusive`
    disposition rather than admitting it; no profile disposition or profile-supplied code becomes an `Inconclusive`
    run outcome or asserts proof vacuity.
- id: FR-015-AC-83
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-profile classifier, an empty, duplicate or absent construct
    is a typed `KaniOutcome` refusal rather than a partial disposition census.
- id: FR-015-AC-84
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the arithmetic slice (FR-015-AC-109) deletes the finite-input arithmetic lowerer, a profile/dispatch-admitted
    add, subtract, multiply, divide or remainder returns the exact checked `i128` result only inside the
    request's inclusive result range.
- id: FR-015-AC-85
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the arithmetic slice (FR-015-AC-109) deletes the finite-input arithmetic lowerer, an
    inverted result range is `InvalidInput`; zero division, checked overflow and a result outside the
    range are typed refusals, with no arithmetic result or generated artifact.
- id: FR-015-AC-86
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the graph slice (FR-015-AC-111) deletes the finite-graph lowerer, a profile/dispatch-admitted
    request follows only the selected reference field, finds a positive-length path in sorted depth-first
    order, and checks the expansion bound before entering each new object; a target edge is found without
    expanding its target.
- id: FR-015-AC-87
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the graph slice (FR-015-AC-111) deletes the finite-graph lowerer, a zero expansion bound
    or unknown endpoint is `InvalidInput`, and exhausting the bound is `ResourceExhausted`, with no graph
    result or generated artifact.
- id: FR-015-AC-88
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the collection slice (FR-015-AC-110) deletes the finite-collection lowerer, a profile/dispatch-admitted
    `ForAllNonNegative` or `ExistsEqual` query retains input order and duplicates and returns its truth
    value and examined-item count; a decisive item ends that count, otherwise every item counts.
- id: FR-015-AC-89
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the collection slice (FR-015-AC-110) deletes the finite-collection lowerer, an input
    longer than `max_items` is `ResourceExhausted`, with no query result or generated artifact.
- id: FR-015-AC-90
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm ends at its own slice,
    FR-015-AC-109 to FR-015-AC-111), one admitted arithmetic, graph or collection case emits all four
    artifacts (oracle, finite strategy, Kani harness and proof-dependency graph) from the same selected
    profile, validated finite input and family lowering.
- id: FR-015-AC-91
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes the hand-built bounded-corpus
    path, that path returns a generation-time case classification of `Proved` when the family lowering's
    Boolean oracle is true and `Counterexample` when false; neither value is an observed Kani run or native
    replay verdict. This interim classification ends when FR-015-AC-111 deletes the path.
- id: FR-015-AC-92
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm ends at its own slice,
    FR-015-AC-109 to FR-015-AC-111), a declared dependency census with an empty or repeated identity,
    inconsistent kind/state/path or a kind other than `Required` refuses as typed `InvalidInput` with
    no artifact or case-identity claim; a valid census is normalized into the emitted proof graph.
- id: FR-015-AC-93
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes the hand-built bounded-corpus
    path, a profile/input mismatch, family-lowering refusal, proof-graph serialization refusal or duplicate
    case identity returns its typed refusal with the selected profile revision, emits no artifact and
    claims no new case identity.
- id: FR-015-AC-94
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm ends at its own slice,
    FR-015-AC-109 to FR-015-AC-111), equal case inputs produce equal artifact paths and contents regardless
    of emission order, and reordering the same finite graph's objects or references does not change them;
    changing a request field, selected profile, finite input, ceilings or normalized dependency census
    changes the case identity and artifact paths.
- id: FR-015-AC-95
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'AD-004 step 4g retires the hand-built bounded-corpus path in the three per-family slices FR-015-AC-109
    (arithmetic), FR-015-AC-110 (collection) and FR-015-AC-111 (graph, which deletes the whole public
    corpus API and the generation-time `KaniOutcome`); this criterion is the end state they reach together
    and adds no deletion of its own. The end state is: QSL-emitted packages admitted through Contract
    IR feed the one V2 generator for arithmetic, collection and graph cases; the collection and graph
    cases satisfy FR-015-AC-96 to FR-015-AC-102, including typed model-derived symbolic bounds, and each'
- id: FR-015-AC-96
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An admitted QSL V2 clause/function body that contains a `quire.op.collection.forall` or `quire.op.collection.exists`
    operation over a selected model collection field is a collection claim. For a field with typed inclusive
    cardinality `[minimum, maximum]` and supported finite elements, the generator emits a harness with
    symbolic length and symbolic elements. Its assumptions admit both cardinality endpoints and every
    length between them; for `IntRange` elements they admit both inclusive integer endpoints and every
    integer between them, and for `Boolean` elements they admit both `false` and `true
- id: FR-015-AC-97
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: For an admitted collection claim whose selected field has multiplicity lower bound `0` and
    no finite upper bound, QSL admits an unbounded collection type; when its element type is supported,
    this generator settles `requires_bound` naming the selected field or unbounded collection type and
    emits no harness. An admitted field with no finite element domain likewise settles `requires_bound`
    naming that element type and emits no harness. A field with multiplicity lower bound greater than
    `0` and no finite upper bound has no QSL kernel type ([QSL FR-056-AC-10](ix://agent-ix/quire-spec-language/FR-05
- id: FR-015-AC-98
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A graph claim (one reachability application of either form above, at its `state_clause` node
    id and argument path) generates a harness when the admitted package gives its covering object type
    `E` a finite population `[0, N]` through `select_population` and the selected edge has finite multiplicity
    (for `quire.op.model.reaches_field` the field's `Reference`, `Option` or bounded `Collection` member;
    for `quire.op.model.reaches` the destination end's finite multiplicity). Every symbolic object and
    edge is bounded inclusively by those declarations, including zero and maximum where admitted. Reacha
- id: FR-015-AC-99
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The collection harness asserts the admitted clause/function body over symbolic inputs drawn
    from the selected model field's typed domain, ends with the FR-015-AC-7 cover after its assertions,
    and never replaces those inputs with the finite-input corpus's concrete `Vec<i128>`. The independent
    native oracle replays the same authored function through `qsl_replay::replay`; it does not substitute
    a separately authored predicate. A healthy QSL-emitted admitted V2 case yields installed-Kani `Verified`
    with its cover satisfied; a seeded claim or subject mutant yields `Falsified` with a concrete in-dom
- id: FR-015-AC-100
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: For the collection family, changing the selected field's authored cardinality or element bound,
    admitted body claim, customer subject or unwind changes the harness identity; equal admitted inputs
    regenerate byte-identically. The identity records every symbolic collection domain and the solver
    and complete ordered option vector, and FR-017 run evidence records the actual Kani, Rust and solver
    versions, unwind and every option used by the installed backend. For the graph family (AC-98), changing
    the claim's clause id or argument path, the reachability form, relationship or field, its role or
    nam
- id: FR-015-AC-101
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A request whose symbolic collection domain exceeds the generator''s representational or resource
    ceiling settles `unsupported` with a typed reason naming the failing domain or ceiling and no harness;
    it never tightens the model domain to make a proof tractable. A missing or malformed selected declaration
    likewise refuses with its typed reason and no harness. For a graph claim, a population or edge domain
    whose symbolic object or edge count exceeds the ceiling likewise settles `unsupported` naming the
    domain, with no harness and no reduced population or multiplicity. PARTIAL (IR-268, CG #396 fe2'
- id: FR-015-AC-102
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'An admitted collection body claim is settled through the FR-015 V2 obligation path, one disposition
    per requested item; supported items alone emit harnesses. The finite-input lowerer and hand-built
    corpus of AC-86 to AC-95 neither supply the symbolic bounds nor classify the proof verdict. Graph
    items take the same V2 path (IR-805), one disposition per requested item, and are not implementation
    or proof evidence for IR-268. IMPLEMENTED for the collection family (IR-268, CG #396 fe20ecc): `tests/it/kani_obligations_symbolic_domains.rs`
    `collection_negotiation_accounts_for_each_supported_or_refus'
- id: FR-015-AC-103
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Through `generate_routed`, `(x + 1) + 1` over `x: Int[0, 9]` into `Int[0, 20]`, `x * -1` over
    `x: Int[1, 9]` into `Int[-9, -1]`, and `-(x + 1)` over `x: Int[0, 9]` into `Int[-10, -1]` each settle
    `Supported` with an IR-confirmed outer oracle and a harness. Their respective argument ranges are
    `[1, 10]` and `[1, 1]`, `[1, 9]` and `[-1, -1]`, and `[1, 10]`; the result assertions use only the
    outer declared result bounds. Each inferred range carries its source application id in the harness
    identity.'
- id: FR-015-AC-104
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A plain-Integer parameter as any nested leaf keeps `requires_bound` with the Integer `unbounded_type`
    and no harness; a direct reference to a raw application never borrows a separate narrowing conversion's
    domain, and an inferred endpoint outside `i64` settles `unsupported` as `domain_not_representable_in_i64`
    naming both exact endpoints. `OperandRangeLimitExceeded`, `OperandUnsupported`, `OperandTypeMismatch`
    and `InvalidBody` settle `unsupported` with `oracle_refused` carrying the exact scalar refusal, including
    its node, position or limit fields, and no harness. Refused items leave a suppor
- id: FR-015-AC-105
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'For a graph claim the harness draws an object count in `[0, N]` of the one covering population
    and symbolic objects within it; for every object, the selected edge as its shape states: a `Reference`
    field has exactly one target, an `Option` field an absent or one target, a `Collection` field or finite
    multi-valued relationship end a count in `[lower, upper]` (a unique end admits no repeated target,
    a non-unique end repeats, and `lower > 0`, such as `{2,3}`, forces at least `lower` targets) and every
    target is an object of the population. Start and target are drawn from that population and may b'
- id: FR-015-AC-106
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'The independent native oracle replays the same authored clause over each original-domain concrete
    graph through QSL''s replay (`qsl_replay`), and does not substitute a separately authored reachability.
    With the installed backend, a QSL-emitted, IR-admitted healthy case yields `Verified` with its cover
    satisfied for one field-edge claim and one relationship-end claim over a `{0,3}` unique and a `{0,3}`
    non-unique end. Each semantic mutant yields `Falsified` at the clause assertion with an original-domain
    playback and native replay of that same authored clause: reachability that allows a zero-len'
- id: FR-015-AC-107
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Graph claims are located by `state_clause` node id and argument path: one reachability written
    twice in one condition yields two items at positions 0 and 1; one node under two clauses yields two
    items with different identities; a field-edge and a relationship-end reachability yield separate items;
    a collection claim, a clause with no reachability and a path that does not name a reachability application
    each settle `unsupported` naming the missing claim with no harness. A mixed request settles one disposition
    per item and a supported item''s harness is unchanged by a refused sibling. PLANNED (IR'
- id: FR-015-AC-108
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A graph claim settles by the disposition sequence in the Description (the paragraph "Disposition
    sequence for a graph claim"), with no harness, domain reduction or caller `max_expansions` substitute.
    While the population selection for the claim is `Unavailable` (no available population selection for
    this admitted claim), the claim settles `unsupported` naming it. Given a selection, `Unbounded` or
    `UnknownClosureOpenExtent`, that is a claim whose population has no finite authored bound (once a
    selection is available), settles `requires_bound` naming the population; an unbounded `Collection`
    edg
- id: FR-015-AC-109
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-598 arithmetic slice only; merged IR-858 satisfies AD-004 step 4b). After
    this slice no source under `src/` contains the corpus `Arithmetic` request variant and arm, `BoundedCorpusFamily::DefinednessArithmetic`,
    `prepare_checked_arithmetic`, `checked_method`, `render_arithmetic_*` or `lower/definedness_arithmetic.rs`
    and their re-exports, and no test in `tests/` drives them. The slice deletes only what the arithmetic
    family owns: the corpus replay (`BoundedCorpusCase::replay_counterexample`, `compile_corpus_qsl_package`,
    `BoundedCorpusReplayError` and the replay-harness identit'
findings:
- fnd: FND-073
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-111
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '847'
  excerpt: PLANNED (IR-598 graph and final slice; gated on IR-858, AD-004 step 4b, on FR-015-AC-109 and
    FR-015-AC-110 having landed, and on IR-805, whose graph claims need CG integration of the merged IR-851/IR-852
    accessors and QSL-820 real-package population emission (after QSL-819)). Graph claims are served by
    FR-015-AC-98 and FR-015-AC-105 to FR-015-AC-108, and FR-015-AC-106's real-Kani graph replay is the
    replacement for the corpus graph replay; Evidence for a real-Kani precondition is a recorded `make
    kani` or `make kani-gate` run on the slice PR's head that is green, cited in the PR (the real-Kani
  finding: 'Inherited FR-015-AC-111 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-074
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-117
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '853'
  excerpt: Implemented (IR-858). With the installed backend, after the scalar port, the unmutated scalar
    harness classifies `Verified` with its cover satisfied and the existing scalar mutation control (the
    oracle's `Add` swapped for `Subtract`, FR-015-AC-37) classifies `Falsified`; and a hand-built `HarnessSpec`
    with an unsatisfiable `Assume`, rendered by `render`, does not classify `Verified` (L-4). This run
    is a smoke over the ported source and not the evidence of equivalence, which FR-015-AC-115 is (AD-004
    "The HarnessSpec and render contract (step 4b)", P-3).
  finding: 'Inherited FR-015-AC-117 is tagged-by-ignored-test at the frozen candidate: every bound test
    is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or
    qualify that obligation.'
- fnd: FND-075
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-119
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '855'
  excerpt: 'PLANNED (IR-917; depends on FR-015-AC-40 and FR-015-AC-41). QSL source authors an operation
    with one parameter `amount: Int[0, 1000]` and its V2 postcondition `amount < 1000 implies amount +
    1 <= 1000`; both occurrences of `amount` read that operation parameter, not a state field or subject-produced
    post-state value. The admitted `CheckedPackageV2` yields a clause-identified FR-014-AC-38 oracle and
    an FR-015 postcondition harness for that same clause, not a scalar implication harness. Under FR-015-AC-39
    the harness draws the parameter once within its declared finite domain and passes that same'
  finding: 'Inherited FR-015-AC-119 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-076
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-120
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '856'
  excerpt: 'PLANNED (IR-942; depends on QSL ADR-013 O-07/O-09 and QSV FR-123). After the selected owner
    and occurrence validate under AC-122, CG forms the per-step record content identity from an RFC 8785
    object whose only members are `subject_kind: "protocol-refinement-step"` and `record: StepSimulationRecord`
    encoded by QSV. Two separately valid selected refinements with the same obligation and hypotheses
    but different record `refinement` values, each with its own matching occurrence, yield different content
    identities; a record with a mismatched owner has no identity to compare. Within one admitted ref'
  finding: 'Inherited FR-015-AC-120 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-077
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-121
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '857'
  excerpt: PLANNED (IR-942). A per-step `KaniObligationIdentity` uses the QSV-owned `StepSimulationRecord`
    as a typed subject, carries its content identity and the owning refinement's O-07 `generated` occurrence
    key separately, and keeps any CG proof or obligation kind in the harness record rather than the record
    content preimage. CG publishes a persisted harness-record digest over every `KaniObligationIdentity`
    member except `source_span`; changing any included member changes this digest, while changing only
    `source_span` does not. This digest is distinct from the per-step record content identity. The r
  finding: 'Inherited FR-015-AC-121 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-078
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-122
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '858'
  excerpt: 'PLANNED (IR-942). A selected proof-eligible refinement under QSpec FR-322-AC-88 through AC-90
    with an `Initial` record and `Step { row: i }` records for its admitted FR-375 `steps` order yields
    generated occurrences on the owner node at ordinals 0 and `i + 1`, respectively. An ineligible refinement
    under FR-322-AC-91 (hidden field, history field, or `any` step target) carries only its `declaration`
    occurrence and yields no per-step record, harness, or proof credit; the IR reader refuses a generated
    occurrence on it as `invalid_package`/`invalid-value` before CG setup. Two step rows tied on nod'
  finding: 'Inherited FR-015-AC-122 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-079
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-017-AC-7
  related_ids: []
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '859'
  excerpt: A crate whose library source does not contain the harness source byte for byte is refused,
    and no backend runs.
  finding: 'Inherited FR-017-AC-7 is tagged-by-ignored-test at the frozen candidate: every bound test
    is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or
    qualify that obligation.'
- fnd: FND-080
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-017-AC-19
  related_ids: []
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '868'
  excerpt: The generator appends `-Z unstable-options --export-json /proc/self/fd/N` after unchanged harness
    options, with actual child-only anonymous pipe writer N >= 5, and records the actual argv bytes. It
    uses per-run unnamed report authority, never a named target-directory report or fallback. FR-034 AC-32/33
    own collector bounds, resource-stop classification, kernel lifetime and immutable final handoff. Zero-byte
    EOF means no exported report; nonempty partial/malformed content is refused under AC-18 unless an
    independently established resource/deadline stop takes precedence with the existing single-
  finding: 'Inherited FR-017-AC-19 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-081
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-017-AC-26
  related_ids: []
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '875'
  excerpt: A generated FR-035 caller Text-admission harness enters the same production execution path
    in single and compatible batch runs; its evidence carries the exact typed caller identity (profile,
    bounds and finite payload class) supplied with that harness and `None` for contract obligation kind.
    A matching successful report with a satisfied cover and successful agreement assertion classifies
    `Verified`; an unsatisfied cover classifies `CoverUnsatisfied` with counts; a failed agreement assertion
    with its own concrete playback classifies `Falsified` with that playback. A source-mismatched harness,
    mi
  finding: 'Inherited FR-017-AC-26 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-082
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-017-AC-27
  related_ids: []
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '876'
  excerpt: A genuine routed `x + 1` scalar harness from `generate_routed`, its complete emitted oracle
    package and one in-range concrete operand are passed to the paired runner. One real Kani run reports
    `Verified`, and the runner invokes the public oracle from the same staged library source at that operand
    with the request's exact `ScalarLimits`; the receipt contains the Kani evidence and the actual typed
    runtime result of that invocation. A Kani verdict alone or a result reconstructed by the caller is
    insufficient.
  finding: 'Inherited FR-017-AC-27 is tagged-by-ignored-test at the frozen candidate: every bound test
    is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or
    qualify that obligation.'
- fnd: FND-083
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-017-AC-30
  related_ids: []
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '879'
  excerpt: The public receipt exposes the measured identity, scalar identity, ordered operands, exact
    supplied and meter-used `ScalarLimits`, verified Kani evidence and actual typed native outcome through
    read-only access. Callers cannot construct, deserialize or mutate a receipt, set its identity or outcome,
    or turn any non-verified, refused, inconclusive, faulted or malformed run into a receipt.
  finding: 'Inherited FR-017-AC-30 is tagged-by-ignored-test at the frozen candidate: every bound test
    is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or
    qualify that obligation.'
- fnd: FND-084
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-017-AC-47
  related_ids: []
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '896'
  excerpt: Two receipts for the same package and harness with different operands, different `ScalarLimits`
    and different native outcomes have equal `DigestRecord`s; operand order, limits, ranges, outcome and
    Kani evidence appear nowhere in the preimage. This holds for the swapped-operand runs of FR-017-AC-31.
  finding: 'Inherited FR-017-AC-47 is tagged-by-ignored-test at the frozen candidate: every bound test
    is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or
    qualify that obligation.'
- fnd: FND-085
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-017-AC-52
  related_ids: []
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '901'
  excerpt: Production emitted bytes and feature-free exact dependency remain unchanged. The isolated private
    observer uses resolver 2 and its sole example compiles the single canonical adapter in place through
    `../src/bin/ir791_native.rs`, with runtime `exact` and kernel `test-support` exposed only by dev-dependencies;
    no wrapper, copied source, symlink, normal/build logging edge or CLI feature override is allowed.
    Native invocation explicitly names `observer/Cargo.toml` and the example; the real production proof
    retains its harness, ordered options, claims, covers, bounds and backend gates. A control re
  finding: 'Inherited FR-017-AC-52 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-086
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-017-AC-53
  related_ids: []
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '902'
  excerpt: Both package locks are independently freshly resolved with the existing bounded offline generation/update
    workflow, without copying a parent/other lock or substituting a cached resolution. Each resolves both
    owning runtime and exact identities to the build's authoritative lock identities; the independent
    observer regression asserts both literal package names and repository identities independently of
    the renderer's list. Both manifests, locks and all other AC-46 entries are sealed and compared as
    a complete path/byte set before proof, after proof and after native execution; mutation/replacemen
  finding: 'Inherited FR-017-AC-53 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-087
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-017-CON-1
  related_ids: []
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '848'
  excerpt: The generator SHALL NOT convert a non-verified outcome into a proof claim. | Integrity
  finding: 'Inherited FR-017-CON-1 is tagged-by-ignored-test at the frozen candidate: every bound test
    is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or
    qualify that obligation.'
- fnd: FND-088
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-21
  related_ids: []
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '689'
  excerpt: 'Record the closure from a second reading of the package (a second reader that reads presence
    differently), so the shadow of FR-015 and the production oracle disagree on presence and each is correct
    against its own reading. Member order is not a mutant: a permutation changes no compared observable.'
  finding: 'Inherited FR-018-AC-21 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-089
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-22
  related_ids: []
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '690'
  excerpt: Refuse or stop a legal pair (an `absent` against a `null` slot, two `none`, a nested option)
    as `Refused(CheckedInvariant)`, charge one pair too many for a `some`/`some` option or too few for
    an unequal pair that stops early, or complete with the verdict of the other operator.
  finding: 'Inherited FR-018-AC-22 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-090
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-23
  related_ids: []
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '691'
  excerpt: Check the closure against the generator's own reconstruction (the check of AC-21 alone), so
    a reader that narrows a bound or reads `T?` as `T` shrinks the shadow's domain, the refinement domain
    and the oracle's declaration together and every check agrees.
  finding: 'Inherited FR-018-AC-23 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
bindings: []
review_head: 3d303ea6988496c4e1102faa58a56c1f260193c3
review_verdict: 'FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence
  gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls
  now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.'
part: 5
parts: 18
scope_kind: full-repository criterion presence; no blanket semantic acceptance
```

+++