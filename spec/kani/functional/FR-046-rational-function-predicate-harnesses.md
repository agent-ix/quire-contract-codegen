---
id: FR-046
title: "Generate finite rational function predicate harnesses"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-039
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-196
    type: references
---
# FR-046: Generate finite rational function predicate harnesses

## Description

When an admitted request selects a pure Boolean function whose parameters have
finite Rational component domains and whose complete body consists of those
parameters, exact rational addition and rational equality, CG SHALL generate a
bounded proof of that authored Boolean predicate over every value in its declared
domain. This is the IR-258 whole-function slice, distinct from proving an
individual scalar operation valid or comparing two implementations.

The input authority and per-item accounting remain
[FR-039](../../core/functional/FR-039-contract-package-consumption.md)'s one
admitted ContractPackage. This requirement defines the required meaning, not a
new IR schema, unchecked wire constructor or currently available Rust claim API.
An absent owner-lowered whole-function representation is an explicit capability
boundary, not permission to reconstruct package meaning locally.

## Inputs

- The admitted function declaration selection, its original source occurrences,
  complete Boolean body and typed parameter domains supplied by the owner route.
- The requested proof subject, backend and execution bounds governed by
  [FR-015](./FR-015-bounded-kani-obligations.md),
  [FR-025](./FR-025-generated-subject-abi.md) and
  [FR-028](./FR-028-bounded-proof-ceilings.md).

## Outputs

A supported item has its generated predicate oracle, harness, input encoding
correspondence and source-associated proof record. Every refused item has its
named disposition and no Rust harness. Execution results and counterexample
replay remain governed by [FR-017](./FR-017-kani-execution-evidence.md),
[FR-029](./FR-029-run-outcome-terminal-record.md) and
[FR-016](../../replay/functional/FR-016-witness-native-replay.md).

## Behavior

When the function is representable, CG SHALL draw canonical numerator and
positive-denominator components independently for each argument from the exact
inclusive domains in the admitted input. The first encoding represents finite
component endpoints that fit signed `i64`; it does not change the mathematical
Rational domain to a machine arithmetic domain.

When constructing each drawn argument, the harness SHALL call the public owner
`Rational::new(Integer::from(n), Integer::from(d))` and admit that representation
only when the returned Rational's `numerator()` and `denominator()` equal the
original drawn components. This constructor/accessor roundtrip is the canonical
representation constraint: it excludes duplicate encodings, including zero with
denominator other than one, without excluding any declared mathematical value.
It introduces no kernel normalization API or copied GCD implementation.

When evaluating the body, the generated oracle SHALL use the actual owner's
exact operations and Meter with the original expression order, typed outcomes,
charge ordering, stops and laziness. Input-domain restrictions do not constrain
intermediate sums: for example, two permitted inputs equal to nine can sum to
eighteen. Owner arbitrary-precision continuation remains intact.

When asserting the predicate, the harness SHALL require actual completion with
Boolean true over every admitted argument tuple. A false, undefined, refused or
incomplete body is never an assumed-away input or a successful predicate result.
Representation constraints apply before body evaluation and do not depend on its
outcome. Raw backend failure is not by itself evidence of a semantic false
counterexample: native replay retains the actual typed outcome under FR-029.

