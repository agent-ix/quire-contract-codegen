---
id: SR-17940486709564847047573
title: "IR-1401 PR757 spec-review-ears-conformance"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen; IR-1401; src/oracle/scalar/lower.rs; tests/it/scalar_source_forms.rs; tests/it/main.rs; spec/oracle/functional/FR-014-exact-scalar-oracles.md; spec/oracle/matrix/TC-024-exact-scalar-oracles.md"
review_set: subset
---

## Summary

The diff adds acceptance-criterion table rows and a test procedure, not a new or changed EARS requirement statement. Engine validation completed; existing FR-014 line 313 noncanonical-trigger warning is outside the diff. No changed-statement EARS finding.

Frozen candidate 8512ca2536342d46924c2c63868f808ce8719ab4 against origin/main cd184a37. No dependency, CI, public API or lockfile changes. No applicable AssuranceProfile exists under spec/. Full gates and proof execution were excluded by the lead; existing focused execution logs were inspected, not rerun after the reported cache deletion. No cache or worktree was removed by this reviewer.

## Verdict

**PASS** — No new defect found in this method’s changed scope.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Frozen candidate 8512ca2536342d46924c2c63868f808ce8719ab4 against origin/main cd184a37. No dependency, CI, public API or lockfile changes. No applicable AssuranceProfile exists under spec/. Full gates and proof execution were excluded by the lead; existing focused execution logs were inspected, not rerun after the reported cache deletion. No cache or worktree was removed by this reviewer.

Reviewed all changed source/tests/spec files. Source uses one shared shape and operand predicate in both lowerer validation points. Existing caller-declared descriptor policy is preserved. No duplicated catalog, arbitrary-family admission or disabled gate was introduced.

Evidence inspected: /tmp/ir1401-final-focused.log (4 pass, 1 fail); /tmp/ir1401-red.log (IEEE FormMismatch and carried reader failure); /tmp/ir1401-clippy.log and /tmp/ir1401-fmt.log (pass); independently computed /tmp/ir1401-review-matrix.json and /tmp/ir1401-review-spec.log. No Kani or backend proof was executed.

## Examined units

- FR-014-AC-66 — role: examined — spec/oracle/functional/FR-014-exact-scalar-oracles.md — excerpt: For a strict-reader-admitted `quire.op.quantity.convert`, CG accepts a referenced `scalar_type`/`carried_quantity` source operand permitted by the owner operation contract, including the genuine QSL-emitted TC-187 U01 form, and derives and generates an `ir_confirmed` oracle. Source carrier, unit and rounding and the admitted exact or carried target identity and domain remain intact; execution uses the existing owner conversion contract without replacing a carried source with an unbounded or exact carrier.
- FR-014-AC-67 — role: examined — spec/oracle/functional/FR-014-exact-scalar-oracles.md — excerpt: For strict-reader-admitted `quire.op.ieee.numeric_equal`, `quire.op.ieee.total_order` and `quire.op.ieee.bit_identical` nodes, CG accepts the genuine QSL-emitted `expression`/`call` semantic form with the catalogued `call` body operator, including numeric equality emitted from `a = b` and each `quire::value::ieee::*` intrinsic. Existing reader-admitted binary-semantic-form fixtures with that same catalogued call operator remain supported. Derivation and generation retain operand order, float width, Boolean result, operation identity, laws and mode, and dispatch the selected owner IEEE operatio
- FR-014-AC-68 — role: examined — spec/oracle/functional/FR-014-exact-scalar-oracles.md — excerpt: Acceptance of these carried operands and IEEE call forms preserves the existing strict-reader and CG boundaries. Unadmitted operation/body pairings and reader-invalid arity, types, identity, laws or mode remain reader refusals. Admitted shapes, arity, operand/result types, unitless quantity literals and unsupported semantic forms that CG previously refused retain typed refusal, no oracle and supported-sibling isolation. A same-lowering-shape descriptor identity, law or mode mismatch retains the existing FR-014-AC-12 and FR-038-AC-3 behavior: where generation was permitted, its operation remain
- FR-014-AC-69 — role: examined — spec/oracle/functional/FR-014-exact-scalar-oracles.md — excerpt: Qualification of AC-66 and AC-67 emits genuine QSL source through the public facade, passes its unchanged emitted bytes through the strict Contract IR reader, derives descriptors and generates, compiles and executes the resulting native scalar oracles. Tests assert completed conversion values and units and IEEE true and false values, including signed-zero and NaN cases that distinguish numeric equality, total order and bit identity according to the owner contract. Generated and direct owner executions agree in complete, refused or incomplete outcome, ordered admitted charges and consumed count

