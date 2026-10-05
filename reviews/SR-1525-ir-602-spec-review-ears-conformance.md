---
id: "SR-1525"
title: "CG PR 281 EARS conformance: FR-031 new and rewritten statements"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-contract-codegen@607a975f0ab00aab0730539b75ac506a1380dbaa; spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md (Behavior: The kernel the native oracle calls, Arithmetic in the native oracle, Arithmetic in the Kani bundle oracle, Refusals, Consumers), spec/core/functional/interface-001-codegen-api.md oracle_slice, spec/strategy/functional/FR-008-bound-domain-strategy-admission.md AC-3"
---

# SR-1525: CG PR 281 EARS conformance

## Summary

Ticket: IR-602. `make spec` (quire 0.36.1) reports no EARS grammar warning on any file this PR
changes, on head or on main. The only warnings in either tree are in FR-024, which this PR does
not touch.

I read the new and rewritten statements by hand:

- The native-oracle "shall take every exact operation from `quire_exact`" is ubiquitous with a
  subject.
- "shall render integer divide and remainder ... as a call of `exact::divide`" is ubiquitous.
- The bundle-oracle "shall render integer divide ... as the Rust infix operator `/` ... and integer
  remainder as the method call `wrapping_rem`" is ubiquitous.
- The saturate refusal "If ... overflow policy `saturate` ... then the generator shall refuse" is
  an unwanted-behaviour statement.

Each is singular in its obligation. "The code `UnsupportedIntegerDivision` does not exist" is a
statement of fact, not a requirement, and AC-18 makes it testable. The "Why divide is infix" bullet
is rationale placed inside Behavior. Neither is a grammar defect.

## Verdict

Clean.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
