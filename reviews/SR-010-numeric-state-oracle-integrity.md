---
id: SR-010
title: "Integrity review of numeric and state oracle lowering"
type: SpecReview
analysis: integrity
scope: "StR-001 to FR-001/interface-001/TC-002/TC-003/TM-001 numeric/state trace"
review_set: subset
---
## Summary

The integrity review reconciled the supported grammar, typed dependency signature, undefined-result
boundary, diagnostic shape, and test trace. Two ambiguous statements were narrowed without changing
the issue #83 exit scope.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Conditional wording for arithmetic conflicted with the closed supported grammar and admitted two interpretations. The requirement now unconditionally refuses Numeric and NumericNegate nodes. | FR-001; interface-001 |
| FND-002 | medium | The required exact refusal span conflicted with an interface field described only as optional. The interface now makes SourceSpan mandatory for expression failure codes and absent for unrelated failures. | FR-001-AC-4; interface-001 |

## Trace and consistency

| Stakeholder | Requirement | Interface | Verification |
| --- | --- | --- | --- |
| StR-001 | FR-001-AC-1/2/4/8 | interface-001 oracle_slice and diagnostics | TC-001, TC-002, TC-003 |

The supported value grammar is now singular: Boolean nodes remain supported; integer literals and
direct Boolean/i64 observations are supported only beneath comparison nodes; every arithmetic,
negation, indirect, object, graph, collection, call, option, rational, text, enum, record, local, and
quantified form refuses. The root remains Boolean, and every typed obligation refuses before
rendering. NFR-001 continues to own reproducibility and NFR-002 continues to own explicit failure,
identity, and licensing; no new NFR scope is implied.
