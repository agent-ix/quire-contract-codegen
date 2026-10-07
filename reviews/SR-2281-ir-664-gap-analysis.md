---
id: "SR-2281"
title: "CG IR-664 gap analysis: i128 consumer adaptation against the CG spec, the computed matrix and the tests"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@code/ir-664-i128-consumers (frozen candidate, no PR yet; reviewed revision recorded in the IR-664 Linear marker only, per this repository's no-SHA rule); spec/core/functional/interface-001-codegen-api.md (primitive_types, dependency_types, FR-016 replay row), spec/strategy/functional/FR-008-bound-domain-strategy-admission.md (FR-008-AC-3), spec/core/non-functional/NFR-002-provenance-boundary.md (NFR-002-AC-3), spec/replay/functional/FR-024-counterexample-envelope-intake.md (FR-024-AC-19, FR-024-AC-29), spec/core/matrix/tests.md (TC-003); src/oracle/boolean_v1.rs, src/strategy/bound/generation.rs, src/kani/abi.rs, src/kani/generate/{clause,v1_bundle}.rs, src/replay/*; tests/it/{oracle_generation,bound_strategy_generation,kani_obligations_state_clause_replay,skeleton_spine,terminal_map}.rs"
---

# SR-2281: CG IR-664 gap analysis

## Summary

Ticket: IR-664. Plan completion: not assessed (planless). Static analysis: no build or test run
by the reviewer.

No `spec/` file changed in the diff, so spec-review does not apply. I read the CG requirements
that govern the changed code:

- interface-001 `primitive_types`/`dependency_types`: "bounded integer maps to i64 with the
  checked IntegerType domain, inclusive minimum, inclusive maximum and overflow policy
  retained". The code keeps i64 and refuses a domain it cannot represent. Met for representable
  domains.
- FR-008-AC-3: an oracle-refused clause is refused with `UnsupportedClause`. The new
  `wide_ir_domain_refuses_the_i64_strategy` asserts exactly that for maxima `i64::MAX + 1`,
  `u64::MAX` and `i128::MAX`. Binding correct.
- NFR-002-AC-3: unsupported states stay explicit. The new oracle test asserts
  `UnsupportedExpression` and its message. Binding correct.
- FR-024-AC-19 (TC-035 shape refusal): it is now driven by QSL-emitted packages with a declared
  parameter or result and asserts the same typed refusal. Binding correct.
- FR-024-AC-29: `src/replay/frame.rs` still builds no `DeclaredDomain`/`DomainKey`. The
  `declared_domains` that `ReplayInputs` carries is the function-path request member, not the
  frame envelope. Met.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Only the maximum side of the new i64 narrowing is tested. Both wide-domain tests vary only `maximum` (`i64::MAX + 1`, `u64::MAX`, `i128::MAX`), with the minimum at 0 or in range. A mutant that deletes `i64::try_from(value.minimum()).is_err() \|\|` at boolean_v1.rs:587 survives every test. Failure scenario: a domain `i128::MIN..=0` is then admitted and its bound rendered into i64 generated code. IR-664 asks for adverse tests at the i128 limits. Add the `i64::MIN - 1` / `i128::MIN` minimum cases to both tests | src/oracle/boolean_v1.rs:587, tests/it/oracle_generation.rs:183-212, tests/it/bound_strategy_generation.rs:172-191 |
| FND-002 | low | Several new narrowing guards cannot be reached through the public pipelines and no test covers them. The bound oracle (boolean_v1.rs:587) refuses every wide integer type before any of them runs. They are the strategy domain and literal guards (generation.rs:341-351, 372-378), the Kani clause ABI and subject-binding guards (clause.rs:260-264, v1_bundle.rs:411-417) and the literal guard (boolean_v1.rs:598-604, which a literal can reach only through a wide type that line 587 already refuses). Deleting any of them leaves every test green. Either remove the unreachable branches, or keep them as stated defence-in-depth with a direct unit test each | src/strategy/bound/generation.rs:341-351, src/kani/generate/clause.rs:260-264, src/kani/generate/v1_bundle.rs:411-417, src/oracle/boolean_v1.rs:598-604 |
| FND-003 | medium | No CG requirement states the new observable behaviour: IR integer domains or literals outside i64, which IR now admits as i128, are refused (`UnsupportedExpression` in the oracle, `UnsupportedClause` in the strategy, `UnsupportedDependency`/`UnsupportedBinding` in Kani). interface-001 says only that a bounded integer "maps to i64". The new tests trace to the generic NFR-002-AC-3, whose TC-003 row in spec/core/matrix/tests.md:50 is still "Planned". IR-664 asks for spec-first work wherever a CG FR states an i64 limit, and it describes i128 harness types. The code chose refuse-beyond-i64 without recording that decision. Add an AC (interface-001 and FR-008/FR-021 as applicable) that states the i64 ceiling and its refusal codes, or file the ticket that defers i128 harness support | spec/core/functional/interface-001-codegen-api.md:296, spec/core/functional/interface-001-codegen-api.md:259, spec/core/matrix/tests.md:50 |

## Verdict

The i128 adaptation implements the existing i64-only spec correctly and fails closed. The
fixture rekey keeps the IR reader as the only oracle, and no traced AC lost an assertion. Gaps:
the minimum side of the new narrowing is untested (FND-001, a surviving mutant), several
narrowing guards are unreachable and untested (FND-002), and the refuse-beyond-i64 decision is
not specified (FND-003).

## Dispositions

Round 1, reviewed at the branch's second frozen head (fix commit "IR-664: close i64 boundary review findings and fixture ordering"; revision recorded in
the IR-664 Linear marker). Static re-check against the code and the computed matrix: `quire matrix
--format tsv`, before 558 rows and after 561 rows. The new rows are FR-015-AC-82 (tagged),
FR-008-AC-6 (tagged) and FR-031-AC-28 (untagged; see FND-004). The other changed rows are
line-number shifts only.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | round-1 fix commit "IR-664: close i64 boundary review findings and fixture ordering" |
| FND-002 | fixed | round-1 fix commit "IR-664: close i64 boundary review findings and fixture ordering" |
| FND-003 | fixed | round-1 fix commit "IR-664: close i64 boundary review findings and fixture ordering" |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | FR-031-AC-28 is untagged in the computed matrix, but spec/oracle/matrix/tests.md marks it "Covered (IR-664)". The test's tag line is `Trace: TC-003, NFR-002-AC-3; TC-044, FR-031-AC-28.`, and the trace reader binds only the group before the `;` (NFR-002-AC-3 is bound at that line; FR-031-AC-28 is not). The same applies to the `; TC-003, NFR-002-AC-3` tail of `wide_ir_domain_refuses_the_i64_strategy`. Failure: the matrix gate and every later reader see the new AC as unbacked while the status row claims coverage. Fix: put all ids in one comma list, or on a second `Trace:` line | tests/it/oracle_generation.rs:183, tests/it/bound_strategy_generation.rs:172, spec/oracle/matrix/tests.md:51 |

Round 2, reviewed at the branch's third frozen head (fix commit "IR-664: make boundary criteria atomic and trace checked seams"; 13 commits, 40 files,
+1530/-498 against `main`; revision recorded in the IR-664 Linear marker). Computed matrix, `main`
against head: 558 to 564 rows. FR-031-AC-28, FR-008-AC-6, FR-008-CON-3, FR-008-CON-4,
FR-015-CON-1 and FR-015-CON-2 are all `tagged`. NFR-002-AC-3 moves from untagged to tagged.
FR-015-AC-82, which never reached `main`, is gone. The other rows are unchanged. The strategy
literal-conversion residue noted under FND-002 is closed: `checked_relation_literal` (Implements
FR-008-CON-4) is a behaviour-preserving extraction, and
`tc_017_direct_literal_conversion_refuses_outside_i64` asserts `UnsupportedRelation` and the
clause span for all four out-of-range values.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | round-2 fix commit "IR-664: make boundary criteria atomic and trace checked seams" |
