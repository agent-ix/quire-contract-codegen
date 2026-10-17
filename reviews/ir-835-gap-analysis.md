---
id: SR-12026
title: "IR-835 gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@f48e60ed913bc7f17f5d010219ff2784479547c3; Cargo.lock, tests/it/kani_obligations.rs, tests/it/kani_obligations_state_frame.rs, spec/kani/functional/FR-015-bounded-kani-obligations.md, src/kani/generate/frame.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
---

## Summary

Ticket: IR-835, PR quire-contract-codegen#411, head f48e60e. The scope is the PR diff, the CG criterion its frame fixtures back (FR-015-AC-29), and the upstream authority for the new frame members: IR FR-040-AC-17..30 at 299a126 and QSpec FR-340/FR-342 at origin/main 2fb2473. Plan completion: not assessed.

## Method

I read each edited fixture node against FR-040 and FR-340. I then mutated the fixtures in a throwaway worktree, which has since been removed, to see which edits any test can detect. Five mutants ran:

| Mutant | Change | Result |
| --- | --- | --- |
| m1 | own-field scope `receiver` -> `every-object` | full `it`: 594 pass, survived |
| m2 | foreign-field scope `every-object` -> `receiver` | full `it`: 594 pass, survived |
| m3 | drop `deletes_scoped` in the state-frame fixture | state-frame tests panic in the package reader, killed |
| m4 | drop `deletes_scoped` in kani_obligations.rs | 9 of 32 `kani_obligations::` tests fail, killed |
| m6 | add `deletes_scoped: [{"kind": "receiver"}]` to every shape | 59 of 59 state-frame tests pass, survived |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-015-AC-29 says a frame that deletes is refused with no harness. No test gives a frame a non-empty `deletes_scoped`, so the scoped-delete half of that criterion is untested and, per SR-12025 FND-001, unimplemented. Mutant m6 (`deletes self` on every frame) survives all 59 state-frame tests. | spec/kani/functional/FR-015-bounded-kani-obligations.md:661, tests/it/kani_obligations_state_frame.rs:393-414 |
| FND-002 | low | No test depends on a field entry's `scope`. Swapping the scopes of the own and foreign entries (m1, m2) leaves the full `it` suite green, because CG ignores `scope` and IR admits both values. The fixture's scope choices are consistent with FR-340 but are not pinned by any assertion. | tests/it/kani_obligations_state_frame.rs:386-403 |
| FND-003 | low | The PR body does not cover two of the ticket's deliverables. (1) It does not report the "behaviour change from the IR-824 operand-1 check order" that the ticket asks for. (2) It reports the QSL move as a rev pair only, but 23f9f34..2a316c0 is 21 commits, including behaviour changes: frame-check record and Text comparison, reachability over relationship ends, and clause-set duplicate refusal. Neither is itemised as checked. | PR #411 body |

## Verdict

Changes requested: FND-001. FND-002 is best closed by the test that comes with the FND-001 fix. FND-003 is a PR-body fix.

What is backed:

- The ticket's failing QSpec-fixture tests pass against QSpec 2fb2473: tc_031_ac33 (4) and tc_024_ac44 (2).
- The `deletes_scoped` member is load-bearing in both fixtures (m3, m4 killed).
- The FR-040-AC-17 empty frame and the type-wide `deletes` shape match the spec text.

PR #397 overlap:

- #397 (IR-17, head 62ab2ac, base 2897d5e) edits the same Cargo.lock hunks. It moves qsl-* 9925955 -> 20b3254, quire-contract-ir/model 9be7387 -> ef4ab49 and quire-semantic-value b7d5531 -> ad6e975. It also moves quire-contract-runtime and quire-exact, which #411 does not touch.
- ef4ab49 (IR-225, #408, 2026-10-15) is an ancestor of IR-816's 6b80801 (2026-10-16), so the PR body's claim that #397's IR rev predates IR-816 holds. QSL 20b3254 is an ancestor of 2a316c0.
- Whichever PR lands second must regenerate the lock rather than take its own hunks. If #397 keeps its hunks after #411 merges, IR falls back to before IR-816, and the 52 `InvalidSemanticGraph` failures the PR body cites return.
- #397's kani_obligations.rs hunks (~2495-2650) do not overlap #411's line 530. #397 touches frame.rs only at `field_range`, not at `frame_grants`.

## New findings (disposition pass 1)

Reviewed at 86510d48b0280a9298becba1370999db5d33cf06.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | Two sentences in the PR body are stale since 86510d4. "CG ignores a field entry's `scope`" (IR-824 paragraph) is no longer true, because `frame_grants` now reads `scope`. "No scope-dependent test or mutant is possible" is also wrong: mutants that narrow the accepted set are killed (refusing `every-object` fails 43 `it` tests, refusing `receiver` fails 7). What remains true, and what the ruling covers, is that no test tells `receiver` from `every-object` (fixture swaps survive) and that the missing or unknown arm has no test. | PR #411 body |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | ba307de: a non-empty `deletes_scoped` now refuses as `UnsupportedFrameEffect::Deletes`, and FR-015-AC-29's scoped-delete half has tests in both refusal tables ("frame deletes self", "frame deletes a parameter"). The round-0 mutant m6 (`deletes self` on every shape) is now killed: 5 tests fail, and so does the parameter variant. Deleting the src line fails the 2 new tests. |
| FND-002 | accepted-no-change | Ruling by titan:kani:plan, relayed by the lead: the single-object frame harness renders `receiver` and `every-object` the same way, QSL emits `every-object` for an unscoped `Modifies: f` on the clause's own field, and so there is no refusal of either scope and no scope-swap test. Re-measured: swapping the own scope to `every-object` and the foreign scope to `receiver` still leaves all 59 state-frame tests passing. The PR body says so, apart from the wording in FND-004. |
| FND-003 | fixed | The PR body now reports IR-824 (#422): no effect observed, measured with a full `it` run and the four QSpec-fixture tests. It lists the 21 QSL commits by area (frame check, reachability, clause sets, spec or test only), and it no longer says src is untouched; it describes the `frame_grants` change. |
