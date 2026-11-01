---
id: FR-048
title: "Retain state-constraint boundaries in Kani method evidence"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: depends_on
  - target: ix://agent-ix/quire-spec-language/FR-158
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-160
    type: references
---
# FR-048: Retain state-constraint boundaries in Kani method evidence

## Description

When a caller selects a checked state constraint as a Kani search method,
CG SHALL retain the constraint as a method input rather than a proof obligation.
When that method reaches a state outside the constraint, CG SHALL evaluate
that state's safety obligations before stopping expansion from it.

This requirement specifies the constrained-method slice of IR-1157. Every
criterion below is PLANNED: the current request, generator, classifier and
terminal map do not implement this method. A standalone constraint collection
claim remains unsupported without a harness under
[FR-015-AC-128](./FR-015-bounded-kani-obligations.md).
This requirement neither turns that refusal into proof evidence nor changes
unconstrained Kani outcomes.

## Inputs

- The admitted checked V2 package, the selected safety obligation and its exact
  claim occurrence, and the retained source and dependency context.
- A typed optional method selection naming one checked constraint-clause node
  in that same package and its checked state context. The selection is not a
  Boolean expression, display name or caller-provided assertion.
- The finite state and transition domains, ordinary proof ceilings, ordered
  backend options, unwind and caller-owned cancellation handle.
- For classification, the original installed-backend evidence, including
  safety checks, the ordinary non-vacuity cover, a separately identified
  boundary cover, and typed completion/refusal information. A display string
  or successful process exit alone is insufficient.

## Outputs

- For accepted selection, a generated constrained-method harness and a distinct
  typed method execution identity retaining both the ordinary obligation
  identity and the selected constraint node and admitted package binding.
- A typed selection refusal naming an absent, non-constraint, ambiguous,
  wrong-package or incompatible-state-context selection, without a harness.
- A typed constrained result retaining the original backend evidence, selected
  method, ordinary non-vacuity observation and boundary-reachability observation.
  Boundary reachability is `Reached`, `NotReached` or `Unknown`; a cover result
  is not an exact count of distinct boundary states.
- A typed non-proof terminal through CG's public terminal map, or a typed
  refusal if the QSL-owned terminal contract cannot represent the result.

## Behavior

### Selection and identity

When a method is selected, CG SHALL resolve its constraint against the same
admitted package and state context as the selected obligation before generation.
If that selection cannot be authenticated, then CG SHALL refuse it before
backend dispatch, without selecting a sibling clause or inferring a default.

