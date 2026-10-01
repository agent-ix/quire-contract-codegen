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
