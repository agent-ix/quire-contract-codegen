---
id: "SR-3023"
title: "CG IR-655 gap analysis: stage-2 oracle criteria against the computed matrix and existing tests"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@spec/ir-655-stage2-temporal-oracle (frozen head, no PR yet; reviewed revision recorded in the IR-655 Linear marker only, per this repository's no-SHA rule); spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md, spec/kani/matrix/tests.md; src/kani/run/namespace.rs (old retained unit tests); computed matrix at CG main and at head"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: references
---

# SR-3023: CG IR-655 gap analysis

## Summary

Ticket: IR-655. Planless mode. The PR is spec-only, so the reverse-gap and stub steps have no new
production code to examine. The guardian source the spec is grounded on is not on CG main:
src/kani/run has no InnerSettlement, TerminalSampling or Commit. That source dependence is
SR-3020 FND-005.

`quire matrix --scope <tree> --format tsv` on main gives 588 criteria and on head gives 592. The
diff adds only FR-034-AC-51..54, all untagged and declared PLANNED/UNRUN. Every prior row is
byte-identical. Repository status counts, main then head: tagged 313/313,
method-without-symbol 32/32, tagged-by-ignored-test 11/11, untagged 232/236. `--strict` exits 1
on both, so it is no worse.

The old unit test `completed_monitor_cleanup_kills_an_orphan_and_its_fork_after_the_last_sample`
(src/kani/run/namespace.rs:577) is tagged FR-028-AC-21 and FR-017-AC-24, not FR-034-AC-8. Both
criteria have other binders.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-51..54 are verified by "Test, Analysis", but their Analysis half has no receipt producer or independent checker. AC-41..50 rows name "CG CODE author supplies the operational receipt; independent CODE reviewer checks it". That source-binding Analysis is what makes the records non-forgeable (FR-034:1131-1132), so the method row could compute satisfied with no accountable receipt. | spec/kani/matrix/TC-049-caller-death-ownership.md:118-121; spec/kani/matrix/TC-049-caller-death-ownership.md:108-117; spec/kani/functional/FR-034-caller-death-ownership.md:1131-1132 |
| FND-002 | low | The retirement text does not say what happens to the old test's trace tags (FR-028-AC-21, FR-017-AC-24) when it is retired. The replacement criteria are FR-034-only, so retiring namespace.rs:577 silently drops one FR-017-AC-24 binder and one FR-028-AC-21 binder, with no replacement trace allocated. | src/kani/run/namespace.rs:575-577; spec/kani/matrix/TC-049-caller-death-ownership.md:132-134 |

## Verdict

FAIL under the skill's mechanical rule: untagged criteria exist, 236 repo-wide and 4 new. The 4
new ones are declared PLANNED/UNRUN, as a spec-only allocation expects, so they are not a defect in
themselves. With the 4 new untagged criteria set aside, the result is CONDITIONAL on FND-001 and
FND-002.

## Coverage

- Criteria added: 4 (FR-034-AC-51..54), all untagged, none claimed covered.
- Prior criteria changed: 0. Strict status: unchanged (exit 1 at main and head).
- Reverse gap and stubs: not applicable. No production code changed.
- Semantic review: skipped. This is a spec-only gate; intent-to-oracle vacuity is covered in SR-3020.
- Plan completion: not assessed

## Dispositions

Round 1. Branch head "Strengthen stage-two mutation and temporal predicates"; its revision is
recorded in the Linear marker only.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | TC-049:118-123: each AC-51..56 row now says "CG CODE author supplies per-predicate source/bounds and applicable runtime receipts; independent CODE reviewer checks before CODE acceptance". |
| FND-002 | fixed | FR-034:1168 and step 28 keep the FR-028-AC-21 and FR-017-AC-24 trace obligations until parity is accepted; replacement bindings transfer only with full genuine original assertions. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | AC-56 is Analysis-only and computes method-without-symbol, which `quire matrix --strict` accepts with no receipt. Its TC-049 row says "Analysis is not Test credit" but omits the AC-41..50 rows' "Completion is zero until actual evidence, never inferred from a matrix method row". Because AC-56 gates the literal-FINAL retirement, the row should carry the same explicit statement. | spec/kani/matrix/TC-049-caller-death-ownership.md:123; spec/kani/matrix/TC-049-caller-death-ownership.md:108-117 |

### Verdict (disposition pass 1)

Matrix: 588 on main, 594 at head. All prior rows are identical. AC-51..55 are untagged and AC-56
is method-without-symbol (repo-wide: method-without-symbol 33, untagged 237). Strict exits 1, no
worse. FAIL under the mechanical untagged rule because the PLANNED criteria are declared untagged.
Otherwise CONDITIONAL on FND-003. Plan completion: not assessed.

## Dispositions (round 2)

Round 2. Branch head "Allocate sampled-membership regression to honest source analysis"; its
revision is recorded in the Linear marker only.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-003 | fixed | The TC-049 AC-55 and AC-56 rows (122-123) and the expectations note now say "Completion is zero until actual evidence, never inferred from a matrix method row". |

### Verdict (disposition pass 2)

Matrix: 588 on main, 594 at head. The only change since round 1 is AC-55, from untagged to
method-without-symbol (repo-wide: method-without-symbol 34, untagged 236). All prior rows are
identical and strict exits 1, no worse. Mechanical untagged rule: FAIL, on declared PLANNED
criteria. No open gap-analysis findings. Plan completion: not assessed.
