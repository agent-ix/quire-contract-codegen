---
id: SR-13291
title: "IR-891 CG PR 458 spec-review EARS: NFR-002 and FR-016 edited statements"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen@242c506c617a078d26d748c3fac375f2056f5931 vs origin/main e45b9d2f; spec/core/non-functional/NFR-002-provenance-boundary.md, spec/replay/functional/FR-016-witness-native-replay.md"
review_set: subset
---

## Summary

Ticket: IR-891. Checked the requirement statements the PR edits for EARS form. `make spec` reports
no EARS warning in the touched files; its warnings are in untouched FR-014 and FR-017.

## Examined criteria

- NFR-002 Statement bullet 1, NFR-002:16-19 (examined)
- FR-016 Inputs bullet, FR-016:31-32 (examined)
- FR-016 Behavior bullet 1, FR-016:45-49 (examined)
- FR-016 unwanted-behaviour bullet, FR-016:67-69 (examined)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | "Without both bounds, the decoder shall refuse to decode before reading witness data." uses a non-canonical trigger ("Without") and a new subject ("the decoder") where every other FR-016 statement uses "the generator". Fix: "If either witness decode bound is absent, then the generator shall refuse the decode before reading witness data." (or drop it per SR-13290 FND-001). | spec/replay/functional/FR-016-witness-native-replay.md:48-49 |
| FND-002 | low | NFR-002 Statement packs two conditional requirements into one ubiquitous sentence with trailing "when present" clauses. Fix: split into a ubiquitous statement (generator name and subject identity) and an optional-feature statement: "Where the generation input carries a requirement, revision or clause identifier, the generated Rust file header shall name it." | spec/core/non-functional/NFR-002-provenance-boundary.md:16-19 |

## Verdict

**MINOR.** Two low wording fixes; FR-016:67-69 is a conformant unwanted-behaviour statement.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | df727475: the "Without both bounds, the decoder shall" sentence is removed; FR-016:53 Behavior bullet uses "the generator". |
| FND-002 | fixed | df727475: NFR-002 Statement split into a ubiquitous bullet and a "Where the generation input carries ..." optional-feature bullet. |
