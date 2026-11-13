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
---
# FR-055: Require every generated harness's mutant to fail in a kani-mutations target

## Description

A proof that has never been seen to fail cannot be told from a proof that cannot fail. Each
generated Kani harness already carries a mutant: a deliberate source defect that its harness must
reject ([FR-041](./FR-041-claimed-module-proof-gate.md), and the mutant controls of the harness
families FR-046 to FR-054). This requirement gives those mutants one run: a `make kani-mutations`
target that injects every generated harness's mutant and fails when any mutant still verifies. It
uses the vocabulary and contract shape of the sibling `quire-contract-runtime` target of the same
name: an injected defect is applied to a scratch copy of the source, never to the working tree; a
row passes when the owning proof rejects the defect; a surviving mutant (the proof accepts the
defect) fails the target.

The target is separate from `make kani-gate`. FR-041's claimed-module controls remain part of the
gate's own lane (NFR-006-AC-19); this target neither replaces nor is replaced by them. Unlike
`make kani` and `make kani-gate`, it is part of `make ci` (FR-055-AC-3), and NFR-006 is amended to
allow exactly that (FR-055-AC-8).

Running the target executes installed Kani (0.68.0 on terra) and is heavy. It is run through the
Kani lane, under the host lock and serially, as `make kani-gate` is. The requirement is
independent of that execution detail.

## Inputs

- The set of generated harnesses and, for each, its declared mutant: the file, the exact anchor
  text, the replacement, and the owning proof.
- The installed `cargo-kani` launcher.

## Outputs

- A listing of every mutant run, one line each, naming the harness, the mutant and its outcome.
- Exit status zero only when every listed mutant was rejected.

## Behavior

- Each mutant has exactly one outcome: `pass` (the owning harness reported a failed check because
  of the defect), `fail` (the harness verified the defect, a surviving mutant), `malformed` (the
  anchor text no longer occurs in the source exactly once, or the mutant does not compile) or
  `unavailable` (no `cargo-kani`, so nothing was injected).
- A mutant that times out, is inconclusive, is skipped, or fails only a dirty-tree or other
  process guard is not `pass`.
- The target never edits the working tree.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|-----------|--------------|
| FR-055-AC-1 | `make kani-mutations` exists. Run with every generated harness's mutant, it injects each mutant into a scratch copy of the source, runs the owning proof through Kani, and exits zero only when every mutant is `pass`. The working tree is byte-identical before and after. | Test |
| FR-055-AC-2 | The target prints one line per mutant run, naming the harness, the mutant and its outcome (`pass`, `fail`, `malformed` or `unavailable`), followed by a total. A mutant that was not run does not appear as run. Mutant: omitting the line of a mutant that ran fails. | Test |
| FR-055-AC-3 | `make -n ci` includes the `kani-mutations` target. | Test |
| FR-055-AC-4 | When a mutant still verifies (a surviving mutant), the target prints that mutant as `fail` and exits non-zero. Mutant: exiting zero after a `fail` row fails. | Test |
| FR-055-AC-5 | A mutant whose anchor text is absent or occurs more than once, one that does not compile, and one that times out or is inconclusive each give an outcome other than `pass` and a non-zero exit. | Test |
| FR-055-AC-6 | With no `cargo-kani` on `PATH`, every mutant is `unavailable` and the target exits non-zero; an absent toolchain never reads as success. | Test |
| FR-055-AC-7 | Deliberately weakening one generated harness (for example replacing its checked proposition with a trivially true one) while leaving its mutant unchanged makes that mutant verify; the target then reports that mutant as `fail` and exits non-zero, and restoring the harness returns it to zero. Executed through installed Kani in the Kani lane. | Test |
| FR-055-AC-8 | NFR-006-AC-12 and TC-045 step 12 state that `make -n ci` expands to no `kani-gate` command and no `kani` lane command other than `kani-mutations`. | Inspection |

## Dependencies

- [FR-017](./FR-017-kani-execution-evidence.md) owns Kani execution.
- [NFR-006](../non-functional/NFR-006-real-kani-lane-gating.md) owns the lane, its lock and the
  `make ci` exclusion amended by FR-055-AC-8.
- [FR-041](./FR-041-claimed-module-proof-gate.md) owns the gate's own module mutation controls.
- [TC-069](../matrix/TC-069-kani-mutations-target.md) defines the planned verification.

## Status

PLANNED (IR-1437): no `kani-mutations` target exists in CG; no criterion is implemented or tagged.
Execution of FR-055-AC-1 and FR-055-AC-7 needs installed Kani and runs through the Kani lane.
