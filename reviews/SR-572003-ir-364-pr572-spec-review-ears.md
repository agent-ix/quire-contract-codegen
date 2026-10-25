---
id: SR-572003
title: "IR-364 PR #572 spec-review/ears-conformance review"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen; PR #572 head bc7213ef556911eb71a19e02b83554c8dde4999f from base 1944268f; spec/strategy/functional/FR-011-numeric-harness-campaigns.md, spec/strategy/functional/FR-002-tristate-proptest.md; IR-364"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-011
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: reviews
---

## Summary

EARS conformance review of the requirement statements PR #572 adds or edits, at frozen head
bc7213ef. Ticket: IR-364. `quire validate --scope . 'spec/**/*.md' --summary` (quire 0.36.2,
engine 0.50.2) reports no grammar finding in the changed files; the findings below are manual.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The refusal sentences make a condition the subject of `shall` ("A missing source or missing request point shall yield `MissingExecutionPoint`") instead of the unwanted-behaviour form "If ..., then the generator shall return ..."; "a missing source" (singular) can be read as "the binding is missing" even when a caller context exists | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:72-77; spec/strategy/functional/FR-002-tristate-proptest.md:82-86 |
| FND-002 | low | Each new bullet packs four obligations (verify, refuse missing, refuse mismatch, no kind default) into one statement; split into one event statement and two unwanted-behaviour statements so each AC traces to one shall | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:72-77; spec/strategy/functional/FR-002-tristate-proptest.md:82-86 |

## Verdict

CONDITIONAL (low only). FR-011's bullet uses the event form ("When generation is requested, the
generator shall ...") and the edited `construct_verdict` statement keeps its ubiquitous form with
the runner as subject. FR-002's "Planned (IR-815): the V2 harness generator shall ..." matches the
neighbouring planned statements. Only the refusal clauses and their atomicity need rewording.

## New findings (disposition pass 1)

Checked at fix-round head fd1de474dd98225d6817e921f4d012e3c3ff9f5f.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The new verdict-point statements make a verdict or outcome the subject of shall (a RejectedPrecondition verdict shall use ..., a non-completed precondition outcome shall retain ...) instead of the harness generator or generated harness. | spec/strategy/functional/FR-002-tristate-proptest.md:96-101 |

## Dispositions

Round 1, checked against the repository at head fd1de474dd98225d6817e921f4d012e3c3ff9f5f.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Refusals use the If ..., then the generator shall ... form with the generator as subject. |
| FND-002 | fixed | Split into one source statement, two unwanted-behaviour statements and a kind prohibition. |
| FND-003 | fixed | Round 2 at cc1917b5: The four verdict-point statements now have the generated harness as subject, one obligation each. |

## Coverage

```yaml
scope:
- id: FR-011
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "When generation is requested, the generator shall require the request's typed execution point to equal the selected clause's accepted QSL binding point when present, or the caller's typed invocation point when the binding supplies none. A missing source or missing request point shall yield `MissingExecutionPoint`; a mismatch shall yield `ExecutionPointMismatch`. Either refusal produces no artifact; the generator shall not infer a point from clause kind or silently substitute another point."
- id: FR-002
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: "Planned (IR-815): the V2 harness generator shall independently verify each selected clause's request point against that clause's accepted QSL binding point when supplied, or its caller invocation context when the binding supplies none. A missing source or request point shall return `MissingExecutionPoint`; a mismatch shall return `ExecutionPointMismatch`; both produce no artifact. Clause kind shall not supply a default point."
```
