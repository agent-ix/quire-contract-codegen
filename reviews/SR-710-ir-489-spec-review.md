---
id: "SR-710"
title: "IR-489 spec review: FR-015 V2 clause claim and V2 census input"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@2c5291e5b2fc8db4562505732e5088b10f2b7231; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/tests.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/core/functional/interface-001-codegen-api.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TM-004
    type: reviews
---

# SR-710: IR-489 spec review

## Summary

Ticket: IR-489. PR: agent-ix/quire-contract-codegen#218, head 2c5291e. Method: spec-review with
the EARS, integrity, matrix and traceability checks folded in, plus consistency with AD-003 and
AD-004 (merged) and with Contract IR FR-040 and FR-038 at IR origin/main.

`make spec` at the head and at origin/main both raise the same 3 warnings (FR-017 line 159,
FR-014 line 278 twice). The PR adds none.

## Method

- Diffed FR-015 against origin/main: 57 additions, 0 deletions, so FR-015-AC-1 to FR-015-AC-37 are
  byte-for-byte unchanged.
- Checked the V2 clause node shape against IR FR-040 at IR origin/main. The `state`/`state_clause`
  body has `operation.member` `{kind: state_clause, clause}`, with `clause` one of invariant,
  precondition or postcondition. Its parameters are `self: Reference<C>`, then the result (for a
  postcondition), then the operation's parameters; an invariant binds `self` alone. The PR's
  description of the shape is correct.
- Checked AD-004's 4c, 4d, 4e and 4f text, the cover rule (IR-464, L-4) and the regression
  control (L-5). Also checked QSL `tests/it/integer_lowering.rs` and quire-integration
  `tests/qsl_kani_exemplar.rs` (the `amount < 1000 implies amount + 1 <= 1000` control).
