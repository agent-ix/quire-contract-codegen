---
id: SR-14724
title: "IR-17 optional admitted capability — dependency review"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-contract-codegen; IR-17; spec/assurance/AD-001-codegen-architecture.md, spec/assurance/AD-004-cg-crate-layout.md, spec/core/functional/interface-001-codegen-api.md, spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/matrix/TC-029-composite-equality-oracles.md, spec/spec.md"
review_set: subset
---

## Summary

Reviewed PR490's optional admitted capability and direct Integer root clarification. No defects found in this method's specified scope.

## Verdict

**PASS** — The only new relationship is a references edge to QSpec FR-149, which exists and owns bounded Int-to-Integer versus forbidden unbounded narrowing equality conversion. It creates no reverse prerequisite or cycle. Existing FR-014 enablement precedes FR-018 feature generation; genuine QSL context enables nominal identity resolution, and QSL-937 still blocks complete record/tuple adoption. Standalone optionality does not pretend the enum facade unblocks composite key ownership.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope examined

```yaml
scope:
- id: FR-018-AC-1
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Every requested item receives exactly one generated or typed-refused disposition; a refused item contributes
    no function, no environment constructor and no symbol to the generated crate, while its siblings are generated
    unchanged.
- id: FR-018-AC-2
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'On every vector of the composite-equality corpus, the generated oracle''s `Outcome<bool>`, its admitted
    charge sequence and its consumed counters are equal to those of `quire_contract_runtime::exact::TypeEnvironment::check_equality`
    plus `CheckedEquality::evaluate` invoked directly on an independently constructed environment, operands and
    fresh `Meter`. This criterion claims no agreement with the QSL value authority: that boundary is owned by quire-integration
    IR-842 (FR-014-AC-47).'
- id: FR-018-AC-3
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Each generated item's claim-map entry records the descriptor the request supplied — operator, both operand
    source types and both conversion targets — and it is equal to the request's, so the independent native run of
    AC-2 is driven from the request and never read back out of the generated crate. An authentic asymmetric source
    control preserves the actual bounded-left/unbounded-Integer-right source IDs and independent caller target;
    a source conversion erased from the emitted graph is not evidence of a retained conversion-reference branch.
- id: FR-018-AC-4
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Each generated item's claim-map entry records an `EqualitySchedule` equal to `CheckedEquality::schedule()`
    of the `CheckedEquality` its descriptor admits, for a top-level text, enum and quantity pair and for each `Plan`-schedule
    composite and collection shape.
- id: FR-018-AC-5
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A descriptor with a `convert<T>` operand outside `admits_equality_conversion`, distinct text profiles,
    distinct enum declarations, incompatible dimensions, distinct units, or no common type is refused at generation
    time with its `IllTypedCause`, emits no code, and admits no charge on any `Meter`.
- id: FR-018-AC-6
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: An operand type bearing an IEEE value at any depth — including a `float64` field of a record nested inside
    a `sequence` — is refused as `OperatorIneligible`, and the refusal agrees with `TypeEnvironment::contains_ieee`
    on the reconstructed type at every depth in the corpus.
- id: FR-018-AC-7
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: On the FR-330 package path, model, function, state and temporal roots outside the equality lowering profile
    retain their exact typed `Unsupported` feature-accounting record and source occurrences in `PackageAccounting`;
    each has its own requested key and node tag, no symbol or oracle source, and does not change an independent
    admitted sibling. A `reference` composite rejected by Contract IR's strict reader is a reader-stage `InvalidPackage/InvalidValue`
    control, not a CG blocker. A lowered expression of form `call` is refused as blocked on quire-contract-runtime#34
    with no oracle.
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A declaration closure `TypeEnvironment::new` refuses is refused at generation time carrying that `DeclarationCause`,
    including both recursion passes and a duplicate record field; and each generated oracle calls `TypeEnvironment::check_type`
    on both operand comparison types before `check_equality`, so calling it with an empty environment, one omitting
    a key its operand types reach, or one declaring that closure under a different key, yields `Outcome::Refused(Refusal::CheckedInvariant)`
    with no charge admitted, rather than a panic or a completed Boolean.
- id: FR-018-AC-9
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'Every generated oracle function returns `Outcome<bool>`, never `bool`: with a denial injected at each
    of `equality.plan-form`, `equality.plan`, `equality.pair`, `equality.result-retain` and each conversion charge
    point in turn, the oracle yields `Outcome::Incomplete` naming that point, the denied charge is not applied —
    every counter equals those of the same run stopped immediately before that point — and never a completed Boolean.'
- id: FR-018-AC-10
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Generated bytes are identical across repeated runs and across permutations of the request order, including
    a request carrying two descriptors over one node id, and claim-map entries are ordered by the descriptor key
    — expression node id, operator rank, then each operand's source-type and conversion-target node ids, every node
    id compared in node-id order; the generated crate is compiled and executed under AC-2 against a native run driven
    from the request's own descriptor (AC-3), so a generated crate with a changed emitted operator, operand order
    or descriptor fails AC-2.
- id: FR-018-AC-11
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Every claim marks its operation `caller_declared`, the claim map carries the blocked item "operation
    identity not consumed by codegen's generators", and two items over one node differing only in `EqualityOperator`
    both generate with that mark, under distinct symbols, each built from its own operator's stem, and produce complementary
    outcomes on a vector whose operands differ.
- id: FR-018-AC-12
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The generated crate declares `publish = false`, names direct exact/QSV and root replay dependencies through
    the one manifest authority, contains no charge amount and no planned pair count (every charge and every pair
    comes from runtime metering), and compiles.
- id: FR-018-AC-13
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Every claim-map entry carries the node id, IR id, package id, source map, claims, reconstructed declaration
    keys and selected schedule of its item, and its genuine owner-issued declaration keys equal the exact domain
    and digest of the V2 node ids its operand types reach, selected through owning lookup without CG/emitted key
    construction.
- id: FR-018-AC-14
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Admitted `bounded_domain` nodes an operand type reaches retain the exact named bound values FR-014 specifies.
    For `integer_range` and `collection_bounds`, Contract IR FR-038 admission requires exactly `min` then `max`
    bindings with their canonical typed integer literals; a reordered, bare, missing, duplicate or unlisted binding
    control is an IR-intake refusal before CG, with no admitted package or public CG `UnreadableBound` result. CG
    private parser tests retain their precise malformed-bound defenses separately and do not count as public-generator
    acceptance for these reader-ineligible inputs
- id: FR-018-AC-15
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'A `binary` node''s operand is read through its `reference`: a reference to a `convert` expression node
    is read as the type of what it converts, following nested conversions to the first node that is not a conversion,
    and a reference to any other node, including a non-`convert` application, as that node''s own `semantic_type`.
    A descriptor whose `source_type` is the read type generates, and any other source is refused as an operand-type
    disagreement at that position, reporting the read type as found. A genuine source fixture with no emitted convert
    node exercises its actual parameter source types'
- id: FR-018-AC-16
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'A tuple position whose type node is a `text_bounds` `bounded_domain` node generates the declaration
    `Text(min, max, profile)` read from that node''s own members, an integer, decimal or rational `bounded_domain`
    member reads as the same type as a member naming its base scalar, and a second bound over the same base is never
    read: naming either of two `text_bounds` nodes over one text scalar reads that node''s own `min` and `max`.
    A `bounded_domain` member whose form does not fit its base scalar is refused as missing the form the base reads,
    and one over a boolean scalar, over QSL''s enum declaratio'
- id: FR-018-AC-17
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The `src/lib.rs` that `generate_composite_equality_oracles` returns for the TC-029 step 1 package (every
    generated item) contains zero occurrences of the panic tokens FR-018-AC-19 lists, no `.ok()`, `.unwrap_or(`
    or `.unwrap_or_default(` anywhere, and no `[` index or slice expression directly after an identifier character,
    `)` or `]`; every reconstruction helper in it returns `Result<_, ReconstructionError>`, and each of `rebuild_integer`,
    `rebuild_interval`, `rebuild_rational`, `rebuild_decimal`, `rebuild_text` and `rebuild_cardinality` turns a
    failed constructor or parse into exactly the mat
- id: FR-018-AC-18
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '`render_value_type` called with `ValueType::Quantity` and with `ValueType::Reference` returns `Err(RenderError::UnsupportedValueType
    { family })` with `family` equal to `"quantity"` and `"reference"` respectively and does not panic, and the
    item-boundary mapping turns `RenderError::UnsupportedValueType` into `CompositeEqualityRefusal::Unsupported`
    with `node_tag` equal to `family` and `unsupported_node_id` equal to the item''s expression node id, emitting
    no code for the item and leaving its siblings unchanged, while `RenderError::Generation(OracleGenerationError::UnknownRuntimeVariant
    { .. })`'
- id: FR-018-AC-19
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'The non-test code of `src/oracle/equality/mod.rs` (comments and every `#[cfg(test)]` item removed wherever
    the item sits, string literals the generator emits counted) contains zero panic tokens: the identifiers `unwrap`,
    `expect`, `unwrap_unchecked`, `unwrap_err`, `expect_err`, `unwrap_err_unchecked`, `panic_any` and `resume_unwind`
    however written (called, with whitespace before the paren, named on a path such as `Option::unwrap`, or imported);
    the macros `panic`, `unreachable`, `todo`, `unimplemented`, `assert`, `assert_eq`, `assert_ne`, `debug_assert`,
    `debug_assert_eq` and `debug_assert_ne'
- id: FR-018-AC-20
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: When an admitted FR-330 package contains a `Failed` feature-accounting record, its equality claim is
    refused as `CompositeEqualityRefusal::PackageAccounting` with the exact `limit_kind`, `limit`, `consumed` and
    source occurrences from that record; it is never relabelled as work exhaustion. When a checked package is admitted
    under a byte ceiling one byte below its otherwise canonical lowered contract package length, Contract IR FR-038-AC-95
    returns `Failed(Bytes)` for every requested root with that ceiling as `limit` and one shared `consumed` above
    it, and exposes neither canonical package byte
- id: FR-018-AC-21
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Each generated item's claim-map entry records its declaration closure as every declaration's key, form
    (record, tuple or option), members in declaration order with each member's name, presence and type, and each
    leaf's family and declared bound, and the closure equals member by member (name, presence, value type, order)
    the declarations of the `TypeEnvironment` the item's generated environment constructor returns, for every item
    of the TC-029 corpus whose leaves are Boolean or bounded integers. PLANNED (IR-264).
- id: FR-018-AC-22
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'For every pair of operand values in the item''s refinement domain (FR-028-AC-16: every case where the
    domain fits the case cap, the boundary and seeded cases otherwise; every presence state of an option and of
    an optional field included) of an item whose leaves are Boolean or bounded integers, the generated oracle called
    with its own environment and a `Meter` whose every limit is `u64::MAX` returns `Outcome::Completed` with the
    verdict of QSpec FR-149 for the item''s operator and admits exactly `equality.plan-form`, `equality.plan`, one
    `equality.pair` per node of the occurrence-pair tree, then '
- id: FR-018-AC-23
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: For every item of the TC-029 corpus whose leaves are Boolean or bounded integers, the claim-map closure's
    member names, presence and each integer leaf's inclusive bounds equal those of an independent read of the checked
    package (the V2 composite and `bounded_domain` nodes read through Contract IR's reader, never through the generator's
    reconstruction); a closure that dropped a member, narrowed a bound by one or read an optional member as required
    fails the check. PLANNED (IR-264).
- id: FR-018-AC-24
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Type reconstruction of QSpec's `positive-recursive-records.json` List (through `Option<List>`) and Tree
    (through `Sequence<Tree>` bounded `[0,3]`) closes each back-edge at the in-progress record key, generates equality
    items built over both record types, and terminates; a synthetic unclosed `Option` self-cycle and a `Sequence`
    self-cycle each refuse as `TypeResolutionCycle` naming the repeated type node, without a panic, stack overflow
    or generated symbol. PLANNED (IR-643).
- id: FR-018-AC-25
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'The per-item type-resolution counter charges one unit for each root type-node entry or followed type-node
    reference or bounded-domain `semantic_type` edge, including a repeated or memoized reference, across both operands
    and conversion targets, independently of Contract IR lowering; with the default limit, an acyclic chain reaches
    its 65,537th entry and refuses only its item as `TypeResolutionWorkExhausted { limit: 65_536, consumed: 65_537
    }` before further descent and without call-stack exhaustion or generated symbols, while a sibling needing at
    most 65,536 entries still generates. PLANNED (I'
- id: FR-018-AC-26
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Reading the QSpec List `next` field as `binding(next, aggregate([binding(optional, reference Option<List>)]))`
    yields exactly one `FieldDeclaration` named `next`, with `Presence::Optional` and payload type `ValueType::Composite(List
    NodeKey)`; removing or renaming `optional`, adding a second nested member, or targeting a non-option node refuses
    only that item as malformed, never as a required field or a generated oracle. PLANNED (IR-643).
- id: FR-018-AC-27
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Reading the QSpec Tree `kids` field's direct reference to `collection_bounds` follows that node's `semantic_type`
    to `Sequence<Tree>`, reads its sole element reference as the Tree `NodeKey`, and reconstructs cardinality `[0,3]`
    from that bound node's named `min` and `max`; changing the semantic type to a non-sequence, dropping or duplicating
    either bound, or making either value non-integer refuses only that item with a typed cause and emits no code,
    never silently yielding an unbounded collection. PLANNED (IR-643).
- id: FR-018-AC-28
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: With a caller-selected type-resolution work limit of at least 100,000 and `cg.equality.source_bytes`
    raised to the measured flat source size, an admitted acyclic type chain of 100,000 entries reconstructs and
    generates an oracle with the same result as an equivalent shallow type on a 512 KiB stack; with the same raised
    source-byte limit, the default type-resolution work limit refuses it at entry 65,537, and a selected work limit
    one below its entry count refuses it at the first denied entry. No depth setting or fixed depth refusal participates
    in these runs. PLANNED (IR-511).
- id: FR-018-AC-29
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A cycle detected after a charged repeated entry reports `TypeResolutionCycle` without following a child;
    if that repeated entry is the first denied work entry, `TypeResolutionWorkExhausted` wins and reports the effective
    limit and first denied count. An unaffected sibling keeps its original disposition in either case. PLANNED (IR-511).
- id: FR-018-AC-30
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The 100,000-entry acyclic closure renders as flat source without a type-depth-sized constructor expression;
    under the 1,048,576-byte default it returns `SourceTooLarge` naming that limit when the source exceeds it, and
    under a caller limit equal to its measured source bytes it emits a complete, compilable oracle within the publication
    artifact and bundle byte limits. One byte less refuses before that byte is appended and publishes no partial
    artifact. PLANNED (IR-511).
- id: FR-018-AC-31
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A caller-selected type-resolution work limit of `u64::MAX` refuses as `InvalidTypeResolutionWorkLimit`
    before lowering or charging; with `u64::MAX - 1`, the first denied count fits in the `u64` `consumed` field
    and is reported without wraparound. PLANNED (IR-511).
- id: FR-018-AC-32
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Each IR-admitted direct `temporal`/`formula`, `temporal`/`fairness` and `expression`/`case` request has
    one claim-map entry keyed by its requested node id and refused as `CompositeEqualityRefusal::Unsupported`. For
    an IR `Unsupported` lowering record, the refusal's `unsupported_node_id` and `node_tag` equal that record's
    first unsupported reachable node and tag, which need not be the requested root; for a lowered `case` root they
    name the root and `"expression"`. The request contributes no generated symbol or source, never panics or disappears,
    and leaves a supported sibling's claim and source
- id: FR-018-AC-33
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A genuine root-facade compilation of authentic retained source, original dependencies and admitted request
    limits has equal compiled/context/original package identities and exact full recompiled/original CheckedPackageV2
    equality before its context is exposed. An explicitly supplied different context package refuses generation
    and emitted environment construction before classification or lookup, including primitive-only requests, publishes
    no prepared value, and cannot evaluate with a completed Boolean or meter charge.
- id: FR-018-AC-34
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The actual generated ABI executes ordered and unordered enum comparisons over all declared cases, including
    a case absent from every selected literal and an enum parameter with no member literals. Exact shape variants
    and canonical positions come from genuine facade/QSV values. Same-spelled cases from distinct declarations retain
    distinct identities; matching and mismatching declaration comparisons preserve the owning equality disposition.
- id: FR-018-AC-35
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Unknown declaration, unknown case and a directly or transitively imported declaration produce the original
    typed Unknown/Foreign details at consumed and generated lookup boundaries. Malformed source/dependency input,
    QSL stage or above-reader resource limits, and internal Fault remain Package, Resource and Fault at the source-retaining
    boundary; a subsequent IR read stop remains the separate PackageReadLimit, with no partial context or successful
    prepared wrapper. The current ImportedContextUnsupported guard remains until original admitted dependency packages
    are retained.
- id: FR-018-AC-36
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Emitted source uses only WireNodeId and case identifiers for lookup and contains no NodeKey constructor/decoder
    or generated nominal preimage/hash. Only the prepared wrapper's private genuine index reaches check_equality.
    A private environment-only fault fixture still satisfies AC-8, every original charge-stop case still satisfies
    AC-9, and a wrong item/descriptor prepared wrapper refuses before charging. Record/tuple/recursive/collection
    cases remain required by AC-2 and AC-21 through AC-30; an absent owner lookup capability is recorded as unresolved
    migration work, not as accepted Unsupporte
- id: FR-018-AC-37
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Equality's generated manifest and native scratch consumers use the one profile authority's direct exact/QSV/replay
    dependencies, while ordinary Boolean/integer FR-014 scalar proof manifests and oracle bodies acquire no replay
    source-compilation/context leg. Native generated enum acceptance is reported separately from original TC025/TC027
    proof qualification; neither result substitutes for the other.
- id: FR-018-AC-38
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: With `None`, standalone generation and emitted preparation accept successfully reconstructed non-nominal
    primitive and nested Option/Collection closures; their index is empty only after complete zero-enum resolution.
    Existing admitted-IR conversion-form and quantity-family negatives retain respectively exact FormMismatch("conversion")
    and Unsupported("quantity"), with no replacement ContextPackageMismatch or MissingAdmittedContext, no symbols
    for the refused item and unchanged independent siblings. This does not claim matching QSL source emission or
    source authentication for those IR fixtures.
- id: FR-018-AC-39
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '`None` at the first reached nominal identity returns typed MissingAdmittedContext naming that declaration.
    Cover left and right source operands, left and right conversion targets from primitive sources, nested Option
    and Collection enum types, and eventual record/tuple field and recursive closures through their genuine owner
    capability. No key, default/partial index, omitted nominal declaration or Unsupported substitute is produced.
    Retain the existing charged-entry/cycle/error precedence and unaffected sibling results; the separate missing
    record/tuple owner lookup remains outstanding.'
- id: FR-018-AC-40
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Some(wrong-package context) fails the whole standalone call after existing setting/accounting checks
    and before item classification, for primitive-only, nominal and empty requests. Emitted context-free and nominal
    constructors both reject the same supplied mismatch before reconstruction. None and Some(wrong) have distinct
    outcomes; no failure publishes partial artifacts or a prepared wrapper.
- id: FR-018-AC-41
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A reached enum still reconstructs every owner-admitted case, including unused cases and an enum parameter
    with no member literal, using the supplied genuine keys and canonical positions. Failed nominal admission/lookup/traversal
    cannot yield an empty-index success. A successfully resolved zero-enum closure may use an empty index; merely
    inspecting an outer primitive/Option/Collection tag does not establish zero enums.
- id: FR-018-AC-42
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Generated oracle invocations under Some or valid context-free None preserve private package/item/descriptor/index
    binding, both check_type guards, same-index check_equality and evaluate. Private wrong-binding, present-context
    mismatch, missing nominal context and environment-only faults refuse without charge. The native OriginalCompositeEqContext
    route retains every mandatory authentic source/lock/package/whole-IR comparison and cannot use None to bypass
    them. No scalar proof manifest/body acquires replay compilation.
- id: FR-018-AC-43
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'For genuine source with a: Int[0,9] and b: Integer, the admitted left parameter names its integer_range
    and the right parameter names canonical base Integer. Direct source/target reconstruction preserves Int[0,9]
    versus unbounded Integer despite the sibling range: a caller descriptor converting the left to Integer compares
    with the unbounded right under the original equality rule, including right values outside [0,9]; an unbounded
    Integer source cannot acquire a narrowing conversion by borrowing that sibling interval. Claim/native descriptor
    checks remain independent of generated output. If so'
- id: AD-001 / Direct owning execution surface view / changed-unit-1
  path: spec/assurance/AD-001-codegen-architecture.md
  role: examined
  excerpt: "- FR-014 scalar oracles call the kernel scalar operations over `rt::Integer` and the other exact\n \
    \ scalar types.\n- FR-018 equality oracles call QSV `check_type`, `check_equality` with their\n  complete genuine\
    \ enum index, and `CheckedEquality::evaluate`. A generated\n  opaque prepared environment retains an optional\
    \ immutable replay-root\n  context, environment, private index, descriptors and expected package/item\n  binding.\
    \ Standalone None is valid only for context-free reconstruction;\n  every reached nominal identity requires\
    \ genuine owner context.\n- FR-021 function-application oracles admit a "
- id: AD-001 / Direct owning execution surface view / changed-unit-2
  path: spec/assurance/AD-001-codegen-architecture.md
  role: examined
  excerpt: 'Only the source-retaining preparation boundary calls root `compile_admitted`,

    authenticates original source/lock, compares the complete recompiled package

    and retains its context. That native route always supplies Some. Standalone

    CG generation takes `Option<&qsl_replay::AdmittedContext>`, not a peer

    `replay` module type: None does not authenticate source, and Some with a

    wrong original package fails before even primitive classification. Missing

    context at an actual nominal reconstruction is a typed refusal, not a key

    fallback or a reclassification as Unsupported. Emitted code resolves `WireNo'
