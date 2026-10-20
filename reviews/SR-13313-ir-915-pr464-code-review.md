---
id: SR-13313
title: "IR-915 PR 464 code review (with Rust lane): excluded accounting-only positions"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@ac741bb5de3bb421fcfe16c466b0085937c86d8e vs origin/main merge-base e49072f697dfbba25bf0d1514ed4d49deb8fabc6; src/core/contract_package.rs, src/routed/capability.rs, src/routed/generate.rs, src/oracle/function/mod.rs, src/routed/contract_package_tests.rs, tests/it/exact_function_generation.rs, tests/it/kani_scalar_native_fixture.rs, tests/it/routed_generation.rs"
review_set: subset
---

## Summary

Ticket: IR-915. PR agent-ix/quire-contract-codegen#464. The Rust lane is folded into this file.
The reviewer is not the author. The review used a fresh detached worktree at the PR head with one
target dir inside it.

The change adds `ContractRequestPosition::Excluded { node_id }`. `PackageItems` keeps excluded
positions in the driver-to-accounting map, so driver indexes are preserved, and keeps a separate
`bound_driver_indices` list for FR-331 envelope items. `negotiate_contract_package` validates the
envelope, then checks `EnvelopeItemCount` against bound items only, then threads the original
driver index into both `ContractItemSettlement.request_index` and nested
`ItemSettlement.request_index`. `PackageRequestError::OrphanContextRoot` refuses a context root
that directly follows an excluded position. `generate_routed` refuses a routed item at an excluded
index as `RoutedGenerationError::ExcludedDriverIndex` before any arm. The Kani selection table and
the admitted function-application body index are now indexed by driver position.

Measured by the reviewer:

- New tests use `#[trace(...)]` only. The diff adds no `/// Trace:` or `Tracing:` line.
- No compatibility layer, fallback, default or shim. `driver_count()` is replaced, not kept beside
  `bound_count()`. Every match on `ContractRequestPosition` that the diff touches is exhaustive
  (no `_` arm).
- The public names in the PR body match the code: `ContractRequestPosition::Excluded { node_id }`,
  `PackageRequestError::OrphanContextRoot { accounting_index, node_id }`,
  `RoutedGenerationError::ExcludedDriverIndex { request_index, node_id }`,
  `ContractItemSettlement.request_index` and `ItemSettlement.request_index`.
- quire-driver#86 builds `cg::ContractRequestPosition::Excluded { node_id }` with the same
  spelling. quire-integration#16 names only `ContractRequestPosition`, `RoutedGenerationError` and
  related existing types. See FND-002 for two problems in that composition.
- Mutation sample, run by the reviewer on the `routed::` lib tests (13 mutants; 2 survived and one
  is equivalent). Killed: outer index set to the envelope index (M01); nested index set to the
  envelope index (M02); `ExcludedDriverIndex` check removed (M03); orphan context-root refusal
  removed (M04); `EnvelopeItemCount` accepts extra items (M06); excluded `node_id` identity check
  skipped (M07); excluded positions compacted out of the driver map (M08); Kani selection table
  compacted (M09); admitted package dropped, which forces a preflight re-read (M10); count check
  moved ahead of envelope validation (M11). Survived: the context span extended across an excluded
  position (M05). This mutant is equivalent, because the orphan refusal fires first for any root
  after an excluded position. Also survived: the admitted function-body index compacted to skip
  excluded positions (M12, a real gap; see SR-13314 FND-004). Also survived: `ExcludedDriverIndex`
  reporting the accounted key instead of the routed item's key (M13, see FND-003).
