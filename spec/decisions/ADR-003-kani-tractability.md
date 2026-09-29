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
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: relates_to
  - target: ix://agent-ix/quire-spec-language/ADR-011
    type: relates_to
---
# ADR-003: Kani tractability of generated obligations

## Status

Accepted. The owner ruled on Q1, Q2 and Q3 as the Decision states.

## Context

A generated Kani harness embeds its generated oracle. The oracle calls
`quire_contract_runtime::exact`, so the prover proves the production code through the production
data structures. No requirement stated how large an obligation the prover must discharge, what
happens when it cannot, or what the prover is proving when the production code is too large for it.

### Facts measured in this repository

- FR-015-AC-9 forbids any option that enables stubbing, and FR-015 bounds every symbolic argument
  by its IR `bounded_domain` and the unwind bound to `1..=1024`.
- FR-015 refuses a harness whose generated source exceeds the crate's byte ceiling
  (FR-015-AC-13). That ceiling limits source size and says nothing about prover cost.
- FR-017 runs every harness under a caller-declared wall-clock budget. A run that does not
  conclude is killed and classified `inconclusive`/`timed_out` (`src/kani_execution.rs`,
  `KaniInconclusiveReason::TimedOut`). The run sets no memory ceiling.
- The FR-015 scalar harnesses widen each `i64` argument into `rt::Integer` before calling the
  oracle. In Contract Runtime, `Integer` is an inline `i64` with an optional
  boxed `num_bigint::BigInt`, and `TypeEnvironment` holds two `BTreeMap`s.
- A routed scalar harness over one integer operation verifies in the
  ignored `make kani` lane (FR-017-AC-11).

### Facts reported elsewhere and not re-measured here

- An open Contract Runtime spike (agent-ix/quire-contract-runtime#78) reports three harnesses
  through `PackageDeclarations` → `CheckedPackage` → `Frame::call`. Each timed out under a 900 s,
  12 GB limit without a verdict. It places the blow-up in a bare `PackageDeclarations::check` and
  drop, before any call.
- A harness through `CheckedEquality::evaluate` over two `Option<Boolean>` values is reported to
  have reached a 16 GB memory guard after about 18 minutes without a result, while a
  `plan_equality`-only probe verified in about 2.5 s.

If those reports hold, FR-021's function-application oracles and FR-018's equality oracles cannot
be proved by embedding them in a harness as FR-015 does for scalar oracles.

## Decision

The goal is the widest proof coverage that can be obtained, with every proof bounded so that it
can finish.

### Q1: a family proves production code where it verifies, and a bounded shadow elsewhere

Proof coverage is maximised per family:

- A family whose harness over the production `exact` code verifies within its ceilings proves the
  production code, as FR-015's scalar harnesses do.
- A family whose production harness cannot verify within its ceilings proves its claim over a
  bounded shadow model, together with a separate refinement obligation showing that the production
  code agrees with the shadow on the bounded domain. The shadow is derived independently of the
  code under proof (QSL ADR-011 §2.3). Contract Runtime co-owns the shadow and the refinement
  obligation for its kernel types.

A shadow proof is never reported without its refinement obligation.

### Q2: ceilings in the harness identity, with their own inconclusive reasons

- Every harness is bounded. Its symbolic arguments carry inclusive bounds from their declared model
  domains, and an item with no finite bound gets no harness, which settles `requires-bound`
  (FR-015, FR-025).
- Every harness identity records a memory ceiling and a wall-clock ceiling. A run is held to both.
- A run that exceeds its wall-clock ceiling settles `inconclusive` with its own timed-out reason. A
  run that exceeds its memory ceiling settles `inconclusive` with its own memory-exhausted reason.
  Neither is ever reported as verified or falsified.
- The evidence of every run records the bounds and the ceilings it ran under.
- Coverage is reportable per family. Every harness identity, refusal and execution evidence names
  its family, so a gap stays visible.
- A bound may be tightened inside the declared model domain to let a proof finish only when the
  harness identity and the evidence record both the declared domain and the tightened bound. A
  harness over a tightened bound covers that bound and never reads as a proof over the declared
  domain.

### Q3: the stubbing ban stays

FR-015-AC-9's ban stands: no option enabling stubbing is emitted. Every stub is an assumption the
proof does not discharge.

[FR-028](../functional/complete-v1/FR-028-bounded-proof-ceilings.md) states Q1 and Q2.

## Consequences

- FR-028 states the ceilings, their inconclusive reasons, the tightened-bound record, the
  per-family naming and the proof subject. It stays planned until code and a test back
  it.
- FR-017 runs under the ceilings the harness identity records, rather than under a budget the
  caller declares at run time.
- The shadow model and its refinement obligation for the runtime kernel types need a Contract
  Runtime requirement. FR-018 and FR-021 oracles get a Kani harness through FR-015 only over such a
  shadow.

## Alternatives Considered

- **Production code only.** Rejected, because an intractable family would then have no proof
  coverage at all.
- **Production code with Kani-specific representations behind a `cfg(kani)` switch in Contract
  Runtime.** Rejected, because the prover would then prove a build that does not ship.
- **The caller-declared wall-clock budget only.** Rejected, because memory is then unbounded and a
  run killed by the host is not classified, and a ceiling outside the harness identity cannot be
  reproduced.
- **Stubs admitted as proof-graph assumption edges, or only as contract-verified stubs.** Rejected,
  because every stub is an assumption the proof does not discharge.
- **Tune harnesses case by case.** Rejected, because the reported blow-up comes before any call, in
  package admission, so no per-harness unwind or bound change reaches it.
