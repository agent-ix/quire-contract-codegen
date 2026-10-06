---
id: SR-1822
title: "IR-631 follow-up EARS conformance: FR-032 scalar identity statements and criteria"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen@f84c4fc93fe4ba5f8460e60bb8e75998d5cbcfd8; spec/replay/functional/FR-032-routed-scalar-replay-binding.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
---

# SR-1822: IR-631 follow-up EARS conformance

## Summary

Ticket: IR-631. PR: quire-contract-codegen, not yet opened. It is spec only. Reviewer: claude-opus-5-5, session 8950150c-5937-457d-9408-c19aa15729b8, run f92b6bc1-4101-4d05-affe-ea238bac36af. One low finding.

## Method

I checked the changed FR-032 statements for EARS form: the Behavior bullet on full-claim comparison, the "Scalar Obligation Identity" section and the terminal paragraph. I also checked new and amended criteria AC-1 and AC-3 to AC-10 for a single trigger and response. Unchanged statements were read for context only.

## Verdict

**PASS with one low finding.** The changed Behavior bullet is event-driven: "When asked to return a settlement or derive its terminal value, the converter shall compare…". The identity section uses "CG shall" for each obligation it imposes. AC-6, AC-7 and AC-9 are long but describe one subject each.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-10 combines at least eight independently failing obligations in one criterion:<br>1. byte-exact canonical conformance<br>2. QSL acceptance of the CG-minted identity<br>3. a digest-mismatch refusal with a named cause and codes<br>4. mutated operand, occurrence or kind refusals<br>5. a ban on copied verifiers or synthetic success<br>6. invented occurrence or operand and inline-literal refusals<br>7. encoding refusal with no identity<br>8. unchanged function, clause and frame behaviour<br>A test that covers only some of them has no partial status, and a matrix row cannot say which part is covered. Split AC-10 into separate criteria for conformance and acceptance, mismatch refusal, membership refusal, encoding refusal, and the unchanged other paths. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:301 |
