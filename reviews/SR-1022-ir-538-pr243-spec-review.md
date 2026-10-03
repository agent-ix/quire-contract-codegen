---
id: "SR-1022"
title: "CG PR 243 spec review: matrix flip for FR-018-AC-17..19 and FR-014-AC-39"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@0573ebdcff3fc0bca4ad2815961813c5fe8d9943; spec/oracle/matrix/tests.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/functional/FR-018-composite-equality-oracles.md (context), spec/oracle/functional/FR-014-exact-scalar-oracles.md (context); diff origin/main...HEAD, base 4b6e09c"
---

# SR-1022: CG PR 243 spec review: matrix flip for FR-018-AC-17..19 and FR-014-AC-39

## Summary

Ticket: IR-538. PR: agent-ix/quire-contract-codegen#243 at 0573ebd. The spec diff is three changes:

- tests.md flips FR-018-AC-17 through FR-018-AC-19 from Planned to `✅ Covered`.
- tests.md flips FR-014-AC-39 from Planned to `✅ Covered`.
- TC-024 step 11 drops its "Planned until the scan test lands." sentence.

Measured:

- The TC-024 and TC-029 inventory rows already listed AC-39 and AC-17..19 on base. They stay `🚧 Planned` overall because of other planned ACs, which is consistent.
- No other prose still says these ACs are planned (grep for IR-538 and "Planned until" in spec/oracle).
- `quire validate --scope . 'spec/**/*.md' 'reviews/**/*.md'` exits 0 with warnings only. It was run with quire from PATH (0.33.0), because the Makefile's hardcoded path is missing.
- `quire coverage --strict` reports 66 unbacked rows and 0 contradicted, the same 66 as base 4b6e09c, so the counts hold.

The FR text was merged in #240 and is in this review only as context for the two wording gaps that the mutation probes in SR-1021 exposed.

Conflict check: PR #242 (IR-540, fc8dae4, base c2099b8) already conflicts with current main in tests.md. Its TC-031 inventory line collides with #240's TC-024/TC-029 row edits at lines 79-81. #243 changes only rows 33-34, and #242 changes rows 40-41 and the TC-031 inventory line. Those are disjoint, and no src file overlaps: #242 touches function/mod.rs, and its added lines carry no panic token, so #243's scan stays green. #242 needs a rebase whichever lands first, but #243 adds no new conflict.

## Verdict

The flips are consistent and the counts are true, apart from the AC-18 sibling clause, which SR-1021 FND-001 says should not read Covered until it is really tested. Two low wording gaps are in the merged ACs. Neither blocks this PR; both are candidates for a follow-up spec edit.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-018-AC-17 constrains where reconstruction happens and how its `Err` is handled, but not what a helper does on failure. A silent widen inside `rebuild_cardinality` (`.or_else(\|_\| CardinalityBound::new(0, u64::MAX))`) and a wrong variant in `rebuild_text` both pass every test (SR-1021 M5, M8). This is the "silently widens the compared type" outcome the AC's negative case names; the generator code does not do this today. | spec/oracle/functional/FR-018-composite-equality-oracles.md:273, spec/oracle/functional/FR-018-composite-equality-oracles.md:307 |
| FND-002 | low | The banned-token lists in FR-018-AC-19, FR-018-AC-17 and FR-014-AC-39 are substring spellings. They miss path forms (`.map(Option::unwrap)`, `Result::expect(x, ..)`), whitespace before the paren (`.unwrap ()`), and a bare `abort()` after `use std::process::abort`. Any of these is a panic site that a conforming scan passes. | spec/oracle/functional/FR-018-composite-equality-oracles.md:275, spec/oracle/functional/FR-014-exact-scalar-oracles.md:355 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The panic-token definition in FR-018-AC-19, FR-014-AC-39 and the Behavior bullets bans only `unwrap`, `expect` and `unwrap_unchecked`. It misses other std panicking calls: `expect_err` and `unwrap_err` (both panic on `Ok`), `std::panic::panic_any` and `resume_unwind`. The scanner self-test lists `x.expect_err(e)` as an allowed lookalike. Probe P4 put `x.expect_err("p")` in the generator, and it passed every scan. | spec/oracle/functional/FR-018-composite-equality-oracles.md:279, spec/oracle/functional/FR-014-exact-scalar-oracles.md:356, tests/common/panic_scan.rs:10-13, tests/it/exact_scalar_generation.rs:2157 |
| FND-004 | low | AC-19 and AC-39 say "the identifier `abort` unless it is a method (`process::abort`, or a bare `abort` after an import)". The parenthetical attaches to "method", so the sentence reads as if `process::abort` were the allowed form. The scan does the opposite: it bans those two forms and allows `.abort`. The Behavior bullet ("`abort` other than as a method") and the scan's doc comment are clear. | spec/oracle/functional/FR-018-composite-equality-oracles.md:279, spec/oracle/functional/FR-014-exact-scalar-oracles.md:356 |

## Dispositions

Round 1 was reviewed at d55fdd404a2a7f86e6f62f0ee1d4816e90ceec46.

- The tightened AC-17, AC-19 and AC-39 wording matches the tests: the helper-variant clause, the identifier-based token list, and `#[cfg(test)]` removal "wherever the item sits". So do the Behavior bullets, the mutation rows, TC-024 step 11 and TC-029 steps 9 and 11.
- M5, M8, P1, P2 and P3 are killed (SR-1021 dispositions).
- `quire validate` exits 0. `quire coverage --strict` reports 66.
- The new FND-003 and FND-004 are listed above.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | d55fdd4 |
| FND-002 | fixed | d55fdd4 |

Round 2 was reviewed at 58a6050e9d163c2c03d76c2951f1831121b4d766 (delta d55fdd4..58a6050):

- **FND-003.** `expect_err`, `unwrap_err`, `unwrap_err_unchecked`, `panic_any` and `resume_unwind` are now banned identifiers. The change is consistent across the FR-014 and FR-018 Behavior bullets, AC-19, AC-39, TC-024 step 11, TC-029 step 11, and the scanner. The self-test names each one, and its lookalike list now holds `unwrap_or_default` instead of `expect_err`.
  - Probe P4 (`x.expect_err("p")` in the equality generator) is now killed by tc_029_ac19.
  - The full suite passes, so the generator sources and generated crates carry none of the new identifiers: no false positive.
- **FND-004.** AC-19 and AC-39 now read "`abort` anywhere except as a method call (`.abort()`), so `process::abort` and a bare `abort` after an import are both counted". That matches the Behavior bullets and the scan.
- **Nothing new in the delta.**

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-003 | fixed | 58a6050 |
| FND-004 | fixed | 58a6050 |
