---
id: "SR-1489"
title: "CG PR 274 gap analysis: FR-030, TC-041 and the FR-029 rows against ir_outcome_terminal_value and its tests"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@944ef8f3909fabb3e8e7c247b7c904b363bbc5ac; spec/kani/functional/FR-030-ir-outcome-terminal-map.md, spec/kani/matrix/{TC-041,tests}.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md (rows only), src/kani/terminal.rs, tests/it/terminal_map.rs (diff 48a517b...944ef8f)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-030
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-041
    type: references
---

# SR-1489: CG PR 274 gap analysis

## Summary

Ticket: IR-465, with IR-358. Plan completion: not assessed.

I measured coverage with quire 0.36.1 on both trees rather than taking it from the PR:

- `quire coverage --strict` reports 71 unbacked rows on `main` 48a517b and 56 on the head, with 0
  contradicted statuses on both. The 15 rows that drop out are the 13 FR-030 verification rows,
  the FR-030 functional-coverage row and the TC-041 traces-to row. No other row changes, apart
  from line-number shifts.
- `quire matrix --format tsv`, diffed per criterion: exactly FR-030-AC-1 to AC-6, AC-8, AC-9 and
  AC-11 to AC-13 move from `untagged` to `tagged`. Each binds to one test whose `Trace:` line names
  that AC. AC-7 gains its `syn` inspection test as binder, and its status stays
  `method-without-symbol` because it is verified by Inspection. FR-030-AC-10 stays `untagged`, and
  so do FR-029-AC-3 and AC-10. No FR-029 row changes status.
- Is FR-030-AC-10 made to look backed? It is not. `--strict` no longer lists the AC-10
  verification row, because a test tagged with the bare `TC-041` exists. That is how the tool
  counts, as the author disclosed, and it is the same on `main` for FR-029-AC-3 and AC-10 under
  TC-040. Nothing the PR wrote claims AC-10. The CG-defect test is tagged `Trace: TC-041` only,
  and its doc says AC-10 stays planned. The kani `tests.md` splits AC-10 into its own
  `🚧 Planned` row. The TC-041 index row says "every criterion but FR-030-AC-10 is covered". FR-030
  Status and TC-041 Status both say planned, for the right reason: `qsl-replay` at c8f0c28 does
  not re-export `InternalFault`, which I checked in `qsl-replay/src/lib.rs`. So the honest
  strict-unbacked count is 57, as the author says.
- Reverse gap. The one new public item, `ir_outcome_terminal_value`, is owned by FR-030 and listed
  in interface-001. Nothing in the diff is unowned except the pairing refusal (FND-001).
- Stubs and inflation. No `todo!`, `unimplemented!`, `#[ignore]` or tautological assertion is
  added. Every claimed-backed AC's test fails on a plausible mutant (SR-1488, fourteen mutants,
  all killed).
- Semantic review: covered by the per-AC mutant pass and by reading each test against its AC
  (SR-1488). The caller's brief asked for test-oracle strength, so I ran no separate LLM pass.

Each AC claimed backed, against its test:

- AC-1: all ten kinds. Each non-counterexample kind with a settlement is refused with
  `UnexpectedSettlement`, a counterexample with none with `MissingSettlement`, and 9 + 6 values are
  produced.
- AC-2: three causes, each with its own registered code, plus `Refused` with the unregistered
  `kani_corpus_identity_collision` (whose `is_registered()` is asserted false). Each is equal to
  `Declined { cause, code: DeclineCode::Std001(code) }`.
- AC-3, AC-4, AC-5, AC-8, AC-9: exact-value assertions. AC-5 passes count 3 and expects 0. AC-8
  asserts both `kani_backend_absent` and a third code.
- AC-6: 15 values, none `Tested`.
- AC-7: a `syn` inspection over the function finds exactly one `match` over a two-tuple, no `_`
  or binding catch-all, and all ten kinds named (M9 killed).
