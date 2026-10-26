---
id: SR-583001
title: Code review of IR-917 CG PR 583 (V2 clause harness oracle and arithmetic proof controls)
type: SpecReview
analysis: code-review
scope: 'agent-ix/quire-contract-codegen; PR #583 head cc02657484bbd99fdba3ec26326968434ee5a7b8 vs base
  9196cbe40a9341c32b0f85afda22d342dddf193c; src/kani/generate/clause/v2.rs, src/kani/generate/negotiate.rs,
  src/oracle/clause.rs, tests/it/kani_obligations_state_frame.rs'
review_set: subset
date: '2026-10-26'
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-014
  type: references
- target: ix://agent-ix/quire-contract-codegen/FR-015
  type: references
- target: ix://agent-ix/quire-contract-codegen/TC-025
  type: references
---

## Summary

Ticket: IR-917. PR: quire-contract-codegen#583, frozen head cc02657484bbd99fdba3ec26326968434ee5a7b8
over base 9196cbe40a9341c32b0f85afda22d342dddf193c (merge-base equals base). Reviewer model:
Claude Opus 5.5 (`claude-opus-5-5`). Method: code-review with the rust-review lane folded in
(rust-style baseline plus its testing and implementation references, repository CLAUDE.md and
AGENTS.md). The diff has four Rust files, 472 insertions and 12 deletions, and no spec, Cargo or CI
workflow change.

The change does four things. General-integer V2 postconditions are admitted when they have no
arithmetic, and `negotiate.rs` drops the `native` gate. A checked in-range literal
`quire.op.numeric.narrow` is elided in both the clause oracle lowering and the native expectation.
Native Boolean `eq`/`ne` expectations are added. New tests cover oracle-byte embedding, a general
equality typecheck, and an ignored installed-Kani mutant control.

## Scope

- `src/kani/generate/clause/v2.rs:257-261` general plus arithmetic refusal: examined. A general
  postcondition with an arithmetic probe is refused as `V2ClauseConditionNotRendered` before render,
  so the native probe code never runs over `rt::Integer`.
- `v2.rs:284-288`, `:479-483` general input binding `rt::Integer::from(symbolic)` and
  `ScalarIntegerRepresentation::General` identity: examined. These match the existing precondition path
  (`v2.rs:152-156`, `:194-198`).
- `v2.rs:626-637` and `src/oracle/clause.rs:512-525` literal-narrow elision: examined. Only a single
  `Integer` argument with a `result_bound`, an i128 parse, and an inclusive in-range check are accepted.
  Every other narrow shape is refused (oracle: `UnsupportedCondition`; native: `None`, which becomes the
  typed `V2ClauseConditionNotRendered` in `lower`). The value is unchanged, and the plan gets one literal
  step and no conversion step. The same literal-through-narrow precedent exists in
  `src/kani/generate/collection.rs:806`. I did not inspect QSL's runtime metering of `narrow`, so the
  claim that elision adds or removes no charge relative to QSL is unverified here (evidence limit).
- `v2.rs:810-819` native Boolean `eq`/`ne`: examined. Both operands must be Boolean, and the result is
  rendered `(a == b)` / `(a != b)`.
- `negotiate.rs:299-672` removal of the `native` parameter: examined. Postconditions now reach
  `lower_v2_clause` on both routes.
- `tests/it/kani_obligations_state_frame.rs:552-620` implication test: examined. It exercises a QSL-emitted
  package, one `kani::any()`, the subject call `subject::check(amount_current)`, the oracle source present
  exactly once, and no `implies_short_circuit` in the harness.
- `:622-670` byte-identity loop over 12 conditions: examined. Each compares the public
  `generate_v2_clause_oracle` source against the harness, with an exact single contiguous match.
- `:672-765` general equality typecheck: examined. It asserts `rt::Integer::from(amount_current)`,
  `environment.check_equality` and `checked.evaluate`, and compiles the proof body offline.
- `:767-910` installed-Kani mutant control: examined. It is `#[ignore]`d. It is reachable through
  `make kani` because the `kani_obligations` filter is a substring of the
  `kani_obligations_state_frame::` path. It was not run.
- Trace tags: examined. Every new test carries `#[trace(...)]` with FR AC ids, in the file's existing
  form.
