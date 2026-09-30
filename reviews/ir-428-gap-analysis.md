---
id: "SR-612"
title: "IR-428 gap analysis: bump quire-contract-ir to main"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@140868a8033f5c768fccca02f5c7a9f02a7001dd; Cargo.toml, Cargo.lock, deny.toml"
relationships: []
---

# SR-612: IR-428 gap analysis

## Summary

Ticket: IR-428. PR: agent-ix/quire-contract-codegen#195. I ran this review at a scale that fits
a dependency bump with no Rust source change.
Plan completion: not assessed.

## Method

I checked whether any requirement, test-matrix row or tracked test depends on the dependency
graph that changed. I grepped `spec/` for the dependency pins, deny/licensing clauses,
`arbitrary_precision` and quire-verification-contracts. The only licensing requirement is
NFR-002, which covers the SPDX headers of emitted files and not dependency licenses, and
interface-001:293, which states the crate license. Neither is affected. No code was added, so
there is no untraced code. Golden-output and generation tests pass on the head, and the only 8
failures come from missing host Kani/LLVM tooling.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. The bump introduces no spec, matrix or trace gap. The dev-dependency realignment keeps
tests selecting law definitions from the same operation catalog that IR a7e019a6 validates
against, which is what the Cargo.toml comment says it should do.
