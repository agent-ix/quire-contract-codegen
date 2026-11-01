---
id: SR-202610310142
title: IR-709 CG base review
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen; IR-709; spec/oracle/functional/FR-021-function-application-oracles.md,
  spec/oracle/functional/FR-042-generated-checked-invariant-causes.md, spec/oracle/matrix/TC-031-function-application-oracles.md,
  spec/oracle/matrix/TC-057-generated-checked-invariant-causes.md, spec/oracle/matrix/tests.md,
  src/lib.rs, src/oracle/boolean_v1.rs, src/oracle/function/mod.rs, src/oracle/function/owner_path.rs,
  src/oracle/function/owner_render.rs, tests/composite_equality_support/cases/record.rs,
  tests/exact_function_support/agreement_cases.rs, tests/exact_function_support/imported_cases.rs,
  tests/exact_function_support/invariant_cases.rs, tests/exact_function_support/invariant_inspection.rs,
  tests/it/exact_function_agreement.rs
review_set: subset
---

## Summary

Reviewed frozen head 92eeceb2258bbabb13a89f4ae1e90874f884b3bd against 7f0c1d7d. Ticket: IR-709.

## Verdict

**CONDITIONAL** — Restore the adversarial examples described below.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The three edited adversarial rows repeat the positive acceptance criteria and even add a third Verification cell beneath the two-column forbidden-behavior table. Restore negative counterexamples for wrong-result faults/location loss, synthetic or resource-misclassified environment rejection, and substituted result types or relabelled stopped outcomes. | spec/oracle/functional/FR-021-function-application-oracles.md:543 |

## Coverage

Plan completion: not assessed. No applicable AssuranceProfile exists under spec/. No Cargo, rustc, Kani or proof execution was performed, as directed by the review brief. Author logs are reported evidence only, not independent gate execution. No production edits or foreign worktrees were used.

## Examined scope

```yaml
scope:
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
- id: FR-021-AC-37
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'A mismatch between the acquired unit''s structural name or complete canonical
    V2 bytes and the CG request''s admitted unit refuses the item as `ExactFunctionRefusal::OwnerIdentityMismatch`
    before registration, Begin, body invocation or application charge. The same refusal
    follows a mismatch between `CallableSelection::owner_package_name()` and the selected
    function''s declaring admitted library name, or between the selection''s read-only
    checker-issued structural key and that function''s source-resolved structural
    identity. The selection comparisons run in a private post-lookup guard; TC-031
    tests '
- id: FR-021-AC-38
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: With a genuine selected callable declaring Boolean result, mutate only
    the test copy of the emitted body to return Completed(Value::Integer(...)); the
    unchanged shipped result guard returns Done(Evaluation) with GeneratedUnexpectedOutcome
    at the actual result occurrence and the admitted charge prefix. The unmutated
    registered body completes with its admitted Boolean. No failure injection, caller-chosen
    result type or production hook is used.
- id: FR-021-AC-39
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Native preparation consumes each selected owner record/tuple/union declaration
    once, preserving original keys, ordered member types, required/optional presence
    and sealed union identities. A test-only copy resubmits one unchanged authentic
    declaration to the same QSV constructor; its genuine duplicate-key Refused(InvalidDeclaration)
    yields GeneratedEnvironmentRejected at the declaration occurrence before body
    operations/charges, retaining the original typed diagnostic. No synthetic declaration,
    empty fallback, injected error or production hook is used. Actual constructor
    WorkUnits with tripped
- id: FR-021-AC-40
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'The generated guard uses the same checker-issued selection.signature().result()
    and actual owner QSV environment, never a replacement result type. Undefined,
    Refused and Incomplete retain their variants, payloads, locations and admitted
    charge prefixes. Err(EnvironmentFailure) remains BodyFailure::QsvEnvironment /
    CallFailure::Body, never false, Completed or a genuine invariant fault. Boolean/Integer
    result membership allocates no storage: Allocation/Capacity mapping has inspection
    evidence with no execution credit. The first shipped allocating-result membership
    family activates the actual-req'
- id: FR-021-AC-42
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: The emitted function-oracle crate, compiled with `cfg(kani)`, declares
    every `body_{symbol}` module `pub(crate)` with a `pub(crate)` `Factory` and omits
    every `oracle_{symbol}` host entry, and compiled without `cfg(kani)` is unchanged
    (all `oracle_{symbol}` entries present and behaving as AC-35 through AC-37 require).
    A source-level check of the emitted text proves the `#[cfg(not(kani))]` gate on
    each host entry and the module visibility; the Kani obligation and its explicit
    exclusions are as stated in Behavior, and no test, doc comment or matrix row credits
    the host glue to a Kani proof.
- id: FR-042-AC-1
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: CG's native Boolean, equality and function constructors use the exact eight
    CG-owned causes in the table, retaining `IllTypedCause` from `check_type` and
    `check_equality`; downstream replay consumers preserve the typed payload or match
    it explicitly, retain charge/failure behavior, and accept neither the former unit
    constructor nor an invented default.
- id: FR-042-AC-2
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: The `GeneratedUnexpectedOutcome` producer executes a generated body that
    returns `Completed(value)`, compares that value against its selected callable's
    checker-issued result type via QSV `admits`, and reports only `Ok(false)` as this
    cause at the result location. A test-only emitted-source mutation supplies a reachable
    wrong completed value to the unchanged shipped check; `Undefined`, `Refused` and
    `Incomplete` retain their original outcomes and charges, and `Err(EnvironmentFailure)`
    is never a mismatch. For current Boolean/Integer results, Allocation/Capacity
    mapping is inspected with no exe
- id: FR-042-AC-3
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: The `GeneratedEnvironmentRejected` producer uses only real nominal declarations
    supplied by the selected checked-closure owner, once each in unmutated production
    source. A test-only mutation of emitted pre-body assembly submits one unchanged
    owner declaration twice to the shipped QSV constructor; exactly its genuine duplicate-key
    `EnvironmentFailure::Refused(InvalidDeclaration)` yields this cause before body
    execution at the declaration occurrence, with the typed rejection retained in
    diagnostic evidence. Successful construction does not yield it; `Limit`, `Allocation`
    and `Capacity` are never
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Disposition round 1: inspected 7ece8cdd29356f3398e01c20aa2e7fe63c65e627. All three rows now contain forbidden-behavior counterexamples and exactly two cells; positive ACs are unchanged. |

## Disposition round 1 verdict

**PASS** — FND-001 is fixed. The only candidate delta restores FR021 adversarial examples; no new findings. No Cargo/Kani/proof execution.
