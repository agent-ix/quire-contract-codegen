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
Generate with each selected clause's point from its accepted QSL binding; include distinct
precondition and postcondition anchors. Instrument verdict construction to observe the precondition
point on a rejected case and the postcondition point on failed and passed cases. Drive a
non-completed precondition case and a non-completed postcondition case and observe the point retained
beside each oracle's cause. Compare each runtime string with the binding's variant, UTF-8 anchor
name and exact byte-length rendering. For two separate requests, select the same precondition
checked clause node with two authentic `claim` occurrences bound to its same `Pre` operation point,
keeping the postcondition selection fixed; instrument the accessor to prove each request uses its
own selected occurrence, and compare each generated precondition verdict point with that accepted
point. Repeat each request with the same
admitted package, full `(checked clause id, authentic claim occurrence)` selections and campaign
policy, and compare its bytes with that request's first artifact.
Independently for either selection, supply a wrong-role, wrong-ordinal, and other-node occurrence
while another authentic occurrence of the node has a point; require no sibling substitution. Then
select an authentic claim lacking its accepted point beside an authentic sibling of that node with a
point; require `MissingExecutionPoint` without sibling substitution. Also combine an unknown clause
id and wrong occurrence, and a wrong-kind clause id and wrong occurrence, to check FR-002-AC-10's
diagnostic precedence. Apply a renderer mutant
that changes only the generated harness or verdict point after binding selection. Inspect every
refusal and artifact output and require the diagnostic to name the affected selection.
For each authentic QSL driver-bound FR-331 selection, inspect the generated header, runtime
identities, source map, and symbols against the FR-011 `qocc2_`/`qpkg2_` grammar. Change only the
occurrence node, role, or ordinal, then one package-id field, using separately admitted matching
FR-331 request/package pairs; require the corresponding rendered identity to change. Form an
out-of-driver request with a missing or non-claim occurrence and one whose selected claim
references a model element absent from the admitted V2 relation. Pair a request owner with a
different admitted package. Check CG's typed refusals and absence of artifacts; verify that QSL
keeps `UnknownItem`, `Unbound`, and `Fault` export refusals outside CG.
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
The V2 point controls require accepted QSL binding-point preservation for both selections, including
rejected, failed, passed and non-completed cases (FR-002-AC-19). An absent selected binding point
returns `HarnessErrorCode::MissingExecutionPoint`; a renderer mutant returns
`HarnessErrorCode::ExecutionPointMismatch` before publication. Each names the affected selection,
produces no artifact and derives no point from clause kind (AC-20). The two authentic claim
occurrences on one precondition node each retain their own selection and the same accepted `Pre`
point. Each wrong occurrence returns `HarnessErrorCode::MissingClaimOccurrence`, names its selection, and produces no
artifact despite a valid sibling point. An authentic claim lacking a point returns
`HarnessErrorCode::MissingExecutionPoint` despite its pointed authentic sibling. Unknown or wrong-kind
clause ids retain FR-002-AC-10's diagnostic ahead of occurrence checks. Repeating either full
selection with the same package and policy is byte-identical (FR-002-AC-12). These controls are
planned.
The identity controls require full canonical rendering and a changed rendered identity when one
typed field changes (FR-002-AC-21). An out-of-driver missing or non-claim occurrence returns
`MissingClaimOccurrence`, an absent referenced model element returns `UnboundModelElement`, and a
cross-package request returns `OwnerIdentityMismatch`; each produces no artifact and QSL export
refusals stay with the driver (FR-002-AC-22). These controls are planned.