CG SHALL keep the constraint out of the semantic obligation identity, as
[QSL FR-158](ix://agent-ix/quire-spec-language/FR-158) requires.
CG SHALL distinguish constrained execution requests from unconstrained requests
and from requests using another constraint in their method execution identity.
Changing the constraint SHALL invalidate reuse of execution, replay or terminal
evidence from the other method. Equal admitted requests SHALL regenerate the
same method execution identity and artifact bytes.

### Reached-state semantics

The generated method SHALL test safety at every reached state, including an
initial state outside the constraint and every later boundary state.
If a safety obligation fails there, then the generated method SHALL retain that
failure and its state for playback before stopping expansion.
If the constraint evaluates to false at a reached state, then the generated
method SHALL stop successor expansion from that state after safety evaluation.
The generated method SHALL NOT turn that state into a deadlock or add a
terminal stutter transition.

The generated method SHALL NOT use `kani::assume(constraint)` to remove reached
boundary states before safety evaluation. A constraint SHALL NOT substitute
for a finite domain, cardinality bound, unwind or resource ceiling.
The generated method SHALL evaluate the checked constraint through the same
owner semantics as the safety evaluation, without a second CG evaluator.

If the constraint evaluates `Undefined`, then the generated method SHALL retain
an evaluation violation at that state, distinct from a false constraint and
from an unmet cover. If evaluation is refused or incomplete, then CG SHALL
retain the typed refusal or incomplete cause without treating it as false,
true, an unreachable boundary or a proof.

### Covers and classification

CG SHALL identify the boundary cover separately from the ordinary
non-vacuity cover. The boundary cover witnesses that at least one state was
reached where the constraint completed false, after safety evaluation at that
state. Its absence is not an unsatisfiable-method conclusion: a completed
boundary-cover check may establish `NotReached` only within the exact retained
bounds. An absent, interrupted or malformed boundary check establishes `Unknown`.

For this method, the ordinary cover remains mandatory under
[FR-015](./FR-015-bounded-kani-obligations.md). This is a distinct constrained
harness family; its second, explicitly identified cover does not change the
exactly-one ordinary-cover contract of the existing unconstrained families.
A boundary cover that is not reached SHALL NOT by itself produce ordinary
`CoverUnsatisfied` or discard a safety violation.

CG SHALL classify the complete typed check results rather than the order of
printed diagnostics. A safety failure at a boundary has precedence over a
satisfied boundary cover. An actual safety counterexample retains
`Falsified` and its original playback; a missing or undecodable playback remains
the existing non-proof failure under
[FR-017](./FR-017-kani-execution-evidence.md).
No constrained success SHALL be represented as an ordinary unconditional
`KaniRunOutcome::Verified`. Successful safety checking with `Reached` becomes
`ConstraintReached` in the constrained result. With `NotReached`, it becomes
an explicitly method-qualified bounded success; with `Unknown`, it remains
inconclusive. None of those three is an unconditional proof or grants proof
credit. Ordinary non-vacuity failure, cancellation, unwinding failure, timeout,
resource exhaustion and malformed backend evidence retain their own causes.

### Owner mapping

QSL owns state evaluation and the meanings of `Holds`, `Violated`,
`ConstraintReached`, `Undefined`, refusals and incomplete execution. CG owns
one typed conversion from those method observations and FR-017 backend evidence
to the constrained result, and one extension of its public
[FR-029](./FR-029-run-outcome-terminal-record.md) terminal map.
The driver invokes those public converters; it SHALL NOT parse backend text or
invent a second outcome mapping.

| Owner observation and complete backend evidence | CG constrained result | Terminal rule |
| --- | --- | --- |
| Safety violation, including at a boundary, with retained falsifying playback | Falsified with the original method identity and evidence | Existing FR-029 replay pairing; `refuted` only after same-method replay reproduces the violation |
| Constraint evaluation is Undefined | Evaluation violation retaining the undefined cause and original evidence | Existing violation/replay contract when evidence supports it; no success or boundary credit |
| No safety violation, ordinary cover satisfied, boundary reached | ConstraintReached, boundary observation Reached | Typed inconclusive constrained-method result; never proved |
| No safety violation, ordinary cover satisfied, completed boundary check not reached | Method-qualified bounded success, boundary observation NotReached | Typed non-proof method-qualified result; never an unconditional proved terminal |
| Refused or incomplete evaluation, unknown boundary observation or incomplete backend checks | Typed refusal or inconclusive result with the original cause | No proved or refuted terminal inferred from missing information |
| Ordinary cover unsatisfied, cancellation or resource/unwind failure | Original typed non-proof result and cause | Existing non-proof terminal semantics, retaining the method |

These rows specify semantic correspondence, not a cast of QSL verdict names to
Kani outcome variants. A QSL `Holds` alone cannot establish a Kani success, and
a `ConstraintReached` alone cannot fabricate installed-backend evidence.
CG SHALL require agreement of the selected method identity across the backend,
QSL observation and replay before conversion. If they disagree, then CG SHALL
return a typed binding refusal with no terminal or proof credit.

QSL owns the terminal value/cause vocabulary and serialization. Until its
public API can retain the constrained method and represent these non-proof
results, CG SHALL refuse terminal conversion with a typed unsupported result;
CG SHALL NOT serialize a private terminal variant, drop the method or map the
result to `proved` as a fallback. This requirement does not claim that the
current QSL terminal API already exposes those representations.

## Acceptance Criteria

All controls use admitted source-backed V2 inputs and retained producer evidence.
No implementation or installed-backend qualification is claimed by this spec.

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-048-AC-1 | A selected constraint resolves in the obligation's admitted package and state context; absent, ambiguous, wrong-kind, wrong-package and wrong-context selections each yield a distinct typed refusal before harness generation or backend execution. | Test |
| FR-048-AC-2 | The same safety obligation run without a constraint, with constraint A, and with constraint B has three distinct method execution identities and retains the same semantic obligation identity. Cross-method evidence or replay substitution refuses before conversion; equal requests regenerate identically. | Test |
| FR-048-AC-3 | An initial boundary state and a later boundary state each undergo safety evaluation. A seeded safety violation at either boundary is falsified with in-domain playback; replacing the boundary branch with an early assumption would fail these controls. Neither boundary expands, stutters or counts as deadlock. | Test |
| FR-048-AC-4 | A healthy bounded method reaching a boundary satisfies its separate boundary cover and ordinary cover, yields ConstraintReached with Reached observation, and produces no ordinary Verified or proved terminal. A boundary cover records reachability rather than an invented distinct-state count. | Test |
| FR-048-AC-5 | A healthy method reaching no boundary retains a completed NotReached observation and a method-qualified bounded success without unconditional proof credit. An interrupted or missing boundary check retains Unknown and an inconclusive result. An unmet boundary cover alone cannot classify the ordinary cover as unsatisfied. | Test |
| FR-048-AC-6 | A constraint that is Undefined at a reached state retains the evaluation violation rather than classifying that state as a boundary. Typed Refused and Incomplete evaluations retain their original causes and cannot become a Boolean assumption or proof. | Test |
| FR-048-AC-7 | Removing ordinary non-vacuity evidence, exhausting unwind or resources, cancelling, or supplying malformed backend checks preserves the corresponding non-proof outcome even if the boundary cover was satisfied. Reordering check output does not change classification. | Test |
| FR-048-AC-8 | The public CG conversion implements every mapping row above using typed evidence and exact method binding. A boundary safety violation follows same-method replay and FR-029 precedence; replay disagreement, refusal, absent playback and identity mismatch never produce refuted or proved by default. | Test |
| FR-048-AC-9 | A public QSL terminal representation preserves the constraint method and non-proof status; where that representation is unavailable, conversion returns a typed unsupported refusal. No private wire variant, missing method or proved fallback is emitted. | Test |
| FR-048-AC-10 | Installed-backend healthy-boundary, no-boundary and seeded-boundary-violation controls exercise the generated method and retain their exact typed check evidence. Independent QSL evaluation agrees on the bounded reached states and boundary safety observations. Neither generation-only inspection, saved verdict nor timeout substitutes for this evidence. | Test |
| FR-048-AC-11 | A standalone collection constraint remains unsupported under FR-015-AC-128 with no harness or proof credit. A method constraint never rescues an otherwise unbounded or unsupported state domain. | Test |

## Dependencies

- [FR-015](./FR-015-bounded-kani-obligations.md) owns bounded symbolic domains,
  ordinary covers and the standalone constraint refusal.
- [FR-017](./FR-017-kani-execution-evidence.md) owns original run evidence;
  [FR-029](./FR-029-run-outcome-terminal-record.md) owns replay pairing and the
  public Kani terminal conversion.
- [QSL FR-158](ix://agent-ix/quire-spec-language/FR-158) owns reached boundary
  semantics, constraint selection and semantic obligation identity.
- [QSL FR-160](ix://agent-ix/quire-spec-language/FR-160) owns the QSL terminal
  semantics and method record; a representable public constrained-method
  result is an implementation prerequisite, not an assumed existing API.
