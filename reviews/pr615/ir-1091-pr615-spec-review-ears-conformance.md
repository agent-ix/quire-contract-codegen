---
id: SR-615002
title: IR-1091 PR 615 unique collection capacity refusal — EARS conformance
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen; IR-1091; PR 615 head 18ebb99edf790cd9546ad2fa8d3329a2bf758fbe vs main 92ebb4e2943a47cc172a4237c82077c8b7da9859; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md"
review_set: subset
---

## Summary

Ticket: IR-1091. EARS review of the new normative clauses in FR-015-AC-96 at
head 18ebb99e. No FR, NFR or StR statement changed, and quire's grammar check
reports no EARS diagnostic for FR-015. The new clauses read as one
definition plus two event-style branches:

- the definition of distinct-value capacity
- "When the declared cardinality maximum exceeds that capacity ... the item
  settles `unsupported`"
- "When the maximum is at most the capacity, including equality, the ...
  obligations remain"

Each branch names its subject and its observable response. The one defect is
atomicity. A behaviour with its own PLANNED status is folded into an AC whose
other clauses are PARTIAL, so neither status nor trace can be read per
behaviour.

## Verdict

PASS WITH FINDINGS. The triggers are canonical, the responses are observable
(disposition, typed reason with both values, no harness, no proof credit), and
the negative clauses (no reduced maximum, no non-unique substitute, no
narrowed domain) can be tested. FND-001 is low and does not block.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-015-AC-96 is now compound with mixed status. It carries the original symbolic-domain obligation (PARTIAL), the unbacked uniqueness clause, and the new capacity-refusal branch (PLANNED, pending owner decision) in one row, so a test tagged FR-015-AC-96 cannot show which behaviour it backs and the matrix cannot report the capacity branch separately. A separate AC for the capacity refusal would make status and trace atomic. | spec/kani/functional/FR-015-bounded-kani-obligations.md:846 |

## Coverage

Examined the added clauses of FR-015-AC-96 (line 846) and the TC-025
paragraph (lines 71-86) for trigger, subject and response form. The FR-015
statement is unchanged and `quire validate` raised no EARS warning for
FR-015 or TC-025.
