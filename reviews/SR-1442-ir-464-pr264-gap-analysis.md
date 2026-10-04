---
id: "SR-1442"
title: "IR-464 gap analysis: FR-015-AC-7 and AC-53 to AC-58 against the code and tests of PR #264"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@20b3dff4f65ed8d57e51d4d7d80654cce60c1064; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md, src/kani/generate/, tests/it/"
relationships: []
---

# SR-1442: IR-464 gap analysis (PR #264)

## Summary

Ticket: IR-464. PR: agent-ix/quire-contract-codegen#264, head 20b3dff, merge base 0765cc3. The
spec was merged in #262. This PR flips FR-015-AC-53 to AC-58 from planned to implemented, and
widens the FR-015-AC-7 matrix row to all seven harness kinds.

## Method

For each AC, I traced the test that backs it and checked that the test fails without the code
(mutation results are in SR-1441).

- **FR-015-AC-7** (one cover, the last statement, in every harness).
  `tc_025_every_emitted_harness_ends_with_exactly_one_cover` drives all 11 families across the
  seven kinds. It fails when the V1 cover is removed, when the corpus cover is removed, when the
  cover is moved before the assert, when there are two covers, and when the cover is inside an
  `if`. Backed.
- **AC-53** (the seven kinds, and what each cover witnesses). The same inspection backs it. What
  each cover witnesses for the five older kinds is unchanged in the templates. Backed.
- **AC-54** (the V1 bundle cover follows the contract call). Backed by the inspection, and by
  `tc_025_real_kani...`, whose cover-stripped arm classifies `MissingCoverSummary`.
- **AC-55** (the corpus cover follows its assert, in all three families). Backed by
  `tc_023_every_corpus_family...` and `tc_023_kani_reads_a_corpus_harness_without_its_cover...`.
- **AC-56** (V1 bundle `Verified`, and `CoverUnsatisfied` for an unsatisfiable requires). I ran
  `tc_025_real_kani_classifies_the_v1_bundle_verified_vacuous_and_falsified`: `Verified`,
  `CoverUnsatisfied { satisfied: 0, total: 1 }`, and in addition `MissingCoverSummary` and
  `Falsified`. It passes. Backed.
- **AC-57** (corpus `Verified` per family; false graph and collection `Falsified` with an
  empty-valued playback). I ran the arithmetic, graph and true-collection tests (`Verified`) and
  the false collection and false graph tests (`Falsified`, `vec![]` playback). They pass. Backed.
- **AC-58** (the inspection of every entry point, plus the scan). The inspection half is backed.
  The scan half has a gap; see FND-001.
- Coverage, measured myself with `quire coverage --json` at base and head:
  `unbacked_rows` is 65 at both, and the sets are identical; backed rows go from 246 to 252
  (AC-53 to AC-58). `status_lies` is 0 at both.
- The non-Kani gate, `make ci`, is recorded in SR-1441.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-58 says the scan fails when an unlisted file "emits either proof attribute". The scan only matches the substring `kani::proof` in one literal. A new file that builds the attribute with `format!("#[kani::{}]", "proof")`, `concat!("#[kani::", "proof]")` or a `\` line continuation passes (reproduced, all three). So does an undriven emitter added to a file already in the driven set (reproduced in `frame.rs`), since the check is per file. The risk at run time is low: a harness with no cover classifies `MissingCoverSummary`, never `Verified` (FR-017-AC-4). But the AC claims more than the scan checks. Either narrow the AC to "spells", or make the scan also flag `proof_for_contract` and any `kani::` literal in an unlisted file. | tests/it/cover_last.rs:186-205 |

## Verdict

Every AC claimed implemented has a test that fails without the code, and the real-Kani outcomes
match the ACs. Coverage does not regress: 65 unbacked rows before and after, the same set. The
one gap is low: the AC-58 scan matches a spelling, while the AC claims it catches any emitter.
That AC is a backstop, and the classifier already refuses `Verified` for a harness with no cover.

## Dispositions

Round 1, checked at 86714933f2f9907cbf76dfa8efa4b58ba27110fa. Every mutation ran in a separate worktree with its own target dir.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 86714933f2f9907cbf76dfa8efa4b58ba27110fa. The scan now counts spellings per literal, with `\` continuations joined, against the pinned `PROOF_TEMPLATES`. Each of my four escapes now fails `tc_025_no_proof_attribute_is_spelled_outside_what_the_inspection_drives`: `format!("#[kani::{}]", "proof")`, `concat!("#[kani::", "proof]")`, a `\` continuation, and an undriven template in frame.rs (frame.rs 3 against a pinned 2). AC-58, the FR-015 Behavior bullet and TC-025 item 18 now state what the scan guarantees and what it misses. The stated miss, `concat!("#[kani", "::proof]")`, passes, as the text says. |

Round 1 notes, which are not findings:
- The narrowed text is accurate.
- The scan errs on the safe side. These legitimate edits fail it: a real non-proof Kani attribute outside the four exemptions, such as `#[kani::solver(cadical)]`, added to a driven template (precondition.rs 2 against a pinned 1); and a non-test message string that mentions `kani::proof` in an undriven file (clause.rs flagged).
- That matches AC-58's stated definition, and the assert message names the file and the count. Each such case costs a one-line exemption or a recount, not a silent escape.
- An emitter with no literal spelling at all (`quote!`, or a const reused from a driven file) is outside the AC's stated scope of what a literal spells.
