---
id: SR-17940486709564847047932
title: "IR-1530 PR #816 spec-review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #816 at c17e1ee3 (git diff origin/main...HEAD); IR-1530; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md, docs/qualification/ir1530-general-route-scalar-not-tractable.md, docs/qualification/ir304-kani-guard-native-harness.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-027
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
---

## Summary

Ticket: IR-1530. The PR changes no spec file. It moves four real-Kani tests from the general
`generate_routed` scalar fixture to the `generate_routed_native` fixture and adds a qualification
record. This review checks whether the spec and matrix rows those tests are tagged to still say
what the tests now do.

Rows examined: FR-015-AC-37 (TC-025 step 9 and step 47), FR-017-AC-6, FR-017-AC-7, FR-017-AC-11,
FR-017-AC-27, FR-017-AC-28 to AC-30, FR-017-AC-45 to AC-50, FR-017-AC-31, FR-015-AC-125 and
FR-015-AC-126, the TC-027 "Routed scalar harness (FR-017-AC-11)" and "Paired routed-scalar
proof/native receipt (FR-017-AC-27 to AC-31)" procedure paragraphs, and the `spec/kani/matrix/tests.md`
rows for these criteria. The trace tags are unchanged, and `quoin matrix` still binds each moved
test to the same criteria.

Clean: FR-015-AC-37 and TC-025 steps 9 and 47 are route-neutral ("a scalar function-application
harness"; "the oracle's `Add` replaced by `Subtract`"). The native mutant edits the production
native oracle (`nx::NativeIntegerOperation::Add {` emitted by `src/oracle/scalar/native.rs:116`),
not the harness's independent `i128` reference, so the AC still holds as written. FR-017-AC-6, AC-7
and AC-11 name a `KaniScalarObligationHarness` without a route, and the native harness is one.
FR-015-AC-126 already plans regenerating the TC-027 healthy and bound mutant and the TC-025
arithmetic mutant on the admitted native-integer API, so the move agrees with it.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | FR-017-AC-27 says "A genuine routed `x + 1` scalar harness from `generate_routed`", and TC-027's procedure says to build the routed scalar harness "with `generate_routed`" (FR-017-AC-11 paragraph) and the paired receipt from "a genuine routed `x + 1` harness". The tagged tests `tc_027_attested_scalar_receipt_binds_real_proof_and_native_result`, `tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified` and `kani_generation_real_v2_scalar_verifies` now build it with `generate_routed_native`. The PR edits no spec text, so the AC and the TC-027 procedure name a route the evidence no longer runs. Amend FR-017-AC-27 and the two TC-027 paragraphs to name the native route (or make them route-neutral) and cite `docs/qualification/ir1530-general-route-scalar-not-tractable.md` for why | spec/kani/functional/FR-017-kani-execution-evidence.md (FR-017-AC-27 row), spec/kani/matrix/TC-027-kani-execution-evidence.md:145-153, spec/kani/matrix/TC-027-kani-execution-evidence.md:405-412 |
| FND-002 | low | `docs/qualification/ir304-kani-guard-native-harness.md` still calls `kani_generation_real_v2_scalar_verifies` "the existing general-route lane test" (lines 20-24) and lists the three tests as ending `TimedOut` (lines 95-100). After this PR all three run the native route and pass. The new IR-1530 record does not point back to these lines, and they do not point to it. Add a one-line forward reference in the IR-304 record | docs/qualification/ir304-kani-guard-native-harness.md:20-24, docs/qualification/ir304-kani-guard-native-harness.md:95-100 |

## Verdict

Not mergeable as is: FND-001 leaves FR-017-AC-27 and TC-027 describing the general route while the
evidence runs the native route. The fix is a small spec edit in this PR. FND-002 is a one-line
cross-reference. The other examined rows stay accurate.
