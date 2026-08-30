# Draft dependency pins

Status: provisional; mandatory reconciliation before semantic implementation leaves draft.

| Dependency | Current identity | State | Reconciliation trigger |
|---|---|---|---|
| Program governance | `agent-ix/quire-contract-ir#3` | open, project status Specify; no PR/candidate branch | accepted policy artifact and source revision |
| IR schema/corpus | `agent-ix/quire-contract-ir#10` | open; no PR/candidate branch | reviewed schema, corpus revision, digest, diagnostics, dependency-set contract |
| Runtime helpers | `agent-ix/quire-contract-runtime#5` head `4392a2385f95defdeef2ee883fcc8024cab1d168` | CI-green draft | reviewed/merged source tag, public API and feature reconciliation |

Draft work must not invent missing IR types or publish a stable dependency claim. A later rebase or
rebranch may replace every provisional name and identity without compatibility promises.