- Full `QSPEC_DIR=... cargo test --locked --no-fail-fast --tests`, run by the reviewer:
  - Library: 319 passed, 50 failed, 4 ignored. All 50 failures are in `kani::run::execute`,
    `kani::run::namespace`, `kani_terminal_private` or `kani_scalar_native_private`. Each one
    fails at namespace startup (`MemoryMechanismUnavailable` / "namespace startup refused"). These
    are the known bwrap failures on this host.
  - Integration: 633 passed, 2 failed, 35 ignored, finished in 556 s. The 2 failures are
    `kani_obligations::tc_043_the_terminal_value_of_no_verdict_evidence_ignores_its_diagnostics`,
    which fails with `bwrap: setting up uid map: Permission denied`, and
    `kani_obligations::cbmc_failure_terminal_value_reads_the_retained_cause`, which gets
    `NoVerdict` because the launcher could not start under bwrap. Both are known bwrap failures.
  - No other failure. None of the 7 IR-898 tests appeared.
  - The author's reported "stall" is not a hang. After 636 tests the binary is still running
    `tc_031_ac7_admitted_hundred_thousand_call_nodes_generate_on_a_512_kib_stack`, which is
    CPU-bound (about 60% of a core) and completes in about 9 minutes in a debug build. The author
    stopped the run before it finished.
  - The 7 IR-915 tests and the newly tagged `duplicate_expression_uses_each_selected_conversion_context`
    pass.
- Real Kani was not run (the lead runs it).

## Examined criteria

- FR-039-AC-9 (examined): accounting correspondence for bound, excluded and bound positions;
  preserved driver indexes; no claim or artifact for the excluded position; typed refusal on
  remove, extra or changed `node_id`.
- FR-039-AC-10 (examined): span ends at the excluded position; typed `IncompleteContext` outside
  the span; typed orphan context-root refusal.
- FR-019-AC-53 (examined): bound-only `EnvelopeItemCount`; outer and nested driver indexes 0 and
  2; no excluded settlement; zero classifier calls and preflight reads; count refusals.
- FR-022-AC-29 (examined): typed `ExcludedDriverIndex` before any arm; indexes 0 and 2 remain
  valid.
- src/oracle/function/mod.rs:1798-1813 (examined): admitted body index by driver position.
- quire-driver#86 src/upstream.rs, src/refusal.rs, src/lib.rs and quire-integration#16
  tests/ir914_bound_export.rs (context_only): downstream consumers named in the PR body.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Excluded positions are still treated as generatable driver items by two public oracle entry points. `PackageItems::driver_node` and `PackageItems::at` resolve an excluded index like a bound one. `check_position` (function oracles) and the equality accounting check use only `driver_node`. So `generate_exact_function_oracles` or `generate_composite_equality_oracles`, called with an excluded index in `item_positions`/`positions`, produce a claim for an accounting-only position. That contradicts FR-039-AC-9 ("the excluded position ... has no claim or generated artifact"). Only `generate_routed` (`ExcludedDriverIndex`) and the scalar path (where `result_selection` is `None`) refuse it. Fix: have `at` (or a bound-only accessor used by every generator) refuse an excluded index with a typed error, and add a test. Traced by reading the code, not executed. | src/core/contract_package.rs:281-288, src/core/contract_package.rs:318-330, src/oracle/function/mod.rs:1943-1963, src/oracle/equality/mod.rs:665-708 |
| FND-002 | medium | The downstream composition the PR body names will not work with this CG head. (a) quire-driver#86 negotiates through the envelope-only `negotiate_backend_provider` (driver src/upstream.rs:192-196). That path still numbers settlements by envelope index (`None => (0..envelope.items.len()).collect()`). The driver's `in_order` check expects [0, 2] and gets [0, 1], so it raises `DriverFault::SettlementCount`, and quire-integration#16's `[0, 2]` assertion fails. (b) The driver's exhaustive `generation_code` and `generation_message` matches over `RoutedGenerationError` (driver src/refusal.rs:400-424, 494-) have no `ExcludedDriverIndex` arm, and the enum is not `#[non_exhaustive]`, so the driver fails to compile (E0004) once it takes this CG. The CG names are correct; the defect is in the pairing. Fix: driver#86 should call `negotiate_contract_package` with its positions (or remap explicitly) and add the `ExcludedDriverIndex` arm. This PR's body should state that the driver must switch to the package-aware negotiation, not only consume the variant names. Traced by reading the code, not built. | src/routed/capability.rs:1418-1421, PR #464 body |
| FND-003 | low | The doc on `RoutedGenerationError::ExcludedDriverIndex::node_id` says "Exact key of the excluded position", but `generate_routed` fills it with the routed item's `node_id`. If a routed item names index 1 with some other key, the refusal reports that other key, not the excluded position's key. Mutant M13 (report the accounted key) survives, so no test pins either reading. Fix: report `package_items.driver_node(request_index)`, or reword the doc to "key named by the routed item". | src/routed/generate.rs:197-203, src/routed/generate.rs:320-327 |

