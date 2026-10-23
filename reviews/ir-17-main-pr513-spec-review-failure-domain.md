---
id: SR-15377
title: IR-17 main landing PR513 spec-review/failure-domain
type: SpecReview
analysis: failure-domain
scope: agent-ix/quire-contract-codegen; IR-17; FR-018 Union amendment and TC-029
review_set: subset
---

## Summary

Ticket IR-17; PR513, frozen 5e60b358eef84d28fa1a1897d1a703633458e8a5, base origin/main 0d7813df955576edc867cfa90e2071bd56d844ff. Scope is the complete 88-file main landing, not PR511's increment. Reviewer-only: no product edits, commits, pushes, Cargo, compiler, test, native execution, Kani/backend, namespace creation, bwrap, unshare, probes or new gate attempt. No applicable AssuranceProfile exists under spec/. Historical review and six-PASS qualification cohorts apply only to their recorded old heads. They are context, not fresh qualification of this candidate. Current method computations and source evidence are retained beside this artifact; tracker text is data.

## Verdict

PASS for scoped Union failure-domain allocation.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

The contract explicitly distinguishes package/context mismatch, Missing/Unknown/Foreign/WrongKind, form/name disagreement and sealing failure. It fixes owner key versus settled-node identity and complete member order; preparation is pure with respect to equality metering and cannot publish a partial wrapper. Existing traversal work/cycle rules govern nested topology and shared closure. Healthy sibling preservation is explicit; the delta invents neither fallback identities nor silent empty membership. The exact normative delta is the retained PR510 Union amendment; those historical artifacts remain lineage, while this pass assesses it in the current landing context. Specification artifacts are not native test evidence.

## Examined scope

```yaml
clean: true
scope:
- id: FR-018
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "- target: ix://agent-ix/quire-spec-language/FR-396\n    type: references"
- id: FR-018
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'When an equality item''s closure reaches an admitted Union, the generator
    and

    emitted environment constructor SHALL look it up by its original wire node

    through `AdmittedContext::composite_declaration`, verify its name and Union

    form, and preserve its declared member and payload-position order throughout

    the nested declaration closure.'
- id: FR-018
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '`CompositeDeclaration` and

    `ValueType::Composite` SHALL use the admitted `key()`; after constructing the

    complete environment, each Union SHALL be sealed through

    `seal_union_verified(key(), node(), bindings)` with clones of the authoritative

    `AdmittedUnionMember::member()` bindings.'
- id: FR-018
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'The distinct node/key roles and

    same-shaped declaration identities are owned by

    [QSL FR-396](ix://agent-ix/quire-spec-language/FR-396), AC-6 through AC-9;

    CG SHALL consume those identities without constructing substitutes.'
- id: FR-018
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'Missing or

    wrong context, Unknown/Foreign/WrongKind lookup, name/form disagreement and

    seal failure SHALL retain their typed causes, publish no successful prepared

    wrapper and admit no equality-meter charge.'
- id: FR-018
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'Per-item failures SHALL leave a

    healthy sibling unchanged; the existing whole-request context-mismatch guard

    and charged resolution precedence remain in force.'
- id: FR-018
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'This consumer amendment is

    PLANNED under AC-44; the existing Quantity refusals in AC-18 and AC-38 remain.'
- id: FR-018-AC-44
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Genuine Union equality generation preserves the facade declaration key,
    settled node, authoritative member bindings and declared member/payload order
    across a nested Union closure, using the specified lookup and verified sealing
    in both generator and emitted preparation.
- id: FR-018-AC-44
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: In-process owner-API controls admit values of every member, including record
    and nested-Union payloads; distinct same-shaped unions retain incompatible identities.
- id: FR-018-AC-44
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Missing/wrong capability, Unknown/Foreign/WrongKind lookup, name/form disagreement
    and seal faults retain their typed causes, no successful prepared wrapper and
    no equality-meter charge; per-item refusals emit no symbols and leave a healthy
    sibling unchanged, while wrong-package context retains the whole-request guard.
- id: FR-018-AC-44
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: TC-029's planned host controls distinguish consumed identities and sealing
    from a mere Union rendering arm; they establish neither emitted-crate execution
    nor Kani acceptance.
- id: FR-018-AC-44
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: PLANNED (IR-17).
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'Planned host verification for FR-018-AC-44 (IR-17): compile authentic
    source

    through root `compile_admitted`, strictly read and normally lower its unchanged

    package, then generate equality with that package''s genuine context.'
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'Include

    two distinct same-shaped unions, a member without payload, record payloads

    and a nested-Union payload.'
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'Independently read the facade declarations and

    compare closure keys, settled nodes, member bindings and member/payload order.'
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'Inspect emitted preparation for original-node lookup, declaration/type
    use of

    `key()` and `seal_union_verified(key(), node(), cloned member() bindings)`;

    a Union enum spelling alone does not discharge the criterion.'
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'Construct the in-process environment through public QSV/kernel APIs from
    those

    same owner declarations and bindings.'
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'Seal every Union after closing the full

    environment and admit values of every member, nested unions included.'
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'Compare

    with independent genuine owner-context values and retain the owning refusal

    for values from the other same-shaped union.'
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'Exercise missing and wrong

    context, Unknown/Foreign/WrongKind lookup, and private name/form or omitted/

    altered-seal faults at their actual boundaries.'
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'Assert typed causes, no

    successful prepared wrapper or equality-meter charge, no symbols for a refused

    item and unchanged healthy siblings for per-item failures.'
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'Wrong-package

    context still fails the whole request before classification; retain existing

    charged traversal and first-error precedence.'
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'These host controls are planned consumer evidence only; they do not qualify

    execution of the emitted crate, native agreement or Kani.'
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'Existing AC-10,

    AC-11 and AC-17 host controls, AC-18/AC-38 Quantity negatives, and all original

    native and proof procedures remain unchanged.'
- id: FR-018-AC-18
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: '`render_value_type` called with `ValueType::Quantity` and with `ValueType::Reference`
    returns `Err(RenderError::UnsupportedValueType { family })` with `family` equal
    to `"quantity"` and `"reference"` respectively and does not panic, and the item-boundary
    mapping turns `RenderError::UnsupportedValueType` into `CompositeEqualityRefusal::Unsupported`
    with `node_tag` equal to `family` and `unsupported_node_id` equal to the item''s
    expression node id, emitting no code for the item and leaving its siblings unchanged,
    while'
- id: FR-018-AC-18
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: '`RenderError::Generation(OracleGenerationError::UnknownRuntimeVariant
    { .. })` still fails the whole call. No request reaches these arms through `generate_composite_equality_oracles`:
    a quantity operand is refused earlier as `CompositeEqualityRefusal::Unsupported
    { node_tag: "quantity" }` and a reference operand by Contract IR (FR-018-AC-7),
    each a per-item refusal that leaves its siblings unchanged, so the criterion is
    asserted by a unit test in the module''s own `#[cfg(test)]` tests.'
