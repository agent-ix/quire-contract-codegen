---
id: SR-17940486709564847047905
title: "IR-948 PR #815 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #815 at 4c909c4a, diff against merge-base e1e3ec92; IR-948; FR-015-AC-39, FR-015-AC-43, FR-015-AC-126, FR-015-AC-129, FR-025-AC-8; src/kani/generate/clause/v2.rs, src/replay/witness.rs, tests/it/kani_obligations_v2_profiles.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-025
    type: reviews
---

## Summary

Ticket: IR-948. Plan completion: not assessed. Computed matrix (`quoin matrix --json` at
4c909c4a, quoin 0.29.1): FR-015-AC-43 and FR-015-AC-39 gain the default-lane binder
`the_state_reading_postcondition_yields_a_harness_with_requires_and_a_mut_subject`.
FR-015-AC-126 and FR-015-AC-129 gain the two ignored Kani binders
`tc_025_real_kani_verifies_the_profile_postcondition_with_its_cover_satisfied` and
`tc_025_real_kani_falsifies_the_under_debiting_subject_with_in_domain_native_playback`.
FR-025-AC-8 is bound for the first time (static `tagged`, "no run evidence"). No new production
code lacks an owning criterion: the renderer, `requires` identity member, decode change and
negotiate change are each owned by FR-015-AC-39/43 or FR-016's decode criteria. No stubs. The
default-lane assertions are substantive, not tautological. The executed verdicts were reproduced
(see the code-review artifact).

Examined: FR-015-AC-39 (scalar state postcondition draws: binder checks one state field and one
parameter, order and copy; clean), FR-015-AC-43 (requires and identity: binder checks
`requires.len() == 1` and the assume text; refusal arm unbound, FND-001), FR-015-AC-126 and
FR-015-AC-129 (healthy and mutant postcondition executed members: reproduced; clean), FR-025-AC-8
(`&mut` to a harness-owned copy: binder checks copy-before-call and the `&mut` call; trace tag,
FND-002).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | FR-015-AC-43's new PARTIAL text claims "a sibling holding integer arithmetic, or one the native expression cannot render, refuses naming it (`V2ClauseSiblingRequiresNotRendered`)" for the state path, but no test reaches that arm of `read_state_postcondition`. The only `V2ClauseSiblingRequiresNotRendered` binder is the parameter-only `scalar_postcondition_distinguishes_unsupported_from_unrendered_siblings`, which goes through `negotiate.rs`. The other state-path refusals are also unbound: a sibling reading `pre(...)`, a parameter-domain mismatch, a Boolean state field, a non-integer parameter, and two fields of one name. Add a tagged control per claimed refusal (at least the arithmetic sibling and the `pre(...)` sibling), each checking the named precondition and that no harness is emitted. | src/kani/generate/clause/v2.rs:436-447, src/kani/generate/clause/v2.rs:509-519 |
| FND-002 | low | The new test binds FR-025-AC-8 under `TC-025`, but FR-025-AC-8's verification method is `Test (TC-036)`, and `spec/kani/matrix/tests.md` still lists FR-025-AC-2 through FR-025-AC-8 as Planned under TC-036. Tag the binder `TC-036` for that criterion, or record why TC-025 covers it, and update the matrix index status. | tests/it/kani_obligations_v2_profiles.rs:122, spec/kani/functional/FR-025-generated-subject-abi.md:184, spec/kani/matrix/tests.md:83 |

## Verdict

Coverage of the executed members is real and reproduced. FND-001 is the one coverage gap that
matters: a refusal the amended spec states as delivered has no test. FND-002 is trace hygiene.