- id: AD-004 / Dependency direction / changed-unit-1
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: 'The planned FR-018 admitted-context migration preserves this internal graph:

    `oracle` accepts `Option<&qsl_replay::AdmittedContext>` from the external root,

    not CG''s peer

    `replay::OriginalCompositeEqContext`. The existing source-retaining builder

    in `replay` authenticates the original source/lock, calls root

    `compile_admitted` under the admitted request limits, compares the complete

    original checked package, and retains the genuine compilation/context pair.

    A successful native request exposes its immutable context to the caller,

    which always passes Some to generation. Standalone None permits g'
- id: AD-004 / Dependency direction / changed-unit-2
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: 'Generated equality''s prepared environment privately holds the optional

    facade context, QSV environment and complete enum index plus expected

    package/item/descriptor binding. None is not source authentication. An empty

    index is valid only after successfully resolving a zero-enum closure; it is

    never a lookup/capability failure fallback. Nominal recipes require Some,

    including through conversion targets and nested/recursive types. Every oracle

    checks its private binding, supplied context when present and both comparison

    types before same-index equality/evaluation, refusing faults without charge.'
- id: interface_001 / Contract / changed-unit-1
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: "```yaml\nname: ContractCodegen\nversion: draft-codegen-v1\ninput:\n  contract_package: public IR BoundPackage\
    \ from the public IR derived executable projection decoder; no private wire or codegen input schema\n  configuration:\
    \ backend selection, customer bindings, output profile\noperations:\n  - name: source_obligation_census\n  \
    \  status: planned; complete producer/consumer implementation gated on IR-759 reviewed neutral correspondence\n\
    \    inputs: [SourceCensusRequest borrowing the selected admitted checked package, complete owner semantic-obligation\
    \ inventory, criterion and production-symbol inventor"
