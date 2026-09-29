---
id: TC-025
title: "Verify separate bounded Kani obligations"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-218
    type: references
---
# TC-025: Verify separate bounded Kani obligations

## Description

Verify that complete-V1 contracts produce one bounded harness per obligation
kind and refuse non-finite obligations.

## Test Procedure

Generate harnesses for a contract with a precondition, postcondition,
invariant and frame condition over bounded scalar domains, and for a contract
with an unbounded domain, one with unsatisfiable bounds, and one over a
caller-declared oracle operation. Inspect harness identities, bounds and
assumptions, and run the installed Kani backend on the bounded harnesses.

Additionally: count the covers in each generated harness and locate the
contract cover relative to the contract call; request a postcondition without
the precondition sharing its anchor; read the solver and the option vector out
of each identity; regenerate from equal inputs and compare bytes; regenerate
with a changed unwind bound and a changed subject and compare identities;
search each harness source for a loop bound; and submit a request with no
items, with more items than the ceiling, with an unparsable subject path, and
with an unwind bound on each side of the admissible range.

## Expected Results

Four distinct harnesses carry their bounds; no assumption
excludes an undefined, refused or incomplete outcome; the unbounded, the
unsatisfiable and the caller-declared obligations are refused with no harness.

Each harness contains exactly one cover, and the contract harness's cover
follows its contract call. Each contract harness carries a `requires` for every
package precondition sharing its anchor and embeds no other obligation's
oracle; the postcondition requested without its sibling precondition is refused
naming that precondition, with no harness. Every identity records solver
`cadical` and the ordered option vector, and no option enables stubbing.
Regeneration is byte-identical, while the changed unwind bound and the changed
subject each produce a different identity. Every symbolic argument
carries an inclusive assumption equal to its IR domain, the post-state result
is required to lie in the same domain, and no harness source contains a loop
bound. Each of the four malformed requests is refused whole, with no item
accounted.

## Implementation

`tests/it/kani_obligations.rs`. The default lane negotiates every item before
any harness is exposed and checks separate harnesses, IR-derived bounds,
assumptions and every refusal. `make kani` runs the ignored lane serially under
a host lock against the installed Kani backend: it verifies the precondition,
postcondition and invariant harnesses, falsifies a seeded postcondition defect
with a concrete counterexample, and reports a contract harness with jointly
unsatisfiable requires as `cover_unsatisfied`.

## State transitions, censuses and dispositions

1. Generate a zero-input transition over one bounded-integer state value, run it for an identity
   subject and for a subject that changes the value. The identity subject verifies, and the
   changing subject is falsified with a concrete counterexample (FR-015-AC-19).
2. Generate a harness for each Boolean connective and each integer comparison claim, and compare
   the embedded oracle with the FR-014 crate's function byte for byte. They are identical
   (FR-015-AC-20).
3. Request a precondition that reads a post-state value. It is refused naming the node, with no
   harness (FR-015-AC-21).
4. Request obligations with a declared census that is empty-identity, duplicate-identity,
   kind/state/path-inconsistent or non-`Required`. Each is refused with no harness (FR-015-AC-22).
5. Request items that settle `supported`, `requires-bound` and `unsupported` together. Each has
   exactly one `ObligationDisposition`, and each item that is not `supported` keeps its source
   identity and reason with no harness and no assumption (FR-015-AC-23).
6. Run a falsifiable plain bounded-integer comparison harness. The falsified run prints a concrete
   playback (FR-015-AC-24).
7. Request obligations with valid censuses whose dependencies are all passed, one missing and one
   failed. Each folds into the identity with readiness `ready`, `incomplete` and `incomplete`, and
   every harness records execution `not_run` (FR-015-AC-25).

## Blocked

- Frame harnesses: FR-025 accounts every frame obligation `unsupported` until QSpec decides how a
  frame node lowers into a Kani form (ADR-004).
- V2 scalar harnesses outside `IntegerArithmetic`: an IR-confirmed claim over
  one of the four `quire.op.integer.{add,sub,mul,negate}` identities reaches a
  real harness unless a ground independent of the operation (an
  i64-unrepresentable endpoint, the source ceiling) displaces it. Every other
  confirmed family is refused as `OperationNotRendered`, an unbuilt renderer in
  this generator, not an upstream block. A claim this generator lowered but
  whose operation it did not confirm against the node's own catalogued
  identity, mode or law definition is refused as `CallerDeclaredOperation`.
- Model and graph bounds: refused as blocked on `agent-ix/quire-spec-language#120`.
