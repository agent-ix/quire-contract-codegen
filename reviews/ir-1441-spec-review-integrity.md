---
id: SR-17940486709564847047587
title: "IR-1441 PR #767 FR-052 integrity analysis"
type: SpecReview
analysis: integrity
date: 2026-11-10
scope: "agent-ix/quire-contract-codegen; PR #767 @ 2362dae1; IR-1441; spec/kani/functional/FR-052-text-scalar-harnesses.md, spec/kani/matrix/TC-066-text-scalar-harnesses.md, spec/kani/functional/FR-025-generated-subject-abi.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/matrix/TC-036-generated-subject-abi.md, spec/routed/matrix/TC-030-capability-settlement.md, spec/kani/matrix/tests.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-052
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-066
    type: reviews
---

## Summary

Ticket: IR-1441. Spec-only PR #767 (head 2362dae1) adding FR-052 and TC-066, the planned Kani row for the six text comparisons with a payload-byte bound P and carrier capacity C, with edits to FR-025 (Text ABI row, AC-7, new AC-16), FR-028, TC-036 step 16, TC-030 Text row, spec/kani/matrix/tests.md and spec/spec.md. Reviewed against origin/main cf10ec78 (includes FR-049/050/051) and quire-exact 5db2d5e3 (CG Cargo.lock pin; src/text.rs and accounting.rs unchanged at origin/main 4fe7df6), CG src/oracle/scalar/{derive,lower,render}.rs, src/core/ir/scalar_bounds.rs, src/routed/capability.rs, QSpec FR-196, Linear IR-1445, IR-261 and the IR-1436 design-review comment.

## Verdict

