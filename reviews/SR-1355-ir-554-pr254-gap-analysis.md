---
id: "SR-1355"
title: "CG PR 254 gap analysis: TC-023 mismatch refusal and revision context, spec against code and tests"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@9933e043b35be8b37e2813cb54097118e998c26a; spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, src/kani/generate/corpus/bounded_kani_corpus.rs (diff origin/main...HEAD)"
---

# SR-1355: CG PR 254 gap analysis

## Summary

Ticket: IR-554. Plan completion: not assessed. Scoped to the two sentences the PR adds to
TC-023 Expected Results and the two tests it adds.

Are the sentences true of the code? Yes.

- Mismatch: `generate_bounded_kani_corpus_case` returns `InvalidInput`
  `kani_profile_input_mismatch` with the request's source id and the profile selection's
  revision when `profile.selection() != &input.input().profile`. It returns before any
  artifact is rendered and before `emitted.claim`, so no artifact and no identity
  (bounded_kani_corpus.rs:400-407).
- Revision context: every outcome built in this function uses
  `profile.selection().revision`: proved, counterexample, dependency invalid,
  serialization failed and identity collision (lines 445-527). The `?` lowering refusals come
  from Contract IR at the locked rev cbcd790 (`kani/arithmetic.rs`, `objects.rs`,
  `collections.rs`), and every one of them also passes `profile.selection().revision`. The
  `abi.rs` refusals that take a separate `context` belong to finite-input validation, which
  happens before this function runs.

Are they backed? The mismatch sentence is, except the identity clause (SR-1354 FND-001). The
revision sentence is backed for proved, counterexample, dependency invalid and mismatch. It is
not backed for the identity collision refusal (M7 survives) or for any lowering refusal.
Serialization failed is not reachable from a test, which is acceptable.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-023 now says the revision is the context of "each refusal". No test asserts the context of `kani_corpus_identity_collision`: mutating it to an empty string passes every test (M7). No test asserts the context of a lowering refusal returned through the corpus either, such as `kani_definedness_checked_range`. One assertion on the existing collision test, and one on a lowering refusal, would back the clause | spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md:53-55, src/kani/generate/corpus/bounded_kani_corpus.rs:521-527, src/kani/generate/corpus/bounded_kani_corpus.rs:968-1021 |

## Verdict

The code matches both new sentences. One medium: the "each refusal" clause is broader than
its tests. Not mergeable until FND-001 is fixed, or the clause is narrowed to the cases tested.

## Dispositions

Round 1, reviewed at 9c587819ea33a4504902ace33bc02886f077dcab. The collision test now asserts
`context == "r1"`, so M7 fails it. The collection-lowering ResourceExhausted test now asserts
the context, so M11 fails it. The clause now names its cases explicitly (proved,
counterexample, dependency-invalid, identity-collision, a lowering refusal, the mismatch
refusal), and each named case is backed. M8 (serialization failed) is still unreachable and is
not named.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 9c58781: collision and lowering-refusal context assertions added; clause narrowed to the tested cases (FR-015-AC-51) |