- id: FR-018 / Frontmatter / changed-unit-1
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "---\nid: FR-018\ntitle: \"Generate exact complete-V1 composite equality oracles\"\ntype: FR\nrelationships:\n\
    \  - target: ix://agent-ix/quire-contract-codegen/StR-001\n    type: satisfies\n  - target: ix://agent-ix/quire-contract-codegen/FR-014\n\
    \    type: depends_on\n  - target: ix://agent-ix/quire-contract-runtime/FR-008\n    type: references\n  - target:\
    \ ix://agent-ix/quire-contract-ir/FR-036\n    type: references\n  - target: ix://agent-ix/quire-contract-ir/FR-038\n\
    \    type: references\n  - target: ix://agent-ix/quire-specification/FR-460\n    type: references\n  - target:\
    \ ix://agent-ix/quire-specification/FR-"
- id: FR-018 / Description / changed-unit-2
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'When a caller supplies an admitted `ContractPackageV1`, an optional

    package-bound admitted identity capability, and a set

    of equality expression nodes, the code generator shall emit a deterministic Rust

    oracle crate whose functions decide each node by reconstructing the item''s

    `TypeEnvironment` and calling the QSV declaration/equality owner''s

    `TypeEnvironment::check_equality` and `CheckedEquality::evaluate` with a

    caller-supplied `Meter`. It never compares two values structurally, and never

    copies a resource charge or a planned pair count.'
