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
retained-evidence reader are gone; that the two schemas retained records once named by digest are
deleted with those records and referenced by nothing; that the three live domain schemas are still
named by the generator; and that the gates which replaced them are reachable from `ci` rather than
merely defined.

## Test Procedure

Assert each removed file is absent by name, including the retained tree itself and the two schemas
frozen only for the records that named them. Assert each live domain schema is present and named by
`src/oracle.rs`, so a later tidy-up cannot delete one on the strength of the directory it shares.
Walk the repository's executable and configuration surfaces and require none of them to name a
deleted artifact. Ask Make what `ci` would run and require the plan to name each replacement gate and
the test runner itself.

## Expected Results

Every named file is gone, every live domain schema is present and included by the generator, no
executable or configuration file references a deleted artifact, and `make -n ci` names each gate and
the test runner. The Makefile
declares no directive whose only purpose is to stop a failure propagating, which protects a diff
review rather than an exit code — the residual is recorded, not closed.
