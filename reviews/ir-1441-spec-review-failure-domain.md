---
id: SR-17940486709564847047590
title: "IR-1441 PR #767 FR-052 failure-domain analysis"
type: SpecReview
analysis: failure-domain
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

CHANGES SUGGESTED. Outcome reachability, charge order, the one-parameter-drawn-once rule, the P <= C no-clip rule and the capacity/ceiling statement are sound against text.rs prepare/compare and FR-028 (a run over the request's ceiling is inconclusive, never Verified). One internal contradiction in the tractability contract.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | P1 is defined with the IR-1436 D4 contract ('no allocation, no chars().count(), no SWAR and no iterator loop, counting and ordering as byte loops of at most the carrier capacity'), but the Behavior requires the native expression for normalizing profiles to take the retained sequence from the owner's `NormalizationForm::apply` stream, an iterator over unicode_normalization Decompositions/Recompositions with internal buffering, i.e. exactly the construct D4 bans. The oracle under proof is no better: compare_text -> prepare allocates Strings (to_owned / push), counts scalars with a filter().count() and orders with chars().cmp, so the D4 byte-loop contract cannot bound the oracle side at all. State that D4 governs the operand carrier and the non-normalizing native expression only, that normalizing profiles and the owner oracle path are outside it, and that their tractability is unestablished (covered by the existing 'no capacity is claimed' paragraph). | spec/kani/functional/FR-052-text-scalar-harnesses.md:154 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-052-prerequisite-P1 | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:54 | Prerequisite P1 is the quire-exact text carrier (ticket IR-1445): a symbolic exact `Text` operand and a native expression over it, with the tractability contract of the IR-1436 design review (D4): `Copy`, no allocation, no `chars().count()`, no SWAR and no iterator loop, counting and ordering as byte loops of at most the carrier capacity. It is not delivered. Every criterion that describes a supported harness is conditional on P1. Two facts about today's owner make P1 necessary and are not assumed away. The only public way to obtain an owner `Text` is the metered `admit_text`, so a harness ... |
| FR-052-behavior-native | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:154 | When evaluating, the harness SHALL compare the FR-014 generated oracle with a native expression that is independent of that oracle (not a second call of the same function), from matching entry Meter state and limits. The comparison covers the full typed outcome, ordered charges, the denied next charge and consumed counters. The native expression decides the operator with `==`, `!=`, `<`, `<=`, `>` and `>=` over the byte-lexicographic order of the two retained sequences. UTF-8 byte order equals Unicode scalar order, which is the order the owner uses for every profile other than `binary-utf8`... |
| FR-052-behavior-charges | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:173 | The charges are those of the owner's `prepare` and are expected in this order. `text.input-bytes` (the sum of the two payload byte lengths, and two value occurrences), then `text.decode-scalars` (the sum of the two payload scalar counts). For `nfc`, `nfd`, `nfkc` and `nfkd` then `text.normalize-input`, then one `text.normalize-output` before each emitted scalar across both operands, left then right, with a running emitted count. Last `text.result-retain`. The non-normalizing profiles charge no `text.normalize-*` point. A comparison applies no length bound, so it never refuses a length. |
| FR-052-behavior-outcomes | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:182 | The typed outcomes the owner produces are these. `Completed` with the Boolean. `Incomplete` at a charge point, `text.input-bytes`, `text.decode-scalars`, `text.normalize-input`, `text.normalize-output` or `text.result-retain`, with its limit kind. A comparison produces no `Undefined` and no `Refused`, so neither is a drawn control. None is assumed away, and a stopped outcome never becomes a completed value. |
| FR-052-bound-and-ceiling | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:95 | The owner admits a payload by its retained sequence, so P is not the declared `max`. For `binary-utf8` no normalization runs, retained equals payload, and P = `max`. For `unicode-scalars` retained equals payload and a scalar is at most four bytes, so P = 4 x `max` (checked arithmetic). For `nfc`, `nfd`, `nfkc` and `nfkd` the payload can differ in length from the retained sequence in either direction, and the bound on payload scalars per retained scalar comes from the pinned Unicode 17.0.0 tables. This requirement asserts no multiplier. The carrier (P1) must state P for these four profiles w... |
| FR-052-behavior-draw | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:142 | When the comparison is representable, the harness SHALL draw each operand as a payload through the carrier: a byte length in `0..=C` and C payload bytes, where every byte at or after the length is zero, the prefix is valid UTF-8, and the retained sequence of that payload is admitted by the operand's declared `TextType` (`TextType::admits` over the profile's retained sequence). Every admitted payload is reachable exactly once, including payloads that differ only in normal form (`e` followed by U+0301 and U+00E9 under `nfc`), and no other byte string is reachable. Two distinct parameters are ... |
| FR-052-AC-8 | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:236 | A healthy text comparison receives `Verified` only from an actual successful proof with all checks and exactly one final completion cover satisfied. Unsatisfied cover, failed unwind, timeout, memory exhaustion or missing verdict cannot yield `Verified`, and FR-017 and FR-029 classifications are retained. A capacity whose recorded run exceeds the request's memory or wall-clock ceiling settles `inconclusive` under FR-028, and no capacity is reported verified by a run at a larger ceiling than the request's. |
