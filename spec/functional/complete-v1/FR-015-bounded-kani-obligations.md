---
id: FR-015
title: "Generate separate bounded Kani obligations for complete-V1 oracles"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-196
    type: references
  - target: ix://agent-ix/quire-specification/TC-218
    type: references
---
# FR-015: Generate separate bounded Kani obligations for complete-V1 oracles

## Description

When a caller selects Kani generation for complete-V1 contracts whose scalar
oracles FR-014 generated, the code generator shall emit one bounded Kani
obligation per precondition, postcondition, invariant and frame condition,
each bounded by its model-domain bounds. It is the Kani
backend's one generator
([ADR-001](../../decisions/ADR-001-overlapping-generators-and-input-models.md)),
and every item it lowers is a claim over an admitted `quire.checked-package/v2`
package.

## Inputs

- The FR-014 oracle crate and claim map for the contract's expressions.
- Contract IR's lowered claims and bounds for each obligation, read from an
  admitted `CheckedPackageV2`. Bounds come only from the lowered IR
  `bounded_domain` nodes, never from a caller descriptor, and are tightened
  only as [FR-028](./FR-028-bounded-proof-ceilings.md) records.
- An optional declared proof-dependency census per obligation.
- A complete request. A postcondition or invariant is proved under the
  preconditions of its own anchor operation, so the request must name every
  package precondition sharing that anchor as an item of the same request. The
  caller does not choose a solver.
- The loop unwind bound, which is a request-level value in `1..=1024`.

## Outputs

- One Kani harness per obligation kind and claim, with its bounds, solver,
  option vector and unwind bound recorded in its identity.
- Exactly one non-vacuity cover per harness, which is what distinguishes a
  proof from a harness whose assumptions are jointly unsatisfiable.
- For a postcondition or invariant, the assumed preconditions recorded in the
  harness identity's embedded oracles, so a reader can see the claim actually
  proved rather than the claim the clause states alone.
- A typed refusal for each obligation that has no finite encoding.

## Behavior

- When generating proofs, the generator shall emit precondition,
  postcondition, invariant and frame obligations as separate harnesses.
- When an obligation has a finite model domain, the generator shall bound
  every symbolic input by that domain without assuming away refused,
  undefined or incomplete outcomes.
- If an obligation's domain is unbounded or its family lacks a finite
  encoding, then the generator shall refuse it with a typed reason and emit no
  harness.
- If an obligation's bounds are unsatisfiable, so that the harness would hold
  vacuously, then the generator shall refuse it with a typed reason and emit no
  harness.
- If an obligation depends on an FR-014 oracle whose operation identity is
  `caller_declared`, then the generator shall refuse it with a typed reason
  naming that blocked item and emit no harness.
- The generator shall end every generated harness with exactly one
  non-vacuity cover: a precondition harness covers that the precondition holds
  within the IR bounds, and a contract harness covers, at a point after the
  contract call, that the requires and the IR bounds are jointly satisfiable. A
  backend run that reports success without satisfying that cover has proved
  nothing, and the cover is the only thing that says so.
- When lowering a postcondition or invariant, the generator shall assume every
  package precondition sharing the obligation's anchor operation, emitting each
  as a `requires` on the generated contract and recording it in the harness
  identity's embedded oracles. The claim proved is therefore the obligation
  under those preconditions, and never the obligation alone.
- If a precondition sharing that anchor is not a supported item of the same
  request, then the generator shall refuse the obligation with a typed reason
  naming that precondition and emit no harness, rather than proving the
  obligation under fewer assumptions than the contract states.
- The generator shall lower every obligation against the `cadical` solver and
  without stubbing, and shall record the solver and the complete ordered option
  vector in the harness identity. No option enabling stubbing is emitted,
  because a stub is an assumption this requirement does not admit.
- The generator shall bound every symbolic argument by an inclusive assumption
  taken from its IR `bounded_domain`, and shall require a bounded-integer
  post-state result to remain inside the same domain before the clause can
  hold.
