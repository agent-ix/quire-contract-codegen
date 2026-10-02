---
id: "SR-836"
title: "CG PR 233 gap analysis: does ignoring the corpus tests close IR-467"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@8ce1350d5c8cafa39c0ea2fc82af82ac64fb977a; tests/it/bounded_kani_corpus.rs, tests/it/kani_generation.rs (default-lane cargo kani call sites), Makefile kani/test targets, .github/workflows/ci.yml, spec/core/matrix/suites.md SUITE-011, spec/kani/matrix/tests.md TC-023"
---

# SR-836: CG PR 233 gap analysis

## Summary

Ticket: IR-467. PR: agent-ix/quire-contract-codegen#233 at 8ce1350. Planless run, scoped to the
change, re-measured against the ticket. I treated the ticket text as data.

The ticket names the defect "three default-lane tests run real cargo kani; they fail on a host
without Kani". It says default `cargo test` / `make test` needs Kani installed, and asks for
either `#[ignore]` plus the kani lane, or documenting Kani as a hard prerequisite. The PR picks
the ignore route for the three `bounded_kani_corpus` tests and says `Closes IR-467`.

Re-measurement:

- I searched `tests/` and `src/` for `cargo kani` subprocess launches (`.arg("kani")` /
  `args(["kani", ...])`). Two files have them: `tests/it/bounded_kani_corpus.rs` (now ignored)
  and `tests/it/kani_generation.rs`. Three non-ignored `#[test]`s in `kani_generation.rs` launch
  real `cargo kani` and assert success:
  - `numeric_state_bindings_are_normalized_bounded_and_schema_valid` calls `execute_kani` at
    :579 and asserts the proof verifies.
  - `kani_proves_identity_and_prints_numeric_counterexamples` (TC-014) runs
    `cargo kani --version` with `.expect("cargo-kani must be installed ...")` at :862-866, then
    `execute_kani` at :919, :930, :948 and :1012.
  - `kani_executes_the_generated_contract_proof` does the same at :1426-1430 and launches
    `cargo kani` at :1441, :1460 and :1499.
  On a host without Kani, default `cargo test` / `make test` still fails, which is the ticket's
  defect. Once the three corpus tests are ignored, these three are what remains of that defect.
- The PR's "Kani-less" evidence is disproved by its own log. The body says: `cargo test
  --locked --test it` "with a PATH holding only cargo and rustc (no cargo-kani): 249 passed, 0
  failed, 12 ignored". In that log (`cg-ir-467-nokani.log`, lines 261-269) all three
  `kani_generation` tests above report `ok`, after running past 60 s. They can only pass if
  `cargo kani` actually ran. When cargo looks up an external subcommand it searches
  `$CARGO_HOME/bin` as well as `PATH`, and `~/.cargo/bin/cargo-kani` is installed on this host.
  Trimming `PATH` therefore did not remove Kani. The run does not show that the default lane
  works without Kani, and the code shows it does not.
- Lane coverage of the ignored tests: confirmed, see SR-835. `make kani` on 8ce1350 ran all 12
  ignored tests, including the three, and passed.
- Traces: no requirement, matrix row or test removed. TC-023 trace tags are unchanged on the
  three tests.

## Verdict

FAIL as a close of IR-467. The diff is correct as far as it goes. But the ticket's defect
(default lane needs Kani) remains in three more tests, and the PR's evidence that it was fixed
is invalid. Either ignore the three `kani_generation` tests the same way, add them to the
`make kani` filter, and re-measure on a host or environment where cargo cannot find
`cargo-kani` (for example `CARGO_HOME` pointing at a directory with no `cargo-kani` and PATH
trimmed), or take the ticket's other option and state Kani as a hard `make test` prerequisite.
Then correct the PR body's evidence line either way.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Three non-ignored default-lane tests in `kani_generation.rs` still launch real `cargo kani` and assert success, so default `cargo test` / `make test` still fails on a host without Kani. That is IR-467's stated defect, and the PR says `Closes IR-467` | tests/it/kani_generation.rs:579,862-866,1426-1441 |
| FND-002 | high | PR body's "PATH holding only cargo and rustc (no cargo-kani)" evidence is invalid. Cargo resolves external subcommands from `$CARGO_HOME/bin` as well as PATH. The same log shows the three `kani_generation` real-Kani tests passing, so Kani was reachable during the "Kani-less" run | tests/it/kani_generation.rs:862-866 |
