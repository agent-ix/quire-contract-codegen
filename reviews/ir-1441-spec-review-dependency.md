---
id: SR-17940486709564847047589
title: "IR-1441 PR #767 FR-052 dependency analysis"
type: SpecReview
analysis: dependency
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

CHANGES REQUIRED. depends_on edges to FR-014, FR-015, FR-016, FR-017, FR-025, FR-028, FR-029 and FR-039 match every requirement FR-052 delegates to; FR-035 is correctly a references edge only (no admission claim is added); Contract IR FR-038, FR-046/049/050/051 and QSpec FR-196 (AC-5, finite-encoding correspondence, exists) are references. text_bounds is an admitted bound form today (read_text_bounds), so no CG bound ticket is needed. Of the four items the author lists as unticketed: capacity C with recorded peak memory is IR-1445 scope (its Verification is 'quire-exact Kani proofs VERIFIED with a recorded peak memory' and its AC 'Memory stays within the K-DR budget'); the CG text ABI/decoder/replay grouping is the implementation of this FR, i.e. the IR-261 row's own work, not a prerequisite (qsl_replay WitnessValue::Text exists). The other two are genuine prerequisites with no owner; see the finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-052 assigns two prerequisites to 'P1 (IR-1445)' that IR-1445 does not contain (its AC: carrier and Kani proofs land; memory within K-DR budget; CG's row harness consumes the carrier). (1) A non-metered owner Text constructor: the generated oracle takes `&rt::Text` (render.rs comparison_body), Text's fields are private and admit_text is the only constructor, yet the FR-025 Text row has 'one owner Text per parameter, built by the carrier'. (2) The payload-byte bound P for nfc/nfd/nfkc/nfkd ('open obligations of P1'). Without (2) every normalizing-profile criterion is unsupported (AC-3), so AC-5's normalize points, AC-9 (d), (h) for normalize charges and (i) emitted-count reset, and TC-066 step 2's nfc enumeration can never run; IR-1441's own AC 'the row ticket is unblocked by this spec' and the PR's 'unblocks row IR-261' are then overstated. Add both to IR-1445's AC (or file tickets) and cite them, or scope the normalizing-profile criteria as conditional on that ticket. | spec/kani/functional/FR-052-text-scalar-harnesses.md:54 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-052-frontmatter | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:5 | relationships: - target: ix://agent-ix/quire-contract-codegen/StR-001 type: satisfies - target: ix://agent-ix/quire-contract-codegen/FR-014 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-015 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-016 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-017 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-025 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-028 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-029 type: depends_on - target: ix:/... |
| FR-052-dependencies | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:243 | - [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) owns text comparison oracle semantics and the derivable operation list; its oracle agreement is not this proof. - The text carrier (IR-1445) is prerequisite P1. The Contract IR `text_bounds` shape (IR-1263) and literal membership (IR-1260) are recorded Done; the IR-1436 design review (D4) fixes the carrier contract and gates the memory evidence. - [FR-035](../../oracle/functional/FR-035-caller-text-admission.md) owns caller text admission and the TC-050 proof-cost record (IR-494, IR-837, IR-796). Its planned edits (open PRs ... |
| FR-052-prerequisite-P1 | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:54 | Prerequisite P1 is the quire-exact text carrier (ticket IR-1445): a symbolic exact `Text` operand and a native expression over it, with the tractability contract of the IR-1436 design review (D4): `Copy`, no allocation, no `chars().count()`, no SWAR and no iterator loop, counting and ordering as byte loops of at most the carrier capacity. It is not delivered. Every criterion that describes a supported harness is conditional on P1. Two facts about today's owner make P1 necessary and are not assumed away. The only public way to obtain an owner `Text` is the metered `admit_text`, so a harness ... |
| FR-052-status | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:266 | PLANNED (IR-1441). This specification supplies no implementation or generated-proof credit. All eleven criteria remain planned and untagged until P1 is delivered with the IR-269 text harness. The public Text replay value exists in `qsl_replay`; the CG decoder grouping and the carrier's physical bindings do not. Oracle agreement under FR-014 is not Kani credit. No capacity C is known to verify within the request's ceilings, and the four normalizing profiles have no stated payload-byte bound; both are open obligations of P1, not waived gaps. |
| FR-052-bound-and-ceiling | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:95 | The owner admits a payload by its retained sequence, so P is not the declared `max`. For `binary-utf8` no normalization runs, retained equals payload, and P = `max`. For `unicode-scalars` retained equals payload and a scalar is at most four bytes, so P = 4 x `max` (checked arithmetic). For `nfc`, `nfd`, `nfkc` and `nfkd` the payload can differ in length from the retained sequence in either direction, and the bound on payload scalars per retained scalar comes from the pinned Unicode 17.0.0 tables. This requirement asserts no multiplier. The carrier (P1) must state P for these four profiles w... |
| FR-025-text-row | examined | spec/kani/functional/FR-025-generated-subject-abi.md:75 | / Bounded Text (planned [FR-052](./FR-052-text-scalar-harnesses.md) scalar comparisons only) / `i64` calls: the byte length, then the payload bytes packed eight to a little-endian word for exactly C bytes (C the carrier capacity), length in `0..=C`, bytes at and after the length zero, prefix valid UTF-8, retained sequence admitted by the owner `TextType`; the payload-byte bound must not exceed C, else the item is `unsupported` / one owner `Text` per parameter, built by the carrier from that payload and its declared `TextType` / |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | The fix adds a new deliverable to P1: 'For the four normalizing profiles it also provides a loop-free normalization', and the Behavior now takes the normalizing retained sequence from it. IR-1445's AC (carrier and Kani proofs land; memory within K-DR budget; CG's row harness consumes the carrier) does not name normalization, and IR-1500 covers only the bound P. A loop-free NFC/NFD/NFKC/NFKD over the Unicode 17 tables is a substantial piece of work with no owner, so every normalizing-profile criterion is again gated on an unticketed item (the same class as FND-001). Add it to IR-1445's AC or ticket it, and list it in the conditional sentence. | spec/kani/functional/FR-052-text-scalar-harnesses.md:58 |
| FND-003 | low | The P3 conditional list ('AC-5, AC-9 (d), (h) and (i) and AC-10') omits AC-2, whose 'normal-form variants of one text' and TC-066 step 2's nfc enumeration need a normalizing profile, and AC-6, whose Incomplete at text.normalize-input/-output is only reachable under one. Add both (or state the rule 'every normalizing-profile part' without a list). | spec/kani/functional/FR-052-text-scalar-harnesses.md:252 |