- Checked AD-003 E-1's member list against FR-015-AC-48. They match.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-39 binds every V2 clause argument by the IR `bounded_domain` of its type and refuses any claim with a parameter that has no finite `bounded_domain`. But every V2 clause binds `self: Reference<C>` first (IR FR-040), and a Reference has no `bounded_domain`. Read literally, AC-39 refuses every V2 clause, which contradicts AC-38 and AC-41. Nothing says how `self` (state fields, pre/post reads) or `result` is bound. AC-48's "declared domain" per argument has the same gap. | spec/kani/functional/FR-015-bounded-kani-obligations.md:244, spec/kani/functional/FR-015-bounded-kani-obligations.md:54-56, spec/kani/functional/FR-015-bounded-kani-obligations.md:253 |
| FND-002 | medium | AC-40 and AC-41 require embedding "the FR-014 oracle of that body". The clause condition, though, is an inline term (third argument) of a `state`/`state_clause` node, with no node id of its own. FR-014 takes node ids, and FR-014-AC-7 refuses state nodes. The amendment names no FR-014 dependency or amendment that produces this oracle. | spec/kani/functional/FR-015-bounded-kani-obligations.md:182-185, spec/kani/functional/FR-015-bounded-kani-obligations.md:245-246 |
| FND-003 | medium | AC-41 keeps only the clause-level control (the left+right+1 mutant falsified at 999). It drops FR-015-AC-37's (IR-458) independent native i128 assertion of each arithmetic outcome. So a mutant in the other direction (for example `+` returning `left`) still verifies the healthy clause, and nothing detects it. | spec/kani/functional/FR-015-bounded-kani-obligations.md:246 |
| FND-004 | medium | AC-46 fixes only where the cover goes. It does not say what the cover states, as AC-7 does (precondition holds; requires and bounds jointly satisfiable) and as IR-464 requires (the property's own reachability). It also leaves "the subject call" undefined for V2 precondition and invariant harnesses. | spec/kani/functional/FR-015-bounded-kani-obligations.md:251, spec/kani/functional/FR-015-bounded-kani-obligations.md:195-196 |
| FND-005 | medium | The identities conflict. AC-47 says changing the unwind bound or the subject changes "its identity". AC-48 says the identity of a V2 clause obligation is formed by AD-003 E-1, whose members exclude both. AC-45 folds the census into "that harness's identity". The amendment never distinguishes the harness identity record from the E-1 obligation identity. | spec/kani/functional/FR-015-bounded-kani-obligations.md:252-253, spec/kani/functional/FR-015-bounded-kani-obligations.md:250 |
| FND-006 | medium | AC-42 says "accounted `unsupported` or refused", so the disposition is left open. The Behavior bullet says `unsupported`, and the cited AC-14 says refused. "Integer-arithmetic grammar" is also ambiguous: AC-41 lists add, sub, mul and negate, but FR-014 also derives integer div and mod. | spec/kani/functional/FR-015-bounded-kani-obligations.md:247, spec/kani/functional/FR-015-bounded-kani-obligations.md:186-188 |
| FND-007 | low | Two of AC-42's cases cannot occur on an admitted `CheckedPackageV2`: "a clause value other than the three kinds" and "a body that is not Boolean". IR FR-040-AC-8 and FR-040-AC-10 refuse them at read, so neither case is testable, yet TC-025 step 12 asks for both. | spec/kani/functional/FR-015-bounded-kani-obligations.md:247, spec/kani/matrix/TC-025-bounded-kani-obligations.md:125-129 |
| FND-008 | low | The Description makes the V2 clause claim and the census "the requirement ... that FR-015-AC-8, FR-015-AC-22 and FR-015-AC-25 state", and says that all three "are backed by the V2 census input". AD-004 says this of AC-22 and AC-25 only; AC-8 is the precondition-assumption rule, not a census rule. It also says "are backed" in the present tense, though the backing is planned. | spec/kani/functional/FR-015-bounded-kani-obligations.md:30-33 |
| FND-009 | low | AC-45 adds order independence ("independently of the order the caller lists its dependencies") while claiming "the same rules as FR-015-AC-25". AC-25 does not state order independence. | spec/kani/functional/FR-015-bounded-kani-obligations.md:250 |
| FND-010 | low | The TM-004 row says "those two rows stay planned and unbacked". AC-22 and AC-25 share one row (FR-015-AC-19 through FR-015-AC-25), so there are no "two rows". | spec/kani/matrix/tests.md:23 |
| FND-011 | low | AC-41 names the concrete playback `amount = 999`. AD-004 L-5 and the quire-integration control name it `amount_current = 999`, and nothing reconciles the two names. | spec/kani/functional/FR-015-bounded-kani-obligations.md:246 |

## Verdict

Not mergeable as written: FND-001 must be fixed first, and FND-002 to FND-006 should be. The
amendment otherwise meets its brief:

- The V2 clause node shape is accurate to IR FR-040.
- FR-015-AC-1 to FR-015-AC-37 are unchanged.
- AC-38 to AC-48 are all marked PLANNED.
- AC-44 copies AC-22's invalid-census rules faithfully.
- The matrix keeps AC-22 and AC-25 verbatim and follows AD-004's 4f planned/unbacked rule.
- The interface-001 sentence is consistent with AD-004 4e.

The PR adds no pins, SHAs, version records or compatibility layers, and discloses nothing
confidential. `make spec` adds no warning.

## New findings (disposition pass 1)

Reviewed at ec187e33afdc23e8991a3c0e5fd4b6e7493d2e09. I checked the new text against IR FR-040 and
FR-038 at IR origin/main, and against CG `src/state_frame.rs` at origin/main, which reads a field as
`project(deref(self), field)` and its pre-state value as `quire.op.state.pre`.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-012 | high | AC-39 and the Inputs bullet make every post-state field read, and a postcondition's result, a nondeterministic argument held inside its domain by an assumption. In a postcondition harness those values are what the customer subject produces: AC-46 calls the subject, AC-27 asserts against the state after the subject runs, and Behavior requires the post-state result to stay inside its domain. If they are drawn independently, the subject does not affect the verdict, and a result outside its domain is assumed away, which breaks AC-4. Only the pre-state, the parameters and (for a precondition or invariant) the state should be drawn. | spec/kani/functional/FR-015-bounded-kani-obligations.md:262, spec/kani/functional/FR-015-bounded-kani-obligations.md:54-63, spec/kani/functional/FR-015-bounded-kani-obligations.md:186-189 |
| FND-013 | medium | FR-014-AC-38 does not say how a field read becomes an oracle operand. A V2 clause condition reads state through `project(deref(self), field)` and `pre(...)` terms. Those are neither a literal nor a reference, so FR-014-AC-10 refuses them, and their bound is the object member's `integer_range`, not a descriptor parameter's reachable `bounded_domain`. As written, FR-014-AC-38 conflicts with FR-014-AC-10. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:348, spec/oracle/functional/FR-014-exact-scalar-oracles.md:301-305 |
| FND-014 | medium | AC-48 identifies a state field read "by the object type's field node". IR FR-040 says a field is named by its declaring node and its name, never by a node of its own. AC-48 also widens AD-003 E-1's "parameter node id" to non-parameter arguments while still claiming the identity is "formed by AD-003 E-1". | spec/kani/functional/FR-015-bounded-kani-obligations.md:271 |
| FND-015 | medium | AC-46 says the cover states that "the clause's evaluation completes", "as FR-015-AC-7 does". For the precondition family, AC-7 and AD-004's cover rule (IR-464) say the cover states that the precondition holds, which is a different property. | spec/kani/functional/FR-015-bounded-kani-obligations.md:269 |

## New findings (disposition pass 2)

Reviewed at 45ba5c2a9b061853dbe8ae7990d5dc6d1cec99ea.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-016 | low | The Inputs bullet and AC-39 disagree on what a postcondition harness draws. The Inputs bullet says the harness "draws only the pre-state and the operation parameters". AC-39 draws only "each `pre(...)` state field read". Take a postcondition whose body reads a field only bare, such as `self.balance >= 0`: under AC-39 it draws no pre-state for that field, yet the subject call needs an input state for it. The fix is to draw the pre-state of every field the body reads, bare or `pre(...)`, or to defer the subject's input state to FR-025 explicitly. | spec/kani/functional/FR-015-bounded-kani-obligations.md:271, spec/kani/functional/FR-015-bounded-kani-obligations.md:60-62 |

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | ec187e33afdc23e8991a3c0e5fd4b6e7493d2e09: a `Reference` parameter such as `self` is not an argument and binds through its object's declared fields (AC-39). The new binding semantics have their own defect, FND-012. |
| FND-002 | fixed | ec187e33afdc23e8991a3c0e5fd4b6e7493d2e09: planned FR-014-AC-38 and its Behavior bullet, cited by AC-40 and the Behavior. The new AC has its own defect, FND-013. |
| FND-003 | fixed | ec187e33afdc23e8991a3c0e5fd4b6e7493d2e09: AC-41 keeps the AC-37 native `i128` assertion per arithmetic subterm, and TC-025 step 11 adds the `left` mutant. |
| FND-004 | fixed | ec187e33afdc23e8991a3c0e5fd4b6e7493d2e09: AC-46 states what the cover states and defines the subject call. The precondition cover wording is FND-015. |
| FND-005 | fixed | ec187e33afdc23e8991a3c0e5fd4b6e7493d2e09: AC-47 now concerns the harness identity record and AC-48 the E-1 obligation identity, and the two are distinct values. |
| FND-006 | fixed | ec187e33afdc23e8991a3c0e5fd4b6e7493d2e09: AC-42 makes an unsupported operator `unsupported` and refuses an unmodelled or absent node as AC-14 states. Division, modulo and abs are named. |
| FND-007 | fixed | ec187e33afdc23e8991a3c0e5fd4b6e7493d2e09: the two unreachable cases are dropped from AC-42, and TC-025 step 12 marks them unreachable. |
| FND-008 | fixed | ec187e33afdc23e8991a3c0e5fd4b6e7493d2e09: the Description is marked Planned (IR-489), AC-44 and AC-45 back AC-22 and AC-25, and AC-8 is no longer said to be census-backed. |
| FND-009 | fixed | ec187e33afdc23e8991a3c0e5fd4b6e7493d2e09: AC-45 states order independence as an addition ("in addition, ..."). |
| FND-010 | fixed | ec187e33afdc23e8991a3c0e5fd4b6e7493d2e09: the row names the single row holding AC-22 and AC-25 (FR-015-AC-19 through FR-015-AC-25). |
| FND-011 | fixed | ec187e33afdc23e8991a3c0e5fd4b6e7493d2e09: the playback is `amount_current = 999` in AC-41 and TC-025. |
| FND-012 | fixed | 45ba5c2a9b061853dbe8ae7990d5dc6d1cec99ea: only the pre-state reads and the operation parameters are drawn; a postcondition's result and post-state come from the subject call, with their domains asserted and never assumed (AC-39, consistent with AC-4, AC-11 and AC-27). The new Inputs and AC-39 mismatch is FND-016. |
| FND-013 | fixed | 45ba5c2a9b061853dbe8ae7990d5dc6d1cec99ea: FR-014-AC-38 makes `project(deref(self), field)` and `pre(...)` of one operands, ranged by the member's `integer_range`, and keeps FR-014-AC-10 for every other operand. This matches state_frame.rs (`quire.op.record.project`, `quire.op.model.deref`, `quire.op.state.pre`). |
| FND-014 | fixed | 45ba5c2a9b061853dbe8ae7990d5dc6d1cec99ea: AC-48 names a field read by declaring node id and field name, applies E-1 as written to parameters, and records E-1's scope as an open question for the AD-003 owner. |
| FND-015 | fixed | 45ba5c2a9b061853dbe8ae7990d5dc6d1cec99ea: AC-46 states a cover per family. Precondition: it holds within the bounds. Postcondition: after the subject call, the requires (AC-43's anchor preconditions) and the bounds are jointly satisfiable, which matches AC-7 and Behavior. Invariant: the drawn state satisfies its assumptions. |