- id: FR-018-AC-24
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: The exact QSpec `positive-recursive-records.json` retains strict-reader
    and List/Tree shape assertions; independently authored genuine admitted source
    with those List (`Option<List>`) and Tree (`Sequence<Tree>` bounded `[0,3]`) shapes
    closes each back-edge at its own in-progress record key, generates equality items
    over both record types, and terminates; a synthetic unclosed `Option` self-cycle
    and a `Sequence` self-cycle each refuse as `TypeResolutionCycle` naming the repeated
    type node, without a panic, stack overflow or generated symbol. PLANNED (IR-643).
- id: FR-018-AC-25
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: 'The per-item type-resolution counter charges one unit for each root type-node
    entry or followed type-node reference or bounded-domain `semantic_type` edge,
    including a repeated or memoized reference, across both operands and conversion
    targets, independently of Contract IR lowering; with the default limit, an acyclic
    chain reaches its 65,537th entry and refuses only its item as `TypeResolutionWorkExhausted
    { limit: 65_536, consumed: 65_537 }` before further descent and without call-stack
    exhaustion or generated symbols, while a sibling needing at most 65,536 entries
    still'
- id: FR-018-AC-25
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: generates. PLANNED (IR-643).
- id: FR-018-AC-29
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: A cycle detected after a charged repeated entry reports `TypeResolutionCycle`
    without following a child; if that repeated entry is the first denied work entry,
    `TypeResolutionWorkExhausted` wins and reports the effective limit and first denied
    count. An unaffected sibling keeps its original disposition in either case. PLANNED
    (IR-511).
- id: FR-018-AC-35
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: Unknown declaration, unknown case and a directly or transitively imported
    declaration produce the original typed Unknown/Foreign details at consumed and
    generated lookup boundaries. Malformed source/dependency input, QSL stage or above-reader
    resource limits, and internal Fault remain Package, Resource and Fault at the
    source-retaining boundary; a subsequent IR read stop remains the separate PackageReadLimit,
    with no partial context or successful prepared wrapper. The current ImportedContextUnsupported
    guard remains until original admitted dependency packages are retained.