- The generated source shall carry no `#[kani::unwind]`; the unwind bound
  reaches the backend only as an explicit option in the recorded vector, so the
  bound a harness was proved under is read from its identity rather than from
  its text.
- Regeneration from equal inputs shall be byte-identical, and the unwind bound
  and the customer subject shall each change the harness identity.
- If the request names no items, more than 256 items, or an unwind bound
  outside `1..=1024`, then the generator shall refuse the whole request and
  account no item.
- If an otherwise-supported obligation's generated harness source would exceed
  the crate's bounded-resource ceiling, then the generator shall refuse it with
  a distinct resource-limit ground naming the generated size and emit no
  harness; if it fits that ceiling but does not parse as Rust, then the
  generator shall refuse it with a distinct syntax ground naming the parse
  error and emit no harness; neither is reported as the internal-invariant
  fallback used for an otherwise-successful render whose harness could not be
  assembled.
- If a V2 scalar-claim item's graph node carries a tag/form pair the generator
  does not model as a contract role, then the generator shall refuse it with a
  typed reason naming that tag and form and emit no harness, rather than
  accounting it as supported with no contract role recorded; a claim naming a
  node id absent from the graph entirely is likewise refused, with no harness
  emitted.
- If a V2 scalar-claim item's claim is a refusal that FR-022 recorded because the
  node carries no derivable FR-014 descriptor (`NoDerivableClaim`), then the
  generator shall account it `unsupported` with the `no_derivable_claim`
  reason naming the node and the derivation refusal, emit no harness, and not
  reject the request.
- Where an operand of a V2 scalar claim is a literal, inline or a reference to a
  `value` node whose body is a literal, the generator shall constrain that
  symbolic argument to exactly the literal's own value.
- If a literal operand's value does not fit `i64`, then the generator shall
  refuse the item as `domain_not_representable_in_i64` naming the value as both
  endpoints and emit no harness.
- If the exact results of the operation over every operand range lie outside
  the result range, then the generator shall refuse the item as
  `result_bound_unreachable` naming the result range and the reachable result
  range, and emit no harness, since no assumed input could meet its
  non-vacuity cover.
- When a transition obligation reads state, the generator shall emit a harness
  that asserts the obligation over the pre-state and the post-state the
  subject ABI binds ([FR-025](./FR-025-generated-subject-abi.md) owns how state
  reaches the subject).
- When a claim is a Boolean connective or a bounded-integer comparison, the
  generator shall embed its FR-014 oracle byte-identical to the oracle crate's
  function, so that the harness decides the same verdict the oracle decides.
- If a precondition reads a post-state value, then the generator shall refuse
  the obligation with a typed reason naming the node and emit no harness.
- When a request declares a proof-dependency census for an obligation, the
  generator shall admit only `Required` dependencies, fold the census into the
  harness identity, and record generation-time readiness `ready` when every
  dependency passed and `incomplete` while any is missing or failed, with proof
  execution recorded `not_run`.
- If a declared census has an empty or duplicate dependency identity, an
  inconsistent kind, state and path combination, or a kind other than
  `Required`, then the generator shall refuse the obligation as a typed invalid
  input and emit no harness.
- The generator shall give every requested item exactly one
  `ObligationDisposition`: `supported`, `requires-bound`, `unsupported` or
  `invalid-request`, the FR-331 disposition set.
- If an item's disposition is `requires-bound`, `unsupported` or
  `invalid-request`, then the generator shall keep its source identity and typed
  reason and emit no harness and no assumption for it.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-015-AC-1 | Pre, post, invariant and frame obligations of one contract produce separate harnesses with distinct identities. | Test (TC-025) |
