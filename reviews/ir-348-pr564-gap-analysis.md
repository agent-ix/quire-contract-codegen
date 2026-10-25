---
id: SR-17016
title: "IR-348 CG564 gap-analysis (AD-004 step 3 scalar split)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-348; PR564; exact 1aa7807ffd12c4582ae9c5e0a2767b9bf1792ec9; base 4eccd682545a240cf2289bf2040e820002bdf2e5; src/oracle/scalar/**, scalar callers, tests/it scanners, spec FR-014/TC-024/AD-004 as context"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/AD-004", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-014", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/TC-024", type: references }
---

## Summary

Ticket: IR-348 (AD-004 step 3, `exact_scalar` split). This is a planless gap analysis of CG #564 at the frozen head `1aa7807f`. `quire coverage --scope .` was run at head and over a `git archive` export of base `4eccd682`. Both report 626/979 rows backed. The outputs are identical except for the line number of one pre-existing `oracle-resembles-implementation` note, which follows the moved test. No trace tag or `// Implements:` comment was lost: the normalised line-multiset comparison of base `mod.rs` against the four head files shows no removed line.

Evidence for the PR's own behavior:

- With `QSPEC_DIR` set to a read-only export of quire-specification `origin/main` `9f782921` (`proposals/checked-package-v2`), `exact_scalar_generation::` passes 79 of 79 at head.
- The coder's 3 failures are environmental and are neither inherited code failures nor caused by this PR. `/home/peter/dev/quire-specification` is at `a286b81f`, which lacks `extension-definitions.json`. Without `QSPEC_DIR` the same 3 fail at the env lookup instead.
- The coder's lib QSpec-fixture `MalformedWire` result and the "20 passed / 1 passed" lib results cannot be reproduced at this head: the lib test target does not compile (SR-17015 FND-001). Lib-level scalar behavior evidence is therefore absent at the frozen head.

## Verdict

FAIL. Not mergeable until SR-17015 FND-001 is fixed and the lib scalar tests are re-run. Gap findings: the spec text still names `mod.rs` alone as the scanned scalar generator and the home of the one `Failed` arm (FND-001); one must-scan guard was not widened (FND-002); and the step 3 walker move is not done or explicitly deferred (FND-003). AC coverage is otherwise intact and no AC was narrowed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-014-AC-39, FR-014-AC-42 and their TC-024 steps still name `src/oracle/scalar/mod.rs` alone. AC-42 requires "exactly one such direct-lowering `Failed` arm in `src/oracle/scalar/mod.rs`", but the arm is now at `lower.rs:164`. The tests were widened to the four files without a spec update or a recorded deferral, so the AC text is false as written. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:525; spec/oracle/functional/FR-014-exact-scalar-oracles.md:575; spec/oracle/functional/FR-014-exact-scalar-oracles.md:578; spec/oracle/matrix/TC-024-exact-scalar-oracles.md:309; spec/oracle/matrix/TC-024-exact-scalar-oracles.md:318; src/oracle/scalar/lower.rs:164 |
| FND-002 | low | `no_generation_panics` `MUST_SCAN` still lists only `src/oracle/scalar/mod.rs` among scalar files. The emitted-source generators now live in `render.rs` and `lower.rs`, which the walk covers but the must-scan guard does not pin. | tests/it/no_generation_panics.rs:25-37 |
| FND-003 | low | AD-004 step 3 says the `exact_scalar` PR also "moves shared checked-node walkers into `core/ir`". This PR moves none: `equality/mod.rs` now imports the shared range and text-bound readers and `COLLECTION_BOUNDS_MEMBERS` from `oracle::scalar::lower`. The PR body says only that "remaining AD-004 work stays open", without naming this as deferred. | src/oracle/equality/mod.rs:82-85; spec/assurance/AD-004-cg-crate-layout.md:1030; spec/assurance/AD-004-cg-crate-layout.md:297 |

## Coverage

- Gates the reviewer ran at head:
  - fmt: not re-run, coder reported exit 0.
  - `clippy --all-targets -D warnings`: see Gate addendum.
  - `make rustdoc` equivalent: see Gate addendum.
  - `cargo test --lib oracle::scalar::`: fails to compile.
  - `--test it exact_scalar_generation::`: 79/79 with current QSpec.
- No aggregate gate and no Kani were run, per the brief.
- spec-review was not applicable and was skipped: the diff touches no `spec/**` or `plan/**` file. The spec drift the code causes is recorded here as FND-001.
- AC bindings checked:
  - `tc_024_ac39_scalar_generator_and_generated_crates_hold_no_panic_site` → FR-014-AC-39 is correct; it now scans all four files.
  - `tc_024_ac42_only_the_shared_classifier_reads_a_failed_records_limit_kind` → FR-014-AC-42 is correct; it concatenates the four files for the scalar slot and still asserts one `Failed` arm.
  - `oracle_generators_have_no_panicking_arms` is widened to the four files.

