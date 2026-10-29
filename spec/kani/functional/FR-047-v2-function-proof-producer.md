---
id: FR-047
title: "Produce current-V2 function proof harnesses bound to replay"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-039
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-042
    type: references
---
# FR-047: Produce current-V2 function proof harnesses bound to replay

## Description

When a request selects a function-contract claim occurrence in an admitted
`CheckedPackageV2`, CG SHALL generate an executable Kani harness and its
proving-run identity from that exact selected occurrence and declaration.
When CG generates that harness, CG SHALL retain the original proving context
needed by the existing typed function replay entry. The selected checked package and owner-issued source occurrence
are the authority; a function name, equal body, scalar child, or caller-built
identity is insufficient.

This is the current-V2 producer missing from the function replay consumer.
The existing `KaniObligationHarness` and `KaniObligationIdentity` declarations
do not themselves generate a V2 proof. The current routed generator emits
scalar, clause, frame and collection harnesses, but no V2 function harness.
All acceptance criteria below are PLANNED under IR-1121 until an actual
production entry and its controls exist. This requirement does not claim that
the [FR-021](../../oracle/functional/FR-021-function-application-oracles.md)
concrete owner-path oracle, the
[FR-042](../../oracle/functional/FR-042-generated-checked-invariant-causes.md)
producer guards, or installed-Kani evidence have already passed.

## Inputs

- One [FR-039](../../core/functional/FR-039-contract-package-consumption.md)
  admitted checked V2 package, the exact source and dependency selections from
  which it was admitted, and the caller's original request positions.
- One function claim occurrence, not merely a node id: its selected function
  declaration, claim kind, source occurrence and original request index. The
  declaring owner may be the unit or one explicitly selected dependency.
- The [FR-021](../../oracle/functional/FR-021-function-application-oracles.md)
  admitted callable selection and generated function oracle, with the selected
  declaration's ordered parameters, result type and body. A schedule alone is
  not an executable function oracle.
- The proof backend selection, subject, finite argument domains, unwind,
  options and [FR-028](./FR-028-bounded-proof-ceilings.md) resource ceilings.
- The proving run's exact `ReplayInputs`: unit source, dependency lock,
  accounting and stage limits, declared domains, replay limits and required
  witness decode byte and value-count ceilings. These are retained inputs,
  not defaults inferred after a counterexample is observed.

## Outputs

- For an admitted item, one executable `KaniObligationHarness`, a persisted
  `KaniObligationIdentity` for that harness, and a retained function proof
  context containing the selected checked package, original request index,
  claim occurrence, selected declaration, source/dependency lock, exact
  `ReplayInputs`, proof ceilings and generated artifact identity. The function
  proof identity SHALL carry the owner-issued function and occurrence binding
  that [FR-016](../../replay/functional/FR-016-witness-native-replay.md)
  requires; CG may extend the declared identity types to represent the V2
  selection, but SHALL NOT synthesize a V1 `ClauseRef` or coerce a scalar
  identity to do so.
- For a refused item, one typed refusal naming the failed selection or context
  member, with no executable harness, persisted proof identity or replay
  context. A refused sibling SHALL NOT change an admitted sibling's output.
- The admitted context SHALL supply the exact data the driver needs to build
  `ReplayPackage::new(ReplayInputs, function)` and bind it with
  `with_kani_identity`; the driver SHALL retain this context across the Kani
  run. CG's existing function replay entry remains the replay consumer.

## Behavior

- When a request selects a function claim, CG SHALL join the original request
  index, exact claim occurrence, declaring package and selected function
  declaration before emitting an oracle or a harness. Two claim occurrences on
  one node SHALL remain distinct even if their bodies and names are equal.
- If the claim, occurrence or declaration is absent or ambiguous, then CG SHALL
  return a typed selection refusal before proof execution. CG SHALL NOT
  select by name, first match, request order or body equality.
- If the selected declaration, source owner, dependency lock, admitted package,
  request index or generated oracle disagrees with the selected claim, then CG SHALL
  return a typed mismatch refusal before proof execution, retaining the
  mismatched member and making no replay claim.
- When CG emits a harness, CG SHALL bind its persisted identity to the same
  checked package, function declaration and claim occurrence, obligation kind,
  ordered parameter identities and declared finite domains, generated
  artifact, backend selection and proof ceilings used by the run. Its symbolic
  argument order SHALL be the persisted order that the witness decoder reads;
  no parameter may be omitted or supplied by a caller-only bound.
