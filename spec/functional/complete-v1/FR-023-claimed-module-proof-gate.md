---
id: FR-023
title: "Gate a proof on discharged checks inside each claimed module"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-023: Gate a proof on discharged checks inside each claimed module

## Description

When a caller applies the claimed-module gate to one Kani run, the code
generator shall report the run as proving the claimed modules only when the run
verified and, for each claimed module, the prover's transcript lists at least
one check with status `SUCCESS` located inside that module. A claimed module
the prover compiled but reached no proposition in is reported as `unreached`,
and the gate fails.

This is QSL ADR-011 section 2.3 items 1 and 2 for the skeleton spine: the
generated obligation module and the function under proof are the modules the
spine claims, and the injected violation is the mutation control. A gate that
counts only that the run verified is satisfied by a proof that reached no code
it claims, which is why the count is per claimed module.

## Inputs

- The claimed modules: a list of Rust module paths, checked in beside the test
  that applies the gate, in the form the prover prints a check's location.
- The run's classified outcome and the prover's printed transcript, which
  `execute_kani_obligation_with_transcript` returns beside the FR-017 evidence.

## Outputs

- One report per claimed module, in claim order, saying how many `SUCCESS`
  checks the transcript lists inside it.
- A typed failure: no module claimed, the run did not verify, or the claimed
  modules the prover reached nothing in.

## Behavior

- A check counts as discharged only when its status is `SUCCESS`. `UNREACHABLE`,
  `UNDETERMINED`, a cover's `SATISFIED` and every status the reader does not name
  discharge nothing.
- A check is inside a claimed module when the function its location names is the
  module or is nested under it, matched on whole path segments.
- If the run's outcome is not verified, then the gate shall fail with that
  outcome and shall not consult the transcript, so a falsified run is red however
  many checks succeeded.
- If no module is claimed, then the gate shall fail, because it would discharge
  nothing.
- A violation injected inside a claimed module shall turn the gate red.

The spine applies this gate in a `make kani` test, which is an ignored test
outside `make ci`; the gate's own logic is tested in the default suite.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-023-AC-1 | The gate passes only for a verified run in which every claimed module has at least one `SUCCESS` check located inside it, and reports the count per module. | Test (TC-034) |
| FR-023-AC-2 | A claimed module with no `SUCCESS` check inside it, whether it has no check, only `UNREACHABLE`, `UNDETERMINED` or `SATISFIED` checks, or is compiled and never called, is reported `unreached` and fails the gate. | Test (TC-034) |
| FR-023-AC-3 | A violation injected inside the generated obligation module, and another inside the function under proof's module, each turn the gate red on a real prover run. | Test (TC-034) |
| FR-023-AC-4 | A module claim matches whole path segments, and an empty claim list fails the gate. | Test (TC-034) |

## Dependencies

- **Upstream**: [FR-017](./FR-017-pinned-kani-execution-evidence.md), whose run
  and classification the gate reads, and
  [interface-001](../../interface/interface-001-codegen-api.md).
- **Downstream**: [TC-034](../../test/complete-v1/TC-034-claimed-module-proof-gate.md).
