---
id: TC-011
title: "Verify retained evidence is readable and unmoved"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-006
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/NFR-002
    type: verifies
---
# TC-011: Verify retained evidence is readable and unmoved

## Description

Verify that every retained evidence envelope is read through the pinned Engineering Assurance
mapping, that no retained byte is written by the run and none differs from what was committed, that
the mapping's answer is reported as it stands, and that the mapping is observed accepting as well as
refusing.

## Test Procedure

Run the compatibility view and compare the number of evidence files it read against an independent
walk of the tree. Require the run's own before/after census to show no byte moved and require Git to
report no uncommitted change under `evidence/`. Then run the view's mutation probes, each of which
degrades exactly one load-bearing check.

## Expected Results

Every declared case matches, all retained envelopes report the mapping's real answer for their
schema family, at least one positive control is accepted, no byte moved, and every mutation probe is
detected. "Nothing changed" and "nobody looked" are kept as different answers: the read-only claim is
measured by this process's own census and the committed-bytes claim is answered by Git.
