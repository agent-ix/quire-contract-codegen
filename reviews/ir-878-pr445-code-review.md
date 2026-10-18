---
id: SR-12872
title: IR878 PR445 code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; PR445 head 2eeab5d9c7654a6f39846f768ded0b0792a855a1 vs merge-base cb96317e; tests/it/strategy_generation.rs
review_set: subset
---

## Summary

Ticket: IR-878. Non-author code review of quire-contract-codegen PR 445, with the rust-review lane folded in. The diff is test-only: one file, tests/it/strategy_generation.rs, +11/-3. It adds FR-002-AC-3 to the `#[trace(..)]` attribute of `tc_004_supported_populations_are_directly_shaped_and_shrink_inside_constraints`. Inside the generated scratch crate, it now draws the broad correlated-offset strategy from a separate deterministic `TestRunner` whose `Config` sets `max_local_rejects = 0`.

## Verdict

PASS with one low finding. The change asserts what it claims. In proptest 1.5.0, `Filter::new_tree` calls `TestRunner::reject_local` on every rejected draw, and `reject_local` returns `Err("Too many local rejects")` once `local_rejects >= max_local_rejects`. With a ceiling of zero, the first rejection anywhere in the correlated strategy's 64 generations makes `new_tree(..).unwrap()` panic, which fails the generated crate's `cargo test` and the outer test. The rejection counter is never reset outside `run`, so the guard covers all 64 trees together. The existing static assertion `!correlated.rust.contents.contains("prop_filter")` (line 73) is still there. The new runtime guard is the part that catches rejection-based shaping not spelled `prop_filter`.

A scratch mutation probe built against proptest 1.5.0 confirmed the guard is not vacuous. A filtering correlated strategy fails under the zero-reject runner but passes under `TestRunner::deterministic()`. The directly shaped `prop_map` form passes under the zero-reject runner. The generator's broad correlated body (src/strategy/campaign.rs:585-592) is a direct `(primary, offset).prop_map(..)` pair, so the tested population is the production output. The runner construction mirrors `TestRunner::deterministic()` exactly: `Config::default()` plus `TestRng::deterministic_rng(config.rng_algorithm)`. The trace attribute is the `ix_trace_rs::trace` macro imported at line 1. The diff adds no `/// Trace:` lines, no vendored file or schema, no compatibility layer, no new dependency or pin, no unsafe code and no production code. The new tag has no stub or tautology: it asserts a property that a filtering implementation would fail.

Rust lane: no panics are added beyond `unwrap` on test-only values. There are no integer conversions, locks or async code. The generated crate keeps `RUSTFLAGS=-Dwarnings`, so the new `Config`/`TestRng` imports must be used, and they are. The only finding is a low clarity and idiom point (FND-001).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `correlated_config.max_global_rejects = 0` does nothing on the `new_tree` path, because global rejects are counted only by `TestRunner::run` for `prop_assume!`. It suggests the test guards something it does not. The mutate-after-`Config::default()` form could be the struct-update form `Config { max_local_rejects: 0, ..Config::default() }`. | tests/it/strategy_generation.rs:112-114 |

## Coverage

Examined: the full diff; the whole of `tc_004_supported_populations_are_directly_shaped_and_shrink_inside_constraints`; generator bodies `broad_body`, `boundary_body` and the `StatePinned`/`NoEvent` arms of `strategy_body` (src/strategy/campaign.rs:552-732); proptest 1.5.0 `Filter::new_tree`, `TestRunner::reject_local`, `TestRunner::reject_global`, `TestRunner::deterministic` and `Config` fields.

Scoped execution: `cargo test --locked --test it strategy_generation::` on the frozen head: 13 passed, 0 failed, including the changed TC-004 test. Mutation probe: a standalone scratch crate on proptest =1.5.0 with three tests: the zero-reject runner catches the filtering mutant, the default deterministic runner accepts the same mutant, and the zero-reject runner accepts the shaped form. All three passed.

## Examined units

```yaml
scope:
- id: FR-002-AC-3
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Supported correlated constraints do not rely on improbable filtering.
- id: FR-002-AC-4
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: context_only
  excerpt: Shrinking preserves generated constraints or records a residual rejection.
- id: FR-002-AC-6
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: context_only
  excerpt: Integer and generated Boolean harness campaign cases bind exact values to executable accepted/rejected expectations; enum cases execute an all-admitted membership expectation.
```