| FR-015-AC-2 | Every harness identity records its model-domain bounds, taken only from IR `bounded_domain` nodes, and its solver and options and unwind bound. | Test (TC-025) |
| FR-015-AC-3 | An unbounded or non-finite obligation is refused with a typed reason and no harness. | Test (TC-025) |
| FR-015-AC-4 | No harness assumption excludes an undefined, refused or incomplete runtime outcome. | Test (TC-025) |
| FR-015-AC-5 | An obligation whose bounds are unsatisfiable is refused with a typed reason and no harness. | Test (TC-025) |
| FR-015-AC-6 | An obligation over an oracle whose operation identity is `caller_declared` is refused with a typed reason and no harness. | Test (TC-025) |
| FR-015-AC-7 | Every generated harness contains exactly one non-vacuity cover; a precondition harness covers that the precondition holds within the IR bounds, and a contract harness's cover stands after the contract call, so a run that satisfies every check without satisfying the cover is not a proof. | Test (TC-025) |
| FR-015-AC-8 | A postcondition or invariant harness emits every package precondition sharing its anchor operation as a `requires` on its generated contract and records each in its identity's embedded oracles; it embeds no other obligation's oracle; and an obligation whose sibling precondition is not a supported item of the same request is refused with a typed reason naming that precondition and no harness. | Test (TC-025) |
| FR-015-AC-9 | Every harness identity records solver `cadical` and the complete ordered option vector — function contracts, concrete playback, the exact fully qualified harness, `--exact`, the explicit unwind, the explicit solver, `--output-format regular`, and `--concrete-playback print` — and no option enabling stubbing is emitted. | Test (TC-025) |
| FR-015-AC-10 | Regeneration from equal inputs is byte-identical, and changing the unwind bound or the customer subject changes the harness identity. | Test (TC-025) |
| FR-015-AC-11 | Every symbolic argument carries an inclusive assumption equal to its IR `bounded_domain` (a literal operand's, to its own value: FR-015-AC-16), and a bounded-integer post-state result is required to lie in the same domain; no generated source carries a `#[kani::unwind]`. | Test (TC-025) |
| FR-015-AC-12 | A request naming no items, more than 256 items, an unparsable subject path, or an unwind bound outside `1..=1024` is refused whole, with no item accounted and no harness exposed. | Test (TC-025) |
| FR-015-AC-13 | An otherwise-supported obligation whose generated harness source exceeds the bounded-resource ceiling is refused with a distinct resource-limit reason naming the generated size, and one that fits the ceiling but fails to parse as Rust is refused with a distinct syntax reason naming the parse error; neither is reported as the internal-invariant render-assembly fallback. | Test (TC-025) |
| FR-015-AC-14 | A scalar-claim item whose graph node carries a tag/form pair this generator does not model as a contract role is refused with a typed reason naming that tag and form, and no harness is emitted; a claim naming a node id absent from the graph entirely is refused rather than accounted as supported with no contract role recorded. | Test (TC-025) |
| FR-015-AC-15 | A scalar-claim item whose claim is a `NoDerivableClaim` refusal yields `unsupported` with the `no_derivable_claim` reason naming the node and the derivation refusal, emits no harness, and does not reject the group. | Test (TC-033) |
| FR-015-AC-16 | A scalar harness constrains a literal operand, inline or a reference to a `value` node whose body is a literal, to exactly its own value: `x + 1` over `x: Int[0, 9]` into `Int[0, 10]` has arguments `[0, 9]` and `[1, 1]`. | Test (TC-033) |
| FR-015-AC-17 | A scalar claim whose literal operand does not fit `i64` (`x + 10^23`) is `unsupported` as `domain_not_representable_in_i64` naming the literal as both endpoints, with no harness. | Test (TC-033) |
| FR-015-AC-18 | A scalar claim whose operation over its operand ranges yields no result inside the result range (`x + 100` over `x: Int[0, 9]` into `Int[0, 10]`, reachable `[100, 109]`) is `unsupported` as `result_bound_unreachable` naming both ranges, with no harness. | Test (TC-033) |
| FR-015-AC-19 | A zero-input transition over one bounded-integer state value verifies for an identity subject and is falsified with a concrete counterexample for a subject that changes the value. | Test (TC-025) |
| FR-015-AC-20 | A harness for a Boolean-connective or bounded-integer comparison claim (`and`, `or`, `not`, `implies`, `eq`, `ne` and the six integer comparisons) embeds its FR-014 oracle byte-identical to the oracle crate's function. | Test (TC-025) |
| FR-015-AC-21 | A precondition that reads a post-state value is refused with a typed reason naming the node, and no harness is emitted. | Test (TC-025) |
| FR-015-AC-22 | A declared proof-dependency census with an empty or duplicate identity, an inconsistent kind, state and path combination, or a non-`Required` kind is refused as a typed invalid input with no harness. | Test (TC-025) |
| FR-015-AC-23 | Every requested item receives exactly one `ObligationDisposition` (`supported`, `requires-bound`, `unsupported` or `invalid-request`), and an item that is not `supported` keeps its source identity and typed reason with no harness and no assumption emitted for it. | Test (TC-025) |
| FR-015-AC-24 | A falsifying in-domain assignment of a plain bounded-integer comparison claim is reported through Kani's concrete playback. | Test (TC-025) |
| FR-015-AC-25 | A valid declared census is folded into the harness identity, readiness is `ready` only when every dependency passed and `incomplete` while any is missing or failed, and proof execution is recorded `not_run`. | Test (TC-025) |
| FR-015-AC-26 | One `postcondition` `state_clause` of an admitted package, whose condition is one integer comparison of the pre and post reads of one field through its first parameter `self`, yields a separate operation-contract harness and frame-effect harness for the operation its anchor names. Each has exactly one non-vacuity cover, and its identity is scoped to the operation, anchor, frame and framed object; the two harnesses of two clauses of one operation are written to different paths. | Test (TC-025) |
| FR-015-AC-27 | The operation-contract harness assumes each state field in the integer range that the framed object's body member of the same name declares, and asserts the clause's comparison between the pre-state snapshot and the state after the subject runs; a clause whose field has no such range is refused. | Test (TC-025) |
| FR-015-AC-28 | The frame-effect harness takes the granted fields from the frame node's `modifies` entries and asserts every other state field the caller names unchanged. | Test (TC-025) |
| FR-015-AC-29 | A state clause that is not a postcondition, whose condition is not one comparison of the pre and post reads of one field through `self` (a literal operand, a negation, two fields, two reads of one side, or a read through another parameter), whose frame creates, deletes or grants a relationship or a foreign field, whose frame grants every state field, whose field the caller's state lacks, or whose clause field declares no integer range, is refused with a typed reason and no harness; a malformed request is refused first. | Test (TC-025) |
| FR-015-AC-30 | With the installed backend, the operation contract verifies for a healthy subject and is falsified, naming the postcondition, for a subject mutated to debit. | Test (TC-025) |
| FR-015-AC-31 | With the installed backend, a frame-allowed effect verifies, a frame-forbidden effect is falsified naming the forbidden field, and regenerating the frame from a package whose `modifies` is mutated to grant nothing falsifies the allowed subject. | Test (TC-025) |
| FR-015-AC-32 | The forbidden frame counterexample, executed natively, reproduces as a frame violation of that field through QSL's `replay_frame`, and the allowed run replays as a respected frame. | Test (TC-025) |

## Dependencies

- **Upstream**: [FR-014](./FR-014-exact-scalar-oracles.md),
  [FR-025](./FR-025-generated-subject-abi.md), [FR-028](./FR-028-bounded-proof-ceilings.md).
- **Downstream**: [TC-025](../../test/complete-v1/TC-025-bounded-kani-obligations.md),
  [FR-016](./FR-016-witness-native-replay.md),
  [FR-022](./FR-022-routed-generation.md), whose Kani generation arm calls this generator.
