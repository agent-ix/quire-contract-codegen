---
id: "SR-12220"
title: "IR-863 code review (incl. rust-review lane): Kani-lane Trace: doc lines to #[trace]"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@8aa1f25a81450ea82ae2ca6ba486162201403cdf; src/kani/**, tests/it/kani_obligations*.rs, tests/it/kani_generation.rs, tests/it/kani_gate.rs, tests/it/bounded_kani_corpus.rs, tests/it/kani_batching.rs, tests/it/vacuity_primitives.rs, tests/it/cover_last.rs"
relationships: []
---

# SR-12220: IR-863 code review

## Summary

Ticket: IR-863. PR: agent-ix/quire-contract-codegen#426, head 8aa1f25, diffed with
`git diff origin/main...HEAD` (34 files, all `.rs`, all inside the agreed Kani-lane file set). The
rust-review lane is folded into this file.

The PR replaces deprecated `/// Trace:` doc lines with ix-trace-rs `#[trace("...")]` attributes on
`#[test]` items and adds `use ix_trace_rs::trace;` to every module that uses one.

## Method

I read the full diff and ran scripted checks over every changed file at HEAD:

- Every changed line is a doc comment, a `#[trace(...)]` attribute or one of its argument lines,
  a `use ix_trace_rs::trace;` import, or blank. No code, string literal, template, golden or
  schema file changed. No non-`.rs` file changed.
- 310 `#[trace]` sites. Each sits on a `#[test]` item (with `#[ignore]` or `#[cfg(...)]` between
  where present). None is on a `#[kani::proof]` or production fn. None is path-qualified
  (`ix_trace_rs::trace(`), which the coverage tool would not see. No doc comment follows an
  attribute, and no `#[cfg]` precedes `#[trace]`.
- ix-trace-rs 9378a67 (the locked rev) expands to the item unchanged, so `#[trace]` ahead of
  `#[cfg(target_os = "linux")]` is sound. Because `#[trace]` is the first attribute, the macro is
  always invoked, so the import stays used on every target.
- Each module with a `#[trace]` has its own import, and no module has an unused one (classify.rs
  and execute.rs each have two test modules with an import each). The crate is a dev-dependency
  and every src import sits inside a `#[cfg(test)]` module.
- All 653 attribute arguments pass the macro's `is_id_shaped` rule, so none fails to compile.
- Removed blank `///` lines all separated prose from a removed `Trace:` line. Prose that followed
  `Trace: TC-023.` on the same line was kept as plain doc text.
- `cargo fmt --check` passed. I did not compile or run tests. No CI build check runs on this PR
  (only the CLA checks).
- tests/it/kani_gate.rs, cover_last.rs and layout.rs read some of these sources by path. None of
  them matches `Trace:` text.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The PR body describes the first push, not head 8aa1f25. It says kani_argument_order.rs was converted (the fix round reverted it), that negotiate.rs:99 `Trace: TC-025` was left (it was dropped), and reports binder refs 1142 (head gives 1141, the same as main). Its fmt/clippy/test results predate the fix round, which converted three more files (bounded_kani_corpus.rs, vacuity_primitives.rs, cover_last.rs). | PR #426 body |

## Verdict

Mergeable once the PR body is refreshed. That is a low, documentation-only finding. The conversion
is mechanical and complete within scope, attribute placement and imports are correct under every
cfg, and nothing emitted or golden changed. Re-running the clippy/test lane for the fix-round files
is advisable before merge, but static checks found no compile risk.