When the request selects a function, CG SHALL preserve that declaration occurrence
through generation and playback. A successful operation application, a different
function with an equal body, or a scalar child claim cannot replace this predicate.
The owner-issued callable and replay identities remain the owner's contract;
CG does not mint an occurrence key or introduce an alternative replay API.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-046-AC-1 | The genuine source function `rational_commutative(a: Rational[-9,9;1,9], b: Rational[-9,9;1,9]): Boolean` with body `a + b = b + a` selects its whole declaration and complete body through the public QSL admission, strict IR reader/lowering and CG route. Its artifact proves that predicate, not only validity or parity of either addition. Replacing the body by `a + b = a + a` selects the same authored mutation with unchanged parameter domains. | Test |
| FR-046-AC-2 | Every accepted symbolic argument has canonical constructor/accessor roundtrip equality and lies in its original inclusive component domains. Every semantic value in those domains has exactly one accepted representation. Zero is `0/1`; negative, fractional and boundary values remain reachable. For the original domain there are 111 canonical values per argument and 12,321 argument tuples; admitting all 171 raw component pairs, restricting denominators to one, or clipping an endpoint fails correspondence validation under QSpec FR-196-AC-5. | Test |
| FR-046-AC-3 | Generated evaluation preserves the authored full tree and declaration identity, exact owner arithmetic, intermediate values outside the input domains, arbitrary-precision continuation, full typed outcomes and Meter charge/stop order. Native comparison against an independent owner-operation reference checks those observations, including each denied charge prefix and exact/short budgets; it does not calculate expectations by running the generated oracle twice. | Test |
| FR-046-AC-4 | Generation introduces no assumption on completion, truth or an absence of undefined/refused/incomplete results. Controls producing each reachable noncompletion retain that typed outcome and cannot become a successful predicate proof; an outcome-based assume mutation is rejected. A completion assertion failure without a replayed false value is not labelled a confirmed semantic false counterexample. | Test |
| FR-046-AC-5 | The healthy original function receives `Verified` only from an actual successful proof with all required checks and its completion cover satisfied. The wrong-addend mutation produces a substantive failed predicate assertion with actual solver playback of canonical in-domain arguments, and replay of that same authored function completes with Boolean false. A hypothetical witness or cover witness is not substituted for that playback. | Test |
| FR-046-AC-6 | Each emitted predicate harness has exactly one final completion cover after all predicate assertions, including harnesses with representation constraints. Unsatisfied or unreported completion cover, failed unwind, timeout, memory exhaustion or missing verdict cannot yield `Verified`; classifications retain FR-017 and FR-029 distinctions. | Test |
| FR-046-AC-7 | A missing or unbounded Rational parameter domain yields source-associated `requires-bound` naming the selected function, argument and missing domain, with no harness. A finite component endpoint outside this encoding's signed-i64 capacity yields named `unsupported`, retaining the original endpoint and no harness; generation never clips, casts away precision or substitutes a smaller domain. | Test |
| FR-046-AC-8 | An unavailable owner-lowered whole-function representation, absent one-to-one correspondence or unsupported body operation yields named `unsupported` with its source-associated cause and no harness. Every requested item has exactly one disposition under FR-039; a refused item does not remove or alter a supported sibling. Malformed source or owner-refused packages remain upstream refusals, not fabricated CG claims. | Test |
| FR-046-AC-9 | Counterexample decoding validates canonical components, original domains and the selected declaration/application binding through the public owner replay route. Out-of-domain, duplicate-zero, noncanonical, wrong-declaration and stale-identity witnesses are rejected by the responsible boundary; CG neither mints keys nor substitutes scalar application validity for the authored Boolean result. An unavailable replay capability is reported without claiming replay acceptance. | Test |
| FR-046-AC-10 | Run evidence records the selected function and source association, inclusive component domains and correspondence, actual subject and full ordered backend arguments, every unwind/loop bound and option, Kani/Rust/CBMC/solver versions, target, actual dependency cohort and execution limits, raw checks/cover/playback, exit and typed terminal outcome. Replay uses the retained generated artifact without hand editing. No-run, compile refusal, cover failure, resource exhaustion and semantic falsification remain distinguishable; preparation alone earns no proof result. | Test |

## Dependencies

- [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md)
  owns scalar oracle semantics; its native or scalar parity evidence is not this
  whole-function proof.
- [QSpec FR-196](ix://agent-ix/quire-specification/FR-196) owns exact domains,
  total dispositions and the one-to-one finite encoding obligation.
- [FR-024](../../replay/functional/FR-024-counterexample-envelope-intake.md)
  and the QSL-owned function replay route govern witness submission. An owner
  capability still awaiting delivery remains a dependency, not an invented API.
- [TC-060](../matrix/TC-060-rational-function-predicate-harnesses.md)
  defines the planned feature verification. This requirement changes none of
  FR-015's existing obligations or execution ceilings.

## Status

PLANNED (IR-258). This specification supplies no implementation, current-cohort
source-admission, generated-proof, counterexample or end-to-end acceptance credit.
