---
id: "SR-1020"
title: "CG PR 243 code review (Rust lane): panic-free emitted equality source and generator"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@0573ebdcff3fc0bca4ad2815961813c5fe8d9943; src/oracle/equality/mod.rs, tests/common/mod.rs, tests/common/panic_scan.rs, tests/it/composite_equality_generation.rs, tests/it/exact_scalar_generation.rs; diff origin/main...HEAD, base 4b6e09c"
---

# SR-1020: CG PR 243 code review (Rust lane): panic-free emitted equality source and generator

## Summary

Ticket: IR-538. PR: agent-ix/quire-contract-codegen#243 at 0573ebd. The base, 4b6e09c, is current origin/main. The review covers `git diff origin/main...HEAD` only, using code-review with the rust-review lane folded in. Everything below was measured in a detached throwaway worktree with its own target dir.

- **Emitted source.** All six emitted `.expect` calls are gone. Each runtime constructor, and the integer parse, now sits in one of six `rebuild_*` helpers returning `Result<_, ReconstructionError>`. The helpers are a table, `RECONSTRUCTION_HELPERS`, listed callees first.
  - `ReconstructionError` has the unit variants Integer, Interval, Rational, Decimal, Text and Cardinality.
  - `EnvironmentError` has `Declaration(rt::InvalidDeclaration)` and `Reconstruction(ReconstructionError)`.
  - The per-item functions `composites_*`, `left_source_*`, `left_target_*`, `right_source_*` and `right_target_*` return `Result<_, ReconstructionError>`.
  - `environment_*` returns `Result<rt::TypeEnvironment, EnvironmentError>` and maps a reconstruction failure to `EnvironmentError::Reconstruction`.
  - `oracle_*` matches each per-item call and returns `Outcome::Refused(Refusal::CheckedInvariant)` on `Err`, as in the FR-021 precedent.
  - The oracle no longer calls `unwrap_or_else(left_source_*)`. It builds each type once and moves it into `EqualityOperand`.
- **Helpers are emitted only when used.** `SourceBuilder::finish` makes one reverse pass over the table, which gives the transitive closure because callees come first. `make lint`-equivalent clippy `--all-targets -D warnings` is clean.
  - The repo's own `tc_029_ac12_manifest_is_unpublished_and_charge_free` builds the TC-029 corpus crate with `RUSTFLAGS=-Dwarnings`, and it passed.
  - `tc_029_ac12_integer_helper_omitted_when_unreached` covers the omission path.
- **Generator.** The two `unreachable!` arms are gone. `render_value_type` and `render_composite_declaration` return the private `RenderError { UnsupportedValueType { family }, Generation(OracleGenerationError) }`.
  - `settle_render` maps `UnsupportedValueType` to `CompositeEqualityRefusal::Unsupported { unsupported_node_id: claim.node_id, node_tag: family }`. `Generation` propagates and fails the whole call.
  - `render_decimal_type`, `profile_path` and the other path mappers keep returning `OracleGenerationError`, lifted with `From`.
- **Public API is unchanged.** The only added `pub enum` lines are inside the emitted `SOURCE_HEADER` string. `OracleGenerationError` (`src/oracle/claim.rs`) is untouched.
  - quire-driver (local main c0b2f31) matches `OracleGenerationError` exhaustively in `src/refusal.rs`, and its four variants are unchanged.
  - The driver does not reference the composite-equality API, `environment_*` or `InvalidDeclaration`, so nothing breaks.
- **Render before naming.** Every pending item is rendered before `unique_names` runs, and only rendered items are named. With no render refusal, which is every reachable request today, naming and output are the same as before.
  - The existing exact-string assertion updates (AC-12 helper omission, AC-16 text/int and sibling-bound strings) only re-spell the same bounds in the new `rebuild_*(...)?` form. Each is the intended emitted-source change, and none drops an assertion.
  - No golden or snapshot file changed.
- **Panic surface of the non-test generator code.** There is no `unwrap(`/`expect(`/panic macro. The remaining `.unwrap_or(0)` on `counts.get` is not a panic.
- **Kani.** No `src/kani/**` module references `oracle::equality`, so `make kani` is not needed.
- **Process.** `git diff --check` is clean. There is no heredoc residue (EOF markers, stray `cat`), and the appended tests end with a newline and pass `cargo fmt --check`. No process finding.

## Verdict

Approve as far as the code goes. The implementation matches FR-018's behaviour bullets and the SR-1006 design decisions. It is idiomatic: typed errors, no panics, helpers emitted on demand, and an unchanged public API. There is one low robustness nit in the shared scan helper. The substantive gaps are in test strength (SR-1021) and AC wording (SR-1022).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `non_test_code` splits at the first literal `#[cfg(test)]` text. A doc comment or string that mentions it earlier would silently truncate the AC-19/AC-39 scans to a prefix. The AC-19 test only asserts the text occurs, not that the cut lands on the test module. | tests/common/panic_scan.rs:48-56, tests/it/composite_equality_generation.rs:1534-1546 |

## Dispositions

Round 1 was reviewed at d55fdd404a2a7f86e6f62f0ee1d4816e90ceec46. The coder rebased once onto main 68ca78c, which includes #242. `git range-diff` shows 0573ebd and fff7549 are the same patch, so the rebase changed nothing. The fix-round delta is fff7549..d55fdd4. Everything below was measured in a detached throwaway worktree.

- **Rebase.** No conflict and no interaction with #242's `DuplicateDeclaringNode` (src/oracle/function/mod.rs is untouched by this PR).
- **Public API.** `OracleGenerationError` (`src/oracle/claim.rs`) is still unchanged.
- **Exact-string assertions.** The delta changes none.
- **Gates.**
  - fmt-check and `clippy --locked --all-targets -- -D warnings` are clean.
  - The full `cargo test --locked` passes: 118 lib, 267 `it` (15 ignored Kani), 1 other.
- **The rewritten lexical scan** (`tests/common/panic_scan.rs`) was reviewed for panics of its own.
  - Every slice and index is bounded: `get`, `skip`/`take`, `saturating_sub`, and a `depth -= 1` that cannot underflow because a block comment starts at depth 1.
  - One false negative is possible in principle. A line-start `#[cfg(test)]` on a struct field or an enum variant would remove the following siblings up to the closing `}`. No generator writes this, so it is noted, not filed.
  - The false positives are conservative and intended: the words `unwrap`, `expect` and `abort` are banned even inside emitted string text.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | d55fdd4 |