TC-024 added genuine-source qualification procedure examined in full; prior AC-12 / FR-038-AC-3 and AC-62–65 were context only. No integrity/dependency/object lens applies: no new object, edge or structural artifact schema was added. Criterion-strength skipped because not selected; no Jev client invoked.

## Disposition review scope (round 2)

Reviewed frozen 2afe6beb058b2898b3a372e0e47ba4af985374b1, including actual shared native_quantity_checks harness, checked carried metadata assertions and the explicitly component-only execution test. Inspected /tmp/ir1401-component.log (actual owner admission/conversion harness compiles and executes successfully) and /tmp/ir1401-fix1-ieee.log (genuine IEEE native recheck passes). The component run substitutes only the owner conversion call in the shared harness; it is not generated-oracle or genuine strict-reader qualification. No new production/dependency change, source edit, cache cleanup, full gate or proof execution by reviewer.

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

## Latest verdict (round 3)

**PASS** — no new specification defect found in expanded method scope.

## Focused disposition review (round 4)

Frozen e1f8339098d4ee714849f2907e5ea21a09431932 removes the entire bespoke root/scratch Cargo.lock git-source scan, with no replacement gate. Ordinary root lock seeding, offline metadata resolution and locked native execution remain. The production error/precedence policy is unchanged. Inspected /tmp/ir1401-round3-preparation.log: both actual preparation tests pass; /tmp/ir1401-round3-clippy.log: touched library Clippy passes. No new Cargo run, source edit, cache cleanup or worktree operation by reviewer.

No new finding or changed scope in this method. Existing spec verdicts remain PASS; gap-analysis remains FAIL because the owner catalog blocks actual genuine quantity/IEEE qualification before CG. No component result is counted as genuine acceptance.

## Final frozen review (round 5)

Frozen head da4c8a9d7350503bae89ec8ba16083ee719403dc; full origin/main...HEAD diff, including consumer API migrations, inspected. Historical finding text and disposition history preserved. No applicable AssuranceProfile exists (spec/assurance contains architecture decisions, not profiles). Existing execution evidence was inspected; no Cargo, aggregate gate, workspace/stress build, Kani, branch edit, push or merge was performed by this reviewer. Focused report /tmp/CG757-focused-report.json and every listed log inspected; logs show exactly selected tests executed, not zero-test success. Broad Rust gates and cargo-deny were not rerun, per brief. Format and focused library/integration Clippy evidence passed. Plan completion: not assessed.

Changed FR-014/FR-018 and TC-024/TC-029 were examined against the base checklist, EARS and failure-domain lenses. Integer inference explicitly states finite operands, actual bound provenance, zero-only divisor and retained explicit/narrowing/ambiguous/unreadable refusal. Lazy-meter obligations name actor, observable order, skip/reached counts, typed stop propagation, denial controls and QSL qualification boundary. Preparation defines native allocation/capacity versus semantic refusal, left-before-right precedence and no partial prepared capability. Original package/source authentication is retained. No new domain object, relationship edge or artifact structural schema requires integrity/object/dependency lenses; criterion-strength/Jev was not selected.

Independent quire spec validation succeeds with five existing grammar advisories; changed requirement statements are concrete and classifiable. Global criterion gaps and old Verification cells are baseline and not represented as newly introduced defects.
## Latest verdict (round 5)

