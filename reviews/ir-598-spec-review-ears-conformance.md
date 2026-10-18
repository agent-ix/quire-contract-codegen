---
id: SR-12871
title: EARS review of IR-598 refreshed corpus retirement spec
type: SpecReview
analysis: ears-conformance
scope: 'agent-ix/quire-contract-codegen; IR-598, PR #423; spec/kani/functional/FR-015-bounded-kani-obligations.md,
  spec/replay/functional/FR-024-counterexample-envelope-intake.md'
review_set: subset
---

## Summary

Ticket: IR-598. Independent supplemental spec-review/ears-conformance of the frozen spec candidate. No new defects found.

## Verdict

**PASS** for spec delivery. Future retirement remains Planned and gated; IR598 code waits on IR858, collection additionally on 4h, and graph/final on the preceding slices and IR805.

## Review scope

```yaml
scope:
- id: FR-015-retirement-description
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Current interim ownership (IR-329): the bounded-profile classifier, the
    three

    finite-input semantic-family lowerers and the bounded-corpus emitter are auxiliary

    operations of this Kani generator, governed today by FR-015-AC-82 to

    FR-015-AC-94 and, at the step 4g boundary, by the planned per-family retirement

    slices FR-015-AC-109 (arithmetic), FR-015-AC-110 (collection) and FR-015-AC-111

    (graph and the whole corpus API), with FR-015-AC-95 as their end state.'
- id: FR-015-retirement-gates
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'FR-015-AC-96 to FR-015-AC-102 retain the collection implementation and
    partial

    statuses recorded below; graph clauses remain PLANNED. This retirement plan

    adds no proof credit or current backend results.

    FR-015-AC-109 to FR-015-AC-111 state the slices and FR-015-AC-95 their end

    state. QSL-353 establishes that

    QSL-emitted packages read through Contract IR; it does not supply the symbolic

    corpus or the graph population input. The graph slice also waits for IR-805: CG
    integration of the merged IR-851/IR-852

    accessors and QSL-820 real-package population emission (after QSL-819). Retirement
    of each'
- id: FR-024-corpus-status
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: "- The bounded-Kani corpus retains no counterexample packet, so no corpus\
    \ counterexample reaches\n  QSL as the `Input` arm and FR-024-AC-5 is unbacked.\
    \ The corpus path is to be removed, not\n  built on: FR-015-AC-109 to FR-015-AC-111\
    \ plan its deletion per family, and FR-015-AC-111 drops\n  its replay, so FR-024-AC-5\
    \ stays unbacked."
- id: FR-015-main-statement
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'When a caller selects Kani generation for complete-V1 contracts whose
    scalar

    oracles FR-014 generated, the code generator shall emit one bounded Kani

    obligation per precondition, postcondition, invariant and frame condition,

    each bounded by its model-domain bounds. It is the Kani

    backend''s one generator

    ([ADR-001](../../decisions/ADR-001-overlapping-generators-and-input-models.md)),

    and every item it lowers is a claim over an admitted `quire.checked-package/v2`

    package.'
- id: FR-024-main-statement
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: context_only
  excerpt: 'When a counterexample is to be replayed, the code generator shall carry
    it to QSL as QSL''s

    counterexample envelope (`qsl_replay::WitnessEnvelope`), keyed by QSL''s `ObligationIdentity`
    and

    holding exactly one `qsl_replay::ReplaySource` arm. It shall validate every value
    against its

    declared domain before any replay, and shall replay only through `qsl_replay::replay`,
    or through

    `qsl_replay::replay_frame` for a frame counterexample, or through

    `qsl_replay::replay_state_clause` for a postcondition state-clause counterexample.'
```

## Method and evidence

The frozen candidate changes Markdown only. No source, test, manifest, workflow or source-embedded input changes. No Cargo, Kani or measurements ran; no current proof result is claimed. No applicable AssuranceProfile exists in spec/. The two edited requirement-bearing documents are FR015 and FR024. The changed FR015 lifecycle explanation was checked for event/state intent, concrete deletion gates and measurable response; FR024's added current-state paragraph was checked to avoid turning its future removal into a present-tense claim. No new shall statement was introduced. The main event-triggered shall statements are unchanged context, and the added status/history sentences are explanatory prose, not malformed additional EARS obligations. AC wording is outside this lens's requirement-statement grammar scope.

`quire validate --scope . "spec/**/*.md" --summary` reports 107/109 documents grammar-clean, with three warnings in unchanged FR017 (missing-subject and unclassifiable at line 259) and FR014 (non-canonical-trigger at line 254). None concerns changed FR015/FR024 prose. Semantic inspection finds no event/state inversion, unverifiable response, or unwanted-condition pattern introduced by the lifecycle explanation. Structural validation and grammar warnings do not establish implementation or backend success.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
