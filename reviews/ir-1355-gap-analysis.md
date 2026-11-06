---
id: SR-2026110283890
title: Traceability review of carried quantity acceptance
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-contract-codegen; IR-1355; src/core/ir/scalar.rs, src/oracle/scalar/derive.rs,
  src/oracle/scalar/derive/tests.rs, src/oracle/scalar/lower.rs, src/oracle/scalar/mod.rs,
  src/oracle/scalar/render.rs, src/routed/capability.rs, tests/exact_scalar_support/carried_quantity_agreement_cases.rs,
  tests/exact_scalar_support/package.rs, tests/it/exact_scalar_agreement.rs, spec/oracle/functional/FR-014-exact-scalar-oracles.md,
  spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/tests.md
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-014
  type: references
---

## Summary

Traceability review of carried quantity acceptance. One medium traceability defect found.

Review snapshot: a82f219b9c889e0e6562e7465082a545b11e61a2; base 5326b3a9. Ticket: IR-1355; PR: quire-contract-codegen#754.

Functional acceptance remains **BLOCKED** by QSV IR-1372: CQ-E13 expects completed integer 2 at limits L(2,0,0,1,2,1), but both generated and direct QSV return Incomplete(WorkUnits, limit=2, consumed=2, next_charge=1, UnitResultRetain). The other 25 published dispositions pass. Do not change published limits, synthesize charges, or claim original IR-842 source-to-generated execution. This is a dependency blocker, not a new CG algorithm defect.

Recorded focused execution evidence inspected: `/tmp/ir1355-carried-all.log` (all 26 compiled/executed, only CQ-E13 fails); `/tmp/ir1355-scalar-lib.log` (30/30); `/tmp/ir1355-focused-clippy-final.log` (focused lib/it Clippy passes); `/tmp/ir1355-spec-final.log` (touched-spec validation). Historical equality-generator failure independently reproduced on untouched main in `/tmp/ir1355-equality-baseline.log`; it is not introduced here. This reviewer ran diff checks, computed matrix and coverage, and explicit artifact validation; did not rerun build/test/Clippy, aggregate gates, dependency audit, or Kani. The final scoped gate remains the lead's later step after owner correction.

## Verdict

**CONDITIONAL** — Restore criterion binding before final qualification.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC62–65 are separated from the headed acceptance table by a blank line, so the computed matrix omits them and new test trace tags bind no criterion. Remove the blank line or provide a complete table header; rerun matrix/coverage to confirm all four criteria bind. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:674 |

## Analysis

Computed static matrix and coverage show the four intended criteria do not mint as targets. Manual source inspection identifies the existing headed table ending at AC61, followed by a blank line and four unheaded rows. Both the integration wrapper and generated-crate native test carry AC62–65 tags; this is a spec table structure gap rather than missing test bodies. Runtime comparisons assert completed values, complete retained carrier type, unit, loss, typed refusals/incomplete records, all counters, ordered charges, and denial of each admitted charge through the existing agree2 macro. No source stub or unsupported unowned feature was introduced.

## Examined scope

```yaml
scope:
- id: FR-014-AC-62
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: For each reader-admitted `quire.op.quantity.convert` into `scalar_type`/`carried_quantity`,
    derivation and generation produce an `ir_confirmed` conversion oracle and a claim
    retaining the exact target structural node id, admitted target unit, bounded Int,
    Decimal or Rational carrier domain, Decimal scale interval when applicable, and
    carrier rounding. The selected operation mode equals the target node's rounding;
    `nearest-even` is derivable for a Decimal or Int carrier and a Rational carrier
    uses `exact`. The oracle passes QSV the corresponding owner-constructed target
    unit and `QuantityTarget
- id: FR-014-AC-63
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Executing generated carried-conversion oracles on reader-admitted packages
    returns the same completed QSV conversion value and target unit as direct `convert_quantity`,
    preserving the distinct carried result type and any loss record: exact Decimal
    `25 cm` gives coefficient 25 at scale two in `m`; exact Int `2 m` gives 2; nearest-even
    Decimal `1 in` gives coefficient 3 at scale two with QSV''s nonzero `DecimalLoss`;
    nearest-even Int `25 cm` gives 0 with its loss; exact Rational `25 cm` gives 1/4
    and `-3 m` gives -3 within the stated domain. No Decimal or Integer completion
    is coerced into an exa'
- id: FR-014-AC-64
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'For a reader-admitted carried conversion, generated and direct QSV execution
    agree in typed stopped outcome, cause, absence of value, charge prefix and counters:
    under `exact`, `1 in` into the bounded Decimal scale-two carrier or bounded Int
    carrier refuses `inexact_conversion` at `convert` before target-domain; `250 cm`
    into `Decimal[-2, 2; 0, 0; exact]` refuses `inexact_conversion` before the out-of-domain
    check even though its exact value is also outside the carrier bounds; exact on-grid
    values outside Decimal or Int bounds refuse the respective `decimal_out_of_domain`
    or `integer_out_of_do'
- id: FR-014-AC-65
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: For each reader-admitted carried target, the generated oracle's complete,
    refused or incomplete outcome, ordered admitted charges, denied next charge and
    all consumed counters equal direct QSV `convert_quantity` at the same limits.
    Work limit four still gives U16's `inexact_conversion` after its four admitted
    work units; work limit one makes U17's out-of-domain Decimal case and U29's out-of-domain
    Int case incomplete at `unit.target-domain`; the huge Decimal scale bound gives
    U18's `decimal_digits` incomplete at that charge without materializing a retained
    coefficient; Rational `8 m` at work l
```

