---
id: SR-009
title: "Failure-domain review of numeric and state oracle lowering"
type: SpecReview
analysis: failure-domain
scope: "FR-001, interface-001, TC-002, TC-003 numeric/state slice"
review_set: subset
---
## Summary

The failure-domain review examined undefined arithmetic, dependency identity, diagnostic ownership,
evaluation purity, and bounded expression topology. The revised specification now refuses every
arithmetic path and selects one exact, deterministic IR-owned refusal locus.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The first draft rejected arithmetic only when definedness could not be represented, leaving obligation-free arithmetic open to an unsafe checked-result unwrap. It now rejects every numeric arithmetic and negation node until invalid is representable. | FR-001; interface-001 |
| FND-002 | medium | Multiple unsupported nodes or obligations had no declared tie-breaker for the reported span. Authored preorder and retained IR obligation order now select the locus deterministically. | FR-001-AC-4; interface-001; TC-003 |

## Failure controls

`SourceSpan` retains the IR source identity and both endpoints, so a diagnostic cannot pair an
unowned path with a guessed local offset. NonBooleanRoot points at the clause root;
UnsupportedExpression and UnsupportedDependency point at the first rejected expression node;
UnsupportedObligations points at the first retained obligation. All fail before any artifact bundle
is returned.

The generated evaluator remains pure and accepts only direct typed values. The IR limits expression
nodes and depth, and codegen retains its independent stack and 1 MiB source limits. Dereference,
reachability, collection topology, callbacks, I/O, and shared mutation are outside this slice and
refuse rather than reaching an unbounded traversal or extension point.
