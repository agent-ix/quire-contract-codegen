---
id: "SR-1441"
title: "IR-464 code review: a closing cover for the V1 bundle and corpus harnesses, and the cover-last guard"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@20b3dff4f65ed8d57e51d4d7d80654cce60c1064; src/kani/generate/v1_bundle.rs, src/kani/generate/corpus/bounded_kani_corpus.rs, tests/it/cover_last.rs, tests/it/layout.rs, tests/it/kani_generation.rs, tests/it/bounded_kani_corpus.rs, tests/it/kani_obligations.rs, tests/it/kani_obligations_state_frame.rs, tests/it/main.rs"
relationships: []
---

# SR-1441: IR-464 code review (PR #264)

## Summary

Ticket: IR-464. PR: agent-ix/quire-contract-codegen#264, head 20b3dff, diffed against
origin/main 0765cc3 (the merge base). The rust-review lane is folded into this file.

The PR adds one line to each of two templates. `render_kani_source` (`v1_bundle.rs`) now ends the
`proof_for_contract` harness with `kani::cover!(true, ...)` after the contract call.
`render_artifacts` (`bounded_kani_corpus.rs`) now ends the corpus harness with one cover after
`assert!(corpus_oracle())`. The tests add a `syn` inspection of every emitted harness
(`tests/it/cover_last.rs`), a scan of the non-test string literals of `src/`
(`non_test_string_literals` in `tests/it/layout.rs`), and real-Kani tests for both kinds.

## Method

- Read the whole diff, the two templates, `select_assertion_block`, `counterexample_playback` and
  the lexer in `tests/it/layout.rs` that the new scan reuses.
- Ran the new non-Kani tests at the head: `cover_last`, `layout` and
  `tc_023_every_corpus_family...` all pass (9 tests).
- Ran the Kani lane myself on Kani 0.68.0 / CBMC 6.11.0:
  `cargo test --locked --test it -- --ignored --test-threads=1 bounded_kani_corpus kani_generation`.
  Result: 9 passed (772 s).
- Exemplar check (a scratch crate, no clone of the QSL repo). I built the shape the brief gives:
  oracle `implies_short_circuit(a < 1000, || a + 1 <= 1000)`, a `requires`/`ensures` contract over
  `() `, and a harness that draws `amount_current` in `[0, 1000]`. I ran it with the proof graph
  options (`--harness proof_x::x_proof --exact --unwind 2 --solver cadical -Z function-contracts
  -Z concrete-playback --concrete-playback print`) plus `--export-json`. There were four
  variants: no cover or a cover last, and the oracle as written or with ` + 1_i64` spliced onto
  its Add. I classified each with `classify_kani_run` and decoded it with `decode_falsification`
  from this head:
  - no cover, unmutated: `Inconclusive { MissingCoverSummary }`.
  - no cover, mutated: `Falsified`, decoded `[("amount_current", Integer(999))]`.
  - cover last, unmutated: `Verified`. The cover is satisfied, so 1 of 1 cover properties is
    satisfied.
  - cover last, mutated: `Falsified`, decoded `[("amount_current", Integer(999))]`. The transcript
    now has two playback blocks: a `cover` block (511) and the `assertion` block (999).
    `select_assertion_block` picks the assertion, so the decode does not change.

  The cover does not touch the oracle fn: it is in the harness template, not
  `precondition_source`/`postcondition_source`, so the one `BinOp::Add` stays. The check set
  gains exactly one property (`x_proof.cover.1`). The unwinding assertions are the same (two
  `unwinding assertion loop 0` in every variant), so `--unwind 2` behaves the same.
- Mutation-tested the guard in a separate worktree with its own target dir. Each row lists the
  tests that failed:
  - V1 cover removed: `tc_025_every_emitted_harness...` failed.
  - corpus cover removed: `tc_025_every_emitted_harness...` and
    `tc_023_every_corpus_family...` failed.
  - state-clause cover moved before the assert: the inspection failed.
  - two covers in the precondition harness: the inspection failed.
  - corpus cover wrapped in `if true { }`: the inspection failed.
  - a new file `rogue.rs` with a `"#[kani::proof]"` literal: the scan failed.
  - a literal after a `#[cfg(test)] mod` that holds `"}"` and `r#"{"#`: the scan failed, so the
    gate does not over-skip.
  - `br##"..#[kani::proof_for_contract(c)].."##`: the scan failed.
  - the attribute only inside a `#[cfg(test)] mod`: the scan passed, which is correct.
  - **Escapes:** the scan passed for `format!("#[kani::{}]", "proof")`,
    `concat!("#[kani::", "proof]")` and a `\` line continuation inside the literal, each in a
    new file. It also passed for an undriven `#[kani::proof]` emitter added to `frame.rs`, a file
    already in the driven set. These are recorded in SR-1442 against FR-015-AC-58.
- Kani-lane mutations: see `## Kani mutations` below. All four are caught.
- Checked `kani/test_support.rs`: its only gate is `#[cfg(test)] pub(crate) mod test_support;`
  in `src/kani/mod.rs:27`, so it really is test-only and skipping it is correct.
