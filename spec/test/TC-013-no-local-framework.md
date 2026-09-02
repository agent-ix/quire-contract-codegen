---
id: TC-013
title: "Verify no local evidence framework remains"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-006
    type: verifies
---
# TC-013: Verify no local evidence framework remains

## Description

Verify that the repository-local collector, envelope builder, schema validators for shared record
types, verifier, anchor writer, failure-propagation policer, coverage reimplementation, and
retained-evidence reader are gone; that the three schemas deleted with the formats they described —
the two retained records once named by digest, and the deprecated PGM-01 evidence envelope that
nothing validated against — are absent and referenced by nothing; that the deprecated evidence format
itself is named by no executable or configuration surface, by its schema version string and by the
two serde types that were its only remaining statement; that the two live domain schemas are still
named by the generator; and that the gates which replaced them are reachable from `ci` rather than
merely defined.

## Test Procedure

Assert each removed file is absent by name, including the retained tree itself and the three deleted
schemas. Assert each live domain schema is present and named by
`src/oracle.rs`, so a later tidy-up cannot delete one on the strength of the directory it shares.
Walk the repository's executable and configuration surfaces and require none of them to name a
deleted artifact, over a population held above a floor derived from the tree as it stands rather than
inherited from the tree before the deletion. Ask Make what `ci` would run and require the plan to
name each replacement gate and the test runner itself.

## Expected Results

Every named file is gone, every live domain schema is present and included by the generator, no
executable or configuration file references a deleted artifact, the census population is at or above
its re-derived floor, and `make -n ci` names each gate and the test runner. The live-schema clause is
what keeps this test ranging over a non-empty population: the clauses about deleted material assert
absence, and absence over a population that was just removed would be satisfied by a repository that
had deleted everything. The Makefile
declares no directive whose only purpose is to stop a failure propagating, which protects a diff
review rather than an exit code — the residual is recorded, not closed.