- Hash/pin rule: examined. No new hash, pin or digest is introduced.

## Verdict

CONDITIONAL. The production changes are correct for the shapes they admit, and their refusals are typed.
Oracle embedding is asserted byte-identically on both routes, and the native expectation stays
independent of the oracle and of the runtime helper. Two medium test-strength findings remain. The
installed-Kani mutant control cannot tell which assertion failed. The newly reachable general-route
arithmetic refusal has no test. Five low findings follow. No acceptance credit is given. FR-015-AC-40,
AC-41 and AC-119 and TC-025 remain Planned in spec, which is truthful: the only real-backend test is
ignored and unrun, and AC-119's installed helper-body control belongs to quire-integration #24.

Gates: this reviewer ran no cargo build or test, at the dispatcher's direction (CG #569 holds the shared
host). The author reports `cargo fmt --check`, clippy `-D warnings` on `--lib --test it`, 4 `qsl_v2_`
integration tests and 2 `literal_narrow` unit tests passing. Those are author claims and were not
re-measured. `quire matrix` was computed at the head (see SR-583002).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The installed-Kani mutant control checks `stdout.contains(failed_assertion)`, but `--output-format regular` prints every check's description with its status. The check therefore passes whichever assertion failed. Mutant verdicts are also read from the raw `VERIFICATION:- FAILED` text, not a classified `Falsified` outcome. | tests/it/kani_obligations_state_frame.rs:876-905 |
| FND-002 | medium | The new general-route refusal of a postcondition with arithmetic (`!native_integer && !arithmetic.is_empty()`) is newly reachable now that the `native` gate is removed, and no test covers it. No test requests the general route for `amount < 1000 implies amount + 1 <= 1000` and asserts a typed unsupported item with no harness. | src/kani/generate/clause/v2.rs:257-261 |
| FND-003 | low | The narrow unit test checks only `is_some()`/`is_none()`. A narrow that returned a wrong literal expression or wrong lower/upper interval would still pass it. | src/kani/generate/clause/v2.rs:857-869 |
| FND-004 | low | The implication test checks the independent expectation with `contains("!")` and `contains("\|\|")`. A `!=` satisfies the first, and neither check pins `!(amount < 1000) \|\| (amount + 1 <= 1000)` over `amount_current`. | tests/it/kani_obligations_state_frame.rs:602-617 |
| FND-005 | low | The literal-narrow admission rule exists as two independent copies, in the oracle lowering and in the native expectation. If one changes, the oracle can admit a clause whose native expectation is refused, or the reverse. | src/oracle/clause.rs:512-525 |
| FND-006 | low | Only the `amount = 0` general postcondition is compile-checked. The other 11 newly admitted general postcondition shapes (ordering, connectives, Boolean eq/ne) are asserted only by substring and byte-identity, never typechecked. | tests/it/kani_obligations_state_frame.rs:622-670 |
| FND-007 | low | In `classify_v2_clause`, once the guard refuses every kind other than Precondition and Postcondition, the inner `None =>` arm is unreachable. One match over `Some(kind @ (Precondition \| Postcondition))` would state the rule once. | src/kani/generate/negotiate.rs:655-672 |
| FND-008 | low | The tests carry redundant and shadowing scaffolding. `src/lib.rs` is written and then overwritten (`:725` then `:735`), and the healthy crate is written twice (`:811`, `:849`). The `run` closure is shadowed by its own result, and the mutant `name` is shadowed by the playback name. | tests/it/kani_obligations_state_frame.rs:725-905 |
| FND-009 | low | The AC-41 controls use proxies. `plus_one` mutates the hand-written compiled subject, and `left` replaces the harness's own probe result. Neither mutates the clause's `+` as evaluated by the embedded oracle or the kernel, which is what AC-41 and TC-025 step 11 describe. The spec owner should confirm the subject-mutant reading. | tests/it/kani_obligations_state_frame.rs:810-830 |

## New findings (disposition pass 1)