Plan completion: not assessed

## Gate addendum

- `cargo clippy --locked --all-targets -- -D warnings`: exit 101 (lib test E0433 x4; SR-17015 FND-001).
- `make rustdoc` equivalent: exit 101 (5 unresolved intra-doc links; SR-17015 FND-002).
- `QSPEC_DIR=<origin/main 9f782921 export> cargo test --locked --test it exact_scalar_generation::`: 79 passed, 0 failed.

## Examined scope and bindings

```yaml
clean: false
scope:
  - {id: FR-014-AC-39, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "The generated `src/lib.rs` of every crate the TC-024 corpus generates, and the non-test code of `src/oracle/scalar/mod.rs` (comments and every `#[cfg(test)]` item removed wherever the item sits, string literals the generator emits counted), contain zero panic tokens"}
  - {id: FR-014-AC-42, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "No `limit_kind` of a `failed` record is read in `src/oracle/scalar/mod.rs`, `src/oracle/equality/mod.rs` or `src/oracle/function/mod.rs` but through the one shared classifier ... has exactly one such direct-lowering `Failed` arm in `src/oracle/scalar/mod.rs`, which binds nothing (`Failed { .. }`) and calls `classify_lowering_failure`"}
  - {id: AD-004-step-3, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "It moves shared checked-node walkers into `core/ir`, moves descriptor derivation to `oracle/scalar/derive.rs`, node lowering, bounds and operation checks to `oracle/scalar/lower.rs`, and source and artifact rendering to `oracle/scalar/render.rs`."}
  - {id: AD-004-L-1, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "`oracle/scalar/` declares `derive`, `lower` and `render` from `mod.rs`, and the scalar-specific logic resides in those files as the target tree assigns."}
  - {id: FR-039, path: spec/core/functional/FR-039-contract-package-consumption.md, role: examined, excerpt: "The crate-internal scalar consumers `generate_contract_scalar_oracles` and `preflight_contract_scalar_claim` in `src/oracle/scalar/mod.rs` ... also take `&ContractPackageV1`."}
  - {id: TC-024, path: spec/oracle/matrix/TC-024-exact-scalar-oracles.md, role: context_only, excerpt: "Take the non-test code of `src/oracle/scalar/mod.rs`, `src/oracle/equality/mod.rs` and `src/oracle/function/mod.rs` ... and in each exactly one `Failed` arm"}
bindings:
  - {test_id: tc_024_ac39_scalar_generator_and_generated_crates_hold_no_panic_site, ac_id: FR-014-AC-39, trace: correct}
  - {test_id: tc_024_ac42_only_the_shared_classifier_reads_a_failed_records_limit_kind, ac_id: FR-014-AC-42, trace: correct}
