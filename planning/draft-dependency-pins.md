---
id: REV-001
title: "Codegen draft dependency pins"
type: Review
---

# Draft dependency pins

Status: provisional; mandatory reconciliation before semantic implementation leaves draft.

| Dependency | Current identity | State | Reconciliation trigger |
|---|---|---|---|
| Program governance | PR #12 head `942670a0db78be57cfa9bdd6d04302b453781a49`; envelope schema SHA-256 `0946e235e9e4b0fa79e9b9ec27ae157b303c17de0a9408d3cc04968fb7152256` | in review; current schema/validator provisionally reconciled; other review findings open | merged policy revision and schema digest |
| IR schema/corpus | `agent-ix/quire-contract-ir#10` | open; no PR/candidate branch | reviewed schema, corpus revision, digest, diagnostics, dependency-set contract |
| Runtime helpers | `agent-ix/quire-contract-runtime#5` head `2a6aa82628c34b52a958ede822ed57285f73b75e` | two review rounds addressed; current PGM-01 head reconciled locally; renewed review and remote run pending | reviewed/merged source tag, public API and feature reconciliation |

Draft work must not invent missing IR types or publish a stable dependency claim. A later rebase or
rebranch may replace every provisional name and identity without compatibility promises.
