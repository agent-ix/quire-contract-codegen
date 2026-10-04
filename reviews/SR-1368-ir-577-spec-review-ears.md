---
id: "SR-1368"
title: "CG PR 255 spec review (EARS conformance): NFR-005 statement bullet and FR-022 bullet"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-contract-codegen@3b08c1752f8209a46ea2f3605a769a1916f5a025; spec/core/non-functional/NFR-005-no-generation-panics.md:25-31 (new Statement bullet), spec/routed/functional/FR-022-routed-generation.md:210-213 (new Behavior bullet)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/NFR-005
    type: references
---

# SR-1368: CG PR 255 spec review (EARS conformance)

## Summary

Ticket: IR-577. I checked two new requirement statements.

- FR-022 `If FR-015 reports a DuplicateItem whose first_index is not a position in the Kani
  group, then the generator shall refuse the whole call with ... and shall not index the group by
  it`. This is a well-formed EARS unwanted-behaviour pattern. The refusal and the negative
  obligation describe one response.
- NFR-005's new Statement bullet is a ubiquitous requirement. It packs four obligations into one
  bullet: shall not index, slice or subtract; shall not rely on a panic; shall read with a checked
  accessor; shall return the named refusal. It also carries a scope exclusion and a pointer to
  Scope ("is measured in Scope"), neither of which is a requirement.

## Verdict

Conforms, apart from one low compound statement.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new NFR-005 Statement bullet is compound. It has four `shall` obligations, plus an exclusion sentence and a Scope pointer in the same bullet. Split it into the prohibition and the required checked-read-with-typed-refusal, and move the exclusion to Scope. | spec/core/non-functional/NFR-005-no-generation-panics.md:25-31 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | e26183b: the bullet is split into a prohibition bullet, a checked-read bullet and a scope bullet. |
