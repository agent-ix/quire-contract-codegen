---
id: SR-11913
title: "scope-boundary review of IR-823 FR-014-AC-6/AC-47 TC-187 rows"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen; spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/tests.md (PR #402 head 2707b3158ce70deb49f4ca8e2f367eba4414da2e, diff origin/main...HEAD)"
review_set: subset
date: "2026-10-16"
---

# scope-boundary review of IR-823 (CG PR #402)

## Summary

Ticket: IR-823. Spec-only PR #402 amends FR-014-AC-6 (TC-187 rows U02, U05, U12, U14, U20, U27 agreement; U09 refusal; U06/U11 recorded as admission-only), adds FR-014-AC-47 (QSL value-authority agreement, PLANNED) and updates TC-024 step 4 and tests.md. This `scope-boundary` lens was run as part of the IR-823 spec review set (base, integrity, evidence, scope-boundary, failure-domain; EARS and dependency skipped: no FR statement and no `relationships:` edge changed). Per-vector dispositions were checked against QSpec TC-187 (quire-specification main 9a1d3cf), Contract Runtime at the locked 3cc88c48, and QSL at the locked 23f9f342. `make spec` exits 0.

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
| FND-001 | high | FR-014-AC-47 makes the TC-187 QSL value-authority comparison a CG-owned PLANNED obligation (IR-823), evidenced by TC-024 (tests.md:39, and TC-024's Traces To at tests.md:95). The owner of RT/QSL scalar agreement is quire-integration IR-669: RT's FR-007-AC-5/TC-022 rows name it, and RT removed its own qsl-agreement lane in IR-430. AC-47 never names IR-669. The IR-823 planner ruling (option c) also says CG owns no QSL-authority comparison and AC-47 points to IR-669. Failure: IR-823 closes once the AC-6 rows land, and AC-47 stays '🚧 Planned (IR-823)' against a closed ticket. Its test obligation cannot be discharged in this repo under T12-A (ruling: option (b) breaks it), so the criterion stays open with no owner. Fix: restate AC-47 as a boundary (CG claims QSL-authority agreement for no TC-187 vector; that comparison is IR-669's in quire-integration) and drop it from TC-024's evidence, or re-own it to IR-669. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:487 |
| FND-002 | medium | AC-47's blocking rationale is inaccurate. qsl-replay's public evaluating API is more than replay_operator_parity and replay_value_parity: it also exports replay (the FR-098 executor), replay_composite_parity, replay_frame and replay_state_clause (qsl-replay/src/lib.rs:69-74 and :202 at the locked 23f9f342). replay and replay_value_parity accept ValueType::Quantity parameters with quantity witness bindings (qsl-replay/src/execute.rs:1306, execute/argument.rs:684-696). So a compiled QSL function over quantity parameters could settle value rows such as U12/U19/U24/U28 through the T12-A-permitted API (the ruling's option (a)). That route is only partial (no ill-typed rows, no charges), but it exists. Failure: a reader takes AC-47's text as proof that no route exists and never records why option (a) was rejected. Fix: name the real reason (partial coverage, ruled out in favour of IR-669). | spec/oracle/functional/FR-014-exact-scalar-oracles.md:487 |

## Verdict

Findings above for this lens. Verified clean: every U02/U05/U12/U14/U20/U27 operation and the units it uses exist in `GraphSpec::tc187` (U12 needs no fixture change: `m` and `s` are roots and multiply/divide/power build compound units). U09's `IncompatibleDimensions` matches runtime `type_check` and `compare_quantity` ordering, since a base dimension maps to its own key. U06/U11 are owned by RT FR-012-AC-4 (zero scale and all topology causes). The QSpec values quoted in AC-6 (U02 `L` and the empty dimension) match TC-187. The zero-agreement guard is kept. The author's dependency claims are true: `quire_spec_language` has no `value` module, the evaluator is in `qsl-eval`, `qsl-eval` is in Cargo.lock only under `qsl-replay`, and Cargo.toml names `qsl-replay` alone.

## Dispositions

Round 1, fix commits 098ec55 and c1a001b (head c1a001b6c3d7024e8b0a647bd0e08ac019c88b2a), checked against the current text.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | AC-47's inspection lists only test names, trace tags, test-case steps and matrix rows, so it passes while CG's TC-187 test still states QSL agreement in other places. tests/exact_scalar_support/agreement_cases.rs:606 prints 'TC-187 three-way agreement: ...'. tests/exact_scalar_support/agreement.rs:13 says 'The RT==QSL authority check lives in RT's conformance/qsl-agreement lane', but RT removed that lane in IR-430 (RT ccc722b), and IR-669 now owns it. Failure: a reader of the test log or the module doc concludes the TC-187 rows have three-way, QSL-backed agreement, which AC-47 says CG never claims. Fix: include test output and doc comments in AC-47's inspection, and have the IR-823 code PR reword the println and the doc to name IR-669. | tests/exact_scalar_support/agreement_cases.rs:606 |

Round 2, fix commit e7fef2f (head 01ba4ead1928aa2e0cab7eaa6ea11db86fdfaa3a). No earlier fixed finding regressed.

| FND | outcome | reason |
| --- | --- | --- |
| FND-003 | fixed | - |
