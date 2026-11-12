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

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-004 | medium | The capability-last reorder makes a falsified run of a family with no operator-parity route produce a `Failed` terminal record. Measured at d851eea5: the routed `Implies` harness with a malformed playback returns `Playback(kani_witness_harness_missing)`, and `setup_settlement()` gives `Some(CgDefect)`, which the pair functions turn into `Failed`. FR-032-AC-24 and TC-047 step 13 say `CgDefect` applies only to a CG-raised failure on a supported scalar family, and that a falsified `Implies` run yields no terminal record. Before the fix the absent capability returned `Unavailable` and settled `None`. The precedence clause and AC-24 now conflict for this input; the code or the spec must say which family classification wins. | src/replay/scalar_builder.rs:127-135; src/replay/scalar_builder.rs:412 |
| FND-005 | low | For a harness whose capability is `Unavailable`, `plan` now skips the staged-artifact comparison, which is part of step 1. The comment's justification ("A harness without replay authority retains no artifact package") holds for the test fixture, which comes from `negotiate_kani_obligations` directly. It does not hold for routed generation, which binds the artifact package to every scalar harness, Boolean ones included. A routed `Implies` harness with changed artifacts therefore refuses as `Unavailable` or `Playback`, not `Artifacts`. | src/replay/scalar_builder.rs:115-118; src/routed/generate.rs:816-818 |

## Dispositions

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | d851eea5: `plan` treats `Authority(Unavailable)` as deferred and reports it only after the identity, byte-ceiling, adapter and domain checks. `decode_scalar_falsification` checks the byte ceiling and harness name before `cg_scalar_witness_operation_unsupported`. The precedence test now asserts that an `Implies` harness with an oversize, renamed or garbage playback, or a changed identity, gets the earlier refusal, and `Unavailable` only for a clean request. For supported families the order is unchanged: the tamper seal comes first, then the body and package, then the artifacts. Focused run: 337 passed, 0 failed. FND-004 and FND-005 record what this reorder introduced. |
| FND-002 | fixed | d851eea5 adds `draw_order_is_kept_and_unrelated_enclosing_parameters_are_excluded`: `y - x` with an unused `z` keeps the draw order y, x (ranges (0,9),(10,20)), so an identifier sort would fail it. `z` gets no position. The body equals the IR reader's positional body, and the genuine QSL route agrees. |
| FND-003 | fixed | d851eea5 adds the `replay/obligation.rs` unit test `function_identity_is_the_producer_identity_for_every_kind_and_argument_list`, tagged FR-032-AC-14. It pins that neither the kind nor positional arguments enter a function identity. |
| FND-004 | fixed | 96ae9d85: in `plan`, every refusal on a harness whose capability is `Unavailable` (body or identity mismatch, artifacts, adapter or decode) is wrapped in `ScalarBuildError::UnavailableFamily`, and `setup_settlement` maps that to `None`. `every_refusal_on_an_unavailable_family_settles_nothing` asserts that the routed and the unrouted `Implies` harness give no settlement and that the pair functions give `MissingSettlement`. A clean `Implies` request still returns `Authority(Unavailable)`. Supported-family refusals are unwrapped and classified as before. The only other `ScalarBuildError` consumer is the exhaustive test-local match in `scalar_settlement.rs`, which was updated; the pair functions take the settlement and are unaffected. |
| FND-005 | fixed | 96ae9d85: the artifact check now runs when `available || harness.retains_artifact_package()`. `routed_implies_fixture` builds the `Implies` harness through `generate_routed`, and the precedence test asserts that changed artifacts on it refuse as `UnavailableFamily(Artifacts)` before an oversize playback. The precedence assertions now pass the harness's own artifacts. |
| FND-006 | fixed | FR-032-AC-24 now lists `ScalarBuildError::UnavailableFamily` among the `None` settlements (a refusal on a family with no operator-parity route, wrapping the earlier cause), the status paragraph describes the variant, and TC-047 step 13 mirrors both, including the test-local classification list. |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-006 | low | 96ae9d85 adds the public variant `ScalarBuildError::UnavailableFamily(Box<ScalarBuildError>)`, but FR-032 does not mention it. The FR-032-AC-24 cell lists every variant's classification without it (the code and the test-local match both return `None` for it). The status paragraph still says "`ScalarBuildError` has no separate unsupported-capability variant; a family outside the profile surfaces as `Authority` caused by `Unavailable`, `Occurrence` or `Operands`, or as `ScalarBuildError::Operation`", which is no longer true for a refused request on such a family. The behaviour is correct; the spec text is stale. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:418; spec/replay/functional/FR-032-routed-scalar-replay-binding.md:511; src/replay/scalar_builder.rs:39-41 |
