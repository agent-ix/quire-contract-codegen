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

Verify that complete-V1 contracts produce one pinned, bounded harness per
obligation kind and refuse non-finite obligations.

## Test Procedure

Generate harnesses for a contract with a precondition, postcondition,
invariant and frame condition over bounded scalar domains, and for a contract
with an unbounded domain, one with unsatisfiable bounds, and one over a
caller-declared oracle operation. Inspect harness identities, bounds, every pin
and assumptions, and run the pinned Kani backend on the bounded harnesses.

Additionally: count the covers in each generated harness and locate the
contract cover relative to the contract call; request a postcondition without
the precondition sharing its anchor; read the solver and the option vector out
of each identity; regenerate from equal inputs and compare bytes; regenerate
with a changed unwind bound and a changed subject and compare identities;
search each harness source for a loop bound; and submit a request with no
items, with more items than the ceiling, with an unparsable subject path, and
with an unwind bound on each side of the admissible range.

## Expected Results

Four distinct harnesses carry their bounds and backend pin; no assumption
excludes an undefined, refused or incomplete outcome; the unbounded, the
unsatisfiable and the caller-declared obligations are refused with no harness.

Each harness contains exactly one cover, and the contract harness's cover
follows its contract call. Each contract harness carries a `requires` for every
package precondition sharing its anchor and embeds no other obligation's
oracle; the postcondition requested without its sibling precondition is refused
naming that precondition, with no harness. Every identity records solver
`cadical` and the ordered option vector, and no option enables stubbing.
Regeneration is byte-identical, while the changed unwind bound and the changed
subject each produce a different identity digest. Every symbolic argument
carries an inclusive assumption equal to its IR domain, the post-state result
is required to lie in the same domain, and no harness source contains a loop
bound. Each of the four malformed requests is refused whole, with no item
accounted.

## Implementation

`tests/kani_obligations.rs`. The default lane negotiates every item before any
harness is exposed and checks separate harnesses, IR-derived bounds, pins,
assumptions and every refusal. `make kani` runs the ignored lane serially under
a host lock: it asserts the installed backend equals the committed pins (Kani
0.67.0, launcher and driver digests, CBMC 6.8.0, toolchain nightly-2025-11-21,
target x86_64-unknown-linux-gnu), verifies the precondition, postcondition and
invariant harnesses, falsifies a seeded postcondition defect with a concrete
counterexample, reports a contract harness with jointly unsatisfiable requires
as `cover_unsatisfied`, and refuses a drifted driver digest before running.

## Blocked

- Frame harnesses: not constructible from the merged IR. V1 `ClauseKind` has no
  frame kind, FR-014 refuses V2 `state` nodes as `NoFiniteEncoding`, and
  by-value harness arguments cannot express `kani::modifies`. A typed IR frame
  item is required first.
- Every V2 scalar harness: V2 scalar obligations carry only a caller-declared
  operation identity, so they are refused until agent-ix/quire-specification#76
  lands.
- Model and graph bounds: refused as blocked until
  agent-ix/quire-spec-language#120 lands.
