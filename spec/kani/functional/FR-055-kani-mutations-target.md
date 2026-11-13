---
id: FR-055
title: "Require every generated harness's mutant to fail in a kani-mutations target"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/NFR-006
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-041
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-046
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-047
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-048
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-049
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-050
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-051
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-052
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-053
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-054
    type: depends_on
---
# FR-055: Require every generated harness's mutant to fail in a kani-mutations target

## Description

A proof that has never been seen to fail cannot be told from a proof that cannot fail. Each
generated Kani harness carries a mutant: a deliberate source defect that its harness must reject
([FR-041](./FR-041-claimed-module-proof-gate.md), and the mutant controls of the harness families
FR-046 to FR-054). This requirement gives those mutants one run.

The build shall provide a `make kani-mutations` target that injects each declared mutant into a
scratch copy of the source, never the working tree, runs the owning proof through Kani, and fails
when any mutant still verifies. The vocabulary and contract shape follow the sibling
`quire-contract-runtime` target of the same name, which is reference only: no mutation table or
code is copied from it.

**Mutant declaration.** This repository shall hold one mutant declaration file (its path is an
implementation choice, named in the Makefile recipe). Each entry names the file, the exact anchor
text, the replacement and the owning proof (harness). The owner of each harness family (FR-041 for
the claimed-module controls, FR-046 to FR-054 for the family controls) declares that family's
entries in the same change that adds the harness; the declaration file is the only source of "every
generated harness's mutant".

**Relation to `make kani-gate`.** The target is separate from `make kani-gate`. FR-041's
claimed-module controls remain part of the gate's own lane (NFR-006-AC-19); this target neither
replaces nor is replaced by them. This target is the Kani-affecting merge gate alongside `make kani-gate`.

**Relation to `make ci`.** Like `make kani` and `make kani-gate`, the target is standalone and is
not part of `make ci` (NFR-006 is unchanged). It is run as a Kani-affecting merge gate, alongside
`make kani-gate`.

**Execution.** The target executes installed Kani (0.68.0 on terra), under the host lock and
serially, as `make kani-gate` is.

**Bounded cost.** The per-mutant timeout and the total wall-clock budget of one run are
configuration values of this target. The owner sets their values; this requirement names them and
does not fix a number. The target runs outside `make ci`, so its cost is not charged to every change.

## Statements

- The `kani-mutations` target shall exit zero only when every declared mutant has the outcome `pass`.
- The `kani-mutations` target shall refuse to run, with exit status 2, when the declaration file
  holds fewer than three mutants (the floor), including zero.
- Where a per-mutant timeout or the total wall-clock budget is exceeded, the target shall report
  the affected mutants as `fail` and exit non-zero.
- Where `cargo-kani` is absent, the target shall report every declared mutant as `unavailable` and
  exit non-zero.
- The `kani-mutations` target shall not edit the working tree.

## Inputs

- The mutant declaration file: for each mutant the file, the exact anchor text, the replacement and
  the owning proof.
- The installed `cargo-kani` launcher, and the per-mutant timeout and total budget configuration.

## Outputs

- A listing of every declared mutant, one line each, naming the harness, the mutant and its outcome,
  and a total.
- Exit status zero only when every listed mutant was rejected.

## Behavior

- Each mutant has exactly one outcome:
  - `pass`: the owning harness printed `VERIFICATION:- FAILED` because of the defect.
  - `fail`: the harness verified the defect (a surviving mutant), the mutant did not compile, the
    run exited non-zero without the owning harness printing `VERIFICATION:- FAILED`, or the run
    timed out or was inconclusive.
  - `malformed`: the anchor text does not occur in the source exactly once.
  - `unavailable`: no `cargo-kani`, so nothing was injected.
- A mutant that is skipped, or that fails only a dirty-tree or other process guard, is not `pass`.
- A declaration with fewer than three mutants is refused with exit status 2 before any run.
- The target never edits the working tree.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|-----------|--------------|
| FR-055-AC-1 | `make kani-mutations` exists. Run with the declared mutants, it injects each into a scratch copy of the source, runs the owning proof through Kani, and exits zero only when every mutant is `pass`. The working tree is byte-identical before and after. | Test |
| FR-055-AC-2 | The target prints one line per declared mutant, naming the harness, the mutant and its outcome (`pass`, `fail`, `malformed` or `unavailable`), followed by a total. Under `unavailable` (FR-055-AC-6) the rows are printed and counted in the total as `unavailable`; no row is printed for a mutant that is not declared. Mutant: omitting the line of a declared mutant fails. | Test |
| FR-055-AC-3 | The prerequisite and recipe list of the `ci` target does not include `kani-mutations`, directly or through its prerequisites. | Test |
| FR-055-AC-4 | When a mutant still verifies (a surviving mutant), the target prints that mutant as `fail` and exits non-zero. Mutant: exiting zero after a `fail` row fails. | Test |
| FR-055-AC-5 | A mutant whose anchor text is absent or occurs more than once is `malformed`; one that does not compile, one whose run exits non-zero without the owning harness printing `VERIFICATION:- FAILED`, and one that times out or is inconclusive are `fail`. Each gives a non-zero exit. | Test |
| FR-055-AC-6 | With no `cargo-kani` on `PATH`, every declared mutant is `unavailable` and the target exits non-zero; an absent toolchain never reads as success. | Test |
| FR-055-AC-7 | Deliberately weakening one generated harness (for example replacing its checked proposition with a trivially true one) while leaving its mutant unchanged makes that mutant verify; the target then reports that mutant as `fail` and exits non-zero, and restoring the harness returns it to zero. Executed through installed Kani in the Kani lane. | Test |
| FR-055-AC-8 | `kani-mutations` is documented as a standalone Kani-affecting merge gate alongside `make kani-gate`, and NFR-006 and TC-045 are unchanged by this requirement. | Inspection |
| FR-055-AC-9 | The mutant declaration file holds, for every entry, the file, anchor, replacement and owning proof. With fewer than three declared mutants, including none, the target prints a refusal, runs nothing and exits 2; an empty declaration never reads as success. | Test |
| FR-055-AC-10 | A mutant that exceeds the per-mutant timeout, and a run that exceeds the total wall-clock budget, are reported as `fail` and the target exits non-zero. Both values are configuration of the target. | Test |

## Dependencies

- [FR-017](./FR-017-kani-execution-evidence.md) owns Kani execution.
- [NFR-006](../non-functional/NFR-006-real-kani-lane-gating.md) owns the lane, its lock and the
  `make ci` exclusion, which this target also observes.
- [FR-041](./FR-041-claimed-module-proof-gate.md) and FR-046 to FR-054 own the mutant controls
  whose entries the declaration file holds.
- [TC-069](../matrix/TC-069-kani-mutations-target.md) defines the planned verification.

## Status

IMPLEMENTED on the host (IR-1437): `make kani-mutations` (`scripts/kani_mutations.rs`, declaration
`scripts/kani-mutants.json`) is tagged and tested for FR-055-AC-2 to AC-6, AC-8, AC-9 and AC-10 and for
the stand-in form of AC-1 (`tests/it/kani_mutations.rs`). FR-055-AC-7, and AC-1 against real proofs,
need installed Kani and have not been run.
