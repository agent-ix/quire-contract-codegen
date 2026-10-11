---
id: SR-7689
title: IR-746 final-head gap analysis
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@8f7ec72354ed453e845ac2085d6538ddfc6ac42a; PR #356 canonical corpus identity and TC-023 coverage"
---

## Summary

Read-only acceptance-criterion and test-oracle check for IR-746 PR #356. The fixed hand-authored 635-byte canonical preimage and digest still call the production emitter and therefore preserve the prior SR-7201 correction. The new owner-serializer normalization boundary lacks a positive size test.

## Examined Units

- AD-004 L-9 (examined): one canonical encoder and digest owner.
- FR-015-AC-93 (examined): typed refusal before artifact or identity claim.
- FR-015-AC-94 (examined): normalized finite input affects identity.
- TC-023 (examined): canonical case-name preimage and 1 MiB limit.

## Verdict

**CONDITIONAL.** One TC-023 boundary case is untested and currently falsely refused. Prior SR-7201 digest-oracle finding remains fixed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No positive boundary control covers a verbose owning `FiniteInput` above 1 MiB whose normalized case preimage is below 1 MiB; the new preflight rejects such a valid case. | src/kani/generate/corpus/bounded_kani_corpus.rs:296; src/kani/generate/corpus/bounded_kani_corpus.rs:1659 |

## Coverage

The existing `tc_023_unencodable_case_identity_refuses_without_claiming` covers an oversized final request preimage. The fixed hand-authored digest test covers ordinary-size canonical identity. Neither covers the intermediate owner projection versus final normalized preimage ceiling. See SR-7688 for a 1,054,769-byte versus 407,601-byte read-only witness.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 8ba2c8b5e450504192e459d8fdc00a70893f1170: `tc_023_compact_case_identity_admits_a_verbose_owner_input` asserts the old failing owner-size boundary, the normalized-size boundary, and successful public `emit`; the existing over-final-limit negative remains. |
