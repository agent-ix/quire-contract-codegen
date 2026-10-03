---
id: "SR-1076"
title: "CG PR 244 criterion strength: can the IR-547 planned rows fail"
type: SpecReview
analysis: criterion-strength
review_set: subset
scope: "agent-ix/quire-contract-codegen@86ac6775de82161ea61b4670f94fe59aa413ee69; spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/TC-029-composite-equality-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md, tests/common/panic_scan.rs; quire-contract-ir@cbcd790 crates/quire-contract-model/src/checked_package/v2/{mod.rs,lower.rs,lower/ceiling_tests.rs} (read only); diff origin/main...HEAD, base aba2403"
---

# SR-1076: CG PR 244 criterion strength: can the IR-547 planned rows fail

## Summary

Ticket: IR-547. PR: agent-ix/quire-contract-codegen#244 at 86ac677. This is a spec-only diff over 7 files: FR-014, FR-018 and FR-021, TC-024, TC-029 and TC-031, and tests.md. The PR body and author report were treated as claims and measured.

The test oracle of every planned row was checked against what CG's public surface can actually produce.

- The whole-call byte case can be reached. A lowered package carries every requested node in full, plus its `ir_id`, semantic type and dependency lists, so it can be longer than the checked document (see SR-1074 FND-001 on which length).
- FR-014-AC-43 has a real oracle. A wrong `UnsupportedObligation` variant or a dropped field fails its field-for-field equality.
- The FR-018-AC-20 and FR-021-AC-23 mutation rows each name a change that their AC detects. The exception is the `_ => LoweringWorkExhausted` mutation in the module mapping, which depends on the seam (FND-002).

## Verdict

The per-node clause cannot be tested as written, and the unrecognised-kind rows need the module seam named. Both are fixable in the spec without changing the design.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Per-node byte-failure clause (AC-40, AC-20, TC-024 s1, TC-029 s12) unreachable via public read+lower: preimage shorter than any admitted document; IR reaches it only by in-crate construction | spec/oracle/functional/FR-014-exact-scalar-oracles.md:384 |
| FND-002 | medium | Unrecognised-kind rows only reachable by constructed records; TC-024 s3 tests the classifier, not each module's mapping, so a module-level Unrecognised->WorkExhausted mutant survives | spec/oracle/matrix/TC-024-exact-scalar-oracles.md:168-171 |
| FND-003 | low | AC-42 scan: 'non-test code' undefined (cite AC-39 / panic_scan helpers), classifier unnamed, claim broader than the three-file scan | spec/oracle/functional/FR-014-exact-scalar-oracles.md:386 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | AC-23/TC-031 s11 assert per-function isolation 'on hand-built records given to the Failed arm'; that arm cannot show items generating and no seam injects records into Stage 1-3 | spec/oracle/functional/FR-021-function-application-oracles.md:282 |
| FND-005 | low | Sibling-keeps-disposition check on the per-record `lowered` arm cannot fail (pure per-record function) | spec/oracle/matrix/TC-024-exact-scalar-oracles.md:165-169 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | AC-23 mutation row still claims detection of 'refuse every function on a work failure'; AC-23 no longer asserts isolation and AC-12's tests never drive a failed record | spec/oracle/functional/FR-021-function-application-oracles.md:314 |

## Dispositions

Round 1 was reviewed at 10b5ed2602cb17306ea57e9a5a04a2235959aa7d (delta 86ac677..10b5ed2).

- **FND-001.** The per-node public-API clause is gone. AC-40, AC-20, TC-024 step 2 and TC-029 step 12 now use hand-built `Failed { limit_kind: Bytes }` records in each module's own `#[cfg(test)]` seam, and say plainly that CG's public API cannot reach the per-node case.
- **FND-002.** For scalar and equality, the unrecognised-kind and `bytes` assertions now drive each module's own `Failed` arm, so a module-level unrecognised-to-`LoweringWorkExhausted` mapping fails them. For function, see the new FND-004.
- **FND-003.** AC-42 now names `classify_lowering_failure`, scans exactly the three files, and defines non-test code as FR-014-AC-39 does, using `non_test_code` and `comments_stripped` from tests/common/panic_scan.rs (both exist at aba2403, lines 246 and 262). It requires each file to call the classifier at its `Failed` arm. Each module has exactly one `Failed` arm: scalar `lowered` (shared by generation and `derive_exact_scalar_items`), equality `lowered`, and function `lowered_binary_body` (shared by both lowerings). The scan fails on base, which has no such call, so it is not vacuous.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 10b5ed2 |
| FND-002 | fixed | 10b5ed2 |
| FND-003 | fixed | 10b5ed2 |

