---
id: "SR-644"
title: "CG PR 207 spec review: TC-023 corpus identity text and the corpus proof-graph schema"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@68caecc4bcd01a0049802275f06f8d5613475461; spec/test/TC-023-bounded-kani-profile-corpus.md, schemas/kani-corpus-proof-graph-v1.schema.json"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/TC-023
    type: reviews
---

# SR-644: CG PR 207 spec review

## Summary

Ticket: IR-462. PR: agent-ix/quire-contract-codegen#207, head 68caecc. This is a base-checklist
spec review of the one edited TC-023 sentence and the two schema pattern changes. No requirement,
AC or matrix row is edited.

## Method

I diffed TC-023 and the schema against main 8fcb51f and compared the text with
`src/bounded_kani_corpus.rs` at the head. I checked the schema patterns against the
`ByteDigest` `LowerHex` output and grepped the repo for other data in the old `_[0-9]+` shape.
`make spec` (quire validate) ran inside `make ci` and passed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The TC-023 sentence still opens "carries its corpus case's readable name". It then defines that name as the family label plus a 64-hex SHA-256, which is not readable. It also calls the hashed content "canonical", but the finite input is hashed in the order supplied (SR-642 FND-002). The same paragraph lists the hashed parts as request, finite input, profile selection and census, and leaves out `construct`, which is also hashed. Suggested wording: "carries its corpus case's name: the family label plus the SHA-256 of the case's request content (construct, request, finite input as supplied, profile selection and normalized census)". | spec/test/TC-023-bounded-kani-profile-corpus.md:38-39 |

## Verdict

Mergeable. The only finding is low.

Correct:
- Both schema patterns, `proofId` and `identity`, are anchored and accept exactly
  `(arithmetic|graph|collection)_[0-9a-f]{64}`, with the `corpus_case_` prefix on `proofId`. They
  reject uppercase hex, short digests and the old `_[0-9]+` names. That matches `ByteDigest`'s
  lowercase 64-digit `LowerHex`. The two positive schema tests in `tests/it` pass against
  digest-named graphs. No other data in the repo uses the old shape. The "positional counter" in
  `tests/it/exact_scalar_agreement.rs` belongs to an unrelated generator.
- interface-001 already names `kani_corpus_identity_collision`, so this PR makes the code match an
  existing interface clause rather than adding a new one.
- The TC-023 matrix row stays Planned and no AC claims corpus identity, so nothing is over-claimed.