```

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | FR-014-AC-42 and TC-024 step 4 now place the single scalar `Failed` arm "in `src/oracle/scalar/lower.rs`", but no test asserts that location. `tc_024_ac42_*` concatenates the four scalar files and asserts one arm overall, so it passes if the arm moves to `render.rs` or `derive.rs`. Either assert the arm's file or drop the location clause the fix added. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:578; spec/oracle/matrix/TC-024-exact-scalar-oracles.md:312; tests/it/exact_scalar_generation.rs:3621-3640 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | The round-2 FR-014-AC-42 text places `limit_kind` per file: the refusal field "in `mod.rs`", its conversion "in `lower.rs`", and "`derive.rs` and `render.rs` do not name it". `tc_024_ac42_*` still checks `limit_kind` only over the concatenated scalar text (count 3 plus two `contains` checks), so those location clauses have no test. Reviewer mutation: moving `impl From<LoweringFailure> for ExactScalarRefusal` from `lower.rs` to `render.rs` leaves `tc_024_ac42_*` passing while AC-42 becomes false. Either count `limit_kind` per scalar file (1 in `mod.rs`, 2 in `lower.rs`, 0 in `derive.rs` and `render.rs`) or drop the per-file clauses. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:578; tests/it/exact_scalar_generation.rs:3640-3700; src/oracle/scalar/lower.rs:197-216 |

## Dispositions

### Round 1 — exact `da6d12a72e92f080eaae4887dfd52b122c4777a5` (fix diff `1aa7807f..da6d12a7`)

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The FR-014 statement (line 525), FR-014-AC-39 (575), FR-014-AC-42 (578), TC-024 step 4 (309-313) and panic-scan step 1 (318) now name all four scalar files, and AC-42 names the arm's actual file. The sole scalar non-test `CompleteLoweringRecordV2::Failed` arm is at `lower.rs:191`. No criterion is narrowed: the scanned set grows from one file to four, and the equality and function obligations, the token set and the exclusion rules are unchanged. `quire validate` exits 0 on both files. The wording and location-test residuals are recorded separately as SR-17018 FND-001 and FND-004 above. |
| FND-002 | fixed | `MUST_SCAN` (`no_generation_panics.rs:27-40`) is now 12 entries, adding `scalar/derive.rs`, `lower.rs` and `render.rs`. `no_generation_panics::` passes inside the reviewer's 96/96 `it` run. |
| FND-003 | fixed | `aggregate_members`, `bound_members`, `literal`, `literal_integer` and `literal_count` moved from `oracle/checked_graph.rs` to `core/ir/checked_term.rs`. The six bound readers and the six `*_MEMBERS` tables moved from `scalar/lower.rs` to `core/ir/scalar_bounds.rs`, with bodies byte-identical to `1aa7807f`. Equality, function, `kani/generate/frame.rs` and `kani/generate/scalar.rs` import them from `core::ir`, and no scalar-stage export remains for shared readers. `core/ir` imports only quire crates, and the L-2 layout tests pass. The AD's wider typed-node-access migration (70 `.get` sites, L-8) is out of this finding's scope. |

The new finding FND-004 has no outcome yet; it is routed to the coder.

Gap evidence at `da6d12a7`:
- Test fn, trace and Implements multisets are identical to `1aa7807f`.
- The moved readers in `core/ir` are scanned by the whole-`src` NFR-005-AC-1 walk with the same `panic_tokens_in` scanner. They emit no string literals, so AC-39's literal counting loses nothing.
- `exact_scalar_generation::` passes 79 tests, inside the 96/96 `it` run with current QSpec.

Evidence limit: `quire coverage` was not re-run this round. The trace, test and Implements multisets were compared directly instead.

### Round 2 — exact `212bff4c65308b04723a114b01c68a5d381e0038` (fix diff `da6d12a7..212bff4c`)

| FND | outcome | reason |
| --- | --- | --- |
| FND-004 | fixed | `tc_024_ac42_*` now counts `CompleteLoweringRecordV2::Failed` in each scalar file's non-test code and asserts 1 for `src/oracle/scalar/lower.rs` and 0 for `mod.rs`, `derive.rs` and `render.rs` (`exact_scalar_generation.rs:3630-3640`), so moving the arm out of `lower.rs` now fails the test. The test passes at this head. |

The new finding FND-005 has no outcome yet; it is routed to the coder.

AC narrowing check, round 2: the only spec change is the FR-014-AC-42 rewrite. Against `origin/main`, the per-file `CheckedPackageLimit` and `.limit_kind` bans, the single unbinding `Failed` arm calling `classify_lowering_failure`, and the equality and function `PackageAccounting` obligations are all retained, over six files instead of three. The one loosening in wording ("any conversion" for the equality and function files) is recorded as SR-17018 FND-002. The PR touches no FR-015, FR-037 or FR-039 text.

Gap evidence at `212bff4c`: `exact_scalar_generation::`, `no_generation_panics::`, `checked_term_consumers::` and `layout::` pass 96/96 with current QSpec, and lib `oracle::scalar::` and `core::ir::` pass 22/22. The fix diff adds and removes no `#[trace]`, `#[test]` or `// Implements:` line. `quire validate` on FR-014 and TC-024 exits 0; its one warning (FR-014 line 269) is pre-existing.

### Round 3 — exact `5e83fe1b70d0bb0b34c1c6ad7912c8d4ef37c53d` (fix diff `212bff4c..5e83fe1b`)

| FND | outcome | reason |
| --- | --- | --- |
| FND-005 | fixed | `tc_024_ac42_*` now counts `limit_kind` in each scalar file's non-test code and expects 1 in `mod.rs`, 2 in `lower.rs`, and 0 in `derive.rs` and `render.rs` (`exact_scalar_generation.rs:3638-3648`). The reviewer re-ran the round-2 mutation, moving `impl From<LoweringFailure> for ExactScalarRefusal` from `lower.rs` to `render.rs`, and `tc_024_ac42_*` now fails on it. At the unmutated head, `exact_scalar_generation::` passes 79/79. |

Round 3 scope check: apart from committing the reviewer's SR files under `reviews/` (byte-identical to the round-2 scratchpad copies), the diff from `212bff4c` touches only the FR-014-AC-42 row and the per-file count in `tc_024_ac42_*`. AC-42's only change pins the equality file to the field alone, with no conversion, and the function file to its one named conversion. Both are tighter than the round-2 text and than `origin/main`, so no criterion is narrowed. `quire validate` on FR-014 and the four committed `reviews/ir-348-pr564-*.md` files reports no failure. `cargo fmt --check` exits 0.
