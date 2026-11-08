---
id: SR-17940486709564847047571
title: "IR-1401 PR757 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-1401; src/oracle/scalar/lower.rs; tests/it/scalar_source_forms.rs; tests/it/main.rs; spec/oracle/functional/FR-014-exact-scalar-oracles.md; spec/oracle/matrix/TC-024-exact-scalar-oracles.md"
review_set: subset
---

## Summary

All four new ACs are computed tagged, but tagging is not execution credit. The genuine carried-source test currently fails before CG at strict-reader InvalidDerivedShape (/semantic_graph/nodes/5/node_id). The old-lowerer log reproduces IEEE FormMismatch, while carried input fails at the same reader boundary before both lowerers. Consequently neither carried old-lowerer RED nor carried native GREEN exists.

Frozen candidate 8512ca2536342d46924c2c63868f808ce8719ab4 against origin/main cd184a37. No dependency, CI, public API or lockfile changes. No applicable AssuranceProfile exists under spec/. Full gates and proof execution were excluded by the lead; existing focused execution logs were inspected, not rerun after the reported cache deletion. No cache or worktree was removed by this reviewer.

## Verdict

**FAIL** — Open carried-source qualification prevents acceptance of the whole feature.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Genuine carried-source qualification remains blocked by the locked strict reader before CG; the active test fails InvalidDerivedShape, so carried generation, native execution and old-lowerer RED-to-GREEN evidence required by AC-69 are absent. | tests/it/scalar_source_forms.rs:148 |

## Coverage

Frozen candidate 8512ca2536342d46924c2c63868f808ce8719ab4 against origin/main cd184a37. No dependency, CI, public API or lockfile changes. No applicable AssuranceProfile exists under spec/. Full gates and proof execution were excluded by the lead; existing focused execution logs were inspected, not rerun after the reported cache deletion. No cache or worktree was removed by this reviewer.

Reviewed all changed source/tests/spec files. Source uses one shared shape and operand predicate in both lowerer validation points. Existing caller-declared descriptor policy is preserved. No duplicated catalog, arbitrary-family admission or disabled gate was introduced.

Plan completion: not assessed

Computed repository matrix: 631 tagged, 309 untagged, 93 method-without-symbol and 12 tagged-by-ignored-test. These pre-existing repository gaps preclude a repository-wide PASS; this change adds four tagged criteria. New bindings were checked against test bodies. The carried binding is active but failing, not ignored. No proof or QSL value-authority agreement is claimed. Normal main-owner refresh compilation is unavailable (RT/QSL check_type API mismatch; /tmp/ir1401-refreshed.log); no private catalog or dependency override was used.

Evidence inspected: /tmp/ir1401-final-focused.log (4 pass, 1 fail); /tmp/ir1401-red.log (IEEE FormMismatch and carried reader failure); /tmp/ir1401-clippy.log and /tmp/ir1401-fmt.log (pass); independently computed /tmp/ir1401-review-matrix.json and /tmp/ir1401-review-spec.log. No Kani or backend proof was executed.

## Examined units

- FR-014-AC-66 — role: examined — spec/oracle/functional/FR-014-exact-scalar-oracles.md — excerpt: For a strict-reader-admitted `quire.op.quantity.convert`, CG accepts a referenced `scalar_type`/`carried_quantity` source operand permitted by the owner operation contract, including the genuine QSL-emitted TC-187 U01 form, and derives and generates an `ir_confirmed` oracle. Source carrier, unit and rounding and the admitted exact or carried target identity and domain remain intact; execution uses the existing owner conversion contract without replacing a carried source with an unbounded or exact carrier.
- FR-014-AC-67 — role: examined — spec/oracle/functional/FR-014-exact-scalar-oracles.md — excerpt: For strict-reader-admitted `quire.op.ieee.numeric_equal`, `quire.op.ieee.total_order` and `quire.op.ieee.bit_identical` nodes, CG accepts the genuine QSL-emitted `expression`/`call` semantic form with the catalogued `call` body operator, including numeric equality emitted from `a = b` and each `quire::value::ieee::*` intrinsic. Existing reader-admitted binary-semantic-form fixtures with that same catalogued call operator remain supported. Derivation and generation retain operand order, float width, Boolean result, operation identity, laws and mode, and dispatch the selected owner IEEE operatio
- FR-014-AC-68 — role: examined — spec/oracle/functional/FR-014-exact-scalar-oracles.md — excerpt: Acceptance of these carried operands and IEEE call forms preserves the existing strict-reader and CG boundaries. Unadmitted operation/body pairings and reader-invalid arity, types, identity, laws or mode remain reader refusals. Admitted shapes, arity, operand/result types, unitless quantity literals and unsupported semantic forms that CG previously refused retain typed refusal, no oracle and supported-sibling isolation. A same-lowering-shape descriptor identity, law or mode mismatch retains the existing FR-014-AC-12 and FR-038-AC-3 behavior: where generation was permitted, its operation remain
- FR-014-AC-69 — role: examined — spec/oracle/functional/FR-014-exact-scalar-oracles.md — excerpt: Qualification of AC-66 and AC-67 emits genuine QSL source through the public facade, passes its unchanged emitted bytes through the strict Contract IR reader, derives descriptors and generates, compiles and executes the resulting native scalar oracles. Tests assert completed conversion values and units and IEEE true and false values, including signed-zero and NaN cases that distinguish numeric equality, total order and bit identity according to the owner contract. Generated and direct owner executions agree in complete, refused or incomplete outcome, ordered admitted charges and consumed count

TC-024 added genuine-source qualification procedure examined in full; prior AC-12 / FR-038-AC-3 and AC-62–65 were context only. No integrity/dependency/object lens applies: no new object, edge or structural artifact schema was added. Criterion-strength skipped because not selected; no Jev client invoked.

## Disposition review scope (round 2)

Reviewed frozen 2afe6beb058b2898b3a372e0e47ba4af985374b1, including actual shared native_quantity_checks harness, checked carried metadata assertions and the explicitly component-only execution test. Inspected /tmp/ir1401-component.log (actual owner admission/conversion harness compiles and executes successfully) and /tmp/ir1401-fix1-ieee.log (genuine IEEE native recheck passes). The component run substitutes only the owner conversion call in the shared harness; it is not generated-oracle or genuine strict-reader qualification. No new production/dependency change, source edit, cache cleanup, full gate or proof execution by reviewer.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | still-open | The genuine carried-source test remains active and fails InvalidDerivedShape before CG. Component owner execution cannot supply strict-reader admission, generated native execution or preceding-lowerer RED-to-GREEN. Owner/dependency qualification remains blocked; the feature is not mergeable. |

