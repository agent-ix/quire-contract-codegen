---
id: SR-020
title: "Scope and boundary review of numeric and state Kani lowering"
type: SpecReview
analysis: scope-boundary
scope: "FR-003/interface-001 codegen #2 allocation under quire-spec-language#83"
review_set: subset
---

## Summary

The review assigns deterministic Kani source, typed binding graph, refusal, and generation identity
to codegen while preserving IR domain, cargo-kani execution, Quoin sealing, strategy, and native
replay ownership. No sibling branch or downstream runtime responsibility is absorbed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No issues found after the subject, framing, and replay boundaries were made explicit. | - |

## Responsibilities

| Boundary | Owner | Assumed or guaranteed | Contract |
| --- | --- | --- | --- |
| Typed expressions, `DependencyIdentity`, `IntegerType` domains/overflow, obligations and SourceSpan | quire-contract-ir | guaranteed through pinned public Rust API and checked expression | pinned Cargo revision; FR-003 inputs |
| Executable expression analysis and rendered predicates | quire-contract-codegen oracle core | guaranteed by direct in-crate reuse and parity tests | merged codegen #4; FR-001 |
| Subject ABI, model-bound assumptions, Kani source, v2 graph/schemas, diagnostics and generation attestations | quire-contract-codegen Kani adapter | core in scope | FR-003; interface-001; TC-003/005/007/014 |
| Bounded proof execution and concrete-playback text | cargo-kani 0.67.0 | guaranteed only by exact local SUITE-008 observation | pinned executable digest and option vector |
| Attestation sealing and retained-output digest | Quoin | guaranteed through packaged schema/tool contract | ProofAttestationV1 |
| Model-domain proptest strategies | codegen #3 / agent E | external sibling; not consumed by Kani bounds | consume only after merge; do not edit its branch/files |
| Counterexample input construction and `runtime::execute` verdict comparison | quire-spec-language IT-010 | external downstream contract test | SL #84 |
| Customer subject implementation, signature, link and unmodeled effects | consuming generated crate | externally observed, not inferred by codegen | Rust compilation plus cargo-kani result |

Definedness discharge, arithmetic/negation, indirect/object/graph reads, strategy generation changes,
runtime execution, hosted CI, release approval, v1 schema replacement, and edits to SL
`resources/native-v1/` are out of scope. Each semantic non-goal refuses or remains an explicitly
external observation; none is approximated.
