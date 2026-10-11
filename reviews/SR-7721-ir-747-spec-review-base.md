---
id: SR-7721
title: "IR-747 base review of CG PR 355"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@ff5689f1009ec613244198a5a3ef4293a750b744; spec/assurance/AD-005-source-obligation-producer.md, spec/core/functional/interface-001-codegen-api.md, spec/evidence/functional/FR-036-source-obligation-census.md, spec/evidence/matrix/TC-051-source-obligation-census.md, spec/evidence/matrix/tests.md, spec/spec.md, spec/tests.md"
review_set: subset
---
## Summary

Reviewed the seven-file IR-747 specification diff against current CG, IR, QSL, and quire-code-rs contracts. Ticket: IR-747. All seven FR-036 criteria, AD-005, TC-051, interface-001 and their registry entries were examined. IR-759 is a declared prerequisite; no implementation evidence is claimed.

## Verdict

**FAIL** — IR candidate identity is underspecified for many-to-many criterion links

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | IR candidate identity is underspecified for many-to-many criterion links | spec/evidence/functional/FR-036-source-obligation-census.md:118 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 14d037f27455832114b558fbb8a9b72aee8a311b |

Round 1 reviewed head: `14d037f27455832114b558fbb8a9b72aee8a311b`. The changed FR-036, AD-005, interface-001 and TC-051 contract and controls now address FND-001. No new finding in the six-file fix diff.
