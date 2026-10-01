---
id: "SR-654"
title: "CG PR 212 gap analysis: corpus operator typing, refused-and-pinned cases and coverage"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@58dd43d829f4ee742c87775455e73dc98adde05e; spec/test-matrix.md, spec/functional/complete-v1/FR-014-*.md, spec/functional/complete-v1/FR-018-composite-equality-oracles.md, src/exact_scalar.rs, src/composite_equality.rs, tests/**"
---

# SR-654: CG PR 212 gap analysis

## Summary

Ticket: IR-480. PR: agent-ix/quire-contract-codegen#212 at 58dd43d. This gap analysis is scoped to
the PR's own diff. It checks whether each dropped or changed vector is recorded honestly in the
Test Matrix, whether the tagged tests back the rows that claim them, and whether production code
lost its only test. Plan completion: not assessed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Removing E_SELF leaves no generation, golden-item or generated-crate agreement evidence for any recursive composite type. The generator's recursive-closure path (`TypeClosure.in_progress`) and the runtime's recursive equality are no longer reached end to end. The matrix records this only in the notes prose. The FR-018-AC-2 row ("on every vector of the composite-equality corpus") stays at Covered over a corpus that shrank to exclude recursion. The stated reason ("QSL emits a recursion leaf") is wrong; see SR-653 FND-003. Either restore the vector once IR's cycle refusal is resolved, or mark the gap on the AC-2 row itself | spec/test-matrix.md:57, 94; tests/composite_equality_support/agreement_cases.rs:164-165 |
| FND-002 | low | The TextAdmission generator paths are reachable from no admitted package and have no test: the derivation selector (`Family::NumericConvert` with a text result), the body emission and the runtime call. The selector assertion in `tc_024_each_overloaded_identity_derives_by_its_own_selector` was deleted. The FR-014-AC-2 row records the generation gap honestly, but the FR-014-AC-18 row's note does not mention the lost text-admission selector case | src/exact_scalar.rs:1250, 2263, 2673, 3289-3295; tests/it/exact_scalar_generation.rs:2009-2011; spec/test-matrix.md:39 |
| FND-003 | low | `tc_029_a_recursive_compared_type_is_refused_by_ir_today` carries `Trace: FR-018-AC-8`. AC-8 covers generation-time `TypeEnvironment::new` declaration refusals and the `check_type` guard; an IR admission refusal backs no clause of it. The tag would count toward AC-8 if that row is promoted from Planned | tests/it/composite_equality_generation.rs:549-556 |

## Verdict

Checked and honest:

- FR-014-AC-2 is split into a Partially-covered row. It names text admission and the IR refusal,
  points at the pin, and states the unblock condition. I verified "QSL lowers no text admission"
  at a28a5578: qsl-semantics has no TextAdmission lowering.
- The pin asserts each of the six refusals exactly, and the six are kept in `refused_corpus()`,
  not deleted or ignored.
- The AC-7 note correctly says the direct-reference form is refused by IR before the generator
  runs, and that `E_REFERENCE` now exercises the "operand reaching a reference" clause.
- FR-018-AC-15 has its own test and is listed in TC-029. The FR-018 Covered row is extended
  through AC-15.
- The removed 288 text-admission agreement vectors and the six `admit_*` oracle names follow from
  the refusal and are recorded under FR-014-AC-2.
- The four ordered-enum codes (1073, 1123-1125) are back in the admitted corpus with reference
  operands, and their oracle names are in `SCALAR_ORACLE_NAMES`.

The `> 60` floors are discussed in SR-653 FND-006. They lose no coverage, because the exact-count
assertion stands.
