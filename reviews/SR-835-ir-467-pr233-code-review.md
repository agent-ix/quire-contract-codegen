---
id: "SR-835"
title: "CG PR 233 code review (with rust-review lane): ignore the real-cargo-kani corpus tests"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@8ce1350d5c8cafa39c0ea2fc82af82ac64fb977a; Makefile (kani target and its comment), tests/it/bounded_kani_corpus.rs (three tc_023_kani_* tests), tests/it/main.rs (module doc, context)"
---

# SR-835: CG PR 233 code review

## Summary

Ticket: IR-467. PR: agent-ix/quire-contract-codegen#233 at 8ce1350, a single commit over
origin/main. The diff is 3 `#[ignore = "runs cargo kani; make kani"]` attributes and one
`bounded_kani_corpus` filter added to the `make kani` libtest filter list, plus the comment
above that target. The `rust-review` lane is folded into this file.

What I checked:

- Lane routing. `tests/it/main.rs` declares 25 modules. Only `bounded_kani_corpus` contains the
  substring `bounded_kani_corpus`, so the new filter selects that module and nothing else. The
  module's only `#[ignore]` tests are the three this PR adds, so `--ignored` plus the filter runs
  exactly those three.
- Count reconciliation. The repo has 12 `#[ignore]` test attributes in total: kani_obligations 4,
  kani_obligations_state_frame 3 (picked up by the `kani_obligations` substring filter),
  kani_witness_join 1, skeleton_spine 1, bounded_kani_corpus 3. The coder's kani log
  (`cg-ir-467-kani.log`, `head=8ce1350... exit=0`) shows `12 passed; 0 failed; 0 ignored; 249
  filtered out`, with all three `bounded_kani_corpus::tc_023_kani_* ... ok`. The default lane
  log shows `249 passed; 12 ignored`. The ignored tests are run by `make kani` and are not dropped
  from every gate. The log is consistent with the code, so I did not rerun `make kani`.
- Traces. The three tests keep their `/// Trace: TC-023.` tags. No requirement, matrix row or
  test is removed. SUITE-011 (`make kani`, spec/core/matrix/suites.md:18) is the lane named for
  real-prover runs. TC-023 binds no command.
- Rust idiom. `#[ignore = "reason"]` is the stable form and the reason is stated. No
  `unsafe`, panics, conversions or async code changed. The test bodies are unchanged. The falsify
  case asserts a classified Falsified outcome. The two verify cases assert `cargo kani` exit
  success, which is a real oracle for "the generated proof verifies".
- CI. `.github/workflows/ci.yml` runs plain `cargo test` and does not install Kani. Ignoring
  these tests stops them failing there. `make kani` is not a CI lane, and that is unchanged.
- PR title: "Ignore the three real-cargo-kani corpus tests in the default lane" has no bare
  ticket id. The body ends with `Closes IR-467`.

## Verdict

The diff is correct for the three tests it touches. The Makefile filter is right and the
ignored tests run in `make kani`. Two low findings below. The higher-severity problems with this
PR (ticket intent not met, evidence claim false) are in the gap analysis, SR-836.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `tests/it/main.rs` module doc still says the `kani` target runs `cargo test --test it kani_obligations -- --ignored --test-threads=1`. The target now passes four filters after `--`, and the PR added one of them without updating the doc that points readers at the target | tests/it/main.rs:12-15 |
| FND-002 | low | The new ignore reason `"runs cargo kani; make kani"` uses different wording from the 8 sibling real-prover tests in kani_obligations, kani_obligations_state_frame and skeleton_spine (`"kani lane: run serially through \`make kani\`"`). A grep for the lane's reason string misses these three | tests/it/bounded_kani_corpus.rs:211,264,316 |

## Dispositions

Round 1, reviewed at 13ec9a62cfeda68d3bb15200cc4a66c477de016c (rebased onto main 1629715).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | bc32ac7: `tests/it/main.rs:14-16` now names `cargo test --test it -- --ignored --test-threads=1` with all five filters, matching the Makefile `kani` target |
| FND-002 | fixed | bc32ac7: all six new ignores read `kani lane: run serially through \`make kani\``, the same text as the 8 siblings (kani_witness_join's longer pre-existing wording is unchanged and out of scope) |