## Current verdict

**FAIL** — FND-001 is still open; no merge clearance.

## Expanded review (round 3)

Expanded frozen review 7d509f33e98c192ad901d805a2029e6b5d31bb88 covers Cargo.lock normal main refresh, src/oracle/equality/{mod.rs,preparation_tests.rs,units.rs}, routed SMT caller EmitLimits compatibility, composite equality native fault fixtures and FR-018/TC-029. User-authorized spec fabe197e preceded implementation. No applicable AssuranceProfile found. No new owner API, exact Outcome variant, fake limit, private catalog or mirror carrier was introduced.

Production emitter checks actual owner comparison types left before right during preparation; Refused(IllTyped) moves into private state and prevents later traversal, Allocation/Capacity move the original TypeCheckError into EnvironmentError::TypeCheck. Oracle context/package/item guards precede the stored semantic cause; healthy checked equality retains the same private index and meter. Private prepared state has no public mutation API. Descriptor construction and prepared environment remain bound by the existing immutable package context and item guard.

Component evidence is actual SourceBuilder output compiled/executed using genuine producer-issued identities and owner allocator, not strict IR-admitted source qualification. It tests left/right native allocation errors with original request unit/amount, semantic-left precedence over later native denial, right semantic refusal, healthy values/ordered charges/counters and guard precedence. Capacity is a direct ownership-transfer match verified statically, not fabricated runtime overflow. The native component injects private faults only and adds no production test bypass/public seam. /tmp/ir1401-equality-component.log: two preparation tests pass; /tmp/ir1401-equality-alltargets.log: all-target check passes. No Cargo was run by reviewer.

Current genuine-source evidence /tmp/ir1401-resume-focused.log remains blocked before CG by owner operation_catalog.rs:278 rejecting carried_member in the refreshed catalog, affecting both quantity and IEEE. The component pass does not replace this qualification. IR-946 owner dependency is outstanding. No final aggregate gate/proof execution/merge clearance.

Independent matrix: 633 tagged, 309 untagged, 93 method-without-symbol, 12 tagged-by-ignored-test; new FR-018-AC-48/49 both tagged. Existing repository-wide gaps remain. Plan completion: not assessed. Spec validation passes with five pre-existing grammar warnings; changed EARS statements identify preparation/oracle actors and measurable responses. No new structural object or dependency edge requires another spec lens.

- FR-018-AC-8 — role: examined — spec/oracle/functional/FR-018-composite-equality-oracles.md — excerpt: A declaration closure `TypeEnvironment::new` refuses is refused at generation time carrying that `DeclarationCause`, including both recursion passes. The authentic duplicate-field source control instead retains typed AdmittedRefusal::Package with owning public CompileRefusal::code() equal to Code::AmbiguousDeclaration and cause() equal to None for the Assembly refusal, produces no successful AdmittedContext and is not passed to public CG generation. Private CG declaration-admission defenses retain the exact DuplicateMember name/cause and unchanged healthy sibling separately, without public-acc
- FR-018-AC-42 — role: examined — spec/oracle/functional/FR-018-composite-equality-oracles.md — excerpt: Generated oracle invocations under Some or valid context-free None preserve private package/item/descriptor/index binding, both preparation-time owner type checks, retained semantic-refusal handling, same-index check_equality and evaluate. Private wrong-binding, present-context mismatch, missing nominal context and environment-only faults refuse without charge. The native OriginalCompositeEqContext route retains every mandatory authentic source/lock/package/whole-IR comparison and cannot use None to bypass them. No scalar proof manifest/body acquires replay compilation.
- FR-018-AC-48 — role: examined — spec/oracle/functional/FR-018-composite-equality-oracles.md — excerpt: Emitted preparation uses the actual owner `try_check_type` boundary. A real allocator denial during its type traversal returns `EnvironmentError::TypeCheck(TypeCheckError::Allocation(request))` retaining the original native class, unit and amount; any owner Capacity result retains its original request and variant. Error projection moves the owner error without an allocating clone, produces no prepared wrapper and admits no equality-meter charge. Generated code is compiled and executed for the real denial and healthy retry; structural inspection verifies the direct move for Capacity where a rea
- FR-018-AC-49 — role: examined — spec/oracle/functional/FR-018-composite-equality-oracles.md — excerpt: Preparation checks comparison types in left-before-right order. An actual left `Refused(IllTyped)` is retained unchanged and prevents the right traversal, including a right type whose traversal would hit a real native allocator denial. The later oracle returns the existing `GeneratedTypeCheckRejected` with that left cause and zero equality charges. When the left succeeds, a right semantic refusal retains its own cause; either native error stops preparation losslessly. Valid prepared operands preserve native equality values, charge order and counters, and private context/item guards retain prec
- FR-018-AC-8 — role: examined — spec/oracle/functional/FR-018-composite-equality-oracles.md — excerpt: Emit only `check_equality`, omitting the `check_type` calls; `check_equality` consults the environment solely through `contains_ieee`, which returns `false` for an absent composite key, so an empty environment completes with a Boolean instead of refusing. Or emit `.expect(..)`, which panics instead; count a private duplicate-member defense as public source-authenticated acceptance, or replace its precise private cause with a generic context mismatch; invent a public Assembly cause tag where CompileRefusal::cause() returns None.
- FR-018-AC-42 — role: examined — spec/oracle/functional/FR-018-composite-equality-oracles.md — excerpt: Drop private binding/context checks for None, allow a nominal recipe without context, omit check_type, accept a caller index, or make native authentication optional.

## Dispositions (round 3)

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | still-open | Normal main refresh now compiles, but the active genuine quantity and IEEE cases panic before CG because owner catalog admission rejects carried_member. No current genuine source-to-generated native agreement or carried preceding-lowerer RED-to-GREEN exists; the expanded equality component pass grants no genuine admission credit. |

## Latest verdict (round 3)

**FAIL** — genuine-source owner dependency gap remains open.

## Focused disposition review (round 4)

Frozen e1f8339098d4ee714849f2907e5ea21a09431932 removes the entire bespoke root/scratch Cargo.lock git-source scan, with no replacement gate. Ordinary root lock seeding, offline metadata resolution and locked native execution remain. The production error/precedence policy is unchanged. Inspected /tmp/ir1401-round3-preparation.log: both actual preparation tests pass; /tmp/ir1401-round3-clippy.log: touched library Clippy passes. No new Cargo run, source edit, cache cleanup or worktree operation by reviewer.

No new finding or changed scope in this method. Existing spec verdicts remain PASS; gap-analysis remains FAIL because the owner catalog blocks actual genuine quantity/IEEE qualification before CG. No component result is counted as genuine acceptance.

