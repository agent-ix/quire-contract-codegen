---
id: SR-11641
title: "Delta disposition for IR-268 collection claim fixture"
type: SpecReview
analysis: base
scope: "Delta-only review of SR-11640 findings against tests/it/kani_obligations_symbolic_domains.rs in the IR-268 code worktree"
review_set: subset
---

## Summary

Delta-only review of the fixture changes addressing SR-11640; the FR-015/TC-025 base review was not repeated. The focused QSL-emission and Contract IR admission test passed 1/1 with both `forall` and `exists`. FND-001 is resolved by the admitted model-field projection and its bounded type evidence. FND-002 remains deferred implementation coverage for AC-97, AC-99, AC-100 and AC-101. No new finding arose from this fixture delta.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | RESOLVED. The fixture quantifies over `deref(input).items` for `input: M::Node`; the admitted projection is `quire.op.record.project`, field `items`, declared by the admitted Node object, with a model-typed input receiver. Its admitted collection type has inclusive bounds [0,3] over a sequence, and Contract IR's admitted `Node.items` field is `Sequence<IntRange[0,3]>` with cardinality [0,3]. Both `forall` and `exists` resolve to that same projection and semantic type. Focused command `cargo test --locked --test it qsl_emits_and_ir_admits_the_authored_collection_and_reference_shapes -- --nocapture` passed 1/1. | FR-015-AC-96, FR-015-AC-99, TC-025
| FND-002 | medium | DEFERRED to implementation tests. The fixture now provides source/reader shape evidence for AC-96, AC-98 and AC-102, but does not yet exercise typed refusal controls (AC-97), real Kani verification/native replay and mutation evidence (AC-99), identity changes (AC-100), or typed ceiling/malformed-declaration refusals (AC-101). These controls remain required before implementation coverage is claimed. | FR-015-AC-97, FR-015-AC-99, FR-015-AC-100, FR-015-AC-101, TC-025
