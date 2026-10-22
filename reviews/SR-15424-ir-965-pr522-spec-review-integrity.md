---
id: SR-15424
title: "Integrity review of IR-965 state-clause replay target"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; IR-965; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md; uncommitted diff atop 5424f623908da3a086960be14666060c009f0e3c"
review_set: subset
---

## Summary

Independent review of uncommitted IR-965/STD-261 extension at diff SHA256 `3f934b86dae897bd9be90c3cb9eac7f09c811dae7b0f1510a8106e0a4f47666e`. Grounded in merged QSpec FR-323 (#364, 429c59d) and QSL FR-122 (#1211, 95a93c8) plus QSL FR-070/STD-245 (#1212, e0f8ffb). The three exclusive targets, clause-name ownership, no payload `clause`, pairwise refusal, Q-6 removal, and prospective implementation status agree with those owner specs, subject to the findings below.

## Verdict

CONDITIONAL — Correct the stale TC and matrix claims before using this extension as the implementation contract.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Replay matrix calls AC-12 covered while the revised selection is unimplemented | spec/replay/matrix/tests.md:20; spec/replay/functional/FR-024-counterexample-envelope-intake.md:526-533 |

## Coverage

Only the new uncommitted FR-024 diff and its directly conflicting TC/matrix claims were examined. The prior SR-15418 disposition remains historical context. FR-024-AC-41/42 have no trace binders yet, consistent with this spec-first draft; AC-12 retains old test binders that do not establish the new selection. Quire validation of edited FR-024 is grammar clean. No Cargo or full gates ran.

```yaml
scope:
- id: FR-024-AC-12
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: The state-clause envelope's `clause_node` equals the `ClauseSite` node, and its `occurrence_key`
    the `ClauseSite` occurrence, that `qsl_replay::call_site` returns for the clause name. Its packet
    and request carry `state_clause` as that clause's declared `QualifiedName`, with neither other replay
    target; its payload has no `clause` selection and its `observation` is the `Invocation` arm. For a
    unit with two postcondition clauses, each clause's envelope carries its own node, occurrence and declared
    name. A clause name the unit does not declare returns `StateClauseReplayError::CallSite` holding t
- id: tests.md-FR-024-AC-11..19
  path: spec/replay/matrix/tests.md
  role: examined
  excerpt: '| FR-024 | FR-024-AC-11, FR-024-AC-13 through FR-024-AC-19 | TC-035 | ✅ Covered (IR-460);
    `StateClauseReplay` in `replay/state_clause.rs`, asserted over QSL''s own `replay_state_clause` and
    `call_site` for the QSL twin of the state-frame fixture; FR-024-AC-18 is a real-Kani test of the `kani`
    lane (`make kani`) |'
```

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | The revised TC and matrix state the new selection and its pending implementation consistently. |

## Disposition verdict

PASS on the final uncommitted three-file diff atop `5424f623908da3a086960be14666060c009f0e3c`, diff SHA256 `33ffcc67b065b78d3ef8790b5dfbbfd7e0f9bc35488dd41a959b4f4cb9583a96`. Quire validation is 3/3 grammar-clean; `git diff --check` is clean. The amended TC step 10 requires packet/request `state_clause` and no payload `clause`; the replay matrix isolates AC-12/41/42 as planned while AC-11/13..19 remain covered. The previously ambiguous frame-only refusal bullet was narrowed in this candidate before final disposition.

```yaml
dispositions:
- fnd: FND-001
  outcome: fixed
  after_excerpt: '| FR-024 | FR-024-AC-11, FR-024-AC-13 through FR-024-AC-19 | TC-035 | ✅ Covered (IR-460);
    `StateClauseReplay` in `replay/state_clause.rs`, asserted over QSL''s own `replay_state_clause` and
    `call_site` for the QSL twin of the state-frame fixture; FR-024-AC-18 is a real-Kani test of the `kani`
    lane (`make kani`) |

    | FR-024 | FR-024-AC-12, FR-024-AC-41, FR-024-AC-42 | TC-035 | 🚧 Planned (IR-958 / IR-965): function,
    frame and state-clause packets still construct `selected_function`; the state-clause payload still
    carries `clause`. The three-member selection and typed missing/conflicting refusals h'
```
