---
id: SR-7202
title: IR-746 candidate specification base review
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@952e81e5d694f8ec3e2c6ae320b56a45d178ef02; spec/assurance/{AD-003,AD-004}, spec/core/functional/interface-001, spec/core/matrix/{TC-042,tests}, spec/core/non-functional/NFR-005, spec/kani/matrix/TC-023"
---

## Summary

Pre-PR base specification review of all seven changed spec files at the exact local head above. Ticket: IR-746. The revised panic criterion retains a contradictory sentence from the removed exception.

## Examined Units

- AD-003 evidence chain row 4 (examined): corpus identity now hashes canonical bytes and refuses an unencodable preimage.
- AD-004 L-9 (examined): canonical artifact bytes, limits, and single encoding boundary.
- interface-001 generate_bounded_kani_corpus_case operation (examined): typed identity and graph refusal semantics.
- TC-042 step 1 (examined): the digest scan finds zero panic tokens.
- spec/core/matrix/tests.md NFR-005 and TC-042 rows (examined): the dated exception is removed.
- NFR-005-AC-1 (examined): zero panic tokens, but an old exact-one-expect requirement remains in its final sentence.
- TC-023 corpus identity paragraph (examined): separate canonical preimage and artifact limits, with typed refusals.

## Verdict

**FAIL.** NFR-005-AC-1 cannot be satisfied as written: it requires both no `expect` and exactly one `expect` in the same digest body.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | NFR-005-AC-1 simultaneously requires zero panic tokens and exactly one `expect` in `CaseIdentity::digest`; remove the stale exact-one clause. | spec/core/non-functional/NFR-005-no-generation-panics.md:161 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 75c46eddec8aef7ae1f5bd2c534cc5b91e6077dc |

After excerpt (`spec/core/non-functional/NFR-005-no-generation-panics.md:161`): `the three files FR-014-AC-39, FR-018-AC-19 and FR-021-AC-21 name, and at least 60 files. | Test (TC-042) |`. The exact-one-`expect` clause is gone and the zero-token criterion agrees with TC-042.
