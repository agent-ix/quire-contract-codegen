---
id: "SR-3022"
title: "CG IR-655 spec-integrity-analysis: retirement delta completeness and cross-document consistency"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@spec/ir-655-stage2-temporal-oracle (frozen head, no PR yet; reviewed revision recorded in the IR-655 Linear marker only, per this repository's no-SHA rule); spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md, spec/kani/matrix/tests.md; compared against the same files on CG main"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: references
---

# SR-3022: CG IR-655 integrity analysis

## Summary

Ticket: IR-655. Checked:

- Whether the retirement delta is complete against TC-049's own rule. TC-049:142-145 on head was
  TC-049:125-139 on main. It requires the original oracle, the new O-origin boundary, a stronger
  confirmed whole-outer-tree guarantee and an adverse witness defeating the new cancellation
  authority.
- Silent deletions in the slice table.
- Consistency with FR-034 AC-8, AC-10 and AC-33.
- Consistency with the IR-682 supported-build text, AC-41..50 and TC-049 "Supported-build named
  native workspace".
- Where AC-51..54 appear across the tables.
- The hand-maintained tests.md rows.

Clean units:

- The old oracle is named, a paraphrase faithful to main's slice text.
- The new boundary is named.
- The old tests are named and kept pending parity.
- The AC-24 ignored-inner-EOF oracle stays separate.
- No new cap is selected. No external authority or right is allocated.
- The IR-682 text is not contradicted.
- The AC ids do not collide.
- AC-1..50 and procedure steps 1-27 are byte-unchanged.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The slice-2 owed list for AC-8/10/22 silently drops main's "distinct O-origin inner-confirm/M-reap/outer-confirm observations". The new row keeps only "I-confirm/M-reap/seal order", and outer-confirm no longer appears anywhere in TC-049. The retirement paragraph also does not name the "stronger confirmed whole-outer-tree guarantee" that TC-049:142-145 requires. It names inner I, M and descendant confirmation before outer escalation. The delta does not meet the rule it claims to meet. | spec/kani/matrix/TC-049-caller-death-ownership.md:90; spec/kani/matrix/TC-049-caller-death-ownership.md:142-145; spec/kani/matrix/TC-049-caller-death-ownership.md:148-155 |
| FND-002 | medium | "Measured replacement parity" is not defined. The old assertions stay "until measured genuine replacement controls pass and their independently targeted mutants fail", but nothing names which mutants the replacement must kill. The old test's unsampled-late-fork and sampled-PID teardown mutant is in none of AC-51..54's mutant lists, so parity can be declared and the old test retired while that mutant is no longer killed. No acceptance authority or gate is named either. | spec/kani/functional/FR-034-caller-death-ownership.md:1149-1151; spec/kani/matrix/TC-049-caller-death-ownership.md:132-134; spec/kani/matrix/TC-049-caller-death-ownership.md:153-155 |
| FND-003 | medium | The AC-51..54 expected results sit in a new H3 inside "## Settlement reserve research receipt", not in the "## Expected Results" table that holds the AC-31..50 rows. The unchanged Expected Results row "FR-034-AC-8/9 ... Kill only sampled PIDs" still claims a regression catch whose only witness this PR retires. | spec/kani/matrix/TC-049-caller-death-ownership.md:1104-1114; spec/kani/matrix/TC-049-caller-death-ownership.md:1015-1048; spec/kani/matrix/TC-049-caller-death-ownership.md:1020 |
| FND-004 | low | tests.md widens the FR-034 and TC-049 rows to AC-1..54, which also adds AC-41..50 (absent from both rows on main). This is beyond the declared four-criterion scope. The status cells are unchanged and still name only IR-670 AC-39 and IR-675 AC-40 as UNRUN, with no IR-682 AC-41..50 or IR-655 AC-51..54 PLANNED/UNRUN. | spec/kani/matrix/tests.md:56; spec/kani/matrix/tests.md:87 |

## Verdict

Not merge-ready. FND-001 is a silent deletion that conflicts with TC-049's own retirement rule.
FND-002 leaves the gate for retiring the old test unable to fail on the retired mutant. FND-003
and FND-004 are layout and status drift.

## Dispositions

Round 1. Branch head "Strengthen stage-two mutation and temporal predicates"; its revision is
recorded in the Linear marker only.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | TC-049:90 restores "distinct O-origin inner-confirm/M-reap/outer-confirm observations" and adds "Outer confirmation and confirmed whole-outer-tree termination remain separately owed; inner settlement never supplies them". The retirement paragraph (TC-049:156-158), FR-034:1113-1115 and step 28 name the stronger confirmed whole-outer-tree guarantee. |
| FND-002 | fixed | FR-034:1168-1176 and step 28 name the parity set: sampled-membership teardown, fabricated live-I IN, fabricated M reap, I poll after seal, M reap after seal, gate close before I confirmation, omitted gate-path I confirmation, ignored inner EOF. The CG CODE author supplies per-predicate receipts and the independent CODE reviewer accepts them. |
| FND-003 | fixed | The Stage-2 expectations table is now an H3 under "## Expected Results" (TC-049:1070-1083), before "## Settlement reserve research receipt" (1085). The sampled-PID regression now has an explicit AC-55 row (its adequacy is SR-3020 FND-007). |
| FND-004 | fixed | The tests.md:56 and 87 status cells now name IR-682 AC-41..50 Analysis UNPROVEN and IR-655 AC-51..56 PLANNED/UNRUN. The AC-41..50 index fix is declared in the author's fix map. |

### Verdict (disposition pass 1)

All four findings fixed. Regression checks: FR-034 AC-1..50 (including AC-8 and AC-10) and
TC-049 steps 1-27 are byte-identical to main. AC-55 and AC-56 are unique, and AC-57..64 are
unused, so they stay disjoint from IR-687. No SHAs, local paths or conflict markers were added.
`quire validate` exits 0 on all three documents with the same seven module-loader notices.
