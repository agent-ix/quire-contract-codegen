---
id: SR-17940486709564847047904
title: "IR-948 PR #815 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #815 at 4c909c4a, diff against merge-base e1e3ec92 of code/ir-948-v2-profiles; IR-948; src/kani/generate/clause/v2.rs, src/kani/generate/negotiate.rs, src/kani/generate/spec.rs, src/kani/generate/collection.rs, src/kani/identity.rs, src/kani/run/execute.rs, src/replay/witness.rs, tests/it/kani_obligations_v2_profiles.rs, docs/qualification/ir948-kani-profiles.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-025
    type: reviews
---

## Summary

Ticket: IR-948. Slice 2b adds a state-reading scalar V2 postcondition renderer
(`read_state_postcondition` / `render_state_postcondition` in `src/kani/generate/clause/v2.rs`),
a `requires` member on `V2ClauseObligationIdentity` (skipped when empty, so existing records stay
byte-stable), a generalised `native_term` keyed on `(side, name)` bindings, a relaxed
`ProducedResultDomainAssert` ordering check, the negotiate change that no longer refuses a scalar
postcondition whose lowering renders its `requires`, a state-fields-then-parameters decode in
`decode_v2_clause_falsification`, and two installed-Kani tests plus one default-lane test.

Reviewer reproduction at 4c909c4a (worktree `CARGO_TARGET_DIR=<worktree>/target`, every cargo
step under `flock /tmp/cargo-slot.lock`, Kani runs one at a time in
`systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0`, foreground,
`QUIRE_CVC5_BIN` set as briefed):

- healthy postcondition: `Verified`, 2433 of 2459 checks, `two_phase=false`, 65.7 s,
  4,088,127,488 B (a first run that also compiled the crate inside the same scope read 70.3 s /
  10.2 GB, the compile inflating `memory.peak`; the clean rerun is the comparable figure);
- under-debiting mutant: `Falsified`, `two_phase=true`, 2433 of 2460, 240.4 s,
  19,842,592,768 B; the test's playback parse, in-domain decode and native check pass.

Both match the record (65.8 s / 4.1 GB; 239.9 s / 19.7 GB), so the recorded verdicts correspond
to the committed generator. Default lane: `kani_obligations_v2_profiles`,
`kani_obligations_state_frame`, `kani_obligations_state_clause_replay` and
`kani_collection_operation` 149 passed / 14 ignored; lib `kani::generate` 122 passed;
`cargo fmt --check` clean; `cargo clippy --all-targets -- -D warnings` clean.

