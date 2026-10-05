---
id: "SR-1488"
title: "CG PR 274 code review (with rust-review lane and test-oracle strength): the FR-030 IR-outcome terminal map, its tests, the FR-029 helper refactor and the QSL c8f0c28 lock move"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@944ef8f3909fabb3e8e7c247b7c904b363bbc5ac; Cargo.lock, deny.toml, src/kani/terminal.rs, src/lib.rs, tests/it/terminal_map.rs (diff 48a517b...944ef8f, merge base 48a517b = main); context: Makefile, src/kani/run/execute.rs"
---

# SR-1488: CG PR 274 code review

## Summary

Ticket: IR-465, with IR-358. PR: agent-ix/quire-contract-codegen#274 at 944ef8f, merge base
48a517b, which is `main` HEAD. This review runs the code-review and rust-review lanes. It also
measures test-oracle strength with fourteen hand-applied mutants of `ir_outcome_terminal_value`.
Each mutant was reverted afterwards, and the worktree was clean after the run.

What I measured myself rather than taking from the PR text:

- Gates. `make ci` on the head, with its own target dir, exited 0. It covers fmt-check, spec,
  clippy `-D warnings`, msrv, deny, audit-unsafe, rustdoc and test. The library tests ran 162
  passed. The `it` binary ran 346 passed, 0 failed and 23 ignored. Both match the PR's claim.
  `#[test]` count in `tests/it/terminal_map.rs` goes from 14 to 27: 12 AC-tagged `tc_041_*`
  tests plus one tagged TC-041 only. No test is removed. `make kani` was not re-run. No
  generator or harness source is in the diff. The QSL move from 02530e7 to c8f0c28 changes
  `qsl-replay`'s `proof_result.rs` (adds `DeclineCode::Std001`), its root re-export and its
  comments. The relocated `quire-exact` (a270df3) and `quire-semantic-value` (660a126) differ
  from the in-tree copies at 02530e7 only in comments and test-function names; I spot-checked
  `integer.rs` and `accounting.rs`. So no emitted harness text can change.
- Lock. Only first-party git sources move. The eight QSL crates go from 02530e7 to c8f0c28, which
  is QSL `main` HEAD and QSL #634. `quire-exact` is re-sourced to its own repo at a270df3, and
  `quire-semantic-value` to its own repo at 660a126; each is that repo's `main` HEAD.
  `qsl-replay` gains one dependency edge, `quire-contract-model`. IR stays at dec8ade, which is
  IR `main` HEAD, with one entry each for `quire-contract-ir` and `quire-contract-model`. No
  crates.io package changes. `Cargo.toml` is untouched. `make deny` reports `advisories ok, bans ok,
  licenses ok, sources ok`, and `check_one_copy.awk` passes. The warnings are all ones that `main`
  also has (an unmatched `quire-spec-language` license exception, unmatched license allowances,
  yanked `yoke-derive`).
- deny.toml. It adds exactly two `allow-git` entries, `https://github.com/agent-ix/quire-exact`
  and `https://github.com/agent-ix/quire-semantic-value`. Both are first-party, public repos.
  Their license exceptions were already present.
- QSL types, read at c8f0c28 without checking anything out. `DeclineCode` is `Qsl(Code) |
  Std001(Std001Code)`, `Copy`. `TerminalValue::Declined { cause: ProofRefusalCause, code:
  DeclineCode }`. `qsl-replay/src/lib.rs:70` re-exports `quire_contract_model::{std001_code,
  Std001Code}`. `InternalFault` is not re-exported from `qsl-replay`, so the AC-10 reason holds.
  The map builds `DeclineCode::Std001(outcome.code)`. It records no issuer and has no
  `is_registered` check. That matches QSL's doc ("names the registry, not an issuer, and refuses
  no unregistered code") and FR-030's Behavior text.
- NFR-005 surface of the non-test diff. `ir_outcome_terminal_value` has no `unwrap`, `expect`,
  indexing, `panic!` or `unsafe`. Its `match` has no `_` arm and no binding catch-all. The two
  guarded arms fall through to named-kind arms.
- `Unavailable`. Both `kani_backend_absent` and any other code go to one arm,
  `Unsupported(BackendAbsent)`. FR-030's table gives both the same value, so merging them loses
  no distinction the spec checks. AC-8's test pins `kani_backend_absent` and a third code
  separately (mutants M4, M5 and M6 are killed).
- FR-029 helper refactor. `map_settled(x)` is `map_run(&falsified(), Some(x.into()))`. The new
  `for_each_cg_failure` and `for_each_setup_refusal` callbacks call exactly that expression,
  with the same expected value, over the same inputs. The only loss is the `"{error}"` failure
  message on a few assertions. That is diagnostic only, so no FR-029 oracle is weaker.

Test-oracle strength. I ran each mutant through `cargo test --test it terminal_map::tc_041`:

| Mutant | Result | Killed by |
| --- | --- | --- |
| M1 swap `Refused`/`InvalidInput` causes | killed | tc_041 AC-2 |
| M2 `Declined` code replaced by a constant `Std001` code | killed | tc_041 AC-2 |
| M3 `Declined` code as `DeclineCode::Qsl(Code::InvalidPackage)` | killed | tc_041 AC-2 |
| M4 `Unavailable`, other code, to `SolverAbsent` | killed | tc_041 AC-8 |
| M5 solver guard keyed on `kani_backend_absent` | killed | tc_041 AC-8 |
| M6 extra arm sending `kani_backend_absent` to `SolverAbsent` | killed | tc_041 AC-8 |
| M7 swap `TimedOut`/`Cancelled` | killed | tc_041 AC-3 |
| M8 vacuous proof forwards the count | killed | tc_041 AC-5 |
| M9 last `None` arm becomes `(_, None)` | killed | tc_041 AC-7 (syn inspection) |
| M10 `Refused` arm folded into the `InvalidInput` arm | killed | tc_041 AC-2 |
| M11 every settled counterexample is `Refuted` | killed | AC-9, AC-11, AC-12, AC-13 and the TC-041 CG-defect test |
| M12 `Proved` ignores the count | killed | tc_041 AC-4 |
| M13 vacuous guard always true | killed | tc_041 AC-5 |
| M14 `IncompleteInput` to `InvalidInput` | killed | tc_041 AC-2 |

## Verdict

The map is correct against FR-030's table and QSL c8f0c28. Every claimed-backed AC has a test
that a plausible mutant fails, and the lock and deny changes are minimal and first-party.
There are two findings, both outside `terminal.rs`. Both are falsehoods that this PR's line of
work leaves behind. The `use-local` patch list no longer matches the lock this PR writes, so
`make use-local` against a current QSL checkout fails (FND-001). And a doc comment still says
the terminal map is not implemented (FND-002). Verdict: approve once FND-001 is fixed. FND-002
is a one-line fix and belongs in the same round.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The lock moves `quire-exact` to `https://github.com/agent-ix/quire-exact` and `quire-semantic-value` to `https://github.com/agent-ix/quire-semantic-value`. `LOCAL_PATCHES` still has `quire-spec-language:quire-exact:quire-exact`, and it has no entry for either new repo. QSL at c8f0c28, the revision this lock moves to, has no `quire-exact/` directory (IR-582, QSL #632). I ran `make use-local` against a sibling QSL checkout at c8f0c28, and it fails: `use-local: .../quire-spec-language is not cloned (no Cargo.toml at .../quire-spec-language/quire-exact)`. Even with that directory present, a `[patch."…/quire-spec-language"] quire-exact` entry no longer matches the lock's source. Fix: replace the entry with `quire-exact:quire-exact:.` and add `quire-semantic-value:quire-semantic-value:.` | Makefile:154-160; Cargo.lock:1442,1475 |
| FND-002 | low | The `KaniExecutionEvidence::success_checks` doc still says "the terminal map that will read it (FR-029) is not implemented yet". `run_terminal_value` (FR-029) and now `ir_outcome_terminal_value` (FR-030) both take that count, so the comment is false. Reword it to name the two maps that read the count. | src/kani/run/execute.rs:213-214 |

## Dispositions

Round 1, reviewed at 7bbe22f4a4784006db6c461db3b3742677b6bda2. That is one fix commit, 7bbe22f, on
top of 944ef8f. `diff 944ef8f..7bbe22f` touches only the Makefile, FR-029, FR-030, TC-041, the
kani `tests.md`, `execute.rs` and `tests/it/terminal_map.rs`, and every hunk is a fix for one of
the four findings. `terminal.rs`, `Cargo.lock` and `deny.toml` are unchanged, so the round-0
mutant results for the map stand.

Gate: `make ci` on 7bbe22f, run with its own target dir, exited 0. The library tests ran 162
passed. The `it` binary ran 347 passed, 0 failed and 23 ignored; the extra test is the new AC-14
test. `make deny` reported `advisories ok, bans ok, licenses ok, sources ok`. The only other
warnings are the two FR-017 EARS warnings, which `main` also has.

How I checked FND-001. The new entries `quire-exact:quire-exact:.` and
`quire-semantic-value:quire-semantic-value:.` have exactly two colons, no `::`, and no leading or
trailing colon, so `use-local`'s entry check accepts them. Each repo gets its own
`[patch."https://github.com/agent-ix/<repo>"]` table. I ran `make use-local` end to end with the
Makefile's own `SIBLINGS` override, pointed at a scratchpad sibling set:

- QSL at c8f0c28.
- Temporary scratchpad clones of quire-exact at a270df3 and quire-semantic-value at 660a126.
- IR at dec8ade, quire-contract-runtime at ccc722b and quire-verification-contracts at 1fc0ff6,
  which are the locked revisions.

`use-local` wrote both new patch tables, and `cargo metadata` passed with no "patch was not
used" warning. `quire-exact`, `quire-semantic-value`, `qsl-replay` and `quire-contract-model`
all resolved to the sibling paths. `make use-remote` then restored the lock, and the worktree was
clean. Every scratch clone and worktree was removed afterwards. (Note: a sibling quire-contract-runtime
older than ccc722b still names `quire-exact` through QSL. That is a stale local checkout and not
a Makefile defect.)

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 7bbe22f: `LOCAL_PATCHES` drops `quire-spec-language:quire-exact:quire-exact` and adds `quire-exact:quire-exact:.` and `quire-semantic-value:quire-semantic-value:.`. `make use-local` against a sibling set at the locked revisions succeeds, and both crates resolve to the sibling paths |
| FND-002 | fixed | 7bbe22f: the `success_checks` doc now says the count is the one "the terminal maps (FR-029, FR-030) take as an explicit input" |
