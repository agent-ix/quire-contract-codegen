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
admitted package with enum declarations (eq/ne over any declaration, ordering over an `ordered enum`), public CG generation, installed proof
execution and owner replay. This case is planned; the existing native enum
comparison oracle checks do not establish proof acceptance. Execute generated
proofs only with the authorized runner and budget.

## Test Procedure

1. Admit a package with an `ordered enum` of at least three members and an
   unordered enum declaration. Request `eq` and `ne` for both, and `lt`, `le`,
   `gt`, `ge` for the ordered enum. Retain source association and inspect that
   oracle and native expression derive from the same node (AC-1).
2. Independently enumerate member pairs and compare with the harness domain:
   every ordered pair reachable, no out-of-range `i64` index. Apply mutations
   adding an index, dropping the last member and fixing one operand; each must
   fail the correspondence check (AC-2).
3. Inspect generation: `from_member` runs over the owner route's admitted
   `EnumShape` and checked members in declared order, and the shape passed is
   compared with the owner-admitted one; inside the proof operands come from
   `from_index`. Present a minted identity and assert `MemberNotInShape` and no
   harness (FR-407-AC-3 owns the refusal itself). Mutate the generator to emit
   a renamed key in both its shape and its members and assert the comparison
   with the owner-admitted shape fails. Mutate to a reordered member list and
   assert the ordering operators over the ordered enum fail (AC-3).
4. Inspect assertions and assumptions: completion required, no outcome-based
   assume. Exercise a reachable `Incomplete` outcome and assert it keeps its
   typed outcome; force the `OracleStop::IllTyped` path and assert a
   non-`Verified` outcome retaining the cause (AC-4).
5. Run the healthy harnesses through the approved bounded path with full
   ordered options; verify all checks and exactly one final cover. Run the
   mutants, each against the operator that can detect it: `lt`/`le` swaps,
   `gt`/`ge` swaps, `eq`/`ne` swaps, reversed ordering. Retain failed checks
   and playback and replay the in-domain indices through the owner route (AC-5, AC-6).
6. Check separate controls for unsatisfied cover, failed unwind, timeout,
   memory exhaustion and missing verdict; none may be `Verified` (AC-7).
7. Request an ordering operator over the unordered enum, operands of different
   declarations, and a declaration with an incomplete member list beside a
   supported sibling. Assert named dispositions, no harness and an unchanged
   sibling. Request text, rational, decimal, quantity and composite nodes,
   including an enum leaf inside a composite, and assert their dispositions are
   unchanged (AC-8, AC-9).
8. Inspect the physical schema: one `i64` per enum operand with declaration,
   member list and bounds. Perturb retained playback to an out-of-range index
   and to missing, duplicate and foreign bindings, and assert typed refusal with
   no replay; assert an in-range index replays as the member at that position
   (AC-10).
9. Inspect the run record for node and declaration identity, ordered members and
   ordered flag, operator, encoding, full argv with unwind options, tool
   versions, target, limits, checks, cover, playback, exit and terminal outcome
   (AC-11).

## Expected Results

The symbolic encoding is a bijection onto the declared members; every operator
verifies against its native expression, and each mutant fails with an in-domain
replayable disagreement. Refused, noncomplete or unrun controls keep their own
classification and receive no proof credit.

## Status

PLANNED (IR-1438): all eleven FR-049 criteria are unimplemented and untagged.
