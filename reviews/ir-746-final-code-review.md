---
id: SR-7688
title: IR-746 final-head code review with Rust lane
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@8f7ec72354ed453e845ac2085d6538ddfc6ac42a; PR #356 diff, especially src/core/canonical.rs and src/kani/generate/corpus/bounded_kani_corpus.rs"
---

## Summary

Final-head read-only review of IR-746 PR #356. The earlier independent fixed digest and case-name oracle remains attached to the current `CaseIdentity::digest` path. The earlier NFR-005 contradictory wording remains corrected. Rust review checked owning serializers, canonical projection, finite bounds, typed refusals, and test oracles. No code was edited or heavy gate run.

## Examined Units

- AD-004 L-9 (examined): canonical byte encoding and hashing remain in `src/core/canonical.rs`.
- FR-015-AC-93 (examined): case identity refusal is typed and occurs before a registry claim.
- FR-015-AC-94 (examined): normalized finite input and dependencies contribute to case identity.
- TC-023 (examined): canonical case preimage has a 1 MiB ceiling and an unencodable preimage produces a typed refusal.
- NFR-005-AC-1 (examined): no panic token is required in the corpus digest path.

## Verdict

**CONDITIONAL.** The new full-owner projection applies the canonical preimage ceiling before object/reference normalization, causing a real false refusal. The prior SR-7200 and SR-7202 findings remain fixed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Pre-normalization `FiniteInput` serialization can exceed 1 MiB while the actual normalized `CaseIdentity` preimage fits, so a valid bounded arithmetic case falsely refuses as `kani_corpus_identity_unencodable`. | src/kani/generate/corpus/bounded_kani_corpus.rs:296; src/core/canonical.rs:77-85 |

## Coverage

A read-only JSON size witness with 18,500 distinct six-character finite-object identities, `max_objects=18,500`, `max_references=0`, `max_input_bytes=2,000,000`, and an arithmetic request produces a 1,054,769-byte owning `FiniteInput` projection versus a 407,601-byte normalized `CaseIdentity` projection. Contract IR's `FiniteInput::validate` accepts an owner-selected positive object bound and distinct nonblank objects; `prepare_checked_arithmetic` does not inspect the object collection. This exercises the new premature ceiling without requiring a Cargo gate. The focused boundary regression asserts that the owner projection exceeds 1 MiB, the normalized input fits the ceiling, and emission admits; the separate `tc_023_case_name_matches_hand_authored_canonical_preimage` test asserts the independently authored fixed digest and emitted name.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 8ba2c8b5e450504192e459d8fdc00a70893f1170: `owner_input_value` uses a derived 4 MiB counted ceiling only for the verbose owning input; `CaseIdentity::digest` retains the 1 MiB canonical ceiling. The 18,500-object positive control proves owner serialization exceeds 1 MiB while the normalized input fits and `emit` admits. The existing over-final-limit refusal control remains. |
