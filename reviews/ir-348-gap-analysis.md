---
id: SR-585010
title: IR-348 CG589 exact scalar gap analysis
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-348; PR589; af15747de1c9ffe1cdd9b5d7c8c64b7fd02f142c; AD-004 L-8, FR-014"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/AD-004", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-014", type: references }
---

## Summary

Ticket: IR-348. Independent Opus 5.5 planless gap analysis of CG #589 at original head `af15747d`. The diff introduces no new untagged criterion, uncovered path or stub. The repository matrix still reports two older FR-014 gaps outside this PR's scope.

## Verdict

FAIL under the strict repository-wide gap rule because FR-014-AC-54 is untagged. The PR's own exact-scalar slice is CONDITIONAL pending code-review findings. The two reported matrix gaps predate this PR and are not attributed to it.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-014-AC-54 is untagged in the computed matrix. It is planned under IR-917 and predates this PR. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:612 |
| FND-002 | low | FR-014-AC-47 is an Inspection criterion with no source symbol (method-without-symbol) in the computed matrix; this predates the PR. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:605 |

## Coverage

Read-only `quire matrix --scope . --format json` at the original head: FR-014 has 53 criteria, 51 tagged, AC-47 `method-without-symbol` and AC-54 untagged. Examined changed source and the scoped AD-004 L-8 guard; the diff adds or removes no FR-014 trace tag. Reverse-gap and plan completion were not assessed; plan completion: not assessed. No semantic intent↔test↔code review was requested.

## Examined scope and bindings

```yaml
clean: false
scope:
- id: AD-004-L-8
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: 'L-8. `serde_json::Value` and `.get("` appear for body terms only in `core/ir`.
    Test: a grep gate.'
- id: FR-014-AC-47
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: No test name, trace tag, test output line or doc comment of every oracle-agreement
    corpus of FR-014, FR-018 and FR-021 (the TC-185, TC-186, TC-187, TC-192 and TC-193
    vectors, the FR-014-AC-37 corpus, the composite-equality corpus and the function-application
    corpus), no TC-024, TC-029 or TC-031 step and no oracle-matrix row for those cases
    or corpora claims, or presents as evidence, agreement with the QSL value authority;
    text that states this boundary is permitted.
- id: FR-014-AC-54
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: PLANNED (IR-917). A QSL-authored V2 clause comparison containing an integer
    literal wrapped by the checked FR-093 `quire.op.numeric.narrow` shape derives
    and generates its clause-identified oracle when the complete literal parses and
    lies in the target `Int[..]` interval. The oracle uses the unchanged integer value
    and yields the same ordered admitted charges, consumed counters and outcome as
    independent direct-owner evaluation of that clause arithmetic with that integer;
    the wrapper itself admits no charge. Missing target or operand, out-of-range literal,
    nonliteral operand and unparseable li
bindings: []
```

## Dispositions

### Round 1 — exact `15cfb7a636ed5f72a0804a8c63954445e94ad082`

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | Pre-existing FR-014-AC-54 is PLANNED under IR-917; CG #589 changes neither this criterion nor its tests. Repository-wide matrix remains red. |
| FND-002 | deferred | Pre-existing Inspection criterion method-without-symbol; CG #589 adds or removes no FR-014 trace tags. Out of scope for this PR. |
