---
id: SR-7740
title: "IR-625 code review — state-clause Kani playback"
type: SpecReview
analysis: code-review
scope: "quire-contract-codegen@fd9807659dd6c05d0633373168d68083f9cae14d vs 91d594387257dbe0e6bbf2c69cfb88f58d6e118f; src/kani/generate/frame.rs, src/kani/output/playback.rs, src/lib.rs, src/replay/state_clause.rs, src/replay/witness.rs, tests/it/kani_obligations_state_clause_replay.rs"
review_set: subset
---

## Summary

Reviewed the full six-file diff and the shared decoder, state-frame identity, generated assertion, QSL clause selection, and native replay call sites. The adapter uses the shared positional decoder, retains state-field draw order, converts decoded integers with `i64::try_from`, and the focused synthetic and real-Kani replay tests pass. One assertion-provenance issue remains.

## Verdict

**CONDITIONAL** — one medium finding. No source or test files were edited; full gates were outside this review request.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The adapter accepts `decoded.check_text == expected` or `assertion failed: {expected}` without requiring the exact generated two-line `assert!(predicate, "{}", message)` clause. The generated harness emits only the latter clause, and the real-Kani test proves it can be decoded. A same-harness assertion elsewhere with matching text, or a playback whose generated clause is replaced by the one-line text, passes `decode_falsification` as this postcondition even though its check is not the generated postcondition. The new synthetic test explicitly accepts this simpler form. Require the exact generated clause for state-clause replay and add a negative one-line/matching-message control; keep the shared decoder for bytes and draw order. | src/replay/state_clause.rs:532-545; src/kani/generate/frame.rs:1111-1137; tests/it/kani_obligations_state_clause_replay.rs:62-66, :103-112 |

## Verification

`git diff --check origin/main...HEAD` passed. `cargo test --locked --test it state_clause_kani_playback_adapter_refuses_wrong_evidence -- --nocapture` passed 1/1. `/usr/bin/time -f 'ELAPSED=%e EXIT=%x' cargo test --locked --test it tc_035_real_kani_state_clause_counterexample_replays_through_qsl -- --ignored --nocapture` passed 1/1 (0.78 s test, 1.01 s wall, exit 0); log: `/tmp/ir625-review-real-kani.log`. Test support's `prove` discovers installed cargo-kani 0.68.0/CBMC 6.11.0 and calls the production bounded Kani executor. Independent `execve` tracing was unavailable because sandbox ptrace was denied. No full gate was run.

## Dispositions

| FND | Outcome | Evidence |
| --- | --- | --- |
| FND-001 | fixed `21e2f559856cd110fb9251556c74b9ce138a9ade` | `src/replay/state_clause.rs:536-543` now requires the exact generated two-line clause and removes both one-line fallbacks. `tests/it/kani_obligations_state_clause_replay.rs:62-104` refuses both former forms. The focused adapter test passed 1/1, and the single real-Kani replay test passed 1/1 on the fixed head (18.35 s test, 18.57 s wall, exit 0; `/tmp/ir625-review-real-kani-fix.log`). The changed Rust lines introduce no new issue. |