CHANGES SUGGESTED. The branch is one commit on origin/main cf10ec78 (merge-base equals main tip; git merge-tree clean). Shared lines coexist one per family with FR-049/050/051: FR-025 Text ABI row sits beside Decimal/Enum/Quantity, FR-025-AC-7 lists 'text outside the planned FR-052 row' alongside the other carve-outs, FR-025-AC-16 is inside the AC table after AC-15, TC-036 step 16 and expected result 16 follow 15, tests.md adds FR-052 and FR-025-AC-16 to TC-036's row, spec.md and the FR-028 shadow bullet each add one text clause, and TC-030's Text row adds the FR-052 supersession clause consistent with FR-014/FR-015 (OperationNotRendered via NoRenderer until delivered). make spec passes with no FR-052/TC-066 warnings; quire matrix lists FR-052-AC-1..11 and FR-025-AC-16 as untagged, as planned. Two low wording defects.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | 'the item is refused with the dispositions below and no harness' points at nothing: the next section is the Symbolic ABI, not a disposition list. The dispositions are spread over AC-3 (P > C, unstated P), AC-4 (requires-bound, unreadable bound, distinct profiles) and AC-7 (no carrier: OperationNotRendered). Name them or cite those ACs. | spec/kani/functional/FR-052-text-scalar-harnesses.md:192 |
| FND-002 | low | AC-4 attributes all of 'a non-canonical or beyond-u64 count, min above max, or an unknown profile' to FR-014-AC-16, whose text covers only member lookup (bare literal, missing, duplicate or unlisted name). CG's read_text_bounds (src/core/ir/scalar_bounds.rs:73) does refuse all of them as unreadable, but the cited AC does not state it. Cite the FR-014 text_bounds reader prose or the criterion covering counts, empty bounds and profile codes, or scope the citation to the member cases. | spec/kani/functional/FR-052-text-scalar-harnesses.md:232 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-025-AC-16 | examined | spec/kani/functional/FR-025-generated-subject-abi.md:190 | PLANNED (IR-1441). Every supported FR-052 text operand has ordered `i64` physical bindings (the byte length, then the payload words) grouped to its original parameter, `text_bounds` domain and capacity C. Playback decodes with the existing I64 width/comment checks and entry and byte caps, validates the length, the zero padding, the UTF-8 prefix and the retained sequence's admission by the declared `TextType`, then submits one `WitnessValue::Text` per parameter. Missing, duplicate, foreign, role-swapped or domain-conflicting schema, malformed entries and out-of-domain payloads refuse with a ... |
| FR-025-AC-7 | examined | spec/kani/functional/FR-025-generated-subject-abi.md:181 | An argument with no applicable row (decimal outside the planned FR-051 row, IEEE, text outside the planned FR-052 row, enum outside the planned FR-049 row, composite, collection, function, quantity outside the planned FR-050 row, or Rational outside the planned FR-046 row) is accounted `unsupported` with a typed reason naming its family/capability, and no harness is emitted. The planned Rational row does not authorize a harness without its lossless decoding and public replay prerequisites. |
| FR-025-text-row | examined | spec/kani/functional/FR-025-generated-subject-abi.md:75 | / Bounded Text (planned [FR-052](./FR-052-text-scalar-harnesses.md) scalar comparisons only) / `i64` calls: the byte length, then the payload bytes packed eight to a little-endian word for exactly C bytes (C the carrier capacity), length in `0..=C`, bytes at and after the length zero, prefix valid UTF-8, retained sequence admitted by the owner `TextType`; the payload-byte bound must not exceed C, else the item is `unsupported` / one owner `Text` per parameter, built by the carrier from that payload and its declared `TextType` / |
| FR-028-shadow-bullet | examined | spec/kani/functional/FR-028-bounded-proof-ceilings.md:358 | text (leaves only; scalar comparisons are planned under FR-052), enum (a standalone comparison node is planned under FR-049; an enum leaf inside a composite stays here), rational, decimal (leaves only; scalar operations are planned under FR-051), quantity, float, sequence, set, bag and ordered set; the scalar families are their own rows and PRs (quantity scalar operations: planned FR-050; decimal scalar operations: planned FR-051; text scalar comparisons: planned FR-052). |
| TC-036-step-16 | examined | spec/kani/matrix/TC-036-generated-subject-abi.md:79 | 16. For the planned [FR-052](../functional/FR-052-text-scalar-harnesses.md) text row, generate a text comparison and read its persisted identity: assert the byte-length binding then the packed payload words, grouped to the original parameter with its `text_bounds` domain and capacity. Decode retained playback with the existing I64 checks and caps; assert a valid payload submits one `WitnessValue::Text`, and a missing, duplicate, foreign, role-swapped, over-length, nonzero-padding, invalid-UTF-8, out-of-domain or over-limit entry refuses with a typed reason and no replay. |
| TC-036-expected-16 | examined | spec/kani/matrix/TC-036-generated-subject-abi.md:131 | 16. One text operand has its length binding and payload words grouped with its domain and capacity; a valid payload replays as one Text, and each malformed or out-of-domain case refuses with no replay (FR-025-AC-16). |
| tests.md-TC-036-row | examined | spec/kani/matrix/tests.md:132 | / TC-036 / Verify the generated harness subject ABI / Integration / P0 / FR-025-AC-1, FR-025-AC-2, FR-025-AC-3, FR-025-AC-4, FR-025-AC-5, FR-025-AC-6, FR-025-AC-7, FR-025-AC-8, FR-025-AC-9, FR-025-AC-14, FR-025-AC-15, FR-025-AC-16 / 🚧 Planned / |
| tests.md-FR-052-row | examined | spec/kani/matrix/tests.md:13 | / FR-052 / FR-052-AC-1 through FR-052-AC-11 / TC-066 / 🚧 PLANNED (IR-1441): no text scalar Kani harness; needs the quire-exact text carrier (IR-1445); TC-066 controls have not run. / |
| spec.md-FR-052-row | examined | spec/spec.md:92 | / Text scalar comparison harnesses / [FR-052](kani/functional/FR-052-text-scalar-harnesses.md) planned text comparison proof row with a payload-byte bound (conditional on IR-1445) / [TC-066](kani/matrix/TC-066-text-scalar-harnesses.md) planned proof and mutant controls / |
| TC-030-text-row | examined | spec/routed/matrix/TC-030-capability-settlement.md:399 | / Text scalar `expression` claim, IR-confirmed generated claim / Planned (IR-269) / Text scalar operation forms; the six comparisons are planned under FR-052 (conditional on the quire-exact text carrier, IR-1445), which supersedes this row for them only when delivered. / `OperationNotRendered` (`classify_claim` maps `ScalarLoweringRefusal::NoRenderer`). / |
| FR-052-behavior-refusals | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:192 | When the carrier, a finite `text_bounds` operand bound, a payload-byte bound within C, the resolved operand form or the native expression is unavailable, the item is refused with the dispositions below and no harness. A text leaf inside a composite remains FR-015's `ShadowFamilyNotBuilt`. |
| FR-052-AC-4 | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:232 | A text operand typed by no `text_bounds` (a plain `text`) yields source-associated `requires-bound` naming the operand's type node with no harness. A `text_bounds` that is unreadable under FR-014-AC-16 (a missing, duplicate or unlisted member, a non-canonical or beyond-`u64` count, `min` above `max`, or an unknown profile) yields named `unsupported` retaining that cause. Operands whose profiles differ are refused with `DistinctTextProfiles` and no harness. No result bound is required, read or reported, because a comparison has none. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The fix replaced 'dispositions below' with 'the dispositions of AC-1, AC-3 and AC-4', but three of the listed causes (the carrier, the Text constructor and the native expression unavailable) are settled by AC-7 (TC-030's OperationNotRendered/NoRenderer, or their own named disposition), which none of AC-1, AC-3 or AC-4 covers. Add AC-7. | spec/kani/functional/FR-052-text-scalar-harnesses.md:218 |
| FND-004 | low | AC-4 (and TC-066 step 4) now calls read_text_bounds 'the owner reader ... a CG function'. Everywhere else in FR-052 'owner' means quire-exact; read_text_bounds is CG's src/core/ir/scalar_bounds.rs. Call it CG's text_bounds reader. | spec/kani/functional/FR-052-text-scalar-harnesses.md:261 |

## Scope (disposition pass 1)

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-052-behavior-refusals | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:218 | When the carrier, the `Text` constructor, a finite `text_bounds` operand bound, a payload-byte bound within C, the resolved operand form or the native expression is unavailable, the item is refused with the dispositions of AC-1, AC-3 and AC-4 and no harness. A text leaf inside a composite remains FR-015's `ShadowFamilyNotBuilt`. |
| FR-052-AC-4 | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:261 | A text operand typed by no `text_bounds` (a plain `text`) yields source-associated `requires-bound` naming the operand's type node with no harness. A `text_bounds` that is unreadable yields named `unsupported` retaining its cause: a missing, duplicate or unlisted member (FR-014-AC-16), or a non-canonical or beyond-`u64` count, `min` above `max` or an unknown profile (each yields no bound from the owner reader `read_text_bounds`, a CG function that returns no `TextType` for them). Operands whose profiles differ are refused with `DistinctTextProfiles` and no harness. No result bound is requir... |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | fixed in abde7ddc; the replacement citation is incomplete, see FND-003 |
| FND-002 | fixed | fixed in abde7ddc |
| FND-003 | fixed | fixed in 1be1f107 |
| FND-004 | fixed | fixed in 1be1f107 |
