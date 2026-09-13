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

Verify that strategy generation for a bound clause takes its domains from the IR integer declarations,
admits exactly the single-compare slice, and refuses every other form with a located diagnostic and
no bundle.

## Test Procedure

1. Build `BoundPackage` fixtures through the public IR API at the pinned revision:
   - ConfigVersion `VersionUnchanged` (state `versionNumber`, 0..=1000, read at `Pre` and `Post`,
     `Post` anchor);
   - `amount < 7` (0..=1000);
   - one fixture per refused form: `Numeric`, `NumericNegate`, `And` over two numeric compares, a
     non-empty obligation list, a `Saturate` declaration, and a string-typed declaration.
2. Request strategy generation for each fixture, and for a `ClauseRef` absent from the package.
3. Separately, run bound oracle generation over the same fixtures.

## Expected Results

- `VersionUnchanged` and `amount < 7` produce the census stated in FR-008-AC-1 and FR-008-AC-2.
- Each refused fixture returns the stated code (`UnsupportedRelation`, `UnsupportedObligations`,
  `UnsupportedOverflowPolicy`, `UnsupportedDeclarationType`, or `UnknownClause`) with the full
  `ClauseRef` and the offending span, and emits no artifact.
- Every fixture refused by oracle generation is also refused by strategy generation.
