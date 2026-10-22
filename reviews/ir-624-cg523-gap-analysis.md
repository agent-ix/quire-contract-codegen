---
id: SR-15420
title: "AC-35 trace tag is present but refusal evidence is incomplete"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-624; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, tests/it/kani_obligations_state_frame.rs, tests/state_frame_support/native_twin.rs"
review_set: subset
---

## Summary

Non-author review of CG #523 at 05fecbe44e4cac48d854a7507fa6da11bda80538. AC-35 trace tag is present but refusal evidence is incomplete. The focused Rust result is pending the shared heavy lock.

## Verdict

FAIL — The test needs an assertion on QSL admission cause and offending field before AC-35 can be counted as evidence.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The AC-35 test accepts an unrelated QSL replay refusal | tests/it/kani_obligations_state_frame.rs:3907-3923; spec/replay/functional/FR-024-counterexample-envelope-intake.md:379 |

## Coverage

Computed Quire matrix: AC-35 is tagged by the new test at line 3841; AC-30..33 are tagged and AC-34 is tagged by the named Kani lane. Plan completion: not assessed. No full gate was run by this reviewer. No applicable AssuranceProfile or new dependency edge was found. EARS is inapplicable because the normative requirement sentence was not changed.

Reverse gap: no new production behavior is introduced. Matrix tag alone overstates evidence until the refusal variant, cause, and field are asserted. Optional whole-repository semantic review was not run; the scoped code/test correspondence was inspected as part of code-review.

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
- id: tc_035_same_model_wide_range_refuses_outside_playback
  path: tests/it/kani_obligations_state_frame.rs
  role: examined
  excerpt: "        .replay()\n        .expect_err(\"QSL refuses the pre state outside its model\");\n    let FrameReplayError::Refused\
    \ { refusal, unranged } = &error else {\n        panic!(\"expected QSL refusal: {error}\");\n    };\n    assert_eq!(unranged,\
    \ &harness.unranged);\n    let terminal = quire_contract_codegen::run_terminal_value(\n        &crate::terminal_map::synthetic_evidence(\n\
    \            KaniRunOutcome::Falsified {\n                counterexample: \"wide model field\".to_owned(),\n\
    \            },\n            0,\n        ),"
- id: TC-035-step-32
  path: spec/replay/matrix/TC-035-counterexample-envelope-intake.md
  role: examined
  excerpt: "    so CG records `TypeNotRange`, and whose `i64` playback lies outside the model range;\n    read `Inconclusive`\
    \ with `ReplayRefused`, not a violation or `Verified`. Replay inside the\n    range is unaffected, and an unread\
    \ present `IntRange` within `i64` is never called\n    unranged. Write the out-of-`i64` bounds as canonical\
    \ decimal strings in the selected\n    domain document, and use that same document for QSL's emitted package,\
    \ the invocation\n    and replay; a package-only model override does not establish this case\n    (FR-024-AC-35,\
    \ IR-624)."
```

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | The test now matches QSL admission refusal, invalid-value, audit, and an independent terminal code. |

Round 1 examined new head `1b2a40ab8ee2191c2654882d7d4c00d621904ab6`. The exact-head focused Rust log `/home/peter/dev/qualification-logs/ir624-pr523-focused-1b2a40ab.log` reports 1 passed, 0 failed, 726 filtered. The other new lines are specification status changes; the integrity method records their one new discrepancy.

```yaml
dispositions:
- fnd: FND-001
  outcome: fixed
  after_excerpt: "    let ReplayRefusal::Admission(AdmissionFailure::Refused(record)) = refusal.as_ref()\
    \ else {\n        panic!(\"expected an invalid pre-state value: {refusal}\");\n    };\n    assert_eq!(record.code,\
    \ Code::InvalidRuntimeInput);\n    assert_eq!(record.cause, \"invalid-value\");\n    assert_eq!(\n\
    \        record.fields.get(\"field\").map(String::as_str),\n        Some(\"audit\")\n    );\n    let\
    \ terminal = quire_contract_codegen::run_terminal_value("
```
