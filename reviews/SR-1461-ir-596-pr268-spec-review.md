---
id: "SR-1461"
title: "IR-596 PR 268 spec review: FR-031 status flips, TC-044 and the oracle matrix"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@57ad290a70d39f20989635a10fc6af8bce3dc677; spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md, spec/oracle/matrix/TC-044-boolean-oracle-integer-arithmetic.md, spec/oracle/matrix/tests.md, spec/tests.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-031
    type: reviews
---

# SR-1461: IR-596 PR 268 spec review (integrity)

## Summary

Ticket: IR-596. PR: agent-ix/quire-contract-codegen#268, head 57ad290. The spec diff flips 16
closing ACs from PLANNED to IMPLEMENTED and AC-11 to "IMPLEMENTED IN CG; THE QSL EXEMPLAR RUN IS
PENDING". It also rewrites TC-044's status paragraph and updates the oracle matrix and the index.
Sub-analysis: spec-integrity-analysis, which checks status truthfulness and consistency against
the code at the head. The FR-031 text outside the status prefixes was merged in #263 and is read
here only where the code's behaviour depends on it.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-031's Consumers section says the tri-state harness "shall refuse with `UnsupportedExpression` at the first arithmetic node". The Interim refusals apply divide, remainder and `saturate` refusals "whatever its overflow policy or consumer". The code and `tc_044_consumers_that_need_a_plain_bool_refuse_arithmetic` follow the Interim reading: the harness returns `UnsupportedIntegerDivision` or `UnsupportedSaturatingArithmetic` for those nodes and `UnsupportedExpression` only for a `reject` add, subtract or multiply. That reading is consistent, but the Consumers sentence read alone says otherwise, and AC-10 points to it. No test asserts the harness refusal's span. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:289-293, tests/it/oracle_arithmetic.rs:1744-1793 |
| FND-002 | low | FR-031-AC-20 says "a bundle whose harness assumes the declared domain on every drawn input verifies the same clause". Read literally, that cannot hold for its own clause. Under the bundle's generated `proof_for_contract` harness, `x < 5 && x * 2 <= 10` is false at `x = 5..=10`, so the `ensures` fails. The test therefore uses a hand-written probe harness (`overflow_probe::domain_assumed`) that discards the oracle's value and shows only that no overflow check fails. The AC should say what the test proves: a harness that assumes the declared domain and calls the bundle oracle has no failing arithmetic-overflow check. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:360, tests/it/kani_generation.rs:1616-1680 |

## Verdict

The status flips are truthful.

- Each of the 16 IMPLEMENTED ACs has a tagged test whose assertions match its text.
- AC-4 and AC-12 are declared held agreements, and they pass on the base, as stated.
- AC-11's wording separates what CG proves from the pending QSL run.
- The held AC-2, AC-6, AC-16 and AC-17 stay `HELD (IR-601...)`, and nothing claims them.
- TC-044's "⚠️ Partially covered" paragraph and the index row agree with the matrix.
- `make spec` passes with only the known FR-017 EARS warnings.

Context only, outside this diff: interface-001's `semantics_source` gives the reason for the
infix bundle as "exact arithmetic gave no proof verdict for the exemplar clause". FR-031 says the
decisive reason is the exemplar's text contract, not tractability. That is worth aligning in a
later spec change, and it is not a finding against this PR.

Mergeable after the QSL exemplar run (SR-1460 FND-001) and the dispositions.

## Dispositions

Round 1, reviewed at c2e897c3ce84e0bc115f3c339e1e0ff36a342daf (fix commit c2e897c on 57ad290).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | c2e897c: the Consumers sentence now limits the harness's `UnsupportedExpression` to a first arithmetic node that is a `reject` add, subtract or multiply, and sends divide, remainder and `saturate` to the Interim refusals' own codes for every consumer. It matches the code and the test. Not asserting the span is acceptable: `HarnessDiagnostic` has no span field (src/strategy/harness.rs:103-114), `map_clause_diagnostics` drops the oracle's span, and interface-001 promises none for the harness. So "at that node" describes the oracle diagnostic's locus, which the native refusal tests assert. |
| FND-002 | fixed | c2e897c: AC-20 now names the hand-written probe harness that assumes the domain, calls the bundle oracle and discards its value, and says the bundle's own `proof_for_contract` harness is not that probe because its `ensures` is false for `x` in `5..=10`. It matches `kani_bundle_oracle_overflow_is_a_failing_check_not_a_wrapped_value`. |
