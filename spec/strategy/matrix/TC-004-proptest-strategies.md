---
id: TC-004
title: "Verify shaped proptest strategies"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: verifies
---
# TC-004: Verify shaped proptest strategies

## Description

Verify generated harnesses preserve tri-state outcomes and strategy construction/shrinking respects
supported constraints and boundaries.

## Test Procedure

Generate campaigns for range, membership, enum, correlated, state-pinned, boundary, and residual
fixtures. Execute seeded cases and shrinking through the generated campaign runner. Exercise an
accepted-case floor, an explicit-discard ceiling, zero configured cases, all-rejected cases,
expectation mismatches, empty residual exclusions, and correlated populations whose representable
boundary census would otherwise be vacuous.
Pin the successful mixed campaign's RNG and exact accepted/rejected counts. Exercise framework
exhaustion both before any invocation and after invocations, and require the framework reason and
any missed coverage floor to survive together.
Pass an existing report containing a discard into an otherwise successful deterministic campaign;
require the completed-run policy to reject the cumulative discard excess. An aborted search with
the same prior excess must retain both exhaustion and the exceeded-ceiling policy result.

Planned V2 controls (IR-815): from an admitted `CheckedPackageV2`, select one precondition and one
postcondition and generate the harness. Drive seeded cases that make the precondition false, the
postcondition false, and neither, and cases whose precondition or postcondition oracle stops or is
undefined (an integer operation that is undefined), with an instrumented snapshot, precondition
oracle, subject and postcondition oracle that record call order and whether the subject was invoked. Select an unknown clause identity, a clause of the wrong kind, the same clause for
both selections, and a body outside the FR-014-AC-38 shape. Generate twice and compare bytes.
Inspect the generator source for any `BoundPackage`, `ClauseRef`, `quire_contract_model::TypedExpression`
or `oracle::boolean_v1` use and for any body opened outside `core/ir/`.

## Expected Results

Supported constraints are shaped directly, every emitted boundary census contains accepted and
rejected cases, residual rejection is explicit, and no rejected case becomes a pass. Integer and
enum values carry executable expected-domain checks. Boolean campaign constructors bind disposition
to their exact values; the generated runner owns invocation, explicit discard accounting, and final
accepted/rejected/failed/discarded validation. It rejects zero/below-floor campaigns and campaigns
above the requested discard ceiling.
An exhausted search remains a typed `Exhausted` result even when its retained policy result says
that a floor was missed; completed searches still return the corresponding floor failure directly.

The V2 controls expect a case with both oracles completing true to run the subject once and pass
(FR-002-AC-13), a rejected case to leave the subject uninvoked (AC-14), a false postcondition to be
a failed postcondition (AC-15), a non-completed oracle outcome to be a non-completed case that
keeps its cause and is no pass (AC-16), and a precondition that does not complete invokes neither
the subject (AC-17) nor the postcondition oracle (AC-18), snapshot, precondition, invocation and postcondition to run in that order (AC-8), each
wrong selection to yield a typed, located diagnostic naming the selection and no artifact (AC-10),
an unsupported body to yield a structured unsupported diagnostic, no artifact and no filtering
fallback (AC-11), and identical bytes on repeat for the same package, clauses and policy (AC-12).
These controls are planned.
