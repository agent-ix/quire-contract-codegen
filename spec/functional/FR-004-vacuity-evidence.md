---
id: FR-004
title: "Produce vacuity and rejection evidence"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-004: Produce vacuity and rejection evidence

## Description

When LLVM/cargo coverage export and runtime campaign counts are supplied, the generator shall combine
oracle evaluation, consequent execution, rejection, discard, and test outcome into a deterministic
per-requirement vacuity report without executing a coverage producer itself.

## Inputs

- The IR-owned bound executable clause population, including canonical identities, typed expressions,
  clause kinds, execution anchors, and dependency/declaration context.
- Generated Rust/source-map bytes with entry probes and the independently derived implication census.
- LLVM coverage JSON export `3.0.1` bytes plus native run results binding source, instrumented binary,
  profiles, command, toolchain, target, optimization profile, and runtime campaign results.
- Shared-assurance candidate/record context for downstream retention.

## Outputs

- A versioned analysis outcome containing trusted observations, per-clause classifications where
  measurable, structured diagnostics, input identities, and an explicit non-success state for
  adverse, unavailable, unsupported, malformed, or inconclusive analysis.

## Behavior

- The analyzer shall consume an LLVM coverage JSON export without implementing instrumentation,
  profile merging, or a coverage engine.
- The analyzer shall derive its expected clause population and implication counts from typed IR,
  require exact source-map/artifact population equality, and reject an empty executable population
  as not computed. Missing map rows shall never turn an implication into an implication-free clause.
- The analyzer shall classify a clause as `vacuous` only when its oracle-evaluation probe was observed,
  its typed expression contains at least one implication, and no expected consequent probe was observed.
- The analyzer shall classify a mapped clause as `unexecuted` when its oracle-evaluation region has
  no positive execution count, regardless of its test outcome or campaign counts.
- The analyzer shall classify a clause as `partially_exercised` when some, but not all, expected
  implication consequents were observed; only observation of every mapped consequent may produce
  `exercised` for an implication-bearing clause.
- A clause whose typed expression contains no implications shall be `exercised` only when its oracle-evaluation
  region is observed; vacuity shall not be inferred when there is no implication.
- These four measured classifications shall be mutually exclusive. Missing/unusable observation
  data shall produce a diagnostic with no classification, distinct from a measured zero count.
- Positive observation requires one count-bearing, non-gap LLVM active span to contain the entire
  mapped entry-token probe. Partial intersection, summary counts, and an unterminated final span
  cannot establish observation. Consequent observation with zero oracle-entry count is inconsistent.
- The analyzer shall preserve native accepted, rejected, failed, and discarded counts and execution
  outcome independently. It shall verify run/requirement/revision bindings; a successful test outcome
  cannot coexist with failed postconditions, and positive oracle observation cannot coexist with
  zero recorded invocations when the run declares the generated campaign as its only execution source.
- Source-map requirement/revision identity shall equal the runtime campaign identity, and duplicate,
  missing, ambiguous, malformed, summary-only, or unsupported-version coverage input shall remain
  a structured non-success analysis outcome. Such an outcome shall retain available input digests
  and diagnostics but shall not claim that invalid or absent inputs were analyzed successfully.
- Coverage filenames shall match source-map artifact paths only after stripping the caller-declared
  source root and applying lexical normalization that rejects parent traversal and backslash aliases.
- Every report shall retain the coverage producer name/version, LLVM export format version, coverage
  export digest, source/map and bound-population digests, native run/binary/profile identities,
  toolchain/target/optimization profile, generated-report schema identity, and exact requirement revision.
- The v0.1 qualified producer profile shall be cargo-llvm-cov 0.9.0 emitting LLVM coverage JSON
  export format 3.0.1.
- Every other coverage producer or export format version shall fail as unsupported.
- The export's `cargo_llvm_cov.version` and `manifest_path` shall agree with the retained native
  producer run. Self-declared JSON metadata alone shall not establish executable provenance.
- The default coverage obligation succeeds only for a nonempty, completely bound population whose
  clauses are all exercised and whose native execution completed successfully. Vacuous, unexecuted,
  and partially exercised results are adverse; unavailable or inconclusive execution cannot pass.
  Artifact serialization success shall never stand in for that coverage result. Exceptions require
  an attributed retained owner decision; this implementation shall not manufacture one.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-004-AC-1 | An evaluated implication whose consequent is unobserved yields a vacuity finding even when every oracle return was true. | Test (TC-006) |
| FR-004-AC-2 | A clause whose oracle-evaluation region was not observed is unexecuted, not vacuous. | Test (TC-006) |
| FR-004-AC-3 | The four measured clause classifications form a total partition; an implication-bearing clause is exercised only when every expected consequent entry probe is observed, and implication-free clauses require observed evaluation. | Test (TC-006) |
| FR-004-AC-4 | Every analysis outcome retains available input/run identities; successful analysis verifies bound population, source/map, native execution, producer, binary, profile, and candidate bindings. | Test (TC-006) |
| FR-004-AC-5 | Malformed, summary-only, identity-mismatched, path-ambiguous, unsupported, or missing observation inputs retain structured non-success outcomes without invented classifications. | Test (TC-006) |
| FR-004-AC-6 | Campaign counts and test outcome remain complete facts independent of coverage classification. | Test (TC-006) |
| FR-004-AC-7 | Removing an expected clause, consequent, evaluation probe, or all clauses prevents successful analysis; the expected census comes from bound typed IR. | Test (TC-006) |
| FR-004-AC-8 | Adverse coverage and non-success native execution cannot discharge the coverage obligation merely because a report serialized successfully. | Test (TC-006) |

## Implementation boundary

The first implementation slice provides source probes and bounded LLVM reading/classification
primitives. It does not yet implement the aggregate analysis outcome, native campaign-run binding,
or shared coverage attestation. Those require the IR-owned executable population from IR #50 and a
native producer/result contract. A primitive returning `exercised` is not a discharged coverage
obligation. FR-004/TC-006 remain planned until the complete bound operation and producer gate exist.

## Dependencies

- **Upstream**: [FR-001](./FR-001-deterministic-oracles.md) and [FR-002](./FR-002-tristate-proptest.md).