## Coverage and limitations

No applicable AssuranceProfile exists under the repository spec tree. Review restricted to the feature and changed paths listed in scope; no repository-wide assurance PASS is claimed. Base and dependency lenses ran; EARS, integrity, object, security, risk-complexity, failure-domain and evidence lenses were not independently selected because no new requirement statement, domain object, security boundary or relationship is introduced. Optional gap-analysis semantic expansion was not requested; ordinary code-test alignment inspection is included in code review.

Plan completion: not assessed

Reconciliation: quire matrix 0.36.2 (engine 0.50.2), static axis only; no execution evidence store read. Whole-repository output contains 623 tagged, 309 untagged, 12 tagged-by-ignored-test and 93 method-without-symbol rows, with pre-existing unrelated gaps outside this feature review. AC62–65 are absent, not covered. Coverage reports their wrapper test tags as untracked. Manual requirement-to-test check supplied for these four rows; no computed-matrix credit claimed.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Fixed: the separating blank line is removed. Raw computed matrix recognizes AC62–65 as tagged with 3/2/2/2 test binders, and raw coverage reports zero untracked symbols for those four ids. |

## Disposition round 1 verification

Checked source 210c9fa6373f4c02356d7c5b6ede9013b7c1e7c9. Fixed: the separating blank line is removed. Raw computed matrix recognizes AC62–65 as tagged with 3/2/2/2 test binders, and raw coverage reports zero untracked symbols for those four ids. Original finding text and original review verdict above remain historical. Current disposition: **both requested fixes resolved statically; runtime qualification pending**.

Static disposition only. Post-fix lib/it Clippy, private trace test and generated q_dec helper trace compilation remain pending the build lock; no runtime PASS is claimed. QSV IR-1372 still blocks CQ-E13 normative completion under its unchanged two-work-unit limit. The historical equality-generator failure remains reproduced on untouched main; it is not introduced by this PR. No aggregate gate, Kani backend execution, or merge approval.

## Confined runtime verification supplement

The two original review defects are fixed with confined post-fix execution verified. lib+it Clippy completed successfully (`/tmp/ir1355-fix-clippy.log`); the private mismatched-carrier/mode test passed 1/1 (`/tmp/ir1355-fix-private.log`); the actual generated q_dec agreement helper compiled repeated TC-first trace attributes and passed 1/1 (`/tmp/ir1355-fix-generated-helper.log`). AC62–65 remain computed tagged with 3/2/2/2 binders and zero matching untracked symbols. Source fixes remain those of 210c9fa6; current clean pushed custody head is 7c576cee12b03891dee5238385037a67935c2cfc. Existing fixed dispositions remain current; no finding is reopened.

Scoped review fixes are ready conditional on the QSV owner correction. Functional acceptance remains BLOCKED by IR-1372/CQ-E13; the prior 25/26 normative result and generated/direct agreement remain the qualification evidence. This q_dec rerun does not claim all 26 vectors were rerun after the trace change. The historical equality failure remains proven on untouched main. No final feature aggregate gate, Kani execution, or merge approval.


## Owner adoption review supplement

**PASS for the narrow owner-adoption delta; final combined gate PENDING.** Reviewed frozen CG head `1c8089e0d0c0853a8ae2e6ed37fd62287ea8452f`, delta from `7c576cee12b03891dee5238385037a67935c2cfc`, against base `5326b3a9`. No new findings at any severity; earlier finding text and fixed dispositions remain historical and unchanged.

Inspected Cargo.toml, Cargo.lock, TC-024 qualification changes, the QSV owner source delta `585286d..a16416bf`, and the real native harness and execution receipt `/tmp/ir1355-owner-fix-all26-retained-exact.log`. QSV PR97 keeps `unit.result-retain` with its occurrence/result charges, charging zero additional work only for integer conversion after paid target-domain work; other retention callers explicitly retain one work unit. It introduces no new Exact API dependency. Cargo.lock changes only QSV to `a16416bf014df3c067f8b1640f224c2e3c4bb5e1`; the compatible Exact `7e06eba7` entry remains byte-for-byte unchanged. The manifest removes fourteen inherited exact-version constraints, retaining compatible versions and first-party `main` branches, with no new rev/hash pins.

The receipt records native compiled execution of the reader-admitted generated/direct harness, outer test 1/1 PASS, covering CQ-E01..CQ-E26 via each generated target and checking published values, loss, units, full retained carrier identity, typed stops, charges, counters and charge denials. CQ-E13 now completes with integer 2 at unchanged `L(2,0,0,1,2,1)`; the earlier IR-1372 blocker is resolved for this scope. Qualification now passes all 26 dispositions. Production and test source are unchanged in this adoption delta; later changes are qualification/review custody only. This does not credit original IR-842 source-to-generated integration cells. The untouched-main equality failure remains historical baseline evidence outside the final native filter module.

This bounded disposition does not rerun the full review, global matrix, builds, aggregate gates, Kani or backend execution. The lead must complete the final combined-tree scoped gate after adopting these four existing review records; no final-gate PASS or merge approval is claimed here.