Round 2 was reviewed at 200acd8565478dfbea41645ae22eab1c5c9339cc (delta 10b5ed2..200acd8).

- **FND-004.** AC-23 now asserts only the record-to-refusal mapping on hand-built records at `lowered_binary_body`, in a `#[cfg(test)]` module the code change adds. function/mod.rs has no `cfg(test)` at aba2403 (0 occurrences), so stating that the code change adds the module is accurate.
  - Per-function isolation is handed to FR-021-AC-12's existing tests. tests/it/exact_function_generation.rs:263 `tc_031_ac12_dangling_callee_refuses_only_its_own_items` and :291 `tc_031_ac12_form_mismatch_refuses_only_its_own_item` assert `unrelated_fn` stays `Generated` when another function's Stage-1 classification refuses. A failed lowering record enters through that same `own_shape` path.
  - The call-node-first order is now asserted through the whole call, using differing `consumed` (see SR-1074 FND-006). No private seam is needed.
- **FND-005.** The sibling half is dropped from FR-014-AC-40, FR-018-AC-20, TC-024 step 2 and TC-029 step 12. FR-014-AC-40 now points to FR-014-AC-1 ("a refused item contributes no generated function while its siblings are generated unchanged"), which owns sibling isolation.
- **New, low FND-006:** one clause of the FR-021-AC-23 mutation row is no longer detected by any criterion (below).
- **Earlier findings.** Every finding fixed in rounds 0 and 1 is still fixed at 200acd8. The delta touches only the FR-014-AC-40, FR-018-AC-20 and FR-021-AC-23 rows, the FR-021:162 bullet, and TC-024 step 2, TC-029 step 12 and TC-031 step 11. No reference to FR-014-AC-43 and no "beside a lowered record" wording remains anywhere in spec/.
- **Numbering.** It is contiguous: FR-014 AC-1..42, FR-015 AC-1..50, FR-018 AC-1..20 in both the criteria and mutation tables, and FR-021 AC-1..23 in both. The oracle and kani tests.md rows and the TC-024, TC-025, TC-029 and TC-031 inventories agree with the ACs.
- **Gates.** `quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/**/*.md'` exits 0 with warnings only. `quire coverage --strict` reports 66 unbacked and 0 contradicted, unchanged. Both ran in a detached worktree at 200acd8, since removed.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | 200acd8 |
| FND-005 | fixed | 200acd8 |

Round 3 was reviewed at 4d90bcc8c98c80b64363bb641a233a68c2108671 (delta 200acd8..4d90bcc).

- **FND-006.** The tree diff 200acd8..4d90bcc touches two lines in two files, and both are mutation-table rows.
  - The FR-021-AC-23 row drops "or refuse every function because one body failed for `work` so an unrelated function's items stop generating".
  - The FR-018-AC-20 row drops "or read `limit_kind` in this module with its own match so it can disagree with FR-014's and FR-021's".
  - Nothing else changed: no AC, Behavior bullet, TC step or matrix row.
- **The removed FR-018 clause.** It belongs to FR-014-AC-42's scan, which requires no `limit_kind` in the non-test code of equality/mod.rs. AC-20 asserts the mapping only, so removing the clause from AC-20's row is correct.
- **Each remaining clause is caught by its AC.**
  - FR-018-AC-20:
    - A single `Failed` to `LoweringWorkExhausted` arm fails AC-20's hand-built `bytes` record assertion.
    - A `_ => LoweringWorkExhausted` fallthrough fails its hand-built `depth`, `nodes`, `edges`, `occurrences` and `diagnostics` assertions.
    - A panic on an unrecognised kind fails its "without a panic" clause.
  - FR-021-AC-23:
    - A single `Failed` to `LoweringWorkExhausted` arm fails the hand-built `bytes` record at `lowered_binary_body`.
    - A `_ => LoweringWorkExhausted` or `_ => unreachable!(..)` arm fails the five unrecognised-kind records, either by the wrong variant or by a panic.
    - Reporting the body lowering's `consumed` fails the whole-call assertion, because the fixture makes the call-node and body `consumed` differ.
- **Gates.** `quire validate` on this file exits 0.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-006 | fixed | 4d90bcc |
