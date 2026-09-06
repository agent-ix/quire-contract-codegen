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

The coordinator approved a library-only bound consumer. Local merge `649ffe0` integrates PR #22
`fae8e42`, PR #26 `490fde7`, then PR #23 `a953a20`, with documentation conflicts reconciled and
the source branches preserved. Its 32 focused stable tests pass: seven publication, five harness,
ten oracle, five strategy and five coverage primitives. This is not a full CI or shared receipt.

IR candidate `93674480c572c237fe87c5d509b17206664bdd62` publishes the immutable BoundPackage boundary.
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

Update the IR git dependency, lockfile and IR_CANDIDATE_REVISION together to the published exact head;
retain runtime `e360dad8a3e0e54f9b8457ff7f3748be0f2acdb3` and its generated manifest fixtures unchanged.
Any path override is development-only and cannot qualify the final consumer. Include the consumer
source in the implementation configuration digest and its independent test census. Regenerate the
oracle golden only through the actual generator for the reviewed package-identity change; preserve
independent operator expectations and source-size rejection controls. Commit before clean-tree
attestation assertions, since build identity intentionally records source dirtiness and commit time.

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

The fetched canonical HTTPS IR pin is exactly `93674480c572c237fe87c5d509b17206664bdd62`, without a
path override. Cargo lock reconciliation downgrades stacker 0.1.25 to the IR-required exact 0.1.15,
adding its Windows platform dependencies and removing the replaced windows-sys dependency.
The implementation digest includes both the new bound consumer and the publisher it calls.
The original oracle golden failed against actual generated output only at the three package-aware
symbol names (`57741ce7…` to `15522505…`); the fixture and compiled symbol references were updated to
that output, leaving the independent true/false expectations and expression source range unchanged.
