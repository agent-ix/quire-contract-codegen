---
id: "SR-1487"
title: "CG PR 273 gap analysis: typed Std001Code adaptation, spec truth and test oracles"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@6c6f5b9a09524c2a5ae60cdb26b68938fd149266; spec/assurance/AD-003-evidence-chain.md, spec/core/functional/interface-001-codegen-api.md, spec/core/non-functional/NFR-005-no-generation-panics.md, spec/kani/functional/FR-030-ir-outcome-terminal-map.md, spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, src/kani/generate/corpus/bounded_kani_corpus.rs, src/kani/classify.rs (diff origin/main...HEAD, merge base e52ff44)"
---

# SR-1487: CG PR 273 gap analysis

## Summary

Ticket: IR-620. PR: agent-ix/quire-contract-codegen#273 at 6c6f5b9. Scoped to the PR diff and to
the spec statements about what it changes. Planless.

`quire coverage --scope <root> --json` (quire 0.36.1, engine 0.50.1) on the base e52ff44 and on
the head: totals identical (287 backed of 421 rows, 378 criteria), 71 unbacked rows on both, 0
status lies on both. The PR adds no unbacked row and removes none.

Spec truth, checked against the code and against IR dec8ade:

- AD-003's new cause-code paragraph: true. `KaniOutcome.code`, `KaniProviderRecord.cause` and the
  `CapabilityDisposition` codes are `Std001Code` at dec8ade; `KANI_VACUOUS_PROOF`,
  `KANI_SOLVER_ABSENT` and `KANI_BACKEND_ABSENT` are registered constants; the four CG-minted codes
  are not in `Std001Code::REGISTERED`.
- interface-001 `generate_bounded_kani_corpus_case` output and semantics: true.
- NFR-005's `render_artifacts` row: true; the NFR-005-AC-1 scan passes.
- FR-016, FR-029 and TC-023 make no statement about the error type or a string code; nothing in
  them became false.
- FR-030's Status paragraph and AD-003's "free-string" clause did become false (FND-001, FND-002).

Test oracles: every pre-existing refusal assertion still checks kind, code, source id and context
as before. The new `tc_023_an_outcome_construction_refusal_is_typed_and_carries_no_outcome`
exercises IR's `non_success` refusal, `From<KaniOutcomeError>`, `code()` and `outcome()`. It
cannot reach `CorpusRefusal::raise`'s `Err` arm, because the private enum makes that arm
unreachable; the test's doc says so. `outcome()` returning `None` for every input is killed by the
integration test's `expect`; a `code()` that returned the requested code is killed by the
`KANI_OUTCOME_INVALID` assertion. `Display` is untested (see SR-1486 FND-002).

## Verdict

CONDITIONAL: two stale spec statements, no matrix or code gap.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-030's paragraph on the `Declined` code says the map's input, "Contract IR's `KaniOutcome` at the revision CG locks, is a kind plus IR's own cause string" and that "`KaniOutcome.code` becomes that type instead of `String`" as a future step (IR-605). This PR moves CG's lock to IR dec8ade, where `KaniOutcome.code` is already `quire_contract_model::Std001Code`. Both statements are now false at the revision CG locks. The PR leaves FR-030 untouched on purpose, but its own lock bump is what falsifies the text | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:158-167 |
| FND-002 | low | AD-003's open-seams row, edited by this PR, says "IR's free-string codes in the family lowerings remain IR-347's". At IR dec8ade the family lowerings spell their codes with `std001_code!` (for example `kani_dispatch_unowned` and `kani_arithmetic_range_invalid` in `src/kani/arithmetic.rs`), so they are typed, form-checked and unregistered, not free strings. What remains is that they have no registered constant | spec/assurance/AD-003-evidence-chain.md:367 |

## Coverage

- Rollup (head): 287 of 421 matrix rows backed; 71 unbacked; 0 status lies. Base identical.
- Reverse gap: the new public `BoundedCorpusError` is owned by interface-001's operation entry and
  by NFR-005's `render_artifacts` row. `CorpusRefusal` is private. No unowned behaviour.
- Stubs: none in the diff.
- Semantic review: not run (not opted in). The scoped spec-truth checks above were done by hand.
- Plan completion: not assessed

## Dispositions

Round 1, reviewed at ac9e82143dfe8eb5c522d256abc3030a576e0fdb (one fix commit, ac9e821, on 6c6f5b9). `quire coverage` on the head: 287 of 421 rows backed, 71 unbacked, 0 status lies, the same as the base. FR-030 Status stays "Planned (Linear IR-465, IR-358)" and TC-041 Status stays "Planned". No FR-030 acceptance criterion row changes, and the matrix files are not in the diff. A grep of `spec/` for "free-string", "cause string", "code string", "instead of `String`", "becomes that type" and "after IR's type merges" finds nothing. Three texts this PR does not touch still say the IR arm waits on "IR-605 and QSL-351": AD-004:785, TC-041:60 and the FR-030 row of the kani `tests.md`. That is still true, because the arm needs both and only IR-605 has merged, so it is not recorded as a finding.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | ac9e821 |
| FND-002 | fixed | ac9e821 |
