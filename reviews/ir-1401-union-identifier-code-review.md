---
id: SR-17940486709564847047575
title: code-review review of generated union Identifier reconstruction
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; spec/oracle/functional/FR-018-composite-equality-oracles.md,
  spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/functional/FR-018-composite-equality-oracles.md,
  src/oracle/equality/mod.rs, tests/it/admitted_composite_equality.rs, tests/admitted_composite_support/union_identifier.rs
review_set: subset
---

## Summary

Frozen head 749ecb00cf3212fe2baca443247c7a3adc11dc07; base 95356b5b6d9e5df6832403d1e369cb50de5ea803. Ticket: IR-1401. Only the three-file +106/-1 patch is reviewed. No applicable AssuranceProfile exists. No branch, dependency, integration, or CI changes.

## Verdict

PASS — validated Identifier construction uses the owning constructor and propagates the existing typed Composite reconstruction failure with no panic or widening. Member order, payload equality, original nominal bindings and seal_union verification remain intact. Native operands originate from a fresh compilation of the same original source; generation consumes unchanged facade package bytes via CheckedPackageV2::read. Equality checks cover Empty self, Flag(true) self and distinct members. The regression is deliberately supplementary: it does not claim standalone full meter/charge agreement or failure injection coverage for AC-2/8/17. Rust API, ownership, error boundary, safety, test execution and duplication checks found no defect.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Reviewed supplied RED log /tmp/IR-1401-union-identifier-red.log: generated resolved() call fails E0308 (Identifier required; &str supplied). Reviewed GREEN /tmp/IR-1401-union-identifier-focused.log: clippy it target completes and outer focused test passes 1/1 (878 filtered). The harness runs generated Cargo tests and asserts declared native test count equals passed count: exactly one inner native test, containing three completed Boolean checks. No 19-test run is claimed. Gates not rerun by reviewer per dispatch: lead owns final aggregate gate; fmt/clippy evidence supplied; full tests/deny/Kani not claimed.

Plan completion: not assessed

Spec-review, EARS, integrity, object, dependency, evidence, risk and failure-domain spec methods are inapplicable: no spec/plan/relationship or domain definition changes. Rust-review is folded into code-review. No optional repository-wide semantic review was run.

```yaml
scope:
- id: FR-018-AC-2
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'On every vector of the composite-equality corpus, the generated oracle''s
    `Outcome<bool>`, its admitted charge sequence and its consumed counters are equal
    to those of `quire_contract_runtime::exact::TypeEnvironment::check_equality` plus
    `CheckedEquality::evaluate` invoked directly on an independently constructed environment,
    operands and fresh `Meter`. This criterion claims no agreement with the QSL value
    authority: that boundary is owned by quire-integration IR-842 (FR-014-AC-47).'
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'A declaration closure `TypeEnvironment::new` refuses is refused at generation
    time carrying that `DeclarationCause`, including both recursion passes. The authentic
    duplicate-field source control instead retains typed AdmittedRefusal::Package
    with owning public CompileRefusal::code() equal to Code::AmbiguousDeclaration
    and cause() equal to Some("ambiguous-name") for the Assembly refusal, preserving
    the public native cause published by QSL, produces no successful AdmittedContext
    and is not passed to public CG generation. Private CG declaration-admission defenses
    retain the exact DuplicateMember '
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
- id: src/oracle/equality/mod.rs
  path: src/oracle/equality/mod.rs
  role: examined
  excerpt: exact::Identifier::new(member.identifier()).map_err(|_| ReconstructionError::Composite)?
- id: tests/it/admitted_composite_equality.rs
  path: tests/it/admitted_composite_equality.rs
  role: examined
  excerpt: fn genuine_union_reconstruction_compiles_and_prepares_typed_members()
- id: tests/admitted_composite_support/union_identifier.rs
  path: tests/admitted_composite_support/union_identifier.rs
  role: examined
  excerpt: fn reconstructed_union_members_execute_equality()
```
