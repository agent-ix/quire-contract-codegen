---
id: SR-17940486709564847047610
title: "IR-304 slice (a) PR #772 code review (Rust lane)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #772 (head 90aec995, base 6bc424e9); IR-304 slice (a); src/core/requirement.rs, src/routed/generate.rs, src/kani/generate/negotiate.rs, src/kani/generate/outcome.rs, src/oracle/scalar/lower.rs, src/oracle/scalar/mod.rs, src/lib.rs, src/routed/contract_package_tests.rs, tests/common/requirements.rs, tests/it/*.rs, tests/fixtures/kani-scalar/direct-increment-record.json"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: reviews
---

## Summary

Ticket: IR-304 (slice (a) only). Code review with the rust-review lane folded in, over
`git diff origin/main...HEAD`. Examined the new CG-owned record types, the whole-call record
checks in `generate_routed`, the interim guard refusal in `classify_claim`, the V2-frame result
bound translation `selected_result_bound_node`, the public-API change to
`ObligationItem::ScalarClaim` (new `path_condition` field), and every migrated or new test.

Runs (shared target dir, one cargo job at a time): the branch `it` tests for
`requirement_records`, `routed_generation`, `capability_settlement`, `scalar_literal_operands`,
`scalar_playback`, `kani_obligations`, `kani_scalar_native`, `no_generation_panics` pass
(311 passed, 0 failed, 20 ignored Kani-lane); lib `routed::` and `kani::generate` pass except
`production_refinement_live_solver_handoff_reaches_qsl_checker`, which panics with
`live refinement qualification requires QUIRE_CVC5_BIN` identically at the base-equivalent
commit 1a70b8aa (its `src/` is byte-identical to 6bc424e9), so it is environmental.

## Verdict

**PASS WITH FINDINGS.** No correctness defect found. Check order matches FR-022 (after
`ScalarSelectionMismatch`, before per-item `Package` and `UnauthenticatedOperationView`;
per item: RequirementMissing/OccurrenceNotSelected, RecordMissing, ApplicationMismatch,
ResultBoundMismatch, ConversionContextMismatch, PathConditionMismatch). Record lookup is by
the exact occurrence key; node comparisons use `same_source_node`; the admitted bound is
translated with `original_represented_node`. The interim rule settles a non-empty path
condition as `path_guard_not_lowered`/`guard_lowering_not_implemented` only after FR-014
refusals and `operation_not_rendered`, in both `generate_routed` and `generate_routed_native`.
No slice (b) code (no `Assume`, identity fields, `PathConditionUnsatisfiable`,
`GuardContextMissing` or other `PathGuardRefusal` variants). No new production
`unwrap`/`expect`/`panic!`, no hashes, digests or pins, no compatibility layer. Every
`ObligationItem::ScalarClaim` construction site is updated (intended break, named by spec).
The findings are test-strength gaps: mutations of the check logic that the suite would not
catch.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The `selected` leg of `ResultBoundMismatch` and `ConversionContextMismatch` is never exercised: every FR-022-AC-28 case makes the item and record disagree with each other, so deleting `selected_bound...is_some_and(...)` and `!same_conversion(&record.conversion_context, &selected_conversion)` leaves every test green. Add a case where item and record agree with each other but not with the authenticated selection (both bounds, and both conversion contexts, set to the other occurrence's). | src/routed/generate.rs:751-777; tests/it/requirement_records.rs:560-617 |
| FND-002 | low | `PathConditionMismatch` is tested only by a length difference (one guard added to the record). A same-length sequence differing in a guard's `holds`, its occurrence, or order is untested, so reducing the comparison to a length check survives. | src/routed/generate.rs:778-794; tests/it/requirement_records.rs:618-636 |
| FND-003 | low | The interim guard refusal's precedence over `domain_not_representable_in_i64` and `result_bound_unreachable` (the `BoundNotI64`/`ResultUnreachable` arms of the guard match) is untested; removing those arms survives. A guarded item whose unguarded twin is `result_bound_unreachable` would pin it. | src/kani/generate/negotiate.rs:1244-1257; tests/it/requirement_records.rs:967-1027 |
| FND-004 | low | `selected_result_bound_node` returns `None` both for a context with no bound and for any resolution failure (`items.at`, `accounted_node`, `result_context` errors), and `None` skips the selected-bound comparison, so a resolution failure fails open. Unreachable today because `ScalarSelectionMismatch` resolves the same selection first, but nothing ties the two together. | src/oracle/scalar/lower.rs:2356-2410; src/routed/generate.rs:744-755 |