- When constructing the retained replay context, CG SHALL preserve the
  original unit and dependency source bytes, source identities and selected
  library identities, accounting and stage limits, declared domains and
  caller-selected replay bounds. CG SHALL set each decode ceiling to the
  smaller corresponding caller limit or proving capture bound.
- If `ReplayPackage::new` or `with_kani_identity` rejects the retained source,
  function site, arguments or identity, then CG SHALL return its typed cause
  before decoding playback. A successful package SHALL refer to the same
  function occurrence and proving identity as the emitted harness.
- When the selected function's concrete oracle executes in a harness, CG SHALL
  apply the [FR-021](../../oracle/functional/FR-021-function-application-oracles.md)
  selected callable and retain the typed outcome, charges, result occurrence
  and [FR-042](../../oracle/functional/FR-042-generated-checked-invariant-causes.md)
  checked-invariant causes. A stub value, copied evaluator or test-only
  synthetic proof result SHALL NOT satisfy this producer contract.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-047-AC-1 | An admitted current-V2 function claim selected at one exact occurrence produces one executable Kani harness and persisted identity. The harness calls the generated FR-021 owner-path oracle and the installed backend verifies a healthy subject with a satisfied cover; a separately admitted body-only mutant at the corresponding authored claim position is falsified with its own identity and retained in-domain assertion playback. A schedule-only or test-built identity is not evidence. PLANNED (IR-1121; FR-021 concrete oracle remains open). | Test |
| FR-047-AC-2 | For two distinct claim occurrences on one node, both outputs retain their own occurrence, original request index and selected declaration; their persisted identities and replay contexts do not cross-bind. A function name or equal body cannot merge them. PLANNED (IR-1121). | Test |
| FR-047-AC-3 | Unknown claim or declaration, ambiguous occurrence or declaration, and a claim whose selected form is not a function each yield distinguishable typed selection refusals before oracle emission, Kani execution or playback decoding; an unrelated admitted sibling is unchanged. PLANNED (IR-1121). | Test |
| FR-047-AC-4 | Changing only the admitted package, source owner or source bytes, dependency selection, request index, function declaration, claim occurrence, obligation kind, parameter identity/domain, generated artifact or proof ceiling produces a named typed mismatch or a distinct admitted proof identity; the harness and replay package never bind across those changes. The canonical proof-content identity may bind exact proved content; no tracking hash or source-version pin is introduced. PLANNED (IR-1121). | Test |
| FR-047-AC-5 | The retained context builds `ReplayPackage::new` from the original `ReplayInputs` and binds the emitted `KaniObligationIdentity` with `with_kani_identity`. The same package and identity admit CG's typed `replay_counterexample` for a falsifying Kani transcript; wrong package, source, dependency, request or identity is refused with its typed cause before native replay. PLANNED (IR-1121). | Test |
| FR-047-AC-6 | A transcript one byte above the smaller caller/capture byte ceiling, or one decoded value above the smaller value-count ceiling, is refused before native replay. An in-bound transcript decodes values only against the emitted identity's complete ordered domains; missing, extra and out-of-domain values do not become a reproduced failure. PLANNED (IR-1121). | Test |
| FR-047-AC-7 | The producer uses one admitted checked package and owner-selected callable for generation and proving, and preserves FR-021's `Undefined`, `Refused`, `Incomplete` and checked-invariant distinctions. The FR-042 wrong-completed-result and genuine environment-construction guards remain their own separately planned producer tests; this criterion does not count an invented fault as coverage of either guard. PLANNED (IR-1121; FR-021/FR-042 prerequisites). | Test and Inspection |

## Dependencies

- **Upstream:** [FR-021](../../oracle/functional/FR-021-function-application-oracles.md)
  concrete admitted V2 oracle, [FR-039](../../core/functional/FR-039-contract-package-consumption.md)
  package admission, and [FR-015](./FR-015-bounded-kani-obligations.md)
  generation and [FR-017](./FR-017-kani-execution-evidence.md) capture.
- **Downstream:** [FR-016](../../replay/functional/FR-016-witness-native-replay.md)
  typed replay consumer and quire-driver IR-1106/IR-610 E9 composition.
- **Independent controls:** [FR-042](../../oracle/functional/FR-042-generated-checked-invariant-causes.md)
  retains its own producer-guard tests in TC-057. FR-047 does not close them.
