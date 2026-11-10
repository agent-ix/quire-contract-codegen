---
id: TC-063
title: "Verify enum scalar comparison harnesses"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-049
    type: verifies
---
# TC-063: Verify enum scalar comparison harnesses

## Description

Verify [FR-049](../functional/FR-049-enum-scalar-harnesses.md) through an
admitted package with a model-owned enum, public CG generation, installed proof
execution and owner replay. This case is planned; the existing native enum
comparison oracle checks do not establish proof acceptance. Execute generated
proofs only with the authorized runner and budget.

## Test Procedure

1. Admit a package containing an enum of at least three ordered members and one
   comparison node per operator (`eq`, `ne`, `lt`, `le`, `gt`, `ge`). Request
   each harness and retain the source association. Inspect that oracle and native
   expression derive from the same node (AC-1).
2. Independently enumerate member pairs and compare with the harness domain:
   every ordered pair reachable, no out-of-range index. Apply mutations adding an
   index, dropping the last member and fixing one operand; each must fail the
   correspondence check (AC-2).
3. Inspect operand construction from the owner's checked member identities in
   declared order. Mutate to a reordered list and a minted identity; the ordered
   operators must fail (AC-3).
4. Inspect assertions and assumptions: completion required, no outcome-based
   assume. Exercise reachable non-`Completed` outcomes and assert they keep their
   typed outcome (AC-4).
5. Run the healthy harnesses through the approved bounded path with full ordered
   options. Verify all checks and exactly one final cover. Run each mutant
   (`lt`/`le`, `gt`/`ge`, `eq`/`ne` swaps, reversed order, reordered member list),
   retain the failed check and playback, and replay the in-domain indices through
   the owner route (AC-5, AC-6).
6. Check separate controls for unsatisfied cover, failed unwind, timeout, memory
   exhaustion and missing verdict; none may be `Verified` (AC-7).
7. Request cross-enum operands, a non-member operand, a zero-member enum, a
   member count above the index bound and an unbounded member set beside a
   supported sibling. Assert named dispositions, no harness and an unchanged
   sibling. Request a text, rational, decimal, quantity and composite node and
   assert their dispositions are unchanged (AC-8, AC-9).
8. Inspect the run record for node and enum identity, ordered members, operator,
   encoding, full argv with unwind options, tool versions, target, limits, checks,
   cover, playback, exit and terminal outcome (AC-10).

## Expected Results

The symbolic encoding is a bijection onto the declared members; every operator
verifies against its native expression, and each mutant fails with an in-domain
replayable disagreement. Refused, noncomplete or unrun controls keep their own
classification and receive no proof credit.

## Status

PLANNED (IR-1438): all ten FR-049 criteria are unimplemented and untagged.
