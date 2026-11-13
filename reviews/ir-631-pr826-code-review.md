---
id: SR-17940486709564847047964
title: "IR-631 PR #826 code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #826 at 25030e08, git diff origin/main...HEAD; IR-631; docs/qualification/ir631-scalar-replay-real-kani-evidence.md, spec/replay/functional/FR-032-routed-scalar-replay-binding.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
---

## Summary

Ticket: IR-631. The diff is documentation plus spec status: a new qualification note recording a real
Kani run of the routed native `x + 1` arithmetic mutant, and FR-032 AC-7/AC-8/status-paragraph text that
moves the falsified-run to `Diverged` join to "IR-610 driver evidence". No Rust changed, so the
rust-review lane has nothing to review; the code claims made by the doc were checked against
`src/kani/identity.rs`, `src/replay/scalar_builder.rs`, `src/kani/run/scalar.rs` and
`tests/it/kani_scalar_native_private.rs`.

Verified: the mutant test decodes the real playback with `decode_scalar_falsification`, the decoder
`OriginalScalarContext::plan` uses, and asserts two draws with `left` in `[0, 9]` and `right == 1`, as the
doc says. `execute_kani_scalar_native` returns `ProofNotVerified` for any non-`Verified` proof, and
`ScalarReplayPlan::complete` refuses any staged-entry difference. Kani 0.68.0 and the cadical solver
match the installed toolchain and `execute.rs`. The 29 s / 2.49 GB numbers were not re-run (cold target
directory; a Kani rerun is not needed to judge this PR).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | The doc's stated mechanism does not match the run it records. The mutant is re-sealed with `KaniScalarObligationHarness::generated(identity, mutated_rust, record)`, so `has_valid_provenance()` is true for it and neither the seal check nor `execute_kani_scalar_native::validate_request` is what stops it (the run calls `execute_kani_obligation` directly and never enters `execute_kani_scalar_native`). `plan` would refuse this harness because `generated()` binds no replay body and no original package (`ScalarAuthorityError::Unavailable`). That gap is closable in a crate-private test with the `pub(crate)` `bind_scalar_replay_body` and `bind_artifact_package`, the same kind of deliberate re-seal the test already does. So "cannot be produced inside CG" holds only for public APIs, and the doc should say so and name the real refusal. | docs/qualification/ir631-scalar-replay-real-kani-evidence.md:22-30, src/kani/identity.rs:881-900, tests/it/kani_scalar_native_private.rs:86-93 |
| FND-002 | low | The evidence is not reproducible from the doc. It gives no command line, reviewed commit, rustc/CBMC version or retained output, though sibling notes (`ir304-kani-guard-cover-attempt.md`, `ir1447-kani-control-probe.md`) give the command and CBMC 6.11.0. Also, "decoded by the decoder `plan` uses" is right for the decode step, but the test parses with an entry limit of `usize::MAX`, not `plan`'s `arguments.len() + guard_inputs.len()`. | docs/qualification/ir631-scalar-replay-real-kani-evidence.md:6-17, src/replay/scalar_builder.rs:126-132, tests/it/kani_scalar_native_private.rs:95-101 |

## Verdict

Not mergeable as written. The measurements look sound, but the doc's explanation of why the join
cannot be built cites refusals that do not apply to the re-sealed mutant. That explanation is what the
IR-610 handoff in FR-032 rests on (see the spec-review artifact SR-17940486709564847047966, FND-001).
