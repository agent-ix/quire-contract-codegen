---
id: SR-11912
title: "evidence review of IR-823 FR-014-AC-6/AC-47 TC-187 rows"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen; spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/tests.md (PR #402 head 2707b3158ce70deb49f4ca8e2f367eba4414da2e, diff origin/main...HEAD)"
review_set: subset
date: "2026-10-16"
---

# evidence review of IR-823 (CG PR #402)

## Summary

Ticket: IR-823. Spec-only PR #402 amends FR-014-AC-6 (TC-187 rows U02, U05, U12, U14, U20, U27 agreement; U09 refusal; U06/U11 recorded as admission-only), adds FR-014-AC-47 (QSL value-authority agreement, PLANNED) and updates TC-024 step 4 and tests.md. This `evidence` lens was run as part of the IR-823 spec review set (base, integrity, evidence, scope-boundary, failure-domain; EARS and dependency skipped: no FR statement and no `relationships:` edge changed). Per-vector dispositions were checked against QSpec TC-187 (quire-specification main 9a1d3cf), Contract Runtime at the locked 3cc88c48, and QSL at the locked 23f9f342. `make spec` exits 0.

Units examined:

- `FR-014-AC-6` (spec/oracle/functional/FR-014-exact-scalar-oracles.md, examined)
- `FR-014-AC-47` (spec/oracle/functional/FR-014-exact-scalar-oracles.md, examined)
- `TC-024` (spec/oracle/matrix/TC-024-exact-scalar-oracles.md, examined)
- `tests.md FR-014-AC-6 row` (spec/oracle/matrix/tests.md, examined)
- `tests.md FR-014-AC-47 row` (spec/oracle/matrix/tests.md, examined)
- `tests.md TC-024 row` (spec/oracle/matrix/tests.md, examined)
- `FR-014-AC-37` (spec/oracle/functional/FR-014-exact-scalar-oracles.md, context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | 'Adapted to the generated oracle's limits tuple' is undefined, and the rows already run show it is not one rule: QSpec U19 L(4,0,0,2,5,1) runs as [3,0,0,2,5,1], U24 L(4,0,0,2,5,1) as [2,0,0,2,5,1], U23 L(15,..) as [13,..], U28 L(20/13/19,..) as [19/12/18,..] (tests/exact_scalar_support/agreement_cases.rs:545-547, 559, 586; unit_tuple maps fields one-to-one, agreement.rs:100-101). The new U20 and U27 rows inherit it. U27's only content is its tuple boundary (QSpec: undefined, then incomplete{work_units,1,1,1,unit.identity-read}). Because the row asserts generated==runtime under whatever tuple is chosen, an adapted second U27 tuple that no longer exhausts work_units still agrees and passes while QSpec's incomplete disposition is never exercised. Fix: state the adaptation, or require each adapted tuple to reproduce QSpec's outcome kind and limit_kind. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:447 |
| FND-002 | low | FR-014-AC-47 quantifies over 'every QSpec TC-187 vector that FR-014-AC-6 lists', which includes the ill-typed rows U03, U05, U09, U14 and U20's u1/u3 cases. QSpec U14 says these are refused statically, so a compiled QSL function over such operands does not compile and QSL yields no value outcome 'for the same operation and operands' to compare with. For those rows the comparison is undefined, so with 'every' the criterion can never be fully met, even with a route to QSL. Fix: scope it to value-completing rows, or name the compile-time refusal as the authority outcome. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:487 |

## Verdict

Findings above for this lens. Verified clean: every U02/U05/U12/U14/U20/U27 operation and the units it uses exist in `GraphSpec::tc187` (U12 needs no fixture change: `m` and `s` are roots and multiply/divide/power build compound units). U09's `IncompatibleDimensions` matches runtime `type_check` and `compare_quantity` ordering, since a base dimension maps to its own key. U06/U11 are owned by RT FR-012-AC-4 (zero scale and all topology causes). The QSpec values quoted in AC-6 (U02 `L` and the empty dimension) match TC-187. The zero-agreement guard is kept. The author's dependency claims are true: `quire_spec_language` has no `value` module, the evaluator is in `qsl-eval`, `qsl-eval` is in Cargo.lock only under `qsl-replay`, and Cargo.toml names `qsl-replay` alone.