## Final frozen review (round 5)

Frozen head da4c8a9d7350503bae89ec8ba16083ee719403dc; full origin/main...HEAD diff, including consumer API migrations, inspected. Historical finding text and disposition history preserved. No applicable AssuranceProfile exists (spec/assurance contains architecture decisions, not profiles). Existing execution evidence was inspected; no Cargo, aggregate gate, workspace/stress build, Kani, branch edit, push or merge was performed by this reviewer. Focused report /tmp/CG757-focused-report.json and every listed log inspected; logs show exactly selected tests executed, not zero-test success. Broad Rust gates and cargo-deny were not rerun, per brief. Format and focused library/integration Clippy evidence passed. Plan completion: not assessed.

Historical FND-001 is fixed: genuine U01 source is emitted through public compile_package, unchanged bytes pass CheckedPackageV2::read, derived generated source is compiled and its native values_and_ledgers test is required to execute. Source carrier/unit/rounding are asserted; native owner admission rejects out-of-domain/grid/unit controls; generated and direct conversion complete at 5/2 m and compare full incomplete records, charges and all counters. IEEE source covers a=b and all three intrinsics at both widths, signed zero and NaN, true/false values and denial paths. The preserved red logs fail at the intended CG OperandTypeMismatch/FormMismatch, and restored green logs execute successfully. Component-only preparation evidence is not counted as genuine scalar qualification.

Independent computed matrix remains 633 tagged, 309 untagged, 93 method-without-symbol, 12 tagged-by-ignored-test. These historical repository gaps prevent a repository-wide PASS; changed criterion bindings are examined against their actual test bodies and introduce no new gap. Review verdict for the changed PR scope: PASS; repository audit verdict: FAIL (pre-existing untagged criteria), not a new PR finding. Optional standalone semantic gap-analysis mode was not invoked; criterion/body agreement is evaluated by code-review as requested.
## Dispositions (round 5)

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Inspected genuine unchanged QSL emission through strict admission, CG generation and required compiled native values_and_ledgers execution; carried/IEEE red logs fail at the intended CG refusals and restored green logs pass. |

## Latest verdict (round 5)

**FAIL (repository baseline); PASS (changed PR scope)** — FND-001 is fixed; no open PR findings.

## Final examined scope

```yaml
scope:
- id: FR-014-AC-2
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Integer (add, subtract, multiply, negate, truncating/floor/Euclidean division,
    modulo), rational, ordering, decimal, IEEE arithmetic/comparison/width conversion,
    text comparison, enum comparison (including equality and inequality over a selected
    model-owned `model`/`variant_type`, using its original checked member identities),
    quantity arithmetic/comparison and exact quantity conversion (FR-014-AC-48) descriptors
    for admitted checked-package expression nodes each generate a function calling
    the matching direct owner operator described above. Entry-point dispositions for
    a `quire.op.quantity.co
- id: FR-014-AC-3
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A descriptor whose arity, result or operand scalar type disagrees with
    the lowered node, a node that is not a lowered scalar expression, a node that
    is unbounded, has an invalid or incomplete body, or exhausts lowering work, or
    a node id requested more than once, is refused with a typed reason.
