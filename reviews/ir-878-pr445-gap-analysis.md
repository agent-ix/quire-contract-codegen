---
id: SR-12873
title: IR878 PR445 gap-analysis
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-contract-codegen; PR445 head 2eeab5d9c7654a6f39846f768ded0b0792a855a1 vs merge-base cb96317e; spec/strategy/, tests/it/strategy_generation.rs
review_set: subset
---

## Summary

Ticket: IR-878. Planless quoin gap analysis of quire-contract-codegen PR 445, scoped to the strict-coverage delta and the one criterion the PR newly backs, FR-002-AC-3.

## Verdict

PASS for this delta with one low finding. Fresh strict `quire coverage --json --strict` runs (quire 0.36.2, engine 0.50.2) confirm the PR's claim. On the merge-base, backed is 526 of 904 with 226 unbacked rows. On the head, backed is 527 of 904 with 225 unbacked rows. status_lies is 0 on both. The only row-level differences are these four. FR-002-AC-3 leaves `unbacked_rows` and leaves `no_symbol_rows` (declared test type `Analysis`, 48 to 47). The FR-002 acceptance-criterion group goes from 5 to 6 backed of 18. The FR-002-AC-3 minted target flips to backed. The suspicions (7), unmatched_tags (123) and diagnostics (10) counts are unchanged. No other criterion was touched, and no criterion was weakened.

The new binding is genuine evidence. The tagged test generates the real correlated-offset strategy, compiles it with denied warnings, and draws 64 trees from a zero-local-reject runner. That fails on any filter rejection, as SR-12872 shows. The test also keeps its static `prop_filter` absence assertion. The boundary (`proptest::sample::select` over a fixed list) and no-event (`Just`) correlated bodies are filter-free by construction (src/strategy/campaign.rs:559-570, 716-731), so testing the broad body alone is a reasonable reading of "supported correlated constraints". The diff adds no production code, so it has no unowned behavior.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-002-AC-3 still declares Verification `Analysis`, a no-source-symbol method, yet it is now backed by Test TC-004. That test is already listed under TC-004 in spec/strategy/matrix/tests.md. The FR's acceptance-criteria table and the matrix disagree on how AC-3 is verified, so a reader of the FR does not learn that a test backs it. | spec/strategy/functional/FR-002-tristate-proptest.md:87 |

## Coverage

Plan completion: not assessed

Strict coverage was measured on both the head worktree and a detached merge-base worktree, and the two JSON reports were compared record by record across every list field. The criterion text, the tagged test body, the generator source for every correlated campaign arm, and the TC-004 matrix row were read directly. The full suite and Kani were not run, by dispatch: the author's gate and the lead's final gate own them. The scoped `strategy_generation::` tests were run on the frozen head: 13 passed, 0 failed.

## Examined units

```yaml
scope:
- id: FR-002-AC-3
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Supported correlated constraints do not rely on improbable filtering.
- id: FR-002
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: context_only
  excerpt: When executable pre/post contracts are lowered, the generator shall emit entry-point harnesses and proptest strategies that preserve pass, failed postcondition, and rejected precondition outcomes.
- id: TC-004
  path: spec/strategy/matrix/tests.md
  role: context_only
  excerpt: Preserve shaped proptest strategies
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Fix commit 2f7eb2bd changes the FR-002-AC-3 Verification cell to `Test (TC-004)`, which matches the TC-004 row in spec/strategy/matrix/tests.md. No other spec/ or plan/ document records AC-3 as Analysis. `quoin advise` reports authored Test, mismatch false. Strict coverage at 2f7eb2bd: 527/904 backed, 225 unbacked, 0 contradicted. |
