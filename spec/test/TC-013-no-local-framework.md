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
types, verifier, anchor writer, failure-propagation policer, and coverage reimplementation are gone;
that the schemas retained records name by digest are frozen and referenced by nothing executable; and
that the gates which replaced them are reachable from `ci` rather than merely defined.

## Test Procedure

Assert each removed file is absent by name. Assert each frozen artifact is present and carries its
recorded digest. Walk the repository's executable and configuration surfaces and require none of them
to name a frozen artifact. Ask Make what `ci` would run and require the plan to name each replacement
gate and the test runner itself.

## Expected Results

Every named file is gone, every frozen artifact is unchanged, no executable or configuration file
references a frozen artifact, and `make -n ci` names each gate and the test runner. The Makefile
declares no directive whose only purpose is to stop a failure propagating, which protects a diff
review rather than an exit code — the residual is recorded, not closed.