Disposition pass 1 reviewed fix head 738ca6faa267e2966456a2bb26066a70ae19cca4. The PR was rebased onto
main 1175b995bc4314798cafe839a85a1ca2a745408c, which merges CG #584's FR-014-AC-54 and FR-015-AC-41
spec text. `git range-diff` shows the original commit carried over unchanged (cc026574 = 52cde938).
The fix delta is ab92a63b plus 738ca6fa.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-010 | medium | The `plus_one` owner-kernel mutant asserts playback `amount_current = 999`. The mutated `Add` makes the independent arithmetic assertion fail for every admitted `amount` in 0..=999 (the result is wrong, or refused where the true one completes), so the backend's chosen counterexample is not forced to be 999. When first run, the assertion can fail although the mutant was correctly falsified. | tests/it/kani_obligations_state_frame.rs:917-921 |
| FND-011 | medium | Both owner-kernel mutant runs call raw `cargo kani` outside `execute_kani_obligation`. They therefore run without the bounded memory, teardown and ceiling path the healthy run uses, on a shared host for a clause with known memory pressure. Their verdicts are also not produced under the same ceiling as the healthy run. | tests/it/kani_obligations_state_frame.rs:997-1010 |

## New findings (disposition pass 2)

Disposition pass 2 reviewed fix head 0afa590bca21eb2ebcb0afb185911ac82892d9a9 (one commit over
738ca6fa, base main 1175b995bc4314798cafe839a85a1ca2a745408c).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-012 | low | The 999 replay test hand-builds its own `Add` plan (no semantic domain, its own operand intervals) and restates the independent check as `assert_ne!(actual, 999 + 1)`. It does not execute the generated harness's arithmetic assertion at `amount_current = 999`. Only its `oracle_clause(&999)` half runs generated code. | tests/it/kani_obligations_state_frame.rs:1004-1033 |

## New findings (disposition pass 3)

Disposition pass 3 reviewed fix head be9328efa7d0ad383377b55e381a70a19f480813: commits 3814199f and
be9328ef over the rebased round-2 head 21c7f85d, on main aa55d2256bebea3d06a583d5febf7c77f77801a2.
`git range-diff` shows every earlier commit carried over unchanged. Main's delta is CG #585
(FR-015-AC-41 and TC-025 step 11 text) plus its review files. I ran no cargo build or test: the host
was at load average 27-41 with CG #569 running, and I found no log for the round-3 commits.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-013 | low | The playback source-check unit test covers only a missing test, a duplicated test and a wrong function name. It does not exercise several refusals `require_concrete_playback_in_crate` implements: an unparseable appended test, a wrong module, zero or two `#[test]` attributes, altered sealed harness bytes beside a correct appended test, and a V2 harness with invalid provenance. | src/kani/run/execute.rs:2053-2172 |
| FND-014 | low | `KaniConcretePlaybackOutcome::Completed` reports only raw process success. A libtest run that matches zero tests also exits 0, so the public API leaves "the declared test ran and passed" to each caller parsing libtest stdout (documented at :83-85). The IR-917 caller does this; any other caller that skips the check gets false evidence. | src/kani/run/execute.rs:82-102 |

## New findings (disposition pass 4)

Disposition pass 4 reviewed fix head 75b6c4426ebeae30b9e301178a9b8d70f37e6061 (one commit over
be9328ef, still on main aa55d2256bebea3d06a583d5febf7c77f77801a2). I ran no cargo build or test,
because CG #569's cargo test was still running on the shared host, and I found no log for this commit.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-015 | high | `playback_test_passed` accepts only the exact line `test {test_name} ... ok`. The only real caller, the IR-917 999 playback, is a `#[should_panic(expected = ...)]` test, and libtest prints such a test as `test {name} - should panic ... ok`: `write_test_name` in library/test/src/formatters/pretty.rs (toolchain 1.98.1) adds `- {test_mode}` whenever `TestDesc::test_mode()` returns "should panic". A genuinely passing playback therefore returns `ConcretePlaybackTestNotObserved`, and the installed test's `.expect("bounded concrete playback starts and tears down")` fails on its first run. The unit stand-in prints the plain form, so this is not caught. | src/kani/run/execute.rs:507-517 |
| FND-016 | low | The passing-test guard is unit-tested only for a zero-match summary and one exact plain pass. Untested cases are those TC-027 step 3 of draft CG #586 lists: a different passing test, a summary other than exactly one pass and zero failures, an assertion failure kept as `Completed { succeeded: false }`. Incidental or duplicated result lines are also untested; the matcher checks the test line and the summary line independently, so their order and pairing are unchecked. | src/kani/run/execute.rs:2233-2256 |

