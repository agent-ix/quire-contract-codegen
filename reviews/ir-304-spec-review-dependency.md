---
id: SR-17940486709564847047595
title: "IR-304 PR #768 FR-022 per-occurrence requirement records: spec-review dependency"
type: SpecReview
analysis: dependency
date: 2026-11-10
scope: "agent-ix/quire-contract-codegen; PR #768 @ de3d7ee1; IR-304; spec/routed/functional/FR-022-routed-generation.md, spec/routed/matrix/TC-033-routed-generation.md, spec/routed/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-033
    type: reviews
---

## Summary

Ticket: IR-304. Spec-only PR #768 (head de3d7ee1, base origin/main cf10ec78) rewrites FR-022's QSL requirement-record intake, adds the 'Per-occurrence requirement records' and 'Path conditions in the scalar harness' subsections, rewrites AC-26/27/28, adds AC-31..35 and an IR-304 Rationale, and updates the TC-033 procedure for slices (a)/(b) and the routed tests.md row. Verified against CG src/routed/generate.rs, src/kani/generate/{spec,scalar}.rs, src/kani/identity.rs, src/replay/scalar.rs, src/core/contract_package.rs, src/oracle/scalar/lower.rs; IR quire-contract-model c8dca8c (ContractOccurrenceKeyV1, result_context); QSL origin/main ffec6c72 (qsl-semantics check/claims.rs PathGuard/RequirementRecord/key_claims, qsl-route request.rs RequirementItem).

## Verdict

