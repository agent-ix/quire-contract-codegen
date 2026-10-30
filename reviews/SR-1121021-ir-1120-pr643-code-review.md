---
id: SR-1121021
title: "IR-1120 CG #643 code-review re-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; PR #643 head 3f40b53ab057c52d72fdf49ca29faf2bcef89572; code-review over changed SMT and FR-045 files"
review_set: subset
---

## Summary

Independent static re-review of CG #643 against FR-045 AC4–7 and TC-059, with AC2 status behavior checked for the shared parser. The production validator and proof framing are sound. The new source-backed test does not discriminate rows, because its seven operations encode to the same query.

## Verdict

CONDITIONAL — production code is sound; the source-row test needs distinct per-row queries before it can back FR-045-AC-4.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Prior finding: the generic status parser refused a standalone success notice after a status. Re-checked at this head. | src/routed/smt/mod.rs:326-330 |
| FND-002 | high | The seven operations have empty pre, post and frame and share one receiver, so QSL's FR-315 encoder gives every Step row the same bytes: operation identity enters the script only through those clauses. The per-row equality at line 293 therefore cannot detect a wrong row, and binding `Step { row: sources[row] }` at lines 231-236 should survive it. This result comes from reading the source; no test was run to confirm it. Same root as SR-1121022 FND-001. | src/routed/smt/refinement_source_tests.rs:63-66; src/routed/smt/refinement_source_tests.rs:231-236; src/routed/smt/refinement_source_tests.rs:293 |

## Coverage

Static read of the full PR diff, fix diff, `src/routed/smt/{mod,refinement,refinement_source_tests}.rs`, and the QSL 93fcdc1 checkout (`qsl-eval/src/smt/{refinement,relation}.rs`, `qsl-replay/src/refinement_query.rs`, `qsl-semantics/src/check/refinement.rs`).

Checked with no finding:

- `RefinementRun` fields are private, so records can only come from `from_execution`.
- Handoff re-derives the proof from the same invocation's stdout.
- Absent, empty, malformed and status-only proof tails stay distinct.
- `unsat\nsuccess\n` is `Unsat` in generic parsing and `MalformedResponse` in proof mode.
- Synthetic controls use a distinct query per role, so association logic is exercised there.
- `Cargo.lock` moves all seven QSL crates to 93fcdc17, and `qsl_replay` exports the facade (`qsl-replay/src/lib.rs:261`). Duplication with CG #654 is not treated as a defect.

This reviewer ran no build, test, Quire validation or `git` command. The focused pass in `cg643-ac4-rerun.log` is the lead's result; the log records neither the head SHA nor the commands. The `/code-review` skill was not loadable here, so its method was applied from the brief. Plan completion: not assessed.

## Examined scope and bindings

```yaml
scope:
- id: FR-045-AC-2
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: A base `Sat` response remains only a candidate for QSL reconstruction and
    replay; base `Unsat` with step `Sat` is not induction; solver `Unknown`, QSL encoding
    refusal, resource stop, missing or malformed response, process failure and a response
    paired with another product, item, instance, depth, kind or query bytes remain
    distinct typed non-proof evidence. A resource stop is handed to FR-182 V-7 as
    incomplete. Every branch retains its available identities and depth, and no CG
    branch claims terminal refutation or certification.
- id: FR-045-AC-4
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: For a refinement with seven step rows, CG retains one `Base` and exactly
    seven typed `Step { row }` records numbered 0 through 6 from the recompiled, canonically
    ordered `steps` array, even when records arrive out of order or the source rows
    were written in another order. Each record binds the refinement item, subjects,
    distinct invocation attempt, role and row, and exact FR-315 `Initial` or `Step
    { row }` query bytes sent on stdin; changing a query byte or exchanging the base
    and a step role refuses the handoff. With zero step rows, the complete shape has
    only `Base`.
- id: FR-045-AC-7
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: Starting from AC-5's collection, making row 3's query or proof field absent
    yields a typed missing-field refusal and no handoff; making the proof present
    but empty preserves the empty bytes in the handoff. An FR-315 refusal, a `Sat`
    or `Unknown` answer, a missing or malformed response, a failed solver invocation
    or a reached limit remains typed non-proof evidence and produces no proof-bearing
    handoff.
bindings: []
```

## Dispositions

| Finding | Outcome | Evidence |
| --- | --- | --- |
| FND-001 | fixed | `valid_solver_tail` accepts a whitespace-delimited `success` (src/routed/smt/mod.rs:326-330) and the proof test asserts `parse_solver_status(b"unsat\nsuccess\n") == Unsat`. `captured_alethe_proof` still refuses that tail, so no proof is taken from it. |
| FND-002 | still-open: give the seven operations distinct encoded behavior (for example, a different frame per operation), assert pairwise-distinct Step queries, and assert `reordered_sources[r] == 6 - first_sources[r]` for every row. |  Static derivation from `qsl-eval/src/smt/relation.rs:471-503,617-633` and `qsl-eval/src/smt/refinement.rs:259-275,562-581`. QSL's own facade test asserts distinct row scripts (`qsl-replay/tests/it/refinement_query_facade.rs:112`); this test does not. |