- id: FR-018 / Inputs / changed-unit-3
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "- An admitted `ContractPackageV1` from Contract IR's FR-330 lowerer and strict\n  reader, with the caller's\
    \ complete root-position request and the positions\n  of the equality items in it. Its retained checked-V2 graph\
    \ supplies the\n  equality expressions and declaration closure below.\n- `Option<&qsl_replay::AdmittedContext>`:\
    \ `None` permits genuinely\n  context-free reconstruction and existing precise early refusals that need\n  no\
    \ nominal key. `Some` is the immutable genuine capability returned beside\n  the original compiled package by\
    \ root `qsl_replay::compile_admitted`; its\n  `package_id()` mus"
- id: FR-018 / Outputs / changed-unit-4
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "- A generated crate: `Cargo.toml` (`publish = false`, direct exact/QSV\n  dependencies, and root `qsl-replay`\
    \ for admitted-context lookup) and\n  `src/lib.rs`. The single `core/profile` manifest authority supplies normal\n\
    \  Git branch-main dependencies; consumer lockfiles perform ordinary resolution.\n- One public opaque `PreparedEqualityEnvironment<'ctx>`\
    \ type whose private\n  fields retain `Option<&'ctx qsl_replay::AdmittedContext>`, expected\n  original package\
    \ and item/descriptor binding, admitted QSV `TypeEnvironment`,\n  complete genuine `EnumMemberIndex`, and reconstructed\
    \ operand descriptors."