FAIL (two medium findings). The new depends_on edges (FR-039 for the context span, FR-014 for operand ranges) and the references edges (FR-017 for CoverUnsatisfied, FR-002 for the IR-815 text rendering) are correct. CG needs no new QSL crate: CG depends on qsl-replay only, and the projection is driver-built. The missing edges are FR-032 (replay of a guarded harness) and the FR-015 reason vocabulary.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Guard inputs change the routed scalar harness's playback schema, and FR-032 (routed scalar replay binding) is neither updated, referenced nor scoped out. src/replay/scalar.rs decode_scalar_falsification requires playback.entries.len() == operand arity and reads one eight-byte i64 per argument, and FR-032 binds 'one eight-byte i64 value per symbolic operand'. A harness with a guard input (Boolean or integer `y`, AC-27/AC-33) yields extra entries, so every falsification of a guarded harness fails decode as kani_witness_arity_mismatch, and a Boolean entry is not eight bytes. Fix: add FR-032 to relationships and state how guard_inputs enter (or are excluded from) the replay schema and the QSL witness, or record this as an explicit out-of-scope gap with a follow-up ticket. | spec/routed/functional/FR-022-routed-generation.md:303-320; src/replay/scalar.rs:33-35,92-99; spec/replay/functional/FR-032-routed-scalar-replay-binding.md:125-128 |
| FND-002 | medium | FR-022 defines two new FR-015 reasons, `UnsupportedObligation::PathGuardNotLowered { guard, reason }` and `PathConditionUnsatisfiable`, and calls them 'FR-015's', but FR-015 is not edited. FR-015 enumerates its own reason vocabulary (a closed set the driver consumes; prior extensions are declared there, for example 'plus the five new reasons named below'). PathGuardNotLowered has no wire name, and its reason enum (UnsupportedGuardForm / UnboundedGuardOperand) has no type name or wire form. Rationale 3 says the change 'avoids widening the disposition vocabulary the driver consumes', but it widens the reason vocabulary. Fix: add both variants, their payloads and wire names to FR-015 (with an FR-015 AC or a cross-reference), and correct Rationale 3. | spec/routed/functional/FR-022-routed-generation.md:251-259, 291-296, 664-667; spec/kani/functional/FR-015-bounded-kani-obligations.md:528-531 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-022-relationships | examined | spec/routed/functional/FR-022-routed-generation.md | relationships: - target: ix://agent-ix/quire-contract-codegen/StR-001 type: satisfies - target: ix://agent-ix/quire-contract-codegen/FR-015 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-019 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-039 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-014 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-017 type: references - target: ix://agent-ix/quire-contract-codegen/FR-002 type: references --- |
| FR-022-scalar-identity-fields | examined | spec/routed/functional/FR-022-routed-generation.md | The harness identity `ScalarObligationIdentity` gains three fields:  - `requirement_occurrence: Option<RequirementOccurrence>`, the item's occurrence key, set on the routed path. It is serialized as its node, role and ordinal fields and omitted when `None`, so the records of a direct FR-015 caller keep their bytes. - `path_condition: Vec<RequirementGuard>`, in outermost-first order, omitted when empty. - `guard_inputs: Vec<ScalarGuardInput>`: each extra guard input's identifier, parameter node and Boolean or narrowed inclusive integer domain, omitted when empty.  This is the interim structural |
| FR-022-path-guard-forms | examined | spec/routed/functional/FR-022-routed-generation.md | A guard is lowerable in one of two forms:  - a direct reference to a Boolean parameter (`if p`); - an IR-confirmed `quire.op.integer.{lt,le,gt,ge,eq,ne}` comparison whose two operands are each a direct reference to an integer parameter typed by a bounded domain, or an integer literal, with at least one parameter reference.  Any other guard (arithmetic inside the guard, a call, a model read, a negation node) has form `UnsupportedGuardForm`; a comparison reading an integer parameter with no bounded domain has `UnboundedGuardOperand { parameter }`. Either settles the item `unsupported` as FR-015' |
| FR-022-path-guard-unsatisfiable | examined | spec/routed/functional/FR-022-routed-generation.md | If a parameter's interval becomes empty, the occurrence lies on a path no input reaches. QSL accepts such a dead branch, so CG neither proves its claim vacuously nor reports `result_bound_unreachable`: it settles the item `unsupported` as `UnsupportedObligation::PathConditionUnsatisfiable { guard, parameter, minimum, maximum }` (wire `path_condition_unsatisfiable`), naming the guard at which the interval became empty and the parameter's declared range, with no harness. This check runs before the reachable-result check. A guard set that is infeasible but not detected by narrowing (for example t |
| FR-022-evidence-slices | examined | spec/routed/functional/FR-022-routed-generation.md | - Slice (a), typed record input: `RequirementOccurrence`, `RequirementGuard`, `RequirementRecordInput`, `RequirementRecords`, `RoutedRequirement`, `RoutedGenerationItem::with_requirement`, `KaniGenerationContext.requirement_records` and the `RequirementRecord` refusal with every `RequirementRecordMismatch` variant except `GuardContextMissing`. It covers FR-022-AC-26 and FR-022-AC-28. Until slice (b) lands, an item whose path condition is not empty settles `PathGuardNotLowered` with `UnsupportedGuardForm` for its first guard, which slice (b) keeps for forms it does not lower; it never yields an |
| FR-022-rationale-ir-304 | context_only | spec/routed/functional/FR-022-routed-generation.md | ## Rationale (IR-304 decisions)  1. **Guards become assumptions, not oracles.** A path guard restricts which inputs the claim is about; it is not a second property to prove. `Statement::Assume` is the renderer's existing way to restrict inputs, and `HarnessSpec` already refuses an assumption after the oracle call (`AssumeAfterCall`), so a guard cannot be placed where it would mask a failing call. Adding the guard to the embedded oracle would change the oracle crate shared by the group and break the byte-identity FR-015 requires between an embedded oracle and the crate's function. Only the two  |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 at d7760170. Verified: FR-032 gains guard-input decoding (eight-byte i64 or one-byte bool, in guard_inputs order after the operands) and cg_scalar_witness_guard_out_of_domain, tested by FR-032-AC-19 and TC-047 step 9. The FR-032 references edge is added. The widths match render.rs, which draws a Boolean Input as `let p: bool = kani::any()`. The arity and width codes are existing replay/scalar.rs codes, and the out-of-domain code is reachable by synthetic playback. |
| FND-002 | fixed | Round 1 at d7760170. Verified: FR-015 now lists path_guard_not_lowered and path_condition_unsatisfiable as two EARS bullets pointing to FR-022, with a references edge. Rationale 3 now says the reason vocabulary widens. The two bullets do not contradict FR-015's existing reason statements. |
