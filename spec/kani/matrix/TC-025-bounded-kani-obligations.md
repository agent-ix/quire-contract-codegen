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

State clauses: generate the operation-contract and frame-effect harnesses of one
`postcondition` clause over one integer field; read each identity's scope, field
ranges, granted and forbidden fields and cover count; and submit a clause that is
a precondition, one comparing two reads of the same side, one comparing two
fields, a frame that creates or deletes an object or grants a relationship or a foreign field, a condition that is a negation, compares to a literal or reads through a second parameter, a clause field with no integer range,
a state without the clause's field, a frame granting every field, and an
unparsable path and an out-of-range unwind bound. Run the installed Kani backend
over a healthy subject, a subject mutated to debit, a subject writing a granted
field, a subject also writing an ungranted field, and the allowed subject against
a frame regenerated from a package whose `modifies` is emptied; then execute the
forbidden counterexample natively and replay it, and the allowed run, through
QSL's `replay_frame`.

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

`tests/it/kani_obligations.rs`, and for state clauses `tests/it/kani_obligations_state_frame.rs`,
whose module name `make kani`'s `kani_obligations` filter selects. The default lane negotiates every item before
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
8. Generate the operation-contract and frame-effect harnesses of one `postcondition` state clause
   over one integer field. Each carries one cover, an identity scoped to the operation, anchor,
   frame and object, and the IR range of each state field (FR-015-AC-26 to FR-015-AC-28). Each clause or frame
   shape outside that encoding is refused by name with no harness (FR-015-AC-29). The installed
   backend verifies the healthy subject, falsifies the debiting subject, verifies the granted write,
   falsifies the ungranted write naming its field, falsifies the allowed subject against a frame
   regenerated with nothing granted, and QSL's `replay_frame` reproduces the forbidden write and
   finds the allowed run inside its frame (FR-015-AC-30 to FR-015-AC-32). The default suite builds that
   replay's request and envelope through `qsl_replay::call_site` and settles a forbidden and a
   granted write without Kani, and refuses an operation with no frame, declared or not (FR-015-AC-33 to FR-015-AC-36).
9. Generate the scalar harness of each integer operation (add, subtract, multiply, negate). Each
   asserts the oracle's outcome against the operation evaluated natively in `i128`, with a `Completed`
   arm, a `Refused` arm and a failing catch-all. The installed backend verifies the healthy `x + 1`
   harness and falsifies it with the oracle's `Add` replaced by `Subtract` (FR-015-AC-37).

## V2 clause claims and the V2 census (planned, IR-489)

10. Request one V2 clause claim each over a `precondition`, a `postcondition` and an
    `invariant` node of an admitted `CheckedPackageV2`. Each yields one harness of its own
    kind with one drawn value per drawn input (the pre-state of every field the body reads, bare or `pre(...)`, and the
    operation parameters, bounded by their declared domains; the postcondition's result
    and post-state come from the subject call and their domain is asserted), and none takes a
    bound from a caller (FR-015-AC-38, FR-015-AC-39). The invariant harness asserts the
    clause with no subject call (FR-015-AC-49).
11. Generate a clause of connectives and comparisons and compare the embedded oracle with
    the FR-014 crate's function byte for byte (needs FR-014-AC-38); generate the
    arithmetic postcondition `amount < 1000` implies `amount + 1 <= 1000`, check the
    native `i128` assertion of each arithmetic subterm, and run the installed backend over
    the unmutated clause and over one whose `+` returns `left + right + 1` (expected
    playback `amount_current = 999`), and one whose `+` returns `left`, which is
    falsified on the native assertion (FR-015-AC-40,
    FR-015-AC-41). The package comes from QSL's facade on source, never a copied fixture
    (AD-004 step 4a); if the facade cannot build it, the real-Kani control is the
    quire-integration exemplar of AD-004 L-5, a test in a repository above both.
12. Request a clause with an unsupported operator (division, modulo, absolute value), a
    node that is not a modelled clause role and an absent node id beside a supported item.
    The operator is `unsupported`, the other two refused as FR-015-AC-14 states, each
    keeps its reason and gets no harness (FR-015-AC-42). A clause value outside the three
    kinds and a non-Boolean body cannot reach this arm on an admitted package (Contract IR
    FR-040-AC-8 and FR-040-AC-10 refuse them at admission), so that guard is unreachable
    and not tested here. Request a postcondition without its sibling precondition
    (FR-015-AC-43).
13. Declare the four invalid censuses of FR-015-AC-22 and the three valid censuses of
    FR-015-AC-25 on V2 clause claims, with the dependency list reordered. The invalid ones
    are refused; the valid ones fold into the identity and settle `ready`, `incomplete` and
    `incomplete`, and every harness records execution `not_run` (FR-015-AC-44,
    FR-015-AC-45).
14. Count the covers and locate each relative to the assumptions and the subject call; run
    a clause with jointly unsatisfiable assumptions; regenerate from equal inputs; vary the
    unwind bound, the subject, a parameter's domain and the source span: the harness
    identity record changes with the first two, the obligation identity with the domain
    and not with the span, unwind bound or subject (FR-015-AC-46 to FR-015-AC-48).
15. Feed `negotiate` a scalar refusal of each of `LoweringByteLimitExceeded` and
    `LoweringLimitUnrecognised` and read `OracleRefused` carrying it unchanged, field for field
    (FR-015-AC-50, IR-547).

## Blocked

- Frame harnesses in the clause negotiation: FR-025 accounts every frame obligation
  `unsupported` there until QSpec decides how a frame node lowers into a Kani form (ADR-004). The
  frame of a `postcondition` state clause over one integer field is instead generated with that
  clause by `generate_state_frame_obligations` (FR-015-AC-26 to FR-015-AC-32).
- V2 scalar harnesses outside `IntegerArithmetic`: an IR-confirmed claim over
  one of the four `quire.op.integer.{add,sub,mul,negate}` identities reaches a
  real harness unless a ground independent of the operation (an
  i64-unrepresentable endpoint, the source ceiling) displaces it. Every other
  confirmed family is refused as `OperationNotRendered`, an unbuilt renderer in
  this generator, not an upstream block, and so is a claim of a rendered family
  from a claim map this generator did not produce (NFR-005-AC-2). A claim this generator lowered but
  whose operation it did not confirm against the node's own catalogued
  identity, mode or law definition is refused as `CallerDeclaredOperation`.
- Model and graph bounds: refused as blocked on `agent-ix/quire-spec-language#120`.
