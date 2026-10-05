---
id: SR-1602
title: "IR-631 spec review (base checklist): FR-032, TC-047 and the replay registry row"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen#290; spec/replay/functional/FR-032-routed-scalar-replay-binding.md, spec/replay/matrix/TC-047-routed-scalar-replay-binding.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-047
    type: reviews
---

# SR-1602: IR-631 spec review, base checklist

## Summary

Ticket: IR-631. PR: quire-contract-codegen#290 (closed only to stay under the open-PR cap; frozen candidate). Reviewer: claude-opus-5-5, run d75eb678-590e-4bb0-b33c-ca7f9c5bde50. This review applied the base checklist to the new FR-032, TC-047 and the edited Counterexample replay registry row in spec/spec.md. `quire validate --scope . "spec/**/*.md"` passes (quire 0.36.1, engine 0.50.1). One high finding: FR-032-AC-8's result-range-refusal clause can never fire on the fixture it names. One low finding: the AC cells do not carry the repository's inline PLANNED marker.

## Method

I read FR-032 and TC-047 in full and re-measured their claims against the candidate's source (`src/kani/generate/scalar.rs`, `src/kani/identity.rs`, `src/kani/terminal.rs`, `src/replay/witness.rs`, `tests/it/routed_generation.rs`). I also checked QSL's `qsl-replay` at the revision CG's lockfile resolves and on QSL's main branch. Checked: ID formats, verification cells (method only, no TC id), error paths, boundary criteria, cross-references, and hashes/pins.

## Verdict

**FAIL**: one high finding (FND-001). The following parts of the review were clean:
- ID formats are correct.
- Verification cells name only `Test`.
- Every cross-link resolves.
- No version pin or file-tracking record is introduced.
- The "canonical proof-content identity" of the proved generated oracle is the one digest the repository's CLAUDE.md permits: a canonical identity that binds a proof to the exact content it proved.
- Every source fact in the prerequisites section matches the source:
  - the parity assertion at scalar.rs:427-431;
  - singleton literal bounds and result-range fallback at scalar.rs:248-249;
  - the `u64::MAX` meter at scalar.rs:412;
  - `CallSiteSelection` sealed with five impls (function, operation, clause, field, population);
  - `NotAPredicate` refusal for non-Boolean functions;
  - no operator-level arm in QSL.
- The planned and gated status is stated accurately, and no implementation is claimed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-032-AC-8 requires that an "expected exact result-range refusal is not reported as a source violation" on the real QSL-emitted `x + 1` over `Int[0,9]` case. That package narrows into `Int[0, 10]` (FR-022-AC-15). Its operand ranges `[0,9]` and `[1,1]` reach only `[1,10]`, which lies inside `[0,10]`, so no admitted operand vector has an out-of-range exact result, with or without the arithmetic mutation. The clause passes vacuously, and TC-047 step 2's "unmutated out-of-result-range value" cannot occur. Fix: name a fixture whose reachable results exceed its result bound, or remove the clause. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:140, spec/replay/matrix/TC-047-routed-scalar-replay-binding.md:27, tests/it/routed_generation.rs:675 |
| FND-002 | low | FR-032-AC-1 to AC-8 carry no inline status in their criterion cells. The repository convention puts it there: FR-024-AC-31 reads "PLANNED (IR-624) ..." and FR-015-AC-41 reads "PLANNED (IR-489)". FR-032 states Planned and Gated only in prose at lines 183-186, so a reader of the AC table or of the computed matrix sees eight untagged criteria with no status. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:133, spec/replay/functional/FR-032-routed-scalar-replay-binding.md:183 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | TC-047 step 2 says "Select `(600,600)`" for the harness-defect agreement control, but the playback is chosen by Kani, not by the test. A harness-assertion mutation that rejects every correct refusal can be falsified at any out-of-range operand pair, and FR-032-AC-8 says the observed exact result is 1200. One reader writes a test that depends on which counterexample the solver picks; another accepts any out-of-range playback. Fix: make the seeded mutation falsify only at `(600,600)`, or have AC-8 and TC-047 assert any admitted operand pair whose exact result is outside `[-1000,1000]`, with `(600,600)` as the documented reachability witness only. | spec/replay/matrix/TC-047-routed-scalar-replay-binding.md:33, spec/replay/functional/FR-032-routed-scalar-replay-binding.md:186 |

## Dispositions

Round 1 re-check of the fix-round candidate of PR #290. The fixing commit is recorded in the private ticket marker.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | FR-032-AC-8 no longer claims a refusal for the increment; FR-032 states its `Int[0,10]` result supplies no refusal witness. The refusal case moved to the bounded-addition fixture: operands and result `[-1000,1000]` (`integer_add()`, corpus `INT`), where `(600,600)` is admitted and the exact sum 1200 is outside the result bound. That fixture is admissible: CG's renderer accepts it (reachable `[-2000,2000]` overlaps `[-1000,1000]`), and QSL FR-057 admits a narrowing conversion wrapping a scalar application with the application's result bound as the target. |
| FND-002 | fixed | Every FR-032 criterion cell now opens with `PLANNED (IR-631)`, plus `/ GATED (QSL-641)` wherever the admitted route is involved, matching the FR-024-AC-31 and FR-015-AC-41 convention. |

Round 2 re-check of the next fix-round candidate of PR #290, covering FND-003 and regressions in the two-file fix. The fixing commit is recorded in the private ticket marker.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-003 | fixed | FR-032-AC-8, FR-032's fixture paragraph and TC-047 step 2 no longer select `(600,600)`; they call it illustrative and say Kani may choose another pair. TC-047 step 2 seeds a harness-assertion mutation that rejects only correct refusals outside the result range, so the only falsifiers are admitted pairs whose exact sum is out of range. The test decodes the actual retained pair, asserts both operands are admitted and the sum is outside the range, and derives the expected exact result from that pair. TC-047's AC-8 row names "assume `(600,600)` instead of reading actual playback" as a mutation that must fail. No regression: no other text changed, `quire validate` passes, and FR-032 has no EARS warning. |
