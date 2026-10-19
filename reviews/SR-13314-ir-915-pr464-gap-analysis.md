---
id: SR-13314
title: "IR-915 PR 464 gap analysis: excluded accounting-only CG positions"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@ac741bb5de3bb421fcfe16c466b0085937c86d8e; FR-039-AC-9, FR-039-AC-10, FR-019-AC-53, FR-022-AC-29 vs spec/core/matrix/tests.md, spec/routed/matrix/tests.md, TC-053, TC-030, TC-033; src/routed/contract_package_tests.rs, src/core/contract_package.rs, src/routed/capability.rs, src/routed/generate.rs, src/oracle/function/mod.rs"
review_set: subset
---

## Summary

Ticket: IR-915. PR agent-ix/quire-contract-codegen#464. This is a planless gap analysis; plan
completion was not assessed.

`quoin matrix --json` (quoin 0.28.3) and `quire matrix --strict` (quire 0.36.2) at the head both
report all four criteria as `tagged`:

- FR-039-AC-9: `excluded_position_keeps_accounting_and_driver_index`.
- FR-039-AC-10: `excluded_position_ends_context_span_and_orphans_following_root` and
  `duplicate_expression_uses_each_selected_conversion_context`.
- FR-019-AC-53: `excluded_position_consumes_no_envelope_item_and_preserves_both_indices` and
  `kani_function_after_excluded_position_settles_at_driver_index`.
- FR-022-AC-29: `routed_excluded_driver_index_is_refused_before_generation`.

All of these pass in the reviewer's full run. `quire matrix --strict` exits 1 because of
pre-existing untagged criteria elsewhere (for example FR-004-AC-12 and FR-034-AC-75), not because
of this PR. Every tag is a `#[trace(...)]` attribute; no `/// Trace:` line is added.

The matrix edits touch only the four rows for these criteria and the TC-030, TC-033 and TC-053
status cells, which name only these criteria. No other row changed. No production code in the
diff lacks an owning requirement.

The mutation sample is in SR-13313. Every main-path rule is pinned by a mutant the tests kill. The
findings below are criterion clauses that the tagged tests do not assert, or assert too weakly to
fail.

## Examined criteria

- FR-039-AC-9 (examined): "In an admitted package with a bound driver item, an excluded accounting-only position and another bound driver item, CG matches all three positions and exact node_id values to feature_accounting; the excluded position keeps its driver index but has no claim or generated artifact, and the following bound item keeps its original driver index. ... Removing the excluded position, adding an extra one or changing its node_id causes a typed whole-request correspondence refusal before any artifact."
- FR-039-AC-10 (examined): span-before-excluded owned (asserted, generated claims for [0, 2]); outside-span `IncompleteContext` (asserted); orphan root refused as typed `OrphanContextRoot { accounting_index: 2 }` through both `PackageItems::new` and `negotiate_contract_package` (asserted). Clean.
- FR-019-AC-53 (examined): two-item settle with outer and nested indexes [0, 2], one-item and three-item `EnvelopeItemCount { requested: 2 }`, and excluded accounting `Lowered` are asserted. The zero-count and context-root clauses are checked below.
- FR-022-AC-29 (examined): typed `ExcludedDriverIndex { request_index: 1, node_id }` and valid routes at 0 and 2 (asserted); Kani route at driver index 2 after the exclusion (asserted in the FR-039-AC-10 test). Clean.
- spec/core/matrix/tests.md and spec/routed/matrix/tests.md rows for these four criteria and for TC-030, TC-033 and TC-053 (examined).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The FR-039-AC-9 refusal clause is asserted only as `assert!(PackageItems::new(&admitted, &changed).is_err())` for the removed, duplicated and changed-`node_id` cases. The criterion requires a typed whole-request correspondence refusal; any error variant (or a wrong one, such as `OrphanContextRoot`) passes. Fix: match the expected variant per case (for example `AccountingIdentity` for the changed key, and the length/correspondence variant for remove and extra), and assert it through a public entry point (`negotiate_contract_package` or `generate_routed`) to cover "before any artifact". | src/routed/contract_package_tests.rs:513-543 |
| FND-002 | medium | The FR-039-AC-9 clause "the excluded position ... has no claim or generated artifact" has no assertion in the AC-9-tagged test, which inspects only `PackageItems`. The generator paths that do refuse an excluded index are tagged to FR-022-AC-29 and FR-019-AC-53, and two public oracle generators do not refuse it (SR-13313 FND-001). So the ✅ Covered row overstates AC-9. Fix: add an AC-9-tagged assertion that no claim or artifact is produced for index 1 across the generator entry points, after SR-13313 FND-001 is fixed. | src/routed/contract_package_tests.rs:513-543, spec/core/matrix/tests.md:27 |
| FND-003 | medium | The FR-019-AC-53 clause "its own classifier-call and Kani-preflight-read counts are both zero" is not backed by an assertion that can fail. No classifier-call counter exists. `take_function_preflight_reads()` is incremented only on the non-admitted re-read branch (src/oracle/function/mod.rs:1821), which `negotiate_contract_package` never takes, so the `== 0` checks cannot detect the excluded position being classified or preflighted. The clause "A context-only root changes neither count nor driver indexes" is also not asserted at negotiation level under an AC-53 tag; only `PackageItems::bound_driver_index` is checked, in the AC-10 test. Fix: count classifier and preflight calls per request index (or per node) through a test seam, and assert the excluded index's count is zero. Add an AC-53 case with an interleaved context root that settles two items at [0, 2]. | src/routed/contract_package_tests.rs:605-657, src/routed/contract_package_tests.rs:659-740 |
| FND-004 | medium | The `src/oracle/function/mod.rs` change, which maps the admitted body index by driver position and counts excluded positions, is untested. Mutant M12 (skip excluded positions when computing the body index) survives every `routed::` test, because the only Kani function case puts the excluded position after the body. Scenario: positions `[Excluded X, DriverItem body, DriverItem call]`. The mutated code looks up driver index 0 (X), fails `check_position`, and reports the function application as unsupported. Fix: add an FR-019-AC-53 case with the excluded position before the function body and assert `Supported` at the call's driver index. | src/oracle/function/mod.rs:1798-1813, src/routed/contract_package_tests.rs:659-740 |

## Verdict

**Gaps remain in clause-level evidence; main paths covered.** All four criteria are tagged and
their main paths are pinned by killed mutants. The matrix rows were updated only for these
criteria. FND-001 to FND-004 are clause-level gaps: a weak typed-refusal assertion, an unasserted
no-claim clause, an unfailable zero-count clause plus an untested context-root clause, and an
untested function-body index branch. FND-002 depends on the code fix in SR-13313 FND-001. These
are fix-in-PR items before the ✅ Covered rows for FR-039-AC-9 and FR-019-AC-53 are accurate.
