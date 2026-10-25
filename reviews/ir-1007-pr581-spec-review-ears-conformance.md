---
id: SR-581003
title: "EARS conformance review of IR-1007 CG PR 581 (FR-021 changed statements)"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-contract-codegen@ee51b2590f4e2909fc121fbeda082348a64fd37d; spec/oracle/functional/FR-021-function-application-oracles.md"
---

## Summary

Ticket: IR-1007. PR: quire-contract-codegen#581, frozen head ee51b2590f4e2909fc121fbeda082348a64fd37d.
Reviewer model: Claude Opus 5.5 (`claude-opus-5-5`). Method: spec-review/ears-conformance.
The engine check `quire validate --scope . spec/oracle/functional/FR-021-function-application-oracles.md --summary`
(quoin 0.29.1) reports "1/1 docs grammar-clean (100%); 0 grammar finding(s)". The one
requirement-bearing `shall` in the changed Description paragraph is unchanged. The diff only
edits status prose around it. Clean.

## Examined scope

- FR-021 Description lines 43-48: examined. "QSL arch-lint T12-A still bars CG from qsl-eval
  and checker/package internals; CG shall use only the FR-485 facade for checked-closure
  acquisition and application, not RT's port or those internal modules (FR-014-AC-47)." The
  statement has a named subject (CG), one `shall` and a concrete, inspectable response, and it
  matches base. The surrounding sentences ("The QSL facade API is delivered ...", "The IR-595
  CG consumer is not merged delivery ...") are status facts with no `shall`, so they are not
  requirement statements.
- FR-021 Behavior bullet line 81 ("The delivered facade entry is ..."): examined. It is
  descriptive, with no `shall` and no EARS obligation.
- FR-021 Dependencies "QSL owner path" bullet, lines 426-431: examined. It has no `shall`.
- Changed AC suffixes (AC-2, AC-4, AC-5, AC-9, AC-17, AC-32, AC-35 to AC-37): context only.
  ACs are outside EARS statement scope, and their bodies are unchanged.
- AD-004 and TC-031: context only (ADR and test-case archetypes, not FR/NFR/StR).

## Verdict

Clean. No `non-singular`, `vague-response`, `missing-subject`, `non-canonical-trigger` or
`unclassifiable` result appears. On the semantic side, the obligation pattern (a ubiquitous
constraint on CG) is unchanged and correct.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
