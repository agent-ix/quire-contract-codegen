---
id: SR-14685
title: IR-17 admitted equality ABI failure-domain review
type: SpecReview
analysis: failure-domain
scope: agent-ix/quire-contract-codegen; IR-17 PR488; spec/assurance/AD-001-codegen-architecture.md,
  spec/assurance/AD-004-cg-crate-layout.md, spec/core/functional/interface-001-codegen-api.md,
  spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/functional/FR-018-composite-equality-oracles.md,
  spec/oracle/matrix/TC-029-composite-equality-oracles.md, spec/spec.md
review_set: subset
---

## Summary

Identity: original checked package identity, exact complete package equality and source/library-lock authentication precede context exposure; emitted constructor rejects wrong context before lookup; oracle rejects wrong item/descriptor before charge. Complete genuine enum index includes unused cases and never derives identity from ordinals/names. Unknown declaration vs case and direct/transitive Foreign retain fields; Package/Resource/Fault and later IR PackageReadLimit remain separate. Purity: source compilation stays before metered oracle evaluation, wrapper exposes no raw index constructor. Topology: existing record/tuple recursion, collections, work and source bounds remain required; owner gap is explicit.

## Scope

```yaml
scope:
- id: AD-001-L57
  path: spec/assurance/AD-001-codegen-architecture.md
  role: examined
  excerpt: "- Contract IR reads and lowers the package through the `CheckedPackageV2` reader and lowering.\n- `quire-exact` owns kernel values, scalar types, metering and outcomes. QSV\n  (`quire-semantic-value`) owns declarations, equality and enum semantics.\n  Contract Runtime retains its own residual functions and execution surface;\n  it is not a re-export facade for the extracted owners.\n- QSL owns replay/proof types and the executor behind root `qsl-replay`, and\n  FR-394's package-bound `AdmittedContext` with genuine checker-issued enum\n  declaration/case keys. No QSL implementation-layer import is per"
  lines: 57-65
- id: AD-001-L118
  path: spec/assurance/AD-001-codegen-architecture.md
  role: examined
  excerpt: '### Direct owning execution surface view'
  lines: '118'
- id: AD-001-L120
  path: spec/assurance/AD-001-codegen-architecture.md
  role: examined
  excerpt: 'The direct-owner/enum ABI migration is planned under

    [FR-018](../oracle/functional/FR-018-composite-equality-oracles.md) and

    [interface-001](../core/functional/interface-001-codegen-api.md). Every generated

    oracle calls the owner of its operation and interprets no expression itself:'
  lines: 120-123
- id: AD-001-L125
  path: spec/assurance/AD-001-codegen-architecture.md
  role: examined
  excerpt: "- FR-014 scalar oracles call the kernel scalar operations over `rt::Integer` and the other exact\n  scalar types.\n- FR-018 equality oracles call QSV `check_type`, `check_equality` with their\n  complete genuine enum index, and `CheckedEquality::evaluate`. A generated\n  opaque prepared environment retains the immutable replay-root context,\n  environment, index, descriptors and original-package/item binding.\n- FR-021 function-application oracles admit a `PackageDeclarations` through\n  `PackageDeclarations::check` and apply one function through `CheckedPackage::call`."
  lines: 125-132
- id: AD-001-L134
  path: spec/assurance/AD-001-codegen-architecture.md
  role: examined
  excerpt: 'Only the source-retaining preparation boundary calls root `compile_admitted`,

    authenticates original source/lock, compares the complete recompiled package

    and retains its context. CG''s oracle generator takes that root facade type,

    not a peer `replay` module type. Emitted code resolves `WireNodeId`/case

    identifiers by genuine owner lookup and constructs no kernel key. The

    facade''s enum-only inventory leaves record/tuple owner-key lookup as separate

    unresolved migration work; existing composite behavior remains required.

    Scalar Boolean/integer proof execution has no added source-compiler leg.

    Th'
  lines: 134-143
- id: AD-004-L347
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: 'The planned FR-018 admitted-context migration preserves this internal graph:

    `oracle` accepts the external root `qsl_replay::AdmittedContext`, not CG''s peer

    `replay::OriginalCompositeEqContext`. The existing source-retaining builder

    in `replay` authenticates the original source/lock, calls root

    `compile_admitted` under the admitted request limits, compares the complete

    original checked package, and retains the genuine compilation/context pair.

    A successful request exposes its immutable context to the caller, which

    passes it to generation. This adds no `oracle --> replay` or QSL

    implementation-'
  lines: 347-355
- id: AD-004-L357
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: 'Generated equality''s prepared environment privately holds the facade context,

    QSV environment and complete enum index plus package/item binding. Exact

    kernel types/meters/outcomes come from `quire-exact`; QSV owns declaration,

    equality and enum APIs; Runtime imports name only its remaining functions.

    `core/profile` remains the sole emitted-manifest authority. Equality''s

    manifest/scratch consumers add root `qsl-replay` when they name its context;

    ordinary scalar Boolean/integer proof manifests/body acquire no such leg.

    No emitted key constructor or backend decoder is permitted. The enum-only

    fa'
  lines: 357-368
- id: interface_001-L10
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: "```yaml\nname: ContractCodegen\nversion: draft-codegen-v1\ninput:\n  contract_package: public IR BoundPackage from the public IR derived executable projection decoder; no private wire or codegen input schema\n  configuration: backend selection, customer bindings, output profile\noperations:\n  - name: source_obligation_census\n    status: planned; complete producer/consumer implementation gated on IR-759 reviewed neutral correspondence\n    inputs: [SourceCensusRequest borrowing the selected admitted checked package, complete owner semantic-obligation inventory, criterion and production-symbol inventor"
  lines: 10-410
- id: FR-014-L21
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'When a caller supplies an admitted `quire.checked-package/v2` package and a set

    of scalar expression nodes, the code generator shall emit a deterministic Rust

    oracle crate whose functions evaluate each node by calling the

    direct `quire-exact` kernel operators and metering, QSV semantic operators,

    and only the remaining Contract Runtime functions it still owns. It never approximates a

    scalar family through another family and never copies a resource charge.'
  lines: 21-26
- id: FR-014-L34
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'FR-014 owns the integer and Boolean `eq` and `ne` nodes. Their oracles call the

    QSV''s one equality relation, `TypeEnvironment::check_equality` followed by

    `CheckedEquality::evaluate`, as FR-018''s oracles do, and return its

    `Outcome<bool>` unchanged; no oracle compares two values itself. A Boolean

    connective''s oracle calls `exact::evaluate_boolean` with the matching

    `BooleanConnective` when both operands are decided values, and

    `exact::evaluate_boolean_short_circuit` when its right operand is itself an

    oracle that may stop, and returns the runtime''s `Outcome<bool>` unchanged.'
  lines: 34-41
- id: FR-014-L50
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'The direct-owner migration is PLANNED until implementation and focused

    qualification. Kernel values, scalar types, outcomes and metering come from

    `quire-exact`; declarations/equality and enum semantics come from QSV

    (`quire-semantic-value`); functions still owned by Contract Runtime remain

    at that owner. Historical uses of the runtime''s `exact` facade below do not

    authorize re-exporting removed owner APIs or retaining a compatibility shim.

    Boolean/integer equality passes QSV an empty `EnumMemberIndex`, which is valid

    for those domains. These scalar oracle bodies and their proof manifests need'
  lines: 50-61
- id: FR-014-L65
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: "- An admitted `CheckedPackageV2` read through Contract IR's strict reader.\n- A request of items, each naming one checked node id and one typed exact\n  scalar operation descriptor.\n- Direct exact/QSV dependencies and the Runtime residue dependency, named by\n  the one `core/profile` emitted-manifest authority and this repository's\n  `Cargo.toml`; no dependency on a QSL implementation layer."
  lines: 65-70
- id: FR-014-L248
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: "- A generated crate: `Cargo.toml` (`publish = false`, direct exact/QSV and\n  Runtime residue dependencies from the one profile authority) and `src/lib.rs` with one oracle function\n  per supported item.\n- A coverage-probe source map for the generated crate.\n- `derive_exact_scalar_items`: one `Result<ExactScalarItem, ExactScalarRefusal>`\n  per requested node id, in input order. A refusal is a lowering or bound\n  refusal `generate_exact_scalar_oracles` also gives, or `NoDerivableClaim`\n  carrying a `ClaimDerivationRefusal`, a tagged `code` enum:\n  `not_application`, `missing_operation_identity`, "
  lines: 248-265
- id: FR-014-AC-8
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: The generated crate declares `publish = false`, names the direct exact/QSV and Runtime residue dependencies from the one profile authority without a replay-context dependency for Boolean/integer scalar proofs, contains no charge amount (every charge comes from runtime metering), and compiles.
  lines: '496'
- id: FR-018-L1
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "---\nid: FR-018\ntitle: \"Generate exact complete-V1 composite equality oracles\"\ntype: FR\nrelationships:\n  - target: ix://agent-ix/quire-contract-codegen/StR-001\n    type: satisfies\n  - target: ix://agent-ix/quire-contract-codegen/FR-014\n    type: depends_on\n  - target: ix://agent-ix/quire-contract-runtime/FR-008\n    type: references\n  - target: ix://agent-ix/quire-contract-ir/FR-036\n    type: references\n  - target: ix://agent-ix/quire-contract-ir/FR-038\n    type: references\n  - target: ix://agent-ix/quire-specification/FR-460\n    type: references\n  - target: ix://agent-ix/quire-spec-language/ADR"
  lines: 1-27
- id: FR-018-L31
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'When a caller supplies an admitted `ContractPackageV1`, its package-bound

    admitted equality context, and a set

    of equality expression nodes, the code generator shall emit a deterministic Rust

    oracle crate whose functions decide each node by reconstructing the item''s

    `TypeEnvironment` and calling the QSV declaration/equality owner''s

    `TypeEnvironment::check_equality` and `CheckedEquality::evaluate` with a

    caller-supplied `Meter`. It never compares two values structurally, and never

    copies a resource charge or a planned pair count.'
  lines: 31-38
- id: FR-018-L67
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'That second stage must include `TypeEnvironment::check_type` on both operand

    comparison types, before `check_equality`. `check_equality` consults the

    environment only through `contains_ieee`, which treats a composite key absent

    from the environment as bearing no IEEE value and returns `false`; and

    `CheckedEquality::evaluate` takes no environment at all, comparing the caller''s

    two values directly. Without `check_type` the emitted function''s

    prepared environment therefore decides almost nothing, and an environment

    that declares none of the item''s composites still completes. `check_type` is the

    r'
  lines: 67-78
- id: FR-018-L80
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'The admitted-context ABI and direct-owner migration below are PLANNED until

    CG implementation, native generated-code controls and independent review

    qualify them. Upstream facade availability is not a CG acceptance result.

    The existing record, tuple, recursive closure and collection behavior remains

    required; the enum-only facade does not justify withdrawing those routes.'
  lines: 80-84
- id: FR-018-L88
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "- An admitted `ContractPackageV1` from Contract IR's FR-330 lowerer and strict\n  reader, with the caller's complete root-position request and the positions\n  of the equality items in it. Its retained checked-V2 graph supplies the\n  equality expressions and declaration closure below.\n- The immutable genuine `qsl_replay::AdmittedContext` returned alongside the\n  original compiled package by root `qsl_replay::compile_admitted`. Its\n  `package_id()` must equal the original checked-package identity in\n  `ContractPackageV1::wire().package.source.package_id`, not the lowered\n  ContractPackage's own `"
  lines: 88-123
- id: FR-018-L152
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'A declaration or case key SHALL be the original checker-issued key selected

    through its owning admitted lookup. Equality with the corresponding V2 wire

    identity is retained, including its domain; it is not permission for CG to

    hash a preimage, decode a digest into `NodeKey`, or emit a key constructor.

    A generated identifier literal is a facade `WireNodeId` used for lookup.'
  lines: 152-156
- id: FR-018-L158
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'For enums, the generator SHALL call `AdmittedContext::enum_declaration` and

    `enum_case`, verify the declaration''s qualified name and ordering against the

    selected IR node, and use all `AdmittedEnum::cases()`, including unused cases.

    QSV `EnumDeclaration::new` receives that genuine declaration key, ordered flag

    and complete case names. Each `EnumDeclaration::member` receives the exact

    case key returned by the facade. Every resulting `EnumValue` is recorded in

    one `EnumMemberIndex`; its `variant()` and `position()` supply exact enum shape

    variants and canonical positions. A position, spelling, a'
  lines: 158-167
- id: FR-018-L169
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'The published facade currently supplies enum lookups only. A record or tuple

    `CompositeDeclaration` still requires its genuine owner-issued `NodeKey`.

    Complete migration of those routes requires an owning composite/general-key

    lookup or prepared composite capability; passing a record id to

    `enum_declaration` is not that capability. This residual owner gap authorizes

    neither backend key construction nor a new `Unsupported` regression for an

    existing record/tuple route. The existing closure obligations remain pending

    qualification through the actual owning capability.'
  lines: 169-176
- id: FR-018-L180
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "- A generated crate: `Cargo.toml` (`publish = false`, direct exact/QSV\n  dependencies, and root `qsl-replay` for admitted-context lookup) and\n  `src/lib.rs`. The single `core/profile` manifest authority supplies normal\n  Git branch-main dependencies; consumer lockfiles perform ordinary resolution.\n- One public opaque `PreparedEqualityEnvironment<'ctx>` type whose private\n  fields retain the borrowed `&'ctx qsl_replay::AdmittedContext`, expected\n  original package and item/descriptor binding, admitted QSV `TypeEnvironment`,\n  complete genuine `EnumMemberIndex`, and reconstructed operand descrip"
  lines: 180-189
- id: FR-018-L191
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "  ```rust\n  pub fn environment_symbol<'ctx>(\n      context: &'ctx qsl_replay::AdmittedContext,\n  ) -> Result<PreparedEqualityEnvironment<'ctx>, EnvironmentError>;"
  lines: 191-194
- id: FR-018-L196
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "  pub fn oracle_symbol(\n      prepared: &PreparedEqualityEnvironment<'_>,\n      left: &quire_exact::Value,\n      right: &quire_exact::Value,\n      meter: &mut quire_exact::Meter,\n  ) -> quire_exact::Outcome<bool>;\n  ```"
  lines: 196-202
- id: FR-018-L204
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "  `symbol` denotes the existing deterministic per-item generated symbol.\n  Constructors and descriptor/module helpers thread this same context;\n  neither a zero-argument helper nor a planning-only context may reconstruct\n  nominal keys. The constructor compares the expected original package\n  identity before any key lookup and publishes no partial prepared value.\n- Generated `EnvironmentError` SHALL distinguish `ContextPackageMismatch`,\n  `Lookup(qsl_replay::AdmittedLookupRefusal)`, `Declaration` retaining the\n  QSV `EnvironmentFailure`, and `Reconstruction(ReconstructionError)`.\n  QSV admissi"
  lines: 204-224
- id: FR-018-L228
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '### Admitted-context ABI'
  lines: '228'
- id: FR-018-L230
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'The public generation signature SHALL be:'
  lines: '230'
- id: FR-018-L232
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "```rust\npub fn generate_composite_equality_oracles(\n    package: &ContractPackageV1,\n    admitted: &qsl_replay::AdmittedContext,\n    requested: &[ContractRequestPosition],\n    positions: &[usize],\n    items: &[CompositeEqualityItem],\n    limits: CompositeEqualityLimits,\n) -> Result<CompositeEqualityOracles, OracleGenerationError>;\n```"
  lines: 232-241
- id: FR-018-L243
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'After the existing whole-request accounting/position checks, a mismatched

    context package SHALL refuse the whole generation with a typed context

    mismatch and no artifact. Unknown declaration, unknown case and foreign

    package lookup failures SHALL remain distinct typed per-item refusals,

    retaining the declaration, optional case and foreign package identity as the

    owning cause supplies them. Claim-map serialization may project these fields

    into CG''s typed refusal representation but never infer the cause from text.'
  lines: 243-249
- id: FR-018-L251
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'The generated constructor SHALL create its private enum index only from the

    same genuine context. The oracle SHALL reject a prepared package or

    item/descriptor binding other than its own as `Refused(CheckedInvariant)`

    before a meter charge. It SHALL check both expected operand comparison types

    against the prepared environment and call QSV `check_equality` with the

    prepared index before every evaluation. A private negative-test replacement

    of that environment does not replace its genuine context/index or create a

    public unchecked constructor. These tests retain AC-8''s empty, missing-key

    and wro'
  lines: 251-261
- id: FR-018-L263
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '### Existing equality behavior'
  lines: '263'
- id: FR-018-L265
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "- When generation begins, the generator shall consume each requested node's\n  existing complete-lowering accounting in the admitted ContractPackage,\n  with its full request-position mapping, and select exactly one generated\n  or refused disposition per bound item; it shall not lower a second package.\n- When a lowered node is a `binary` expression whose operand `semantic_type`s\n  reconstruct to a declaration closure that `TypeEnvironment::new` admits, and\n  whose descriptor `TypeEnvironment::check_equality` admits, the generator shall\n  emit that item's environment constructor and oracle functi"
  lines: 265-477
- id: FR-018-AC-12
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The generated crate declares `publish = false`, names direct exact/QSV and root replay dependencies through the one manifest authority, contains no charge amount and no planned pair count (every charge and every pair comes from runtime metering), and compiles.
  lines: '504'
- id: FR-018-AC-13
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Every claim-map entry carries the node id, IR id, package id, source map, claims, reconstructed declaration keys and selected schedule of its item, and its genuine owner-issued declaration keys equal the exact domain and digest of the V2 node ids its operand types reach, selected through owning lookup without CG/emitted key construction.
  lines: '505'
- id: FR-018-AC-17
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The `src/lib.rs` that `generate_composite_equality_oracles` returns for the TC-029 step 1 package (every generated item) contains zero occurrences of the panic tokens FR-018-AC-19 lists, no `.ok()`, `.unwrap_or(` or `.unwrap_or_default(` anywhere, and no `[` index or slice expression directly after an identifier character, `)` or `]`; every reconstruction helper in it returns `Result<_, ReconstructionError>`, and each of `rebuild_integer`, `rebuild_interval`, `rebuild_rational`, `rebuild_decimal`, `rebuild_text` and `rebuild_cardinality` turns a failed constructor or parse into exactly the mat
  lines: '509'
- id: FR-018-AC-33
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A genuine root-facade compilation of authentic retained source, original dependencies and admitted request limits has equal compiled/context/original package identities and exact full recompiled/original CheckedPackageV2 equality before its context is exposed. A different context package refuses generation and emitted environment construction before lookup, publishes no prepared value, and cannot evaluate with a completed Boolean or meter charge.
  lines: '525'
- id: FR-018-AC-34
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The actual generated ABI executes ordered and unordered enum comparisons over all declared cases, including a case absent from every selected literal and an enum parameter with no member literals. Exact shape variants and canonical positions come from genuine facade/QSV values. Same-spelled cases from distinct declarations retain distinct identities; matching and mismatching declaration comparisons preserve the owning equality disposition.
  lines: '526'
- id: FR-018-AC-35
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Unknown declaration, unknown case and a directly or transitively imported declaration produce the original typed Unknown/Foreign details at consumed and generated lookup boundaries. Malformed source/dependency input, QSL stage or above-reader resource limits, and internal Fault remain Package, Resource and Fault at the source-retaining boundary; a subsequent IR read stop remains the separate PackageReadLimit, with no partial context or successful prepared wrapper. The current ImportedContextUnsupported guard remains until original admitted dependency packages are retained.
  lines: '527'
- id: FR-018-AC-36
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Emitted source uses only WireNodeId and case identifiers for lookup and contains no NodeKey constructor/decoder or generated nominal preimage/hash. Only the prepared wrapper's private genuine index reaches check_equality. A private environment-only fault fixture still satisfies AC-8, every original charge-stop case still satisfies AC-9, and a wrong item/descriptor prepared wrapper refuses before charging. Record/tuple/recursive/collection cases remain required by AC-2 and AC-21 through AC-30; an absent owner lookup capability is recorded as unresolved migration work, not as accepted Unsupporte
  lines: '528'
- id: FR-018-AC-37
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Equality's generated manifest and native scratch consumers use the one profile authority's direct exact/QSV/replay dependencies, while ordinary Boolean/integer FR-014 scalar proof manifests and oracle bodies acquire no replay source-compilation/context leg. Native generated enum acceptance is reported separately from original TC025/TC027 proof qualification; neither result substitutes for the other.
  lines: '529'
- id: FR-018-L577
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '| FR-018-AC-33 | Compare the context against lowered ContractPackage identity instead of original source package identity, skip full-package/source-lock authentication, or expose the retained context before those checks. |'
  lines: '577'
- id: FR-018-L578
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '| FR-018-AC-34 | Build shape/index from only used literals, substitute positions for member keys, or conflate identical case names under distinct declarations. |'
  lines: '578'
- id: FR-018-L579
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '| FR-018-AC-35 | Collapse Unknown case and declaration, drop Foreign package details, map Resource/Fault to Package, or publish a partial context after stopped admission. |'
  lines: '579'
- id: FR-018-L580
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '| FR-018-AC-36 | Emit a NodeKey literal, accept an arbitrary caller index, skip check_type on the prepared environment, or evaluate a wrapper for another descriptor. |'
  lines: '580'
- id: FR-018-L581
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '| FR-018-AC-37 | Add replay compilation to scalar proof execution, let scratch and emitted manifest sources drift, or count native enum success as an original Kani receipt. |'
  lines: '581'
- id: FR-018-L585
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "- **Upstream**: [FR-014](./FR-014-exact-scalar-oracles.md), Contract IR FR-036/FR-038\n  (CheckedPackage V2 lowering), Contract Runtime FR-008 (composite, collection\n  and equality families before their owner extraction), QSL\n  [FR-394](ix://agent-ix/quire-spec-language/FR-394) and\n  [FR-087](ix://agent-ix/quire-spec-language/FR-087) (genuine admitted context\n  and key lookup), and QSV [FR-060](ix://agent-ix/quire-semantic-value/FR-060)\n  (no identity minting). Direct owning imports follow the migration above.\n- **Downstream**: [TC-029](../matrix/TC-029-composite-equality-oracles.md);\n  [FR-015"
  lines: 585-596
- id: TC-029-L28
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'The FR-018 admitted-context and direct-owner procedure amendments are

    PLANNED/UNRUN until CG implements them and qualifies the generated ABI.

    The existing full record/tuple/recursive/collection corpus remains required;

    the enum-only owner facade does not excuse dropping it.'
  lines: 28-31
- id: TC-029-L35
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: "1. Build an admitted V2 package holding `binary` expression nodes over: record,\n   tuple and option types; `sequence`, `set`, `bag` and `ordered_set` collection\n   types with their `collection_bounds` domains; nested composites inside\n   collections and collections inside records; scalar leaves of every equality\n   schedule (text, enum, quantity, and the `Plan` types); admitted `convert<T>`\n   operands from each `admits_equality_conversion` row exercised; plus operands\n   with distinct text profiles, distinct enum declarations, incompatible\n   dimensions, distinct units, no common type, a `con"
  lines: 35-114
- id: TC-029-L269
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: '### Package-bound enum migration controls'
  lines: '269'
- id: TC-029-L271
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'Use authentic retained QSL source, original dependency provision and original

    request stage/replay limits. Call root `compile_admitted`, check the emitted

    and context package identities against the original checked-package identity,

    strictly re-read emitted bytes and compare the complete package with the

    original. Preserve original source-byte/owner and library-lock authentication;

    only a successfully authenticated request exposes `admitted_context()`.

    Exercise malformed source and dependency input (Package), QSL stage and

    above-reader resource refusal (Resource), a subsequent IR read stop

    (Pa'
  lines: 271-281
- id: TC-029-L283
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'Run the real generated `environment_symbol(context)` and

    `oracle_symbol(prepared, left, right, meter)` ABI, with ordered and unordered

    enums including unused cases and a parameter without member literal nodes.

    Check canonical positions and original member identities against genuine

    facade/QSV values. Use same-spelled cases under distinct declarations to

    expose conflated identity. Reject a different package context before lookup;

    reject a different item/descriptor prepared wrapper before charging. Exercise

    Unknown declaration, Unknown case and Foreign direct/transitive dependency

    identities, pr'
  lines: 283-293
- id: TC-029-L295
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'Retain the empty/missing/wrong-key environment controls of step 6 and every

    charge-stop vector of step 5 with the same genuine index. Mutate generated

    operator, descriptor and case lookup independently so direct-vs-generated

    agreement fails. Check deterministic bytes and claim order after context

    threading, with no NodeKey constructor/decoder, generated nominal preimage,

    hash or ordinal-as-identity in emitted source. All helpers/modules receive

    the actual context; retaining one only on the planning side is a failing

    control. Source compilation occurs before oracle invocation, and the

    emitted o'
  lines: 295-303
- id: TC-029-L305
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: 'The enum-only lookup cannot issue record/tuple keys. Record the missing

    owner lookup/preparation capability separately until it exists; preserve all

    original closure/cycle/bound/collection corpus requirements rather than

    accepting an Unsupported regression or synthetic key. Manifest/scratch checks

    cover equality''s root replay dependency and scalar Boolean/integer proof

    bodies with a legitimate empty index and no added replay compile leg.

    Native enum results are not TC025/TC027 healthy/mutant proof receipts, and

    those original proof domains and controls remain unchanged.'
  lines: 305-312
- id: spec-master-L50
  path: spec/spec.md
  role: examined
  excerpt: 'StR-001 is the one stakeholder requirement. Every functional requirement satisfies it. NFR-001 and

    NFR-002 constrain it, NFR-004 constrains FR-008 to FR-013, and NFR-005 constrains the panic-free

    generation of FR-014, FR-015, FR-019, FR-021 and FR-022. NFR-006 constrains FR-015, FR-017,

    FR-031 and FR-041 by gating a change that can alter what real Kani proves on the real-Kani lane.

    `interface-001` defines the serialized input, the library and CLI operations, the artifact bundle,

    the diagnostics and the evidence contract. AD-001 describes the architecture and its seams to

    Contract IR, Contract '
  lines: 50-72
- id: FR-018-AC-2
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: 'On every vector of the composite-equality corpus, the generated oracle''s `Outcome<bool>`, its admitted charge sequence and its consumed counters are equal to those of `quire_contract_runtime::exact::TypeEnvironment::check_equality` plus `CheckedEquality::evaluate` invoked directly on an independently constructed environment, operands and fresh `Meter`. This criterion claims no agreement with the QSL value authority: that boundary is owned by quire-integration IR-842 (FR-014-AC-47).'
  lines: '494'
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: A declaration closure `TypeEnvironment::new` refuses is refused at generation time carrying that `DeclarationCause`, including both recursion passes and a duplicate record field; and each generated oracle calls `TypeEnvironment::check_type` on both operand comparison types before `check_equality`, so calling it with an empty environment, one omitting a key its operand types reach, or one declaring that closure under a different key, yields `Outcome::Refused(Refusal::CheckedInvariant)` with no charge admitted, rather than a panic or a completed Boolean.
  lines: '500'
- id: FR-018-AC-9
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: "Every generated oracle function returns `Outcome<bool>`, never `bool`: with a denial injected at each of `equality.plan-form`, `equality.plan`, `equality.pair`, `equality.result-retain` and each conversion charge point in turn, the oracle yields `Outcome::Incomplete` naming that point, the denied charge is not applied \u2014 every counter equals those of the same run stopped immediately before that point \u2014 and never a completed Boolean."
  lines: '501'
- id: FR-018-AC-21
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: Each generated item's claim-map entry records its declaration closure as every declaration's key, form (record, tuple or option), members in declaration order with each member's name, presence and type, and each leaf's family and declared bound, and the closure equals member by member (name, presence, value type, order) the declarations of the `TypeEnvironment` the item's generated environment constructor returns, for every item of the TC-029 corpus whose leaves are Boolean or bounded integers. PLANNED (IR-264).
  lines: '513'
- id: FR-018-AC-22
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: 'For every pair of operand values in the item''s refinement domain (FR-028-AC-16: every case where the domain fits the case cap, the boundary and seeded cases otherwise; every presence state of an option and of an optional field included) of an item whose leaves are Boolean or bounded integers, the generated oracle called with its own environment and a `Meter` whose every limit is `u64::MAX` returns `Outcome::Completed` with the verdict of QSpec FR-149 for the item''s operator and admits exactly `equality.plan-form`, `equality.plan`, one `equality.pair` per node of the occurrence-pair tree, then '
  lines: '514'
- id: FR-018-AC-23
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: For every item of the TC-029 corpus whose leaves are Boolean or bounded integers, the claim-map closure's member names, presence and each integer leaf's inclusive bounds equal those of an independent read of the checked package (the V2 composite and `bounded_domain` nodes read through Contract IR's reader, never through the generator's reconstruction); a closure that dropped a member, narrowed a bound by one or read an optional member as required fails the check. PLANNED (IR-264).
  lines: '515'
- id: FR-018-AC-24
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: Type reconstruction of QSpec's `positive-recursive-records.json` List (through `Option<List>`) and Tree (through `Sequence<Tree>` bounded `[0,3]`) closes each back-edge at the in-progress record key, generates equality items built over both record types, and terminates; a synthetic unclosed `Option` self-cycle and a `Sequence` self-cycle each refuse as `TypeResolutionCycle` naming the repeated type node, without a panic, stack overflow or generated symbol. PLANNED (IR-643).
  lines: '516'
- id: FR-018-AC-25
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: 'The per-item type-resolution counter charges one unit for each root type-node entry or followed type-node reference or bounded-domain `semantic_type` edge, including a repeated or memoized reference, across both operands and conversion targets, independently of Contract IR lowering; with the default limit, an acyclic chain reaches its 65,537th entry and refuses only its item as `TypeResolutionWorkExhausted { limit: 65_536, consumed: 65_537 }` before further descent and without call-stack exhaustion or generated symbols, while a sibling needing at most 65,536 entries still generates. PLANNED (I'
  lines: '517'
- id: FR-018-AC-26
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: Reading the QSpec List `next` field as `binding(next, aggregate([binding(optional, reference Option<List>)]))` yields exactly one `FieldDeclaration` named `next`, with `Presence::Optional` and payload type `ValueType::Composite(List NodeKey)`; removing or renaming `optional`, adding a second nested member, or targeting a non-option node refuses only that item as malformed, never as a required field or a generated oracle. PLANNED (IR-643).
  lines: '518'
- id: FR-018-AC-27
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: Reading the QSpec Tree `kids` field's direct reference to `collection_bounds` follows that node's `semantic_type` to `Sequence<Tree>`, reads its sole element reference as the Tree `NodeKey`, and reconstructs cardinality `[0,3]` from that bound node's named `min` and `max`; changing the semantic type to a non-sequence, dropping or duplicating either bound, or making either value non-integer refuses only that item with a typed cause and emits no code, never silently yielding an unbounded collection. PLANNED (IR-643).
  lines: '519'
- id: FR-018-AC-28
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: With a caller-selected type-resolution work limit of at least 100,000 and `cg.equality.source_bytes` raised to the measured flat source size, an admitted acyclic type chain of 100,000 entries reconstructs and generates an oracle with the same result as an equivalent shallow type on a 512 KiB stack; with the same raised source-byte limit, the default type-resolution work limit refuses it at entry 65,537, and a selected work limit one below its entry count refuses it at the first denied entry. No depth setting or fixed depth refusal participates in these runs. PLANNED (IR-511).
  lines: '520'
- id: FR-018-AC-29
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: A cycle detected after a charged repeated entry reports `TypeResolutionCycle` without following a child; if that repeated entry is the first denied work entry, `TypeResolutionWorkExhausted` wins and reports the effective limit and first denied count. An unaffected sibling keeps its original disposition in either case. PLANNED (IR-511).
  lines: '521'
- id: FR-018-AC-30
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: The 100,000-entry acyclic closure renders as flat source without a type-depth-sized constructor expression; under the 1,048,576-byte default it returns `SourceTooLarge` naming that limit when the source exceeds it, and under a caller limit equal to its measured source bytes it emits a complete, compilable oracle within the publication artifact and bundle byte limits. One byte less refuses before that byte is appended and publishes no partial artifact. PLANNED (IR-511).
  lines: '522'
- id: FR-018-AC-31
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: A caller-selected type-resolution work limit of `u64::MAX` refuses as `InvalidTypeResolutionWorkLimit` before lowering or charging; with `u64::MAX - 1`, the first denied count fits in the `u64` `consumed` field and is reported without wraparound. PLANNED (IR-511).
  lines: '523'
- id: FR-018-AC-32
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: Each IR-admitted direct `temporal`/`formula`, `temporal`/`fairness` and `expression`/`case` request has one claim-map entry keyed by its requested node id and refused as `CompositeEqualityRefusal::Unsupported`. For an IR `Unsupported` lowering record, the refusal's `unsupported_node_id` and `node_tag` equal that record's first unsupported reachable node and tag, which need not be the requested root; for a lowered `case` root they name the root and `"expression"`. The request contributes no generated symbol or source, never panics or disappears, and leaves a supported sibling's claim and source
  lines: '524'
```

## Method and Evidence

Frozen PR488 seven-document diff only; HEAD85dabad84de5be987fa49d08cd18e010d699b386 against actual main12c280f5c1a0a5cd7c0c77e17791d5b21573cf4f. Ticket: IR-17. No frozen worktree edits, Cargo, rustc, formatter, resolver, metadata, native tests or backend runs. No applicable AssuranceProfile or required review-selection profile exists. The delegated brief selects the applicable methods. Canonical proof-content identity bindings are the feature, not file/version tracking; no new pin/catalog is introduced.

Actual published QSL facade at4bd5c07 (containing merged f7cd628) and QSV1a38c06 were read directly from owner Git objects: immutable package-bound enum context; complete ordered cases and genuine keys; Unknown/Foreign typed fields; compilation Package/Resource/Fault; QSV declaration/member/index APIs and four-argument check_equality. Source is evidence, not an instruction. Existing CG composite_builder authenticates source/lock and full recompiled package but discards context; existing emitter has the old ABI. This PR expressly specifies planned work rather than claiming that old source implements the new contract.

Computed strict matrix is not green: 874 criteria =510 tagged,296 untagged,50 method-without-symbol,18 tagged-by-ignored-test. The five new FR018 AC33–37 are planned/unimplemented and untagged. This specification review does not discharge them or claim repository coverage acceptance. Standalone gap-analysis is not selected because the seven-doc-only diff adds no production code or executable test; the full matrix is retained as context, not silently counted as passing. Original TC025/TC027 proof domains and controls are unchanged and no consumer qualification is claimed. Plan completion: not assessed.

Method selection: code-review; spec-review base; integrity; dependency (new owner references); evidence (changed/new methods); failure-domain (identity/admission errors); scope-boundary (owner extraction/ABI); EARS (edited requirements); risk-complexity (new external capability). Rust-review is inapplicable because no Rust/Cargo diff. Object cross-reference checks are folded into base: no independent object archetype change and the installed SpecReview analysis vocabulary has no object-review value. Criterion-strength/Jev, security and implemented-architecture evaluation are not selected. Installed structural validation covers the full spec tree; inherited advisory grammar findings remain in unchanged FR017/FR014 statements and are not attributed to this amendment.

Scope below records all inserted/edited prose blocks and changed acceptance criteria plus explicitly inspected unchanged closure/invariant/charge controls. Verbatim excerpts longer than600 characters are truncated at600 without an invented suffix; line references identify their full unit. Clean units are deliberate examinations, not missing findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for this method and frozen specification diff. Implementation and original consumer proof acceptance remain outstanding.
