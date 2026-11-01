---
id: SR-2026103036
title: "code-review review of constrained Kani method semantics"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-048-constrained-state-clause-method.md"
review_set: subset
---

## Summary

Ticket: IR-1157. Reviewed frozen PR #679 at 6c3dbf3b, only the new FR-048. All eleven criteria and its Description, Inputs, Outputs, Behavior, and dependency edges were examined. CG FR-015, FR-017 and FR-029 and QSL FR-158/FR-160 were context. No applicable AssuranceProfile exists in this repository. This is spec-only: Rust review, production gap-analysis and execution tests do not apply; implementation evidence is explicitly Planned.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for this method; semantic findings in the other method artifacts still block the PR.

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
    "excerpt": "A healthy method reaching no boundary retains a completed NotReached observation and a method-qualified bounded success without unconditional proof credit. An interrupted or missing boundary check retains Unknown and an inconclusive result. An unmet boundary cover alone cannot classify the ordinary cover as unsatisfied."
  },
  {
    "id": "FR-048-AC-6",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "A constraint that is Undefined at a reached state retains the evaluation violation rather than classifying that state as a boundary. Typed Refused and Incomplete evaluations retain their original causes and cannot become a Boolean assumption or proof."
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
    "excerpt": "A public QSL terminal representation preserves the constraint method and non-proof status; where that representation is unavailable, conversion returns a typed unsupported refusal. No private wire variant, missing method or proved fallback is emitted."
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
  },
  {
    "id": "FR-048",
    "path": "spec/kani/functional/FR-048-constrained-state-clause-method.md",
    "role": "examined",
    "excerpt": "When a caller selects a checked state constraint as a Kani search method,\nCG SHALL retain the constraint as a method input rather than a proof obligation.\nWhen that method reaches a state outside the constraint, CG SHALL evaluate\nthat state's safety obligations before stopping expansion from it."
  }
]
```
