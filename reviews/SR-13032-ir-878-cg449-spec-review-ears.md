---
id: SR-13032
title: "IR-878 CG PR 449 EARS review: FR-042 statements"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen@f69874b0a8d63bf6f2384656961ee15c105de642 vs origin/main 4fb3402; spec/oracle/functional/FR-042-generated-checked-invariant-causes.md"
review_set: subset
---

## Summary

Ticket: IR-878. FR-042 has three SHALL statements, and each is ubiquitous EARS with an explicit subject ("The code generator SHALL ...", "The generated oracle SHALL ..."). The `quire validate` EARS grammar checks raise no warning on FR-042. The AC text is a verbatim re-home and not a requirement statement, so it is out of EARS scope; it is reviewed in SR-13030.

## Examined criteria

- Description: "The code generator SHALL assign the shared quire-exact typed checked-invariant cause at each generated Boolean, equality, function and pre-check placeholder constructor." (clean)
- Behavior: "The code generator SHALL use the nine CG-owned causes in quire-exact FR-369 at their owning constructors." (clean)
- Behavior: "The generated oracle SHALL preserve downstream replay payload and charge behavior." (clean as grammar; the scope of the subject is raised in SR-13030 FND-001)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

**PASS**: all three statements conform to ubiquitous EARS.
