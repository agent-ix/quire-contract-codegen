---
id: SR-1131
title: "base spec review of quire-contract-codegen PR 247 (IR-553 function-path obligation identity)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@f04c7a88081180ddf8170e79a9ea7005f55ae953; spec/replay/functional/FR-016-witness-native-replay.md (FR-016-AC-21..23, Behavior), spec/assurance/AD-002-cg-qsl-replay-seam.md, spec/assurance/AD-003-evidence-chain.md"
review_set: subset
---

## Summary

Ticket: IR-553. Base checklist review of the PR diff (`git diff origin/main...HEAD`, base
5adfd3c), checked against the merged quire-spec-language text at f6c3974 (ADR-013 O-09, the
ADR-013 §2 one-encoder rule, the O-26 replay-request row and FR-121-AC-15, read-only) and
against CG source at the reviewed sha (`src/kani/identity.rs`, `src/replay/function.rs`,
`Cargo.toml`, `tests/it/skeleton_spine.rs`).

Examined and clean: AC-21's member list (function node id, `declaration` occurrence key,
obligation kind, arguments as parameter node id plus declared domain) matches O-09; source
span exclusion matches O-09 and is asserted by AC-22; "never the digest of the transcript"
matches the O-26 row (the function selection's slot is the function-contract obligation);
"two functions with the same parameters differ" and "comments and blank lines leave it
unchanged" match FR-121-AC-15; "one identity per kind it requests, not per conjunct" matches
O-09's wording; `ObligationKind` has exactly four variants (`Precondition`, `Postcondition`,
`Invariant`, `Frame`) at `src/kani/identity.rs:19-30`; AD-002 and AD-003 no longer record the
transcript digest as accepted design; ACs are direct assertions, carry no "shall", are marked
planned, and invent no preimage member beyond O-09 except the kind noted in FND-001; no QSL
text is copied beyond citation and paraphrase. The untrusted ticket body's claim of "a kind
distinct from the other two" is contradicted by O-09 ("The three subject kinds need no
separate tag") and by the ticket's own later comment ("no kind tag").

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-22 mandates a new `ObligationKind` variant distinct from all four, which O-09 does not define (O-09 says no subject tag is needed and a function yields one obligation per CG kind it requests); with a single function-contract kind, AC-22's own "differs when the obligation kind differs" cannot be exercised on the function path | spec/replay/functional/FR-016-witness-native-replay.md:143 |
| FND-002 | medium | AC-21 omits the `arguments` order, which changes the digest; O-09 fixes it ascending by identifier, while `FunctionSite.parameters` is in declared order, so two implementers would encode different bytes | spec/replay/functional/FR-016-witness-native-replay.md:142 |
| FND-003 | medium | AC-23's compile-time clause cannot fail in any CG test (QSL FR-121 types `function` and `declaration` as non-optional), and "no code under src/ computes a node id" names no token set for the scan TC-026 relies on | spec/replay/functional/FR-016-witness-native-replay.md:144 |
| FND-004 | low | AC-21 embeds an upstream commit id ("commit f6c3974") as provenance inside the criterion; the content citation (ADR-013 O-09 as amended) is what the AC needs | spec/replay/functional/FR-016-witness-native-replay.md:142 |
| FND-005 | low | AC-21's "CG's one canonical encoder" is not defined in FR-016; it means AD-003's `core::canonical` over `quire-canonical` (ADR-013 §2's one encoder), which CG does not yet depend on | spec/replay/functional/FR-016-witness-native-replay.md:142 |

## Verdict

AC-21 to AC-23 track O-09 closely on the preimage members, the span exclusion and the
per-kind rule. The defects are the extra kind variant AC-22 decides without a source in O-09,
the unstated argument order in a digest preimage, and an AC-23 clause that cannot fail.

## New findings (disposition pass 1)

Reviewed at 146b5d6306cf1195b82885c3f2705f116978f01c.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | AC-23's scan has no false positives in `src/replay/` today (zero hits), but it does not establish its "so" clause: it covers only `src/replay/`, while AD-003 E-1's one identity function serves the V1, scalar and frame paths and is likely to live outside it (`src/kani/` already names `CheckedNodeId` and `node_tag` widely), and a lexical identifier scan that drops literals misses a node preimage built with string keys | spec/replay/functional/FR-016-witness-native-replay.md:144 |
| FND-007 | low | AC-22 asserts the identity "differs between the function's `Postcondition` and `Invariant` harnesses", but the function path replays only the Postcondition harness today (`tests/it/skeleton_spine.rs:602`, `supported_contract_harnesses(..).remove(1)`, the postcondition per `tests/it/kani_obligations.rs:637`) against a per-clause native twin; the pair exists only if the test builds it, so the clause should say "two different `ObligationKind`s" rather than name a pair | spec/replay/functional/FR-016-witness-native-replay.md:143 |

## Dispositions

Round 1, reviewed at 146b5d6306cf1195b82885c3f2705f116978f01c.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 146b5d6 |
| FND-002 | fixed | 146b5d6 |
| FND-003 | fixed | 146b5d6 |
| FND-004 | fixed | 146b5d6 |
| FND-005 | fixed | 146b5d6 |
| FND-006 | fixed | 6b16acb |
| FND-007 | fixed | 6b16acb |

## New findings (disposition pass 2)

Reviewed at 6b16acb75637b6625c6495edaf749071df8410f9.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | medium | AC-23's scan clause can pass vacuously and can fail a correct implementation: `src/core/canonical.rs` does not exist (AD-004 step 1a is held), and the AC does not say a missing scanned path fails, so that half finds nothing until the file exists; and it bans the identifier `OccurrenceKey`, which `qsl-replay` re-exports as the type of `FunctionSite.declaration` that the preimage must carry, so it bans naming that type, not deriving a key | spec/replay/functional/FR-016-witness-native-replay.md:144 |

## Dispositions (disposition pass 3)

Round 3, reviewed at 59cf84a7324cc414fa36bc06624b837eab13731f.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-008 | fixed | 59cf84a |
