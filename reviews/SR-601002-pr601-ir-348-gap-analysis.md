---
id: SR-601002
title: "IR-348 PR #601 gap-analysis (V1 Kani retirement: AC-to-test ownership)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; PR #601 head 64992f5be8aebb7d036fba5491fa9ef61af70d29 against merge-base 46dbaccacbd96f55654fc4b331a9d42883e70a2c; deleted and changed tests under tests/it and src unit tests, scripts/cg_gates.rs, spec/*/matrix/tests.md rows for every AC that lost a tagged test; IR-348"
review_set: subset
---

## Summary

Ticket: IR-348. Plan completion: not assessed. This gap-analysis has no plan input. It was done by
hand from the diff and was not a full `quoin matrix` run, because the brief makes this a static
review with no cargo or test runs.

Method: collect every requirement id in a `#[trace]` or trace comment on a removed line. For each
one, count the tagged tests left at head. Then read the matrix row for every AC left with none.

Seventeen AC ids lost every tagged test: FR-005-AC-4, FR-015-AC-1/4/8/13/19/20/24/25/54/56,
FR-031-AC-11/19/20/21 and FR-036-AC-2/3. Sixteen of them are already marked Planned, Open or
Retired in their matrix rows. The exception is FR-015-AC-4. FR-016-AC-9 and AC-10 keep their
default native replay tests, and their installed-Kani control is recorded as Open under IR-1047,
which is honest: no false PASS. FR-041-AC-1 to AC-5 stay Open, and the hold grants no credit.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The matrix row "FR-015-AC-3 through FR-015-AC-6 ✅ Covered" still includes FR-015-AC-4. The PR deleted its only tagged test, `tc_025_assumptions_constrain_only_arguments_to_their_ir_bounds`, a V1 `BoundClause` test, and no test anywhere in `src` or `tests` now carries `FR-015-AC-4`. The matrix therefore claims coverage that no test backs. | spec/kani/matrix/tests.md:14; spec/kani/functional/FR-015-bounded-kani-obligations.md:752 |
| FND-002 | medium | The FR-041 hold the PR adds to `cg_gates.rs::claim_counts` has no negative control. No test checks that a hold on another module, a missing reason or expiry, or two held rows fails the gate. No test checks the `kani-gate: held=` and `claims active=` lines NFR-006 requires either. The claim test's `validate_claim_holds` checks only the one valid checked-in row. | scripts/cg_gates.rs:243-281; tests/it/kani_generation_claims.rs:67-101 |

## Verdict

Changes requested, for FND-001: a ✅ row with no backing test is a false coverage claim. Either
split FR-015-AC-4 out as Open after IR-348, or tag a surviving V2 test that really asserts it.
FND-002 is a test gap in gate code that the PR itself introduced.

Test ownership is otherwise right:

- The V2 routed-scalar real-Kani test moved into the `kani_generation` lane intact.
- The plain-Bool arithmetic refusal test was renamed and kept its FR-031-AC-10 trace.
- The FR-016 identity tests now rely on a direct test-only fixture, and nothing presents that
  fixture as generator evidence.
- No surviving AC lost its only test, except FR-015-AC-4 above.

## Coverage

```yaml
scope:
- {id: FR-015-AC-4, path: spec/kani/functional/FR-015-bounded-kani-obligations.md, role: examined, excerpt: "No harness assumption excludes an undefined, refused or incomplete runtime outcome."}
- {id: FR-015-AC-1, path: spec/kani/matrix/tests.md, role: examined, excerpt: "Planned/open; AC-2's only tagged V1 BoundClause test retires under IR-348"}
- {id: FR-015-AC-8, path: spec/kani/matrix/tests.md, role: examined, excerpt: "Open after IR-348: their only tagged BoundClause tests retire"}
- {id: FR-015-AC-13, path: spec/kani/matrix/tests.md, role: examined, excerpt: "Open after IR-348: its only tagged V1 BoundClause test retires"}
- {id: FR-015-AC-19, path: spec/kani/matrix/tests.md, role: examined, excerpt: "Planned/open: AC-19/20/24 lose V1 evidence and require supported-path measurement"}
- {id: FR-015-AC-54, path: spec/kani/matrix/tests.md, role: examined, excerpt: "AC-54/56 retired as V1 bundle-only under the 2026-10-27 IR-348 ruling"}
- {id: FR-031-AC-11, path: spec/oracle/matrix/tests.md, role: examined, excerpt: "Retired V1 bundle-only claim under the 2026-10-27 IR-348 ruling"}
- {id: FR-031-AC-19, path: spec/oracle/matrix/tests.md, role: examined, excerpt: "FR-031-AC-19 through FR-031-AC-21 Retired V1 bundle-only claims"}
- {id: FR-005-AC-4, path: spec/core/matrix/tests.md, role: examined, excerpt: "FR-005 | FR-005-AC-4 | TC-007 | Planned"}
- {id: FR-036-AC-2, path: spec/evidence/matrix/tests.md, role: examined, excerpt: "TC-051 ... Planned; Gated on IR-759; no executed producer/consumer test claimed"}
- {id: FR-016-AC-9, path: spec/replay/matrix/tests.md, role: examined, excerpt: "FR-016-AC-9, FR-016-AC-10 | TC-026 | Partially backed, Open under IR-1047"}
- {id: FR-016-AC-21, path: tests/it/skeleton_spine.rs, role: examined, excerpt: "tc_026_the_request_slot_is_the_recomputed_function_contract_digest uses spine_identity()"}
- {id: FR-041, path: spec/kani/functional/FR-041-claimed-module-proof-gate.md, role: examined, excerpt: "A missing hold reason or expiry, a held name absent from the declaration, a removed claim relative to the merge-base declaration, or a hold on any other module SHALL fail the gate."}
- {id: FR-031-AC-10, path: tests/it/oracle_arithmetic.rs, role: examined, excerpt: "#[trace(\"TC-044\", \"FR-031-AC-10\")] fn tc_044_boolean_sinks_refuse_arithmetic()"}
- {id: FR-017-AC-7, path: tests/it/kani_generation.rs, role: examined, excerpt: "#[trace(\"NFR-006-AC-4\", \"FR-017-AC-6\", \"FR-017-AC-7\", \"FR-017-AC-11\", \"TC-045\")] fn kani_generation_real_v2_scalar_verifies()"}
```

## Dispositions

Re-checked at exact head 5380318cb380788ba0512e8765d2faa651078965 (fix 9183a6bb, merge 3bb40d0c, 5380318c).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | spec/kani/matrix/tests.md now splits the row: "FR-015-AC-3, FR-015-AC-5, FR-015-AC-6 ✅ Covered" and "FR-015-AC-4 🚧 Open after IR-348: the only tagged V1 BoundClause assumption test retired; surviving V2 runtime-outcome assumptions need criterion-preserving evidence." The FR-015-AC-4 criterion text is unchanged. |
| FND-002 | fixed | 9183a6bb adds `claim_hold_tests` in scripts/cg_gates.rs, which run under `make test` because `cargo test` covers the `cg_gates` bin. They check the exact `kani-gate: held=` and `claims active=0 held=1` lines, and refusal of an unauthorized hold, a missing reason, a missing expiry, a duplicate row and a removed base row. |
