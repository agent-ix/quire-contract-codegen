---
id: "SR-1429"
title: "CG PR 262 spec review (base): a non-vacuity cover in every Kani harness kind"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@a5cd4bd54fece9ad071a399691d4dd106d9ade2d; spec/kani/functional/FR-015-bounded-kani-obligations.md (preamble lines 37-43, Behavior bullets 248-265, FR-015-AC-53..58 lines 323-328), spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md, spec/assurance/AD-004-cg-crate-layout.md (four edited sentences, lines 392, 426, 762-769, 1053) (diff origin/main...HEAD, merge base 83f9687)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
---

# SR-1429: CG PR 262 spec review (base)

## Summary

Ticket: IR-464. Spec-only PR. It adds FR-015-AC-53 to AC-58 (all PLANNED), four Behavior
bullets, a preamble paragraph, TC-025 items 16-18, a TC-023 paragraph, two `tests.md` rows and
the TC-023/TC-025 coverage lists, and edits four AD-004 sentences.

What I checked, and how:

- **Harness-site inventory.** I grepped `src/` for `kani::proof`, `proof_for_contract`,
  `cfg(kani)` and `kani::cover`, and for every string literal holding a proof attribute. Harnesses
  are emitted from six files: `precondition.rs`, `contract.rs` (one template that serves both
  postcondition and invariant), `scalar.rs`, `frame.rs` (two templates: state-clause and
  frame-effect), `v1_bundle.rs` (`render_kani_source`) and `corpus/bounded_kani_corpus.rs`
  (`render_artifacts`, one template for all three families). That is seven kinds and seven
  templates, not eight. `strategy/`, `evidence/`, `publication/`, `replay/` (which emits
  playback `#[test]`s, not proofs), `routed/`, `kani/generate/lower/*` and `test_support.rs`
  (gated `#[cfg(test)]` from `kani/mod.rs`) emit no proof harness. The V1 bundle and the
  corpus emit no cover. `precondition.rs`, `contract.rs` and `scalar.rs` end in the cover. On
  `origin/main` (65efb07, which now includes CG #260) the state and frame templates also end in
  it. The PR's kind list in FR-015-AC-53 and AC-58 is complete.
- **Real Kani 0.68 measurements** (scratch crate `cg262-kani-a5cd4bd`, with shapes copied from
  the generators' templates and CG's production adapter options, classified by CG's own
  `classify_kani_run` at this sha, with kind `None`):
  - corpus arithmetic with no cover (today's shape): `MissingCoverSummary`, with 2 successful
    checks.
  - corpus arithmetic / graph / collection-true with a trailing cover: `Verified` (2 / 516 / 65
    successful checks, cover 1 of 1 satisfied). So AC-57's claim holds. The ground assertion
    still counts as a successful check, so the run is not routed to `VacuousProof`.
  - corpus collection-false, with and without the trailing cover: exit 1, and `Falsified` with an
    empty-valuation playback (`concrete_vals = vec![]`) of the assertion. The cover is
    UNREACHABLE, and the classifier's Failure branch does not consult covers. It is not
    `FailedWithoutCounterexample`.
  - V1 bundle, healthy, with the cover: `Verified` (84 checks). Without the cover (today):
    `MissingCoverSummary`.
  - V1 bundle whose requires no bounded argument satisfies (requires `v > 100`, bounds
    `[-10, 10]`): `CoverUnsatisfied { satisfied: 0, total: 1 }`, with 83 successful checks, so it
    is not routed to `VacuousProof`. AC-56 holds.
  - V1 bundle whose subject violates the ensures, with the cover: `Falsified` with the ensures
    playback, and the cover UNREACHABLE. A failing `ensures` blocks the path, so AC-54's "the
    cover follows every check" is accurate.
- **Guard (AC-58).** Part 1 can be built with `syn` (the crate already depends on
  `syn = { features = ["full"] }` and the frame and scalar paths already `syn::parse_file` their
  output). The walker must descend into inline `mod` items, because the bundle and families wrap
  the harness in `#[cfg(kani)] mod`. A `kani::cover!` inside a nested block is correctly not the
  last statement. Asserts hidden behind a helper call are not visible, but none of the templates
  has one. A `proof_for_contract` harness's `ensures` is inside the call, which AC-54 states.
  Part 2 can reuse the literal scanner in `tests/it/layout.rs`. It must treat
  `kani/test_support.rs` as test code, although the gate sits in its parent module. Rejecting a
  `HarnessSpec` constructor now is justified: `src/kani/generate/spec.rs` does not exist, and
  AD-004 schedules `HarnessSpec` for step 4b. The 2026-10-01 IR-464 comment says the ticket is
  "Closed structurally by CG AD-004 step 4b". That comment is untrusted ticket text, and the code
  contradicts it. The 2026-10-04 reopen says the same.
- **Numbering.** `git log --all -S` finds FR-015-AC-53 to AC-58 only in a5cd4bd. No local or
  remote branch's FR-015 goes past AC-52, except this branch. Open PR #263 adds no FR-015 AC.
  FR-015-AC-37 on the stale `feat/ir-277` branch is untouched.
- **Merge.** CG #260 is MERGED (65efb07, head d5db692, not b2598f8). `git merge-tree origin/main
  a5cd4bd` is clean (tree 9bc6008), and so is a merge with b2598f8.
- **Gates.** `quire validate 'spec/**/*.md'` exits 0. Its only warnings are the pre-existing
  FR-017 line 152 EARS warnings, outside the diff. `quire coverage --strict`: 65 unbacked at
  a5cd4bd and 65 at origin/main. FR-015 is 31/58 here against 31/52 on main, and no 🚧 row is
  listed as unbacked. Planned rows are not counted, as the author says.

## Verdict

The measurements back the author's real-Kani claims for AC-54, AC-56 and AC-57. The site
inventory is complete. AC-58's two-part inspection can be built, and deferring the constructor to
AD-004 step 4b is correct. **Not mergeable as is**, because of one high finding: AD-004 still
orders the planned `HarnessSpec` as covers before assertions, which contradicts the PR's own
FR-015-AC-53 and AC-58. Four medium findings should also be fixed in this PR: the AC-7 status
overclaim, the AC-58 "exactly one" wording, the unstated falsified-corpus outcome, and the
undefined term "healthy" (SR-1431).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AD-004 still specifies the planned `HarnessSpec` order as "the assumptions, the subject call, the covers and the assertions, in that order", and its cover rule and L-4 place covers only "after all assumptions and the subject call". That puts covers before assertions, which contradicts FR-015-AC-53 (the cover is the last statement, after every assertion) and AC-58 (the inspection stays as the check over `HarnessSpec`-rendered text), and reintroduces the IR-451 shape at step 4b for the scalar family. The PR edits this same cover-rule bullet (line 392) but leaves the contradiction. | spec/assurance/AD-004-cg-crate-layout.md:381-392, spec/assurance/AD-004-cg-crate-layout.md:614-616, spec/kani/functional/FR-015-bounded-kani-obligations.md:323,328 |
| FND-002 | medium | FR-015-AC-7 says "Every generated harness contains exactly one non-vacuity cover" and its matrix row is `✅ Covered`, yet this PR's preamble states that the V1 bundle and corpus emit no cover. The preamble's claim that AC-53..58 "restate FR-015-AC-7 and FR-015-AC-46 for the kinds those criteria do not name" is inaccurate: AC-7 names every harness. The PR makes the overclaim textual. Fix in this PR by narrowing the AC-7 matrix row (or AC-7 itself, now that #260 has merged) to the five kinds that carry a cover, pointing the bundle and corpus at AC-53..55. | spec/kani/matrix/tests.md:15, spec/kani/functional/FR-015-bounded-kani-obligations.md:37-43,277 |
| FND-003 | medium | FR-015-AC-58's guard wording, "whose body does not end in exactly one `kani::cover!` statement with no assertion after it", can be read as only "the last statement is a cover". On that reading `cover!(a); assert!(x); cover!(b);` passes the guard, although AC-53 makes "more than one" cover, or a cover before an assertion, a violation. TC-025 item 16 states the stronger reading ("the only `kani::cover!` of the body"). AC-58 should say the body contains exactly one cover and it is the last statement. | spec/kani/functional/FR-015-bounded-kani-obligations.md:328, spec/kani/matrix/TC-025-bounded-kani-obligations.md:156-160 |
| FND-004 | medium | No criterion states the outcome of a falsified corpus case. A corpus harness has exactly one (empty) valuation, so the cover's position is what separates `Falsified` from `FailedWithoutCounterexample` (the IR-451 analogue that FR-017's #260 text describes). Measured on Kani 0.68: false oracle plus trailing cover gives exit 1, `Falsified` with an empty-valuation playback of the assertion, and the cover UNREACHABLE. The existing `tc_023_kani_falsifies_the_generated_false_collection_harness` asserts this but backs no FR-015 AC. AC-57 (and TC-023) should state it, so the placement in AC-55 has a run-level guard. | spec/kani/functional/FR-015-bounded-kani-obligations.md:327, spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md:58-62 |
| FND-005 | low | AD-004's edited cover-rule sentence says "FR-015-AC-58 guards every emitter until this constructor exists". FR-015-AC-58 says the inspection "stays as the check over the rendered text" once families render through `HarnessSpec`. Both cannot hold; AD-004 should say the inspection stays. | spec/assurance/AD-004-cg-crate-layout.md:392, spec/kani/functional/FR-015-bounded-kani-obligations.md:328 |
| FND-006 | low | Duplicate statements of one obligation. The new Behavior bullet (line 248) restates the existing bullet at line 110 ("shall end every generated harness with exactly one non-vacuity cover"). After merging onto main, FR-015-AC-53 restates AC-7's #260 last-statement rule for the five kinds AC-7 already covers. One of each pair should reference the other rather than repeat it. | spec/kani/functional/FR-015-bounded-kani-obligations.md:110,248,277,323 |

## New findings (disposition pass 1)

Re-checked at 5a9fe3e040dd83251a17540d0fe623f69d4fadba: origin/main 65efb07 (#260) merged in as
ad78d5b, plus fixes 1216b32 and 5a9fe3e.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | Leftover wording in the matrix and TC notes this PR adds. `tests.md:25` still says "`Verified` healthy, `CoverUnsatisfied` vacuous", and TC-023:60 still says "classifies a healthy case of each family", although the ACs dropped "healthy". `tests.md:25` also says "FR-015-AC-7 and FR-015-AC-46 state the cover for the kinds that already carry one". FR-015-AC-46 is the planned V2 clause harness (IR-489), which no kind emits today, so it states no cover for a kind that carries one. Use AC-56/AC-57's own wording and drop AC-46 from that sentence. | spec/kani/matrix/tests.md:25, spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md:60 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 1216b32: AD-004 now orders `HarnessSpec` as "the subject call, the assertions and the covers, in that order". The cover rule puts every cover after every assertion, as the last statement. L-4 says "after all assumptions, the subject call and every assertion, as the last statement". A grep finds no remaining covers-before-assertions ordering. |
| FND-002 | fixed | 1216b32: the AC-7 matrix row is split out and claims Covered only for the five kinds that carry a cover today (re-measured on HEAD: precondition, contract, scalar, and state/frame since #260). The preamble now says AC-7 requires a cover everywhere and AC-53..58 bring the bundle and corpus under it. |
| FND-003 | fixed | 1216b32: AC-58 now requires "exactly one `kani::cover!`, which must be the last statement, with no assertion after it and no other cover", matching TC-025 item 16. |
| FND-004 | fixed | 1216b32: AC-57 now states a false corpus case classifies `Falsified`, carrying the empty-valued playback and never `Inconclusive`. TC-023 names the existing test. That matches the round-0 measurement. |
| FND-005 | fixed | 1216b32: AD-004 now says the AC-58 inspection "stays beside this constructor once a family renders through it". |
| FND-006 | fixed | 1216b32: the duplicating "every harness" Behavior bullet is gone, and AC-53 now defers the cover-last rule to FR-015-AC-7, keeping only the per-kind witness list. |
| FND-007 | fixed | 9bebed4 (round 2): `tests.md:25` now states both bundle outcomes by their conditions ("`Verified` for a bundle whose requires some bounded argument satisfies and whose `ensures` holds for every such argument, `CoverUnsatisfied` for one whose requires no bounded argument satisfies"), and its last sentence reads "FR-015-AC-7 states the cover for the kinds that already carry one", with AC-46 gone. TC-023 now says "a case of each family whose oracle is true". This PR's diff adds no "healthy"; every remaining occurrence in `spec/` has the same count on origin/main. |
