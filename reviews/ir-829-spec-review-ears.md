---
id: SR-11967
title: "IR-829 EARS review"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen@ae138524cf6c4d0eeeb78589999f47d109f4acb3; spec/core/functional/FR-005-cli-conformance.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-005
    type: references
---

## Summary

Ticket: IR-829. Examined the four appended criteria FR-005-AC-6..9, the rewritten FR-005-AC-3 and the edited Behavior bullet for a single subject, a stated condition and one observable response, and checked each against the code it describes (`src/core/artifact.rs`, `src/publication/publish.rs`). `quire validate` is clean on the file.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-005-AC-3 | examined | For every shared-corpus case on which two backends' normalized outcomes differ, the TC-007 Differential Disposition Register holds exactly one row naming the case, the backends, and either a regression fixture that exists in the test tree or a documented profile/backend distinction; ... fails the check. |
| FR-005-AC-6 | examined | When an artifact path is absolute, or contains a `..` segment, a `.` segment, an empty segment, a trailing separator or a backslash, or aliases an earlier path in the same bundle, bundle construction is refused with `UnsafeArtifactPath` ... |
| FR-005-AC-7 | examined | When publication fails before the destination is replaced, or fails after replacement began and the prior destination is restored, the publisher reports destination state `unchanged`, ... |
| FR-005-AC-8 | examined | A failure to remove the replaced bundle's backup after the staged bundle has replaced the destination is reported with destination state `published`; ... |
| FR-005-AC-9 | examined | When replacement fails and restoring the prior destination also fails, the publisher reports destination state `unknown`, ... |
| FR-005 Behavior bullet | examined | Every differential discrepancy shall become a fixture or documented semantic difference, retained as one row of the TC-007 Differential Disposition Register. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-005-AC-6 says a path that "aliases an earlier path in the same bundle" is refused with `UnsafeArtifactPath`. The code has no alias check: `validate_path` refuses `.`/`..`/empty segments per path, and an exact repeat of an earlier path is refused with `DuplicateArtifactPath`, not `UnsafeArtifactPath`. Read as including exact duplicates, the clause contradicts the code; read as noncanonical aliases only, it is redundant with the `.`/empty-segment clauses. Either narrow it to "an interior-dot or empty-segment alias" or drop it, and state the exact-duplicate code. | spec/core/functional/FR-005-cli-conformance.md:52, src/core/artifact.rs:160-170, src/core/artifact.rs:191-211 |
| FND-002 | low | FR-005-AC-8 has no trigger clause: "A failure to remove ... is reported with ..." is a passive declarative, unlike AC-6, AC-7 and AC-9, which open with `When`. Recast as "When removing the replaced bundle's backup fails after the staged bundle has replaced the destination, the publisher reports ...". | spec/core/functional/FR-005-cli-conformance.md:54 |

## Verdict

CHANGES REQUESTED (one medium, one low). FR-005-AC-7 joins two triggers that share one response (`unchanged` with prior bytes, untouched developer file, no residue), which matches the code path; it is singular in response and acceptable. FR-005-AC-9 is singular and matches the rollback-failure branch exactly. FR-005-AC-3 is a single universally quantified condition with three named failure forms: falsifiable, and it invents no numeric bound. The Behavior bullet is a well-formed `shall`. No criterion was renumbered; AC-6..9 are appended.
