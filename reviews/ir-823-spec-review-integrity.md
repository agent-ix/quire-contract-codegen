---
id: SR-11911
title: "integrity review of IR-823 FR-014-AC-6/AC-47 TC-187 rows"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/tests.md (PR #402 head 2707b3158ce70deb49f4ca8e2f367eba4414da2e, diff origin/main...HEAD)"
review_set: subset
date: "2026-10-16"
---

# integrity review of IR-823 (CG PR #402)

## Summary

Ticket: IR-823. Spec-only PR #402 amends FR-014-AC-6 (TC-187 rows U02, U05, U12, U14, U20, U27 agreement; U09 refusal; U06/U11 recorded as admission-only), adds FR-014-AC-47 (QSL value-authority agreement, PLANNED) and updates TC-024 step 4 and tests.md. This `integrity` lens was run as part of the IR-823 spec review set (base, integrity, evidence, scope-boundary, failure-domain; EARS and dependency skipped: no FR statement and no `relationships:` edge changed). Per-vector dispositions were checked against QSpec TC-187 (quire-specification main 9a1d3cf), Contract Runtime at the locked 3cc88c48, and QSL at the locked 23f9f342. `make spec` exits 0.

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
| FND-001 | low | FR-014-AC-47 mixes the criterion with point-in-time repository status and rationale (no vector runs on main; Cargo.lock contents; which qsl-replay entries exist; the QSL module move). These facts change without any behaviour change: Cargo.lock pins QSL 23f9f342 while QSL main is already 2df53cdd, and the next dependency bump or qsl-replay export silently makes the AC text false. Keep the acceptance condition atomic and move the status/rationale to a note or the ticket. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:487 |

## Verdict

Findings above for this lens. Verified clean: every U02/U05/U12/U14/U20/U27 operation and the units it uses exist in `GraphSpec::tc187` (U12 needs no fixture change: `m` and `s` are roots and multiply/divide/power build compound units). U09's `IncompatibleDimensions` matches runtime `type_check` and `compare_quantity` ordering, since a base dimension maps to its own key. U06/U11 are owned by RT FR-012-AC-4 (zero scale and all topology causes). The QSpec values quoted in AC-6 (U02 `L` and the empty dimension) match TC-187. The zero-agreement guard is kept. The author's dependency claims are true: `quire_spec_language` has no `value` module, the evaluator is in `qsl-eval`, `qsl-eval` is in Cargo.lock only under `qsl-replay`, and Cargo.toml names `qsl-replay` alone.
