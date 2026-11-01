---
id: SR-202610310154
title: "gap-analysis review of constrained method implementation"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-1157 PR700 changed paths"
review_set: subset
---

## Summary

Ticket IR-1157, CG PR #700 frozen head bcc6436780d39ee712717e722518d0aa202adbd1 against origin/main 78e725a8. One independent review; no source edits or Cargo execution. FR-048 AC1..11, TC-062 and changed AD-004 boundaries were checked. No applicable AssuranceProfile exists. The generation entry is negotiate-only, renderer remains private/shared, identities sit below runner, and terminal conversion above runner. QSL UnsupportedTerminalRepresentation fallback is allowed and emits no forged terminal. Plan completion: not assessed. Computed quoin matrix: all 11 criteria tagged; AC3 only by the explicitly ignored installed lane. No registered matrix run evidence; retained logs provide separate qualification evidence. The real installed four-control lane passed, not the discarded memory prototypes. The original-state-clause independent oracle gap remains.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The owner oracle appends EvalSafety/EvalConstraint pure functions by rewriting self field references to parameters and executes those mirror functions with run_with_requirements. It never independently evaluates the authored Safe/Limit state clauses at admitted object observations, so it cannot verify their state-clause binding/observation semantics and violates the no-mirrors constraint. Use the public QSL evaluate_clause facade on the actual authored clauses and authentic snapshots in the exhaustive-domain and installed controls. | tests/it/constrained_method/support.rs:64-87,223-264 |

## Verdict

REQUEST_CHANGES

## Coverage

Plan completion: not assessed. Parent owns final-head gates after fixes. Inspected qualification logs under qualification-logs/titan-ir1157: focused 9 integration controls pass, cover audit 7 pass, fmt and all-target/all-feature Clippy pass, installed lane four controls pass in 2.29 seconds. Full integration gate pending. No 16 GiB proof run.

## Examined scope

```json
[
  {
    "id": "FR-048-AC-1",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "A selected constraint resolves in the obligation's admitted package and state context; absent, ambiguous, wrong-kind, wrong-package and wrong-context selections each yield a distinct typed refusal before harness generation or backend execution."
  },
  {
    "id": "FR-048-AC-2",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "The same safety obligation run without a constraint, with constraint A, and with constraint B has three distinct method execution identities and retains the same semantic obligation identity. Cross-method evidence or replay substitution refuses before conversion; equal requests regenerate identically."
  },
  {
    "id": "FR-048-AC-3",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "An initial boundary state and a later boundary state each undergo safety evaluation. A seeded safety violation at either boundary is falsified with in-domain playback; replacing the boundary branch with an early assumption would fail these controls. Neither boundary expands, stutters or counts as deadlock."
  },
  {
    "id": "FR-048-AC-4",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "A healthy bounded method reaching a boundary satisfies its separate boundary cover and ordinary cover, yields ConstraintReached with Reached observation, and produces no ordinary Verified or proved terminal. A boundary cover records reachability rather than an invented distinct-state count."
  },
  {
    "id": "FR-048-AC-5",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "A healthy method reaching no boundary retains a completed NotReached observation and, only with complete exhaustive evidence, a method-qualified Proved/Exhaustive/Uncertified terminal retaining the constraint under QSL FR-160; it receives no unconditional proof credit. An interrupted or missing boundary check retains Unknown and an inconclusive result. An unmet boundary cover alone cannot classify the ordinary cover as unsatisfied."
  },
  {
    "id": "FR-048-AC-6",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "A constraint that is Undefined at a reached state retains the evaluation violation rather than classifying that state as a boundary. Typed Refused and Incomplete evaluations retain their original causes and cannot become a Boolean assumption or proof. CallFailure instead stops with runtime_invariant / established-invariant-broken, without being recast as Undefined, Refused or a boundary."
  },
  {
    "id": "FR-048-AC-7",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "Removing ordinary non-vacuity evidence, exhausting unwind or resources, cancelling, or supplying malformed backend checks preserves the corresponding non-proof outcome even if the boundary cover was satisfied. Reordering check output does not change classification."
  },
  {
    "id": "FR-048-AC-8",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "The public CG conversion implements every mapping row above using typed evidence and exact method binding. A boundary safety violation follows same-method replay and FR-029 precedence; replay disagreement, refusal, absent playback and identity mismatch never produce refuted or proved by default."
  },
  {
    "id": "FR-048-AC-9",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "A public QSL terminal representation preserves the constraint method, the ConstraintReached non-proof status and the no-boundary method-qualified proof status; where that representation is unavailable, conversion returns a typed unsupported refusal. No private wire variant, missing method or unconditional proved fallback is emitted."
  },
  {
    "id": "FR-048-AC-10",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "Installed-backend healthy-boundary, no-boundary and seeded-boundary-violation controls exercise the generated method and retain their exact typed check evidence. Independent QSL evaluation agrees on the bounded reached states and boundary safety observations. Neither generation-only inspection, saved verdict nor timeout substitutes for this evidence."
  },
  {
    "id": "FR-048-AC-11",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "A standalone collection constraint remains unsupported under FR-015-AC-128 with no harness or proof credit. A method constraint never rescues an otherwise unbounded or unsupported state domain."
  }
]
```

## Dispositions

Disposition round 1 inspected fix head 043d398c9dc520e04c504a92455f67ceda75bce6 against the frozen reviewed head bcc64367.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Deleted all EvalSafety/EvalConstraint mirror functions. Actual Safe/Limit source clauses now evaluate against admitted snapshots through public QSL evaluate_clause; all 32 default states and installed small-domain controls use the original clauses. Public mapping retains borrowed ClauseEvaluation directly. |

## Final disposition verdict

PASS. Every finding is fixed within the original PR; no new finding was found in the fixes. The source is mergeable subject to the parent-owned final integration gate. Focused fixes pass 10 integration controls plus the conversion unit; installed four-control authored-clause lane passes in 1.94s. Final fmt and all-target/all-feature Clippy logs pass. Reviewer ran no Cargo or Kani and did not edit source.

## Merge synchronization

This PR700 artifact was remapped from SR-202610310146 to SR-202610310154 because main PR699 independently introduced the prior id. Original findings and round-1 dispositions above are unchanged. At rebased head 8054ccc3c5c666dd8a5e142aa01193a372ac5009 against main 0989d178, the previously dispositioned constrained generation, identity, execution, terminal and authored-clause test source is byte-identical to 043d398c. The prior PASS disposition therefore applies unchanged to this source. QSL dependency synchronization and the parent-owned rerun gates remain pending qualification; the stopped earlier aggregate gate is not a pass. This is an id correction, not a second broad review.
