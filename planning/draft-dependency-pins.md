---
id: REV-001
title: "Codegen draft dependency pins"
type: Review
---

# Draft dependency pins

Status: provisional; mandatory reconciliation before semantic implementation leaves draft.

| Dependency | Current identity | State | Reconciliation trigger |
|---|---|---|---|
| Program governance | merged `main` revision `7dac9d8c19952412b56a0347387666e2ca81e01d`; envelope schema SHA-256 `0946e235e9e4b0fa79e9b9ec27ae157b303c17de0a9408d3cc04968fb7152256` | merged tree matches reviewed PR #12 head; merged-main release check passes; exact identity and schema digest reconciled locally | any later policy revision or schema digest |
| IR schema/corpus | reviewed PR #19 snapshot `37eb00153d5c139ebc01622b6e12a4ab79256f88` | open and not accepted; current `db24d900…` head is under explicit REQUEST CHANGES / NOT MERGEABLE review | accepted schema, corpus revision, digest, diagnostics, dependency-set contract |
| Runtime helpers | merged `main` revision `e360dad8a3e0e54f9b8457ff7f3748be0f2acdb3`; reviewed PR #5 head `4e0edec972c7e1431cf0d81ed8346a0ab8817af7` has the identical tree | ten review rounds processed; every finding is closed, strict MSRV protection is reconciled, and complete local evidence is retained; merged-main manual run and human release decision pending | runtime release decision or a later public API or feature revision |

Draft work must not invent missing IR types or publish a stable dependency claim. A later rebase or
rebranch may replace every provisional name and identity without compatibility promises.
