---
id: TC-008
title: "Verify the shared component pins through the packaged matrix"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-006
    type: verifies
---
# TC-008: Verify the shared component pins through the packaged matrix

## Description

Verify that every adopted component is classified by the packaged Engineering Assurance compatibility
matrix, that the artifacts this repository reads from the pinned release still carry their pinned
digests, that no install line names the internal mirror or a version the matrix calls incompatible,
and that the acceptance state is reported rather than gated on.

## Test Procedure

Run the shared-pin gate in the pinned interpreter and read its report. Then feed the gate a pins
document carrying a mirror reference and require it to report an offender, so the check is observed
refusing as well as accepting. Then classify a version the matrix names incompatible and one it has
never seen, and require only the incompatible one to be ruled out. Finally run the gate with a fake
`quoin` on `PATH` that reports a version the matrix names incompatible, and require the report's
`accepted` to be false and the exit status 1.

## Expected Results

Four components are classified, none of them `incompatible`; an installed version the matrix has
never seen is reported as `unknown` and does not fail the gate. No consumed artifact digest differs,
the mirror and incompatible-install scans are empty for the tree as committed and non-empty for the
injected case, and the acceptance state is present as a reported string with
`acceptance_recorded_here` false. The injected incompatible component makes `accepted` false.
