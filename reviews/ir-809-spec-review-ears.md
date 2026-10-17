---
id: SR-11904
title: "EARS review of IR-809 NoVerdict diagnostics"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md lines 344-428 (PR #403)"
review_set: subset
---

## Summary

Ticket: IR-809. Classified each shall-statement of the NoVerdict diagnostics subsection. Most are
ubiquitous ("The generator shall ...") or event-driven ("When the classified outcome ... the
generator shall ...", "Where a group's process exits ..."), and Quire's EARS checker raised no
warning on them. One low finding.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | Three statements name a non-system subject before `shall` ("The phase shall name ...", "Each excerpt shall carry ...", "The diagnostic record shall serialize ..."), and two normative-looking bullets have no `shall` at all ("A CBMC failure that Kani observes is not NoVerdict ...", "The 8192-byte excerpt is a copy cut for reading ..."). Recast with "the generator shall" so each obligation has the system as its actor. | spec/kani/functional/FR-017-kani-execution-evidence.md:381, :386, :408, :418, :426 | wrong-requirement |

## Verdict

Low wording finding only.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 (fe3a4e1): Subject-less statements recast with 'the generator shall'; the two no-shall bullets are now marked 'Rationale, not a requirement' or rewritten as generator obligations. |
