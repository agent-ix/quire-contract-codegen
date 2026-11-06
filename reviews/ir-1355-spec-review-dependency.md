---
id: SR-2026110283892
title: Dependency review of carried quantity acceptance
type: SpecReview
analysis: dependency
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

Dependency review of carried quantity acceptance. No findings in this method.

Review snapshot: a82f219b9c889e0e6562e7465082a545b11e61a2; base 5326b3a9. Ticket: IR-1355; PR: quire-contract-codegen#754.

Functional acceptance remains **BLOCKED** by QSV IR-1372: CQ-E13 expects completed integer 2 at limits L(2,0,0,1,2,1), but both generated and direct QSV return Incomplete(WorkUnits, limit=2, consumed=2, next_charge=1, UnitResultRetain). The other 25 published dispositions pass. Do not change published limits, synthesize charges, or claim original IR-842 source-to-generated execution. This is a dependency blocker, not a new CG algorithm defect.

Recorded focused execution evidence inspected: `/tmp/ir1355-carried-all.log` (all 26 compiled/executed, only CQ-E13 fails); `/tmp/ir1355-scalar-lib.log` (30/30); `/tmp/ir1355-focused-clippy-final.log` (focused lib/it Clippy passes); `/tmp/ir1355-spec-final.log` (touched-spec validation). Historical equality-generator failure independently reproduced on untouched main in `/tmp/ir1355-equality-baseline.log`; it is not introduced here. This reviewer ran diff checks, computed matrix and coverage, and explicit artifact validation; did not rerun build/test/Clippy, aggregate gates, dependency audit, or Kani. The final scoped gate remains the lead's later step after owner correction.

## Verdict

**PASS** — No method-specific defects found; this is not functional acceptance or merge approval.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Analysis

Classification: reader admission is enablement; CG derivation/rendering is feature behavior; owner conversion and metering correctness is prerequisite enablement for normative value/accounting qualification; IR-842 source-to-generated integration is downstream feature verification. Order: admitted QSpec node schema and owner carrier APIs -> reader-admitted CG generation -> normative generated/direct qualification -> original IR-842 integration. IR-1372 presently blocks qualification at CQ-E13; it does not authorize a consumer-side compensation. No new relationships edge is added, no cycle is introduced, and no upstream-owned algorithm is moved into CG. Original source emission/admission and IR-842 integration are not credited by reader-admitted tests.

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
