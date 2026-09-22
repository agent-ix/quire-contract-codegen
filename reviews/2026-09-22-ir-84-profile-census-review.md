---
id: SR-032
title: "IR-84 bounded Kani profile census review"
type: SpecReview
analysis: code-review
scope: "Working-tree diff on branch peter/ir-84-codegen60-defect-bounded-kani-profile-classification-returns (base origin/main f716300); src/bounded_kani_profile.rs only"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-007
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-023
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/AP-001
    type: references
---
# SR-032: IR-84 bounded Kani profile census review

## Summary

One independent review of the uncommitted IR-84 change to `src/bounded_kani_profile.rs` (85
insertions, 40 deletions, no other file touched), covering code review, Rust review and gap
analysis. The change removes a wrapper loop that walked the upstream disposition census and
returned a single `Err(KaniOutcome)` on the first `Refused` or `Inconclusive` entry, discarding
every other construct's disposition. The core claim was verified against the pinned upstream
source: `quire_contract_ir::kani::KaniProfile::classify` (rev `97f5065`,
`src/kani/profile.rs:104-133`) already returns one `CapabilityEntry` per requested construct, in
request order, and reserves `Err` for an empty or duplicated construct name
(`kani_capability_request_invalid`) or a construct absent from the matrix
(`kani_capability_missing`). The deletion is correct and the defect was real.

The residual issues are about what the new contract is pinned by, not about the deletion. The
replacement test passes against a `classify` that ignores its `constructs` argument entirely, the
module header still describes the deleted gating behavior, and the narrowed `Err` meaning is
documented in prose without naming the two codes a caller would match on.

## Verdict

**CONDITIONAL** — the deletion is correct and the gates are green; three medium findings concern
the strength of the new test and the accuracy of the surrounding documentation.

## Assurance Context

`spec/assurance/AP-001-codegen-release.md` (AP-001, profile_version 0.2, status proposed) applies:
its `review_policy` requires `code-review` and `gap-analysis`, both performed here, and its impact
scenarios name "silent construct loss" and "pass/rejection conflation" — exactly the defect IR-84
closes and the residual ambiguity FND-003 records. Evaluated source: working tree of
`/Users/peter/dev/quire-contract-codegen/worktrees/ir-84-profile-census`, uncommitted, based on
`origin/main` f716300; pinned upstream `quire-contract-ir` rev
`97f50655c48f48be0bf5ab04dff5aac029c3745c` per `Cargo.toml` and `Cargo.lock`.

Unavailable context: no shared-assurance producer output was regenerated for this review, so no
`make assurance-inputs` / `assurance-chain` evidence was examined; `make test`'s assurance-inputs
prerequisite was not run and the integration lane was not executed. Impact-assessment,
independence and producer-reliance evidence beyond the profile document itself was not examined.
Active exceptions: none — AP-001 records no standing exceptions and none was claimed here.

Gates run verbatim:

- `cargo fmt --check` — exit 0, no output.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` — clean.
- `cargo test --locked --lib` — `45 passed; 0 failed`.
- `cargo deny` and the integration/conformance lanes — not run.

## Findings

| ID      | Severity | Summary                                                                                          | Refs                             |
| ------- | -------- | ------------------------------------------------------------------------------------------------ | -------------------------------- |
| FND-001 | medium   | New TC-023 test passes against a `classify` that ignores `constructs` and returns the whole matrix | src/bounded_kani_profile.rs:89-152 |
| FND-002 | medium   | Module header still claims the deleted gating behavior ("returns typed non-Boolean outcomes before a generator can render an artifact") | src/bounded_kani_profile.rs:3-4  |
| FND-003 | medium   | Narrowed `Err` contract is prose-only; both malformed-request outcomes still carry `KaniOutcomeKind::Refused`, and the doc names no code a caller can match | src/bounded_kani_profile.rs:31-33 |
| FND-004 | low      | The `Err` branch the doc now defines as the sole `Err` meaning has no test in this repo           | src/bounded_kani_profile.rs:53-152 |
| FND-005 | low      | `BoundedKaniProfile::classify` is now pure delegation; the seam has no production caller at all   | src/bounded_kani_profile.rs:35-41, src/lib.rs:57 |
| FND-006 | low      | `classify_bounded_kani_profile` takes `KaniProfile` by value but only borrows it                  | src/bounded_kani_profile.rs:44-51 |

### FND-001 — the replacement test does not pin request-driven classification

Measured, not inferred. Two mutations were applied to the working tree and reverted:

1. Reintroducing the deleted early-return loop verbatim → the test **fails** at
   `src/bounded_kani_profile.rs:139` with
   `KaniOutcome { kind: Refused, code: "unsupported", ... }`. The regression guard against the
   original defect is genuine and non-trivial.
2. Replacing the body with `let _ = (constructs, source_id); Ok(self.profile.capabilities.clone())`
   → the test **passes**.

Mutation 2 passes because the fixture's capability matrix is element-for-element identical to the
request, in the same order. The test therefore asserts "the returned vector equals the matrix" and
cannot distinguish that from "the returned vector answers the request". It pins neither request
order (versus matrix order) nor the fact that unrequested constructs are excluded — both of which
the new doc comment claims ("complete ordered disposition census", `Err` for an absent construct).

Concrete fix: give the fixture matrix at least one construct the request omits, list the matrix in
a different order from the request, and assert the census follows the request.

### FND-002 — stale module header

`src/bounded_kani_profile.rs:3-4` still reads: "This boundary consumes Contract IR's matrix without
re-describing semantic families and returns typed non-Boolean outcomes before a generator can
render an artifact." After this change the module returns a census and gates nothing; the second
half of that sentence describes the behavior the diff deleted. The method-level doc was rewritten
carefully; the `//!` header two lines above it was not. A doc comment describing an intention the
code does not implement is a finding in its own right, and here it directly contradicts the new
method doc on the same screen.

### FND-003 — `Err` narrowed in prose, not in the type

The doc now states that `Err` is "a malformed request, not a disposition". That distinction is
sound as a concept — the two upstream `Err` paths (`src/kani/profile.rs:110-127`) are request
validity checks, not matrix dispositions — but it is not representable in the returned value. Both
malformed-request outcomes are constructed as `KaniOutcome::non_success(KaniOutcomeKind::Refused,
...)`, so a caller that matches on `KaniOutcomeKind` sees `Refused` for a malformed request exactly
as it previously saw `Refused` for a refused construct. The only discriminator is the code string,
and the doc names neither. AP-001 lists "pass/rejection conflation" as a material impact scenario,
which is what a caller reading `Refused` off a malformed request and reporting a construct refusal
would produce.

Concrete fix: name the two codes in the doc comment —
`kani_capability_request_invalid` (empty or duplicated construct name) and
`kani_capability_missing` (construct absent from the matrix) — and state explicitly that both
arrive with `KaniOutcomeKind::Refused`, so a caller discriminates on `code`, not on `kind`.

Noted but out of scope for this diff: upstream passes `construct.clone()` as the `context`
argument of the `kani_capability_missing` outcome (`src/kani/profile.rs:125`) where every other
construction site passes `selection.revision`. That is an upstream inconsistency in
`quire-contract-ir`, not a defect introduced here.

### FND-004 — the new sole `Err` meaning is untested here

With the loop gone, every `Err` this function can return comes from upstream request validation.
Nothing in this repo asserts that an empty name, a duplicate name, or an absent construct still
produces `Err` rather than a partial `Ok`. That is precisely the contract the new doc comment
introduces, and it is the branch most likely to be misread by a future caller. One additional test
covering the absent-construct case would also close FND-001's blind spot.

### FND-005 — the seam has no production caller

`BoundedKaniProfile::classify` now forwards to `self.profile.classify` with no added behavior, and
a repo-wide search confirms the only consumers of `classify_bounded_kani_profile` /
`BoundedKaniProfile` are `src/lib.rs:57` (the `pub use` re-export) and this file's own unit test.
The caller search was verified beyond a plain grep: there is no trait implementation, no dynamic
dispatch, and no macro expansion reaching these items — they are inherent methods on a concrete
struct plus one free function, the crate is `publish = false`, and no other module, integration
test, or example calls `.classify(` on a `KaniProfile` at all. The generation path,
`generate_bounded_kani_corpus_case` (`src/bounded_kani_corpus.rs:109-230`), compares
`profile.selection` against the input and then lowers; it never consults the capability matrix.

