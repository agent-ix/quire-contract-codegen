---
id: "SR-1007"
title: "CG PR 240 gap analysis: the FR-018 panic ban against the generators' code"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@daa93aedc7a5e88c9a43b4114cb6010a4ede7d38; spec/oracle/functional/FR-018-composite-equality-oracles.md (FR-018-AC-17, FR-018-AC-18), spec/oracle/functional/FR-021-function-application-oracles.md (FR-021-AC-21, context), src/oracle/equality/mod.rs, src/oracle/scalar/mod.rs, src/oracle/function/mod.rs, src/kani/generate/scalar.rs, src/kani/generate/corpus/bounded_kani_corpus.rs, src/routed/capability.rs; diff origin/main...HEAD, base 85b8114"
---

# SR-1007: CG PR 240 gap analysis

## Summary

Ticket: IR-538. PR: agent-ix/quire-contract-codegen#240 at daa93ae. Plan completion: not assessed.

The PR is spec-only. This pass checks that the new ACs describe the code truthfully, and that the
matrix status matches it. It also judges whether the panic sites outside FR-018 fall under this
AC.

Spec against code:

- FR-018-AC-17/18 are 🚧 Planned, and the tests.md row says the scan test is not written and the
  equality generator still has six `.expect` calls and two `unreachable!` arms. All of that is
  true at head (equality/mod.rs 1414, 1559, 1577, 1588, 1603, 1611; 1574, 1595). The status is
  honest.
- The scalar generator (`src/oracle/scalar/mod.rs`) has no production panic site. Every hit
  comes after `#[cfg(test)]` at 3343.
- The `//!` doc comment at equality/mod.rs:50 mentions `.expect(` but is a comment line, so the
  AC-18 scan as defined does not count it.

A crate-wide grep for production panic sites, before each file's first `#[cfg(test)]` and with
comment lines excluded, found these sites outside FR-018's two files. The author named some of
them; I checked each one myself:

- `src/oracle/function/mod.rs`: 901 `.unwrap()`, and 958, 960 and 999 `.expect(..)`. All four
  run at generation time in FR-021's generator, and the file has no test module. FR-021-AC-21
  bans only `unreachable!`, `panic!`, `todo!` and `unimplemented!` in that file, not
  `.unwrap(`/`.expect(`. The emitted function crate itself has no panic token.
- `src/kani/generate/scalar.rs`: 120, 183 and 199 `unreachable!` at generation time, in Kani
  lowering. The author's line 659 falls inside `#[cfg(test)]` (which starts at 452), so it is a
  test site, not a production one.
- `src/routed/capability.rs`: 396 `unreachable!` in routed capability classification.
- Missing from the author's list: `src/kani/generate/corpus/bounded_kani_corpus.rs` 349 and 610
  `.expect(..)` (corpus digest and graph serialization), and `src/evidence/bound_coverage.rs`
  446 and 480 `.expect(..)` (evidence, not generation).

Judgement on scope. The ticket asks to "extend FR-018 to ban ... in all emitted source and at
generation time". That covers FR-018's generator only. None of the sites above belongs to
FR-018's code, so none is in scope for AC-17/18, and the ACs correctly stop at
equality/scalar. But `function/mod.rs` is in scope of FR-021-AC-21, which is narrower than the
rule FR-018-AC-18 now sets. The Kani and routed sites belong to their own FRs, which have no
such ban.

## Verdict

The spec matches the code. The planned status and the matrix note are accurate. One medium
finding records the policy gap across sibling FRs. It is out of this PR's files, and no ticket
is filed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Sibling generators are held to a weaker panic rule than the one FR-018 now sets, and no spec owns their remaining sites. FR-018-AC-18 bans `.unwrap(`/`.expect(` in the equality and scalar generator code. FR-021-AC-21 bans only the four macros in `src/oracle/function/mod.rs`, which still has generation-time `.unwrap()` at 901 and `.expect(..)` at 958, 960 and 999. Kani lowering (`src/kani/generate/scalar.rs` 120, 183, 199 `unreachable!`), routed capability (`src/routed/capability.rs:396` `unreachable!`) and the Kani corpus (`src/kani/generate/corpus/bounded_kani_corpus.rs` 349, 610 `.expect`) have no such AC at all. These are outside this PR and do not block it. The lead should decide whether FR-021-AC-21 and the Kani and routed FRs adopt the same ban | spec/oracle/functional/FR-021-function-application-oracles.md:247, src/oracle/function/mod.rs:901, src/oracle/function/mod.rs:958-960, src/oracle/function/mod.rs:999, src/kani/generate/scalar.rs:120, src/kani/generate/scalar.rs:183, src/kani/generate/scalar.rs:199, src/routed/capability.rs:396, src/kani/generate/corpus/bounded_kani_corpus.rs:349, src/kani/generate/corpus/bounded_kani_corpus.rs:610 |

## Dispositions

Round 1, reviewed at f2f2786a5f17533700ba370beb088b5d35cf47bc (fix-round delta daa93ae..f2f2786).
The delta does not touch FR-021, the Kani FRs or the routed FRs. That is correct for this PR:
the finding concerns sibling generators outside IR-538's FR-018/FR-014 scope.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | deferred | outside PR #240's scope: the panic sites are in FR-021's, Kani's and routed's generators, which IR-538 does not cover; recorded for the lead, no ticket filed per the reviewer brief |