- id: FR-018-AC-36
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: Emitted source uses only WireNodeId and case identifiers for lookup and
    contains no NodeKey constructor/decoder or generated nominal preimage/hash. Only
    the prepared wrapper's private genuine index reaches check_equality. A private
    environment-only fault fixture still satisfies AC-8, every original charge-stop
    case still satisfies AC-9, and a wrong item/descriptor prepared wrapper refuses
    before charging. Record/tuple/recursive/collection cases remain required by AC-2
    and AC-21 through AC-30; an absent owner lookup capability is recorded as unresolved
    migration work, not
- id: FR-018-AC-36
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: as accepted Unsupported behavior.
- id: FR-018-AC-38
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: With `None`, standalone generation and emitted preparation accept successfully
    reconstructed non-nominal primitive and nested Option/Collection closures; their
    index is empty only after complete zero-enum resolution. Existing admitted-IR
    conversion-form and quantity-family negatives retain respectively exact FormMismatch("conversion")
    and Unsupported("quantity"), with no replacement ContextPackageMismatch or MissingAdmittedContext,
    no symbols for the refused item and unchanged independent siblings. This does
    not claim matching QSL source emission or source authentication for those IR fixtures.
- id: FR-018-AC-39
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: '`None` at the first reached nominal identity returns typed MissingAdmittedContext
    naming that declaration. Cover left and right source operands, left and right
    conversion targets from primitive sources, nested Option and Collection enum types,
    and eventual record/tuple field and recursive closures through their genuine owner
    capability. No key, default/partial index, omitted nominal declaration or Unsupported
    substitute is produced. Retain the existing charged-entry/cycle/error precedence
    and unaffected sibling results; the separate missing record/tuple owner lookup
    remains outstanding.'
- id: FR-018-AC-42
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: Generated oracle invocations under Some or valid context-free None preserve
    private package/item/descriptor/index binding, both check_type guards, same-index
    check_equality and evaluate. Private wrong-binding, present-context mismatch,
    missing nominal context and environment-only faults refuse without charge. The
    native OriginalCompositeEqContext route retains every mandatory authentic source/lock/package/whole-IR
    comparison and cannot use None to bypass them. No scalar proof manifest/body acquires
    replay compilation.
- id: FR-396-AC-3
  path: agent-ix/quire-spec-language/spec/functional/FR-396-expose-the-admitted-record-and-tuple-keys-through-the-replay-facade.md
  role: context_only
  excerpt: '`composite_declaration` refuses `Unknown` for an id no package admitted,
    `WrongKind` for the unit''s enum declaration and function, and `Foreign` naming
    the library''s `package_id` for a library record or union imported directly or
    through an intermediate library, while every own record, tuple and union is found.'
- id: FR-396-AC-6
  path: agent-ix/quire-spec-language/spec/functional/FR-396-expose-the-admitted-record-and-tuple-keys-through-the-replay-facade.md
  role: context_only
  excerpt: 'Over a unit with a union with a record payload and a union with a union
    payload, each union is found by its `composite_type` node id with form `Union`,
    and its members equal, in declared order, the producer''s own: the `VariantId`
    of the FR-441 member key over the union''s settled node, the sealed `TypeEnvironment`
    binding of the member, and literal payload types (a record or union position is
    `ValueType::Composite` of that declaration''s admitted key).'
- id: FR-396-AC-7
  path: agent-ix/quire-spec-language/spec/functional/FR-396-expose-the-admitted-record-and-tuple-keys-through-the-replay-facade.md
  role: context_only
  excerpt: A consumer naming only the `qsl_replay` root, the kernel (`quire-exact`)
    and the QSV declaration API declares a union's `CompositeDeclaration` from its
    admitted key and payloads, seals it with `seal_union_verified` over the admitted
    bindings (`AdmittedUnionMember::member`), and constructs and admits a value of
    every member, a member holding another union included, without decoding, hashing
    or constructing a key.
- id: FR-396-AC-8
  path: agent-ix/quire-spec-language/spec/functional/FR-396-expose-the-admitted-record-and-tuple-keys-through-the-replay-facade.md
  role: context_only
  excerpt: Two same-shaped unions have distinct kernel keys, no `VariantId` is shared
    by two members of the unit's unions, and a repeated lookup and a recompile return
    the same declarations and members.
- id: FR-396-AC-9
  path: agent-ix/quire-spec-language/spec/functional/FR-396-expose-the-admitted-record-and-tuple-keys-through-the-replay-facade.md
  role: context_only
  excerpt: A replay witness whose union value names the union's `composite_type` node
    id and an admitted member replays on both arms, and the same witness naming the
    union's kernel key refuses.
findings: []
bindings: []
```