- AC-11, AC-12, AC-13: these reuse FR-029's setup-refusal and repeated-identity fixtures
  unchanged, with the map swapped.

## Verdict

CONDITIONAL. The AC claims are not inflated. Twelve FR-030 criteria are backed by real, failable
tagged tests, and AC-10 is honestly planned. The finding is that the function returns more than
FR-030 states: it has a typed pairing refusal that no FR-030 Behavior line or AC specifies. FR-029
fixed the same gap with AC-15 (SR-1484 FND-002).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `ir_outcome_terminal_value` returns `Result<TerminalValue, TerminalPairError>`, refusing a `Counterexample` with no settlement (`MissingSettlement`) and any other kind with one (`UnexpectedSettlement`). FR-030's Outputs say "One `qsl_replay::TerminalValue`". Inputs says only that "No other kind takes a settlement", and no Behavior line or AC states the refusal. `tc_041_every_expressible_pair_maps_to_one_value` asserts both errors under an FR-030-AC-1 tag, although AC-1 is about expressible pairs mapping to a value. interface-001 names the error, so FR-030 is the document that is short. Fix: add `TerminalPairError` to FR-030's Outputs, add a Behavior shall-statement, and add an AC (as FR-029-AC-15 does) with a TC-041 step, and tag the assertions to it. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:44-57,59-118; src/kani/terminal.rs:124-143; tests/it/terminal_map.rs:617-638 |

## Coverage

- `quire coverage --strict`: 56 unbacked on the head (71 on `main`), 0 contradicted. The honest
  count is 57 once FR-030-AC-10 is counted as planned.
- FR-030: 12 of 13 criteria tagged (AC-7 by inspection). AC-10 is planned.
- FR-029: unchanged. AC-3 and AC-10 are planned, the rest tagged.
- Semantic review: replaced by the mutation pass in SR-1488.
- Plan completion: not assessed

## Dispositions

Round 1, reviewed at 7bbe22f4a4784006db6c461db3b3742677b6bda2 (fix commit 7bbe22f on 944ef8f).

Re-measured with quire 0.36.1:

- `quire coverage --strict`: 56 unbacked rows and 0 contradicted. The honest count is still 57
  with FR-030-AC-10 counted as planned.
- `quire matrix`: FR-030 AC-1 to 9 and 11 to 14 are tagged (AC-7 by inspection). AC-10 is
  untagged and planned in `tests.md`, FR-030 Status and TC-041 Status. No test is tagged AC-10.

FR-030 now states the pairing refusal in Outputs, in a Behavior shall-statement and in the new
AC-14. TC-041 adds step 13 and expected result 13, and both matrix rows add AC-14.
`tc_041_a_settlement_accompanies_a_counterexample_only` (Trace: FR-030-AC-14, TC-041) asserts
three things. A counterexample with no settlement is `Err(MissingSettlement)`. A counterexample
with a reproduced replay is `Ok(Refuted)`. Each of the nine other kinds with a settlement is
`Err(UnexpectedSettlement)` and maps with none.

Mutants of `ir_outcome_terminal_value`, run through `cargo test --test it terminal_map::tc_041`:

- Swapping the two error variants: killed by the AC-14 test.
- A counterexample with no settlement mapping to `Failed`: killed by the AC-14 test.
- `Proved` accepting a settlement: killed by the AC-14 test and the AC-7 inspection test.
- `Inconclusive` accepting a settlement: killed by both.
- `Cancelled` accepting a settlement: killed by both.

The AC-1 test lost its pairing assertions, but they moved word for word into the AC-14 test.
AC-1 still builds every expressible pair through `map_ir` and `map_ir_settled`, and both
`expect` an `Ok`, so a valid pair that is refused still fails AC-1. No other test changed.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 7bbe22f: FR-030 Outputs list `TerminalPairError`, a Behavior shall-statement states both refusals, and FR-030-AC-14 is added with TC-041 step and expected result 13 and the matrix rows. The new AC-14 test asserts both errors, and the five pairing mutants are all killed |
