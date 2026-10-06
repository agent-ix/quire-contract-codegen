---
id: SR-1800
title: "IR-643 PR A code-review review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@622299873a5157c1fcc20f42e280858d0914e9cd; src/oracle/equality/mod.rs; tests/composite_equality_support/package.rs"
review_set: subset
---

## Summary

Ticket: IR-643. Reviewed PR #306's reader slice with the Rust idiom and code-smell checklist at the pinned head. The optional-field and bounded-sequence readers match the fixture shapes.

## Verdict

**PASS** — no code or Rust-lane defect found in this two-file reader diff. Self-cycle and type-resolution work limits belong to PR B; the QSL644 conformance lane remains separate.

## Coverage

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-018-AC-23 | context_only | For every item of the TC-029 corpus whose leaves are Boolean or bounded integers, the claim-map closure's member names, presence and each integer leaf's inclusive bounds equal those of an independent read of the checked package (the V2 composite and `bounded_domain` nodes read through Contract IR's reader, never through the generator's reconstruction); a closure that dropped a member, narrowed a bound by one or read an optional member as required fails the check. PLANNED (IR-264). |
| FR-018-AC-24 | examined | Type reconstruction of QSpec's `positive-recursive-records.json` List (through `Option<List>`) and Tree (through `Sequence<Tree>` bounded `[0,3]`) closes each back-edge at the in-progress record key, generates equality items built over both record types, and terminates; a synthetic unclosed `Option` self-cycle and a `Sequence` self-cycle each refuse as `TypeResolutionCycle` naming the repeated type node, without a panic, stack overflow or generated symbol. PLANNED (IR-643). |
| FR-018-AC-26 | examined | Reading the QSpec List `next` field as `binding(next, aggregate([binding(optional, reference Option<List>)]))` yields exactly one `FieldDeclaration` named `next`, with `Presence::Optional` and payload type `ValueType::Composite(List NodeKey)`; removing or renaming `optional`, adding a second nested member, or targeting a non-option node refuses only that item as malformed, never as a required field or a generated oracle. PLANNED (IR-643). |
| FR-018-AC-27 | examined | Reading the QSpec Tree `kids` field's direct reference to `collection_bounds` follows that node's `semantic_type` to `Sequence<Tree>`, reads its sole element reference as the Tree `NodeKey`, and reconstructs cardinality `[0,3]` from that bound node's named `min` and `max`; changing the semantic type to a non-sequence, dropping or duplicating either bound, or making either value non-integer refuses only that item with a typed cause and emits no code, never silently yielding an unbounded collection. PLANNED (IR-643). |
| src/oracle/equality/mod.rs | examined | read_record_field and resolve_collection reconstruct wrapped optional fields and bounded Sequence<Tree>. |
| tests/composite_equality_support/package.rs | examined | The recursive corpus next field uses the optional wrapper shape. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
