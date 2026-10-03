---
id: "SR-1075"
title: "CG PR 244 gap analysis: lowering failed-record ACs against code at aba2403 and IR cbcd790"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@86ac6775de82161ea61b4670f94fe59aa413ee69; spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/functional/FR-021-function-application-oracles.md, src/oracle/scalar/mod.rs, src/oracle/equality/mod.rs, src/oracle/function/mod.rs, src/kani/generate/negotiate.rs, tests/it/exact_function_generation.rs, tests/it/exact_scalar_generation.rs; quire-driver@fc53a75 (read only); quire-contract-ir@cbcd790 (read only); diff origin/main...HEAD, base aba2403"
---

# SR-1075: CG PR 244 gap analysis: lowering failed-record ACs against code at aba2403 and IR cbcd790

## Summary

Ticket: IR-547. PR: agent-ix/quire-contract-codegen#244 at 86ac677. This is a spec-only diff over 7 files: FR-014, FR-018 and FR-021, TC-024, TC-029 and TC-031, and tests.md. The PR body and author report were treated as claims and measured.

Code claims measured at base aba2403:

- **The three `Failed` arms.** They are at scalar/mod.rs:894 (`lowered`), equality/mod.rs:875 and function/mod.rs:673 (`lowered_binary_body`). Each is `Failed { limit, consumed, .. } => LoweringWorkExhausted`. There is no shared helper, and no non-test `limit_kind` or `CheckedPackageLimit` in src/. The one hit is scalar/mod.rs:3347/3386, inside `#[cfg(test)]`, and it is a `BodyIncomplete` record.
- **negotiate.** negotiate.rs:600-665 has an exhaustive, wildcard-free match on `ExactScalarRefusal` that sends `LoweringWorkExhausted` to `OracleRefused`. negotiate matches only `ExactScalarRefusal`, not the other two enums.
- **Public API.** The three refusal enums are re-exported from lib.rs and are not `#[non_exhaustive]`, so the new variants are a semver break in principle.
  - quire-driver at fc53a75 names none of the three enums. Its matches are on `UnsupportedObligation` and `ClaimDisposition` only, so nothing there breaks.
  - In this repo, negotiate.rs:600 and `refusal_variant_name` in tests/it/exact_scalar_generation.rs:1114 must gain arms. That test's fixture-pairing companion should then drive `LoweringByteLimitExceeded` through a whole-package fixture, and record `LoweringLimitUnrecognised` as undrivable through admission, as it already does for `InvalidBody` and `BodyIncomplete`.
- **Other lowering sites.** kani/generate/frame.rs:440 also lowers, but it wraps any non-lowered record in `StateFrameRefusal::NotLowered` without reading the kind, so the PR rightly leaves it alone.
- **IR.** See SR-1074's summary: 7 variants, `Failed` carries only work or bytes, and nothing contradicts AC-95.

## Verdict

One high finding blocks: FR-021-AC-23 asks for whole-package refusal, which contradicts covered FR-021-AC-12 and the code. FR-014 and FR-018 match the code shape.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-021-AC-23, its bullet and its mutation row require whole-package refusal on one body's failed record; contradicts covered FR-021-AC-12 and code (per-function; unrelated items still generate) | spec/oracle/functional/FR-021-function-application-oracles.md:166-172, 273, 304 |
| FND-002 | medium | FR-021's second lowering (call nodes, same arm) is unspecified; 'the record's consumed' and 'canonical length' are ambiguous between two lowered packages | src/oracle/function/mod.rs:862, 1084, 1292 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | Amended FR-021 bullet says items of every function whose body lowers stay unchanged; code refuses a caller of the failed function as UnknownCallee | spec/oracle/functional/FR-021-function-application-oracles.md:162-166 |

## Dispositions

Round 1 was reviewed at 10b5ed2602cb17306ea57e9a5a04a2235959aa7d (delta 86ac677..10b5ed2).

- **FND-001.** AC-23 is now scoped per function ("only the items bound to that function are refused ... while an unrelated function's items generate unchanged (as FR-021-AC-12)"). The mutation row now lists refusing every function on a `work` failure as the defect. The older bullet at FR-021:162 now says "refuse every item bound to that function", which matches AC-12, the Stage 2 survivor filter at function/mod.rs:964-968 and the tc_031_ac12_* tests. The amended bullet's last clause overclaims; see the new FND-003.
- **FND-002.** The new bullet matches the code at aba2403. `item_disposition` (function/mod.rs:1292) calls `lowered_binary_body(record)?` on the call-node record before it looks up the function's `resolved` refusal (line 1307), so a failed call-node record wins and carries the call-node lowering's `limit` and `consumed`. Both lowerings run under the same retained ceiling, so their `limit` is equal. The `DuplicateDeclaringNode` and `DuplicateRequest` checks at 1095-1100 come earlier still. The bullet's "checked first" is relative to the function refusal only, which is accurate for what it describes.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 10b5ed2 |
| FND-002 | fixed | 10b5ed2 |

Round 2 was reviewed at 200acd8565478dfbea41645ae22eab1c5c9339cc (delta 10b5ed2..200acd8).

- **FND-003.** The FR-021:162 bullet now reads "leaving the items of an unrelated function unchanged (FR-021-AC-12). A function whose own body lowers but whose nested `call` names a refused function is itself refused as `UnknownCallee`, and so on to its callers." This matches function/mod.rs:921-934 at aba2403, where a `Call` body whose callee is resolved `Err` becomes `UnknownCallee`, and the bounded fixed-point pass carries that to callers of callers.
- **Earlier findings.** Every finding fixed in rounds 0 and 1 is still fixed at 200acd8. The delta touches only the FR-014-AC-40, FR-018-AC-20 and FR-021-AC-23 rows, the FR-021:162 bullet, and TC-024 step 2, TC-029 step 12 and TC-031 step 11. No reference to FR-014-AC-43 and no "beside a lowered record" wording remains anywhere in spec/.
- **Numbering.** It is contiguous: FR-014 AC-1..42, FR-015 AC-1..50, FR-018 AC-1..20 in both the criteria and mutation tables, and FR-021 AC-1..23 in both. The oracle and kani tests.md rows and the TC-024, TC-025, TC-029 and TC-031 inventories agree with the ACs.
- **Gates.** `quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/**/*.md'` exits 0 with warnings only. `quire coverage --strict` reports 66 unbacked and 0 contradicted, unchanged. Both ran in a detached worktree at 200acd8, since removed.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-003 | fixed | 200acd8 |