- id: FR-014-AC-6
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Executing generated oracles on the QSpec TC-185, TC-186, TC-192 and TC-193
    vectors, and on these QSpec TC-187 vectors, yields outcomes, charges and consumed
    counters equal to direct runtime execution: U02 (dimension normalization through
    a product and a quotient, and a zero power), U03 (the dimension mismatch), U05
    (arithmetic and a power on `degC` and `degF`, refused `ill_typed`), U09 (a base
    dimension of the same declaration and shape as `L` under a different node key,
    refused `ill_typed`), U12 (compound-unit products, quotients and powers, with
    the `s` family), U13 and U21 (powers), U14 (th'
- id: FR-014-AC-12
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A descriptor naming a different catalogued operation than the node's own
    is not confirmed, including where the two share one operand shape and one checked
    bound; nor is a descriptor naming an operation the catalogue has no entry for.
- id: FR-014-AC-18
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: '`derive_exact_scalar_items` over an admitted V2 package returns, per node
    id in input order, the descriptor determined by the node''s `operation.identity`,
    `operation.mode`, `operation.laws`, operand forms and the one bound of the needed
    form on its result type, or for an unnarrowed Integer division/modulo result the
    sound interval derived from both finite operands as described above; a node that
    is not an application, has no identity, names an identity outside the derivable
    set, has operand forms, a law or a mode that select no parameter, is refused with
    the matching `ClaimDerivationRefusal` a'
- id: FR-014-AC-21
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A node whose result is typed by a scalar type and whose reachable bounds
    are its operands'' own and exactly one other takes that other as its result bound;
    a sole reachable operand-owned bound is not a result bound and yields `MissingBound`
    unless a plain reference operand requires a bound under FR-014-AC-25. For an unnarrowed
    Integer `div` or `mod`, both finite operands instead derive `[-A, A]` or `[0,
    max(B - 1, 0)]` respectively, using the largest absolute dividend/divisor endpoints;
    missing operand bounds still refuse `MissingBound`. Explicit or narrowed result
    bounds remain authoritative. '
- id: FR-014-AC-25
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A `reference` operand to a plain scalar parameter or a non-literal `value`
    node is refused as `RequiresBound` naming its type, in generation and in derivation,
    where another operand owns a bound or the result is typed by one (`x + y`, `y
    + y`, and `x + y` over a scalar-typed result); this refusal takes precedence over
    FR-014-AC-21''s `MissingBound` when the only reachable result-bound candidate
    is another operand''s own bound. A literal operand is never refused as `RequiresBound`.
    An eligible application reference follows FR-014-AC-45 instead. The unnarrowed
    division/modulo operand-derived case '
- id: FR-014-AC-66
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: For a strict-reader-admitted `quire.op.quantity.convert`, CG accepts a
    referenced `scalar_type`/`carried_quantity` source operand permitted by the owner
    operation contract, including the genuine QSL-emitted TC-187 U01 form, and derives
    and generates an `ir_confirmed` oracle. Source carrier, unit and rounding and
    the admitted exact or carried target identity and domain remain intact; execution
    uses the existing owner conversion contract without replacing a carried source
    with an unbounded or exact carrier.
- id: FR-014-AC-67
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: For strict-reader-admitted `quire.op.ieee.numeric_equal`, `quire.op.ieee.total_order`
    and `quire.op.ieee.bit_identical` nodes, CG accepts the genuine QSL-emitted `expression`/`call`
    semantic form with the catalogued `call` body operator, including numeric equality
    emitted from `a = b` and each `quire::value::ieee::*` intrinsic. Existing reader-admitted
    binary-semantic-form fixtures with that same catalogued call operator remain supported.
    Derivation and generation retain operand order, float width, Boolean result, operation
    identity, laws and mode, and dispatch the selected owner IEEE operatio
- id: FR-014-AC-68
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Acceptance of these carried operands and IEEE call forms preserves the
    existing strict-reader and CG boundaries. Unadmitted operation/body pairings and
    reader-invalid arity, types, identity, laws or mode remain reader refusals. Admitted
    shapes, arity, operand/result types, unitless quantity literals and unsupported
    semantic forms that CG previously refused retain typed refusal, no oracle and
    supported-sibling isolation. A same-lowering-shape descriptor identity, law or
    mode mismatch retains the existing FR-014-AC-12 and FR-038-AC-3 behavior: where
    generation was permitted, its operation remain'
- id: FR-014-AC-69
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Qualification of AC-66 and AC-67 emits genuine QSL source through the public
    facade, passes its unchanged emitted bytes through the strict Contract IR reader,
    derives descriptors and generates, compiles and executes the resulting native
    scalar oracles. Tests assert completed conversion values and units and IEEE true
    and false values, including signed-zero and NaN cases that distinguish numeric
    equality, total order and bit identity according to the owner contract. Generated
    and direct owner executions agree in complete, refused or incomplete outcome,
    ordered admitted charges and consumed count
- id: FR-014-AC-70
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Each generated lazy And, Or and Implies scalar oracle accepts its right
    operand as a function of the caller's mutable exact Meter. A reached right operand
    runs exactly once on that same meter after any admitted caller prefix and before
    the single parent boolean result-retention charge. A left-decided result skips
    the right operand and retains only the parent result. A right Refused or Incomplete
    outcome propagates unchanged without parent result retention; parent-retention
    denial preserves the complete owner Incomplete record. No replacement meter, charge
    subtraction, replay or duplicate charg
- id: FR-014-AC-71
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A compiled generated crate exercises lazy And, Or and Implies with both
    left values, actual metered owner Boolean equality on the right, and a nonempty
    admitted caller-meter prefix. Compared with an independently sequenced owner reference,
    it checks values or complete typed stops, ordered admitted charges and every consumed
    counter at generous limits and every denied next-charge boundary, including right-evaluation
    denial and parent-retention denial. Skipped right closures have zero invocations
    and charges; reached closures have exactly one invocation. The controls fail for
    the old no-meter ri
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
- id: FR-018-AC-42
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Generated oracle invocations under Some or valid context-free None preserve
    private package/item/descriptor/index binding, both preparation-time owner type
    checks, retained semantic-refusal handling, same-index check_equality and evaluate.
    Private wrong-binding, present-context mismatch, missing nominal context and environment-only
    faults refuse without charge. The native OriginalCompositeEqContext route retains
    every mandatory authentic source/lock/package/whole-IR comparison and cannot use
    None to bypass them. No scalar proof manifest/body acquires replay compilation.
- id: FR-018-AC-48
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Emitted preparation uses the actual owner `try_check_type` boundary. A
    real allocator denial during its type traversal returns `EnvironmentError::TypeCheck(TypeCheckError::Allocation(request))`
    retaining the original native class, unit and amount; any owner Capacity result
    retains its original request and variant. Error projection moves the owner error
    without an allocating clone, produces no prepared wrapper and admits no equality-meter
    charge. Generated code is compiled and executed for the real denial and healthy
    retry; structural inspection verifies the direct move for Capacity where a rea
- id: FR-018-AC-49
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Preparation checks comparison types in left-before-right order. An actual
    left `Refused(IllTyped)` is retained unchanged and prevents the right traversal,
    including a right type whose traversal would hit a real native allocator denial.
    The later oracle returns the existing `GeneratedTypeCheckRejected` with that left
    cause and zero equality charges. When the left succeeds, a right semantic refusal
    retains its own cause; either native error stops preparation losslessly. Valid
    prepared operands preserve native equality values, charge order and counters,
    and private context/item guards retain prec
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Emit only `check_equality`, omitting the `check_type` calls; `check_equality`
    consults the environment solely through `contains_ieee`, which returns `false`
    for an absent composite key, so an empty environment completes with a Boolean
    instead of refusing. Or emit `.expect(..)`, which panics instead; count a private
    duplicate-member defense as public source-authenticated acceptance, or replace
    its precise private cause with a generic context mismatch; invent a public Assembly
    cause tag where CompileRefusal::cause() returns None.
- id: FR-018-AC-42
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Drop private binding/context checks for None, allow a nominal recipe without
    context, omit check_type, accept a caller index, or make native authentication
    optional.
- id: Cargo.lock
  path: Cargo.lock
  role: examined
  excerpt: '# This file is automatically @generated by Cargo.'
- id: Cargo.toml
  path: Cargo.toml
  role: examined
  excerpt: '[package]'
- id: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: '---'
- id: spec/oracle/functional/FR-018-composite-equality-oracles.md
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '---'
- id: spec/oracle/matrix/TC-024-exact-scalar-oracles.md
  path: spec/oracle/matrix/TC-024-exact-scalar-oracles.md
  role: examined
  excerpt: '---'
- id: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: '---'
- id: src/core/contract_package.rs
  path: src/core/contract_package.rs
  role: examined
  excerpt: // Copyright (C) 2026 Agent-IX
- id: src/core/ir/clause.rs
  path: src/core/ir/clause.rs
  role: examined
  excerpt: //! Admitted checked-package state-clause body access.
- id: src/core/ir/node_key.rs
  path: src/core/ir/node_key.rs
  role: examined
  excerpt: // Copyright (C) 2026 Agent-IX
- id: src/core/scalar_authority.rs
  path: src/core/scalar_authority.rs
  role: examined
  excerpt: //! Original admitted positional scalar body retained by the scalar renderer
    (FR-032).
- id: src/evidence/v2_llvm.rs
  path: src/evidence/v2_llvm.rs
  role: examined
  excerpt: //! Checked V2 clause population and bounded LLVM probe observations (FR-004-AC-16..24).
- id: src/kani/generate/clause/v2.rs
  path: src/kani/generate/clause/v2.rs
  role: examined
  excerpt: //! Bounded harnesses for admitted V2 scalar clauses (FR-015).
- id: src/kani/generate/collection.rs
  path: src/kani/generate/collection.rs
  role: examined
  excerpt: //! Source-bound collection domains derived from admitted V2 model-field
    projections.
- id: src/kani/generate/constrained.rs
  path: src/kani/generate/constrained.rs
  role: examined
  excerpt: //! A constrained search method retains its boundary separately from its
    safety obligation.
- id: src/kani/generate/negotiate.rs
  path: src/kani/generate/negotiate.rs
  role: examined
  excerpt: //! Separate bounded Kani obligations with per-item backend negotiation
    (FR-015).
- id: src/kani/generate/render.rs
  path: src/kani/generate/render.rs
  role: examined
  excerpt: //! Pure ordered emission of validated harness descriptions (FR-015).
- id: src/kani/generate/role_parity_tests.rs
  path: src/kani/generate/role_parity_tests.rs
  role: examined
  excerpt: //! Crate-internal parity checks for state-frame role negotiation.
- id: src/oracle/checked_graph.rs
  path: src/oracle/checked_graph.rs
  role: examined
  excerpt: //! Borrowed checked terms and keyed nodes shared by oracle consumers.
- id: src/oracle/checked_graph/tests.rs
  path: src/oracle/checked_graph/tests.rs
  role: examined
  excerpt: // Copyright (C) 2026 Agent-IX
- id: src/oracle/clause.rs
  path: src/oracle/clause.rs
  role: examined
  excerpt: //! V2 state-clause condition oracles over admitted checked packages.
- id: src/oracle/equality/mod.rs
  path: src/oracle/equality/mod.rs
  role: examined
  excerpt: //! Exact composite/structural equality oracle generation from one admitted
- id: src/oracle/equality/preparation_tests.rs
  path: src/oracle/equality/preparation_tests.rs
  role: examined
  excerpt: // Copyright (C) 2026 Agent-IX
- id: src/oracle/equality/union_tests.rs
  path: src/oracle/equality/union_tests.rs
  role: examined
  excerpt: //! Host controls for the actual generator's admitted Union closure and
    seals.
- id: src/oracle/equality/units.rs
  path: src/oracle/equality/units.rs
  role: examined
  excerpt: //! Attach the original admitted owner's unit metadata to a checked environment.
- id: src/oracle/function/mod.rs
  path: src/oracle/function/mod.rs
  role: examined
  excerpt: //! Exact complete-V1 function-application oracle generation from one admitted
- id: src/oracle/function/owner_path.rs
  path: src/oracle/function/owner_path.rs
  role: examined
  excerpt: '//! FR-021: apply a generated body through QSL''s retained checked closure.'
- id: src/oracle/scalar/derive.rs
  path: src/oracle/scalar/derive.rs
  role: examined
  excerpt: // Copyright (C) 2026 Agent-IX
- id: src/oracle/scalar/lower.rs
  path: src/oracle/scalar/lower.rs
  role: examined
  excerpt: // Copyright (C) 2026 Agent-IX
- id: src/oracle/scalar/lower/opaque_metadata_tests.rs
  path: src/oracle/scalar/lower/opaque_metadata_tests.rs
  role: examined
  excerpt: // Copyright (C) 2026 Agent-IX
- id: src/oracle/scalar/lower/source_selection_tests.rs
  path: src/oracle/scalar/lower/source_selection_tests.rs
  role: examined
  excerpt: // Copyright (C) 2026 Agent-IX
- id: src/oracle/scalar/mod.rs
  path: src/oracle/scalar/mod.rs
  role: examined
  excerpt: //! Exact complete-V1 scalar oracle generation from CheckedPackage V2 (FR-014).
- id: src/oracle/scalar/render.rs
  path: src/oracle/scalar/render.rs
  role: examined
  excerpt: // Copyright (C) 2026 Agent-IX
- id: src/replay/composite_builder.rs
  path: src/replay/composite_builder.rs
  role: examined
  excerpt: // Copyright (C) 2026 Agent-IX
- id: src/replay/frame.rs
  path: src/replay/frame.rs
  role: examined
  excerpt: '//! Native replay of a frame counterexample: a Kani falsification of an
    operation''s frame is put'
- id: src/replay/function.rs
  path: src/replay/function.rs
  role: examined
  excerpt: '//! The replay adapter: a decoded Kani falsification goes to QSL''s layer-6
    `replay` facade, and'
- id: src/replay/scalar_builder.rs
  path: src/replay/scalar_builder.rs
  role: examined
  excerpt: //! Checked scalar plans. Execution and the QSL facade call belong to the
    driver.
- id: src/replay/state_clause.rs
  path: src/replay/state_clause.rs
  role: examined
  excerpt: '//! Native replay of a postcondition state-clause counterexample: a Kani
    falsification of an'
- id: src/routed/capability.rs
  path: src/routed/capability.rs
  role: examined
  excerpt: '//! Capability settlement: the one point where a requested claim is settled.'
- id: src/routed/contract_package_tests.rs
  path: src/routed/contract_package_tests.rs
  role: examined
  excerpt: // Copyright (C) 2026 Agent-IX
- id: src/routed/smt/refinement_source_tests.rs
  path: src/routed/smt/refinement_source_tests.rs
  role: examined
  excerpt: //! Source-authored FR-315 query association for FR-045-AC-4.
- id: src/strategy/v2_numeric.rs
  path: src/strategy/v2_numeric.rs
  role: examined
  excerpt: // Copyright (C) 2026 Agent-IX
- id: tests/checked_package_support/base.rs
  path: tests/checked_package_support/base.rs
  role: examined
  excerpt: // The base CheckedPackage V2 document the exact-scalar and composite-equality
    package builders
- id: tests/checked_package_support/evidence.rs
  path: tests/checked_package_support/evidence.rs
  role: examined
  excerpt: //! Reader evidence held from published QSpec definition artifacts, independently
    of wire claims.
- id: tests/checked_package_support/rekey.rs
  path: tests/checked_package_support/rekey.rs
  role: examined
  excerpt: //! Reader admission for fixtures whose producers assign structural node
    IDs.
- id: tests/clause_admission_support/mod.rs
  path: tests/clause_admission_support/mod.rs
  role: examined
  excerpt: //! Admission-only QSL fixtures for request-level clause callers.
- id: tests/composite_equality_support/cases/record.rs
  path: tests/composite_equality_support/cases/record.rs
  role: examined
  excerpt: //! Original numeric domains and adverse vectors, with independently admitted
    source identities.
- id: tests/composite_equality_support/package.rs
  path: tests/composite_equality_support/package.rs
  role: examined
  excerpt: //! Admitted CheckedPackage V2 fixtures for composite/structural equality
- id: tests/exact_function_support/agreement_cases.rs
  path: tests/exact_function_support/agreement_cases.rs
  role: examined
  excerpt: '//! Executed FR-021 corpus: generated bodies and independent FR-485 session
    application.'
- id: tests/exact_function_support/imported_cases.rs
  path: tests/exact_function_support/imported_cases.rs
  role: examined
  excerpt: //! Executed imported-owner FR-021 parity through the FR-485 facade.
- id: tests/exact_function_support/invariant_cases.rs
  path: tests/exact_function_support/invariant_cases.rs
  role: examined
  excerpt: //! Execute the same emitted guards with genuine checked-owner inputs.
- id: tests/exact_function_support/package.rs
  path: tests/exact_function_support/package.rs
  role: examined
  excerpt: //! Admitted CheckedPackage V2 fixtures for function-application oracle
- id: tests/exact_scalar_support/boolean_agreement_cases.rs
  path: tests/exact_scalar_support/boolean_agreement_cases.rs
  role: examined
  excerpt: '//! FR-014-AC-37: admitted V2 Boolean and scalar equality oracles against
    direct Runtime calls.'
- id: tests/exact_scalar_support/package.rs
  path: tests/exact_scalar_support/package.rs
  role: examined
  excerpt: //! Admitted CheckedPackage V2 fixtures for exact scalar generation.
- id: tests/it/admitted_composite_equality.rs
  path: tests/it/admitted_composite_equality.rs
  role: examined
  excerpt: //! Authentic source-backed record/tuple generation and native owner agreement.
- id: tests/it/admitted_enum_equality.rs
  path: tests/it/admitted_enum_equality.rs
  role: examined
  excerpt: //! Native generated-ABI acceptance uses genuine source compilation, never
    synthetic keys.
- id: tests/it/capability_settlement.rs
  path: tests/it/capability_settlement.rs
  role: examined
  excerpt: '//! FR-019 capability settlement: one `negotiate_*` arm over a closed
    backend kind.'
- id: tests/it/composite_artifact_binding.rs
  path: tests/it/composite_artifact_binding.rs
  role: examined
  excerpt: //! FR-033-AC-18 native artifact authentication. Native Boolean execution
    is not
- id: tests/it/composite_equality_agreement.rs
  path: tests/it/composite_equality_agreement.rs
  role: examined
  excerpt: //! FR-018-AC-2, AC-8 (the `check_type` guard half), AC-9 and AC-11 (the
- id: tests/it/composite_equality_generation.rs
  path: tests/it/composite_equality_generation.rs
  role: examined
  excerpt: '//! FR-018: composite/structural equality oracle generation over admitted'
- id: tests/it/composite_original_eq.rs
  path: tests/it/composite_original_eq.rs
  role: examined
  excerpt: // Copyright (C) 2026 Agent-IX
- id: tests/it/composite_parity_converter.rs
  path: tests/it/composite_parity_converter.rs
  role: examined
  excerpt: '//! FR-033/FR-030: bind public QSL composite reports before reading their
    terminal values.'
- id: tests/it/composite_record_residuals.rs
  path: tests/it/composite_record_residuals.rs
  role: examined
  excerpt: //! Native artifact publication boundaries; no backend-proof or shadow
    credit.
- id: tests/it/constrained_method.rs
  path: tests/it/constrained_method.rs
  role: examined
  excerpt: //! Authentic QSL inputs exercise the single generation entry and dedicated
    method execution.
- id: tests/it/constrained_method/support.rs
  path: tests/it/constrained_method/support.rs
  role: examined
  excerpt: //! Authentic QSL emission, public IR admission, then the constrained family.
- id: tests/it/exact_function_agreement.rs
  path: tests/it/exact_function_agreement.rs
  role: examined
  excerpt: //! FR-021 direct-function corpus from one QSL source and separate Contract
    IR admission.
- id: tests/it/exact_function_generation.rs
  path: tests/it/exact_function_generation.rs
  role: examined
  excerpt: '//! FR-021: function-application oracle generation over admitted CheckedPackage'
- id: tests/it/exact_scalar_agreement.rs
  path: tests/it/exact_scalar_agreement.rs
  role: examined
  excerpt: '//! FR-014-AC-6: generated exact scalar oracles and direct Contract Runtime'
- id: tests/it/exact_scalar_generation.rs
  path: tests/it/exact_scalar_generation.rs
  role: examined
  excerpt: '//! FR-014: exact complete-V1 scalar oracle generation over admitted'
- id: tests/it/inline_integer_equality.rs
  path: tests/it/inline_integer_equality.rs
  role: examined
  excerpt: //! Authentic inline Integer operands for the composite equality generator.
- id: tests/it/integer_division_bounds.rs
  path: tests/it/integer_division_bounds.rs
  role: examined
  excerpt: // Copyright (C) 2026 Agent-IX
- id: tests/it/kani_collection_operation.rs
  path: tests/it/kani_collection_operation.rs
  role: examined
  excerpt: //! Whole-object collection operation contexts from authentic QSL emission
    and IR admission.
- id: tests/it/kani_obligations.rs
  path: tests/it/kani_obligations.rs
  role: examined
  excerpt: //! FR-015 separate bounded Kani obligations, FR-017 Kani execution, and
    Contract IR FR-036
- id: tests/it/kani_obligations_state_clause_replay.rs
  path: tests/it/kani_obligations_state_clause_replay.rs
  role: examined
  excerpt: '//! FR-024 (IR-460): a postcondition state-clause counterexample is put
    before QSL''s'
- id: tests/it/kani_obligations_state_frame.rs
  path: tests/it/kani_obligations_state_frame.rs
  role: examined
  excerpt: '//! FR-015 (IR-412): one `state_clause` postcondition over one integer
    field becomes an'
- id: tests/it/kani_obligations_symbolic_domains.rs
  path: tests/it/kani_obligations_symbolic_domains.rs
  role: examined
  excerpt: //! Source-backed symbolic collection and graph obligation fixtures.
- id: tests/it/main.rs
  path: tests/it/main.rs
  role: examined
  excerpt: '//! IR-237: the integration test crate. Cargo builds a `tests/<name>/main.rs`
    directory as one'
- id: tests/it/model_enum_scalar.rs
  path: tests/it/model_enum_scalar.rs
  role: examined
  excerpt: //! Exact scalar generation over the original QSpec model variant comparison
    fixture.
- id: tests/it/routed_generation.rs
  path: tests/it/routed_generation.rs
  role: examined
  excerpt: '//! FR-022 routed generation: `generate_routed` runs the routed kind''s
    generation arm keyed by the'
- id: tests/it/scalar_literal_operands.rs
  path: tests/it/scalar_literal_operands.rs
  role: examined
  excerpt: // Copyright (C) 2026 Agent-IX
- id: tests/it/scalar_playback.rs
  path: tests/it/scalar_playback.rs
  role: examined
  excerpt: // Copyright (C) 2026 Agent-IX
- id: tests/it/scalar_replay_binding.rs
  path: tests/it/scalar_replay_binding.rs
  role: examined
  excerpt: '//! Driver-owned native scalar controls: no Kani proof or backend falsification
    credit.'
- id: tests/it/scalar_replay_residuals.rs
  path: tests/it/scalar_replay_residuals.rs
  role: examined
  excerpt: //! Native replay authority residuals; no backend proof or fabricated success
    receipt.
- id: tests/it/scalar_source_forms.rs
  path: tests/it/scalar_source_forms.rs
  role: examined
  excerpt: // Copyright (C) 2026 Agent-IX
- id: tests/it/skeleton_spine.rs
  path: tests/it/skeleton_spine.rs
  role: examined
  excerpt: '//! FR-016: the surviving replay adapter and function-contract identity
    behavior.'
- id: tests/it/terminal_map.rs
  path: tests/it/terminal_map.rs
  role: examined
  excerpt: '//! FR-029 and FR-030: the total maps from a Kani run outcome (FR-029)
    and from a Contract IR'
- id: tests/it/v2_clause_integer_literals.rs
  path: tests/it/v2_clause_integer_literals.rs
  role: examined
  excerpt: //! Public generation controls for the general V2 clause integer-leaf policy.
- id: tests/it/v2_integer_comparison_admission.rs
  path: tests/it/v2_integer_comparison_admission.rs
  role: examined
  excerpt: //! Admitted source-to-comparison controls for the public V2 numeric admission.
- id: tests/it/v2_llvm_coverage.rs
  path: tests/it/v2_llvm_coverage.rs
  role: examined
  excerpt: //! V2 clause population and LLVM probe observations from admitted checked
    packages.
- id: tests/it/v2_numeric_strategy.rs
  path: tests/it/v2_numeric_strategy.rs
  role: examined
  excerpt: //! Public selected-claim V2 numeric source, map, runner, and no-artifact
    refusals.
- id: tests/it/v2_point_renderer.rs
  path: tests/it/v2_point_renderer.rs
  role: examined
  excerpt: //! Authored point-qualified V2 clause identities and exact generated verdict
    points.
- id: tests/it/v2_selected_claim_entry.rs
  path: tests/it/v2_selected_claim_entry.rs
  role: examined
  excerpt: //! Public V2 strategy selection of an admitted state-clause claim.
- id: tests/model_enum_support/agreement_cases.rs
  path: tests/model_enum_support/agreement_cases.rs
  role: examined
  excerpt: //! Generated functions execute against original selected model member
    identities.
- id: tests/model_enum_support/fixture.rs
  path: tests/model_enum_support/fixture.rs
  role: examined
  excerpt: //! Genuine model enum values reconstructed from native-admitted equality
    witnesses.
- id: tests/state_frame_support/native_twin.rs
  path: tests/state_frame_support/native_twin.rs
  role: examined
  excerpt: '//! The QSL twin of the state-frame fixture: the `test/bank` domain package,
    the native unit'
```

## Targeted review (round 8)

Targeted frozen review of 3e0b07eae29610a1f8c3922640e9971157af586e...81b71455b65af47c0122e289ca92e3486bb771ab: all twelve changed files (+164/-38) inspected. No spec or requirement changes. No Cargo, aggregate run, branch/source edit, push or merge by reviewer. Original findings and all prior dispositions remain byte-for-byte prefixes of the updated artifact.

The production writer correction fixes a real regression missed by the earlier review. Inspected the locked IR 43ef764 owner contract: graph() preserves node-held operand slots and operand_rows; resolved_graph() is the checked index-form inspection view. Only the two production complete-package encoding call sites use graph(); semantic node inspection continues using resolved_graph(). Sidecars retain document-held identities, and complete-byte owner/context guards are unchanged. The four actual own/imported/nested owner tests and mixed-equality test distinguish this bug from a test-only adaptation. Foreign equal slot bytes are interpreted only through their holder rows; no raw-slot global lookup is introduced.

Fixture review checks that original producer bytes are admitted unchanged before intentional mutation. The common missing-row helper never rewrites existing rows, slots or document indices. Name-only cohort pruning asserts empty rows, keeps order and filters row/node pairs together. The opaque test uses public holder-aware document inspection only on a detached mutation draft, then authors rows by deterministic object-field and array traversal; foreign IDs are left alone. The forged-key control now retains original node order and asserts the intended exact package-locus refusal instead of triggering malformed indexing. Supported/unsupported sibling premises are explicitly asserted.

Compiler-artifact selection verifies active rustc metadata instead of timestamps and still compiles/runs the actual generated interval guard. Test-only emit budgets are finite checked arithmetic (8 KiB per record plus 1 MiB), applied to both unchanged source compilers. They are not runtime defaults, CG limit changes, copied content allowances or a replacement tracking gate. The 50,000-record 512 KiB generation/runtime stack and source split/exact-byte boundaries and 65,536-record work-limit assertions remain present.

Inspected original failure log /tmp/IR-1401-final-docs-tests-3e0b07eae.log and every individual log listed in /tmp/CG757-round8-report.json. The real cvc5 environmental case passes with the lead-provided official binary; no implementation change or fake solver is credited. Prior full gate failed 24 cases; focused receipts qualify those cases only, not a completed aggregate gate. Scoped lint receipt is outstanding and not inferred from an earlier head.

Targeted requirement/test bindings for FR-018-AC-25/28/30/33/42 and FR-021-AC-17/35/36 inspected. Writer-byte mismatch was a real implementation gap, now corrected and tracked as code-review FND-005. Existing binding tests remain behavioral, not weakened to accept mismatch. No new gap remains in the changed scope after exact receipts; original genuine scalar qualification is unchanged. No spec lens rerun is applicable because no spec changed. Historical repository-wide untagged criteria remain, so this subset result is not a repository-wide PASS. Plan completion: not assessed.

## Latest verdict (round 8)

**PASS (targeted changed scope)** — all newly recorded gate defects fixed; no open review findings. Scoped lint and aggregate continuation remain outstanding.

## Targeted examined scope (round 8)

```yaml
scope:
- id: src/kani/generate/negotiate.rs
  path: src/kani/generate/negotiate.rs
  role: examined
  excerpt: //! Separate bounded Kani obligations with per-item backend negotiation
    (FR-015).
- id: src/kani/generate/role_parity_tests.rs
  path: src/kani/generate/role_parity_tests.rs
  role: examined
  excerpt: //! Crate-internal parity checks for state-frame role negotiation.
- id: src/oracle/boolean_v1.rs
  path: src/oracle/boolean_v1.rs
  role: examined
  excerpt: //! Deterministic, fail-closed lowering for Boolean clauses over Boolean
    and integer values.
- id: src/oracle/equality/mod.rs
  path: src/oracle/equality/mod.rs
  role: examined
  excerpt: //! Exact composite/structural equality oracle generation from one admitted
- id: src/oracle/function/owner_path.rs
  path: src/oracle/function/owner_path.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
- id: src/oracle/scalar/lower/opaque_metadata_tests.rs
  path: src/oracle/scalar/lower/opaque_metadata_tests.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
- id: src/replay/state_clause.rs
  path: src/replay/state_clause.rs
  role: examined
  excerpt: '//! Native replay of a postcondition state-clause counterexample: a Kani
    falsification of an'
- id: src/routed/contract_package_tests.rs
  path: src/routed/contract_package_tests.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
- id: tests/checked_package_support/rekey.rs
  path: tests/checked_package_support/rekey.rs
  role: examined
  excerpt: //! Reader admission for fixtures whose producers assign structural node
    IDs.
- id: tests/clause_admission_support/mod.rs
  path: tests/clause_admission_support/mod.rs
  role: examined
  excerpt: //! Admission-only QSL fixtures for request-level clause callers.
- id: tests/composite_equality_support/package.rs
  path: tests/composite_equality_support/package.rs
  role: examined
  excerpt: //! Admitted CheckedPackage V2 fixtures for composite/structural equality
- id: tests/exact_scalar_support/package.rs
  path: tests/exact_scalar_support/package.rs
  role: examined
  excerpt: //! Admitted CheckedPackage V2 fixtures for exact scalar generation.
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
- id: FR-018-AC-28
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: With a caller-selected type-resolution work limit of at least 100,000 and
    `cg.equality.source_bytes` raised to the measured flat source size, an admitted
    acyclic type chain of 100,000 entries reconstructs and generates an oracle with
    the same result as an equivalent shallow type on a 512 KiB stack; with the same
    raised source-byte limit, the default type-resolution work limit refuses it at
    entry 65,537, and a selected work limit one below its entry count refuses it at
    the first denied entry. No depth setting or fixed depth refusal participates in
    these runs. PLANNED (IR-511).
- id: FR-018-AC-30
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The 100,000-entry acyclic closure renders as flat source without a type-depth-sized
    constructor expression; under the 1,048,576-byte default it returns `SourceTooLarge`
    naming that limit when the source exceeds it, and under a caller limit equal to
    its measured source bytes it emits a complete, compilable oracle within the publication
    artifact and bundle byte limits. One byte less refuses before that byte is appended
    and publishes no partial artifact. PLANNED (IR-511).
- id: FR-018-AC-33
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A genuine root-facade compilation of authentic retained source, original
    dependencies and admitted request limits has equal compiled/context/original package
    identities and exact full recompiled/original CheckedPackageV2 equality before
    its context is exposed. An explicitly supplied different context package refuses
    generation and emitted environment construction before classification or lookup,
    including primitive-only requests, publishes no prepared value, and cannot evaluate
    with a completed Boolean or meter charge.
- id: FR-018-AC-42
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Generated oracle invocations under Some or valid context-free None preserve
    private package/item/descriptor/index binding, both preparation-time owner type
    checks, retained semantic-refusal handling, same-index check_equality and evaluate.
    Private wrong-binding, present-context mismatch, missing nominal context and environment-only
    faults refuse without charge. The native OriginalCompositeEqContext route retains
    every mandatory authentic source/lock/package/whole-IR comparison and cannot use
    None to bypass them. No scalar proof manifest/body acquires replay compilation.
- id: FR-018-AC-25
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Reuse the Contract IR lowering counter for reconstruction, charge only
    record declarations while skipping option and sequence edges, or recurse through
    65,537 acyclic entries before checking the budget; a large item then completes
    or overflows the stack instead of yielding the named per-item refusal.
- id: FR-018-AC-28
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Keep a fixed 65,536-entry ceiling even after the caller raises the work
    limit, or use recursive Rust calls for each nested type; the 100,000-entry case
    refuses or exhausts the 512 KiB stack.
- id: FR-018-AC-30
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Retain the fixed source ceiling, emit one nested constructor per type level,
    or measure bytes only after publishing; the 100,000-entry case refuses despite
    a sufficient caller byte limit, overflows a compiler stack, or leaks partial output.
- id: FR-018-AC-33
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Compare the context against lowered ContractPackage identity instead of
    original source package identity, skip full-package/source-lock authentication,
    or expose the retained context before those checks.
- id: FR-018-AC-42
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Drop private binding/context checks for None, allow a nominal recipe without
    context, omit check_type, accept a caller index, or make native authentication
    optional.
- id: FR-021-AC-17
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Each location record uses the request function binding and function-body
    ordering for its origin; the runtime cross-check uses the FR-485 session's location
    derived from the declaring checked graph's root call site, for own and imported
    callables.
- id: FR-021-AC-35
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: For a concrete FR-485 corpus request, the caller supplies all seven acquisition
    inputs and the separately admitted Contract IR package. CG calls `qsl_replay::acquire_checked_closure`
    once for the generated run and receives `CheckedClosureCompilation` with emitted
    package, admitted context and retained `CheckedClosureHandle`; it does not reconstruct
    a QSL checked closure from the V2 graph or emitted source. Acquisition failure
    remains `AdmittedRefusal` and yields no handle.
- id: FR-021-AC-36
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'CG selects the own or imported declaring package explicitly by its admitted
    structural library name and compares its complete canonical V2 bytes, including
    the typed term sidecar, with the bytes retained by the same QSL acquisition. Imported
    bytes come from `CheckedClosureCompilation::owner_package_bytes`; own-unit bytes
    come from `package()`. CG resolves the function identity in its owning V2 frame
    through `ContractSourceKeyV1::from_v2_request(source, request)` and mechanically
    represents that structural key as the Exact `NodeKey`; no rehash, cross-frame
    index comparison, fabricated identity '
- id: FR-021-AC-17
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Take function origin from request ordinal or emitted source instead of
    the binding, allowing claim and source to drift together.
```

## Final exact evidence receipt (round 8)

Final exact 50,000-record receipt /tmp/CG757-round8-deep-acyclic.log now passes: 1 passed, 0 failed, 523 filtered, 368.17 seconds; runner /tmp/CG757-round8-results.tsv records exit 0. This runs the unchanged deep-generation and independently compiled native small-stack assertions; the 65,536-record work-limit exact log also passes. All 24 listed focused failure cases now have successful exact receipts. Scoped lint is still pending, and no aggregate gate clearance is claimed.
