---
id: SR-202610310149
title: "spec-review/dependency review of constrained method implementation"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-contract-codegen; IR-1157 PR700 changed paths"
review_set: subset
---

## Summary

Ticket IR-1157, CG PR #700 frozen head bcc6436780d39ee712717e722518d0aa202adbd1 against origin/main 78e725a8. One independent review; no source edits or Cargo execution. FR-048 AC1..11, TC-062 and changed AD-004 boundaries were checked. No applicable AssuranceProfile exists. The generation entry is negotiate-only, renderer remains private/shared, identities sit below runner, and terminal conversion above runner. QSL UnsupportedTerminalRepresentation fallback is allowed and emits no forged terminal. New TC-062 verifies FR-048; no cycle introduced. Complete canonical operation census comes from adopted IR-1236. QSL terminal representation remains an explicit unsupported prerequisite rather than a private schema.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for this lens; code-review and gap-analysis findings still block the PR.

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
