---
id: SR-15360
title: IR-17 Union PR511 code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; IR-17; Cargo.lock; Cargo.toml; src/oracle/equality/admitted.rs;
  src/oracle/equality/mod.rs; src/oracle/equality/resolution.rs; src/oracle/equality/union_tests.rs;
  tests/admitted_composite_support/fixture.rs; tests/it/admitted_composite_equality.rs
review_set: subset
---

## Summary

Frozen candidate d93e5189e2b8c78a0c63bb4754a2931f17a05cd3, PR511, ticket IR-17. Reviewed only the implementation delta from origin/code/ir17-qsl-current-migration. No source, test, specification, dependency, execution-owner or branch edit was made. No applicable AssuranceProfile exists under spec/. Specification review is already in PR510; no new spec diff exists here.

No reviewer Cargo/build/test/backend/native-generated execution, namespace creation, probe, bwrap or unshare ran. Read-only quoin/quire computation, diff checks and the lockfile one-copy AWK check ran. Existing author logs were read as supplied evidence, with their limited host scope; they do not establish generated-crate or Kani acceptance. Aggregate gates are expressly forbidden in this phase, not silently omitted as passed.

## Verdict

PASS for the scoped host implementation: no new defect found. Rust review is folded into this code-review artifact. Mergeability is limited to the owned stacked baseline after artifact custody and the lead's safe final checks; this is no native/backend execution acceptance.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Authentic owner lookup retains original wire nodes, qualified name and Union form. Composite descriptors use facade key(), while verified seals use node() and cloned member() bindings after the complete environment is built. Resolution checks member names, lengths, ordered payload types and nested closure, preserving charged entry semantics and nominal cycle handling. Generated reconstruction reacquires owner declarations with typed lookup/reconstruction/declaration propagation before publishing the prepared capability; it derives no owner IDs. Failure preparation has no Meter and performs no equality evaluation. Owner tests independently construct values through public admitted bindings and check value admission, actual equality and distinct same-shaped identities. Public tests check original closure identities, missing/wrong context, charged refusal precedence, deterministic output and unchanged healthy sibling artifacts. Generated-source syntax/contents checks are host evidence only; native behavior is explicitly outside AC44's phase.

Quantity remains explicitly NonRangeType in the existing frame match and Unsupported in equality resolution/rendering (AC18/38). No UnitTable or quantity-operation support is introduced or credited. The manifest/lock update consumes the genuinely required current QSL facade, keeps one copy, and introduces no private API/schema/identity copy. No CI lane, warning gate, resource threshold, unsafe code, wire decoder, async/lifecycle path or coverage gate changes. Search of existing equality reconstruction helpers and the shared genuine fixture found no second owner implementation; the new fixture is one source program shared by both test lanes. All 8 changed files were read. Original fixture lines, 15-case corpus/literals/domains, the six existing controls, protected qualification sources and execution-owner sources have zero incremental diff.

Supplied evidence: union-red.log reaches the intended old Unsupported(composite_type) refusal (not setup failure); union-public-controls.log 2 passed and union-owner-controls.log 2 passed. lint.log and fmt-check.log pass. Six-host-results.json has six exact single-test PASS results, each completing the original 15 cases plus mixed. Existing panic-token and Quantity/Reference renderer controls pass. No broad execution module was run by this reviewer.

## Examined scope

