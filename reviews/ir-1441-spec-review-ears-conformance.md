---
id: SR-17940486709564847047588
title: "IR-1441 PR #767 FR-052 EARS conformance analysis"
type: SpecReview
analysis: ears-conformance
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

PASS. The FR-052 statement and each Behavior and ABI paragraph use the event or unwanted-behaviour pattern ('When ..., CG/the harness SHALL ...', 'If ..., then CG SHALL ...') with a named subject; make spec emits no ears: or ac: warning for FR-052, TC-066 or the edited FR-025/FR-028/TC-036/TC-030 lines (the warnings it prints are pre-existing in FR-015, FR-017, FR-014 and FR-021).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-052 | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:43 | When an admitted request selects a reader-admitted `text` scalar comparison (`quire.op.text.eq`, `.ne`, `.lt`, `.le`, `.gt`, `.ge`) over two parameters each typed by a `text_bounds` domain, and the planned prerequisite below is available, CG SHALL generate a bounded Kani harness that proves the generated text oracle of [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) agrees with an independent native expression of the same comparison over every admitted symbolic operand pair. This is the text row of the IR-269 scalar renderer that TC-030's Text scalar row settles today as `O... |
| FR-052-behavior-draw | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:142 | When the comparison is representable, the harness SHALL draw each operand as a payload through the carrier: a byte length in `0..=C` and C payload bytes, where every byte at or after the length is zero, the prefix is valid UTF-8, and the retained sequence of that payload is admitted by the operand's declared `TextType` (`TextType::admits` over the profile's retained sequence). Every admitted payload is reachable exactly once, including payloads that differ only in normal form (`e` followed by U+0301 and U+00E9 under `nfc`), and no other byte string is reachable. Two distinct parameters are ... |
| FR-052-behavior-native | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:154 | When evaluating, the harness SHALL compare the FR-014 generated oracle with a native expression that is independent of that oracle (not a second call of the same function), from matching entry Meter state and limits. The comparison covers the full typed outcome, ordered charges, the denied next charge and consumed counters. The native expression decides the operator with `==`, `!=`, `<`, `<=`, `>` and `>=` over the byte-lexicographic order of the two retained sequences. UTF-8 byte order equals Unicode scalar order, which is the order the owner uses for every profile other than `binary-utf8`... |
| FR-052-behavior-profiles | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:166 | The operands must share a profile. If the two operands' `text_bounds` name different profiles, then CG SHALL refuse the item with the owner's ill-typed cause `DistinctTextProfiles`, source association and no harness. At run time `compare_text` returns `Err(IllTyped)` with that cause and consumes nothing, which the oracle surfaces as `OracleStop::IllTyped`. The two operands' `min` and `max` may differ. |
| FR-052-abi | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:199 | When expanding a logical text operand, CG SHALL persist physical `i64` bindings for its one real parameter in this order: the byte length, then the payload bytes packed eight to a word, little-endian, in position order, for exactly C bytes. The bindings are grouped to the original parameter identity, role, the complete `text_bounds` domain and C. Logical parameter order remains ascending by parameter identifier; Boolean and integer ordering is unchanged. The carrier (P1) owns the exact binding facts; these are required conceptual facts, not a claim that a current Rust field carries them.  W... |
| FR-052-AC-1 | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:229 | Each admitted `quire.op.text.eq`, `.ne`, `.lt`, `.le`, `.gt` and `.ge` node over two parameters typed by `text_bounds` selects a harness naming the operation identity, both operand forms and their bounds from the admitted node; the oracle is FR-014's and the expected side is an independent native expression, not a second call of the oracle. `quire.op.text.size` selects no harness and keeps FR-014's `OperationNotDerivable`. A text admission claim selects no harness here and stays FR-035's. A text literal operand, and an operand that is not a parameter, is accounted `unsupported` naming `Oper... |
| FR-052-AC-9 | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:237 | Each mutant below fails with actual solver playback of an in-domain input, caught by the listed operators over a control domain that contains its distinguishing point. (a) `eq` for `ne` (and the converse): `eq` and `ne`. (b) `lt` for `le`, `gt` for `ge` (and converses): the matching ordering operator, equal operands. (c) Operand swap: `lt`, `le`, `gt` and `ge` over distinct operands; `eq` and `ne` swaps are equivalent and are not claimed. (d) Normalization skipped, or a wrong profile in the oracle: all six operators over `e` followed by U+0301 and U+00E9 under `nfc`, in both operand orders.... |
