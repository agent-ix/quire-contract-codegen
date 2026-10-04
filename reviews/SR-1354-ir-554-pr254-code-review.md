---
id: "SR-1354"
title: "CG PR 254 code review (with rust-review lane): TC-023 profile mismatch refusal and revision context tests"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@9933e043b35be8b37e2813cb54097118e998c26a; src/kani/generate/corpus/bounded_kani_corpus.rs (diff origin/main...HEAD)"
---

# SR-1354: CG PR 254 code review

## Summary

Ticket: IR-554. PR: agent-ix/quire-contract-codegen#254 at 9933e04. The diff adds two
`#[cfg(test)]` tests to `bounded_kani_corpus.rs` and changes no production line. The
`rust-review` lane is folded into this file.

What I checked:

- `cargo test --locked --lib tc_023` at the head: 25 passed, both new tests included.
- `make fmt-check` and `make spec` exit 0 at the head.
- Ten mutations, each in a throwaway copy of the checkout (not the review worktree), each run
  against every `tc_023` lib test:

| Mutation | Result |
| --- | --- |
| M1 drop the profile/input mismatch check (`if false`) | mismatch test FAILED |
| M2 mismatch context = the input's revision | mismatch test FAILED |
| M3 mismatch source id = constant | mismatch test FAILED |
| M10 mismatch kind `Refused` instead of `InvalidInput` | mismatch test FAILED |
| M4 proved context = empty | revision-context test FAILED |
| M5 counterexample context = empty | revision-context test FAILED |
| M6 `kani_corpus_dependency_invalid` context = empty | revision-context test FAILED |
| M7 `kani_corpus_identity_collision` context = empty | all passed (SR-1355 FND-001) |
| M8 `kani_corpus_serialization_failed` context = empty | all passed (path not reachable from a test) |
| M9 mismatch branch claims the case identity before refusing | all passed (FND-001) |

- Rust idiom: tests follow the module's existing `fixture_with` pattern and the `tc_NNN`
  trace convention, use `unwrap_err` / `expect` with a message, and add no `unsafe`, no
  panics outside test assertions, and no new dependency.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The "claims no identity" half of `tc_023_an_input_for_another_profile_selection_is_refused_before_any_artifact` cannot fail. The follow-up emission uses the matching input, whose `InputIdentity` carries `profile` (r1, not r2), so its digest differs from any identity the refused call could have claimed. M9, which makes the mismatch branch claim its identity before returning, passes every test. Through the public API the claim is unobservable (a valid call can never reproduce the (profile r1, input r2) identity), so the test would need to inspect the in-module `emitted` set directly, or the clause should be dropped | src/kani/generate/corpus/bounded_kani_corpus.rs:956-966, src/kani/generate/corpus/bounded_kani_corpus.rs:283-301 |

## Verdict

The mismatch refusal's kind, code, source id and context, and the proved, counterexample and
dependency-invalid contexts, are each pinned by a test that fails under mutation. One medium:
the identity half of the mismatch test is vacuous. Not mergeable until FND-001 is fixed.

## Dispositions

Round 1, reviewed at 9c587819ea33a4504902ace33bc02886f077dcab (fix commit 9c58781 over 9933e04).
On a clean target, `cargo test --locked --lib tc_023` passes 25 of 25. I re-ran the mutations in
a throwaway copy. M9 (the mismatch branch claims its identity) now fails the mismatch test.
M1-M7, M10 and M11 (collection lowering refusal context rewritten in the corpus) are each
caught. M8 (serialization failed) cannot be reached from a test. M12 (arithmetic lowering
refusal context rewritten) survives. That is acceptable: FR-015-AC-51 names "a lowering
refusal", and M11 backs it.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 9c58781: the mismatch test now asserts `emitted.0.is_empty()`; M9 fails it |
