---
id: TC-017
title: "Verify bound-clause domain derivation and refusal"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-008
    type: verifies
---
# TC-017: Verify bound-clause domain derivation and refusal

## Description

Verify that bound strategy generation takes its domain from the IR integer declarations, admits
exactly the oracle-admitted single-comparison slice, and refuses every other clause in the stated
order with a located diagnostic and no bundle.

## Test Procedure

1. Build `BoundPackage` fixtures through the public IR API at the pinned revision:
   - ConfigVersion `VersionUnchanged` shaped as quire-spec-language FR-034 projects it
     (`Postcondition`, one state declaration for the `versionNumber` field named by an SL-style field
     alias `SymbolName`, 0..=1000, `Post` left, `Pre` right);
   - SL's `integer-healthy` invariant `amount < 7` (0..=1000);
   - oracle-refused clauses: a `Numeric` addition, a `NumericNegate`, and a clause carrying a
     definedness obligation;
   - strategy-refused clauses: a total-and connective over two integer comparisons, `amount == amount`,
     a Boolean equality, Boolean literal/reference/negation roots, a Text operand, a literal-only
     comparison, and a comparison of `Current` and `Pre` reads of one declaration;
   - an `Assertion` clause and a `Case` clause over `amount < 7`;
   - a clause that is both an `Assertion` and a Boolean connective, and one that is both an
     `Assertion` and oracle-refused;
   - an oracle-refused package requested through an absent `ClauseRef`.
2. Request strategy generation for each fixture, and for a `ClauseRef` absent from the package.
3. Run bound oracle generation over the oracle-refused fixtures.

## Expected Results

- `VersionUnchanged` and `amount < 7` produce the census stated in FR-008-AC-1 and FR-008-AC-2.
- Every fixture reaches the generator as a `quire_contract_ir::BoundPackage` decoded through the
  public API at the pinned revision, and each read's inclusive domain in the generated source equals
  its `IntegerType` minimum and maximum, so a generator holding a private decoder or a local copy of
  the IR wire shape cannot produce these results (FR-008-CON-1).
- Each oracle-refused fixture returns `UnsupportedClause` with the same lower-level code, terminal
  state, and span that step 3 reports.
- Each strategy-refused fixture returns `UnsupportedRelation` with terminal state `unsupported`.
- The `Assertion` and `Case` fixtures return `UnsupportedClauseKind`; the absent `ClauseRef` returns
  `UnknownClause` with `invalid-input`.
- The combined `Assertion` connective returns only `UnsupportedClauseKind`.
- Every refusal carries the full `ClauseRef`, and no refusal emits an artifact. Expression failures
  carry the exact offending span; `UnknownClause` and `UnsupportedClauseKind` carry no span.