## New findings (disposition pass 5)

Disposition pass 5 reviewed fix head d571e110df94360c23d8ddad854c5cb1c663d652 on CG main
4788b46723d7454af86fdbe9c7b4f4ad1154cbb6, which merges spec PR #586 and IR-348 (CG #569).
`git range-diff aa55d225..75b6c442 4788b467..47f60919` shows every earlier commit carried over
unchanged, and main's delta shares no file with the PR. The fix delta is 43bd0aed, 63e998b5,
3a17dbea, d2a3e0df, 38107575 and d571e110. I ran no cargo build or test. Local installed-Kani logs
(`target/ir917-installed-kani.log`, `target/ir917-installed-kani-24g.log`, 2026-10-25 23:28/23:39)
show the healthy V2 proof `Inconclusive { reason: MemoryExhausted }` at 16 GiB after 257.85 s and at
24 GiB after 328.69 s (peak 25,796,603,904 bytes), before any playback or mutant. IR-1022 owns this
tractability.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-017 | medium | The PR adds an ignored installed test, matched by the `kani_obligations` filter of `KANI_FILTERS`, whose healthy proof is known to end `MemoryExhausted`. NFR-006 makes `make kani-gate` run that lane and fail unless every listed test passes. This PR touches `src/kani/` and `tests/it/kani_*.rs`, so it is itself in kani-gate scope, and after merge every later Kani-touching PR fails `make kani-gate` until IR-1022 lands. `make ci` excludes the lane, so a green `make ci` hides this. Merging needs an explicit owner/lead decision on the red lane; quarantining the test would weaken the gate and needs the same decision. No source defect is involved. | tests/it/kani_obligations_state_frame.rs:816-819 |

## New findings (disposition pass 7)