## Scope (disposition pass 1)

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-052-prerequisite-P1 | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:58 | - P1 (ticket IR-1445) is the quire-exact text carrier: a symbolic exact `Text` operand and a native expression over it, with the tractability contract of the IR-1436 design review (D4): `Copy`, no allocation, no `chars().count()`, no SWAR and no iterator loop, counting and ordering as byte loops of at most the carrier capacity C. The carrier's capacity C and the recorded peak memory per capacity, within the K-DR memory budget its acceptance criteria name, are within IR-1445's scope. For the four normalizing profiles it also provides a loop-free normalization; absent that, those profiles are... |
| FR-052-AC-conditions | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:252 | All supported-harness criteria are conditional on P1 (IR-1445) and P2 (IR-1499). The normalizing-profile parts of AC-5, AC-9 (d), (h) and (i) and AC-10 are also conditional on P3 (IR-1500). |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | AC-12 runs 'the carrier's cfg(kani) replacement of compare_text' on the host. Code under cfg(kani) does not compile in a host build, so the replacement has to be exposed host-side (a feature, cfg(any(kani, test)) or a public twin). IR-1501 asks for this only for the normalizing profiles. For binary-utf8 and unicode-scalars the replacement is P1's (IR-1445), and neither the P1 bullet nor IR-1445's acceptance criteria require it to be host-callable, so AC-12 has no stated way to run for those profiles. Add host exposure of the replacement to P1 (and to IR-1445). | spec/kani/functional/FR-052-text-scalar-harnesses.md:58 |

## Scope (disposition pass 2)

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-052-prerequisite-P1 | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:58 | - P1 (ticket IR-1445) is the quire-exact text carrier: a symbolic exact `Text` operand and a native expression over it, with the tractability contract of the IR-1436 design review (D4): `Copy`, no allocation, no `chars().count()`, no SWAR and no iterator loop, counting and ordering as byte loops of at most the carrier capacity C. The carrier's capacity C and the recorded peak memory per capacity, within the K-DR memory budget its acceptance criteria name, are within IR-1445's scope. It also provides the loop-free length check the proof-path draw constraint uses (AC-2), and a loop-free compa... |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | fixed in abde7ddc |
| FND-002 | fixed | fixed in 1be1f107 |
| FND-003 | still-open | Half-applied, and partly regressed. The AC-section sentence now puts the normalizing parts of AC-2, AC-5, AC-6, AC-9 (d)(h)(i), AC-10 and AC-12 under P4, but under P3 only 'the payload-byte bound of AC-3 and the normalizing-profile parts of AC-2 and AC-10'. AC-6 is still missing from the P3 list, and AC-5 and AC-9 (d)(h)(i), which were on it before, have been dropped. Yet AC-3 makes every normalizing profile unsupported until IR-1500 states P, and the Description and the P3 bullet ('Every normalizing-profile criterion is conditional on it') say the same. Make the P3 list match the P4 list (or say 'every normalizing-profile part ... is conditional on P3 and P4'). |
| FND-003 | fixed | fixed in 672d8079 |
| FND-004 | fixed | fixed in 672d8079 |
