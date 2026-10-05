---
id: "SR-1528"
title: "CG PR 282 code review (with rust-review lane and test-oracle strength): the QSL bcca433 lock move, the UnknownField and UnknownPopulation settlement arms and the typed DomainKey"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@bd19f73acf6aa3830f8e0ce7b5abc6401516195a; Cargo.lock, src/replay/function.rs, src/replay/state_clause.rs, tests/it/terminal_map.rs, tests/it/kani_obligations_state_clause_replay.rs (diff 735e704...bd19f73); context: Makefile, deny.toml, scripts/check_one_copy.awk, src/replay/frame.rs; QSL agent-ix/quire-spec-language at bcca433 and c8f0c28; quire-exact at a270df3 and 2ec5e1e"
---

# SR-1528: CG PR 282 code review

## Summary

Ticket: IR-623. PR: agent-ix/quire-contract-codegen#282 at bd19f73, one commit on merge base
735e704. `main` is now e3d3ab4 (#281 adds spec and reviews only), and `git merge-tree` merges the
PR onto it cleanly. This review runs the code-review and rust-review lanes and checks the oracle
strength of the tests with eight hand-applied mutants. I took nothing from the PR text without
measuring it:

- Lock. `git ls-remote` puts QSL `main` at bcca43356937348525bc098d49557d96039c1bf0. The diff moves
  only the eight `qsl-*` `source` lines, from c8f0c28 to bcca433. No dependency list, checksum or
  third-party entry changes. `quire-exact` has a single lock entry at a270df3. It was already
  a270df3 on the merge base, so the PR leaves it unchanged rather than "putting it back". a270df3
  is the commit QSL bcca433's own `Cargo.lock` pins. quire-exact `main` is 2ec5e1e (`ls-remote`).
  It renames `Refusal::DivisionPairOutOfDomain` to `DivisionOutOfDomain` (read in both checkouts),
  and QSL bcca433 still names `DivisionPairOutOfDomain` in `qsl-foundation/src/diagnostic.rs:1044`.
  The author's claim that QSL does not build against quire-exact `main` therefore holds. Holding
  it back is stable under `cargo update -p qsl-replay` and under `--locked` builds. A bare
  `cargo update` would float it to 2ec5e1e and fail to compile, which is loud and not silent, and
  stays expected until QSL adopts the single-member divide (the IR-602 prerequisite). The PR body
  says so. No repo text needs to: CG does not name quire-exact in `Cargo.toml`, and the breakage
  cannot pass a gate. `deny.toml` is unchanged. `cargo deny check` reports advisories, bans,
  licenses and sources ok, and the one-copy check passes. The yanked `yoke-derive` 0.8.3 warning
  comes from an entry this PR does not touch.
- Two new arms. At bcca433, `CallSiteRefusal` gains `UnknownField` and `UnknownPopulation`.
  `code()` maps both to `Code::MissingDeclaration`. QSL's own
  `TerminalValue::from_call_site_refusal` adds both to the non-fault arm beside `UnknownClause`.
  CG's `From<&CallSiteRefusal> for ReplaySettlement` does the same, so the two settle as
  `SetupRefused(code())`, which matches FR-029 and FR-030 (SR-1529). The match stays total, with
  no wildcard.
- DomainKey. QSL turned the `DomainKey` struct `{node, path}` into an enum `Node{node, path} |
  Population{member_type, ordinal}`, and removed `DeclaredDomain::parameter()` and
  `DomainKey::new` / `path()`. The new `DomainKey::Node { node: parameter, path: vec![path] }`
  has the same fields and values as the old `DomainKey::new(parameter, vec![path])`, so the
  change preserves behaviour. Whether that key is the right one under QSL's new state-field
  definition is SR-1529 FND-001.
- Gates. I ran `make ci` on bd19f73 with my own target dir and read the whole log
  (`makeci-cg282-reviewer-bd19f73.log`). It exited 0. fmt-check, spec, clippy `-D warnings`,
  msrv, deny, audit-unsafe (with self-test), rustdoc and test all passed. The library ran 167
  passed. `it` ran 370 passed, 0 failed and 24 ignored, the same under msrv and stable. Doctests
  ran 1. These match the author's counts. `#[test]` counts are unchanged: `terminal_map.rs` has
  30 and `kani_obligations_state_clause_replay.rs` has 10, before and after. No `#[ignore]` is
  added. I did not run `make kani`, per the brief. The diff touches no generator or harness
  source: nothing under `src/kani` and no snapshot changes. The two `src` edits are in replay
  conversion and in envelope building, so no generated harness text can change.
- Test-oracle strength. I applied eight mutants by hand and reverted each one. The worktree was
  clean afterwards. Each was run against the three affected tests, and each was killed by a test
  assertion, not by a compile error:
  M1 `UnknownField => CgDefect`, M2 `UnknownPopulation => Fault`, M3 `UnknownField =>
  SetupRefused(InvalidPackage)`, M4 `ReplayPackageError::CallSite => CgDefect`, M5
  `FrameReplayError::CallSite => CgDefect`. Each of these is killed by both
  `tc_040_a_setup_refusal_after_a_refutation_is_replay_refused_never_declined` and
  `tc_041_a_setup_refusal_after_a_counterexample_is_replay_refused_never_declined`.
  M6 key path `+1`, M7 empty path and M8 wrong node are each killed by
  `tc_035_the_envelope_declares_each_ranged_field_and_the_transcript_names_the_playback`.
  Every changed assertion keeps its subject and its expected value. The first `tc_035` assertion
  is now stronger, because it compares the whole key. The second still checks only the path, as
  it did before.
- Rust lane and NFR-005. No `unwrap`, `expect`, `panic!`, indexing, `unsafe` or `as` cast is
  added in `src`. The `u32::try_from` and `map_err` path is unchanged. No new public item is
  added. The import changes are rustfmt-ordered. Nothing in the diff is unrelated to the change.

## Verdict

APPROVE (code). The lock move is exactly the QSL crates. The two arms follow QSL's own helper and
the merged FR-029 and FR-030 rows. The DomainKey edit preserves behaviour. All eight mutants are
killed. The PR body still says "Draft: full make ci and make kani still to run", although the PR is
not a draft. I re-ran make ci and it is green. Kani was not re-run by me. The lead should update
that line or check the kani claim before merging. The spec-side consequences of the lock move are
in SR-1529.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
