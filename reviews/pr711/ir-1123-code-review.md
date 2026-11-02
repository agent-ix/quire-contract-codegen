---
id: SR-202610310182
title: "IR-1123 PR 711 code-review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; tests/it/kani_obligations_state_frame.rs; PR 711 frozen source b88b5d6c"
review_set: subset
---

## Summary

Ticket: IR-1123. code-review scoped to the two installed-control ceiling selections. Includes the Rust review lane, repository conventions, duplication, resource bounds, integrity and test alignment checks.

Frozen source b88b5d6cfc54ceafe41a3f26ab8f00afcdf6d776; base c401acbdba8be9915040823949562ed73d668114; locked QSL e9e7cdfc026cba5b5c8ea58a4d4d77d3ed48c449. Only two existing generation call sites change. The shared helper receives the same twin, clause, state fields and subject and retains unwind 4. Existing domains, native calls, typed playback intake and assertions are unchanged. The reused ceiling is 4 GiB / 120 seconds; no 16 GiB execution was performed. No production, API, dependency, spec or CI change. No applicable AssuranceProfile was found.

Evidence inspected in /home/peter/dev/qualification-logs/titan-ir1123/: fmt/clippy exit 0; focused TC-035 22 passed, 0 failed, 1 ignored; each exact installed control passed. Retained compiled/generated sources retain both 0..1000 state domains and the audit preservation assertion. Backend receipts show forbidden Falsified with concrete playback and healthy Verified, both using unwind 4 and requested ceilings. The unchanged forbidden control executes the decoded playback natively and asserts a QSL reproduced violation naming account.audit. The healthy control explicitly uses handcrafted playback after a Verified outcome; it is not backend-produced playback. Bounded parse, identity and out-of-range value refusal tests passed in the focused group.

This qualifies the existing typed boundary only. Full FR-024-AC-30 and IR-624 emitted-package/no-Twin::aligned obligations remain OPEN. Parent lead owes the full ordinary integration gate after review; no aggregate result is claimed here. No Cargo or Kani command was run by this reviewer.

## Verdict

PASS for the dispatched PR subset: no introduced defects found. Full ordinary integration gate remains the parent lead responsibility. Repository-wide assurance and full FR-024-AC-30 are not assessed as passed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Reconciliation: quoin matrix 0.29.1, repository root explicitly supplied. Raw output retained as matrix.json; coverage.json retains stale-tag diagnostics. Repository baseline: tagged 590, untagged 331, method-without-symbol 93, tagged-by-ignored-test 12; all 1026 report no run evidence in the Quoin store. Coverage reports 10 diagnostics and 21 untracked symbols. These existing repository gaps are not repaired or claimed covered by this two-call-site PR. FR-024-AC-30 is statically tagged (3 binders); FR-015-AC-32 and FR-024-AC-34 are ignored-only in the static axis, while the explicit installed lane receipts demonstrate the scoped executions. Static tags do not establish full AC coverage. No new trace tags or requirement units are introduced.

Scope is the dispatched PR subset, not a repository assurance PASS. Two changed test behaviors and the shared generation helper/ceiling were inspected; zero added production behaviors, zero new reverse gaps, zero source/test stubs introduced. Existing tests assert real failure/verification and replay behavior. Full repository reverse inventory and optional semantic review were outside this scoped dispatch; no repository completeness assertion is made. Plan completion: not assessed.

Examined units and context (same scope as the tracker comment):

```yaml
scope:
- id: tc_035_real_kani_frame_counterexample_replays_through_qsl
  path: tests/it/kani_obligations_state_frame.rs
  role: examined
  excerpt: "    let forbidden = generated_from_twin_with_ceilings(\n        &twin,\n\
    \        \"BalanceNeverDrops\",\n        &STATE_FIELDS,\n        \"crate::subject::deposit_touching_audit\"\
    ,\n        SMALL_FRAME_CEILINGS,\n    );"
- id: tc_025_real_kani_the_allowed_frame_effect_replays_as_a_respected_frame
  path: tests/it/kani_obligations_state_frame.rs
  role: examined
  excerpt: "    let allowed = generated_from_twin_with_ceilings(\n        &twin,\n\
    \        \"BalanceNeverDrops\",\n        &STATE_FIELDS,\n        SUBJECT_PATH,\n\
    \        SMALL_FRAME_CEILINGS,\n    );"
- id: generated_from_twin_with_ceilings
  path: tests/it/kani_obligations_state_frame.rs
  role: examined
  excerpt: 'unwind: 4,'
- id: SMALL_FRAME_CEILINGS
  path: tests/it/kani_obligations_state_frame.rs
  role: examined
  excerpt: 'memory_bytes: std::num::NonZeroU64::new(4 * 1024 * 1024 * 1024).unwrap(),

    wall_clock: Duration::from_secs(120),'
- id: FR-024-AC-30
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: context_only
  excerpt: The harness is generated from QSL's emitted package and its scope node
    ids equal those `qsl_replay::call_site` names, without `Twin::aligned` (FR-024-AC-32).
```
