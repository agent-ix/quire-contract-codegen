---
id: SR-12200
title: "Base spec review of IR-598 per-family corpus retirement (CG #423 delta)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR #423 delta f22c59d9..1163a768 (IR-598): spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/assurance/AD-001-codegen-architecture.md, spec/assurance/AD-004-cg-crate-layout.md, spec/core/functional/interface-001-codegen-api.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/spec.md, spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, spec/kani/matrix/tests.md"
review_set: subset
date: 2026-10-17
---

## Summary

Ticket: IR-598. Base checklist review of the CG #423 delta only (`git diff f22c59d..1163a76`): FR-015 narrative and AC-51/52/55/57/58/82-95 edits, new AC-109/110/111, AD-001, AD-004 step 4g, interface-001, FR-024, spec.md, TC-023 and tests.md. Each claim was checked against CG origin/main 0e0f92e source (read-only, no cargo). The merge commit f22c59d is an exact union of #417 (81126d7) and #421 (0652a64): FR-015 keeps #417's AC-105..108 and #421's AC-112..117 intact, and an all-files diff of the changed lines matches #421's own delta. `quire validate --scope . "spec/**/*.md"` exits 0. Two high findings: AC-109's replacement replay test cannot be built on main, and AC-109 deletes the corpus replay in the arithmetic slice, although that replay only serves the graph and collection arms, which stay live.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-109's replacement real-Kani test cannot be built: a V2 scalar harness carries `ScalarObligationIdentity` (src/kani/identity.rs:185, routed/generate.rs:826), while `replay_counterexample` takes the V1 contract `KaniObligationIdentity` (src/replay/function.rs:1006). The V2 scalar run path (src/kani/run/scalar.rs) has no QSL replay. The AC-37 mutation changes the harness oracle, not the QSL function, so QSL replay of the authored semantics would not reproduce it, and `Reproduced` is the wrong expected verdict. The test also replaces nothing, because the arithmetic corpus never falsifies. | FR-015-AC-109, FR-015-AC-37, TC-023 (spec/kani/functional/FR-015-bounded-kani-obligations.md:800) |
| FND-002 | high | Corpus replay is deleted in the arithmetic slice, but it serves only the Graph and Collection arms. The only caller, tests/it/bounded_kani_corpus.rs:511-554, replays graph and collection cases, and an arithmetic oracle is always true (AC-57). Deleting it at AC-109 removes replay from arms that stay live until AC-110 and AC-111. That contradicts the ruling 'removed in the same slice that deletes its arm' and the rule that each slice deletes only what its own family owns. Move the deletion to the last falsifiable slice (AC-111), with AC-99 (collection) and AC-106 (graph) as the real-Kani replacements, and make the same change in AD-001, AD-004 4g, interface-001, spec.md, FR-024 and TC-023. | FR-015-AC-109, FR-015-AC-110, FR-015-AC-111, FR-015-AC-57 (spec/kani/functional/FR-015-bounded-kani-obligations.md:800) |
| FND-003 | medium | Amended AC-57 says a Falsified corpus playback is 'run evidence only: no corpus playback is replayed against QSL' while the arm remains. That is false today: the live test tc_023_real_corpus_playback_replays_through_its_qsl_source (Trace: FR-015-AC-57) replays it through `BoundedCorpusCase::replay_counterexample`. The tests.md row for AC-57 still cites 'empty-valued playback', which the AC no longer mentions. | FR-015-AC-57, FR-015-AC-109, TC-023 (spec/kani/functional/FR-015-bounded-kani-obligations.md:748) |
| FND-004 | medium | The div/rem refusal is misstated. `quire.op.integer.rem` is a refused catalogue identity with no oracle family (src/oracle/scalar/mod.rs:5326, family map :3935-3936), so it never reaches `lower_scalar_claim`'s NoRenderer. `ScalarLoweringRefusal` is a pub(super) internal; the public disposition for div is `Unsupported(OperationNotRendered)` (negotiate.rs:871-876). `quire.op.integer.mod`, which has a family and is owned by IR-854, is not named. State each case's public settled disposition. | FR-015-AC-109, AD-004, TC-023, IR-854 (spec/kani/functional/FR-015-bounded-kani-obligations.md:800) |
| FND-005 | medium | The AC-84/85 loss record overstates what is lost. The corpus divide and remainder QSL text (bounded_kani_corpus.rs:742-748) uses `/`, which is QSL's `Divide` and not `div`/`rem`, and `%`, which the pinned qsl-forms 23f9f34 does not parse (its operator table at value.rs:1177-1194 has no `%`). So a Remainder case's retained QSL cannot compile through `call_site` (inferred, not run). The loss is a Rust checked_div/checked_rem literal check with no mutation control. Record exactly that, so IR-854 does not inherit it as QSL division coverage. | FR-015-AC-109, FR-015-AC-84, FR-015-AC-85, IR-854 (spec/kani/functional/FR-015-bounded-kani-obligations.md:800) |

## Verdict

Not mergeable as is. Sound parts: the per-family split, the staged no-wrapper retirement, AC-95 kept as an end-state invariant, the IR-858 gate on every slice, the IR-805/QSL-819/IR-852/IR-851 graph gate, the IR-854 pointer for division, and the stale AD-001 replay claim, which is now corrected. The merge resolution is exact. The replay-deletion timing and the replacement test must be fixed before the code slices can be planned from this spec.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Verified at 455b1b1 (round 1) |
| FND-002 | fixed | Verified at 455b1b1 (round 1) |
| FND-003 | fixed | Verified at 455b1b1 (round 1) |
| FND-004 | fixed | Verified at 455b1b1 (round 1) |
| FND-005 | fixed | Verified at 455b1b1 (round 1) |
