---
id: ADR-003
title: "Kani tractability of generated obligations"
type: ADR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-001
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: relates_to
  - target: ix://agent-ix/quire-spec-language/ADR-011
    type: relates_to
---
# ADR-003: Kani tractability of generated obligations

## Status

Proposed. This record states the problem, the facts measured so far and the decisions it needs. It
designs no solution. Every question is open until the owner rules.

## Context

A generated Kani harness embeds its generated oracle. The oracle calls
`quire_contract_runtime::exact`, so the prover proves the production code through the production
data structures. No requirement states how large an obligation the prover must discharge, what
happens when it cannot, or what the prover is actually proving when the production code is too
large for it.

### Facts measured in this repository

- FR-015-AC-9 forbids any option that enables stubbing, and FR-015 bounds every symbolic argument
  by its IR `bounded_domain` and the unwind bound to `1..=1024`.
- FR-015 refuses a harness whose generated source exceeds the crate's byte ceiling
  (FR-015-AC-13). That ceiling limits source size and says nothing about prover cost.
- FR-017 runs every harness under a caller-declared wall-clock budget. A run that does not
  conclude is killed and classified `inconclusive`/`timed_out` (`src/kani_execution.rs`,
  `KaniInconclusiveReason::TimedOut`). No criterion names that reason (see FR-017's open items),
  and the run sets no memory ceiling.
- The FR-015 scalar harnesses widen each `i64` argument into `rt::Integer` before calling the
  oracle. At the pinned Contract Runtime revision, `Integer` is an inline `i64` with an optional
  boxed `num_bigint::BigInt`, and `TypeEnvironment` holds two `BTreeMap`s.
- A routed scalar harness over one integer operation verifies under the committed pins in the
  ignored `make kani` lane (FR-017-AC-11). This record did not re-run that lane.

### Facts reported elsewhere and not re-measured here

- An open, unmerged Contract Runtime spike pull request (agent-ix/quire-contract-runtime#78)
  reports three harnesses through `PackageDeclarations` → `CheckedPackage` → `Frame::call`. Each
  timed out under a 900 s, 12 GB limit without a verdict. It places the blow-up in a bare
  `PackageDeclarations::check` and drop, before any call.
- Linear IR-241 reports that a harness through `CheckedEquality::evaluate` over two
  `Option<Boolean>` values reached a 16 GB memory guard after about 18 minutes without a result,
  while a `plan_equality`-only probe verified in about 2.5 s.

If those reports hold, then FR-021's function-application oracles and FR-018's equality oracles
cannot be proved by embedding them in a harness as FR-015 does for scalar oracles.

## Decision

Nothing is decided. The owner rules on the questions below.

### Q1: What does a Kani obligation prove?

Options:

1. **The production code.** The harness calls the generated oracle over the production `exact`
   types, as today. An obligation that is intractable there is refused or ends inconclusive.
2. **A bounded shadow plus a refinement proof.** The harness proves the claim over a bounded shadow
   model, for example fixed-width integers and fixed-capacity arrays in place of `BigInt` and
   `BTreeMap`. A separate refinement obligation shows that the production code agrees with the
   shadow on the bounded domain. The shadow must be derived independently of the code under proof
   (QSL ADR-011 §2.3).
3. **The production code with Kani-specific representations behind a `cfg(kani)` switch in Contract
   Runtime.** The prover then proves a build that does not ship.

Recommendation: option 1 for the families that verify today, and option 2 for the families that do
not. Option 3 proves code that is never run. Option 2 is sound only with its refinement obligation,
and the shadow and that obligation need an owner, which may be Contract Runtime rather than CG.

### Q2: What memory and time ceiling applies, and what does exceeding it settle?

Options:

1. **The caller-declared wall-clock budget only**, as today. Memory is unbounded, and a run killed
   by the host is not classified.
2. **A per-run memory ceiling and wall-clock ceiling, both in the harness identity.** A run that
   exceeds either settles as its own `inconclusive` reason, and the ceiling reaches the FR-331
   terminal record as a resource-exhausted cause.
3. **A static tractability refusal at generation**, for families whose harness is known to exceed
   the ceiling, so that no run starts.

Recommendation: option 2, with option 3 added for a family once a measured run exceeds the ceiling.
A ceiling that is not part of the harness identity cannot be reproduced, and an unclassified host
kill is a result nobody can read.

### Q3: Does the stubbing ban stay?

Options:

1. **Keep FR-015-AC-9's ban.** Every stub is an assumption the proof does not discharge.
2. **Admit stubs that are proof-graph edges.** A stub is admitted only as a declared assumption
   edge that makes readiness `conditional` (as FR-003 records it). It is never admitted in a
   counted gate (QSL ADR-011 §2.3).
3. **Admit contract-verified stubs only.** A function stubbed by its own verified Kani contract,
   whose contract is itself a separate obligation.

Recommendation: option 1 for now, and revisit option 3 only when Q1 option 2 does not suffice. A
stubbed proof cannot count toward a claimed-module gate (FR-023), so relaxing the ban buys nothing
for the gates that matter.

## Consequences

Until the owner rules, FR-018 and FR-021 oracles get no Kani harness requirement, and FR-015 stays
scoped to what it generates today. Each ruling becomes criteria in FR-015 and FR-017, and possibly a
Contract Runtime requirement for a shadow model.

## Alternatives Considered

- **Tune harnesses case by case.** The reported blow-up comes before any call, in package
  admission, so no per-harness unwind or bound change reaches it.