- id: FR-018 / Outputs / changed-unit-5
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "```rust\n  pub fn environment_symbol<'ctx>(\n      context: Option<&'ctx qsl_replay::AdmittedContext>,\n\
    \  ) -> Result<PreparedEqualityEnvironment<'ctx>, EnvironmentError>;"
- id: FR-018 / Outputs / changed-unit-6
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "`symbol` denotes the existing deterministic per-item generated symbol.\n  Constructors and descriptor/module\
    \ helpers thread this same optional\n  capability. A nominal recipe requires genuine context before key lookup;\n\
    \  a completely context-free recipe may publish a prepared value retaining\n  `None`. The constructor checks\
    \ a supplied context's expected original\n  package identity before reconstruction, even for primitive recipes,\
    \ and\n  publishes no partial value on failure. Expected package/item/descriptor\n  binding stays private and\
    \ checked in both cases. Recorded package identity\n  without co"
- id: FR-018 / Admitted-context ABI / changed-unit-7
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "```rust\npub fn generate_composite_equality_oracles(\n    package: &ContractPackageV1,\n    admitted:\
    \ Option<&qsl_replay::AdmittedContext>,\n    requested: &[ContractRequestPosition],\n    positions: &[usize],\n\
    \    items: &[CompositeEqualityItem],\n    limits: CompositeEqualityLimits,\n) -> Result<CompositeEqualityOracles,\
    \ OracleGenerationError>;\n```"
