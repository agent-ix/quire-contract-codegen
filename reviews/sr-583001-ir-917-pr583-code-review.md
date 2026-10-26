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