**PASS** — no new defects; prior dispositions remain effective.

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
```

## Targeted review (round 10)

Targeted frozen review of 9ce68c8dceedc7750e6169bc3d2a623be6a41bfa...9b4ce31758ddfee5c345abd5c42848867166ce5c, all nine files examined. Committed code/gap artifacts compare byte-for-byte with the reviewer's round9 files. All original findings and disposition history are preserved as exact prefixes in these updates. No Cargo, broad review/build, source edit, branch change, push or merge by reviewer. No applicable AssuranceProfile exists; no object/schema/dependency edge change calls for another lens.

FND-013: the four independent algorithms are removed. One tests/checked_package_support/draft_rows.rs::author_row owns explicit detached-row authoring. Iterative LIFO traversal pushes object values/array items in reverse so processing retains canonical field/element order; local document indices deduplicate in encounter order and become fresh operand slots. A nonempty input row or existing local slot fails explicitly. Foreign ID objects are skipped intact, preserving their document frame. This is one shared source compiled into test modules, not copied source or a production protocol mirror. Search confirms only the shared implementation remains.

Call boundaries retain the qualification distinction: the case control authors only its newly appended equality row; the arithmetic control authors only collected intentional modified rows after public holder-aware inspection. The carried producer authors its own deliberate fixture nodes. The opaque control first admits original unchanged published bytes, then authors a detached inspection draft. The helper accepts one selected node and fresh row; it neither reads/replaces whole packages nor normalizes reader inputs. Existing source maps and imported graph nodes are untouched.

Gap FND-002: FR-018-AC-8, its adverse case and TC-029 now require the actual public native Some("ambiguous-name") cause and prohibit erasing or synthesizing it. The AmbiguousDeclaration code, owning Package refusal, absent successful AdmittedContext, no public CG generation and private duplicate-member separation remain explicit. Inspected current locked QSL public-owner evidence in round9 and unchanged exact duplicate-field test. No source/test error projection drops the native cause.

Inspected /tmp/CG757-round10-report.json and every listed log. All seven exact tests actually run one named case and pass: carried/equality/bounded draft producers, incoherent IllTyped, malformed aggregate reader negative, opaque isolation and authentic native duplicate cause. Scoped Clippy reports success; format success and its empty log match the runner receipt. Independent quire validation of the two changed specs passes (/tmp/cg757-round10-review-spec.log). No full forty-seven-case or aggregate rerun is claimed. Docs remain unrun in the lead's continuation.

Targeted EARS disposition: edits change an acceptance criterion, its adverse-case example and test procedure; no FR Description/Behavior/Constraints shall-statement changes. Existing actor/trigger obligations retain their prior EARS assessment. Changed responses are concrete (owning Package, AmbiguousDeclaration, exact native cause, no admitted context); no newly introduced grammar finding.

## Latest verdict (round 10)

**PASS** — no open findings in this method. Earlier fixed dispositions remain effective; docs/aggregate continuation is not accepted by this review.

## Targeted examined scope (round 10)

```yaml
scope:
- id: spec/oracle/functional/FR-018-composite-equality-oracles.md
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '---'
- id: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: '---'
- id: src/oracle/scalar/lower/opaque_metadata_tests.rs
  path: src/oracle/scalar/lower/opaque_metadata_tests.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
- id: tests/checked_package_support/draft_rows.rs
  path: tests/checked_package_support/draft_rows.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
- id: tests/exact_scalar_support/package.rs
  path: tests/exact_scalar_support/package.rs
  role: examined
  excerpt: //! Admitted CheckedPackage V2 fixtures for exact scalar generation.
- id: tests/it/composite_equality_generation.rs
  path: tests/it/composite_equality_generation.rs
  role: examined
  excerpt: '//! FR-018: composite/structural equality oracle generation over admitted'
- id: tests/it/v2_clause_integer_literals.rs
  path: tests/it/v2_clause_integer_literals.rs
  role: examined
  excerpt: //! Public generation controls for the general V2 clause integer-leaf policy.
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Test |
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: '`cause() == Some("ambiguous-name")` for this Assembly refusal, retaining
    the'
```
