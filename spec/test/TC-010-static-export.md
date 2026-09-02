---
id: TC-010
title: "Verify the sealed impact snapshot is the Quire export"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-006
    type: verifies
---
# TC-010: Verify the sealed impact snapshot is the Quire export

## Description

Verify that the sealed change-assurance record's impact snapshot is the SHA-256 of the Quire static
export, that the export is a populated document naming every requirement in this repository, and that
the chain read it as a populated export rather than as a run whose result was not computed.

## Test Procedure

Read the chain report's impact-snapshot digest and compare it to an independently computed digest of
the export file. Parse the export and require it to name each requirement identifier.

## Expected Results

The digests agree, the export is a non-empty object naming every requirement, and the export's proof
obligation is attested `passed`. An empty document has a digest too, so the digest alone is not
accepted as evidence that a snapshot happened.
