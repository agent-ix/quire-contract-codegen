---
id: TC-064
title: "Verify bounded quantity scalar operation harnesses"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-050
    type: verifies
---
# TC-064: Verify bounded quantity scalar operation harnesses

## Description

Verify [FR-050](../functional/FR-050-quantity-scalar-harnesses.md) through
admitted quantity packages (conditional on the K-EXACT-quantity carrier, IR-1443, and an upstream quantity bound mechanism (IR-1497), neither delivered), public CG generation and installed proof execution.
This case is planned; FR-014 oracle agreement does not establish Kani acceptance.
Execute generated proofs only with the authorized runner and budget.

## Test Procedure

1. Admit packages covering quantity `add`, `sub`, `mul`, `div`, `pow` with a literal
   exponent and each comparison over simple units, plus the compound-unit and
   carried-result forms reused from FR-014 (#750-#754). Request each harness and assert
   the operation, operand forms and units are those of the admitted node. Request a
   conversion node and assert it is not selected (AC-1).
2. Independently enumerate each operand's magnitude domain with exact host arithmetic
   and compare it with the carrier encoding for soundness, injectivity and completeness.
   Mutate by clipping an endpoint, admitting a duplicate or extra value; each fails the
   correspondence check. Inspect that units are fixed, resolved through the unit table
   or scope and never symbolic or CG-minted (AC-2).
3. Compare the generated arithmetic oracle with an independent native expression over
   public `evaluate_quantity_unit` from matching entry state and limits: completed
   value, identified result unit, outcome, ordered charges, denied next charge and
   counters at zero, exact and short budgets. Compare comparisons on `Outcome<bool>`,
   charges and counters only (AC-3).
4. Exercise `IllTyped` dimension mismatch, `1 m + 1 cm` distinct units, `Identity`,
   affine arithmetic, division by a zero quantity, zero base with a negative literal
   exponent and a large literal exponent (the owner's `IntegerBits` limit-incomplete stop at
   `UnitRationalArithmetic`). Assert each agreed typed outcome and
   charges. Inspect both harnesses for no outcome-based assume (AC-4).
5. Request a missing, unbounded or repeated domain, a non-literal exponent and an
   endpoint beyond carrier capacity. Remove the carrier and, separately, the upstream
   bound mechanism and assert `OperationNotRendered` (the `NoRenderer` settlement carries no prerequisite name; assert a name only where a typed reason carries one).
   Exercise an unsupported operand form, an absent native expression and a quantity
   composite leaf beside a supported sibling and assert total accounting (AC-5, AC-6).
6. Run the healthy harness through the approved bounded production path with its
   complete ordered options. Verify all checks and exactly one final cover. Separately
   force an unsatisfied cover, failed unwind, timeout and missing verdict and assert
   none is `Verified` (AC-7).
7. Run the AC-8 mutants one at a time over control domains stating their distinguishing
   points: wrong operator with nonzero second operand; swapped `sub`, `div`, `lt`,
   `le`, `gt`, `ge` with distinct nonzero operands (no swap for `eq`/`ne`); `lt` for
   `le` and `gt` for `ge` with equal operands; wrong result unit; different literal
   exponent over a base other than 0, 1 and -1; dropped or reordered charge; stopped
   reported completed. Each must fail with actual playback (AC-8).
8. Perturb retained witnesses (out of domain, noncanonical, duplicate, role-swapped,
   foreign, malformed width or comment, over entry-count or byte bound) and assert
   typed refusal with no replay; for a valid witness assert one owner quantity per
   parameter reaches public replay for that declaration (AC-9).
9. Inspect the run record for operation and source association, domains and
   correspondence, literal exponent, fixed units, argv, unwind bounds, tool versions,
   target, dependency cohort including the carrier, limits, checks, cover, playback,
   exit and terminal classification (AC-10).

## Expected Results

Healthy operations verify with a satisfied cover; every mutant and refusal retains
its own classification and receives no proof credit. Until the carrier and the
generated-proof path run, IR-1439 remains unaccepted regardless of oracle agreement.

## Status

PLANNED: all ten FR-050 criteria are unimplemented and untagged. No missing
carrier capability or generated-proof result is waived.