```yaml
clean: true
scope:
- id: FR-018-AC-1
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Every requested item receives exactly one generated or typed-refused disposition;
    a refused item contributes no function, no environment constructor and no symbol
    to the generated crate, while its siblings are generated unchanged.
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A declaration closure `TypeEnvironment::new` refuses is refused at generation
    time carrying that `DeclarationCause`, including both recursion passes. The authentic
    duplicate-field source control instead retains typed AdmittedRefusal::Package
    with owning public CompileRefusal::code() equal to Code::AmbiguousDeclaration
    and cause() equal to None for the Assembly refusal, produces no successful AdmittedContext
    and is not passed to public CG generation. Private CG declaration-admission defenses
    retain the exact DuplicateMember name/cause and unchanged healthy sibling separately,
    without public-acc
- id: FR-018-AC-10
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Generated bytes are identical across repeated runs and across permutations
    of the request order, including a request carrying two descriptors over one node
    id, and claim-map entries are ordered by the descriptor key — expression node
    id, operator rank, then each operand's source-type and conversion-target node
    ids, every node id compared in node-id order; the generated crate is compiled
    and executed under AC-2 against a native run driven from the request's own descriptor
    (AC-3), so a generated crate with a changed emitted operator, operand order or
    descriptor fails AC-2.
- id: FR-018-AC-11
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Every claim marks its operation `caller_declared`, the claim map carries
    the blocked item "operation identity not consumed by codegen's generators", and
    two items over one node differing only in `EqualityOperator` both generate with
    that mark, under distinct symbols, each built from its own operator's stem, and
    produce complementary outcomes on a vector whose operands differ.
- id: FR-018-AC-13
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Every claim-map entry carries the node id, IR id, package id, source map,
    claims, reconstructed declaration keys and selected schedule of its item, and
    its genuine owner-issued declaration keys equal the exact domain and digest of
    the V2 node ids its operand types reach, selected through owning lookup without
    CG/emitted key construction.
- id: FR-018-AC-17
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The `src/lib.rs` that `generate_composite_equality_oracles` returns for
    the TC-029 step 1 package (every generated item) contains zero occurrences of
    the panic tokens FR-018-AC-19 lists, no `.ok()`, `.unwrap_or(` or `.unwrap_or_default(`
    anywhere, and no `[` index or slice expression directly after an identifier character,
    `)` or `]`; every reconstruction helper in it returns `Result<_, ReconstructionError>`,
    and each of `rebuild_integer`, `rebuild_interval`, `rebuild_rational`, `rebuild_decimal`,
    `rebuild_text` and `rebuild_cardinality` turns a failed constructor or parse into
    exactly the mat
- id: FR-018-AC-18
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '`render_value_type` called with `ValueType::Quantity` and with `ValueType::Reference`
    returns `Err(RenderError::UnsupportedValueType { family })` with `family` equal
    to `"quantity"` and `"reference"` respectively and does not panic, and the item-boundary
    mapping turns `RenderError::UnsupportedValueType` into `CompositeEqualityRefusal::Unsupported`
    with `node_tag` equal to `family` and `unsupported_node_id` equal to the item''s
    expression node id, emitting no code for the item and leaving its siblings unchanged,
    while `RenderError::Generation(OracleGenerationError::UnknownRuntimeVariant {
    .. })`'
- id: FR-018-AC-19
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'The non-test code of `src/oracle/equality/mod.rs` (comments and every
    `#[cfg(test)]` item removed wherever the item sits, string literals the generator
    emits counted) contains zero panic tokens: the identifiers `unwrap`, `expect`,
    `unwrap_unchecked`, `unwrap_err`, `expect_err`, `unwrap_err_unchecked`, `panic_any`
    and `resume_unwind` however written (called, with whitespace before the paren,
    named on a path such as `Option::unwrap`, or imported); the macros `panic`, `unreachable`,
    `todo`, `unimplemented`, `assert`, `assert_eq`, `assert_ne`, `debug_assert`, `debug_assert_eq`
    and `debug_assert_ne'
- id: FR-018-AC-25
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'The per-item type-resolution counter charges one unit for each root type-node
    entry or followed type-node reference or bounded-domain `semantic_type` edge,
    including a repeated or memoized reference, across both operands and conversion
    targets, independently of Contract IR lowering; with the default limit, an acyclic
    chain reaches its 65,537th entry and refuses only its item as `TypeResolutionWorkExhausted
    { limit: 65_536, consumed: 65_537 }` before further descent and without call-stack
    exhaustion or generated symbols, while a sibling needing at most 65,536 entries
    still generates. PLANNED (I'
- id: FR-018-AC-38
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: With `None`, standalone generation and emitted preparation accept successfully
    reconstructed non-nominal primitive and nested Option/Collection closures; their
    index is empty only after complete zero-enum resolution. Existing admitted-IR
    conversion-form and quantity-family negatives retain respectively exact FormMismatch("conversion")
    and Unsupported("quantity"), with no replacement ContextPackageMismatch or MissingAdmittedContext,
    no symbols for the refused item and unchanged independent siblings. This does
    not claim matching QSL source emission or source authentication for those IR fixtures.
- id: FR-018-AC-44
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'Genuine Union equality generation preserves the facade declaration key,
    settled node, authoritative member bindings and declared member/payload order
    across a nested Union closure, using the specified lookup and verified sealing
    in both generator and emitted preparation. In-process owner-API controls admit
    values of every member, including record and nested-Union payloads; distinct same-shaped
    unions retain incompatible identities. Missing/wrong capability, Unknown/Foreign/WrongKind
    lookup, name/form disagreement and seal faults retain their typed causes, no successful
    prepared wrapper and no '
- id: Cargo.lock
  path: Cargo.lock
  role: examined
  excerpt: '# This file is automatically @generated by Cargo.'
- id: Cargo.toml
  path: Cargo.toml
  role: examined
  excerpt: '[package]'
- id: src/oracle/equality/admitted.rs
  path: src/oracle/equality/admitted.rs
  role: examined
  excerpt: //! Consume the public replay facade's checker-issued nominal identities.
- id: src/oracle/equality/mod.rs
  path: src/oracle/equality/mod.rs
  role: examined
  excerpt: //! Exact composite/structural equality oracle generation from one admitted
- id: src/oracle/equality/resolution.rs
  path: src/oracle/equality/resolution.rs
  role: examined
  excerpt: //! Bounded, iterative reconstruction of equality operand types.
- id: src/oracle/equality/union_tests.rs
  path: src/oracle/equality/union_tests.rs
  role: examined
  excerpt: //! Host controls for the actual generator's admitted Union closure and
    seals.
- id: tests/admitted_composite_support/fixture.rs
  path: tests/admitted_composite_support/fixture.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
- id: tests/it/admitted_composite_equality.rs
  path: tests/it/admitted_composite_equality.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
findings: []
bindings:
- test_id: src/oracle/equality/union_tests.rs::union_closure_seals_owner_members_and_admits_each_nested_payload
  ac_id: FR-018-AC-44
  trace: correct
- test_id: src/oracle/equality/union_tests.rs::union_lookup_and_seal_faults_keep_typed_causes_before_equality
  ac_id: FR-018-AC-44
  trace: correct
- test_id: tests/it/admitted_composite_equality.rs::genuine_union_host_generation_closes_nested_owner_declarations
  ac_id: FR-018-AC-44
  trace: correct
- test_id: tests/it/admitted_composite_equality.rs::genuine_union_capability_refusals_preserve_the_healthy_sibling
  ac_id: FR-018-AC-44
  trace: correct
```