This is pre-existing and is not introduced by the diff, but it bounds what IR-84 achieves:
FR-007-AC-1's "one exact profile disposition before lowering" is now correctly *computed* at a seam
that no generator calls, so the census still gates nothing. Worth a follow-up ticket rather than a
change to this diff.

### FND-006 — ownership at the signature

`classify_bounded_kani_profile` takes `profile: KaniProfile` by value, wraps it, calls a `&self`
method, and drops it. `&KaniProfile` would express the same operation without the move. Pre-existing
and cosmetic; listed for completeness.

## Spec impact

`spec/functional/FR-007-bounded-kani-profile-corpus.md` needs no edit. FR-007-AC-1 is the
requirement this change satisfies, and the Behavior clauses it touches ("select exactly one
supported, refused, or inconclusive disposition for every encountered construct"; "Where a
construct is refused or inconclusive, the generator shall preserve its source identity and
disposition") describe the post-change behavior more accurately than the pre-change behavior. The
old early-return was the deviation, not the spec.

`spec/test-matrix.md` needs no edit: it maps FR-007 and TC-023 by requirement id, not by test
function name, and both rows remain accurate. The renamed test carries the repo's
`/// Trace: FR-007-AC-1, FR-007-AC-3, TC-023.` tag and the `tc_023_` prefix, and no artifact
anywhere in the tree referenced the old function name
`tc_023_profile_refusal_and_inconclusive_remain_non_boolean`.

One trace note: the deleted test asserted `outcome.boolean_claim() == None`, and the replacement
asserts no equivalent. FR-007-AC-3's "typed non-Boolean result" is still traced here by the
`Refused` / `Inconclusive` dispositions being asserted exactly, and the stronger no-Boolean-claim
assertion survives in
`bounded_kani_corpus::tests::tc_023_non_success_emits_no_partial_artifacts_or_boolean_claim`, so
the AC is not left uncovered.

## Coverage

Gap-analysis rollup, scoped to the IR-84 change rather than to the whole v0.1 plan.

- **Plan completion.** No task in `plan/PLAN-001-codegen-v01/` owns FR-007 or IR-84; the change is
  a defect fix with no plan task to close. Task-005, Task-006 (`in_progress`) and Task-007
  (`not_started`) are the wider v0.1 backlog and are unrelated to this diff, so they do not force a
  FAIL here.
- **Matrix verification.** `spec/test-matrix.md:21` (FR-007 → TC-023) and `:295` (TC-023 → all six
  ACs) are both backed by tagged tests: fourteen `tc_023_*` functions across
  `src/bounded_kani_corpus.rs`, `src/bounded_collections.rs`, `src/definedness_arithmetic.rs`,
  `src/bounded_kani_replay.rs`, `src/bounded_kani_profile.rs` and
  `tests/it/oracle_generation.rs`, each carrying a `/// Trace:` tag. No matrix row is left unbacked
  by this change, and the renamed test keeps its tag and `tc_023_` prefix.
- **Underspecified code.** No code in the diff lacks an owning requirement; FR-007-AC-1 owns it
  directly. The reverse observation is FND-005 — a specified seam with no production caller, so the
  requirement is computed but not enforced anywhere.
- **Semantic review.** Performed for the single requirement↔test↔code triple in scope
  (FR-007-AC-1 ↔ `tc_023_census_reports_every_construct_including_ones_after_an_early_refusal` ↔
  `BoundedKaniProfile::classify`) by mutation, which is how FND-001 was established. Not fanned out
  across the rest of FR-007's ACs.

## Duplication

No vendoring and no new duplication. The change is net-negative on duplication: the deleted loop
was a second, local statement of a classification rule that the upstream matrix already owns, and
removing it leaves one home for the rule. The added `entry()` test helper is local to this module's
test mod; no shared capability-entry fixture exists in the repo to reuse.
