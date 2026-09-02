---
id: REV-008
title: "LLVM vacuity and rejection analysis preimplementation review"
type: SpecReview
analysis: gap-analysis
scope: "issue #5 per-requirement vacuity and rejection evidence"
review_set: subset
---

# LLVM vacuity and rejection analysis preimplementation review

## Summary

FR-004 cannot distinguish vacuity from code that never ran using the current source map alone. It
marks the whole generated file as a clause and marks implication consequents, but it does not identify
the executable region proving that the oracle was entered. The proposed slice adds that region and
consumes LLVM JSON segments plus the accepted runtime campaign report. LLVM/cargo-llvm-cov remains
the coverage producer; codegen only validates, maps, classifies, and packages the supplied facts.

The external format boundary follows LLVM's documented `llvm-cov export` contract, which exports
regions, functions, branches, expansions, and summaries in JSON, and cargo-llvm-cov's documented
root metadata, which separately identifies its own version and manifest path. Tool version and LLVM
export-format version are therefore retained as different fields. The initial qualified pair is
cargo-llvm-cov 0.9.0 and LLVM coverage JSON export 2.0.1; other versions fail as unsupported rather
than inheriting compatibility from a similar-looking tuple layout.

Primary format references:

- [LLVM `llvm-cov export`](https://llvm.org/docs/CommandGuide/llvm-cov.html#export-command)
- [cargo-llvm-cov JSON metadata](https://github.com/taiki-e/cargo-llvm-cov#additional-json-information)

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-2001 | high | Consequent count zero is ambiguous without proof that the owning oracle evaluation ran. Add exactly one `oracle_evaluation` region per clause; zero evaluation means unexecuted, while positive evaluation plus zero consequent means vacuous. | FR-004-AC-1, FR-004-AC-2 |
| FND-2002 | high | File summaries and line percentages can claim coverage without the mapped consequent. Reconstruct active spans from ordered LLVM segment transitions and use only positive-count, count-bearing, non-gap spans intersecting the exact mapped region. | FR-004-AC-3 |
| FND-2003 | high | Requirement aggregation can hide an unobserved nested or sibling implication. Report `partially_exercised` when only a proper subset of mapped consequents is observed; only complete observation is exercised. | FR-004-AC-3, TC-006 |
| FND-2004 | high | A source map, runtime report, or coverage file from a different requirement/revision can produce a plausible false report. Recompute the source-map digest, require one identity throughout, and reject missing, duplicate, or ambiguous file/region matches. | FR-004-AC-4, FR-004-AC-5 |
| FND-2005 | medium | LLVM segment tuples, export type/version, and full-versus-summary shape are an external schema boundary. Reject malformed tuples, unsupported formats, absent segments, and extra candidates rather than defaulting counts to zero. | FR-004-AC-5 |
| FND-2006 | medium | Absolute and relative filenames can alias after ad hoc suffix matching, and `..` can escape a declared root. Strip one exact caller-declared root, normalize lexically, reject traversal, and require exactly one match. | FR-004-AC-5 |
| FND-2007 | medium | Rejection, discard, failed-postcondition, and test status are evidence about the campaign, not substitutes for consequent execution. Consume the pinned runtime `CampaignReport` rather than a lookalike and preserve its four counters plus test outcome independently. | FR-004-AC-6 |
| FND-2008 | medium | Provenance can conflate the producer executable, LLVM's JSON format, and the analyzer's successful packaging result. Retain producer name/version, export format version, raw export digest, source-map digest, schema identity, and candidate binding in the report; the packaged ProofAttestationV1 result describes successful report generation and never replaces the report's coverage classification or test outcome. | FR-004-AC-4 |
| FND-2009 | medium | Returning a report before every file, region, identity, counter, and attestation check completes creates a partial false artifact. Validate the full request first and return an all-or-nothing bundle or structured diagnostic set. | interface-001 |
| FND-2010 | low | Clauses without implications can be mislabeled vacuous merely because there is no consequent region. Such clauses are exercised when their evaluation region is observed and unexecuted otherwise; vacuity is inapplicable. | FR-004-AC-1, TC-006 |

## Decision

Proceed to independent review of the specification and plan delta. Implementation may begin only
after that review accepts the classification lattice, segment/path rules, identity boundary, and
all-or-nothing result. The implementation shall reuse the existing source-map artifact,
quire-contract-runtime campaign accounting, packaged ProofAttestationV1, and Quoin/Quire assurance
chain. It shall not add a local coverage engine, evidence store, attestation schema, or verifier.

## Verdict

**READY FOR INDEPENDENT SPECIFICATION REVIEW.** This is not implementation approval and closes no
FR-004 matrix row. TC-006 and FR-004 remain planned until the reviewed contract is implemented and
the tool-produced fixtures, negative mutations, and full local gate pass on an exact head.
