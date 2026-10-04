---
id: "SR-1387"
title: "CG PR 258 spec review (EARS conformance): new FR-017 and FR-028 behaviour bullets"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-contract-codegen@1add57d77cfcd3a9e39e515f063fde12b81f4d70; spec/kani/functional/FR-017-kani-execution-evidence.md:90-122 (seven new or edited Behavior bullets), spec/kani/functional/FR-028-bounded-proof-ceilings.md:55-66 (one new bullet) (diff origin/main...HEAD)"
---

# SR-1387: CG PR 258 spec review (EARS conformance)

## Summary

Ticket: IR-277. Eight new or edited Behavior bullets.

These conform to EARS:

- the stream-read bullet (ubiquitous);
- the over-limit refusal (`If ... then the generator shall`);
- the capture-failure refusal (`If ... then`);
- the group-cleanup bullet (ubiquitous);
- the batch refusal bullet (`If ... then`).

The batch launch bullet is `Where ... shall` but compound. The batch verdict bullet and the FR-028
batch bullet stray from EARS. Every bullet names the generator as its subject. Each one's planned
marker is in its text.

The stable codes `kani_output_over_limit` and `kani_output_unread` fit the repo's snake_case
`kani_*` code vocabulary (`kani_witness_*`, `kani_corpus_*`, `kani_vacuous_proof`).
`KaniExecutionRefusal` currently has typed variants and no string codes. The code PR has to add a
code to that enum, which is a design note, not a spec defect.

## Verdict

Wording only. Low findings. None blocks merging on its own.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The FR-028 batch bullet mixes three things: a normative EARS rule; a "Basis:" rationale paragraph; and "The owner may choose differently, for example a batch size limit or a retry of members singly", which is an open design question. Rationale and owner options belong in a note or an open-questions section, not in a `shall` bullet. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:55-66 |
| FND-002 | low | The batch verdict bullet is not EARS. It puts a conditional verdict rule ("verified only when ... otherwise it is inconclusive") inside descriptive prose. It should be an `If <member success> and <unsuccessful exit> and <no member failed check>, then the generator shall classify ... inconclusive` statement. The batch launch bullet is compound: three `shall` clauses plus a definition ("A batch of one is the single run above"). Split it. | spec/kani/functional/FR-017-kani-execution-evidence.md:105-118 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed a0eba86 | FR-028's batch rules are now four EARS bullets (two `Where`, two `If ... then`). The Basis paragraph and the owner's options moved to a `## Rationale` section. |
| FND-002 | fixed a0eba86 | The verdict rule is now "If a member's entry states success, the batch process exited non-zero and no member's entry states failure, then the generator shall classify that member inconclusive ...". The launch bullet is split into separate grouping, argument-vector, outer-bound, matching, entry, playback and evidence bullets. |
