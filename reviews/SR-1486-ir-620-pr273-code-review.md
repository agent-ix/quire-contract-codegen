---
id: "SR-1486"
title: "CG PR 273 code review (with rust-review lane): adapt to Contract IR's typed Std001Code"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@6c6f5b9a09524c2a5ae60cdb26b68938fd149266; Cargo.lock, src/kani/classify.rs, src/kani/generate/corpus/bounded_kani_corpus.rs, src/kani/generate/lower/{bounded_kani_profile,definedness_arithmetic}.rs, src/lib.rs, tests/it/bounded_kani_corpus.rs (diff origin/main...HEAD, merge base e52ff44)"
---

# SR-1486: CG PR 273 code review

## Summary

Ticket: IR-620. PR: agent-ix/quire-contract-codegen#273 at 6c6f5b9 (one commit on e52ff44). The
`rust-review` lane is folded into this file. Contract IR read at dec8ade (read only):
`crates/quire-contract-model/src/code.rs` and `src/kani/outcome.rs`.

What I ran myself, in a fresh detached worktree with its own target directory:

- `make ci` on the head, full log read (not trimmed): fmt-check clean; `make spec` exit 0 (two
  pre-existing EARS warnings on FR-017 line 152, not in this diff); clippy `-D warnings` clean;
  msrv (1.98.1) test 162 lib + 333 `it` passed, 23 ignored, 1 doctest; `cargo deny check`
  advisories/bans/licenses/sources ok (pre-existing yanked `yoke-derive 0.8.3` warning);
  one-copy awk check passed; unsafe-audit selftest and audit passed; rustdoc `-Dwarnings` clean;
  stable `cargo test` 162 lib + 333 `it` passed, 23 ignored. Exit 0.
- `make cargo-audit`: only the pre-existing allowed yanked `yoke-derive` warning.
- Test counts: `#[test]` in `src/` 168 -> 169, in `tests/` 395 -> 395; `#[ignore` 29 -> 29. No
  test removed, no `#[ignore]` added.