- NFR-005: in `src/` the PR adds only one string line and one `let _ = writeln!(..)`. It adds no
  panic token, index or subtraction, and it does not touch the AC-8 scan bodies.
- Gate: I ran `make ci` on a clean target dir (`target-ci`). See `## Gates`.

## Kani mutations

These ran in the separate mutation worktree, each under the host lock:

- V1 cover moved before the contract call:
  `tc_025_real_kani_classifies_the_v1_bundle...` fails. The unsatisfiable-requires bundle
  classifies `Verified` instead of `CoverUnsatisfied { 0, 1 }`. A cover placed before the call
  is reached even when no argument satisfies the requires. That is the vacuity the AC rules out,
  and the test catches it.
- Corpus cover moved before the assert: the false-collection and false-graph tests both fail. The
  run is refused by `classify_kani_run` (bounded_kani_corpus.rs:437). The cover's empty
  valuation is the assertion's too, so Kani prints only one playback.
- Corpus cover removed: `tc_023_kani_executes_the_generated_arithmetic_harness` fails, with
  `Inconclusive { MissingCoverSummary }` where `Verified` was expected.
- V1 cover removed: `tc_025_real_kani...` fails at its first arm, again with
  `Inconclusive { MissingCoverSummary }` where `Verified` was expected.

## Gates

`make ci` on a clean target dir (`CARGO_TARGET_DIR=target-ci`) exits 0. It runs fmt-check, spec,
lint, msrv, deny, audit-unsafe, rustdoc and test (161, 297 with 21 ignored, and 1, both under
msrv and under stable). The only spec warning is the old FR-017 line 152 EARS warning. The Kani
lane passes 9 of 9 (see Method).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `gated_item_end` ends a `#[cfg(test)]` item at its first top-level `;`. A test item with a `;` before its body (`fn t() -> [u8; 2] { .. }`, `const X: [u8; 2] = ..;`) therefore leaks its literals into the scan. Reproduced: the scan fails falsely on a test-only literal. The error is in the safe direction but noisy, and the doc comment says the rule holds for every test function. | tests/it/layout.rs:241-263 |
| FND-002 | low | `guard_sources` maps every kind other than precondition and postcondition to "v1 contract invariant" through a `_` arm. If `supported_contract_harnesses` ever returned another kind, it would get the invariant label, and the family-set equality could pass with the real invariant harness missing. | tests/it/kani_obligations.rs:396-400 |

## Verdict

The two template changes are correct and minimal. Each harness now ends with one cover after its
last check. The real Kani outcomes match the claims: V1 bundle `Verified`, cover-stripped
`MissingCoverSummary`, unsatisfiable requires `CoverUnsatisfied { 0, 1 }`, broken ensures
`Falsified`, corpus true cases `Verified`, false graph and collection `Falsified` with an
empty-valued playback, and cover-stripped corpus `MissingCoverSummary`. The exemplar keeps its
oracle text, its decoded counterexample `[(amount_current, 999)]` and its unwind behaviour. The
check set and the transcript gain exactly one cover property and one cover playback block, which
production skips. Downstream, an unmutated V1 bundle now classifies `Verified` instead of
`Inconclusive { MissingCoverSummary }`, which is the intent of the AC. A QSL assertion that counts
checks or playback blocks, or that expects the old inconclusive result, would see the change.
Every mutation of the template or its placement is caught. Both findings are low and are in test
helpers.

## Dispositions

Round 1, checked at 86714933f2f9907cbf76dfa8efa4b58ba27110fa. That is one fix commit over 20b3dff, and it does not change `src/`. Every mutation below ran in a separate worktree with its own target dir.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 86714933f2f9907cbf76dfa8efa4b58ba27110fa. `gated_item_end` now counts `{`, `[` and `(` as depth, and returns only on a `}` at depth 0. My leak reproduction (`#[cfg(test)] fn t() -> [u8; 2] { let _ = "#[kani::proof]"; .. }` in a new file) now passes the scan. With the fix reverted on the copy, the new `tc_025_the_literal_scan_skips_a_test_item_with_an_array_type` fails (left `["\"#[kani::proof]\"", "\"kept\""]`). |
| FND-002 | fixed | 86714933f2f9907cbf76dfa8efa4b58ba27110fa. The match is now exhaustive: `ObligationKind::Invariant => "v1 contract invariant"`, and `ObligationKind::Frame => "unexpected frame harness"`, a label no `FAMILIES` row lists, so the family-set equality fails on it. |

Round 1 gates at 8671493:

- `make ci` on a clean target dir (`target-ci`) exits 0. Tests: 161, 299 with 21 ignored, and 1, both under msrv and under stable.
- `src/` is unchanged from 20b3dff, so the Kani lane result (9 of 9) and the four Kani-lane mutations from the review pass still hold.
- Coverage (`quire coverage`, unbacked rows):
  - merge base 0765cc3: 65; head: 65, the identical set.
  - Current main bc47d95, which adds #263: 89. A merge of the head into bc47d95 (a clean merge-tree): 89, the identical set.
  - So the PR adds no unbacked row against either base, and backed rows rise by 6.
