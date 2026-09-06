---
id: REV-014
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
cargo-llvm-cov 0.9.0 and LLVM coverage JSON export 3.0.1; other versions fail as unsupported rather
than inheriting compatibility from a similar-looking tuple layout.

Correction history: the original PR #23 proposed 2.0.1 without a native fixture. Recovery executed
cargo-llvm-cov 0.9.0 with rustc 1.94.1 and observed export 3.0.1. The strict reader first refused that
input, exposing the mismatch; the coordinator approved qualifying the measured 3.0.1 profile instead.
Native tests now explicitly select Rust 1.94.1, separately from library MSRV testing on Rust 1.75.0.

Primary format references:

- [LLVM `llvm-cov export`](https://llvm.org/docs/CommandGuide/llvm-cov.html#export-command)
- [cargo-llvm-cov JSON metadata](https://github.com/taiki-e/cargo-llvm-cov#additional-json-information)

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-2001 | high | Consequent count zero is ambiguous without proof that the owning oracle evaluation ran. Add exactly one `oracle_evaluation` region per clause; zero evaluation means unexecuted, while positive evaluation plus zero consequent means vacuous. | FR-004-AC-1, FR-004-AC-2 |
| FND-2002 | high | File summaries and partial intersection can claim coverage without entry. Require one count-bearing, non-gap active span to contain the complete entry-token probe; retain measured zero separately from unavailable observation. | FR-004-AC-3 |
| FND-2003 | high | Requirement aggregation can hide an unobserved nested or sibling implication. Report `partially_exercised` when only a proper subset of mapped consequents is observed; only complete observation is exercised. | FR-004-AC-3, TC-006 |
| FND-2004 | high | A source map, runtime report, or coverage file from a different requirement/revision can produce a plausible false report. Recompute the source-map digest, require one identity throughout, and reject missing, duplicate, or ambiguous file/region matches. | FR-004-AC-4, FR-004-AC-5 |
| FND-2005 | medium | LLVM segment tuples, export type/version, and full-versus-summary shape are an external schema boundary. Reject malformed tuples, unsupported formats, absent segments, and extra candidates rather than defaulting counts to zero. | FR-004-AC-5 |
| FND-2006 | medium | Absolute and relative filenames can alias after ad hoc suffix matching, and `..` can escape a declared root. Strip one exact caller-declared root, normalize lexically, reject traversal, and require exactly one match. | FR-004-AC-5 |
| FND-2007 | medium | Rejection, discard, failed-postcondition, and test status are evidence about the campaign, not substitutes for consequent execution. Consume the pinned runtime `CampaignReport` rather than a lookalike and preserve its four counters plus test outcome independently. | FR-004-AC-6 |
| FND-2008 | high | Serialization success cannot attest coverage success. Bind native executable/run identities and expected IR population, and deny coverage discharge for adverse or unavailable analysis/execution. JSON metadata is only a cross-check, never producer provenance. | FR-004-AC-4, FR-004-AC-8 |
| FND-2009 | medium | All-or-nothing success must not erase refusal evidence. Retain structured non-success outcomes and available input identities without inventing classifications or a passed coverage attestation. | interface-001 |
| FND-2010 | low | Clauses without implications can be mislabeled vacuous merely because there is no consequent region. Such clauses are exercised when their evaluation region is observed and unexecuted otherwise; vacuity is inapplicable. | FR-004-AC-1, TC-006 |

## Decision

The original PR #23 review requires repair before aggregate implementation. The coordinator has
authorized the bounded source-probe/LLVM-primitive slice and unambiguous spec corrections; REV-015
records its review dispositions and open integration gates. Full analysis waits for IR #50 and
native-run binding. Reuse source-map artifacts and the runtime's campaign accounting, then consume
the shared Quoin/Quire chain without creating a local evidence framework. REV-014 replaces the
colliding REV-008 identifier; it does not claim the unrelated Kani review.

## Verdict

**PARTIAL RECOVERY; INDEPENDENT REVIEW REQUIRED.** TC-006 and FR-004 remain planned until the bound
contract, load-bearing aggregate negative controls and full integrated gates pass on an exact head.
