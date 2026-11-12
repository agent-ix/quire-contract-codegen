---
id: SR-17940486709564847047726
title: "IR-631 PR #803 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #803 at 37e3e1f6; IR-631; tests/it/scalar_replay_acceptance.rs, tests/it/scalar_replay_binding.rs, tests/it/scalar_replay_residuals.rs, tests/it/scalar_settlement.rs, tests/it/skeleton_spine.rs, tests/it/state_frame_obligation_identity.rs, tests/it/main.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
---

## Summary

Ticket: IR-631. The PR adds `tests/it/scalar_replay_acceptance.rs` (five tests: AC-2 setup
precedence, AC-6 single-member report binding, AC-17 QSL stale-dependency, AC-18 absent retained
kind or operand identity, AC-12 invented membership) and adds criterion tags to nine existing
tests. No production code changes.

Gates, run in a separate detached worktree at 37e3e1f6 with its own target dir through
`locked-build.sh`: `cargo fmt --check` clean; `cargo clippy --locked --test it --all-features -- -D warnings`
clean; `cargo test --locked --test it` over the new module and every newly tagged test,
15 passed.

Reviewer probes and mutations (throwaway, reverted, tree clean afterwards):

- M1: drop `report.claim() != &actual_claim` in `CompletedScalarReplay::settlement`. Killed by
  `each_single_retained_member_change_refuses_the_genuine_report` (line 221). The AC-6 test can fail.
- P1: playback with only the `// 2` comment changed to `// 12` gives
  `kani_witness_comment_mismatch`, an adapter defect, not an operand-domain defect (FND-002).
- P2: a retained record with an unrelated added key, or with only its trailing newline removed,
  gives the same `Authority(GeneratedArtifactChanged)` as the AC-18 test's removed `kind` or
  operand `child` (FND-001).
- P3: a routed `Implies` harness with an oversize or garbage playback returns
  `Authority(Unavailable)`. This is reported in the gap-analysis SR.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | The AC-18 test `absent_retained_kind_or_operand_identity_refuses_setup` cannot tell an absent member from any record edit. The generator seal (`has_valid_provenance`) rejects every change to `record`: an unrelated added key or a dropped trailing newline gives the same `GeneratedArtifactChanged`. The typed `ScalarReplayBody.kind` is non-optional and the record JSON `kind` is never read for authority. The test asserts the seal, not the absent-authority refusal its name, its doc and the AC-18 IMPLEMENTED cell claim. | tests/it/scalar_replay_acceptance.rs:298-336 |
| FND-002 | low | Two precedence assertions do not test what their comments say. (1) "The adapter's checks beat the operand domain" replaces `// 2` with `// 12`, which edits only the playback comment. The decoded value stays 2, so the input pairs two adapter defects (wrong harness plus `kani_witness_comment_mismatch`) and has no operand-domain defect. (2) Under "Each defect alone", `oversize` is built on `wrong_harness`, so it is not alone, and the "oversize and wrong harness together" assertion repeats the same input. Harness-name-before-domain precedence is untested (arity and width before domain are tested). | tests/it/scalar_replay_acceptance.rs:103-145 |

## Verdict

The Rust lane has no further findings. Checks that passed: the AC-6 test changes one member at a
time against a genuine QSL report, asserts that each changed claim differs, and is killed by M1.
The AC-12 test remints every changed claim with QSL's public `parity_obligation`, so the identity
stays self-consistent, and asserts the specific membership variant and never `Obligation`. The
AC-17 test asserts both the typed `ScalarIdentityMismatch::Node`/`Obligation` and the
`StaleDependency` terminal value. The new tags on existing tests were checked against their
bodies: AC-1 (plan admission over four families; `Implies` reaches `Unavailable`), AC-4
(`complete` refuses the compiled mutant's observation and every changed tie), AC-5 (genuine
`Agrees` for four operators), AC-15 and AC-16 (reader body equality, distinct draws, span
invariance, changed occurrence), and AC-14 (function replay settles through the function route;
frame identity is cross-checked against QSL's mint). Each asserts its AC. The AC-14 depth is in
the gap-analysis SR.
