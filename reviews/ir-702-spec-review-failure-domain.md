---
id: SR-3506
title: "IR-702 Failure-domain review: early report identity"
type: SpecReview
analysis: failure-domain
review_set: subset
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-034-caller-death-ownership.md FR-034-AC-95..98; spec/kani/matrix/TC-049-caller-death-ownership.md early-report checks a-d and expected results; spec/kani/matrix/tests.md FR-034/TC-049 rows"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: references
---

# SR-3506: IR-702 Failure-domain review

## Summary

Ticket: IR-702. PR: quire-contract-codegen#330. Checked missing/malformed/wrong/late identity, foreign valid pipe, owner/capability mismatch, cleanup and original cutoff, plus feature-off and ordinary unbound behavior.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS. C authenticates provenance before storing the claim and cannot claim backing equality. O checks the echoed identity against its retained actual collector. Failed admission retains custody and typed refusal; the spec forbids writer transfer and new control authority. TC-049 plans a separate negative control for each fault class.

Examined: FR-034-AC-95, FR-034-AC-96, FR-034-AC-97, FR-034-AC-98; the matching TC-049 allocation, procedure and expected-result rows; and both tests.md trace rows. The new tests and Guardian implementation remain PLANNED/UNRUN. No Cargo, Kani or full CI verdict is inferred from this specification review.
