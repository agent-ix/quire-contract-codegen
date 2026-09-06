---
id: REV-018
title: "Complete bound coverage observation implementation"
type: SpecReview
analysis: gap-analysis
scope: "Coordinator-approved REV-017 phase A; no native run qualification"
review_set: subset
---

# Complete bound coverage observation implementation

## Summary

Base is published PR #27 `cd345e1dc0199db9abeac9955fd1bfcc121cddc9`.
The design was committed as `5104b3b` and explicitly approved for phase A only.
Four initial aggregate controls were banked at `d34823b` before the new API existed;
their first run failed at the absent imports, not by passing a smaller population.
Additional native/schema/resource controls accompany the implementation.

`analyze_bound_coverage` consumes public BoundPackage, immutable generated outputs,
complete actual artifact bytes, optional LLVM export, and an absolute source root.
It reads no filesystem paths and runs no producer. Its own strict domain output
schema is not an assurance/attestation schema. Every result says `unqualified`.
The concrete read-only API exposes computation state and bounded deterministic JSON;
private report fields have no Deserialize or mutation boundary. Per-requirement membership
is retained through ordered full ClauseRefs rather than a second duplicated bucket array.
No requirement or campaign counters are inferred or summed.
The pinned IR/runtime revisions, dependency roles, source/source-map generation,
source-size guards, publisher, campaign policy, and shared assurance inputs remain unchanged.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-25001 | high | Closed in phase A: exact bound digest, complete clauses, expression/declaration identities, informational population and every generated artifact byte are joined before observations. Foreign or missing inventory emits no classifications. | FR-004-AC-7 |
| FND-25002 | high | Closed in phase A: the independent iterative typed implication census uses left-own-right emission order; exact immutable map bytes, role counts, full identity, ranges and unique probes must match. | FR-004-AC-3, FR-004-AC-7 |
| FND-25003 | high | Closed in phase A: unavailable counts remain null with diagnostics, distinct from measured zero; global refusals retain an explicit not-emitted population. | FR-004-AC-5 |
| FND-25004 | high | Deliberately withheld: native authentication, runtime campaign transport, shared obligation mapping, retained coverage attestations and human sufficiency remain separate owner gates. An all-exercised result is still unqualified. | FR-004-AC-4, FR-004-AC-6, FR-004-AC-8, FR-004-AC-9 |
| FND-25005 | medium | The output is bounded before allocating serialized bytes; oversize analysis clears unpublishable populations with a resource diagnostic instead of manufacturing zeros. Input artifact and LLVM limits remain enforced. | FR-004-AC-5 |

## Native and adversarial controls

The new native test derives seven executable clauses and one informational clause from a
synthetic public projection, generates the whole batch, publishes it, and uses actual
cargo-llvm-cov 0.9.0 / Rust 1.94.1 / LLVM JSON 3.0.1. It re-reads every published artifact
after the native command. Expected classifications are vacuous, implication-free exercised,
sibling partial, unexecuted, nested-consequent partial, exercised, and nested-antecedent
partial. The last clause pins consequent counts `[1, 0]`, distinguishing left-own-right
emission order from ordinary node pre-order. The analysis still says unqualified: this
test does not publish an authenticated native result manifest or imported campaign counts.

Synthetic controls independently remove all/one artifact, mutate actual generated Rust,
remove each map role, duplicate inventory, remove informational population, rebind to a
consistent different package, omit LLVM or one file's measured spans, inject foreign
generated paths, normalize valid root aliases, duplicate normalized coverage filenames,
exceed input size, and select an unsupported format. Valid empty/informational-only
packages are NoExecutable; an executable generation bound to either cannot masquerade as
that result. Each global binding failure has no classified clauses.

Every test-produced output is validated against the new strict schema. Independent schema
mutations refuse run-qualified/passed states, absent identities/counts, qualification flags,
and inconsistent completeness markers. The schema is an output shape contract; there is no
public imported-report decoder whose acceptance could be mistaken for authentication.

## Qualification record

The seven new controls, including native LLVM, passed on stable and Rust 1.75.0 during
bring-up. Initial broader runs correctly refused the dirty source tree in the existing
oracle attestation control (`source_dirty`); that gate is unchanged. Full clean-head
regression and independent review are required before publication.

No full make-ci/shared receipt or FR-004 ticket closure is claimed here. Existing shared
measurement producers still do not publish campaign/vacuity outcomes. Their historical
adopted records are not rewritten into current execution claims by this phase.
