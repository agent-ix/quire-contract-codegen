---
id: SR-15422
title: "Spec consistency check of the same-model correction"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; IR-624; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, tests/it/kani_obligations_state_frame.rs, tests/state_frame_support/native_twin.rs"
review_set: subset
---

## Summary

Non-author review of CG #523 at 05fecbe44e4cac48d854a7507fa6da11bda80538. Spec consistency check of the same-model correction. The focused Rust result is pending the shared heavy lock.

## Verdict

PASS — The edited specification scope is coherent; AC-35 correctly stays planned until its focused test and review pass.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Computed Quire matrix: AC-35 is tagged by the new test at line 3841; AC-30..33 are tagged and AC-34 is tagged by the named Kani lane. Plan completion: not assessed. No full gate was run by this reviewer. No applicable AssuranceProfile or new dependency edge was found. EARS is inapplicable because the normative requirement sentence was not changed.

```yaml
scope:
- id: FR-024-AC-35
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: PLANNED (IR-624), same-model TC-035 evidence pending. A falsified frame or postcondition harness draws
    a present model field whose accessor type has no `i64` range (for example `IntRange` with a lower endpoint below
    `i64::MIN` and an upper endpoint of -1), and playback binds an `i64` value outside that model range. QSL refuses
    the pre state and settlement is `Inconclusive` with `ReplayRefused` (FR-029-AC-16), naming the field and its
    persisted `TypeNotRange` reason; it never reports `Verified` or a violation. Playback inside the model range
    is unaffected, and an unread present `IntRa
- id: FR-024-current-state
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: FR-024-AC-35 remains planned until a same-model wide-range replay test passes.
- id: TC-035-step-32
  path: spec/replay/matrix/TC-035-counterexample-envelope-intake.md
  role: examined
  excerpt: "    so CG records `TypeNotRange`, and whose `i64` playback lies outside the model range;\n    read `Inconclusive`\
    \ with `ReplayRefused`, not a violation or `Verified`. Replay inside the\n    range is unaffected, and an unread\
    \ present `IntRange` within `i64` is never called\n    unranged. Write the out-of-`i64` bounds as canonical\
    \ decimal strings in the selected\n    domain document, and use that same document for QSL's emitted package,\
    \ the invocation\n    and replay; a package-only model override does not establish this case\n    (FR-024-AC-35,\
    \ IR-624)."
- id: tests.md-FR-024-AC-30..35
  path: spec/replay/matrix/tests.md
  role: examined
  excerpt: '| FR-024 | FR-024-AC-30 through FR-024-AC-34 | TC-035 | ✅ Covered (IR-624): emitted-package state field
    ranges come from `model_object_fields`, including unread declared fields; replay validates names against the
    accessor and frame/postcondition witnesses from QSL-emitted harnesses. The real-Kani cases run in the `kani_obligations`
    lane. |'
```

## Reassessment (disposition pass 1)

At `1b2a40ab8ee2191c2654882d7d4c00d621904ab6`, the six edited spec documents validate 6/6. The newly implemented state/model accessor claims have test binders. The replay matrix still contains one contradictory migration status.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | AC-30 is simultaneously covered and still being migrated | spec/replay/matrix/tests.md:21-22 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-002 | still-open | The AC-20..29 row retains the AC-30 migration sentence while the following row and FR-024/TC-035 claim implementation. |

```yaml
new_findings:
- fnd: FND-002
  severity: medium
  confidence: high
  method: spec-review/integrity
  check: soundness
  artifact_id: tests.md-FR-024-AC-20..29
  related_ids:
  - FR-024-AC-30
  path: spec/replay/matrix/tests.md
  lines: 21-22
  excerpt: '| FR-024 | FR-024-AC-20 through FR-024-AC-29 | TC-035 | ✅ Covered (IR-459); the frame path''s
    one obligation identity (`replay/obligation.rs`), `state_fields` record, decoded witness, domain,
    pre-state and scope ties, asserted over QSL''s own `call_site` and `replay_frame` for the QSL twin
    of the state-frame fixture |'
  finding: The AC-20..29 row says the former AC-30 real-Kani test is being migrated under IR-624, while
    the adjacent AC-30..34 row now marks the same path covered. The conflicting status will mislead readers
    about whether the emitted-package AC-30 evidence is complete.
dispositions:
- fnd: FND-002
  outcome: still-open
  reason: Remove the stale AC-30 migration sentence from the AC-20..29 row or make the status statements
    agree.
```

## Disposition pass 2

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-002 | fixed | The AC-20..29 row no longer claims AC-30 migration; AC-30..34 coverage remains in its own row. |

The sole delta from `1b2a40ab8ee2191c2654882d7d4c00d621904ab6` to `45373bce3c770972eed0feef1552d3ba6d326f84` removes the stale clause in `spec/replay/matrix/tests.md:21`. Quire validation of that document and `git diff --check origin/main...HEAD` pass. The SR-15419/SR-15420 fixed refusals and exact-head focused 1/1 Rust log carry unchanged, because no Rust or other spec file changed.

```yaml
dispositions:
- fnd: FND-002
  outcome: fixed
  after_excerpt: '| FR-024 | FR-024-AC-20 through FR-024-AC-29 | TC-035 | ✅ Covered (IR-459); the frame
    path''s one obligation identity (`replay/obligation.rs`), `state_fields` record, decoded witness,
    domain, pre-state and scope ties, asserted over QSL''s own `call_site` and `replay_frame` for the
    QSL twin of the state-frame fixture |'
```
