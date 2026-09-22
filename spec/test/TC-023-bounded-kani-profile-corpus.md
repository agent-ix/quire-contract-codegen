---
id: TC-023
title: "Verify bounded Kani profile corpus parity"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-007
    type: verifies
---
# TC-023: Verify bounded Kani profile corpus parity

## Description

Verify that the complete selected bounded-Kani corpus preserves one profile
disposition and typed outcome across native execution, generated oracle,
strategy, Kani harness, provenance, proof graph, and retained-counterexample
replay for arithmetic/definedness, graph, and collection cases.

## Test Procedure

For each semantic family, execute canonical valid boundary cases and malformed,
incomplete, unavailable, refused, inconclusive, exhausted, and counterexample
cases. Generate all codegen artifacts from the same validated Contract IR
selection, run the generated oracle/strategy/Kani cases where the profile is
supported, compare classifications with native execution, and replay every
retained counterexample through the public Contract IR native runtime boundary.
Inspect the resolved Cargo graph and generated manifests for a reverse Contract
IR-to-codegen dependency.

## Expected Results

Every supported case has matching typed classification and exact retained
identity across backends. Every unsupported or adverse case has its original
typed non-Boolean result and no partial artifact or proof claim. Every retained
counterexample either reproduces native false or reports a typed replay
non-success. The dependency graph keeps Contract IR below codegen. Every
generated `#[kani::proof]` symbol is derived from, and carries, its corpus
case's own identity digest, the same identity its artifact paths carry. A
declared proof-dependency census that is empty or duplicate-identity,
kind/state/path-inconsistent, or names any non-`Required` kind is refused
with a typed `InvalidInput` `kani_corpus_dependency_invalid` result and
leaves no artifact and no identity-registry entry.
