---
id: TC-023
title: "Verify bounded Kani profile corpus parity"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: verifies
---
# TC-023: Verify bounded Kani profile corpus parity

## Description

Verify that the complete selected bounded-Kani corpus preserves one profile
disposition and typed outcome across native execution, generated oracle,
strategy, Kani harness and proof graph for arithmetic/definedness, graph, and
collection cases. Replay of a retained counterexample is not part of what the
current tests back: the corpus retains no counterexample packet, and the replay
clauses below are unbacked until each corpus case carries a real QSL source
replayed through QSL's replay facade.

## Test Procedure

For each semantic family, execute canonical valid boundary cases and malformed,
incomplete, unavailable, refused, inconclusive, exhausted, and counterexample
cases. Generate all codegen artifacts from the same validated Contract IR
selection, run the generated oracle/strategy/Kani cases where the profile is
supported, compare classifications with native execution, and replay every
retained counterexample through the public QSL replay boundary.
Inspect the resolved Cargo graph and generated manifests for a reverse Contract
IR-to-codegen dependency.

## Expected Results

Every supported case has matching typed classification across backends. Every unsupported or adverse case has its original
typed non-Boolean result and no partial artifact or proof claim. Every retained
counterexample either reproduces native false or reports a typed replay
non-success (unbacked). The dependency graph keeps Contract IR below codegen. Every
generated `#[kani::proof]` symbol carries its corpus case's readable name,
family label plus positional counter, the same name its artifact paths carry. A
declared proof-dependency census that is empty or duplicate-identity,
kind/state/path-inconsistent, or names any non-`Required` kind is refused
with a typed `InvalidInput` `kani_corpus_dependency_invalid` result and
leaves no artifact.
