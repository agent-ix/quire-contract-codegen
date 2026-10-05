---
id: SR-1519
title: "integrity review of quire-contract-codegen#280 (frame obligation identity scheme)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@70f5e08ac399d0880b50c1610592220c15298e09; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/assurance/AD-003-evidence-chain.md, spec/assurance/AD-002-cg-qsl-replay-seam.md (diff only); context: spec/kani/functional/FR-015-bounded-kani-obligations.md, QSL c8f0c28 (CG lock 934e26f; files below unchanged between them)"
review_set: subset
---

## Summary

Ticket: IR-459. This analysis checks the new identity scheme for consistency against three
sources:

- merged CG text: FR-015-AC-34, FR-015-AC-48, AD-003 E-1 and the function path's O-09 code;
- QSL's own statements at c8f0c28: `qsl-replay/src/identity.rs` ObligationIdentity docs,
  ADR-013 O-04/O-07/O-09, ADR-011 E3/E7, FR-104 "Requirements" and FR-105's frame row;
- the PR's own Open questions.

The relevant QSL files are the same at CG's locked QSL revision, 934e26f: the
934e26f..c8f0c28 diff of `call_site.rs` and `identity.rs` touches only the compile path and
`Backend`.

QSL facts measured:

- QSL never mints, recomputes or compares the obligation identity. `ObligationIdentity` only
  wraps the 32 bytes. `replay` passes `request.obligation_identity()` into witness decoding as a
  label (`execute.rs:461`). The frame executor never reads it, and `WitnessEnvelope::reconstruct`
  checks only that it is present (`witness.rs:899-902`).
- QSL states the form of the envelope's identity in prose only, and states it with four members:
  subject node id, its occurrence key, kind, and arguments as parameter node id plus domain
  (identity.rs doc; ADR-013 row "Public type"; ADR-011 E7).
