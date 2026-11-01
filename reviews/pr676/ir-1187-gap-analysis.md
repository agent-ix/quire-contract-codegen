---
id: SR-2026103037
title: IR-1187 PR 676 gap-analysis review
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-contract-codegen; src/oracle/equality/mod.rs; FR-018-AC-28/30/33;
  PR #676 076119305c70562ddb52215a3c4873aaab327c0d'
review_set: subset
---

## Summary

Ticket: IR-1187. Reviewed the exact PR head and 50,000-record fixture. Computed matrix exposes one missing trace on the new full-package equality assertions.

## Verdict

**CONDITIONAL** — Add the AC-33 trace to the deep test.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The deep test adds full-package identity assertions for FR-018-AC-33 but its trace attribute names only AC-28 and AC-30; add FR-018-AC-33 to that attribute. | src/oracle/equality/mod.rs:3779 |

## Coverage

- FR-018-AC-28 (examined): With a caller-selected type-resolution work limit of at least 100,000 and `cg.equality.source_bytes` raised to the measured flat source size, an admitted acyclic type chain of 100,000 entries reconstructs and generates an oracle with the same result as an equivalent shallow type on a 512 KiB stack; with the same raised source-byte limit, the default type-resolution work limit refuses it at entry 65,537, and a selected work limit one below its entry count refuses it at the first denied entry. No depth setting or fixed depth refusal participates in these runs. PLANNED (IR-511).
- FR-018-AC-30 (examined): The 100,000-entry acyclic closure renders as flat source without a type-depth-sized constructor expression; under the 1,048,576-byte default it returns `SourceTooLarge` naming that limit when the source exceeds it, and under a caller limit equal to its measured source bytes it emits a complete, compilable oracle within the publication artifact and bundle byte limits. One byte less refuses before that byte is appended and publishes no partial artifact. PLANNED (IR-511).
- FR-018-AC-33 (examined): A genuine root-facade compilation of authentic retained source, original dependencies and admitted request limits has equal compiled/context/original package identities and exact full recompiled/original CheckedPackageV2 equality before its context is exposed. An explicitly supplied different context package refuses generation and emitted environment construction before classification or lookup, including primitive-only requests, publishes no prepared value, and cannot evaluate with a completed Boolean or meter charge.
- Changed Rust test (examined): src/oracle/equality/mod.rs:3779-4082.
- Different-context control (examined): tests/it/admitted_enum_equality.rs:553-574.
- Computed matrix: AC-28 and AC-30 bind the deep test; AC-33 binds five other tests.
- Deep test log: 1 passed, 0 ignored, 1179.11 seconds wall time at exact head 4292dc0422c6fe77023f13854daa0e23ba11c06b. Focused different-context control: 1 passed.
- No applicable AssuranceProfile. No production, spec, plan, or CI workflow files changed.
- Plan completion: not assessed. Optional semantic review not requested.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At 2b91b7679e7b1f57e10f5a6f7002b6227c2f2797, the deep test trace attribute includes FR-018-AC-33 alongside AC-28 and AC-30. |

## Final disposition verdict

**PASS at 2b91b7679e7b1f57e10f5a6f7002b6227c2f2797** — the sole gap finding is fixed. The prior 50,000-record run and different-context refusal evidence remain applicable because the fix changes only trace metadata.

## Expanded PR review at 076119305c70562ddb52215a3c4873aaab327c0d

**PASS for the reviewed gap scope** — The computed matrix at the final head binds the deep test to FR-018-AC-28, FR-018-AC-30, and FR-018-AC-33. The new chunk loop follows the ordered composites iterator, appends each generated module's declarations in order, and propagates each typed error before exposing an environment. The 50,000-record deep test exercises the chunk path and independent package-byte assertion; the different-context refusal control remains in the test tree. No new trace, reverse-ownership, stub, or coverage-inflation gap appears in this diff. FND-001's fixed disposition above remains current.

A pre-rebase full deep run passed in 1405.54 s on its own cohort; the final rebased head still requires the code lead's aggregate gate. Plan completion: not assessed. Optional semantic review not requested. No spec or plan file changed, so spec-review methods do not apply.