- id: FR-018 / Admitted-context ABI / changed-unit-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'After the existing work-setting and whole-request accounting/position

    checks, `Some` with a mismatched source package SHALL refuse the whole

    generation as the existing typed context mismatch before item classification,

    including primitive-only or empty requests. An explicitly wrong capability

    is never treated as `None`. Generated preparation applies the same supplied

    context mismatch check before reconstructing any descriptor.'
- id: FR-018 / Admitted-context ABI / changed-unit-9
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'When traversal reaches the first operation requiring an enum, record or

    tuple identity, `None` SHALL refuse only that item as

    `CompositeEqualityRefusal::MissingAdmittedContext { type_node_id }`, naming

    that nominal declaration, without constructing a key or emitting its symbols.

    Generated preparation of a nominal recipe with `None` SHALL return the

    corresponding `EnvironmentError::MissingAdmittedContext` before lookup or

    partial publication. Both operands'' source types and conversion targets,

    all nested Option payloads and collection elements, record/tuple fields and

    recursive closure edges ar'
- id: FR-018 / Admitted-context ABI / changed-unit-10
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'Capability demand SHALL occur in the existing charged traversal, immediately

    before nominal identity reconstruction/lookup, not as a preflight scan of

    outer tags, all descriptors or the complete context. Existing precise

    key-free classification and descriptor refusals remain earlier where the

    original procedure makes them earlier. Work charges, first-denied-entry and

    cycle precedence, deterministic request/descriptor order, source-byte limits

    and sibling isolation remain unchanged. A failed lookup or absent required

    capability never becomes `Unsupported`, a default key or omitted declaration.

    '
- id: FR-018 / Admitted-context ABI / changed-unit-11
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'A successfully resolved closure with zero enums SHALL use a private empty

    `EnumMemberIndex`; emptiness is a result of complete reconstruction, never a

    fallback after absent capability, stopped traversal or failed lookup. This

    includes genuine zero-enum nominal closures only when their required owner

    keys have been obtained. A reached enum SHALL construct every admitted case,

    including unused cases, from that same immutable genuine capability.'
- id: FR-018 / Admitted-context ABI / changed-unit-12
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'Every prepared wrapper SHALL retain private expected package, item and

    descriptor binding, its descriptors, admitted environment and index, and its

    optional context plus whether its recipe requires nominal identity. The

    oracle SHALL refuse a wrong private binding, a mismatched retained `Some`,

    or missing context for a nominal recipe as `Refused(CheckedInvariant)` before

    any meter charge. For a valid context-free recipe, retained `None` is not

    such a fault. The oracle SHALL check both expected comparison types against

    its prepared environment and call QSV `check_equality` with that same private'
