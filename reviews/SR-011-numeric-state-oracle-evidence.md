---
id: SR-011
title: "Evidence review of numeric and state oracle lowering"
type: SpecReview
analysis: evidence
scope: "FR-001-AC-1 through FR-001-AC-8 and TM-001"
review_set: subset
---
## Summary

`quoin advise --json` found no authored-method mismatch for any FR-001 criterion. The planned
TC-002/TC-003 integration controls provide the recommended example, unit/property, and end-to-end
evidence shapes without claiming implementation evidence early.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No issues found | - |

## Method disposition

The advisor recommends golden/example/unit evidence for deterministic output, property/end-to-end
evidence for the comparison corpus, property evidence for universal refusal behavior, and
example/unit evidence for the remaining identity and package cases. Every criterion is authored as
`Test`, which is among the recommended classes; none is inconclusive or uncatalogued.

TM-001 deliberately leaves the new numeric/state row planned. TC-002 will compile generated Rust,
repeat artifacts byte-for-byte, and compare all six operators over bounded current/pre/post inputs.
TC-003 will verify exact rejection codes and loci with no artifact. Existing Boolean and publication
symbols back only their existing portions; they are not treated as evidence that the new numeric
slice has already passed.
