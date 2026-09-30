---
id: REV-016
title: "Public bound-package oracle consumer preimplementation review"
type: SpecReview
analysis: gap-analysis
scope: "IR #50 consumer after reviewed codegen #22, #26 and #23 integration"
review_set: subset
---

# Public bound-package oracle consumer

## Summary

The coordinator approved a library-only bound consumer. A local merge integrates PR #22,
PR #26, then PR #23, with documentation conflicts reconciled and
the source branches preserved. Its 32 focused stable tests pass: seven publication, five harness,
ten oracle, five strategy and five coverage primitives. This is not a full CI or shared receipt.

The IR candidate publishes the immutable BoundPackage boundary.
The consumer uses only its public accessors. Inputs remain derived projections from the authoritative
frontend/model lane; manually assembled test projections are labelled synthetic, not frontend proof.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-16001 | high | Dropping an unsupported executable clause could create a false complete batch. Iterate the complete bound population; any failure returns its full ClauseRef and no ArtifactBundle. | FR-001-AC-6 |
| FND-16002 | high | Historical map rows and symbol hashes omit package identity. Add required packageId to map rows and include package in oracle naming, testing equal local identities across two packages. | FR-001-AC-7 |
| FND-16003 | medium | Empty executable populations are valid IR, not malformed or unsupported clauses. Return NoExecutable with bound digest/information references and no artifact or attestation. | FR-001-AC-6 |
| FND-16004 | high | Expression-only generation input does not bind complete package/declaration semantics. The bound path uses the existing attested input-digest slot for the exact IR bound digest/profile; retain per-clause canonical expression/declaration identities without claiming native execution. | FR-001-AC-7 |
| FND-16005 | high | Whole-batch allocation before validation bypasses effective publisher bounds. Preflight count and names, enforce byte limits while collecting, then construct its validated ArtifactBundle; publication stays a separate call. | FR-005-AC-1 |

## Planned public shape

`generate_bound_oracles(&BoundPackage, AttestationContext)` returns `Result<BoundOracleGeneration,
BoundGenerationError>`. Its generated variant owns an immutable ordered per-ClauseRef collection,
canonical bound/declaration/expression identities, informational references and a validated
ArtifactBundle. Its NoExecutable variant has only bound identity and information. The low-level
OracleRequest API stays available, but cannot assert complete package binding. A private derivation
context distinguishes bound from low-level calls; it neither invents executable CLI flags nor adds a
local evidence envelope. Shared attestations remain source-generation statements only.

Any path override is development-only and cannot qualify the final consumer. Regenerate the
oracle golden only through the actual generator for the reviewed package-identity change; preserve
independent operator expectations and source-size rejection controls.

## Verdict

**APPROVED BOUNDED IMPLEMENTATION; NOT COMPLETION.** After implementation, qualify actual bound
generation/publication and failure-before-publication, source-map schema identity, empty/information
cases, cross-package collisions, exact input provenance, stable/MSRV and integrated native producers.
No CLI, pre/post pairing, Kani, aggregate vacuity report or campaign-provenance closure is included.

## Implementation checkpoint (not independent acceptance)

The public consumer now lowers the complete immutable IR population. Seven synthetic projection
tests exercise complete population/order, exact bound/expression/declaration identities, explicit
NoExecutable, cross-package symbol/map identity, whole-batch refusal of a later unsupported clause,
count preflight, declaration-only provenance changes, and actual publication followed by native
compilation/execution against the unchanged exact runtime. A separate byte-accounting unit control
tests both exact publisher limits, one-past refusal and overflow without allocating maximum-sized
fixtures. These controls pass locally; integrated committed-head qualification follows separately.

FND-16001 through FND-16005 have implementations and local controls, not campaign closure. Output
collection retains both immutable clause bundles and the publication bundle; its 128 MiB limit is
an emitted-byte budget, not an assertion of 128 MiB peak process memory. No input/output coverage
verdict is generated. Native coverage remains the earlier six measured standalone-oracle controls.

The original oracle golden failed against actual generated output only at the three package-aware
symbol names; the fixture and compiled symbol references were updated to
that output, leaving the independent true/false expectations and expression source range unchanged.

The implementation passes all 40 focused tests on
stable and exact Rust 1.75.0: eight unit, seven bound consumer, five harness, ten oracle, five strategy
and five coverage primitives. Native generated crate controls and the six pinned stable LLVM cases
execute in these runs; the LLVM producer is still the separately qualified stable toolchain, not
Rust 1.75 LLVM. All-target Clippy with denied warnings, warning-free rustdoc, format and spec
validation pass. No path patch qualified these results. The
coordinator owns integrated assurance and independent acceptance. A documentation-only followup
removes stale interface claims that the IR-owned binding is absent and the old package-less naming
description; it does not implement CLI or aggregate analysis.