- id: FR-018 / Admitted-context ABI / changed-unit-13
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'Standalone `None` authenticates neither source meaning nor native semantic

    agreement. It changes no strict IR admission requirement or caller-declared

    operation limitation. It SHALL NOT bypass native

    `OriginalCompositeEqContext::request` source/lock/package checks, exact full

    recompiled-package equality, typed admission failures, selected occurrence/

    operator checks or import guard. A successful native request always retains

    and supplies its genuine admitted capability. Ordinary scalar proof manifests

    and bodies acquire no replay compilation/context leg.'
- id: FR-018 / Direct Integer operand and target identity / changed-unit-14
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '### Direct Integer operand and target identity'
- id: FR-018 / Direct Integer operand and target identity / changed-unit-15
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'When a directly selected operand source or caller conversion-target ID names

    canonical `scalar_type` / `integer`, the generator SHALL reconstruct unbounded

    `Integer`, even when another parameter or context root has an `integer_range`

    over that same canonical base. When that selected ID names `integer_range`,

    the generator SHALL use that domain''s exact own interval. A package-wide

    base-to-bound index, an unrelated sibling range or a union of ranges is not

    a direct operand''s declared bound. This follows the parameter type identity

    of Contract IR FR-038 and the distinct Int/Integer conversion row'
- id: FR-018 / Direct Integer operand and target identity / changed-unit-16
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'This direct-root rule does not rewrite AC-16''s existing numeric member/base

    equivalence or its precise regression. Genuine record/tuple member association

    is separate unresolved scope: an unqualified canonical base key alone does

    not identify a lexical owner or distinguish a genuinely associated member

    bound from another parameter''s range. Do not invent that association or claim

    this operand-only clarification resolves it. The genuine record/tuple key

    capability gap also remains. Existing specified member behavior and tests

    remain required pending explicit owning resolution.'
- id: TC-029 / Package-bound enum migration controls / changed-unit-1
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'Run the real generated `environment_symbol(Some(context))` and

    `oracle_symbol(prepared, left, right, meter)` ABI, with ordered and unordered

    enums including unused cases and a parameter without member literal nodes.

    Check canonical positions and original member identities against genuine

    facade/QSV values. Use same-spelled cases under distinct declarations to

    expose conflated identity. Reject a different package context before lookup;

    reject a different item/descriptor prepared wrapper before charging. Exercise

    Unknown declaration, Unknown case and Foreign direct/transitive dependency

    identiti'
- id: TC-029 / Package-bound enum migration controls / changed-unit-2
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'Retain the empty/missing/wrong-key environment controls of step 6 and every

    charge-stop vector of step 5 with the same genuine index. Mutate generated

    operator, descriptor and case lookup independently so direct-vs-generated

    agreement fails. Check deterministic bytes and claim order after context

    threading, with no NodeKey constructor/decoder, generated nominal preimage,

    hash or ordinal-as-identity in emitted source. All helpers/modules receive

    the same actual optional capability, with genuine Some on nominal paths; retaining one only on the planning
    side is a failing

    control. Source compilati'
- id: TC-029 / Optional standalone capability controls / changed-unit-3
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: '### Optional standalone capability controls'
- id: TC-029 / Optional standalone capability controls / changed-unit-4
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'Keep the native authenticated preparation above mandatory and unchanged:

    its successful request always supplies Some. Separately exercise standalone

    IR generation with Option context. With None, use reader-admitted

    non-nominal primitive and nested Option/Collection closures and verify their

    exact original successful behavior, descriptors, values/domains and charges.

    Retain the exact conversion FormMismatch("conversion") and quantity

    Unsupported("quantity") IR negatives where they need no nominal key; do

    not substitute a numeric source fixture for a retained conversion node or

    claim the unavail'
- id: TC-029 / Optional standalone capability controls / changed-unit-5
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'Cover None at nominal identity reconstruction from each left/right operand

    source, each primitive-source nominal conversion target and nested

    Option/Collection enum types. After the separate genuine record/tuple lookup

    capability exists, cover its fields and recursive closures too. Assert

    MissingAdmittedContext names the first actually reached nominal declaration,

    with no fabricated key, fallback index, Unsupported substitute, missing

    member or partial publication. Preserve the existing charged traversal,

    first-error/cycle/work precedence, deterministic order and sibling isolation;

    capability '
- id: TC-029 / Optional standalone capability controls / changed-unit-6
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'Pass Some of a genuine wrong-package context to primitive-only, nominal and

    empty standalone requests. After existing setting/accounting checks, each

    must fail the global context guard before classification. Both primitive

    and nominal emitted constructors reject that Some, while a context-free

    recipe can prepare with None. Nominal emitted preparation with None gives

    the typed missing-capability error before lookup or partial publication.

    Equality''s typed Option ABI may still name root replay in its manifest;

    ordinary scalar proof manifests and bodies do not acquire that dependency.'
- id: TC-029 / Optional standalone capability controls / changed-unit-7
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'After complete zero-enum closure resolution only, require a private empty

    index. Reached enums must still include every genuine case, used or not;

    failed lookup, missing context or a stopped walk never proves zero enums.

    For wrappers prepared with None and Some, retain expected package/item/

    descriptor binding and the complete index privately. Exercise wrong binding,

    present-context mismatch, missing context in a nominal recipe, and private

    environment-only faults; assert invariant refusal before any meter charge.

    Then execute both check_type guards, same-index check_equality and evaluate

    on t'
