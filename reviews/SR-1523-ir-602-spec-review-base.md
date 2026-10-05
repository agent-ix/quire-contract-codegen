---
id: "SR-1523"
title: "CG PR 281 spec review (base): FR-031 divide and remainder over quire_exact::divide"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@607a975f0ab00aab0730539b75ac506a1380dbaa; spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md, spec/oracle/matrix/TC-044-boolean-oracle-integer-arithmetic.md, spec/oracle/matrix/tests.md, spec/tests.md, spec/core/functional/interface-001-codegen-api.md, spec/core/matrix/TC-003-unsupported-diagnostics.md, spec/strategy/functional/FR-008-bound-domain-strategy-admission.md, spec/kani/non-functional/NFR-006-real-kani-lane-gating.md (diff origin/main...607a975, merge base 6a6e721)"
---

# SR-1523: CG PR 281 spec review (base)

## Summary

Ticket: IR-602. PR: agent-ix/quire-contract-codegen#281 at 607a975, spec only. I checked the
measured claims against the merged kernel (agent-ix/quire-exact at 2ec5e1e), IR main (dec8ade),
Contract Runtime (ccc722b, the commit CG's lock holds, and main 9597b43), QSL main (c5fcc438) and
CG main (6a6e721).

Confirmed against the code:

- `quire_exact::divide(profile, member, dividend, divisor, domain, meter) -> Outcome<Integer>`,
  `DivisionProfile::{Truncating, Floor, Euclidean}`, `DivisionMember::{Quotient, Remainder}`
  (src/division.rs). The charge order is operands, then arithmetic, then domain, then
  result-retain. A zero divisor stops after operands with `Undefined(DivisionByZero)`. A refusal
  stops after domain with `Refusal::DivisionOutOfDomain { domain, member }`, whose code is
  `division_out_of_domain` and whose cause is `quotient-outside-domain` or
  `remainder-outside-domain` (src/outcome.rs:227, 254-256). `i64::MIN / -1` is the exact 2^63;
  under a signed-64 domain the quotient is refused and the remainder completes with 0 (kernel test
  `i64_min_divided_by_minus_one_is_exact`). `quire_exact::Integer` is `Integer(BigInt)`.
- Contract Runtime keeps its own `Integer`, `Meter` and `Outcome` at ccc722b and at main. Its
  `Integer` has an inline `i64` small form, with a CBMC comment (src/exact/integer.rs:27-38).
- `quire_exact` exports `planned_equality` and has no `TypeEnvironment` or `CheckedEquality`.
- IR `check_integer_operator` and `integer_pair_range`
  (crates/quire-contract-model/src/expression.rs:2470-2660) refuse a full-range guarded divide,
  because the quotient interval reaches 2^63, and admit a full-range guarded remainder, because
  its magnitude is at most 2^63-1.
- `wrapping_rem` equals the truncating remainder (`a - b*trunc(a/b)`, computed in i128) on a
  grid that includes negative dividends and divisors, `i64::MIN`, `i64::MAX` and -1. It returns 0
  at `(i64::MIN, -1)`. The sign convention (the remainder takes the dividend's sign) matches
  `BigInt::div_rem`, which `Integer::div_rem_truncating` calls.
- I computed by hand R-1 (q 2, r 0, `2 <= 10`), R-2 (q 10, r 0, `0 <= 5`), O-3 (refused,
  Quotient, -5..=5), O-5 (`Completed(true)`), Z-1 to Z-4 (Undefined; for Z-3 and Z-4,
  `y + 11 = 0` completes inside -10..=11). All agree with the spec.
- The interim refusal sites on CG main are src/core/diagnostic.rs:68-93,
  src/oracle/boolean_v1.rs:752-780, tests/it/bound_strategy_generation.rs:531-551 (TC-017) and
  tests/it/oracle_generation.rs:1356-1431 (tc_023). No IR or QSL code names
  `UnsupportedIntegerDivision`, so the stated break reaches no external caller.
- `make spec` passes on main and on head. Neither tree reports an EARS warning on a changed file.
  `quire coverage --strict` gives 44 unbacked rows and 0 contradicted on both trees. Rows go
  from 442 to 446, and FR-031 goes from 17/21 to 17/25. These match the PR body.

## Verdict

Changes requested. The base defect is a false measured claim (FND-001). The blocking
cross-repository problems are in SR-1527, and the status consistency problems are in SR-1524.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | OD-5 is false. IR's `lower_checked_arithmetic` works on `i128` operands (`CheckedArithmeticRequest.left/right: i128`), and `(i64::MIN as i128).checked_rem(-1)` is `Some(0)`, which is in range. I measured this with rustc. `checked_div` gives `Some(2^63)` there, and the range check then refuses it. So IR's kani_arithmetic agrees with the kernel at both pairs, and its two components do not disagree. The spec records a false IR defect and says "To report to IR". Delete OD-5, and do not file an IR ticket. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:520-527; IR src/kani/arithmetic.rs:15-27,107-118 |
| FND-002 | low | Raw `/` is safe in the bundle, but the spec argues it from one `check_expression` measurement. The safety actually follows from IR FR-015's range obligation under `reject`: `check_integer_operator` refuses any node whose quotient interval leaves the type, and the type is inside i64. Cite that guarantee so the safety does not rest on one observed run. | FR-031:318-330 |
| FND-003 | low | OD-3 can be settled from evidence now. For two `Integer` operands, Contract Runtime's `CheckedEquality::evaluate` takes `EqualitySchedule::Plan`, calls `planned_equality` and negates the result for `NotEqual`. Its operand conversion from `Integer` to `Integer` charges nothing (RT ccc722b src/exact/equality.rs:162-185, 427-440). So the route is the same in value and in charges. State that, rather than leave it open for QSL. | FR-031:509-512 |

## Dispositions

Round 1, reviewed at f27977083e292d1334aa7a92c63f570af305b532 (fix commit f279770 plus a merge of main 735e704).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | f279770: OD-5 is deleted, and no dependent sentence remains (grep finds no OD-5, kani_arithmetic or disagreement text). The remaining "checked_div and checked_rem compute in IR's own arithmetic preimage" (FR-031:206) is true: they are i128 and truncating. |
| FND-002 | fixed | f279770: raw `/` safety is now argued from IR FR-015's non-zero-divisor obligation and the range proof under `reject`, and the `check_expression` run is called an observation of that guarantee, not its basis. |
| FND-003 | fixed | f279770: OD-3 is closed by measurement. RT `CheckedEquality::evaluate` takes `EqualitySchedule::Plan` and calls `planned_equality`, negated for NotEqual, with no extra charge. This matches RT ccc722b src/exact/equality.rs:162-185 and 427-440. |
