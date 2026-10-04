---
id: "SR-1438"
title: "CG PR 263 integrity: FR-031 and TC-044 structure, numbering and trace"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@e27fe84435e45db9987a40c39f0b5549c28c5c09; spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md, spec/oracle/matrix/TC-044-boolean-oracle-integer-arithmetic.md, spec/oracle/matrix/tests.md, spec/tests.md, spec/spec.md, spec/assurance/AD-004-cg-crate-layout.md, spec/core/matrix/TC-003-unsupported-diagnostics.md, spec/core/functional/interface-001-codegen-api.md; context: spec/kani/functional/FR-030-ir-outcome-terminal-map.md; diff origin/main...HEAD, merge base 83f9687"
---

# SR-1438: CG PR 263 integrity: FR-031 and TC-044 structure, numbering and trace

## Summary

Ticket: IR-596. Numbering, trace and index integrity were checked.

- **Numbering.** `git log --all -- '*FR-031*' '*TC-044*'` shows only this commit. Every earlier "FR-031" in CG history is Contract IR's, qualified as such. No open CG PR adds FR-031 or TC-044.
- **ACs and trace.** AC-1..AC-11 are contiguous. TC-044 `verifies` FR-031. The oracle matrix row and the TC-044 inventory row list all 11 ACs.
- **Indexes.** spec/tests.md and spec/spec.md rows were added. The AD-004 layout line names FR-031 against boolean_v1.rs. TC-003 now names `saturate` arithmetic instead of all numeric arithmetic, consistent with FR-031.
- **Honest markers.** Every AC is "PLANNED (IR-596)", and the TC and matrix rows say "Planned; no test exists yet".
- **No ceremony.** No SHAs, pins or line citations were introduced, and there is no compatibility wording.

## Verdict

Structurally sound. There are two low findings: one on atomicity and one on a now-ambiguous bare reference.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-3 bundles four independently failing claims: signature shape, byte-identity with today's output, regeneration determinism, and the absence of unwrap/expect/panic. AC-4 bundles a native held agreement with a defect check on arithmetic operands. A failure cannot be attributed to one claim; split them, or name sub-clauses in TC-044 | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:212-213 |
| FND-002 | low | FR-030 cites Contract IR's criterion as bare "FR-031-AC-5". With CG's own FR-031-AC-5 now existing, it reads as CG's criterion; qualify it as "Contract IR FR-031-AC-5" | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:32 |

## Dispositions

Round 1 was reviewed at 8435fbeb50f2b817315902736ccdbff921e26a6b.

- **FND-001.** AC-3 is now signature only. Byte-identity, determinism and the panic scan are AC-12, AC-13 and AC-14. AC-4 is now the arithmetic-free agreement, and the arithmetic-operand emission is AC-15. TC-044 step 2 states these are separate tests.
- **FND-002.** FR-030:32 now reads "Contract IR FR-031-AC-5".
- **Index rows.** The matrix splits FR-031 into a closing row (AC-1, AC-3..5, AC-7..15, AC-18) and a held row (AC-2, AC-6, AC-16, AC-17). Together they partition AC-1..18, and the TC-044 inventory lists all 18.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 8435fbe |
| FND-002 | fixed | 8435fbe |