Disposition pass 7 reviewed head 98f7c46c9d14a9150b690fbbd28353f225a184b3 on CG main
8db3554e0426ece64840e36cb73d1016de99960b, which adds #590 (AD-007) and IR-348 commits. `git
range-diff 4788b467..3fa8bce4 8db3554e..3f92b906` shows every earlier commit carried over unchanged.
The PR-owned delta is commit 98f7c46c (IR-1022): AD-007 status set to accepted; a required
`V2SymbolicEncodingPolicy` (`original_i64_v1`, `post_assumption_u16_rebind_v1`) in
`V2ClauseObligationIdentity` and the serialized sealed record; and a post-assumption `i64::from(x as
u16)` rebind in both `render_clause` (precondition, as `<input>_rebound`) and `render_postcondition`
(shadowing the symbolic name). The rebind is selected only when both checked endpoints lie in [0,
u16::MAX]. The one `i64` `kani::any()` draw, its exact inclusive assumption, the 8-byte witness and
the decoder are unchanged, and the oracle, subject call, expected value and probes consume the
rebound value. The cast is lossless after the assumption, so the full checked domain is preserved.
A provenance control refuses a harness whose identity policy was changed after generation. I ran no
cargo build or test while CG #591 holds the host, and found no logged test or compile output for
98f7c46c. The lead state records that the coder's focused tests passed and that the emitted
precondition scratch compile was interrupted (exit 130, no verdict).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-018 | low | The loop `for amount in 0_i64..=1000 { assert_eq!(i64::from(amount as u16), amount) }` asserts a Rust cast identity over test-supplied values; no production change can make it fail. The generated-source and record assertions beside it carry the actual evidence. | tests/it/kani_obligations_state_frame.rs:628-634 |
| FND-019 | low | The `original_i64_v1` path is tested only at the private selection helper. No generated-harness or record test asserts that a clause whose checked domain has a negative endpoint or exceeds `u16::MAX` emits no rebind and records `symbolicEncodingPolicy = original_i64_v1`, as AD-007 requires ("otherwise retain the existing generation"). | src/kani/generate/clause/v2.rs:245-275 |

## New findings (disposition pass 8)

Disposition pass 8 reviewed head 92efc9c705735a7c765154cb54d93cdc678a8775: one commit (92efc9c7) over
98f7c46c, still on main 8db3554e0426ece64840e36cb73d1016de99960b, with no rebase. The source change is
a trace retag only. The tests replace the cast-identity loop, add an original-policy control for
QSL-emitted edge domains, and add a twin helper that edits the checked range. The spec adds
FR-015-AC-128 to AC-131, FR-015 prose, a TC-025 step 11 paragraph and interface-001 `public_types`. I
ran no cargo build, test or Kani while CG #591 holds the host, and found no log for the coder's
reported focused runs.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-020 | low | The new `original_i64_v1` generated-harness and record control covers only the postcondition constructor (`render_postcondition`, condition `amount = 0`). The precondition constructor `render_clause` has its own policy branch and binding names, but no test generates an out-of-u16 precondition and asserts no rebind and an `original_i64_v1` record. FR-015-AC-130 covers a V2 clause harness under either policy. | src/kani/generate/clause/v2.rs:143-176 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Mutant verdicts come from `classify_kani_run` over the exported JSON report and require `KaniRunOutcome::Falsified` and a `Failure` check at the rendered line of the independent arithmetic assertion (one `assert!` per line, harness first in `src/lib.rs`). |
| FND-002 | fixed | `general_v2_arithmetic_postcondition_refuses_without_a_harness` asserts zero harnesses and a `V2ClauseConditionNotRendered { node_id: clause }` record beside a supported general equality control. Its trace tag is recorded in SR-583002 FND-005. |
| FND-003 | fixed | The test asserts the exact `(expression, lower, upper, small_expression)` tuple for `0` and `1000`, and refusal for `-1` and `1001`. |
| FND-004 | fixed | The test parses the expectation and compares its token stream with the exact required `!left \|\| right` expression. |
| FND-005 | fixed | Both call sites use the single `crate::oracle::clause::admitted_literal_narrow`. |
| FND-006 | fixed | All 12 general harnesses are compiled together, one module per case, in one offline `cargo check` with `-Dwarnings`. |
| FND-007 | fixed | One `Some(kind @ (Precondition \| Postcondition))` arm, with every other kind refused by the `_` arm. |
| FND-008 | fixed | Each scratch `src/lib.rs` is written once, the `run` closure is removed, and the playback binding is renamed `parameter`. |
| FND-009 | fixed | Each mutant now patches `left.0.checked_add(right.0)` in an isolated clone of the locked quire-exact checkout. Both the embedded native oracle and the probe execute this through `evaluate_native_integer_arithmetic`, while the proof source, oracle bytes and subject stay unchanged. This matches the updated FR-015-AC-41 on main (CG #584). |
| FND-010 | fixed | Round 2: the Kani run no longer asserts the backend's arbitrary witness equals 999, and it asserts only an in-domain playback. A separate `playback999` cargo test against the same patched owner crate asserts `Completed(1001)` and `oracle_clause(&999) == Ok(Completed(false))` from the unchanged generated proof source. The test strength is recorded as FND-012; the AC-41 spec tension is recorded as SR-583002 FND-007. |
| FND-011 | fixed | Round 2: both mutants run through `execute_kani_obligation` on the isolated patched crate. The healthy run uses the same path, and the verdict must be `KaniRunOutcome::Falsified`. |
| FND-012 | fixed | Round 3: the hand-restated replay is removed. The playback test sits inside the sealed generated module and calls `kani::concrete_playback_run(vec![999_i64.to_le_bytes().to_vec()], prove_clause)`, so the generated `prove_clause` and its own arithmetic `assert!` run at `amount_current = 999` against the same patched owner crate. Failure is attributed by `#[should_panic(expected = "arithmetic subterm agrees with independent i128 expectation")]`, after asserting that message occurs once in the harness, plus `test <module>::playback_amount_current_999 ... ok` in libtest stdout. The test checks byte-identity of the proof source minus the playback test, and `execute_kani_concrete_playback` re-checks it with the same bounded launcher (wall clock, cancellation, memory, owned teardown). Not run here: the installed lane is unrun. |
| FND-013 | fixed | Round 4: `concrete_playback_uses_the_proof_process_bounds` now also asserts `ConcretePlaybackSourceMismatch` with zero launcher calls for a wrong module, an unparseable test, no `#[test]`, two `#[test]` attributes and altered sealed harness bytes. It asserts `HarnessProvenanceMismatch` with zero calls for a V2 clause harness whose record was changed after generation. The prior missing, duplicated and wrong-name cases and the timeout, cancellation and memory teardown controls are unchanged. |
| FND-014 | fixed | Round 4: a successful launcher exit now returns `Completed { succeeded: true }` only when stdout has the exact test line and a `test result: ok. 1 passed; 0 failed;` summary. Otherwise it returns the new typed `ConcretePlaybackTestNotObserved { test_name }`, which is unit-tested for a zero-match run. `CARGO_TERM_COLOR=never` keeps output parseable. The matcher wrongly refuses should-panic tests (new FND-015), and its coverage is limited (FND-016). |
| FND-015 | fixed | Round 5: `playback_test_passed` accepts exactly `test {name} ... ok` or `test {name} - should panic ... ok`, matching libtest's `write_test_name`. It requires that one pass line to precede exactly one `test result: ok. 1 passed; 0 failed;` summary, and fails closed on any other `test ... ` line or `test result:` line. `concrete_playback_parser_pairs_the_exact_test_and_summary` accepts the plain, should-panic and stderr-separated panic forms. `--lib` restricts playback to the library target, so a doc-test summary cannot appear. |
| FND-016 | fixed | Round 5: the parser test refuses a wrong test, zero passes, two passes, a failed summary, a summary first, an interleaved pass line, a duplicate test, a duplicate summary, an unrelated passing test, incidental earlier and later results, and non-UTF-8 output. The unit test also keeps a failed build as `Completed { succeeded: false, exit_code: Some(101) }` with its bounded stdout. |
| FND-017 | still-open | Round 6 (head 3fa8bce4fa8d61ca7c0b59f03952b47b5de2e62d, a test-only delta from d571e110 that touches no Makefile, `KANI_FILTERS`, `scripts/cg_gates.rs` or ignore attribute): unchanged. The ignored installed TC-025 test's healthy proof is still `MemoryExhausted` at 16 GiB and IR-1022 owns tractability, so `make kani` and `make kani-gate` stay red for this Kani-scope PR. The lead holds #583 as a draft, will not quarantine the test and will not merge with a red Kani gate. This is not a source defect. |
| FND-017 | still-open | Round 7 (head 98f7c46c): unchanged. The IR-1022 rebind exists in source only; no installed healthy proof has run at this head. The latest recorded installed runs remain `MemoryExhausted` (16 GiB and 24 GiB), and the lead state records the isolated diagnostic as `MemoryObservationFailed` with no verdict. `make kani` and `make kani-gate` still select this ignored test, and it stays red until a healthy `Verified` run with its cover satisfied, plus both same-clause mutants and the 999 playback, run at the original 16 GiB profile. |
| FND-017 | still-open | Round 8 (head 92efc9c7): unchanged. No installed healthy `Verified` run with its cover satisfied, no same-clause owner-kernel mutant `Falsified` runs, and no healthy or exact-999 playback exist at any IR-1022 head. The ignored test is still selected by `make kani` and `make kani-gate`. |
| FND-018 | fixed | Round 8: the cast-identity loop is removed. The test now locates the generated exact assumption `kani::assume(amount_current >= 0_i64 && amount_current <= 1000_i64);`, the rebind `let amount_current = i64::from(amount_current as u16);` and `let expected = `. It asserts that order (assumption < rebind < expectation) and exactly one `let amount_current = i64::from(` occurrence, so a missing, duplicated or misplaced generated rebind fails. Because the rebind shadows the symbolic name, the oracle input, subject call and probes after it consume the rebound value. |
| FND-019 | fixed | Round 8: `qsl_v2_outside_u16_domain_keeps_original_i64_harness_and_record` generates QSL-emitted packages whose checked parameter range is (-1, 1000) and (0, 65536), the edges on each side of [0, u16::MAX]. For each it asserts the identity policy `OriginalI64V1`, arguments equal to the edited range (proving the edit reached the parameter), one `kani::any()`, the exact inclusive assumption, no ` as u16`, `subject::check(amount_current)`, `identity.symbolicEncodingPolicy = original_i64_v1` in the sealed record, and parseable Rust. Only the postcondition constructor is covered (new FND-020). |
