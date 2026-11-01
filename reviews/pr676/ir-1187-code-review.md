---
id: SR-2026103036
title: IR-1187 PR 676 code-review review
type: SpecReview
analysis: code-review
scope: 'agent-ix/quire-contract-codegen; src/oracle/equality/mod.rs; FR-018-AC-28/30/33;
  PR #676 076119305c70562ddb52215a3c4873aaab327c0d'
review_set: subset
---

## Summary

Ticket: IR-1187. Reviewed the exact PR head and 50,000-record fixture. The original/context/recompiled byte equality and generated 512 KiB runtime comparison passed; production guard was unchanged.

## Verdict

**PASS** — No Rust behavior defect found.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

- FR-018-AC-28 (examined): With a caller-selected type-resolution work limit of at least 100,000 and `cg.equality.source_bytes` raised to the measured flat source size, an admitted acyclic type chain of 100,000 entries reconstructs and generates an oracle with the same result as an equivalent shallow type on a 512 KiB stack; with the same raised source-byte limit, the default type-resolution work limit refuses it at entry 65,537, and a selected work limit one below its entry count refuses it at the first denied entry. No depth setting or fixed depth refusal participates in these runs. PLANNED (IR-511).
- FR-018-AC-30 (examined): The 100,000-entry acyclic closure renders as flat source without a type-depth-sized constructor expression; under the 1,048,576-byte default it returns `SourceTooLarge` naming that limit when the source exceeds it, and under a caller limit equal to its measured source bytes it emits a complete, compilable oracle within the publication artifact and bundle byte limits. One byte less refuses before that byte is appended and publishes no partial artifact. PLANNED (IR-511).
- FR-018-AC-33 (examined): A genuine root-facade compilation of authentic retained source, original dependencies and admitted request limits has equal compiled/context/original package identities and exact full recompiled/original CheckedPackageV2 equality before its context is exposed. An explicitly supplied different context package refuses generation and emitted environment construction before classification or lookup, including primitive-only requests, publishes no prepared value, and cannot evaluate with a completed Boolean or meter charge.
- Changed Rust test (examined): src/oracle/equality/mod.rs:3779-4082.
- Different-context control (examined): tests/it/admitted_enum_equality.rs:553-574.
- Computed matrix: AC-28 and AC-30 bind the deep test; AC-33 binds five other tests.
- Deep test log: 1 passed, 0 ignored, 1179.11 seconds wall time at exact head 4292dc0422c6fe77023f13854daa0e23ba11c06b. Focused different-context control: 1 passed.
- No applicable AssuranceProfile. No production, spec, plan, or CI workflow files changed.

## Expanded PR review at 076119305c70562ddb52215a3c4873aaab327c0d

**PASS** — Re-reviewed the complete diff against f935378e56cde1e7aaabe3d073669d9e58085bde. The production change bounds generated composite constructors to 64 declarations and calls their typed function pointers in a loop. This retains declaration order, sequential enum-index mutation, and typed error propagation while removing the long sequence of call sites from the aggregator's debug frame. The deep fixture retains 50,000 records, authentic original/context/recompiled full-package-byte checks, and its 512 KiB generation and runtime workers. No new code-review finding arose; the original Findings table remains the clean placeholder.

A pre-rebase deep run passed (1 test, 0 ignored, 1405.54 s wall time; /tmp/ir1187-deep-f3d-loop.log). This is not a full-fixture result at the final rebased head. On the final head, cargo fmt --check and the focused source-limit unit test passed. The final aggregate gate belongs to the code lead. TC-031's escaped-subject fixture change is already in main and is outside this PR diff. No spec or plan file changed, so spec-review methods do not apply.