- id: TC-029 / Direct Integer root and asymmetric descriptor control / changed-unit-8
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: '### Direct Integer root and asymmetric descriptor control'
- id: TC-029 / Direct Integer root and asymmetric descriptor control / changed-unit-9
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'Use the genuine source fixture with `a: Int[0,9]` and `b: Integer` and its

    reader-admitted graph, preserving the original package/lock/source bindings

    for any native route. Assert a''s parameter semantic_type names its own range

    and b''s names canonical base Integer, which is also that range''s base. The

    standalone descriptor uses the actual left/right source IDs and an independent

    caller-selected Integer conversion target for the left. Reconstruct direct

    base Integer sources/targets as unbounded and the explicit range as [0,9],

    without adopting the sibling''s bound. Compare generated/native behav'
- id: TC-029 / Direct Integer root and asymmetric descriptor control / changed-unit-10
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'Inspect whether the genuine graph actually retains a convert node. If source

    `convert<Integer>(a)` is erased, this is the parameter/asymmetric caller-target

    control of AC-3/AC-43, not proof of AC-15''s conversion-reference branch.

    Retain the exact admitted-IR conversion-reference assertion independently,

    using the correctly scoped standalone capability without claiming unavailable

    source emission. The existing AC-16 numeric member/base equivalence test and

    record/tuple member semantics stay unchanged. Their unresolved genuine member

    bound association must not be filled by guessing lexical owner'
- id: spec/spec.md / Requirements Architecture / changed-unit-1
  path: spec/spec.md
  role: examined
  excerpt: 'StR-001 is the one stakeholder requirement. Every functional requirement satisfies it. NFR-001 and

    NFR-002 constrain it, NFR-004 constrains FR-008 to FR-013, and NFR-005 constrains the panic-free

    generation of FR-014, FR-015, FR-019, FR-021 and FR-022. NFR-006 constrains FR-015, FR-017,

    FR-031 and FR-041 by gating a change that can alter what real Kani proves on the real-Kani lane.

    `interface-001` defines the serialized input, the library and CLI operations, the artifact bundle,

    the diagnostics and the evidence contract. AD-001 describes the architecture and its seams to

    Contract IR, Contract '
- id: interface-001 / OriginalCompositeEqContext::request
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: "  - name: OriginalCompositeEqContext::request\n    inputs: [original CheckedNodeId, original CheckedOccurrence,\
    \ EqualityOperator (Equal or NotEqual), harness ProofBound list, IR operand-projection work ceiling]\n    output:\
    \ immutable OriginalCompositeEqRequest | CompositeBuildError\n    semantics: Eq or Ne graph-child route through\
    \ IR composite_application_operands; require the requested EqualityOperator to equal the original node's structural.eq\
    \ or structural.ne operation and refuse a difference with the typed OperatorMismatch before invocation (the\
    \ claim carries the node's operator); require ad"
```

## Evidence and limitations

Ticket: IR-17. Frozen PR: quire-contract-codegen#490; source head faad88b53f4d345d2beb4b48b4188445b1d31ad8; base eaaa1c4c7ac58615c0f5e542771a936a596897e1. Six existing Markdown documents only. Independent reviewer; no repository edits, push, native execution, Cargo, formatter, Kani or aggregate gate. No applicable AssuranceProfile/review_selection exists under spec/. Author and research conclusions were checked against the actual diff, unchanged-control byte comparison and actual public owner contracts, rather than treated as authority.

The optional generator/preparation ABI remains PLANNED. Source/compiler fixture receipts are context only, never CG generated-oracle or original TC025/TC027 qualification. QSL-937 record/tuple genuine lookup and member-bound ownership remain unresolved; this review neither supplies that capability nor withdraws existing composite behavior. Native source/lock/package/whole-IR authentication remains mandatory and its entire interface block is byte-identical. AC-8/14/16/26 are byte-identical. Erased numeric source conversions receive no retained-conversion-node credit.

Method selection: code-review plus base, failure-domain, integrity, dependency, evidence, scope-boundary and ears-conformance. These cover the changed public capability ABI, typed failures, traversal/resource ordering, new reference edge, verification controls, ownership and requirement wording. Rust/React lanes and gap-analysis are inapplicable to a spec-only planned amendment; no production code changed. Object review is not selected: no domain-object schema, owner identity vocabulary or independently exchanged semantic payload is introduced; the existing private prepared wrapper's optional capability is covered directly by base/integrity/scope checks. Risk-complexity is not selected because no new algorithm, deployment or volatile service assumption is added. Criterion-strength/Jev is not selected; no calibration result is claimed.

Validation of the six exact changed files passed: 6/6 grammar-clean; zero EARS diagnostics. The engine's duplicate-module and inline-schema notices concern installed module configuration. Diff whitespace check passes. New AC-38..43 remain planned/unqualified tests, not a claim of tagged/executed implementation evidence. No new file-tracking catalog, source digest ledger, dependency pin or copied producer implementation appears. Existing external identity provenance is used only as the product's binding contract.


The actual scoped computed matrix exits 1: the new planned AC-38..43 have no implementation binders, as expected for this specification-first amendment. No trace or executed-evidence completion is claimed and no existing test tag was removed. The initial unsupported matrix --json invocation produced no report; the corrected --format json invocation is the recorded result.
