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