- `make kani` was not re-run (the change touches no generated harness text; the claimed 23/0 is
  the author's and unverified here).

Checks the brief named:

1. Compile break fixed, minimal: the src changes are the code-type adaptation plus the
   `BoundedCorpusError` wrapper `non_success`'s new `Result` forces. Every changed assertion keeps
   the same subject and expected value (string literal -> `std001_code!` of the same literal, or
   `.code.as_str() == literal`); none is weakened. `.map_err(refusal_outcome)` panics in test code
   on the unexpected variant, so the old assertions still run on a `KaniOutcome`.
2. Public API: `Err(KaniOutcome)` -> `Err(BoundedCorpusError)`. The `Outcome` variant holds the
   whole `KaniOutcome` (kind, code, source_id, context) unchanged, so no information the old error
   carried is lost and every old caller gets it via `outcome()` or a match. `OutcomeConstruction`
   is reachable only if a `CorpusRefusal` arm mapped to `Proved`/`Counterexample`, which the
   private two-variant enum makes unwritable; on that path no other kind or cause is substituted
   (FR-030's rule holds). Given IR's `pub(super) NonSuccessKind`, this is the only shape that meets
   both NFR-005 (no `unwrap`/`expect`) and FR-030 (no substitution); see FND-003 for the cost.
   Consumers: `generate_bounded_kani_corpus_case` has no caller outside this repo (`gh search code
   --owner agent-ix` finds only CG files; local quire-driver c0b2f31 and quire-spec-language
   eddbdc54 reference neither the function nor `KaniOutcome.code`).
3. NFR-005: the non-test additions hold no `unwrap`, `expect`, panic macro, index or slice; the
   NFR-005-AC-1 scan passed in `make ci`.
4. `std001_code!` expands to a `const` item, so a malformed literal fails to compile (IR's
   compile_fail doctests); the four CG codes are byte-identical to the old literals; the wire form
   is unchanged because `Std001Code` serializes with `serialize_str(as_str())`. No CG test pins the
   serialized `KaniOutcome` (none did before either).
5. Cargo.lock: only the `quire-contract-ir` and `quire-contract-model` source lines move
   (7e4dc54 -> dec8ade); their dependency lists, and every third-party entry, are unchanged; one
   entry each; `Cargo.toml` untouched.
7. Rust smells: hand-written `Display`/`Error` follows the repo's existing pattern (no
   `thiserror` in CG). `classify.rs` now compares against `Std001Code::KANI_VACUOUS_PROOF`.
8. No other CG entry point builds a `KaniOutcome` via `non_success`. The other `Result<_,
   KaniOutcome>` functions (`classify_bounded_kani_profile`, the three `prepare_*` lowerings)
   forward IR results and need no change.

## Verdict

Mergeable after the low findings are fixed or dispositioned: no correctness defect. The adaptation
is minimal and the gates pass.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The `BoundedCorpusError` doc opens "Every refusal is a typed Contract IR `KaniOutcome`", which the second variant it then documents contradicts (`OutcomeConstruction` holds no outcome). And the `generate_bounded_kani_corpus_case` rustdoc still says a refused case "returns its original typed outcome" with no mention that it now arrives wrapped in `BoundedCorpusError::Outcome` | src/kani/generate/corpus/bounded_kani_corpus.rs:368-372, src/kani/generate/corpus/bounded_kani_corpus.rs:457-458 |
| FND-002 | low | `Display` for `Outcome` prints only `"{code}: {kind:?}"` and drops `source_id` and `context`. A caller that turns the error into a message (`Box<dyn Error>`, `?` into `anyhow`, a log line) can no longer tell which request was refused, which the old `Err(KaniOutcome)`'s `Debug` showed. `Error::source` is not implemented for `OutcomeConstruction`, which wraps an `Error` (`KaniOutcomeError`) | src/kani/generate/corpus/bounded_kani_corpus.rs:414-423 |
| FND-003 | low | The public return type now carries a variant CG cannot produce. Every caller must handle `outcome() == None` and a second arm that never happens, because IR exposes no infallible constructor for an `InvalidInput` or `Refused` outcome. A pub IR constructor per non-success kind, or a pub `NonSuccessKind`, would let CG return `Err(KaniOutcome)` again and delete the variant. This is an IR request, not a CG code defect; recorded so the design debt is visible | src/kani/generate/corpus/bounded_kani_corpus.rs:377-380, src/kani/generate/corpus/bounded_kani_corpus.rs:436-451 |

## Dispositions

Round 1, reviewed at ac9e82143dfe8eb5c522d256abc3030a576e0fdb (one fix commit, ac9e821, on 6c6f5b9; base still e52ff44). The fix commit touches only `spec/assurance/AD-003-evidence-chain.md`, `spec/kani/functional/FR-030-ir-outcome-terminal-map.md` and `src/kani/generate/corpus/bounded_kani_corpus.rs`. In the source file it changes rustdoc, `Display`, `Error::source` and one test; no render function and no generated harness text changes, so the earlier real-Kani result is unaffected and re-running `make kani` is not needed. `make ci` on ac9e821, run by me with its own target dir and full log read: exit 0 (fmt-check, spec, clippy, msrv 162 lib + 333 `it` passed / 23 ignored / 1 doctest, deny ok, one-copy ok, unsafe audit passed, rustdoc clean, stable test same counts).

- FND-001: the `BoundedCorpusError` doc now names `Outcome` as the usual case and the `OutcomeConstruction` variant doc says CG cannot produce it today. The `generate_bounded_kani_corpus_case` rustdoc names both variants. "Almost always" is softer than "always, today", but the variant doc right below it states the exact fact, so nothing is false.
- FND-002: `Display` for `Outcome` prints code, kind, source_id and context; `Error::source` returns the wrapped `KaniOutcomeError` for `OutcomeConstruction` and `None` for `Outcome`. The test asserts the exact `Display` string ("kani_corpus_serialization_failed: Refused for source \"case-7\" in context \"rev-9\""), `source().is_none()` for `Outcome`, and `source().is_some()` for `OutcomeConstruction`. A mutant that drops source_id or context from the message, or that returns `None` from `source` for the wrapped error, fails it. Observation, not recorded as a finding: `OutcomeConstruction` both prints the wrapped error's message and returns it from `source`, so an error-chain printer shows that message twice. The arm is unreachable today.
- FND-003: the fix is in Contract IR. IR-621 (Backlog) asks for a public infallible per-kind constructor or a public `NonSuccessKind`, and its wording matches this finding and the new `OutcomeConstruction` rustdoc.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | ac9e821 |
| FND-002 | fixed | ac9e821 |
| FND-003 | deferred | IR-621: the fix belongs to Contract IR (public infallible non-success constructor or public NonSuccessKind); CG's OutcomeConstruction rustdoc now states it cannot be produced today |
