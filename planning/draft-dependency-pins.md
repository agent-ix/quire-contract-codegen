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
| IR schema/corpus | `agent-ix/quire-contract-ir#10` | open; no PR/candidate branch | reviewed schema, corpus revision, digest, diagnostics, dependency-set contract |
| Runtime helpers | `agent-ix/quire-contract-runtime#5` main-based head `4e0edec972c7e1431cf0d81ed8346a0ab8817af7` | nine review rounds processed; round-9 glob-census and collector-ownership findings are closed, strict MSRV protection is reconciled, and complete local evidence is retained; current-head follow-up review and manual run pending | reviewed/merged source tag, public API and feature reconciliation |

Draft work must not invent missing IR types or publish a stable dependency claim. A later rebase or
rebranch may replace every provisional name and identity without compatibility promises.