Soundness walk of the renderer: the harness draws one pre-state per field (union of the
postcondition's bare and `pre(...)` reads and the siblings' reads, keyed by field name and
declaring node, same-name-different-node refused), then each declared parameter; it assumes each
sibling as a native expression with the sibling's bare state reads bound to the pre-state; it
copies each pre-state into `post_<field>`, calls the subject with `&mut post_<field>, ...,
<parameter>, ...`, asserts every produced field's declared domain (never assumed), then computes
`expected` (pinned after the call by `domain < expected` in the default-lane test), feeds the
oracle the pre copy for `pre_` inputs and the produced value for `current_` inputs, and asserts
domain preservation, oracle-equals-expectation and clause-holds before one final cover. The
`requires` cannot be vacuous: the final cover (after the call, over the oracle's `Completed`) is
satisfied in the healthy run, so the assumptions and bounds are jointly satisfiable. The produced
state is asserted, not assumed (checked by the default-lane test and by reading the renderer).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | The stack is not on its base: #815's merge-base is e1e3ec92, but `code/ir-948-v2-profiles` is at 8a1ed73f with #812's fix round (4630b054). `git merge-tree origin/code/ir-948-v2-profiles HEAD` conflicts in `tests/it/kani_obligations_v2_profiles.rs`, `docs/qualification/ir948-kani-profiles.md` and FR-015. The two new Kani tests use the Debug-string `cover_satisfied` and no `assert_within_cap`, the exact helpers #812 FND-001 / FND-004 replaced, and #812 also changed `Twin::handler` in `native_twin.rs`. Rebase onto 8a1ed73f, move the new tests to `named_cover_satisfied` and `assert_within_cap`, and rerun both postcondition verdicts on the rebased head. | tests/it/kani_obligations_v2_profiles.rs:226-232, tests/it/kani_obligations_v2_profiles.rs:367-427 |
| FND-002 | medium | `decode_v2_clause_falsification` now decodes `state_fields` then `arguments` for every scalar V2 clause ("A harness draws its state fields, then its parameters"). That is true of `render_state_postcondition` but not of `render_clause`, which draws in oracle-descriptor order: `ClauseInput` sorts by source name, so the profile precondition `amount <= self.balance` draws `parameter_amount` before `current_balance`. A falsified mixed precondition harness would decode with names swapped (silently when the domains coincide, as here) or with a false `cg_witness_out_of_domain`. Decode in the harness's real draw order (record it, or decode from the rendered `Input` order) and add a decode test over a mixed precondition. | src/replay/witness.rs:206-220, src/kani/generate/clause/v2.rs:1022-1071 |
| FND-003 | medium | The sibling `requires` is a native rendering of the sibling's condition (`native_term`) that no harness checks against the sibling's oracle. The precondition harness only covers; it has no native expectation. A `native_term` mis-rendering of a sibling silently changes the assumed input space, and an over-strong one can hide a postcondition defect behind `Verified`. The `oracle_clause` collision argument holds only for the flat module: the oracle's `oracle_clause`, `value_N` and `ClauseStop` items would collide, but a nested `mod requires_<n>` per sibling oracle would not. Either assume the sibling's embedded oracle (`Completed(true)`) in a nested module, or add `assert(native == oracle)` for the sibling in its own precondition harness and record that dependency, so the native `requires` is cross-checked. | src/kani/generate/clause/v2.rs:509-523, src/kani/generate/clause/v2.rs:1529-1533 |
| FND-004 | low | The default-lane test pins draw, requires, copy, call, domain assert, `expected` and oracle order, but not the oracle input wiring (`pre_balance` from `pre_balance_current`, `current_balance` from `post_balance_current`) or that the native expectation reads the produced value for bare reads. A pre/post swap would only show up in the 66 s ignored Kani run. Assert the two `nx::NativeInteger::new(i128::from(...))` bindings and the post variable in `expected`. | tests/it/kani_obligations_v2_profiles.rs:124-167, src/kani/generate/clause/v2.rs:525-550 |
| FND-005 | low | The mutant test destructures the decoded values by position and never checks their names (`pre_balance_current`, `parameter_amount_current`), so it would not catch FND-002-class ordering errors. Its "native evaluation" recomputes the mutant subject in the test (`balance - (amount - 1).max(0)`) instead of evaluating the postcondition over a produced state. Assert the decoded names, and state in the record that the produced value is the test's restatement of the subject. | tests/it/kani_obligations_v2_profiles.rs:417-432 |

## Verdict

NOT mergeable as is. FND-001 blocks: the PR conflicts with its fixed base and reintroduces two
helpers #812 already replaced; after the rebase the verdicts must be rerun. The renderer itself
is sound on the profile: pre-state per field, `requires` over the pre-state, produced domains
asserted, expectation after the call, cover non-vacuous, and both executed verdicts reproduce at
4c909c4a. FND-002 is a latent wrong-label defect in the decoder this PR generalised, and FND-003
is a soundness gap in how `requires` is trusted. Both should be fixed in this PR.

Memory fragility, recorded here rather than as a finding: the mutant peaks at 19.8 GB against the
24 GiB (25.77 GB) cap, 77 % with about 6 GB of headroom, and takes 240 s. A small change to the
harness, subject or package could push it into `MemoryExhausted`, which counts as `Inconclusive`,
not as a pass. The healthy proof uses 4.1 GB.

Remaining refusals in `negotiate.rs` (question 5) still hold: a missing sibling refuses with
`V2ClauseMissingSiblingPrecondition`, an unsupported one with
`V2ClauseUnsupportedSiblingPrecondition`, and a scalar postcondition whose lowering has no
`StatePostcondition` still refuses with `V2ClauseSiblingRequiresNotRendered`. The existing
`scalar_postcondition_distinguishes_unsupported_from_unrendered_siblings` test (a parameter-only
postcondition) still exercises that last arm. The state path's own sibling refusals have no test;
gap-analysis records this.