## Verdict

**Mergeable once FND-001 is fixed or explicitly deferred.** The core change is sound and well
tested. Bound-only envelope counting, outer and nested driver indexes, orphan refusal, span end,
excluded-index refusal, identity correspondence and refusal precedence are each pinned by a test
that a reviewer-run mutant fails. There is no compatibility layer, the names match the PR body and
driver#86, and the full suite is green apart from the known bwrap failures. FND-001 is a remaining
path where an excluded position can still get a claim through public CG APIs. FND-002 does not
block this CG PR, but it blocks quire-driver#86 and quire-integration#16 as written, and the lead
should route it to the driver lane before either is merged against this head. Real Kani was not
run (lead).

## Dispositions

Disposition pass 1, against head c167e8dc49021fc8674e8ad73d971cf32f28e663. The fix commits are 0f004600, 4efc021a and c167e8dc; 5146506d merges origin/main. The reviewer measured the following in a fresh worktree with one target dir:

- `make fmt-check lint spec` passes.
- Full `cargo test --locked --no-fail-fast --tests`:
  - Library: 325 passed, 50 failed, 4 ignored.
  - Integration: 636 passed, 2 failed, 36 ignored, in 460 s.
  - Every failure is a known bwrap or namespace-startup failure in `kani::run`, `kani_terminal_private`, `kani_scalar_native_private`, or the two `kani_obligations` terminal-value tests. No other failure occurred.
- Mutants re-run on the `routed::` lib tests:
  - Killed: M12 (body index skips excluded positions), M13 reversed (refusal reports the routed key), N1 (`bound_driver_node` accepts excluded), N3 (classifier index not recorded), and N5 (both the `at` guard and the equality guard removed).
  - N2 (the `at` guard alone removed) and N4 (the equality guard alone removed) survive individually. They are equivalent, because each guard backs up the other on every current caller.
- The `#[cfg(test)]` index recorders only observe and never change behaviour, matching the existing `FUNCTION_PREFLIGHT_READS` seam.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | `PackageItems::bound_driver_node` refuses an excluded index with `PackageRequestError::ExcludedDriverIndex { request_index, node_id }`. `at`, `check_position` (function), the equality accounting check and both scalar loops now use it. New tests `excluded_position_refuses_{function,equality,scalar}_oracle_before_claim_or_artifact` assert the typed refusal |
| FND-002 | deferred | Downstream defect owned by quire-driver#86. At review time driver#86 (af64a541) still calls `negotiate_backend_provider` (src/upstream.rs:196) and has no `ExcludedDriverIndex` arm. The CG PR body now says: "quire-driver#86 must use `negotiate_contract_package` with its full positions so settlements retain `[0, 2]`, and handle `RoutedGenerationError::ExcludedDriverIndex` in its exhaustive refusal matches before quire-integration#16 can use this CG head." This still blocks driver#86 and integration#16 |
| FND-003 | fixed | `generate_routed` now reports the excluded position's admitted key (`package_items.excluded_node(..)`), matching the doc. The test adds a mismatched routed key at index 1 and asserts `node_id == excluded`; the reversed mutant is killed |
