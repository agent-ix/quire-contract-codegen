---
id: SR-17940486709564847047727
title: "IR-631 PR #803 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #803 at 37e3e1f6; IR-631; FR-032-AC-1, AC-2, AC-4, AC-5, AC-6, AC-12, AC-14, AC-15, AC-16, AC-17, AC-18; src/replay/scalar_builder.rs, src/kani/identity.rs, src/core/scalar_authority.rs; tests/it/scalar_replay_acceptance.rs and the newly tagged tests"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-047
    type: reviews
---

## Summary

Ticket: IR-631. Plan completion: not assessed. `quoin matrix --repo .` (quoin 0.30.1) shows every
reviewed criterion as `tagged` (AC-1 3 binders, AC-6 2, AC-12 1, AC-14 3, AC-15 5, AC-17 3, AC-18
5) and FR-032-AC-19 `untagged` (intentionally Planned, out of scope). No production code changed.
The question for each criterion is whether its tagged tests back the IMPLEMENTED cell this PR
writes.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | FR-032 Setup Refusal Precedence puts capability availability last (step 5) and says "the absent capability never hides" an earlier refusal. `OriginalScalarContext::plan` calls `harness.scalar_replay_body()` first and returns `Authority(Unavailable)` before the body, artifact, byte-ceiling or adapter checks. Measured: a routed `Implies` harness with an oversize playback, or with the text `garbage`, returns `Authority(Unavailable)` instead of `cg_witness_decode_bytes_limit` or an adapter refusal. The new precedence test covers steps 1 to 4 only, while its doc claims "the stated step order". AC-2 ("under the stated precedence") and AC-1 ("after the ordered setup checks ... unsupported-capability only while") were flipped to IMPLEMENTED although the code violates this clause. Fix: either check playback before the capability, or amend the spec to place capability refusal at the accessor, and add a step-5 ordering assertion. | src/replay/scalar_builder.rs:97-99; tests/it/scalar_replay_acceptance.rs:85-163 |
| FND-002 | medium | AC-15 was flipped to IMPLEMENTED, but its exclusions "no identifier sort ... or unrelated enclosing parameter passes" have no test. Every multi-operand fixture draws in identifier order (`x + y`, `x - y`, `x * y`) or uses one identifier (`x + x`, `x + 3`), so sorting operands by identifier gives the same body. No fixture has an enclosing parameter that the application does not use. | tests/it/scalar_replay_residuals.rs:15-50; tests/it/scalar_replay_binding.rs:304-310 |
| FND-003 | low | The AC-14 tags are legitimate as non-regression evidence. `tc_026` fails if the function path is redirected through scalar parity, and `tc_035` fails if the frame identity encoding changes, because it compares against QSL's mint. However, `tc_025` asserts only that hand-written identities are carried, and no AC-14 test pins the function identity encoding that the criterion names. | tests/it/state_frame_obligation_identity.rs:67-91; tests/it/skeleton_spine.rs:255-268 |

## Verdict

Not mergeable while FND-001 is open: two status cells (AC-1, AC-2) claim behaviour that the code
measurably violates. Backed flips: AC-6 (one-member mutations against a genuine report; M1 in the
code-review SR kills it), AC-12, AC-16, AC-17 (QSL's typed `stale_dependency` at the facade plus the
CG `BodyMismatch` for a body-only edit) and AC-14 (with the FND-003 caveat). AC-18 rests on the
seal test described in code-review SR-17940486709564847047726 FND-001. Its other evidence (zero or
several occurrences in `core/scalar_authority.rs`, plan refusals for a caller-supplied body) is
real, but no test shows the retained-member absence refusal itself. AC-4 and AC-5 correctly stay
Partial. Their tests assert the native observation tie and genuine `Agrees`, but the real-Kani
proved-artifact observation (AC-4) and a QSL `Diverged` from changed generated arithmetic (AC-5,
not the driver-asserted `Completed(7)` of step 10) are not exercised. Flipping them would have
been wrong. AC-19 stays Planned as stated.