- Node ids are content-addressed over the O-04 preimage, and occurrences and regions are excluded
  (ADR-013 "Conversions": "Occurrences are excluded from the node-identity and package-identity
  preimages"; "includes the clause occurrence key, never its regions").
- A frame's `generated` occurrence is minted "one occurrence per distinct operation", ordered by
  (declaring `DeclarationKey`, operation name) and "NOT depend[ent] on source order"; "every
  clause naming the same operation shares its occurrence"; "two operations are two occurrences
  even when their frame nodes coincide" (FR-104 Requirements; FR-105 frame row).

So AC-21's span-insensitivity (a blank-line shift leaves the identity unchanged) is supported by
QSL's spec, and AC-21 measures it through `call_site` rather than assuming it. Two consequences
follow from the occurrence rule:

- The frame occurrence already tells apart two operations with equal frames, so the `anchor`
  member is redundant (FND-001).
- The per-clause frame harnesses `frame_{anchor}_{clause}` of one operation share one identity
  (FND-004). That is consistent with QSL's one frame record per operation.

Dropping the harness and module symbols is therefore safe. Two generated artifacts that mint one
identity are the same QSL obligation, and the decode check (`witness.rs`
`decode_falsification` on module and harness symbols) ties a playback to its artifact.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-024-AC-20 and the AD-003 E-1 edit widen the O-09 preimage in two ways: a fifth `anchor` member, and `{domain, field}` arguments named by field name. This contradicts three sources. (1) Merged FR-015-AC-48 says a state field is named "by its declaring node id and field name", that "E-1 lists parameters only is an open question for the AD-003 owner", and that "E-1 is not widened here". (2) E-1's own sentence "O-09 fixes the members". (3) QSL's four-member statement (identity.rs, ADR-013 O-09, ADR-011 E7). The stated reason for `anchor`, that "two operations with equal frames are two proofs", is already met by the frame occurrence key: FR-104 and FR-105 give each operation its own `generated` occurrence even when frame nodes coincide. The anchor is redundant, and it adds a member O-09 does not have. Field naming and the `anchor` member are the owner question FR-015-AC-48 left open. They are settled here by the edit without an owner answer. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:286; spec/assurance/AD-003-evidence-chain.md:139-176; spec/kani/functional/FR-015-bounded-kani-obligations.md:454 |
| FND-002 | medium | The Open questions section opens with "the criteria above do not depend on an answer", which is false for at least three questions. FR-024-AC-20 mandates the Q-6 rename and the rewritten vectors. FR-024-AC-22 fixes the full `i64` range for an unranged field, which Q-2 leaves open. FR-024-AC-20 and FR-024-AC-21/22 fix field naming by name under the anchor, which Q-5 leaves open. Q-6 and Q-2 (and Q-5 with FND-001) must be settled before merge, or the ACs made conditional. Q-1, Q-3 and Q-4 are correctly recorded: Q-3 keeps E-1's existing no-label interim, and Q-1 and Q-4 are outside the slice. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:350-373 |
| FND-003 | medium | Q-5's premise, "because CG has no declaring node id", is contradicted by the code and by QSL. `StateFrameScope.object` (`src/kani/identity.rs:247`) is the framed object type node, and the harness generator already matches field members by it (`src/kani/generate/frame.rs:647,744`). QSL's frame `modifies` entries are (object type node, field name) pairs (FR-105). The FR-015-AC-48 form, declaring node plus field name, is therefore available today. With it, the anchor-as-namespace reason for FND-001 falls away. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:368 |
| FND-004 | low | Two members are left out without being named, and an edge case goes unstated. (1) `StateFrameIdentity.clause` is neither a member nor listed among the non-members (AC-21 lists symbols, paths, unwind and options only). Because the frame harness is generated per clause (`frame_{anchor}_{clause}`, `src/kani/generate/frame.rs:341-345`), two post clauses on one operation mint one frame identity. That is correct per FR-104 ("every clause naming the same operation shares its occurrence"), but it is stated nowhere and tested nowhere. (2) Ordinals run over the operations the unit's clauses name, so adding a clause on another operation that sorts earlier changes this operation's frame occurrence and therefore its identity. AC-21's stability claim should state that edge. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:287 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | Q-2 is marked 'settled here' by the spec author: an unranged state field gets no range member and is not refused. Q-2 itself (70f5e08) assigned that decision to 'the AD-003 owner and IR'. The decision makes `domain` optional in an argument, which departs from O-09 ('each a parameter node id and its declared domain'; QSL identity.rs) and from E-1's AD-016 arrow 5 rule for function arguments (no bound means requires-bound, no identity). It contradicts no merged state-field requirement: FR-015-AC-27 and AC-61 refuse an unbounded field only for the clause's own field, and the frame harness draws other fields as unconstrained i64. But it is an owner decision taken by the author. Either get the AD-003 owner's ruling or restore Q-2 as open and make AC-22 conditional. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:294,372-376 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | A stale clause left over from 70f5e08: AD-003 still says StateFrameIdentity 'records no draw order for the state fields, which the playback decode and the identity's arguments need (FR-024-AC-23)'. At ef692c0 a frame's arguments are empty and AC-23 says state_fields 'is not minted'. Drop 'and the identity's arguments'. | spec/assurance/AD-003-evidence-chain.md:324-325 |
| FND-007 | low | Leaving fields and ranges out of the preimage rests on 'the frame node names the grants, the ranges come from the model'. AC-21 measures the grants half through call_site (two units differing only in modifies). It never measures that a changed model range reaches the frame node and so the identity (ADR-011 row 8 says model-bound nodes and their referrers are re-minted). The AC's 'does not change when ... ranges change' is about the harness record at a fixed site, which is truthful, but the justification is never exercised. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:294 |

## Dispositions

Round 1, reviewed at b094aaca23d924831ee362d8865d31b74acbb753.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | still-open | The anchor member is gone and the function/declaration names are kept, so the four members are now QSL's four. But E-1's `arguments` now admits a second element shape: `{field:{node,name}}` with `domain` only when a range is declared, beside `{domain, parameter}`. FR-015-AC-48 says that E-1 listing parameters only 'is an open question for the AD-003 owner and E-1 is not widened here', and QSL's O-09 says arguments are 'each a parameter node id and its declared domain'. AD-003 and FR-024 now say 'E-1 is not widened', which is false at the argument level. This needs an AD-003 owner decision admitting state-field arguments, recorded as that decision, and the 'not widened' wording corrected. |
| FND-002 | fixed | b094aac |
| FND-003 | fixed | b094aac |
| FND-004 | fixed | b094aac |

Round 2, reviewed at ef692c01cf2a91585c7c0012928becd15be6cb98.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | ef692c0 |
| FND-005 | fixed | ef692c0 |
