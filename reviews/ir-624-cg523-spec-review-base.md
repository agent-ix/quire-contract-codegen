---
id: SR-15421
title: "Spec base check of AC-35 status and TC-035"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; IR-624; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, tests/it/kani_obligations_state_frame.rs, tests/state_frame_support/native_twin.rs"
review_set: subset
---

## Summary

Non-author review of CG #523 at 05fecbe44e4cac48d854a7507fa6da11bda80538. Spec base check of AC-35 status and TC-035. The focused Rust result is pending the shared heavy lock.

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
- id: tests.md-FR-024-AC-30..34
  path: spec/replay/matrix/tests.md
  role: examined
  excerpt: '| FR-024 | FR-024-AC-30 through FR-024-AC-34 | TC-035 | ✅ Covered (IR-624): emitted-package state field
    ranges come from `model_object_fields`, including unread declared fields; replay validates names against the
    accessor and frame/postcondition witnesses from QSL-emitted harnesses. The real-Kani cases run in the `kani_obligations`
    lane. |'
- id: tests.md-FR-024-AC-35
  path: spec/replay/matrix/tests.md
  role: examined
  excerpt: FR-024-AC-35 remains planned pending same-model wide-range replay evidence.
```

## Reassessment (disposition pass 1)

At `1b2a40ab8ee2191c2654882d7d4c00d621904ab6`, the edited FR-015, TC-025, Kani matrix, FR-024, TC-035, and replay matrix documents validate 6/6. The newly covered statuses for FR-015-AC-27 and AC-77..81, and FR-024-AC-31..35, correspond to tagged tests in the computed matrix; the named Kani-lane binders for AC-80 and AC-34 are deliberately ignored outside that lane. No new base finding. The cross-row stale migration sentence is recorded by the integrity method.
